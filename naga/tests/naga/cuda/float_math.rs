use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const NAMES: [&str; 4] = ["min", "max", "fminf", "fmaxf"];

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty());
    assert!(source.get(range).is_some());
    error
}

fn call_count(function: &naga::Function) -> usize {
    function
        .body
        .iter()
        .filter(|statement| matches!(statement, naga::Statement::Call { .. }))
        .count()
}

#[test]
fn cuda_float_minmax_accept_float_pairs_and_produce_float_values() {
    for name in NAMES {
        let source = format!(
            "__global__ void kernel(float *out, float first, float second) {{
                out[0] = {name}(first, second);
            }}"
        );
        let (module, info) = validate(&source);
        let function = &module.entry_points[0].function;
        let value = function
            .body
            .iter()
            .rev()
            .find_map(|statement| match *statement {
                naga::Statement::Store { value, .. } => Some(value),
                _ => None,
            })
            .unwrap();
        assert_eq!(
            info.get_entry_point(0)[value].ty.inner_with(&module.types),
            &naga::TypeInner::Scalar(naga::Scalar::F32)
        );
        assert!(module.functions.is_empty());
    }
}

#[test]
fn cuda_float_minmax_use_integer_classification_and_selection() {
    for name in NAMES {
        let (module, info) = validate(&format!(
            "__global__ void kernel(float *out, float first, float second) {{
                out[0] = {name}(first, second);
            }}"
        ));
        let function = &module.entry_points[0].function;
        let mut selects = 0;
        let mut input_bitcasts = 0;
        for (_, expression) in function.expressions.iter() {
            match *expression {
                naga::Expression::Math { .. } | naga::Expression::Relational { .. } => {
                    panic!("{name}: floating math/classification depends on target semantics")
                }
                naga::Expression::Binary { left, right, .. } => {
                    for argument in [left, right] {
                        assert_ne!(
                            info.get_entry_point(0)[argument]
                                .ty
                                .inner_with(&module.types),
                            &naga::TypeInner::Scalar(naga::Scalar::F32),
                            "{name}: floating arithmetic/comparison"
                        );
                    }
                }
                naga::Expression::Select { accept, reject, .. } => {
                    selects += 1;
                    for argument in [accept, reject] {
                        assert_eq!(
                            info.get_entry_point(0)[argument]
                                .ty
                                .inner_with(&module.types),
                            &naga::TypeInner::Scalar(naga::Scalar::U32)
                        );
                    }
                }
                naga::Expression::As {
                    kind: naga::ScalarKind::Uint,
                    convert: None,
                    ..
                } => input_bitcasts += 1,
                _ => {}
            }
        }
        assert!(selects > 0);
        assert_eq!(input_bitcasts, 2);
    }
}

#[test]
fn cuda_float_named_minmax_convert_all_scalar_argument_pairs() {
    for name in ["fminf", "fmaxf"] {
        for first in ["true", "-3", "0xffffffffu", "1.5f"] {
            for second in ["false", "2", "0x80000000u", "-0.0f"] {
                validate(&format!(
                    "__global__ void kernel(float *out) {{ out[0] = {name}({first}, {second}); }}"
                ));
            }
        }
    }
}

