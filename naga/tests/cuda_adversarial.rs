#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

fn validate(source: &str) -> naga::Module {
    let module = parse_str(source, &Options::default()).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    module
}

fn integer_value(function: &naga::Function, handle: naga::Handle<naga::Expression>) -> i32 {
    match function.expressions[handle] {
        naga::Expression::Literal(naga::Literal::I32(value)) => value,
        naga::Expression::Unary {
            op: naga::UnaryOperator::Negate,
            expr,
        } => -integer_value(function, expr),
        naga::Expression::Binary { op, left, right } => {
            let left = integer_value(function, left);
            let right = integer_value(function, right);
            match op {
                naga::BinaryOperator::Add => left + right,
                naga::BinaryOperator::Subtract => left - right,
                naga::BinaryOperator::Multiply => left * right,
                naga::BinaryOperator::Divide => left / right,
                _ => panic!("unexpected operator: {op:?}"),
            }
        }
        ref expression => panic!("unexpected expression: {expression:?}"),
    }
}

#[test]
fn cuda_arithmetic_preserves_precedence_and_left_associativity() {
    let module = validate(
        "__global__ void arithmetic(int *out) {\n\
         out[0] = 20 - 3 - 2;\n\
         out[1] = 24 / 3 / 2;\n\
         out[2] = 2 + 3 * 4;\n\
         out[3] = -(2 + 3) * 4;\n\
         out[4] = 20 - (3 - 2);\n\
         }",
    );
    let function = &module.entry_points[0].function;
    let values: Vec<_> = function
        .body
        .iter()
        .filter_map(|statement| match *statement {
            naga::Statement::Store { value, .. } => Some(integer_value(function, value)),
            _ => None,
        })
        .collect();
    assert_eq!(values, [15, 4, 14, -20, 19]);
}

#[test]
fn cuda_mutable_local_loads_follow_assignments() {
    let module = validate(
        "__global__ void update(int *out) {\n\
         int value = out[0];\n\
         out[1] = value;\n\
         value = out[2] + 1;\n\
         out[3] = value;\n\
         }",
    );
    let function = &module.entry_points[0].function;
    let output_values: Vec<_> = function
        .body
        .iter()
        .filter_map(|statement| match *statement {
            naga::Statement::Store { pointer, value }
                if matches!(
                    function.expressions[pointer],
                    naga::Expression::Access { .. }
                ) =>
            {
                Some(value)
            }
            _ => None,
        })
        .collect();
    assert_eq!(output_values.len(), 2);
    assert_ne!(output_values[0], output_values[1]);
}

#[test]
fn cuda_comments_preserve_utf8_and_crlf() {
    validate("/* Δ kernel 漢 */ __global__ void empty() { // café\r\n return; }");
}

#[test]
fn cuda_carriage_return_cannot_hide_a_store() {
    let source = "__global__ void mixed(int *out) { // comment\r out[0] = 1;\n }";
    if let Ok(module) = parse_str(source, &Options::default()) {
        assert!(module.entry_points[0]
            .function
            .body
            .iter()
            .any(|statement| matches!(statement, naga::Statement::Store { .. })));
    }
}

#[test]
fn cuda_unrecognized_utf8_has_a_valid_source_span() {
    let source = "__global__ void empty() { 漢; }";
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert_eq!(source.get(range), Some("漢"));
}

#[test]
fn cuda_rejects_line_splices_before_comment_handling() {
    for ending in ["\n", "\r\n"] {
        for continuation in ["\\", "??/"] {
            let source = format!(
                "__global__ void hidden(int *out) {{ // hidden {continuation}{ending} out[0] = 1;\n }}"
            );
            assert!(parse_str(&source, &Options::default()).is_err());
        }
    }
}

#[test]
fn cuda_rejects_invalid_names_and_mutation() {
    for source in [
        "__global__ void bad(int *if) {}",
        "__global__ void bad(const int const *out) {}",
        "__global__ void bad(int *out) { int return = 1; }",
        "__global__ void bad(int *out) { const int const value = 1; }",
        "__global__ void bad(int *out) { int value = 1; int value = 2; }",
        "__global__ void bad(int *out) { const int value = 1; value = 2; }",
        "__global__ void bad(unsigned int *out) { unsigned int threadIdx = 0u; out[0] = threadIdx.x; }",
        "__global__ void bad(int *out) { int value = 1; (value + 1)++; }",
        "__global__ void bad(int *out) { int value = 1; out[0] = ++(value + 1); }",
        "__global__ void bad(int *out) { int value = 1; out[0] = --(value + 1); }",
        "__global__ void bad(int *out) { out[0] = 2--1; }",
        "__global__ void bad(int *out) { out[0] = 1, 2; }",
        "__global__ void bad(int *out) { out[0] = 1; }\0",
    ] {
        assert!(
            parse_str(source, &Options::default()).is_err(),
            "accepted invalid or unsupported source: {source:?}"
        );
    }
}

#[test]
fn cuda_rejects_excessive_expression_nesting() {
    for expression in [
        format!("{}1{}", "(".repeat(512), ")".repeat(512)),
        format!("{}1", "-".repeat(512)),
        format!("{}1", "1 + ".repeat(512)),
    ] {
        let source = format!("__global__ void deep(int *out) {{ out[0] = {expression}; }}");
        assert!(parse_str(&source, &Options::default()).is_err());
    }
}

#[test]
fn cuda_large_launch_dimensions_do_not_overflow_the_parser() {
    let source = "__global__ void large(unsigned int *out) { out[0] = blockDim.x; }";
    for workgroup_size in [[0x4001, 1, 1], [u32::MAX, 1, 1], [u32::MAX; 3]] {
        if let Ok(module) = parse_str(source, &Options { workgroup_size }) {
            assert_eq!(module.entry_points[0].workgroup_size, workgroup_size);
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::empty(),
            )
            .validate(&module)
            .unwrap();
        }
    }
}
