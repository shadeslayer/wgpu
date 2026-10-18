use naga::front::cuda::{parse_str, Options};

use super::helpers::{validate, validate_with_options};

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

#[test]
fn cuda_shared_vectors_preserve_all_nine_variable_and_array_layouts() {
    for (name, scalar) in [
        ("float", naga::Scalar::F32),
        ("int", naga::Scalar::I32),
        ("uint", naga::Scalar::U32),
    ] {
        for (width, stride, alignment) in [(2, 8, 8), (3, 12, 4), (4, 16, 16)] {
            let (module, _) = validate(&format!(
                "__global__ void kernel() {{
                    __shared__ {name}{width} value;
                    __shared__ {name}{width} values[2];
                }}"
            ));
            assert_eq!(module.global_variables.len(), 2);
            let mut globals = module.global_variables.iter();
            let (_, value) = globals.next().unwrap();
            let (_, values) = globals.next().unwrap();
            for global in [value, values] {
                assert_eq!(global.space, naga::AddressSpace::WorkGroup);
                assert_eq!(global.binding, None);
                assert_eq!(global.init, None);
            }
            let naga::TypeInner::Array {
                base,
                size: naga::ArraySize::Constant(count),
                stride: actual_stride,
            } = module.types[values.ty].inner
            else {
                panic!("expected a fixed shared array");
            };
            assert_eq!(base, value.ty);
            assert_eq!(count.get(), 2);
            assert_eq!(actual_stride, stride);
            let mut layouter = naga::proc::Layouter::default();
            layouter.update(module.to_ctx()).unwrap();
            assert_eq!(layouter[value.ty].size, stride);
            assert_eq!(layouter[values.ty].size, stride * 2);
            assert_eq!(
                layouter[value.ty].alignment,
                naga::proc::Alignment::new(alignment).unwrap()
            );
            match &module.types[value.ty].inner {
                naga::TypeInner::Vector {
                    size,
                    scalar: actual,
                } => {
                    assert_eq!(*size as u32, width);
                    assert_eq!(*actual, scalar);
                }
                naga::TypeInner::Struct { members, span } => {
                    assert_eq!(width, 3);
                    assert_eq!(*span, 12);
                    assert_eq!(members.len(), 3);
                    for (index, member) in members.iter().enumerate() {
                        assert_eq!(member.offset, index as u32 * 4);
                        assert_eq!(
                            module.types[member.ty].inner,
                            naga::TypeInner::Scalar(scalar)
                        );
                    }
                }
                _ => panic!("expected a vector or packed scalar-field struct"),
            }
            let function = &module.entry_points[0].function;
            assert!(function.local_variables.is_empty());
            assert!(function.body.is_empty());
        }
    }
}

#[test]
fn cuda_shared_vectors_keep_scalar_uniform_and_storage_bindings() {
    let (module, _) = validate(
        "using Triple = float3;
        __global__ void kernel(float factor, const Triple *input, Triple *output) {
            __shared__ Triple value;
            __shared__ Triple values[2];
            value = input[0]; values[1] = value;
            values[1].x *= factor; output[0] = values[1];
        }",
    );
    assert_eq!(module.global_variables.len(), 5);
    for (index, (_, global)) in module.global_variables.iter().enumerate() {
        assert_eq!(
            global.binding,
            (index < 3).then_some(naga::ResourceBinding {
                group: 0,
                binding: index as u32,
            })
        );
    }
    let (_, factor) = module.global_variables.iter().next().unwrap();
    assert_eq!(factor.space, naga::AddressSpace::Uniform);
    let naga::TypeInner::Struct { members, span } = &module.types[factor.ty].inner else {
        panic!("scalar parameters must remain uniform blocks");
    };
    assert_eq!(*span, 16);
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].offset, 0);
    assert_eq!(
        module.types[members[0].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::F32)
    );
}

#[test]
fn cuda_shared_vector_aliases_and_names_follow_block_and_loop_scopes() {
    let (module, _) = validate(
        "using Triple = float3;
        __global__ void kernel(int *out) {
            __shared__ Triple value;
            value = make_float3(1, 2, 3);
            { __shared__ Triple value; value = make_float3(4, 5, 6); }
            { using Triple = int; Triple value = 7; out[0] = value; }
            for (__shared__ Triple step; false;) { step = value; }
            out[1] = int(value.x);
        }",
    );
    let shared: Vec<_> = module
        .global_variables
        .iter()
        .filter(|(_, global)| global.space == naga::AddressSpace::WorkGroup)
        .collect();
    assert_eq!(shared.len(), 3);
    assert!(shared.iter().all(|(_, global)| global.ty == shared[0].1.ty));
    for body in [
        "{ __shared__ float3 value; } value.x = 1;",
        "for (__shared__ float3 value; false;) {} value.x = 1;",
        "__shared__ float3 value; __shared__ float3 value;",
    ] {
        rejected(&format!("__global__ void kernel() {{ {body} }}"));
    }
}