#[test]
fn cuda_float_minmax_keep_mixed_integer_overload_ambiguities() {
    for name in ["min", "max"] {
        for arguments in ["1.0f, 2", "2, 1.0f", "true, 1.0f", "1.0f, 2u"] {
            let source = format!("__global__ void kernel() {{ {name}({arguments}); }}");
            let error = rejected(&source);
            assert!(error.message.contains("ambiguous"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
    }
}

#[test]
fn cuda_float_minmax_preserve_selected_user_overloads() {
    for name in NAMES {
        let (module, _) = validate(&format!(
            "__device__ int {name}(float first, int second) {{ return int(first) + second; }}
            __device__ bool {name}(bool first, bool second) {{ return first || second; }}
            __global__ void kernel(float *out) {{
                out[0] = {name}(1.5f, 2.5f);
                out[1] = {name}(1.5f, 2);
                out[2] = {name}(true, false);
            }}"
        ));
        assert_eq!(call_count(&module.entry_points[0].function), 2);
    }
}

#[test]
fn cuda_float_minmax_reject_redefinitions_of_canonical_float_signatures() {
    for name in NAMES {
        for result in ["float", "int", "void"] {
            let body = if result == "void" { "" } else { "return 0;" };
            let source = format!(
                "using Sample = const float;
                __device__ {result} {name}(Sample a, float const b) {{ {body} }}
                __global__ void kernel() {{}}"
            );
            let error = rejected(&source);
            assert!(error.message.contains("duplicate"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
    }
}

#[test]
fn cuda_float_minmax_respect_local_value_and_alias_hiding() {
    for name in NAMES {
        let source = format!("__global__ void kernel() {{ int {name} = 1; {name}(1.0f, 2.0f); }}");
        assert_eq!(&source[rejected(&source).span.to_range().unwrap()], name);
        validate(&format!(
            "__global__ void kernel(float *out) {{
                {{ using {name} = int; int value = {name}(2.5f); out[0] = value; }}
                out[1] = {name}(1.0f, 2.0f);
            }}"
        ));
    }
}

#[test]
fn cuda_float_minmax_capture_arguments_before_later_mutation() {
    for name in NAMES {
        let (module, _) = validate(&format!(
            "__global__ void kernel(float *out) {{
                float value = 3.0f; out[0] = {name}(value, value++); out[1] = value;
            }}"
        ));
        let function = &module.entry_points[0].function;
        let pointer = function
            .expressions
            .iter()
            .find_map(|(handle, expression)| {
                matches!(expression, naga::Expression::LocalVariable(_)).then_some(handle)
            })
            .unwrap();
        let loads: Vec<_> = function
            .expressions
            .iter()
            .filter_map(|(handle, expression)| {
                matches!(*expression, naga::Expression::Load { pointer: p } if p == pointer)
                    .then_some(handle)
            })
            .collect();
        assert!(loads.len() >= 3);
        let first_emit = function.body.iter().position(|statement| {
            matches!(*statement, naga::Statement::Emit(ref range) if range.clone().any(|handle| handle == loads[0]))
        }).unwrap();
        assert!(function.body.iter().skip(first_emit + 1).any(|statement| {
            matches!(*statement, naga::Statement::Store { pointer: p, .. } if p == pointer)
        }));
    }
}

#[test]
fn cuda_float_minmax_evaluate_nested_helper_arguments_once() {
    for name in NAMES {
        let (module, _) = validate(&format!(
            "__device__ float next(float value) {{ return value + 1.0f; }}
            __global__ void kernel(float *out) {{
                float value = 1.0f;
                out[0] = {name}(next(value++), next(value++));
            }}"
        ));
        assert_eq!(call_count(&module.entry_points[0].function), 2);
    }
}

#[test]
fn cuda_float_minmax_work_in_device_dual_and_void_helpers() {
    validate(
        "using Sample = const float;
        __device__ float lower(Sample first, Sample second) { return fminf(first, second); }
        __host__ __device__ float upper(float first, float second) { return max(lower(first, second), second); }
        __device__ void consume(float value) { if (fmaxf(value, 0.0f)) return; }
        __global__ void kernel(float *out) { consume(1.0f); out[0] = upper(2.0f, -1.0f); }",
    );
}

#[test]
fn cuda_float_minmax_stay_in_selected_control_flow() {
    validate(
        "__global__ void kernel(float *out, int choose) {
            float value = 0.0f;
            out[0] = choose ? fminf(value++, 2.0f) : max(value++, 3.0f);
            bool flag = choose && min(value++, 1.0f);
            out[1] = flag;
        }",
    );
}

#[test]
fn cuda_float_minmax_validate_dead_calls_and_reject_invalid_results() {
    for name in NAMES {
        validate(&format!(
            "__global__ void kernel() {{ return; {name}(1.0f, 2.0f); }}"
        ));
        for body in [
            format!("{name}(1.0f, 2.0f) = 3.0f;"),
            format!("++{name}(1.0f, 2.0f);"),
            format!("return; {name}(missing, 2.0f);"),
        ] {
            rejected(&format!("__global__ void kernel() {{ {body} }}"));
        }
    }
}

#[test]
fn cuda_float_minmax_require_two_arguments() {
    for name in NAMES {
        for arguments in ["", "1.0f", "1.0f, 2.0f, 3.0f"] {
            let source = format!("__global__ void kernel() {{ {name}({arguments}); }}");
            let error = rejected(&source);
            assert!(error.message.contains("no matching"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
    }
}

#[test]
fn cuda_float_minmax_void_arguments_preserve_the_inner_call_span() {
    for name in NAMES {
        for arguments in ["empty(), 1.0f", "1.0f, empty()"] {
            let source = format!(
                "__device__ void empty() {{}} __global__ void kernel() {{ {name}({arguments}); }}"
            );
            let error = rejected(&source);
            assert_eq!(&source[error.span.to_range().unwrap()], "empty()");
        }
    }
}

#[test]
fn cuda_float_minmax_select_dependencies_obey_the_graph_depth_limit() {
    let mut value = String::from("value");
    for _ in 0..16 {
        value = format!("fminf({value}, 1.0f)");
    }
    let source = format!("__global__ void kernel(float *out, float value) {{ out[0] = {value}; }}");
    assert!(rejected(&source).message.contains("graph depth"));
    validate("__global__ void kernel(float *out) { out[0] = fminf(1.0f, fmaxf(2.0f, 3.0f)); }");
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_float_minmax_wgsl_roundtrip_preserves_integer_bit_selection() {
    let (module, info) = validate(
        "__global__ void kernel(float *out, float first, float second) {
            out[0] = min(first, second); out[1] = max(first, second);
            out[2] = fminf(first, second); out[3] = fmaxf(first, second);
        }",
    );
    let (output, _) = super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    assert!(output.contains("bitcast<u32>"));
    assert!(output.contains("bitcast<f32>"));
    assert!(!output.contains("isNan"));
}
