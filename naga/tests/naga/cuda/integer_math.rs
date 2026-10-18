use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const INTEGERS: [(&str, &str, naga::Scalar); 3] = [
    ("bool", "true", naga::Scalar::BOOL),
    ("int", "-3", naga::Scalar::I32),
    ("unsigned", "7u", naga::Scalar::U32),
];

fn math(function: &naga::Function) -> Vec<naga::Handle<naga::Expression>> {
    function
        .expressions
        .iter()
        .filter_map(|(handle, expression)| {
            matches!(*expression, naga::Expression::Math { .. }).then_some(handle)
        })
        .collect()
}

fn operands(
    function: &naga::Function,
    handle: naga::Handle<naga::Expression>,
) -> [naga::Handle<naga::Expression>; 2] {
    let naga::Expression::Math {
        arg,
        arg1: Some(second),
        arg2: None,
        arg3: None,
        ..
    } = function.expressions[handle]
    else {
        panic!("expected a binary math expression")
    };
    [arg, second]
}

fn calls(function: &naga::Function) -> Vec<naga::Handle<naga::Function>> {
    function
        .body
        .iter()
        .filter_map(|statement| match *statement {
            naga::Statement::Call { function, .. } => Some(function),
            _ => None,
        })
        .collect()
}

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty());
    assert!(source.get(range).is_some());
    error
}

#[test]
fn cuda_integer_minmax_promote_all_integral_scalar_pairs() {
    for (name, fun) in [
        ("min", naga::MathFunction::Min),
        ("max", naga::MathFunction::Max),
    ] {
        for (left_ty, left, left_scalar) in INTEGERS {
            for (right_ty, right, right_scalar) in INTEGERS {
                let source = format!(
                    "__global__ void kernel(int *out) {{
                        {left_ty} left = {left}; {right_ty} right = {right};
                        out[0] = {name}(left, right);
                    }}"
                );
                let (module, info) = validate(&source);
                let function = &module.entry_points[0].function;
                assert!(module.functions.is_empty());
                let handles = math(function);
                assert_eq!(handles.len(), 1, "{source}");
                let handle = handles[0];
                assert!(
                    matches!(function.expressions[handle], naga::Expression::Math { fun: actual, .. } if actual == fun)
                );
                let expected = if [left_scalar, right_scalar].contains(&naga::Scalar::U32) {
                    naga::Scalar::U32
                } else {
                    naga::Scalar::I32
                };
                for expression in operands(function, handle).into_iter().chain([handle]) {
                    assert_eq!(
                        info.get_entry_point(0)[expression]
                            .ty
                            .inner_with(&module.types),
                        &naga::TypeInner::Scalar(expected),
                        "{source}"
                    );
                }
            }
        }
    }
}

#[test]
fn cuda_integer_minmax_keep_unsigned_extremes_and_mixed_signedness() {
    let (module, info) = validate(
        "__global__ void kernel(unsigned *out) {
            out[0] = min(-1, 1u); out[1] = max(1u, -1);
            out[2] = min(0xffffffffu, 0x80000000u);
            out[3] = max(-2147483647 - 1, 2147483647);
        }",
    );
    let function = &module.entry_points[0].function;
    let expected = [
        naga::Scalar::U32,
        naga::Scalar::U32,
        naga::Scalar::U32,
        naga::Scalar::I32,
    ];
    let handles = math(function);
    assert_eq!(handles.len(), expected.len());
    for (handle, scalar) in handles.into_iter().zip(expected) {
        assert_eq!(
            info.get_entry_point(0)[handle].ty.inner_with(&module.types),
            &naga::TypeInner::Scalar(scalar)
        );
    }
}

#[test]
fn cuda_integer_minmax_accept_aliases_and_const_operands() {
    validate(
        "using Flag = bool; typedef const int Signed; using Natural = unsigned;
        __global__ void kernel(int *out) {
            Flag first = true; Signed second = -2; Natural third = 3u;
            out[0] = min(first, second); out[1] = max(second, third);
        }",
    );
}

