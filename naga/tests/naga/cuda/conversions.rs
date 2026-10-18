#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

const INDEXED: &str = include_str!("../../in/cuda/numeric-conversions.cu");

const TYPES: [(&str, &str, naga::Scalar); 4] = [
    ("bool", "true", naga::Scalar::BOOL),
    ("int", "1", naga::Scalar::I32),
    ("unsigned int", "1u", naga::Scalar::U32),
    ("float", "1.5f", naga::Scalar::F32),
];

use super::helpers::validate;

fn expression_scalar(
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

const COMMON: [[naga::Scalar; 4]; 4] = [
    [
        naga::Scalar::I32,
        naga::Scalar::I32,
        naga::Scalar::U32,
        naga::Scalar::F32,
    ],
    [
        naga::Scalar::I32,
        naga::Scalar::I32,
        naga::Scalar::U32,
        naga::Scalar::F32,
    ],
    [
        naga::Scalar::U32,
        naga::Scalar::U32,
        naga::Scalar::U32,
        naga::Scalar::F32,
    ],
    [
        naga::Scalar::F32,
        naga::Scalar::F32,
        naga::Scalar::F32,
        naga::Scalar::F32,
    ],
];

#[test]
fn cuda_usual_arithmetic_conversions_cover_supported_scalar_pairs() {
    for (row, (_, left, _)) in TYPES.into_iter().enumerate() {
        for (column, (_, right, _)) in TYPES.into_iter().enumerate() {
            let scalar = COMMON[row][column];
            let ty = TYPES
                .iter()
                .find(|(_, _, candidate)| *candidate == scalar)
                .unwrap()
                .0;
            let source =
                format!("__global__ void arithmetic() {{ {ty} value = {left} + {right}; }}");
            let (module, info) = validate(&source);
            let function = &module.entry_points[0].function;
            let (handle, left, right) = function
                .expressions
                .iter()
                .find_map(|(handle, expression)| match *expression {
                    naga::Expression::Binary {
                        op: naga::BinaryOperator::Add,
                        left,
                        right,
                    } => Some((handle, left, right)),
                    _ => None,
                })
                .unwrap();
            for expression in [handle, left, right] {
                assert_eq!(
                    expression_scalar(&module, &info, expression),
                    scalar,
                    "{source}"
                );
            }
        }
    }
}

#[test]
fn cuda_comparisons_convert_both_operands_before_comparing() {
    for (row, (_, left, _)) in TYPES.into_iter().enumerate() {
        for (column, (_, right, _)) in TYPES.into_iter().enumerate() {
            let scalar = COMMON[row][column];
            let source =
                format!("__global__ void comparison() {{ bool result = {left} < {right}; }}");
            let (module, info) = validate(&source);
            let function = &module.entry_points[0].function;
            let (handle, left, right) = function
                .expressions
                .iter()
                .find_map(|(handle, expression)| match *expression {
                    naga::Expression::Binary {
                        op: naga::BinaryOperator::Less,
                        left,
                        right,
                    } => Some((handle, left, right)),
                    _ => None,
                })
                .unwrap();
            assert_eq!(
                expression_scalar(&module, &info, handle),
                naga::Scalar::BOOL
            );
            for expression in [left, right] {
                assert_eq!(
                    expression_scalar(&module, &info, expression),
                    scalar,
                    "{source}"
                );
            }
        }
    }
}

#[test]
fn cuda_initializers_and_assignments_convert_to_the_destination_type() {
    for (ty, _, scalar) in TYPES {
        for (_, expression, _) in TYPES {
            let source = format!(
                "__global__ void assign() {{ {ty} value = {expression}; value = {expression}; }}"
            );
            let (module, info) = validate(&source);
            let function = &module.entry_points[0].function;
            let values: Vec<_> = function
                .body
                .iter()
                .filter_map(|statement| match *statement {
                    naga::Statement::Store { value, .. } => Some(value),
                    _ => None,
                })
                .collect();
            assert_eq!(values.len(), 2);
            for value in values {
                assert_eq!(expression_scalar(&module, &info, value), scalar, "{source}");
            }
        }
    }
}

#[test]
fn cuda_explicit_scalar_casts_share_conversion_semantics() {
    for (ty, _, scalar) in TYPES {
        for (_, expression, _) in TYPES {
            for cast in [
                format!("({ty}){expression}"),
                format!("static_cast<{ty}>({expression})"),
            ] {
                let source = format!("__global__ void casts() {{ {ty} value = {cast}; }}");
                let (module, info) = validate(&source);
                let function = &module.entry_points[0].function;
                let value = function
                    .body
                    .iter()
                    .find_map(|statement| match *statement {
                        naga::Statement::Store { value, .. } => Some(value),
                        _ => None,
                    })
                    .unwrap();
                assert_eq!(expression_scalar(&module, &info, value), scalar, "{source}");
            }
        }
    }
}

#[test]
fn cuda_functional_casts_support_single_token_scalar_types() {
    for (ty, _, _) in TYPES.into_iter().filter(|(ty, _, _)| *ty != "unsigned int") {
        for (_, expression, _) in TYPES {
            validate(&format!(
                "__global__ void casts() {{ {ty} value = {ty}({expression}); }}"
            ));
        }
    }
    validate("__global__ void cast_unsigned() { unsigned value = unsigned(-1); }");
}

#[test]
fn cuda_casts_bind_at_unary_precedence() {
    for source in [
        "__global__ void casts(float *out) { out[0] = (float)1 / 2; }",
        "__global__ void casts(int *out) { out[0] = (int)1.5f + 2; }",
        "__global__ void casts(int *out) { out[0] = (int)(1.5f + 2.0f); }",
        "__global__ void casts(int *out) { out[0] = -static_cast<int>(1.5f); }",
    ] {
        validate(source);
    }
}

#[test]
fn cuda_unary_arithmetic_promotes_bool_and_wraps_unsigned_negation() {
    let (module, info) = validate(
        "__global__ void unary() { int positive = +true; int negative = -true; unsigned int wrapped = -1u; }",
    );
    let function = &module.entry_points[0].function;
    let values: Vec<_> = function
        .body
        .iter()
        .filter_map(|statement| match *statement {
            naga::Statement::Store { value, .. } => Some(value),
            _ => None,
        })
        .collect();
    let expected = [naga::Scalar::I32, naga::Scalar::I32, naga::Scalar::U32];
    assert_eq!(values.len(), expected.len());
    for (value, scalar) in values.into_iter().zip(expected) {
        assert_eq!(expression_scalar(&module, &info, value), scalar);
    }
}

#[test]
fn cuda_signed_indices_and_unsigned_bounds_use_usual_comparison_conversions() {
    let (module, info) = validate(INDEXED);
    let function = &module.entry_points[0].function;
    let (left, right) = function
        .expressions
        .iter()
        .find_map(|(_, expression)| match *expression {
            naga::Expression::Binary {
                op: naga::BinaryOperator::Less,
                left,
                right,
            } => Some((left, right)),
            _ => None,
        })
        .unwrap();
    assert_eq!(expression_scalar(&module, &info, left), naga::Scalar::U32);
    assert_eq!(expression_scalar(&module, &info, right), naga::Scalar::U32);
}

#[test]
fn cuda_conversions_preserve_unsupported_type_and_literal_errors() {
    for source in [
        "__global__ void bad(int *out) { out[0] = (double)1; }",
        "__global__ void bad(int *out) { out[0] = static_cast<long>(1); }",
        "__global__ void bad(int *out) { out[0] = (int*)1; }",
        "__global__ void bad(int *out) { out[0] = static_cast<int*>(out); }",
        "__global__ void bad(int *out) { out[0] = static_cast<int>(out); }",
        "__global__ void bad(int *out) { out[0] = 2147483648; }",
        "__global__ void bad(int *out) { out[0] = 4294967296u; }",
        "__global__ void bad(float *out) { out[0] = 1.0; }",
        "__global__ void bad(bool value) {}",
    ] {
        assert!(parse_str(source, &Options::default()).is_err(), "{source}");
    }
}

#[test]
fn cuda_casts_contribute_to_the_expression_graph_depth_limit() {
    let source = format!(
        "__global__ void deep() {{ bool value = 1.0f{}; }}",
        " == 1.0f".repeat(128)
    );
    assert!(parse_str(&source, &Options::default()).is_err());
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_numeric_conversions_roundtrip_through_wgsl() {
    let (module, info) = validate(INDEXED);
    super::helpers::wgsl_roundtrip(&module, &info, naga::back::wgsl::WriterFlags::empty());
}
