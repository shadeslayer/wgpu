use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

#[test]
fn cuda_vector_resources_preserve_all_nine_element_layouts_and_access_modes() {
    for (name, scalar) in [
        ("float", naga::Scalar::F32),
        ("int", naga::Scalar::I32),
        ("uint", naga::Scalar::U32),
    ] {
        for (width, stride, alignment) in [(2, 8, 8), (3, 12, 4), (4, 16, 16)] {
            for qualifier in ["", "const "] {
                let (module, _) = validate(&format!(
                    "__global__ void kernel({qualifier}{name}{width} *values) {{}}"
                ));
                let global = module.global_variables.iter().next().unwrap().1;
                let access = if qualifier.is_empty() {
                    naga::StorageAccess::LOAD | naga::StorageAccess::STORE
                } else {
                    naga::StorageAccess::LOAD
                };
                assert_eq!(global.space, naga::AddressSpace::Storage { access });
            }
            let (module, _) = validate(&format!(
                "__global__ void copy(const {name}{width} *input, {name}{width} *output) {{
                    output[1] = input[2]; output[0].x = input[0].x;
                }}"
            ));
            assert_eq!(module.global_variables.len(), 2);
            let mut layouter = naga::proc::Layouter::default();
            layouter.update(module.to_ctx()).unwrap();
            for (binding, (_, global)) in module.global_variables.iter().enumerate() {
                assert_eq!(
                    global.binding,
                    Some(naga::ResourceBinding {
                        group: 0,
                        binding: binding as u32,
                    })
                );
                let access = if binding == 0 {
                    naga::StorageAccess::LOAD
                } else {
                    naga::StorageAccess::LOAD | naga::StorageAccess::STORE
                };
                assert_eq!(global.space, naga::AddressSpace::Storage { access });
                let naga::TypeInner::Array {
                    base,
                    size: naga::ArraySize::Dynamic,
                    stride: actual_stride,
                } = module.types[global.ty].inner
                else {
                    panic!("expected a runtime vector array");
                };
                assert_eq!(actual_stride, stride);
                assert_eq!(layouter[base].size, stride);
                assert_eq!(
                    layouter[base].alignment,
                    naga::proc::Alignment::new(alignment).unwrap()
                );
                match &module.types[base].inner {
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
                    _ => panic!("expected vector elements or packed scalar-field structs"),
                }
            }
        }
    }
}

#[test]
fn cuda_vector_resources_keep_mixed_scalar_uniforms_and_source_binding_order() {
    let (module, _) = validate(
        "using Triple = float3; using Frozen = const Triple;
        __global__ void mixed(unsigned n, Frozen *input, int offset, Triple *output, float factor) {
            output[offset] = input[n]; output[offset].x *= factor;
        }",
    );
    assert_eq!(module.global_variables.len(), 5);
    for (index, (_, global)) in module.global_variables.iter().enumerate() {
        assert_eq!(
            global.binding,
            Some(naga::ResourceBinding {
                group: 0,
                binding: index as u32,
            })
        );
        let scalar = match index {
            0 => naga::Scalar::U32,
            2 => naga::Scalar::I32,
            4 => naga::Scalar::F32,
            1 | 3 => {
                let access = if index == 1 {
                    naga::StorageAccess::LOAD
                } else {
                    naga::StorageAccess::LOAD | naga::StorageAccess::STORE
                };
                assert_eq!(global.space, naga::AddressSpace::Storage { access });
                continue;
            }
            _ => unreachable!(),
        };
        assert_eq!(global.space, naga::AddressSpace::Uniform);
        let naga::TypeInner::Struct { members, span } = &module.types[global.ty].inner else {
            panic!("scalar parameters must remain uniform blocks");
        };
        assert_eq!(*span, 16);
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].offset, 0);
        assert_eq!(
            module.types[members[0].ty].inner,
            naga::TypeInner::Scalar(scalar)
        );
    }
}

#[test]
fn cuda_vector_resources_accept_aliases_const_spellings_and_local_name_hiding() {
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            for input_type in ["const Value", "Value const", "Frozen"] {
                validate(&format!(
                    "using Value = {scalar}{width}; using Frozen = const Value;
                    __global__ void copy({input_type} *input, Value *output) {{
                        Value local = input[0]; output[0] = local;
                        {{ using {scalar}{width} = int; {scalar}{width} count = 2;
                           output[1].x = count; }}
                    }}"
                ));
            }
        }
    }
}