#[test]
fn cuda_integer_minmax_capture_each_argument_before_later_mutation() {
    let (module, _) = validate(
        "__global__ void kernel(int *out) {
            int value = 3; out[0] = min(value, value++); out[1] = value;
        }",
    );
    let function = &module.entry_points[0].function;
    let [first, second] = operands(function, math(function)[0]);
    assert_ne!(first, second);
    let naga::Expression::Load { pointer } = function.expressions[first] else {
        panic!("expected captured first argument")
    };
    assert!(
        matches!(function.expressions[second], naga::Expression::Load { pointer: other } if other == pointer)
    );
    let first_emit = function.body.iter().position(|statement| {
        matches!(*statement, naga::Statement::Emit(ref range) if range.clone().any(|handle| handle == first))
    }).unwrap();
    assert!(function.body.iter().skip(first_emit + 1).any(|statement| {
        matches!(*statement, naga::Statement::Store { pointer: stored, .. } if stored == pointer)
    }));
}

#[test]
fn cuda_integer_minmax_evaluate_nested_helper_arguments_once_in_order() {
    let (module, _) = validate(
        "__device__ int next(int value) { return value + 1; }
        __global__ void kernel(int *out) {
            int value = 0; out[0] = max(next(value++), min(next(value++), 4));
        }",
    );
    let function = &module.entry_points[0].function;
    assert_eq!(calls(function).len(), 2);
    assert_eq!(calls(function)[0], calls(function)[1]);
    assert_eq!(math(function).len(), 2);
}

#[test]
fn cuda_integer_minmax_merge_builtin_and_user_scalar_overloads() {
    for name in ["min", "max"] {
        let (module, _) = validate(&format!(
            "__device__ bool {name}(bool first, bool second) {{ return first || second; }}
            __global__ void kernel(int *out) {{
                out[0] = {name}(2, 3); out[1] = {name}(true, false);
            }}"
        ));
        let function = &module.entry_points[0].function;
        assert_eq!(module.functions.len(), 1);
        assert_eq!(math(function).len(), 1);
        assert_eq!(calls(function).len(), 1);
    }
}

#[test]
fn cuda_integer_minmax_merge_user_overloads_with_different_arity() {
    let (module, _) = validate(
        "__device__ int min(int value) { return value + 100; }
        __global__ void kernel(int *out) { out[0] = min(3); out[1] = min(3, 4); }",
    );
    let function = &module.entry_points[0].function;
    assert_eq!(calls(function).len(), 1);
    assert_eq!(math(function).len(), 1);
}

#[test]
fn cuda_integer_minmax_allow_selected_user_overloads_with_float_arguments() {
    let (module, _) = validate(
        "__device__ int min(float first, int second) { return int(first) + second; }
        __global__ void kernel(int *out) { out[0] = min(3.5f, 4); out[1] = min(3, 4); }",
    );
    let function = &module.entry_points[0].function;
    assert_eq!(calls(function).len(), 1);
    assert_eq!(math(function).len(), 1);
}

#[test]
fn cuda_integer_minmax_current_overload_can_call_a_distinct_builtin() {
    let (module, _) = validate(
        "__host__ __device__ bool min(bool first, bool second) {
            return min(int(first), int(second));
        }
        __device__ void consume(int value) { if (max(value, 0)) return; }
        __global__ void kernel(int *out) { out[0] = min(true, false); consume(2); }",
    );
    assert_eq!(module.functions.len(), 2);
    for (_, function) in module.functions.iter() {
        assert_eq!(math(function).len(), 1);
    }
    assert!(rejected(
        "__device__ bool min(bool a, bool b) { return min(a, b); } __global__ void kernel() {}"
    )
    .message
    .contains("recursive"));
}

#[test]
fn cuda_integer_minmax_local_names_hide_the_whole_overload_family() {
    for source in [
        "__global__ void kernel() { int min = 1; min(2, 3); }",
        "__global__ void kernel(int max) { max(2, 3); }",
        "__global__ void kernel() { int min = min(2, 3); }",
    ] {
        assert!(rejected(source).message.contains("cannot be called"));
    }
    let (module, _) = validate(
        "__global__ void kernel(int *out) {
            { using min = int; out[0] = min(3.5f); }
            out[1] = min(2, 3);
        }",
    );
    assert_eq!(math(&module.entry_points[0].function).len(), 1);
}

