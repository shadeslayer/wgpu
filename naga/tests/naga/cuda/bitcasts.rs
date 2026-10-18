use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const SIGNATURES: [(&str, &str, naga::Scalar, &str, naga::Scalar); 4] = [
    (
        "__float_as_int",
        "float",
        naga::Scalar::F32,
        "int",
        naga::Scalar::I32,
    ),
    (
        "__float_as_uint",
        "float",
        naga::Scalar::F32,
        "unsigned",
        naga::Scalar::U32,
    ),
    (
        "__int_as_float",
        "int",
        naga::Scalar::I32,
        "float",
        naga::Scalar::F32,
    ),
    (
        "__uint_as_float",
        "unsigned",
        naga::Scalar::U32,
        "float",
        naga::Scalar::F32,
    ),
];

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
fn cuda_bitcasts_convert_all_scalar_inputs_to_the_declared_parameter_type() {
    for (name, _, parameter, result_name, result) in SIGNATURES {
        for declaration in [
            "bool value = true",
            "int value = 3",
            "unsigned value = 5u",
            "float value = 2.75f",
        ] {
            let (module, info) = validate(&format!(
                "__global__ void kernel({result_name} *out) {{ {declaration}; out[0] = {name}(value); }}"
            ));
            let function = &module.entry_points[0].function;
            let bitcasts: Vec<_> = function
                .expressions
                .iter()
                .filter_map(|(handle, expression)| match *expression {
                    naga::Expression::As {
                        expr,
                        convert: None,
                        ..
                    } => Some((handle, expr)),
                    _ => None,
                })
                .collect();
            assert_eq!(bitcasts.len(), 1, "{name}: {declaration}");
            let (value, argument) = bitcasts[0];
            assert_eq!(
                info.get_entry_point(0)[argument]
                    .ty
                    .inner_with(&module.types),
                &naga::TypeInner::Scalar(parameter)
            );
            assert_eq!(
                info.get_entry_point(0)[value].ty.inner_with(&module.types),
                &naga::TypeInner::Scalar(result)
            );
            assert!(module.functions.is_empty());
        }
    }
}

#[test]
fn cuda_bitcasts_preserve_numeric_conversion_before_reinterpretation() {
    let (module, _) = validate(
        "__global__ void kernel(int *out, unsigned value) { out[0] = __float_as_int(value); }",
    );
    let function = &module.entry_points[0].function;
    let argument = function
        .expressions
        .iter()
        .find_map(|(_, expression)| match *expression {
            naga::Expression::As {
                expr,
                kind: naga::ScalarKind::Sint,
                convert: None,
            } => Some(expr),
            _ => None,
        })
        .unwrap();
    assert!(matches!(
        function.expressions[argument],
        naga::Expression::As {
            kind: naga::ScalarKind::Float,
            convert: Some(4),
            ..
        }
    ));

    let (module, _) = validate(
        "__global__ void kernel(float *out, float value) { out[0] = __uint_as_float(value); }",
    );
    let function = &module.entry_points[0].function;
    let pointer = function
        .expressions
        .iter()
        .find_map(|(_, expression)| match *expression {
            naga::Expression::As {
                expr,
                kind: naga::ScalarKind::Float,
                convert: None,
            } => match function.expressions[expr] {
                naga::Expression::Load { pointer } => Some(pointer),
                _ => panic!("float-result bitcast needs a runtime operand"),
            },
            _ => None,
        })
        .unwrap();
    assert!(function.body.iter().any(|statement| match *statement {
        naga::Statement::Store {
            pointer: stored,
            value,
        } if stored == pointer => matches!(
            function.expressions[value],
            naga::Expression::As {
                kind: naga::ScalarKind::Uint,
                convert: Some(4),
                ..
            }
        ),
        _ => false,
    }));
}

