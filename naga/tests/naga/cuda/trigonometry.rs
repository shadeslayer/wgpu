use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const NAMES: [&str; 2] = ["sinf", "cosf"];
const FIXTURE: &str = include_str!("../../in/cuda/trigonometry.cu");

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

fn named_call_count(module: &naga::Module, function: &naga::Function, name: &str) -> usize {
    let mut count = 0;
    naga::proc::any_statement(&function.body, &mut |statement| {
        if let naga::Statement::Call { function, .. } = statement {
            count += usize::from(module.functions[*function].name.as_deref() == Some(name));
        }
        false
    });
    count
}

#[test]
fn cuda_trigonometry_converts_scalar_arguments_and_returns_float() {
    for name in NAMES {
        for declaration in [
            "bool value = true",
            "int value = -9",
            "unsigned value = 0xffffffffu",
            "float value = 6.25f",
            "using Sample = const float; Sample value = 2.0f",
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
                &naga::TypeInner::Scalar(naga::Scalar::F32),
                "{name}: {declaration}"
            );
        }
    }
}

#[test]
fn cuda_trigonometry_uses_cached_integer_helpers_and_bitcast_results() {
    let (single, _) = validate(
        "__global__ void kernel(float *out, float value) { out[0] = sinf(value); out[1] = cosf(value); }",
    );
    let (repeated, _) = validate(
        "__global__ void kernel(float *out, float value) {
            out[0] = sinf(value); out[1] = cosf(value);
            out[2] = sinf(value + 1.0f); out[3] = cosf(value - 1.0f);
        }",
    );
    assert!(!single.functions.is_empty());
    assert_eq!(single.functions.len(), repeated.functions.len());
    for (module, expected_calls) in [(&single, 2), (&repeated, 4)] {
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
        for function in module
            .functions
            .iter()
            .map(|(_, function)| function)
            .chain(module.entry_points.iter().map(|entry| &entry.function))
        {
            assert!(!function.expressions.iter().any(|(_, expression)| matches!(
                expression,
                naga::Expression::Math {
                    fun: naga::MathFunction::Sin | naga::MathFunction::Cos,
                    ..
                }
            )));
        }
        let function = &module.entry_points[0].function;
        let mut calls = 0;
        naga::proc::any_statement(&function.body, &mut |statement| {
            if let naga::Statement::Call {
                function: callee,
                result,
                ..
            } = statement
            {
                calls += 1;
                let callee = &module.functions[*callee];
                assert!(callee
                    .arguments
                    .iter()
                    .all(|argument| module.types[argument.ty].inner
                        == naga::TypeInner::Scalar(naga::Scalar::U32)));
                assert_eq!(
                    module.types[callee.result.as_ref().unwrap().ty].inner,
                    naga::TypeInner::Scalar(naga::Scalar::U32)
                );
                let result = result.unwrap();
                assert!(function.expressions.iter().any(|(_, expression)| matches!(*expression,
                    naga::Expression::As { expr, kind: naga::ScalarKind::Float, convert: None } if expr == result
                )));
            }
            false
        });
        assert_eq!(calls, expected_calls);
    }
}

#[test]
fn cuda_trigonometry_preserves_aliases_helpers_and_resource_bindings() {
    let (module, _) = validate(
        "using Sample = const float;
        __device__ Sample sine(Sample value) { return sinf(value); }
        __host__ __device__ float cosine(float value) { return cosf(value); }
        __global__ void kernel(const float *input, float scale, unsigned *out) {
            out[0] = __float_as_uint(sine(input[0]) * scale);
            out[1] = __float_as_uint(cosine(input[1]));
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
            access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE,
        }
    );
}

#[test]
fn cuda_trigonometry_merges_ordinary_overloads_and_keeps_conversion_ambiguities() {
    for name in NAMES {
        let (module, _) = validate(&format!(
            "__device__ int {name}(bool value) {{ return value ? 7 : 9; }}
            __device__ int {name}(int value) {{ return value + 10; }}
            __device__ int {name}(int first, int second) {{ return first + second; }}
            __global__ void kernel(float *out) {{
                out[0] = {name}(true); out[1] = {name}(3); out[2] = {name}(0.0f); out[3] = {name}(3, 4);
            }}"
        ));
        assert_eq!(
            named_call_count(&module, &module.entry_points[0].function, name),
            3
        );
        validate(&format!(
            "__device__ float2 {name}(float2 value) {{ return make_float2({name}(value.x), {name}(value.y)); }}
            __global__ void kernel(float *out) {{ out[0] = {name}(make_float2(0.0f, -0.0f)).x; }}"
        ));
        let source = format!(
            "__device__ int {name}(int value) {{ return value; }} __global__ void kernel() {{ {name}(1u); }}"
        );
        let error = rejected(&source);
        assert!(error.message.contains("ambiguous"));
        assert_eq!(&source[error.span.to_range().unwrap()], name);
        validate(&format!(
            "__device__ int {name}(int value) {{ return value; }}
            __global__ void kernel(int *out) {{ out[0] = {name}(true); }}"
        ));
    }
}

