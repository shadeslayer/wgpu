#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

const INTEGER_KERNEL: &str = include_str!("../../in/cuda/integer-operators.cu");

use super::helpers::validate;

fn scalar(
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    handle: naga::Handle<naga::Expression>,
) -> naga::Scalar {
    info.get_entry_point(0)[handle]
        .ty
        .inner_with(&module.types)
        .scalar()
        .unwrap()
}

#[test]
fn cuda_integral_binary_operators_use_usual_arithmetic_conversions() {
    let operands = [
        ("true", naga::Scalar::I32),
        ("7", naga::Scalar::I32),
        ("7u", naga::Scalar::U32),
    ];
    for (text, operator) in [
        ("%", naga::BinaryOperator::Modulo),
        ("&", naga::BinaryOperator::And),
        ("|", naga::BinaryOperator::InclusiveOr),
        ("^", naga::BinaryOperator::ExclusiveOr),
    ] {
        for (left, left_scalar) in operands {
            for (right, right_scalar) in operands {
                let source = format!(
                    "__global__ void binary() {{ unsigned int result = {left} {text} {right}; }}"
                );
                let (module, info) = validate(&source);
                let expected = if [left_scalar, right_scalar].contains(&naga::Scalar::U32) {
                    naga::Scalar::U32
                } else {
                    naga::Scalar::I32
                };
                let (handle, left, right) = module.entry_points[0]
                    .function
                    .expressions
                    .iter()
                    .find_map(|(handle, expression)| match *expression {
                        naga::Expression::Binary { op, left, right } if op == operator => {
                            Some((handle, left, right))
                        }
                        _ => None,
                    })
                    .unwrap();
                for handle in [handle, left, right] {
                    assert_eq!(scalar(&module, &info, handle), expected, "{source}");
                }
            }
        }
    }
}

#[test]
fn cuda_shifts_promote_each_operand_independently() {
    for (left, expected) in [
        ("true", naga::Scalar::I32),
        ("-8", naga::Scalar::I32),
        ("8u", naga::Scalar::U32),
    ] {
        for right in ["true", "1", "1u"] {
            for (text, operator) in [
                ("<<", naga::BinaryOperator::ShiftLeft),
                (">>", naga::BinaryOperator::ShiftRight),
            ] {
                let source =
                    format!("__global__ void shifts() {{ int result = {left} {text} {right}; }}");
                let (module, info) = validate(&source);
                let (handle, left, right) = module.entry_points[0]
                    .function
                    .expressions
                    .iter()
                    .find_map(|(handle, expression)| match *expression {
                        naga::Expression::Binary { op, left, right } if op == operator => {
                            Some((handle, left, right))
                        }
                        _ => None,
                    })
                    .unwrap();
                assert_eq!(scalar(&module, &info, handle), expected, "{source}");
                assert_eq!(scalar(&module, &info, left), expected, "{source}");
                assert_eq!(scalar(&module, &info, right), naga::Scalar::U32, "{source}");
            }
        }
    }
}

#[test]
fn cuda_bitwise_complement_promotes_bool() {
    for (operand, expected) in [
        ("true", naga::Scalar::I32),
        ("7", naga::Scalar::I32),
        ("7u", naga::Scalar::U32),
    ] {
        let (module, info) = validate(&format!(
            "__global__ void complement() {{ int result = ~{operand}; }}"
        ));
        let (handle, operand) = module.entry_points[0]
            .function
            .expressions
            .iter()
            .find_map(|(handle, expression)| match *expression {
                naga::Expression::Unary {
                    op: naga::UnaryOperator::BitwiseNot,
                    expr,
                } => Some((handle, expr)),
                _ => None,
            })
            .unwrap();
        assert_eq!(scalar(&module, &info, handle), expected);
        assert_eq!(scalar(&module, &info, operand), expected);
    }
}

fn without_cast(
    function: &naga::Function,
    mut handle: naga::Handle<naga::Expression>,
) -> naga::Handle<naga::Expression> {
    loop {
        match function.expressions[handle] {
            naga::Expression::As { expr, .. } => handle = expr,
            naga::Expression::Load { pointer } if matches!(function.expressions[pointer], naga::Expression::LocalVariable(local) if function.local_variables[local].name.is_none()) =>
            {
                handle = function
                    .body
                    .iter()
                    .find_map(|statement| match *statement {
                        naga::Statement::Store {
                            pointer: target,
                            value,
                        } if target == pointer => Some(value),
                        _ => None,
                    })
                    .unwrap();
            }
            _ => return handle,
        }
    }
}