#[test]
fn cuda_bitcasts_materialize_literal_float_results_as_runtime_loads() {
    for (name, parameter) in [
        ("__int_as_float", naga::Scalar::I32),
        ("__uint_as_float", naga::Scalar::U32),
    ] {
        for bits in [
            "0u",
            "0x80000000u",
            "0x3f800000u",
            "0x7f800000u",
            "0xff800000u",
            "0x7fc12345u",
            "0x7f800001u",
            "0xff800001u",
        ] {
            let (module, info) = validate(&format!(
                "__global__ void kernel(float *out) {{ out[0] = {name}({bits}); }}"
            ));
            let function = &module.entry_points[0].function;
            let argument = function
                .expressions
                .iter()
                .find_map(|(_, expression)| match *expression {
                    naga::Expression::As {
                        expr,
                        kind: naga::ScalarKind::Float,
                        convert: None,
                    } => Some(expr),
                    _ => None,
                })
                .unwrap();
            let naga::Expression::Load { pointer } = function.expressions[argument] else {
                panic!("{name}({bits}) must not become a const-expression bitcast");
            };
            let naga::Expression::LocalVariable(local) = function.expressions[pointer] else {
                panic!("expected a temporary local");
            };
            assert!(function.local_variables[local].init.is_none());
            assert_eq!(
                info.get_entry_point(0)[argument]
                    .ty
                    .inner_with(&module.types),
                &naga::TypeInner::Scalar(parameter)
            );
            assert!(function.body.iter().any(|statement| matches!(*statement, naga::Statement::Store { pointer: stored, .. } if stored == pointer)));
        }
    }
}

#[test]
fn cuda_bitcasts_accept_aliases_const_inputs_and_helper_results() {
    validate(
        "using Bits = unsigned; using Sample = const float;
        __device__ Sample sample(Bits bits) { return __uint_as_float(bits); }
        __host__ __device__ int signed_bits(Sample value) { return __float_as_int(value); }
        __device__ void consume(int value) { if (__int_as_float(value)) return; }
        __global__ void kernel(Bits *out) {
            const Bits bits = 0x3f800000u; Sample value = sample(bits);
            consume(signed_bits(value)); out[0] = __float_as_uint(value);
        }",
    );
}

#[test]
fn cuda_bitcasts_merge_user_overloads_with_the_fixed_builtin_signature() {
    for (name, parameter_name, _, _, _) in SIGNATURES {
        let (module, _) = validate(&format!(
            "__device__ int {name}(bool value) {{ return value ? 7 : 9; }}
            __device__ int {name}(int first, int second) {{ return first + second; }}
            __global__ void kernel(int *out) {{
                out[0] = {name}(true); out[1] = {name}({parameter_name}(1)); out[2] = {name}(3, 4);
            }}"
        ));
        assert_eq!(call_count(&module.entry_points[0].function), 2);
    }
}

#[test]
fn cuda_bitcasts_distinguish_current_overload_recursion_from_builtin_calls() {
    for (name, parameter_name, _, _, _) in SIGNATURES {
        validate(&format!(
            "__device__ int {name}(bool value) {{ return int({name}({parameter_name}(value))); }}
            __global__ void kernel(int *out) {{ out[0] = {name}(true); }}"
        ));
        let source = format!("__device__ int {name}(bool value) {{ return {name}(value); }} __global__ void kernel() {{}}");
        assert!(rejected(&source).message.contains("recurs"));
    }
}