#[test]
fn cuda_trigonometry_distinguishes_recursion_from_builtin_calls() {
    for name in NAMES {
        validate(&format!(
            "__device__ float {name}(int value) {{ return {name}(float(value)); }}
            __global__ void kernel(float *out) {{ out[0] = {name}(9); }}"
        ));
        let source = format!(
            "__device__ float {name}(int value) {{ return {name}(value); }} __global__ void kernel() {{}}"
        );
        let error = rejected(&source);
        assert!(error.message.contains("recurs"));
        assert_eq!(&source[error.span.to_range().unwrap()], name);
    }
}

#[test]
fn cuda_trigonometry_rejects_canonical_builtin_redefinitions() {
    for name in NAMES {
        for parameter in ["float", "const float", "Sample"] {
            for result in ["float", "int", "void"] {
                let body = if result == "void" { "" } else { "return 0;" };
                let source = format!(
                    "using Sample = const float; __device__ {result} {name}({parameter} value) {{ {body} }}
                    __global__ void kernel() {{}}"
                );
                let error = rejected(&source);
                assert!(error.message.contains("duplicate"));
                assert_eq!(&source[error.span.to_range().unwrap()], name);
            }
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
fn cuda_trigonometry_respects_value_parameter_and_type_hiding() {
    for name in NAMES {
        for source in [
            format!("__global__ void kernel() {{ int {name} = 1; {name}(0.0f); }}"),
            format!("__device__ float helper(int {name}) {{ return {name}(0.0f); }} __global__ void kernel() {{}}"),
        ] {
            assert_eq!(&source[rejected(&source).span.to_range().unwrap()], name);
        }
        validate(&format!(
            "__global__ void kernel(float *out) {{
                {{ using {name} = int; out[0] = {name}(2.5f); }} out[1] = {name}(0.0f);
            }}"
        ));
    }
}

#[test]
fn cuda_trigonometry_requires_one_argument_and_reports_inner_void_calls() {
    for name in NAMES {
        for arguments in ["", "1, 2"] {
            let source = format!("__global__ void kernel() {{ {name}({arguments}); }}");
            let error = rejected(&source);
            assert!(error.message.contains("no matching"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
        rejected(&format!("__global__ void kernel() {{ {name}(1,); }}"));
        let source =
            format!("__device__ void empty() {{}} __global__ void kernel() {{ {name}(empty()); }}");
        assert_eq!(
            &source[rejected(&source).span.to_range().unwrap()],
            "empty()"
        );
    }
}

#[test]
fn cuda_trigonometry_rejects_non_scalar_arguments_and_unsupported_variants() {
    for name in NAMES {
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
        for body in [
            format!("{name}(out);"),
            format!("float values[2]{{}}; {name}(values);"),
            format!("{name}(4.0);"),
            format!("{name}(missing);"),
        ] {
            rejected(&format!("__global__ void kernel(float *out) {{ {body} }}"));
        }
    }
    for name in [
        "sin", "cos", "__sinf", "__cosf", "sinpi", "cospi", "sincosf",
    ] {
        rejected(&format!("__global__ void kernel() {{ {name}(0.0f); }}"));
    }
}

#[test]
fn cuda_trigonometry_results_are_prvalues_and_dead_source_has_utf8_spans() {
    for name in NAMES {
        validate(&format!(
            "__global__ void kernel() {{ return; {name}(0.0f); }}"
        ));
        for statement in [
            format!("{name}(0.0f) = 3.0f;"),
            format!("++{name}(0.0f);"),
            format!("{name}(0.0f)++;"),
            format!("return; {name}(missing);"),
            format!("return; {name}(1.0f, 2.0f);"),
        ] {
            rejected(&format!(
                "/* π */ __global__ void kernel() {{ {statement} }}"
            ));
        }
    }
}

#[test]
fn cuda_trigonometry_is_not_a_source_constant_expression() {
    for name in NAMES {
        for source in [
            format!("constexpr float Result = {name}(0.0f); __global__ void kernel() {{}}"),
            format!("__global__ void kernel() {{ constexpr float result = {name}(0.0f); }}"),
            format!("__global__ void kernel() {{ const int count = int({name}(0.0f)); int values[count]; }}"),
            format!("__global__ void kernel() {{ constexpr int count = ({name}(0.0f), 3); }}"),
        ] { rejected(&source); }
        validate(&format!(
            "constexpr float Result = true ? 9.0f : {name}(-1.0f);
            __global__ void kernel(int *out) {{
                constexpr int Count = false && {name}(-1.0f) ? 2 : 3;
                int values[Count]{{}}; out[0] = int(Result) + values[0];
            }}"
        ));
    }
}

#[test]
fn cuda_trigonometry_snapshots_nested_helper_arguments_once() {
    for name in NAMES {
        let (module, _) = validate(&format!(
            "__device__ float next(float value) {{ return value; }}
            __device__ float pair(float first, float second) {{ return first * 10.0f + second; }}
            __global__ void kernel(float *out) {{
                float value = 0.0f; out[0] = pair({name}(next(value++)), value++); out[1] = value;
            }}"
        ));
        let function = &module.entry_points[0].function;
        assert_eq!(named_call_count(&module, function, "next"), 1);
        assert_eq!(named_call_count(&module, function, "pair"), 1);
    }
}

#[test]
fn cuda_trigonometry_preserves_pending_arrays_and_selected_control_flow() {
    for name in NAMES {
        validate(&format!(
            "__global__ void kernel(float *out, int choose) {{
                float values[]{{{name}(0.0f), (values[0], 2)}};
                constexpr int Count = (values[1], 2); int scratch[Count]{{}};
                out[0] = values[0] + values[1] + float(scratch[1]);
                float value = 0.0f;
                out[1] = choose ? {name}(value++) : {name}(value++);
                bool active = choose && {name}(value++); out[2] = active;
                bool done = choose || {name}(value++); out[3] = done;
                for (int index = 0; index < 2; ++index) {{
                    if (index == 0) continue;
                    float repeated[]{{{name}(value++), (repeated[0], 3)}};
                    out[4] = repeated[0] + repeated[1];
                }}
                out[5] = value;
            }}"
        ));
    }
}

#[test]
fn cuda_trigonometry_bounds_source_depth_and_keeps_flat_call_lowering_linear() {
    for name in NAMES {
        let mut expression = String::from("value");
        for _ in 0..8 {
            expression = format!("{name}({expression})");
        }
        validate(&format!(
            "__global__ void kernel(float *out, float value) {{ out[0] = {expression}; }}"
        ));
        for _ in 0..80 {
            expression = format!("{name}({expression})");
        }
        let source =
            format!("__global__ void kernel(float *out, float value) {{ out[0] = {expression}; }}");
        let error = rejected(&source);
        assert!(error.message.contains("depth") || error.message.contains("nesting"));
        let (single, _) = validate(&format!(
            "__global__ void kernel(float *out, float value) {{ out[0] = {name}(value); }}"
        ));
        let statements = format!("out[0] = {name}(value);").repeat(128);
        let (many, _) = validate(&format!(
            "__global__ void kernel(float *out, float value) {{ {statements} }}"
        ));
        assert_eq!(single.functions.len(), many.functions.len());
        assert!(
            many.entry_points[0].function.expressions.len()
                <= single.entry_points[0].function.expressions.len() * 128 + 256
        );
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_trigonometry_fixture_and_literal_specials_roundtrip_through_shared_wgsl_helpers() {
    for source in [
        FIXTURE,
        "__global__ void kernel(unsigned *out) {
            out[0] = __float_as_uint(sinf(-0.0f)); out[1] = __float_as_uint(cosf(-0.0f));
            out[2] = __float_as_uint(sinf(__uint_as_float(0x7f800000u)));
            out[3] = __float_as_uint(cosf(__uint_as_float(0x7f800001u)));
            out[4] = __float_as_uint(sinf(__uint_as_float(0xff800000u)));
            out[5] = __float_as_uint(cosf(3.402823466e38f));
            out[6] = __float_as_uint(sinf(-3.402823466e38f));
        }",
    ] {
        let (module, info) = validate(source);
        super::helpers::wgsl_roundtrip(
            &module,
            &info,
            naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
        );
    }
}

#[cfg(spv_out)]
#[cfg_attr(miri, ignore)]
#[test]
fn cuda_trigonometry_fixture_emits_valid_spirv_binary() {
    let (module, info) = validate(FIXTURE);
    let options = naga::back::spv::Options {
        lang_version: (1, 1),
        ..Default::default()
    };
    let pipeline_options = naga::back::spv::PipelineOptions {
        entry_point: "trigonometry".to_string(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let words =
        naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline_options)).unwrap();
    crate::helpers::validate_spirv(&words);
}