#[test]
fn cuda_integer_precedence_matches_cpp() {
    for (source, outer, nested, nested_right) in [
        (
            "1 << 1 + 1",
            naga::BinaryOperator::ShiftLeft,
            naga::BinaryOperator::Add,
            true,
        ),
        (
            "8 >> 1 < 3",
            naga::BinaryOperator::Less,
            naga::BinaryOperator::ShiftRight,
            false,
        ),
        (
            "1 & 2 == 0",
            naga::BinaryOperator::And,
            naga::BinaryOperator::Equal,
            true,
        ),
        (
            "1 | 2 ^ 3",
            naga::BinaryOperator::InclusiveOr,
            naga::BinaryOperator::ExclusiveOr,
            true,
        ),
        (
            "1 ^ 2 & 3",
            naga::BinaryOperator::ExclusiveOr,
            naga::BinaryOperator::And,
            true,
        ),
    ] {
        let (module, _) = validate(&format!(
            "__global__ void precedence() {{ int result = {source}; }}"
        ));
        let function = &module.entry_points[0].function;
        let value = function
            .body
            .iter()
            .filter_map(|statement| match *statement {
                naga::Statement::Store { value, .. } => Some(without_cast(function, value)),
                _ => None,
            })
            .next_back()
            .unwrap();
        let naga::Expression::Binary { op, left, right } = function.expressions[value] else {
            panic!("{source}")
        };
        assert_eq!(op, outer, "{source}");
        let nested_expression =
            &function.expressions[without_cast(function, if nested_right { right } else { left })];
        assert!(
            matches!(nested_expression, naga::Expression::Binary { op, .. } if *op == nested),
            "{source}: {nested_expression:?}"
        );
    }
    let (module, _) = validate("__global__ void unary() { int result = ~1 << 2; }");
    let function = &module.entry_points[0].function;
    let left = function
        .expressions
        .iter()
        .find_map(|(_, expression)| match *expression {
            naga::Expression::Binary {
                op: naga::BinaryOperator::ShiftLeft,
                left,
                ..
            } => Some(left),
            _ => None,
        })
        .unwrap();
    assert!(matches!(
        function.expressions[left],
        naga::Expression::Unary {
            op: naga::UnaryOperator::BitwiseNot,
            ..
        }
    ));
}

#[test]
fn cuda_shift_lhs_is_loaded_before_rhs_side_effects() {
    for operator in ["<<", ">>"] {
        let (module, _) = validate(&format!("__global__ void sequencing() {{ int value = 1; int result = value {operator} (value = 2); }}"));
        let function = &module.entry_points[0].function;
        let local = function
            .local_variables
            .iter()
            .find(|(_, variable)| variable.name.as_deref() == Some("value"))
            .unwrap()
            .0;
        let pointer = function
            .expressions
            .iter()
            .find_map(|(handle, expression)| match *expression {
                naga::Expression::LocalVariable(handle_local) if handle_local == local => {
                    Some(handle)
                }
                _ => None,
            })
            .unwrap();
        let mut stores = 0;
        let mut loaded_before_rhs = false;
        for statement in &function.body {
            match statement {
                naga::Statement::Store {
                    pointer: target, ..
                } if *target == pointer => stores += 1,
                naga::Statement::Emit(range) => {
                    for handle in range.clone() {
                        if matches!(function.expressions[handle], naga::Expression::Load { pointer: target } if target == pointer)
                            && stores == 1
                        {
                            loaded_before_rhs = true;
                        }
                    }
                }
                _ => {}
            }
        }
        assert!(loaded_before_rhs, "{operator}");
        assert_eq!(stores, 2);
    }
}

#[test]
fn cuda_integer_compounds_accept_modifiable_integral_values() {
    validate(INTEGER_KERNEL);
    for operator in ["%=", "&=", "|=", "^=", "<<=", ">>="] {
        for ty in ["bool", "int", "unsigned int"] {
            validate(&format!(
                "__global__ void compound() {{ {ty} value = 7; value {operator} 1u; }}"
            ));
        }
        validate(&format!(
            "__global__ void indexed(int *out) {{ int i = 1; out[i++] {operator} ++i; }}"
        ));
        for source in [
            format!("__global__ void bad() {{ const int value = 1; value {operator} 1; }}"),
            format!("__global__ void bad() {{ 1 {operator} 1; }}"),
            format!("__global__ void bad(const int *out) {{ out[0] {operator} 1; }}"),
        ] {
            assert!(parse_str(&source, &Options::default()).is_err(), "{source}");
        }
    }
}

#[test]
fn cuda_integer_operators_reject_float_operands() {
    for operator in ["%", "&", "|", "^", "<<", ">>"] {
        for expression in [format!("1.0f {operator} 1"), format!("1 {operator} 1.0f")] {
            let source = format!("__global__ void bad() {{ int value = {expression}; }}");
            assert!(parse_str(&source, &Options::default()).is_err(), "{source}");
        }
        for (ty, rhs) in [("float", "1"), ("int", "1.0f")] {
            let source =
                format!("__global__ void bad() {{ {ty} value = 1; value {operator}= {rhs}; }}");
            assert!(parse_str(&source, &Options::default()).is_err(), "{source}");
        }
    }
    assert!(parse_str(
        "__global__ void bad() { float value = ~1.0f; }",
        &Options::default()
    )
    .is_err());
}

#[test]
fn cuda_integer_expression_depth_remains_bounded() {
    for operator in [" | ", " << "] {
        let source = format!(
            "__global__ void deep() {{ int value = {}1; }}",
            format!("1{operator}").repeat(128)
        );
        assert!(parse_str(&source, &Options::default()).is_err());
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_integer_operators_roundtrip_through_wgsl() {
    for source in [INTEGER_KERNEL, "__global__ void constants(int *out) { out[0] = 1 << 31; out[1] = 4294967295u << 1; out[2] = -7 % 3; out[3] = 7 % -3; out[4] = -7 % -3; out[5] = -8 >> 1u; }"] {
        let (module, info) = validate(source);
        super::helpers::wgsl_roundtrip(&module, &info, naga::back::wgsl::WriterFlags::empty());


    }
}
