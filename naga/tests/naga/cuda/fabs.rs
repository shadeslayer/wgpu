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
fn cuda_fabs_functions_return_float_after_the_declared_scalar_conversion() {
    for (name, declaration) in [
        ("fabsf", "bool value = true"),
        ("fabsf", "int value = -9"),
        ("fabsf", "unsigned value = 0xffffffffu"),
        ("fabsf", "float value = -6.25f"),
        ("fabs", "float value = -6.25f"),
        ("fabs", "using Sample = const float; Sample value = -2.0f"),
    ] {
        let (module, info) = validate(&format!(
            "__global__ void kernel(float *out) {{ {declaration}; out[0] = {name}(value); }}"
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
fn cuda_fabs_lowering_converts_before_reinterpretation_without_native_abs_or_double() {
    for parameter in ["int", "unsigned"] {
        let (module, _) = validate(&format!(
            "__global__ void kernel(float *out, {parameter} value) {{ out[0] = fabsf(value); }}"
        ));
        assert!(module.entry_points[0].function.expressions.iter().any(
            |(_, expression)| matches!(
                expression,
                naga::Expression::As {
                    kind: naga::ScalarKind::Float,
                    convert: Some(4),
                    ..
                }
            )
        ));
    }
    let (module, _) = validate(
        "__global__ void kernel(float *out, float value) {
            out[0] = fabsf(value); out[1] = fabs(value);
        }",
    );
    for function in module
        .functions
        .iter()
        .map(|(_, function)| function)
        .chain(module.entry_points.iter().map(|entry| &entry.function))
    {
        assert!(!function.expressions.iter().any(|(_, expression)| matches!(
            expression,
            naga::Expression::Math {
                fun: naga::MathFunction::Abs,
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
fn cuda_fabs_integral_calls_require_unsupported_double_instead_of_worse_user_matches() {
    let cases: [(&str, &[&str]); 3] = [
        ("", &["true", "-3", "1u"]),
        (
            "__device__ int fabs(int value) { return value; }",
            &["true", "1u"],
        ),
        (
            "__device__ int fabs(bool value) { return value; }",
            &["-3", "1u"],
        ),
    ];
    for (definitions, arguments) in cases {
        for argument in arguments {
            let source =
                format!("{definitions} /* π */ __global__ void kernel() {{ fabs({argument}); }}");
            let error = rejected(&source);
            assert!(error.message.contains("double"), "{}", error.message);
            assert_eq!(&source[error.span.to_range().unwrap()], "fabs");
        }
    }
}

#[test]
fn cuda_fabs_exact_ordinary_overloads_win_and_builtin_float_matches_remain_visible() {
    let (module, _) = validate(
        "using Flag = const bool; using Count = const unsigned;
        __device__ int fabs(Flag value) { return value ? 7 : 9; }
        __device__ int fabs(const int value) { return value + 40; }
        __device__ unsigned fabs(Count value) { return value + 50u; }
        __device__ int fabs(int first, int second) { return first + second; }
        __device__ int fabsf(bool value) { return value ? 11 : 13; }
        __global__ void kernel(float *out) {
            out[0] = fabs(true); out[1] = fabs(-3); out[2] = fabs(3u);
            out[3] = fabs(-4.0f); out[4] = fabs(3, 4);
            out[5] = fabsf(true); out[6] = fabsf(-2.0f);
        }",
    );
    let function = &module.entry_points[0].function;
    assert_eq!(named_call_count(&module, function, "fabs"), 4);
    assert_eq!(named_call_count(&module, function, "fabsf"), 1);
    let source = "__device__ int fabs(int value) { return value; }
        __device__ float caller(bool value) { return fabs(value); }
        __global__ void kernel() {}";
    let error = rejected(source);
    assert!(error.message.contains("double"));
    assert_eq!(&source[error.span.to_range().unwrap()], "fabs");
}

#[test]
fn cuda_fabs_current_overloads_can_call_scalar_builtins_but_cannot_recurse() {
    for name in ["fabs", "fabsf"] {
        validate(&format!(
            "__device__ float {name}(int value) {{ return {name}(float(value)); }}
            __global__ void kernel(float *out) {{ out[0] = {name}(-9); }}"
        ));
        let source = format!(
            "__device__ float {name}(int value) {{ return {name}(value); }}
            __global__ void kernel() {{}}"
        );
        let error = rejected(&source);
        assert!(error.message.contains("recurs"));
        assert_eq!(&source[error.span.to_range().unwrap()], name);
    }
}

#[test]
fn cuda_fabs_rejects_canonical_float_redefinitions_and_global_name_collisions() {
    for name in ["fabs", "fabsf"] {
        for result in ["float", "int", "void"] {
            let body = if result == "void" { "" } else { "return 0;" };
            let source = format!(
                "using Sample = const float; __device__ {result} {name}(Sample value) {{ {body} }}
                __global__ void kernel() {{}}"
            );
            let error = rejected(&source);
            assert!(error.message.contains("duplicate"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
        for source in [
            format!("using {name} = int; __global__ void kernel() {{}}"),
            format!("__global__ void {name}() {{}}"),
        ] {
            assert_eq!(&source[rejected(&source).span.to_range().unwrap()], name);
        }
    }
}

#[test]
fn cuda_fabs_respects_local_values_parameters_and_type_hiding() {
    for name in ["fabs", "fabsf"] {
        for source in [
            format!("__global__ void kernel() {{ int {name} = 1; {name}(-1.0f); }}"),
            format!("__device__ float helper(int {name}) {{ return {name}(-1.0f); }} __global__ void kernel() {{}}"),
        ] {
            assert_eq!(&source[rejected(&source).span.to_range().unwrap()], name);
        }
        validate(&format!(
            "__global__ void kernel(float *out) {{
                {{ using {name} = int; out[0] = {name}(-2.5f); }}
                out[1] = {name}(-4.0f);
            }}"
        ));
    }
}

#[test]
fn cuda_fabs_requires_one_argument_and_reports_inner_void_calls() {
    for name in ["fabs", "fabsf"] {
        for arguments in ["", "1.0f, 2.0f"] {
            let source = format!("__global__ void kernel() {{ {name}({arguments}); }}");
            let error = rejected(&source);
            assert!(error.message.contains("no matching"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
        rejected(&format!("__global__ void kernel() {{ {name}(1.0f,); }}"));
        let source =
            format!("__device__ void empty() {{}} __global__ void kernel() {{ {name}(empty()); }}");
        assert_eq!(
            &source[rejected(&source).span.to_range().unwrap()],
            "empty()"
        );
    }
}

#[test]
fn cuda_fabs_accepts_user_vector_overloads_without_enabling_builtin_aggregate_calls() {
    for name in ["fabs", "fabsf"] {
        for scalar in ["int", "uint", "float"] {
            for width in [2, 3, 4] {
                let source = format!(
                    "__global__ void kernel() {{ {scalar}{width} value{{}}; {name}(value); }}"
                );
                let error = rejected(&source);
                assert!(error.message.contains("no matching"));
                assert_eq!(&source[error.span.to_range().unwrap()], name);
            }
        }
        for source in [
            format!("__global__ void kernel(float *out) {{ {name}(out); }}"),
            format!("__global__ void kernel() {{ float values[2]{{}}; {name}(values); }}"),
            format!("__global__ void kernel() {{ {name}(4.0); }}"),
            format!("__global__ void kernel() {{ {name}(missing); }}"),
        ] {
            rejected(&source);
        }
        validate(&format!(
            "__device__ float2 {name}(float2 value) {{
                return make_float2({name}(value.x), {name}(value.y));
            }}
            __global__ void kernel(float *out) {{
                out[0] = {name}(make_float2(-1.75f, -1.25f)).x;
            }}"
        ));
    }
}

#[test]
fn cuda_fabs_preserves_aliases_helpers_and_resource_bindings() {
    let (module, _) = validate(
        "using Sample = const float;
        __device__ Sample absolute(Sample value) { return fabs(value); }
        __host__ __device__ float twice(float value) { return fabsf(value) * 2.0f; }
        __device__ void consume(float value) { if (fabs(value)) return; }
        __global__ void kernel(const float *input, float scale, unsigned *out) {
            consume(input[0]); out[0] = __float_as_uint(absolute(input[0]) * scale);
            out[1] = __float_as_uint(twice(input[1]));
        }",
    );
    let resources: Vec<_> = module
        .global_variables
        .iter()
        .map(|(_, resource)| resource)
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
fn cuda_fabs_results_are_prvalues_and_dead_source_remains_checked() {
    for name in ["fabs", "fabsf"] {
        validate(&format!(
            "__global__ void kernel() {{ return; {name}(-1.0f); }}"
        ));
        for statement in [
            format!("{name}(-4.0f) = 3.0f;"),
            format!("++{name}(-4.0f);"),
            format!("{name}(-4.0f)++;"),
            format!("return; {name}(missing);"),
            format!("return; {name}(1.0f, 2.0f);"),
        ] {
            rejected(&format!(
                "/* π */ __global__ void kernel() {{ {statement} }}"
            ));
        }
    }
    let source = "__global__ void kernel() { return; fabs(1u); }";
    let error = rejected(source);
    assert!(error.message.contains("double"));
    assert_eq!(&source[error.span.to_range().unwrap()], "fabs");
}

#[test]
fn cuda_fabs_snapshots_arguments_once_before_later_helper_arguments() {
    for name in ["fabs", "fabsf"] {
        let (module, _) = validate(&format!(
            "__device__ float next(float value) {{ return value; }}
            __device__ float pair(float first, float second) {{ return first * 10.0f + second; }}
            __global__ void kernel(float *out) {{
                float value = -1.25f;
                out[0] = pair({name}(next(value++)), value++); out[1] = value;
            }}"
        ));
        let function = &module.entry_points[0].function;
        assert_eq!(named_call_count(&module, function, "next"), 1);
        assert_eq!(named_call_count(&module, function, "pair"), 1);
    }
}

#[test]
fn cuda_fabs_stays_in_selected_branches_and_loop_bodies() {
    validate(
        "__global__ void kernel(float *out, int choose) {
            float value = -4.0f;
            out[0] = choose ? fabs(value++) : fabsf(value++);
            bool active = choose && fabsf(value++); out[1] = active;
            out[2] = value;
            for (int index = 0; index < 2; ++index) {
                if (index == 0) continue;
                out[3] = fabs(value++);
            }
        }",
    );
}

#[test]
fn cuda_fabs_calls_are_not_source_constant_expressions() {
    for name in ["fabs", "fabsf"] {
        for source in [
            format!("constexpr float Absolute = {name}(-4.0f); __global__ void kernel() {{}}"),
            format!("__global__ void kernel() {{ constexpr float absolute = {name}(-4.0f); }}"),
            format!("__global__ void kernel() {{ const int count = int({name}(-4.0f)); int values[count]; }}"),
            format!("__global__ void kernel() {{ constexpr int count = ({name}(-4.0f), 3); }}"),
        ] {
            rejected(&source);
        }
        validate(&format!(
            "constexpr float Absolute = true ? 9.0f : {name}(-1.0f);
            __global__ void kernel(int *out) {{
                constexpr int Count = false && {name}(-1.0f) ? 2 : 3;
                int values[Count]{{}}; out[0] = int(Absolute) + values[0];
            }}"
        ));
    }
}

#[test]
fn cuda_fabs_preserves_pending_inferred_arrays_and_completed_bounds() {
    for name in ["fabs", "fabsf"] {
        validate(&format!(
            "__global__ void kernel(float *out) {{
                float values[]{{{name}(-4.75f), (values[0], 2)}};
                constexpr int Count = (values[1], 2);
                int scratch[Count]{{}};
                out[0] = values[0] + values[1] + float(scratch[1]);
                for (int index = 0; index < 2; ++index) {{
                    float repeated[]{{{name}(-1.25f), (repeated[0], 3)}};
                    out[1] = repeated[0] + repeated[1];
                }}
            }}"
        ));
    }
}

#[test]
fn cuda_fabs_bounds_nested_calls_and_keeps_flat_lowering_linear() {
    let mut expression = String::from("value");
    for _ in 0..8 {
        expression = format!("fabs(fabsf({expression}))");
    }
    validate(&format!(
        "__global__ void kernel(float *out, float value) {{ out[0] = {expression}; }}"
    ));
    for _ in 0..40 {
        expression = format!("fabs(fabsf({expression}))");
    }
    let source =
        format!("__global__ void kernel(float *out, float value) {{ out[0] = {expression}; }}");
    let error = rejected(&source);
    assert!(error.message.contains("depth") || error.message.contains("nesting"));
    let (single, _) = validate(
        "__global__ void kernel(float *out, float value) { out[0] = fabs(fabsf(value)); }",
    );
    let statements = "out[0] = fabs(fabsf(value));".repeat(128);
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
fn cuda_fabs_literal_exceptional_results_remain_runtime_values_in_wgsl() {
    for name in ["fabs", "fabsf"] {
        let (module, info) = validate(&format!(
            "__global__ void kernel(unsigned *out) {{
                out[0] = __float_as_uint({name}(-1.0f));
                out[1] = __float_as_uint({name}(-0.0f));
                out[2] = __float_as_uint({name}(__uint_as_float(0x7f800000u)));
                out[3] = __float_as_uint({name}(__uint_as_float(0x7f800001u)));
                out[4] = __float_as_uint({name}(__uint_as_float(0xff800000u)));
                out[5] = __float_as_uint({name}(-3.402823466e38f));
                out[6] = __float_as_uint({name}(1e10f));
            }}"
        ));
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
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_fabs_fixture_roundtrips_through_the_shared_wgsl_helper() {
    let (module, info) = validate(include_str!("../../in/cuda/fabs.cu"));
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}
