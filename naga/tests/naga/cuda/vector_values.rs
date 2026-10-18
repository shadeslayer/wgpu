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
fn cuda_vector_values_have_cuda_sizes_alignments_and_array_strides() {
    for (name, scalar) in [
        ("float", naga::Scalar::F32),
        ("int", naga::Scalar::I32),
        ("uint", naga::Scalar::U32),
    ] {
        for (width, size, alignment) in [(2, 8, 8), (3, 12, 4), (4, 16, 16)] {
            let (module, _) = validate(&format!(
                "__global__ void kernel() {{ {name}{width} values[2]; }}"
            ));
            let function = &module.entry_points[0].function;
            let local = function.local_variables.iter().next().unwrap().1;
            let naga::TypeInner::Array {
                base,
                size: naga::ArraySize::Constant(count),
                stride,
            } = module.types[local.ty].inner
            else {
                panic!("expected a fixed vector array");
            };
            let mut layouter = naga::proc::Layouter::default();
            layouter.update(module.to_ctx()).unwrap();
            assert_eq!(count.get(), 2);
            assert_eq!(stride, size);
            assert_eq!(layouter[base].size, size);
            assert_eq!(
                layouter[base].alignment,
                naga::proc::Alignment::new(alignment).unwrap()
            );
            assert_eq!(layouter[local.ty].size, size * 2);
            match &module.types[base].inner {
                naga::TypeInner::Vector {
                    size,
                    scalar: actual,
                } => {
                    assert_eq!(*size as u32, width);
                    assert_eq!(*actual, scalar);
                }
                naga::TypeInner::Struct { members, .. } => {
                    assert_eq!(members.len(), width as usize);
                    for member in members {
                        assert_eq!(
                            module.types[member.ty].inner,
                            naga::TypeInner::Scalar(scalar)
                        );
                    }
                }
                _ => panic!("expected a vector or scalar-field struct"),
            }
            assert!(module.global_variables.is_empty());
        }
    }
}

#[test]
fn cuda_vector_aliases_are_canonical_and_builtin_names_can_be_hidden() {
    let (module, _) = validate(
        "using Triple = float3; using Frozen = const Triple;
        __global__ void kernel(int *out) {
            float3 original = make_float3(1, 2, 3);
            Triple copied = original;
            Frozen frozen = copied;
            { using float3 = int; float3 scalar = 4; out[0] = scalar; }
            int float3 = 5; out[1] = float3;
            copied.x = 9; out[2] = int(frozen.x);
        }",
    );
    let types: Vec<_> = module.entry_points[0]
        .function
        .local_variables
        .iter()
        .take(3)
        .map(|(_, local)| local.ty)
        .collect();
    assert_eq!(types[0], types[1]);
    assert_eq!(types[1], types[2]);
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            rejected(&format!(
                "using {scalar}{width} = int; __global__ void kernel() {{}}"
            ));
        }
    }
}

#[test]
fn cuda_vector_fields_are_scalar_values_and_writable_references() {
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            let arguments = vec!["1"; width].join(", ");
            let fields = ["x", "y", "z", "w"];
            for field in fields.iter().take(width) {
                validate(&format!(
                    "__global__ void kernel(int *out) {{
                        {scalar}{width} value = make_{scalar}{width}({arguments});
                        value.{field} = 3; ++value.{field}; value.{field} += 2;
                        out[0] = int(value.{field}++);
                    }}"
                ));
            }
            for field in fields.iter().skip(width) {
                let source = format!(
                    "__global__ void kernel() {{ {scalar}{width} value = make_{scalar}{width}({arguments}); value.{field}; }}"
                );
                assert_eq!(&source[rejected(&source).span.to_range().unwrap()], *field);
            }
        }
    }
}

#[test]
fn cuda_vector_empty_initializers_zero_each_declaration() {
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            for initializer in ["{}", "= {}"] {
                validate(&format!(
                    "__global__ void kernel(int *out) {{
                        const {scalar}{width} zero {initializer};
                        out[0] = int(zero.x + zero.y);
                        for (int i = 0; i < 3; ++i) {{
                            {scalar}{width} reset {initializer};
                            out[1] += int(reset.x); reset.x = 9;
                        }}
                    }}"
                ));
            }
        }
    }
}

#[test]
fn cuda_vector_const_qualification_reaches_fields_and_array_elements() {
    validate(
        "using Frozen = const float3;
        __global__ void kernel(int *out) {
            Frozen value = make_float3(1, 2, 3);
            float3 copy = value; copy.x = 4; out[0] = int(value.x + copy.x);
            Frozen values[2]{}; out[1] = int(values[1].z);
        }",
    );
    for operation in [
        "value.x = 1;",
        "value.y++;",
        "++value.z;",
        "value = make_float3(4, 5, 6);",
        "values[0].x += 1;",
    ] {
        let source = format!(
            "__global__ void kernel() {{ const float3 value = make_float3(1, 2, 3); const float3 values[2]{{}}; {operation} }}"
        );
        assert!(rejected(&source).message.contains("const"));
    }
}

