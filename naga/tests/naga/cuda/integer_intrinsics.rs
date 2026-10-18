use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const INTRINSICS: [(&str, naga::Scalar, naga::Scalar, naga::MathFunction); 4] = [
    (
        "__clz",
        naga::Scalar::I32,
        naga::Scalar::I32,
        naga::MathFunction::CountLeadingZeros,
    ),
    (
        "__ffs",
        naga::Scalar::I32,
        naga::Scalar::I32,
        naga::MathFunction::FirstTrailingBit,
    ),
    (
        "__popc",
        naga::Scalar::U32,
        naga::Scalar::I32,
        naga::MathFunction::CountOneBits,
    ),
    (
        "__brev",
        naga::Scalar::U32,
        naga::Scalar::U32,
        naga::MathFunction::ReverseBits,
    ),
];

fn type_name(scalar: naga::Scalar) -> &'static str {
    if scalar == naga::Scalar::I32 {
        "int"
    } else {
        "unsigned"
    }
}

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
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
fn cuda_integer_intrinsics_convert_scalar_arguments_and_preserve_declared_result_types() {
    for (name, parameter, result, operation) in INTRINSICS {
        for declaration in [
            "bool value = true",
            "int value = 8",
            "unsigned value = 5u",
            "float value = 8.75f",
        ] {
            let result_name = type_name(result);
            let (module, info) = validate(&format!("__global__ void kernel({result_name} *out) {{ {declaration}; out[0] = {name}(value); }}"));
            let function = &module.entry_points[0].function;
            let operations: Vec<_> = function
                .expressions
                .iter()
                .filter_map(|(handle, expression)| match *expression {
                    naga::Expression::Math {
                        fun,
                        arg,
                        arg1: None,
                        arg2: None,
                        arg3: None,
                    } if fun == operation => Some((handle, arg)),
                    _ => None,
                })
                .collect();
            assert_eq!(operations.len(), 1, "{name}: {declaration}");
            let (math, argument) = operations[0];
            for handle in [math, argument] {
                assert_eq!(
                    info.get_entry_point(0)[handle].ty.inner_with(&module.types),
                    &naga::TypeInner::Scalar(parameter)
                );
            }
            let stored = function
                .body
                .iter()
                .rev()
                .find_map(|statement| match *statement {
                    naga::Statement::Store { value, .. } => Some(value),
                    _ => None,
                })
                .unwrap();
            assert_eq!(
                info.get_entry_point(0)[stored].ty.inner_with(&module.types),
                &naga::TypeInner::Scalar(result)
            );
            assert!(module.functions.is_empty());
        }
    }
}

#[test]
fn cuda_integer_intrinsics_numerically_convert_float_arguments_before_math() {
    for (name, parameter, _, operation) in INTRINSICS {
        let (module, _) = validate(&format!("__global__ void kernel(unsigned *out, float value) {{ out[0] = unsigned({name}(value)); }}"));
        let function = &module.entry_points[0].function;
        let argument = function
            .expressions
            .iter()
            .find_map(|(_, expression)| match *expression {
                naga::Expression::Math { fun, arg, .. } if fun == operation => Some(arg),
                _ => None,
            })
            .unwrap();
        assert!(
            matches!(function.expressions[argument], naga::Expression::As { kind, convert: Some(4), .. } if kind == parameter.kind)
        );
        assert!(!function.expressions.iter().any(|(_, expression)| matches!(
            expression,
            naga::Expression::As { convert: None, .. }
        )));
    }
}

#[test]
fn cuda_ffs_adds_one_to_the_signed_first_trailing_bit_result() {
    let (module, info) =
        validate("__global__ void kernel(int *out, int value) { out[0] = __ffs(value); }");
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
    let naga::Expression::Binary {
        op: naga::BinaryOperator::Add,
        left,
        right,
    } = function.expressions[value]
    else {
        panic!("expected one-based bit position");
    };
    assert!(matches!(
        function.expressions[left],
        naga::Expression::Math {
            fun: naga::MathFunction::FirstTrailingBit,
            ..
        }
    ));
    assert!(matches!(
        function.expressions[right],
        naga::Expression::Literal(naga::Literal::I32(1))
    ));
    assert_eq!(
        info.get_entry_point(0)[value].ty.inner_with(&module.types),
        &naga::TypeInner::Scalar(naga::Scalar::I32)
    );
    assert!(!function.expressions.iter().any(|(_, expression)| matches!(
        expression,
        naga::Expression::Select { .. } | naga::Expression::Unary { .. }
    )));
}