#[test]
fn cuda_shared_vector_fields_copies_helpers_and_selected_references_validate() {
    for scalar in ["float", "int", "uint"] {
        for (width, last) in [(2, "y"), (3, "z"), (4, "w")] {
            let kind = format!("{scalar}{width}");
            let arguments = vec!["1"; width].join(", ");
            validate(&format!(
                "__device__ {kind} alter({kind} value) {{ value.x += 2; return value; }}
                __global__ void kernel(int *out) {{
                    __shared__ {kind} value;
                    __shared__ {kind} values[2];
                    value = make_{kind}({arguments}); values[0] = value;
                    {kind} local = alter(values[0]);
                    (true ? values[0] : local).{last} += 3;
                    ((false ? values[1] : value) = alter(local)).x++;
                    values[1] = alter(value); out[0] = int(values[1].{last});
                }}"
            ));
        }
    }
}

#[test]
fn cuda_shared_vectors_reject_const_types_including_aliases() {
    for declaration in [
        "__shared__ const float3 value;",
        "__shared__ float3 const values[2];",
        "__shared__ Frozen value;",
        "__shared__ Frozen values[2];",
    ] {
        let source = format!(
            "using Frozen = const float3;
            __global__ void kernel() {{ __shared__ float3 valid; {declaration} }}"
        );
        assert!(rejected(&source).message.contains("const"));
    }
}

#[test]
fn cuda_shared_vectors_reject_initializers_and_unsupported_declarations() {
    for declaration in [
        "__shared__ bool value;",
        "__shared__ double value;",
        "__shared__ float3 *value;",
        "__shared__ float3 value = make_float3(1, 2, 3);",
        "__shared__ float3 value{};",
        "__shared__ float3 value = {};",
        "__shared__ float3 values[2]{};",
        "__shared__ float3 values[2] = {};",
        "__shared__ float3 values[2][2];",
        "__shared__ float3 values[];",
        "extern __shared__ float3 values[];",
        "__shared__ float3 first, second;",
        "__shared__ __shared__ float3 value;",
    ] {
        rejected(&format!(
            "__global__ void kernel() {{ __shared__ float3 valid; {declaration} }}"
        ));
    }
    for source in [
        "__shared__ float3 value; __global__ void kernel() {}",
        "__global__ void kernel(__shared__ float3 *value) {}",
        "__device__ float3 helper() { __shared__ float3 value; return value; }
         __global__ void kernel() {}",
        "__device__ float3 helper(float3 *value) { return value[0]; }
         __global__ void kernel() { __shared__ float3 value; helper(value); }",
        "__global__ void kernel(float3 value) { __shared__ float3 valid; }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_shared_vector_array_extent_errors_preserve_spans_and_size_limits() {
    for extent in [
        "0",
        "0u",
        "4294967295u",
        "4294967296u",
        "1.0f",
        "size",
        "08",
    ] {
        let source = format!(
            "__global__ void kernel(unsigned size) {{ __shared__ float3 values[{extent}]; }}"
        );
        assert_eq!(&source[rejected(&source).span.to_range().unwrap()], extent);
    }
    for (width, stride) in [(2, 8), (3, 12), (4, 16)] {
        let count = naga::valid::MAX_TYPE_SIZE / stride;
        let (module, _) = validate(&format!(
            "__global__ void kernel() {{ __shared__ float{width} values[{count}u]; }}"
        ));
        let (_, global) = module.global_variables.iter().next().unwrap();
        assert_eq!(
            module.types[global.ty].inner.try_size(module.to_ctx()),
            Some(count * stride)
        );
        let source = format!(
            "__global__ void kernel() {{ __shared__ float{width} values[{}u]; }}",
            count + 1
        );
        assert!(rejected(&source).message.contains("size limit"));
    }
}

#[test]
fn cuda_shared_vectors_keep_vector_operation_and_array_reference_gates() {
    for operation in [
        "value = 1;",
        "value = make_int3(1, 2, 3);",
        "value.xy;",
        "value[0];",
        "value + value;",
        "values = value;",
        "values[true] = value;",
        "value.w = 1;",
    ] {
        rejected(&format!(
            "__global__ void kernel() {{
                __shared__ float3 value; __shared__ float3 values[2]; {operation}
            }}"
        ));
    }
}

#[test]
fn cuda_shared_vectors_do_not_relax_barrier_placement_restrictions() {
    for body in [
        "if (threadIdx.x == 0u) __syncthreads();",
        "for (int i = 0; i < 1; ++i) __syncthreads();",
        "if (threadIdx.x == 0u) return; __syncthreads();",
    ] {
        let source = format!("__global__ void kernel() {{ __shared__ float3 values[8]; {body} }}");
        let error = rejected(&source);
        assert!(error.message.contains("straight-line"), "{error}");
        assert_eq!(&source[error.span.to_range().unwrap()], "__syncthreads();");
    }
}

#[test]
fn cuda_shared_vector_fixture_roundtrips_through_wgsl() {
    let (module, _info) = validate_with_options(
        include_str!("../../in/cuda/shared-vectors.cu"),
        &Options {
            workgroup_size: [8, 1, 1],
        },
    );
    assert_eq!(module.entry_points[0].workgroup_size, [8, 1, 1]);
    #[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
    super::helpers::wgsl_roundtrip(
        &module,
        &_info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}