#[test]
fn cuda_vector_helpers_copy_parameters_and_return_typed_values() {
    let (module, _) = validate(
        "using Triple = float3;
        __device__ Triple alter(Triple value) { value.x += 2; return value; }
        __device__ int3 identity(const int3 value) { return value; }
        __global__ void kernel(int *out) {
            Triple original = make_float3(1, 2, 3);
            Triple changed = alter(original);
            out[0] = int(original.x * 10 + changed.x);
            out[1] = int(alter(original).x);
            out[2] = identity(make_int3(4, 5, 6)).z;
        }",
    );
    for (_, function) in module.functions.iter() {
        assert_eq!(
            function.arguments[0].ty,
            function.result.as_ref().unwrap().ty
        );
    }
    rejected(
        "__device__ int3 make() { return make_int3(1, 2, 3); }
        __global__ void kernel() { make().x = 7; }",
    );
}

#[test]
fn cuda_vector_overloads_require_exact_vector_parameter_types() {
    validate(
        "__device__ int pick(int2 value) { return value.x; }
        __device__ int pick(int3 value) { return value.y; }
        __device__ int pick(uint3 value) { return int(value.z); }
        __device__ int pick(float3 value) { return int(value.x); }
        __global__ void kernel(int *out) {
            out[0] = pick(make_int2(1, 2)); out[1] = pick(make_int3(3, 4, 5));
            out[2] = pick(make_uint3(6, 7, 8)); out[3] = pick(make_float3(9, 10, 11));
        }",
    );
    for argument in [
        "make_uint3(1, 2, 3)",
        "make_float3(1, 2, 3)",
        "make_int2(1, 2)",
        "1",
    ] {
        rejected(&format!(
            "__device__ int pick(int3 value) {{ return value.x; }}
            __global__ void kernel(int *out) {{ out[0] = pick({argument}); }}"
        ));
    }
    rejected(
        "__device__ float3 wrong() { return make_int3(1, 2, 3); }
        __global__ void kernel() { wrong(); }",
    );
}

#[test]
fn cuda_vector_make_functions_convert_each_scalar_argument() {
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            let arguments = ["true", "2u", "3.5f", "-4"]
                .into_iter()
                .take(width)
                .collect::<Vec<_>>()
                .join(", ");
            validate(&format!(
                "__global__ void kernel(int *out) {{
                    {scalar}{width} value = make_{scalar}{width}({arguments});
                    out[0] = int(value.x + value.y);
                }}"
            ));
        }
    }
}

#[test]
fn cuda_vector_make_functions_merge_extra_arities_and_reject_duplicate_signatures() {
    validate(
        "__device__ int3 make_int3(int value) { return make_int3(value, 2, 3); }
        __global__ void kernel(int *out) {
            out[0] = make_int3(7).x; out[1] = make_int3(4, 5, 6).z;
        }",
    );
    rejected(
        "__device__ int3 make_int3(int x, int y, int z) { return make_int3(x, y, z); }
        __global__ void kernel() {}",
    );
    for arguments in [
        "",
        "1",
        "1, 2",
        "1, 2, 3, 4",
        "make_int2(1, 2), 3",
        "make_int3(1, 2, 3)",
    ] {
        rejected(&format!(
            "__global__ void kernel() {{ make_int3({arguments}); }}"
        ));
    }
}

#[test]
fn cuda_vector_values_do_not_change_kernel_resource_abi() {
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            for parameter in [
                format!("{scalar}{width} value"),
                format!("{scalar}{width} *values"),
                format!("const {scalar}{width} *values"),
            ] {
                rejected(&format!("__global__ void kernel({parameter}) {{}}"));
            }
        }
    }
}

#[test]
fn cuda_vector_values_reject_implicit_struct_operators() {
    validate("__global__ void kernel() { int3 value = make_int3(1, 2, 3); value.x += 1; }");
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            let arguments = vec!["1"; width].join(", ");
            for expression in [
                "value + value",
                "value - value",
                "value * value",
                "value / value",
                "value == value",
                "value < value",
                "-value",
                "+value",
                "!value",
                "~value",
                "value++",
                "++value",
                "value += value",
                "value && true",
                "value ? 1 : 2",
            ] {
                rejected(&format!(
                    "__global__ void kernel() {{ {scalar}{width} value = make_{scalar}{width}({arguments}); {expression}; }}"
                ));
            }
        }
    }
}