#[test]
fn cuda_popc_converts_the_unsigned_count_to_a_signed_result() {
    let (module, _) =
        validate("__global__ void kernel(int *out, unsigned value) { out[0] = __popc(value); }");
    let function = &module.entry_points[0].function;
    let stored = function
        .body
        .iter()
        .rev()
        .find_map(|statement| match *statement {
            naga::Statement::Store { value, .. } => Some(value),
            _ => None,
        })
        .unwrap();
    let naga::Expression::As {
        expr,
        kind: naga::ScalarKind::Sint,
        convert: Some(4),
    } = function.expressions[stored]
    else {
        panic!("CUDA popc returns int");
    };
    assert!(matches!(
        function.expressions[expr],
        naga::Expression::Math {
            fun: naga::MathFunction::CountOneBits,
            ..
        }
    ));
}

#[test]
fn cuda_integer_intrinsics_accept_aliases_const_values_and_helper_results() {
    validate(
        "using Bits = unsigned; using Signed = const int;
        __device__ int count(Bits value) { return __popc(value); }
        __host__ __device__ int scan(Signed value) { return __ffs(value); }
        __device__ void consume(int value) { if (__clz(value)) return; }
        __global__ void kernel(Bits *out) {
            const Bits value = 0x80000001u;
            consume(scan(int(value))); out[0] = unsigned(count(value)); out[1] = __brev(value);
        }",
    );
}

#[test]
fn cuda_integer_intrinsics_merge_builtin_and_extra_user_overloads() {
    for (name, parameter, _, _) in INTRINSICS {
        let parameter_name = type_name(parameter);
        let (module, _) = validate(&format!(
            "__device__ int {name}(bool value) {{ return value ? 7 : 9; }}
            __device__ int {name}(float value) {{ return int(value) + 10; }}
            __device__ int {name}(int first, int second) {{ return first + second; }}
            __global__ void kernel(unsigned *out) {{
                out[0] = unsigned({name}(true)); out[1] = unsigned({name}({parameter_name}(1)));
                out[2] = unsigned({name}(3.75f)); out[3] = unsigned({name}(3, 4));
            }}"
        ));
        assert_eq!(call_count(&module.entry_points[0].function), 3);
    }
}

#[test]
fn cuda_integer_intrinsics_rank_bool_promotion_by_the_declared_parameter() {
    let (module, _) = validate(
        "__device__ int __popc(int value) { return value + 100; }
        __device__ int __clz(unsigned value) { return int(value) + 100; }
        __global__ void kernel(int *out) {
            out[0] = __popc(true); out[1] = __popc(3u);
            out[2] = __clz(true); out[3] = __clz(3u); out[4] = __clz(3);
        }",
    );
    assert_eq!(call_count(&module.entry_points[0].function), 2);
}

#[test]
fn cuda_integer_intrinsics_distinguish_current_overload_recursion_from_builtin_calls() {
    for (name, parameter, _, _) in INTRINSICS {
        let parameter_name = type_name(parameter);
        validate(&format!("__device__ int {name}(bool value) {{ return int({name}({parameter_name}(value))); }} __global__ void kernel(int *out) {{ out[0] = {name}(true); }}"));
        let source = format!("__device__ int {name}(bool value) {{ return {name}(value); }} __global__ void kernel() {{}}");
        assert!(rejected(&source).message.contains("recurs"));
    }
}