#[test]
fn cuda_integer_minmax_implicit_families_reject_global_type_and_kernel_names() {
    for source in [
        "using min = int; __global__ void kernel() {}",
        "typedef int max; __global__ void kernel() {}",
        "__global__ void min() {}",
        "__global__ void max(int *out) {}",
    ] {
        let error = rejected(source);
        assert!(["min", "max"].contains(&&source[error.span.to_range().unwrap()]));
    }
}

#[test]
fn cuda_integer_minmax_reject_definitions_of_implicit_canonical_signatures() {
    for name in ["min", "max"] {
        for (first, second, result) in [
            ("int", "int", "int"),
            ("unsigned", "unsigned", "unsigned"),
            ("int", "unsigned", "unsigned"),
            ("unsigned", "int", "unsigned"),
            ("float", "float", "float"),
            ("const Whole", "Whole", "void"),
        ] {
            let body = if result == "void" { "" } else { "return 0;" };
            let source = format!(
                "using Whole = int; __device__ {result} {name}({first} a, {second} b) {{ {body} }} __global__ void kernel() {{}}"
            );
            let error = rejected(&source);
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
    }
}

#[test]
fn cuda_integer_minmax_keep_ambiguity_with_float_overloads() {
    for name in ["min", "max"] {
        let source = format!(
            "__device__ int {name}(bool first, unsigned second) {{ return 9; }}
            __global__ void kernel(int *out) {{ out[0] = {name}(true, 1); }}"
        );
        assert!(rejected(&source).message.contains("ambiguous"));
        let source =
            format!("__global__ void kernel(float *out) {{ out[0] = {name}(1.0f, 2.0f); }}");
        validate(&source);
        for arguments in ["1.0f, 2", "2, 1.0f", "true, 1.0f", "1.0f, 2u"] {
            let source = format!("__global__ void kernel() {{ {name}({arguments}); }}");
            assert!(rejected(&source).message.contains("ambiguous"));
        }
    }
}

#[test]
fn cuda_integer_minmax_require_two_arguments_without_a_user_overload() {
    for name in ["min", "max"] {
        for arguments in ["", "1", "1, 2, 3"] {
            let source = format!("__global__ void kernel() {{ {name}({arguments}); }}");
            let error = rejected(&source);
            assert!(error.message.contains("no matching"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
    }
}

#[test]
fn cuda_integer_minmax_void_arguments_report_the_inner_call_span() {
    for arguments in ["empty(), 1", "1, empty()"] {
        let source = format!(
            "__device__ void empty() {{}} __global__ void kernel() {{ min({arguments}); }}"
        );
        let error = rejected(&source);
        assert_eq!(&source[error.span.to_range().unwrap()], "empty()");
    }
}

#[test]
fn cuda_integer_minmax_results_are_values_and_source_errors_remain_checked() {
    for body in ["min(1, 2) = 3;", "++max(1, 2);", "return; min(missing, 2);"] {
        rejected(&format!("__global__ void kernel() {{ {body} }}"));
    }
    validate("__global__ void kernel() { return; min(1, 2); }");
    validate("__global__ void kernel() { return; max(1.0f, 2.0f); }");
}

#[test]
fn cuda_integer_minmax_stay_in_selected_control_flow_and_keep_value_conversions() {
    validate(
        "__global__ void kernel(float *out, int choose) {
            int value = 0;
            out[0] = choose ? min(value++, 3) : max(value++, 4);
            bool flag = choose && min(value++, 2);
            out[1] = flag;
        }",
    );
}

#[test]
fn cuda_integer_minmax_math_dependencies_obey_the_graph_depth_limit() {
    let sum = core::iter::repeat_n("1", 64)
        .collect::<Vec<_>>()
        .join(" + ");
    let source = format!("__global__ void kernel(int *out) {{ out[0] = min({sum}, 0) + 1; }}");
    assert!(rejected(&source).message.contains("graph depth"));
    validate("__global__ void kernel(int *out) { out[0] = min(1 + 2, max(3, 4)) + 1; }");
}
