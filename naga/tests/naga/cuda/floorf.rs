use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

fn named_call_count(module: &naga::Module, function: &naga::Function, name: &str) -> usize {
    function
        .body
        .iter()
        .filter(|statement| matches!(statement, naga::Statement::Call { function, .. } if module.functions[*function].name.as_deref() == Some(name)))
        .count()
}

#[test]
fn cuda_floorf_converts_scalar_arguments_and_returns_float() {
    for declaration in [
        "bool value = true",
        "int value = 9",
        "unsigned value = 16u",
        "float value = 6.25f",
        "using Sample = const float; Sample value = 2.0f",
    ] {
        let (module, info) = validate(&format!(
            "__global__ void kernel(float *out) {{ {declaration}; out[0] = floorf(value); }}"
        ));
        let function = &module.entry_points[0].function;
        let result = function
            .body
            .iter()
            .rev()
            .find_map(|statement| match *statement {
                naga::Statement::Store { value, .. } => Some(value),
                _ => None,
            })
            .unwrap();
        assert_eq!(
            info.get_entry_point(0)[result].ty.inner_with(&module.types),
            &naga::TypeInner::Scalar(naga::Scalar::F32)
        );
    }
}

#[test]
fn cuda_floorf_uses_integer_math_without_native_floor_or_double_types() {
    let (module, _) =
        validate("__global__ void kernel(float *out, float value) { out[0] = floorf(value); }");
    for function in module
        .functions
        .iter()
        .map(|(_, function)| function)
        .chain(module.entry_points.iter().map(|entry| &entry.function))
    {
        assert!(!function.expressions.iter().any(|(_, expression)| matches!(
            expression,
            naga::Expression::Math {
                fun: naga::MathFunction::Floor,
                ..
            }
        )));
    }
    assert!(!module.types.iter().any(|(_, ty)| matches!(
        ty.inner,
        naga::TypeInner::Scalar(naga::Scalar {
            kind: naga::ScalarKind::Float,
            width: 8
        }) | naga::TypeInner::Vector {
            scalar: naga::Scalar {
                kind: naga::ScalarKind::Float,
                width: 8
            },
            ..
        }
    )));
}

#[test]
fn cuda_floorf_preserves_pending_arrays_and_completed_constant_bounds() {
    validate(
        "__global__ void kernel(float *out) {
            float values[]{floorf(4.75f), (values[0], 2)};
            constexpr int Count = (values[1], 2);
            int scratch[Count]{};
            out[0] = values[0] + values[1] + float(scratch[1]);
            for (int index = 0; index < 2; ++index) {
                float repeated[]{floorf(-1.25f), (repeated[0], 3)};
                out[1] = repeated[0] + repeated[1];
            }
        }",
    );
}

#[test]
fn cuda_floorf_converts_integer_arguments_before_bit_classification() {
    for parameter in ["int", "unsigned"] {
        let (module, _) = validate(&format!(
            "__global__ void kernel(float *out, {parameter} value) {{ out[0] = floorf(value); }}"
        ));
        let function = &module.entry_points[0].function;
        assert!(function.expressions.iter().any(|(_, expression)| matches!(
            expression,
            naga::Expression::As {
                kind: naga::ScalarKind::Float,
                convert: Some(4),
                ..
            }
        )));
    }
}

#[test]
fn cuda_floorf_preserves_aliases_helpers_and_resource_bindings() {
    let (module, _) = validate(
        "using Sample = const float;
        __device__ Sample rounded(Sample value) { return floorf(value); }
        __host__ __device__ float twice(float value) { return floorf(value) * 2.0f; }
        __device__ void consume(float value) { if (floorf(value)) return; }
        __global__ void kernel(const float *input, float scale, unsigned *out) {
            consume(input[0]); out[0] = __float_as_uint(rounded(input[0]) * scale);
            out[1] = __float_as_uint(twice(input[1]));
        }",
    );
    let resources: Vec<_> = module
        .global_variables
        .iter()
        .map(|(_, global)| global)
        .collect();
    assert_eq!(resources.len(), 3);
    for (index, resource) in resources.iter().enumerate() {
        assert_eq!(
            resource.binding,
            Some(naga::ResourceBinding {
                group: 0,
                binding: index as u32
            })
        );
    }
    assert_eq!(
        resources[0].space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD
        }
    );
    assert_eq!(resources[1].space, naga::AddressSpace::Uniform);
    assert_eq!(
        resources[2].space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE
        }
    );
}