#[test]
fn cuda_integer_intrinsics_reject_canonical_signature_redefinitions() {
    for (name, parameter, _, _) in INTRINSICS {
        let parameter_name = type_name(parameter);
        for result in ["int", "unsigned", "void"] {
            let body = if result == "void" { "" } else { "return 0;" };
            let source = format!("using Argument = const {parameter_name}; __device__ {result} {name}(Argument value) {{ {body} }} __global__ void kernel() {{}}");
            let error = rejected(&source);
            assert!(error.message.contains("duplicate"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
    }
}

#[test]
fn cuda_integer_intrinsics_respect_local_hiding_and_global_name_collisions() {
    for (name, _, _, _) in INTRINSICS {
        for source in [
            format!("__global__ void kernel() {{ int {name} = 1; {name}(1); }}"),
            format!("__device__ int helper(int {name}) {{ return {name}(1); }} __global__ void kernel() {{}}"),
            format!("__global__ void {name}() {{}}"),
            format!("using {name} = int; __global__ void kernel() {{}}"),
        ] {
            assert_eq!(&source[rejected(&source).span.to_range().unwrap()], name);
        }
        validate(&format!("__global__ void kernel(unsigned *out) {{ {{ using {name} = int; out[0] = unsigned({name}(2.75f)); }} out[1] = unsigned({name}(1)); }}"));
    }
}

#[test]
fn cuda_integer_intrinsics_evaluate_arguments_once_before_later_arguments() {
    let (module, _) = validate(
        "__device__ int next(int value) { return value; }
        __device__ int pair(int first, int second) { return first * 100 + second; }
        __global__ void kernel(int *out) {
            int value = 8; out[0] = pair(__ffs(next(value++)), value++);
        }",
    );
    let function = &module.entry_points[0].function;
    assert_eq!(call_count(function), 2);
    let first_argument = function
        .body
        .iter()
        .find_map(|statement| match *statement {
            naga::Statement::Call {
                function: callee,
                ref arguments,
                ..
            } if module.functions[callee].name.as_deref() == Some("pair") => Some(arguments[0]),
            _ => None,
        })
        .unwrap();
    let first_emit = function.body.iter().position(|statement| matches!(*statement, naga::Statement::Emit(ref range) if range.clone().any(|handle| handle == first_argument))).unwrap();
    assert!(function
        .body
        .iter()
        .skip(first_emit + 1)
        .any(|statement| matches!(statement, naga::Statement::Store { .. })));
}

#[test]
fn cuda_integer_intrinsics_stay_in_selected_control_flow() {
    validate(
        "__global__ void kernel(unsigned *out, int choose) {
            int value = 8;
            out[0] = unsigned(choose ? __clz(value++) : __ffs(value++));
            bool active = choose && __popc(unsigned(value++)); out[1] = unsigned(active);
            out[2] = __brev(unsigned(value));
        }",
    );
}

#[test]
fn cuda_integer_intrinsics_require_one_argument_and_reject_malformed_lists() {
    for (name, _, _, _) in INTRINSICS {
        for arguments in ["", "1, 2"] {
            let source = format!("__global__ void kernel() {{ {name}({arguments}); }}");
            let error = rejected(&source);
            assert!(error.message.contains("no matching"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
        rejected(&format!("__global__ void kernel() {{ {name}(1,); }}"));
    }
}

#[test]
fn cuda_integer_intrinsics_reject_vectors_pointers_arrays_and_void_arguments() {
    for (name, _, _, _) in INTRINSICS {
        for scalar in ["float", "int", "uint"] {
            for width in [2, 3, 4] {
                let source = format!(
                    "__global__ void kernel() {{ {scalar}{width} value{{}}; {name}(value); }}"
                );
                let error = rejected(&source);
                assert!(error.message.contains("no matching"));
                assert_eq!(&source[error.span.to_range().unwrap()], name);
            }
        }
        rejected(&format!(
            "__global__ void kernel(int *out) {{ {name}(out); }}"
        ));
        rejected(&format!(
            "__global__ void kernel() {{ int values[2]{{}}; {name}(values); }}"
        ));
        let source =
            format!("__device__ void empty() {{}} __global__ void kernel() {{ {name}(empty()); }}");
        assert_eq!(
            &source[rejected(&source).span.to_range().unwrap()],
            "empty()"
        );
    }
}

#[test]
fn cuda_integer_intrinsics_produce_prvalues_and_validate_dead_calls() {
    for (name, _, _, _) in INTRINSICS {
        validate(&format!(
            "__global__ void kernel() {{ return; {name}(1); }}"
        ));
        for body in [
            format!("{name}(1) = 2;"),
            format!("++{name}(1);"),
            format!("{name}(1)++;"),
            format!("return; {name}(missing);"),
        ] {
            rejected(&format!("__global__ void kernel() {{ {body} }}"));
        }
    }
}

#[test]
fn cuda_integer_intrinsics_count_ffs_adjustment_in_graph_depth_and_reject_64bit_names() {
    let mut value = String::from("value");
    for _ in 0..40 {
        value = format!("__ffs({value})");
    }
    let source = format!("__global__ void kernel(int *out, int value) {{ out[0] = {value}; }}");
    assert!(rejected(&source).message.contains("graph depth"));
    validate("__global__ void kernel(int *out) { out[0] = __ffs(__clz(0)); }");
    for name in ["__clzll", "__popcll", "__brevll", "__ffsll"] {
        let source = format!("__global__ void kernel() {{ {name}(1); }}");
        assert_eq!(&source[rejected(&source).span.to_range().unwrap()], name);
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_integer_intrinsics_roundtrip_shared_wgsl_integer_math() {
    let (module, info) = validate(
        "__global__ void kernel(unsigned *out, int signed_value, unsigned bits) {
            out[0] = unsigned(__clz(signed_value)); out[1] = unsigned(__ffs(signed_value));
            out[2] = unsigned(__popc(bits)); out[3] = __brev(bits);
        }",
    );
    let (output, _) = super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    for name in [
        "countLeadingZeros",
        "firstTrailingBit",
        "countOneBits",
        "reverseBits",
    ] {
        assert!(output.contains(name));
    }
}