#[test]
fn cuda_vector_values_reject_unsupported_indexing_swizzles_and_casts() {
    validate("__global__ void kernel() { int3 values[2]; values[0] = make_int3(1, 2, 3); values[0].x = 4; }");
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            let arguments = vec!["1"; width].join(", ");
            for expression in [
                String::from("value[0]"),
                String::from("value.xy"),
                String::from("value.r"),
                String::from("value.rgba"),
                format!("({scalar}{width})value"),
                format!("{scalar}{width}(value)"),
                format!("static_cast<{scalar}{width}>(value)"),
                String::from("(float3)value"),
                String::from("float3(value)"),
                String::from("static_cast<uint3>(value)"),
                String::from("(int2)value"),
                String::from("int3(1)"),
                String::from("int(value)"),
                String::from("make_int3(value, 1, 2)"),
            ] {
                rejected(&format!(
                    "__global__ void kernel() {{ {scalar}{width} value = make_{scalar}{width}({arguments}); {expression}; }}"
                ));
            }
        }
    }
    rejected("__global__ void kernel() { int3 value = make_int3(1, 2, 3); float3 other = value; }");
}

#[test]
fn cuda_vector_conditional_fields_preserve_selected_reference_identity() {
    validate(
        "__global__ void kernel(int *out) {
            int3 left = make_int3(1, 2, 3), right = make_int3(4, 5, 6);
            bool choose = true;
            out[0] = ((choose ? left : right).x = (choose = false, 9));
            out[1] = ((choose ? left : right) = left).z++;
            out[2] = (true ? make_int3(7, 8, 9) : left).y;
        }",
    );
    rejected(
        "__global__ void kernel() {
            int3 value = make_int3(1, 2, 3);
            (true ? make_int3(4, 5, 6) : value).x = 7;
        }",
    );
}

#[test]
fn cuda_vector_array_fields_accept_rhs_first_updates_and_nested_projection() {
    validate(
        "__global__ void kernel(int *out) {
            int3 values[3]; values[0] = make_int3(1, 2, 3);
            values[1] = make_int3(4, 5, 6); values[2] = make_int3(7, 8, 9);
            int index = 0;
            out[0] = (values[index++].x += ++index);
            bool choose = false;
            out[1] = (choose ? values[0] : (choose ? values[1] : values[index++])).z++;
            out[2] = index;
        }",
    );
}

#[test]
fn cuda_vector_discarded_field_references_do_not_load_uninitialized_elements() {
    let (module, _) = validate(
        "__global__ void kernel() {
            float3 values[2];
            (true ? values[0] : values[1]).x;
        }",
    );
    assert!(!module.entry_points[0]
        .function
        .expressions
        .iter()
        .any(|(_, expression)| matches!(expression, naga::Expression::Load { .. })));
}

#[test]
fn cuda_vector_compose_and_projection_depth_is_bounded() {
    let flat = core::iter::repeat_n("1.0f", 63)
        .collect::<Vec<_>>()
        .join(" + ");
    validate(&format!(
        "__global__ void kernel(float *out) {{ out[0] = make_float4(0.0f, 0.0f, 0.0f, {flat}).w; }}"
    ));
    let flat = core::iter::repeat_n("1.0f", 64)
        .collect::<Vec<_>>()
        .join(" + ");
    let error = rejected(&format!(
        "__global__ void kernel(float *out) {{ out[0] = make_float4(0.0f, 0.0f, 0.0f, {flat}).w; }}"
    ));
    assert!(error.message.contains("graph depth"));
    let mut expression = String::from("1.0f");
    for _ in 0..8 {
        expression = format!("make_float3({expression}, 2.0f, 3.0f).x");
    }
    validate(&format!(
        "__global__ void kernel(float *out) {{ out[0] = {expression}; }}"
    ));
    for _ in 0..80 {
        expression = format!("make_float3({expression}, 2.0f, 3.0f).x");
    }
    let error = rejected(&format!(
        "__global__ void kernel(float *out) {{ out[0] = {expression}; }}"
    ));
    assert!(error.message.contains("depth") || error.message.contains("nesting"));
}

#[cfg(all(feature = "wgsl-out", feature = "wgsl-in"))]
#[test]
fn cuda_vector_fixture_preserves_scalar_storage_and_roundtrips_through_wgsl() {
    let (module, info) = validate(include_str!("../../in/cuda/vector-values.cu"));
    assert_eq!(module.global_variables.len(), 1);
    let global = module.global_variables.iter().next().unwrap().1;
    let naga::TypeInner::Array {
        base,
        size: naga::ArraySize::Dynamic,
        stride: 4,
    } = module.types[global.ty].inner
    else {
        panic!("vector values must retain the scalar output ABI");
    };
    assert_eq!(
        module.types[base].inner,
        naga::TypeInner::Scalar(naga::Scalar::I32)
    );
    assert_eq!(
        global.binding,
        Some(naga::ResourceBinding {
            group: 0,
            binding: 0
        })
    );
    let output = naga::back::wgsl::write_string(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    )
    .unwrap();
    let reparsed = naga::front::wgsl::parse_str(&output).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&reparsed)
    .unwrap();
}