#[test]
fn cuda_floorf_merges_user_overloads_and_preserves_exact_builtin_matches() {
    let (module, _) = validate(
        "__device__ int floorf(bool value) { return value ? 7 : 9; }
        __device__ int floorf(int value) { return value + 10; }
        __device__ int floorf(int first, int second) { return first + second; }
        __global__ void kernel(float *out) {
            out[0] = floorf(true); out[1] = floorf(3); out[2] = floorf(4.0f); out[3] = floorf(3, 4);
        }",
    );
    assert_eq!(
        named_call_count(&module, &module.entry_points[0].function, "floorf"),
        3
    );
    validate(
        "__device__ float2 floorf(float2 value) {
            return make_float2(floorf(value.x), floorf(value.y));
        }
        __global__ void kernel(float *out) {
            out[0] = floorf(make_float2(1.75f, -1.25f)).x;
        }",
    );
    let source = "__device__ int floorf(int value) { return value; }
        __global__ void kernel() { floorf(1u); }";
    let error = rejected(source);
    assert!(error.message.contains("ambiguous"));
    assert_eq!(&source[error.span.to_range().unwrap()], "floorf");
}

#[test]
fn cuda_floorf_distinguishes_current_overload_recursion_from_builtin_calls() {
    validate(
        "__device__ float floorf(int value) { return floorf(float(value)); }
        __global__ void kernel(float *out) { out[0] = floorf(9); }",
    );
    let source = "__device__ float floorf(int value) { return floorf(value); }
        __global__ void kernel() {}";
    let error = rejected(source);
    assert!(error.message.contains("recurs"));
    assert_eq!(&source[error.span.to_range().unwrap()], "floorf");
}

#[test]
fn cuda_floorf_rejects_canonical_builtin_redefinitions() {
    for result in ["float", "int", "void"] {
        let body = if result == "void" { "" } else { "return 0;" };
        let source = format!(
            "using Sample = const float; __device__ {result} floorf(Sample value) {{ {body} }}
            __global__ void kernel() {{}}"
        );
        let error = rejected(&source);
        assert!(error.message.contains("duplicate"));
        assert_eq!(&source[error.span.to_range().unwrap()], "floorf");
    }
    for source in [
        "using floorf = int; __global__ void kernel() {}",
        "__global__ void floorf() {}",
    ] {
        assert_eq!(&source[rejected(source).span.to_range().unwrap()], "floorf");
    }
}

#[test]
fn cuda_floorf_respects_local_value_parameter_and_type_hiding() {
    for source in [
        "__global__ void kernel() { int floorf = 1; floorf(1.0f); }",
        "__device__ float helper(int floorf) { return floorf(1.0f); } __global__ void kernel() {}",
    ] {
        assert_eq!(&source[rejected(source).span.to_range().unwrap()], "floorf");
    }
    validate(
        "__global__ void kernel(float *out) {
            { using floorf = int; out[0] = floorf(2.5f); }
            out[1] = floorf(4.0f);
        }",
    );
}

#[test]
fn cuda_floorf_requires_one_argument_and_reports_inner_void_calls() {
    for arguments in ["", "1, 2"] {
        let source = format!("__global__ void kernel() {{ floorf({arguments}); }}");
        let error = rejected(&source);
        assert!(error.message.contains("no matching"));
        assert_eq!(&source[error.span.to_range().unwrap()], "floorf");
    }
    rejected("__global__ void kernel() { floorf(1,); }");
    let source = "__device__ void empty() {} __global__ void kernel() { floorf(empty()); }";
    assert_eq!(
        &source[rejected(source).span.to_range().unwrap()],
        "empty()"
    );
}

#[test]
fn cuda_floorf_rejects_vectors_arrays_pointers_and_unsupported_variants() {
    for scalar in ["int", "uint", "float"] {
        for width in [2, 3, 4] {
            let source =
                format!("__global__ void kernel() {{ {scalar}{width} value{{}}; floorf(value); }}");
            let error = rejected(&source);
            assert!(error.message.contains("no matching"));
            assert_eq!(&source[error.span.to_range().unwrap()], "floorf");
        }
    }
    for source in [
        "__global__ void kernel(float *out) { floorf(out); }",
        "__global__ void kernel() { float values[2]{}; floorf(values); }",
        "__global__ void kernel() { floorf(4.0); }",
        "__global__ void kernel() { floor(4.0f); }",
        "__global__ void kernel() { __floorf(4.0f); }",
        "__global__ void kernel() { floorf(missing); }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_floorf_result_is_a_prvalue_and_dead_source_is_checked() {
    validate("__global__ void kernel() { return; floorf(-1.0f); }");
    for statement in [
        "floorf(4.0f) = 3.0f;",
        "++floorf(4.0f);",
        "floorf(4.0f)++;",
        "return; floorf(missing);",
        "return; floorf(1.0f, 2.0f);",
    ] {
        rejected(&format!(
            "/* π */ __global__ void kernel() {{ {statement} }}"
        ));
    }
}

#[test]
fn cuda_floorf_snapshots_helper_arguments_once_before_later_arguments() {
    let (module, _) = validate(
        "__device__ float next(float value) { return value; }
        __device__ float pair(float first, float second) { return first * 10.0f + second; }
        __global__ void kernel(float *out) {
            float value = 4.0f; out[0] = pair(floorf(next(value++)), value++); out[1] = value;
        }",
    );
    let function = &module.entry_points[0].function;
    assert_eq!(named_call_count(&module, function, "next"), 1);
    assert_eq!(named_call_count(&module, function, "pair"), 1);
}

#[test]
fn cuda_floorf_stays_in_selected_control_flow() {
    validate(
        "__global__ void kernel(float *out, int choose) {
            float value = 4.0f;
            out[0] = choose ? floorf(value++) : floorf(value++);
            bool active = choose && floorf(value++); out[1] = active;
            out[2] = value;
            for (int index = 0; index < 2; ++index) {
                if (index == 0) continue;
                out[3] = floorf(value++);
            }
        }",
    );
}