#[test]
fn cuda_bitcasts_reject_canonical_builtin_signature_redefinitions() {
    for (name, parameter_name, _, _, _) in SIGNATURES {
        for result in ["int", "float", "void"] {
            let body = if result == "void" { "" } else { "return 0;" };
            let source = format!(
                "using Argument = const {parameter_name}; __device__ {result} {name}(Argument value) {{ {body} }} __global__ void kernel() {{}}"
            );
            let error = rejected(&source);
            assert!(error.message.contains("duplicate"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
    }
}

#[test]
fn cuda_bitcasts_respect_local_value_parameter_and_type_name_hiding() {
    for (name, _, _, _, _) in SIGNATURES {
        for source in [
            format!("__global__ void kernel() {{ int {name} = 1; {name}(1); }}"),
            format!("__device__ int helper(int {name}) {{ return {name}(1); }} __global__ void kernel() {{}}"),
        ] {
            assert_eq!(&source[rejected(&source).span.to_range().unwrap()], name);
        }
        validate(&format!(
            "__global__ void kernel(int *out) {{
                {{ using {name} = int; out[0] = {name}(2.5f); }}
                out[1] = int({name}(1));
            }}"
        ));
    }
}

#[test]
fn cuda_bitcasts_reject_kernel_and_global_alias_name_collisions() {
    for (name, _, _, _, _) in SIGNATURES {
        for source in [
            format!("__global__ void {name}() {{}}"),
            format!("using {name} = int; __global__ void kernel() {{}}"),
        ] {
            assert_eq!(&source[rejected(&source).span.to_range().unwrap()], name);
        }
    }
}

#[test]
fn cuda_bitcasts_require_exactly_one_argument() {
    for (name, _, _, _, _) in SIGNATURES {
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
fn cuda_bitcasts_reject_vectors_and_pointer_or_array_arguments() {
    for (name, _, _, _, _) in SIGNATURES {
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
    }
}

#[test]
fn cuda_bitcasts_void_arguments_keep_the_inner_call_span() {
    for (name, _, _, _, _) in SIGNATURES {
        let source =
            format!("__device__ void empty() {{}} __global__ void kernel() {{ {name}(empty()); }}");
        assert_eq!(
            &source[rejected(&source).span.to_range().unwrap()],
            "empty()"
        );
    }
}

#[test]
fn cuda_bitcasts_evaluate_helper_arguments_once_and_capture_before_later_arguments() {
    let (module, _) = validate(
        "__device__ float next(float value) { return value; }
        __device__ int pair(int first, int second) { return first + second; }
        __global__ void kernel(int *out) {
            float value = 1.5f; out[0] = pair(__float_as_int(next(value++)), int(value++));
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
    assert!(matches!(
        function.expressions[first_argument],
        naga::Expression::As {
            kind: naga::ScalarKind::Sint,
            convert: None,
            ..
        }
    ));
    let first_emit = function.body.iter().position(|statement| matches!(*statement, naga::Statement::Emit(ref range) if range.clone().any(|handle| handle == first_argument))).unwrap();
    assert!(function
        .body
        .iter()
        .skip(first_emit + 1)
        .any(|statement| matches!(statement, naga::Statement::Store { .. })));
}

#[test]
fn cuda_bitcasts_produce_prvalues_and_validate_dead_calls() {
    for (name, _, _, _, _) in SIGNATURES {
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
fn cuda_bitcasts_remain_in_selected_control_flow() {
    validate(
        "__global__ void kernel(unsigned *out, int choose) {
            float value = 1.5f;
            out[0] = choose ? __float_as_uint(value++) : __float_as_uint(value++);
            bool active = choose && __float_as_int(value++); out[1] = active;
            out[2] = __float_as_uint(value);
        }",
    );
}

#[test]
fn cuda_bitcasts_obey_expression_depth_and_reject_other_intrinsic_variants() {
    let mut value = String::from("value");
    for _ in 0..40 {
        value = format!("__float_as_int({value})");
    }
    let source = format!("__global__ void kernel(int *out, float value) {{ out[0] = {value}; }}");
    assert!(rejected(&source).message.contains("graph depth"));
    validate(
        "__global__ void kernel(unsigned *out) { out[0] = __float_as_uint(__int_as_float(3)); }",
    );
    for name in [
        "__float_as_int2",
        "__float_as_uint64",
        "__double_as_longlong",
        "__longlong_as_double",
    ] {
        let source = format!("__global__ void kernel() {{ {name}(1); }}");
        assert_eq!(&source[rejected(&source).span.to_range().unwrap()], name);
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_bitcasts_roundtrip_literal_finite_infinite_and_nan_float_results() {
    let (module, info) = validate(
        "__global__ void kernel(unsigned *out) {
            out[0] = __float_as_uint(__int_as_float(0x7f800000));
            out[1] = __float_as_uint(__uint_as_float(0xff800000u));
            out[2] = __float_as_uint(__int_as_float(0x7fc12345));
            out[3] = __float_as_uint(__uint_as_float(0x7f800001u));
            out[4] = __float_as_uint(__int_as_float(0x3f800000));
            out[5] = __float_as_uint(__uint_as_float(1065353216.0f));
        }",
    );
    let (output, _) = super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    assert!(output.contains("bitcast<f32>"));
    assert!(output.contains("bitcast<u32>"));
}
