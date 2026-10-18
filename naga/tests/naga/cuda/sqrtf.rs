use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const COLLIDING_HELPER_NAMES: &str = "__device__ int sqrtf_bits(int value) { return value + 1; }
        __device__ int _naga_sqrtf(int value) { return value + 2; }
        __global__ void kernel(float *out) {
            out[0] = sqrtf(4.0f); out[1] = sqrtf_bits(3) + _naga_sqrtf(4);
        }";

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
fn cuda_sqrtf_converts_scalar_arguments_and_returns_float() {
    for declaration in [
        "bool value = true",
        "int value = 9",
        "unsigned value = 16u",
        "float value = 6.25f",
        "using Sample = const float; Sample value = 2.0f",
    ] {
        let (module, info) = validate(&format!(
            "__global__ void kernel(float *out) {{ {declaration}; out[0] = sqrtf(value); }}"
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
fn cuda_sqrtf_uses_integer_math_without_native_sqrt_or_double_types() {
    let (module, _) =
        validate("__global__ void kernel(float *out, float value) { out[0] = sqrtf(value); }");
    for function in module
        .functions
        .iter()
        .map(|(_, function)| function)
        .chain(module.entry_points.iter().map(|entry| &entry.function))
    {
        assert!(!function.expressions.iter().any(|(_, expression)| matches!(
            expression,
            naga::Expression::Math {
                fun: naga::MathFunction::Sqrt,
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
fn cuda_sqrtf_caches_one_private_helper_before_source_callers() {
    let (module, _) = validate(
        "__device__ float first(float value) { return sqrtf(value); }
        __device__ float second(float value) { return sqrtf(value) + first(value); }
        __global__ void kernel(float *out, float value) {
            out[0] = first(value); out[1] = second(value); out[2] = sqrtf(value);
        }",
    );
    let private_helpers: Vec<_> = module
        .functions
        .iter()
        .filter(|(_, function)| !matches!(function.name.as_deref(), Some("first" | "second")))
        .collect();
    assert_eq!(private_helpers.len(), 1);
    let (helper_handle, helper) = private_helpers[0];
    assert_eq!(helper.arguments.len(), 1);
    assert_eq!(
        module.types[helper.arguments[0].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::U32)
    );
    assert_eq!(
        module.types[helper.result.as_ref().unwrap().ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::U32)
    );
    for (handle, function) in module.functions.iter() {
        if matches!(function.name.as_deref(), Some("first" | "second")) {
            assert!(helper_handle.index() < handle.index());
        }
    }
    let (independent, _) = validate("__global__ void kernel() {}");
    assert!(independent.functions.is_empty());
    validate(
        "__global__ void kernel(float *out) {
            float values[]{sqrtf(4.0f), (values[0], 2)};
            constexpr int Count = (values[1], 2);
            int scratch[Count]{}; out[0] = values[1] + float(scratch[1]);
        }",
    );
}

#[test]
fn cuda_sqrtf_private_helpers_do_not_claim_source_names() {
    validate(COLLIDING_HELPER_NAMES);
    let (module, _) =
        validate("__global__ void kernel(int *out) { using sqrtf = int; out[0] = sqrtf(2.5f); }");
    assert!(module.functions.is_empty());
}

#[test]
fn cuda_sqrtf_preserves_aliases_helpers_and_resource_bindings() {
    let (module, _) = validate(
        "using Sample = const float;
        __device__ Sample root(Sample value) { return sqrtf(value); }
        __host__ __device__ float twice(float value) { return sqrtf(value) * 2.0f; }
        __device__ void consume(float value) { if (sqrtf(value)) return; }
        __global__ void kernel(const float *input, float scale, unsigned *out) {
            consume(input[0]); out[0] = __float_as_uint(root(input[0]) * scale);
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
fn cuda_sqrtf_merges_user_overloads_and_preserves_exact_builtin_matches() {
    let (module, _) = validate(
        "__device__ int sqrtf(bool value) { return value ? 7 : 9; }
        __device__ int sqrtf(int value) { return value + 10; }
        __device__ int sqrtf(int first, int second) { return first + second; }
        __global__ void kernel(float *out) {
            out[0] = sqrtf(true); out[1] = sqrtf(3); out[2] = sqrtf(4.0f); out[3] = sqrtf(3, 4);
        }",
    );
    assert_eq!(
        named_call_count(&module, &module.entry_points[0].function, "sqrtf"),
        3
    );
    let source = "__device__ int sqrtf(int value) { return value; }
        __global__ void kernel() { sqrtf(1u); }";
    let error = rejected(source);
    assert!(error.message.contains("ambiguous"));
    assert_eq!(&source[error.span.to_range().unwrap()], "sqrtf");
}

#[test]
fn cuda_sqrtf_distinguishes_current_overload_recursion_from_builtin_calls() {
    validate(
        "__device__ float sqrtf(int value) { return sqrtf(float(value)); }
        __global__ void kernel(float *out) { out[0] = sqrtf(9); }",
    );
    let source = "__device__ float sqrtf(int value) { return sqrtf(value); }
        __global__ void kernel() {}";
    let error = rejected(source);
    assert!(error.message.contains("recurs"));
    assert_eq!(&source[error.span.to_range().unwrap()], "sqrtf");
}

#[test]
fn cuda_sqrtf_rejects_canonical_builtin_redefinitions() {
    for result in ["float", "int", "void"] {
        let body = if result == "void" { "" } else { "return 0;" };
        let source = format!(
            "using Sample = const float; __device__ {result} sqrtf(Sample value) {{ {body} }}
            __global__ void kernel() {{}}"
        );
        let error = rejected(&source);
        assert!(error.message.contains("duplicate"));
        assert_eq!(&source[error.span.to_range().unwrap()], "sqrtf");
    }
    for source in [
        "using sqrtf = int; __global__ void kernel() {}",
        "__global__ void sqrtf() {}",
    ] {
        assert_eq!(&source[rejected(source).span.to_range().unwrap()], "sqrtf");
    }
}

#[test]
fn cuda_sqrtf_respects_local_value_parameter_and_type_hiding() {
    for source in [
        "__global__ void kernel() { int sqrtf = 1; sqrtf(1.0f); }",
        "__device__ float helper(int sqrtf) { return sqrtf(1.0f); } __global__ void kernel() {}",
    ] {
        assert_eq!(&source[rejected(source).span.to_range().unwrap()], "sqrtf");
    }
    validate(
        "__global__ void kernel(float *out) {
            { using sqrtf = int; out[0] = sqrtf(2.5f); }
            out[1] = sqrtf(4.0f);
        }",
    );
}

#[test]
fn cuda_sqrtf_requires_one_argument_and_reports_inner_void_calls() {
    for arguments in ["", "1, 2"] {
        let source = format!("__global__ void kernel() {{ sqrtf({arguments}); }}");
        let error = rejected(&source);
        assert!(error.message.contains("no matching"));
        assert_eq!(&source[error.span.to_range().unwrap()], "sqrtf");
    }
    rejected("__global__ void kernel() { sqrtf(1,); }");
    let source = "__device__ void empty() {} __global__ void kernel() { sqrtf(empty()); }";
    assert_eq!(
        &source[rejected(source).span.to_range().unwrap()],
        "empty()"
    );
}

#[test]
fn cuda_sqrtf_rejects_vectors_arrays_pointers_and_unsupported_variants() {
    for scalar in ["int", "uint", "float"] {
        for width in [2, 3, 4] {
            let source =
                format!("__global__ void kernel() {{ {scalar}{width} value{{}}; sqrtf(value); }}");
            let error = rejected(&source);
            assert!(error.message.contains("no matching"));
            assert_eq!(&source[error.span.to_range().unwrap()], "sqrtf");
        }
    }
    for source in [
        "__global__ void kernel(float *out) { sqrtf(out); }",
        "__global__ void kernel() { float values[2]{}; sqrtf(values); }",
        "__global__ void kernel() { sqrtf(4.0); }",
        "__global__ void kernel() { sqrt(4.0f); }",
        "__global__ void kernel() { __fsqrt_rn(4.0f); }",
        "__global__ void kernel() { sqrtf(missing); }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_sqrtf_result_is_a_prvalue_and_dead_source_is_checked() {
    validate("__global__ void kernel() { return; sqrtf(-1.0f); }");
    for statement in [
        "sqrtf(4.0f) = 3.0f;",
        "++sqrtf(4.0f);",
        "sqrtf(4.0f)++;",
        "return; sqrtf(missing);",
        "return; sqrtf(1.0f, 2.0f);",
    ] {
        rejected(&format!(
            "/* π */ __global__ void kernel() {{ {statement} }}"
        ));
    }
}

#[test]
fn cuda_sqrtf_snapshots_helper_arguments_once_before_later_arguments() {
    let (module, _) = validate(
        "__device__ float next(float value) { return value; }
        __device__ float pair(float first, float second) { return first * 10.0f + second; }
        __global__ void kernel(float *out) {
            float value = 4.0f; out[0] = pair(sqrtf(next(value++)), value++); out[1] = value;
        }",
    );
    let function = &module.entry_points[0].function;
    assert_eq!(named_call_count(&module, function, "next"), 1);
    assert_eq!(named_call_count(&module, function, "pair"), 1);
}

#[test]
fn cuda_sqrtf_stays_in_selected_control_flow() {
    validate(
        "__global__ void kernel(float *out, int choose) {
            float value = 4.0f;
            out[0] = choose ? sqrtf(value++) : sqrtf(value++);
            bool active = choose && sqrtf(value++); out[1] = active;
            out[2] = value;
            for (int index = 0; index < 2; ++index) {
                if (index == 0) continue;
                out[3] = sqrtf(value++);
            }
        }",
    );
}

#[test]
fn cuda_sqrtf_is_not_a_source_constant_expression() {
    for source in [
        "constexpr float Root = sqrtf(4.0f); __global__ void kernel() {}",
        "__global__ void kernel() { constexpr float root = sqrtf(4.0f); }",
        "__global__ void kernel() { const int count = int(sqrtf(4.0f)); int values[count]; }",
        "__global__ void kernel() { constexpr int root = (sqrtf(4.0f), 3); }",
    ] {
        rejected(source);
    }
    validate(
        "constexpr float Root = true ? 9.0f : sqrtf(-1.0f);
        __global__ void kernel(int *out) {
            constexpr int Count = false && sqrtf(-1.0f) ? 2 : 3;
            int values[Count]{}; out[0] = int(Root) + values[0];
        }",
    );
}

#[test]
fn cuda_sqrtf_bounds_nested_source_and_reuses_helper_for_flat_calls() {
    let mut expression = String::from("value");
    for _ in 0..8 {
        expression = format!("sqrtf({expression})");
    }
    validate(&format!(
        "__global__ void kernel(float *out, float value) {{ out[0] = {expression}; }}"
    ));
    for _ in 0..80 {
        expression = format!("sqrtf({expression})");
    }
    let source =
        format!("__global__ void kernel(float *out, float value) {{ out[0] = {expression}; }}");
    let error = rejected(&source);
    assert!(error.message.contains("depth") || error.message.contains("nesting"));
    let statements = "out[0] = sqrtf(value);".repeat(128);
    let (module, _) = validate(&format!(
        "__global__ void kernel(float *out, float value) {{ {statements} }}"
    ));
    assert_eq!(module.functions.len(), 1);
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_sqrtf_literal_exceptional_results_remain_runtime_values_in_wgsl() {
    let (module, info) = validate(
        "__global__ void kernel(unsigned *out) {
            out[0] = __float_as_uint(sqrtf(-1.0f));
            out[1] = __float_as_uint(sqrtf(-0.0f));
            out[2] = __float_as_uint(sqrtf(__uint_as_float(0x7f800000u)));
            out[3] = __float_as_uint(sqrtf(__uint_as_float(0x7f800001u)));
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
fn cuda_sqrtf_fixture_roundtrips_through_the_shared_wgsl_helper() {
    for source in [
        include_str!("../../in/cuda/sqrtf.cu"),
        COLLIDING_HELPER_NAMES,
    ] {
        let (module, info) = validate(source);
        super::helpers::wgsl_roundtrip(
            &module,
            &info,
            naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
        );
    }
}