#[test]
fn cuda_floorf_is_not_a_source_constant_expression() {
    for source in [
        "constexpr float Rounded = floorf(4.0f); __global__ void kernel() {}",
        "__global__ void kernel() { constexpr float rounded = floorf(4.0f); }",
        "__global__ void kernel() { const int count = int(floorf(4.0f)); int values[count]; }",
        "__global__ void kernel() { constexpr int rounded = (floorf(4.0f), 3); }",
    ] {
        rejected(source);
    }
    validate(
        "constexpr float Rounded = true ? 9.0f : floorf(-1.0f);
        __global__ void kernel(int *out) {
            constexpr int Count = false && floorf(-1.0f) ? 2 : 3;
            int values[Count]{}; out[0] = int(Rounded) + values[0];
        }",
    );
}

#[test]
fn cuda_floorf_bounds_source_depth_and_keeps_flat_call_lowering_linear() {
    let mut expression = String::from("value");
    for _ in 0..8 {
        expression = format!("floorf({expression})");
    }
    validate(&format!(
        "__global__ void kernel(float *out, float value) {{ out[0] = {expression}; }}"
    ));
    for _ in 0..80 {
        expression = format!("floorf({expression})");
    }
    let source =
        format!("__global__ void kernel(float *out, float value) {{ out[0] = {expression}; }}");
    let error = rejected(&source);
    assert!(error.message.contains("depth") || error.message.contains("nesting"));
    let (single, _) =
        validate("__global__ void kernel(float *out, float value) { out[0] = floorf(value); }");
    let statements = "out[0] = floorf(value);".repeat(128);
    let (many, _) = validate(&format!(
        "__global__ void kernel(float *out, float value) {{ {statements} }}"
    ));
    assert!(
        many.entry_points[0].function.expressions.len()
            <= single.entry_points[0].function.expressions.len() * 128 + 256
    );
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_floorf_literal_exceptional_results_remain_runtime_values_in_wgsl() {
    let (module, info) = validate(
        "__global__ void kernel(unsigned *out) {
            out[0] = __float_as_uint(floorf(-1.0f));
            out[1] = __float_as_uint(floorf(-0.0f));
            out[2] = __float_as_uint(floorf(__uint_as_float(0x7f800000u)));
            out[3] = __float_as_uint(floorf(__uint_as_float(0x7f800001u)));
            out[4] = __float_as_uint(floorf(__uint_as_float(0xff800000u)));
            out[5] = __float_as_uint(floorf(3.402823466e38f));
            out[6] = __float_as_uint(floorf(-3.402823466e38f));
            out[7] = __float_as_uint(floorf(1e10f));
        }",
    );
    let function = &module.entry_points[0].function;
    assert!(function.expressions.iter().any(|(_, expression)| matches!(
        expression,
        naga::Expression::As {
            kind: naga::ScalarKind::Float,
            convert: None,
            ..
        }
    )));
    for (_, expression) in function.expressions.iter() {
        if let naga::Expression::As {
            expr,
            kind: naga::ScalarKind::Float,
            convert: None,
        } = *expression
        {
            assert!(!matches!(
                function.expressions[expr],
                naga::Expression::Literal(_)
                    | naga::Expression::Constant(_)
                    | naga::Expression::ZeroValue(_)
            ));
        }
    }
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_floorf_fixture_roundtrips_through_the_shared_wgsl_helper() {
    let (module, info) = validate(include_str!("../../in/cuda/floorf.cu"));
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}