#[test]
fn cuda_vector_resource_fields_support_scalar_updates_for_all_widths() {
    for scalar in ["float", "int", "uint"] {
        for (width, last) in [(2, "y"), (3, "z"), (4, "w")] {
            validate(&format!(
                "__global__ void fields(const {scalar}{width} *input, {scalar}{width} *output) {{
                    output[0] = input[0]; output[0].x += 2;
                    output[1] = input[1]; ++output[1].{last};
                    output[2] = input[2]; output[2].y = output[0].x++;
                }}"
            ));
        }
    }
}

#[test]
fn cuda_vector_resource_loads_and_helper_results_keep_exact_vector_types() {
    for name in [
        "float2", "float3", "float4", "int2", "int3", "int4", "uint2", "uint3", "uint4",
    ] {
        validate(&format!(
            "__device__ {name} alter({name} value) {{ value.x += 1; return value; }}
            __device__ {name} second({name} first, {name} last) {{ first.y = last.x; return first; }}
            __global__ void helpers(const {name} *input, {name} *output) {{
                {name} copied = input[0]; output[0] = alter(copied);
                output[1] = second(input[1], input[2]);
                output[2] = input[2]; output[2].y = alter(input[0]).x;
            }}"
        ));
    }
}

#[test]
fn cuda_vector_resource_references_preserve_assignment_identity_and_selected_indices() {
    validate(
        "__global__ void references(const int3 *input, int3 *output) {
            int3 local = input[0]; bool choose = true; int index = 0;
            output[index++].x = (index = 1, input[2].x);
            (choose ? output[0] : output[2]).y += 2;
            ((choose ? output[0] : local) = input[1]).x++;
            (choose ? (false ? local : output[index++]) : output[0]).z = index;
            output[3] = local;
        }",
    );
}

#[test]
fn cuda_vector_resources_reject_writes_through_const_elements_and_fields() {
    for operation in [
        "input[0] = output[0];",
        "input[0].x = 1;",
        "input[0].y++;",
        "++input[1].z;",
        "input[1].x += 2;",
        "(true ? input[0] : output[0]).x = 1;",
        "(false ? output[0] : input[0]) = output[1];",
    ] {
        let source = format!(
            "__global__ void readonly(const float3 *input, float3 *output) {{ {operation} }}"
        );
        let error = rejected(&source);
        assert!(
            error.message.contains("const") || error.message.contains("read-only"),
            "{error:?}"
        );
    }
}

#[test]
fn cuda_vector_resources_reject_implicit_element_type_and_width_conversions() {
    for (input, output) in [
        ("int3", "float3"),
        ("uint3", "int3"),
        ("float2", "float3"),
        ("int4", "int2"),
    ] {
        rejected(&format!(
            "__global__ void wrong(const {input} *input, {output} *output) {{ output[0] = input[0]; }}"
        ));
    }
    for expression in ["1", "make_float2(1, 2)", "make_int3(1, 2, 3)"] {
        rejected(&format!(
            "__global__ void wrong(float3 *output) {{ output[0] = {expression}; }}"
        ));
    }
}

#[test]
fn cuda_vector_resources_keep_general_pointer_and_kernel_value_forms_unsupported() {
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            rejected(&format!(
                "__global__ void kernel({scalar}{width} value) {{}}"
            ));
        }
    }
    for source in [
        "__global__ void bad(float3 **value) {}",
        "__global__ void bad(float3 &value) {}",
        "__global__ void bad(float3 value[2]) {}",
        "__global__ void bad(double2 *value) {}",
        "__device__ float3 bad(float3 *input) { return input[0]; } __global__ void kernel() {}",
        "__global__ void bad(const float3 *input, float3 *output) { output = input; }",
        "__global__ void bad(float3 *output) { output + 1; }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_vector_resource_elements_keep_vector_operators_swizzles_and_indexing_unsupported() {
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            for expression in [
                String::from("input[0] + input[1]"),
                String::from("-input[0]"),
                String::from("input[0].xy"),
                String::from("input[0][0]"),
                format!("{scalar}{width}(input[0])"),
            ] {
                rejected(&format!(
                    "__global__ void bad(const {scalar}{width} *input, {scalar}{width} *output) {{ {expression}; output[0] = input[0]; }}"
                ));
            }
        }
    }
}

#[test]
fn cuda_vector_resource_diagnostics_retain_utf8_safe_field_spans() {
    for name in ["float2", "float3", "float4"] {
        let source =
            format!("// λ\n__global__ void bad({name} *output) {{ output[0].missing = 1; }}");
        let error = rejected(&source);
        assert_eq!(&source[error.span.to_range().unwrap()], "missing");
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_vector_resource_fixture_roundtrips_through_wgsl() {
    let (module, info) = validate(include_str!("../../in/cuda/vector-resources.cu"));
    super::helpers::wgsl_roundtrip(&module, &info, naga::back::wgsl::WriterFlags::empty());
}
