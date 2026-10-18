#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::{execute, validate};

fn rejected(statement: &str) {
    let source = format!(
        "__device__ int helper() {{ return 5; }}
        __global__ void kernel(int *out) {{ int value = 3; {statement} }}"
    );
    let error = parse_str(&source, &Options::default()).unwrap_err();
    assert!(!error.message.is_empty());
    assert!(source.get(error.span.to_range().unwrap()).is_some());
}

#[test]
fn cuda_rejects_parenthesized_identifier_declarations_instead_of_casting() {
    for statement in ["int(value);", "int((value));", "int(((value)));"] {
        rejected(statement);
    }
}

#[test]
fn cuda_rejects_parenthesized_array_declarations_instead_of_loading() {
    for statement in ["int(out[3]);", "int((out)[3]);", "int((out[3]));"] {
        rejected(statement);
    }
}

#[test]
fn cuda_rejects_parenthesized_function_declarations_instead_of_calling() {
    for statement in ["int(helper());", "int((helper)());", "int((helper()));"] {
        rejected(statement);
    }
}

#[test]
fn cuda_rejects_unsupported_parenthesized_declarators_with_initializers() {
    for statement in ["int(value) = 1;", "int((value)) = 1;", "int(out[3]) = 1;"] {
        rejected(statement);
    }
}

#[test]
fn cuda_unambiguous_constructor_statements_keep_their_expression_meaning() {
    let source = "__global__ void kernel(int *out) {
        int value = 3;
        int(value) + 1;
        int((value)) + 1;
        int(value = 1);
        (int(value));
        int(value + 1);
        static_cast<int>(value);
        out[0] = value;
    }";
    assert_eq!(execute(source, &[0]), [1]);
}

#[test]
fn cuda_constructor_casts_remain_valid_in_initializers_and_arguments() {
    let (module, _) = validate(
        "__device__ int identity(int value) { return value; }
        __global__ void kernel(int *out) {
            int value = 3;
            int result = int(value);
            out[0] = identity(int(value));
            out[1] = static_cast<int>(result);
        }",
    );
    assert_eq!(module.functions.len(), 1);
}

#[test]
fn cuda_constructor_casts_remain_expressions_in_for_clauses() {
    validate(
        "__global__ void kernel() {
        int count = 0;
        for (int(count) + 1; count < 2; int(count)) ++count;
    }",
    );
}

#[test]
fn cuda_parameter_declarators_are_distinguished_from_constructor_call_arguments() {
    for statement in [
        "int(helper(value));",
        "int(helper(4));",
        "int(helper(value + 1));",
        "int(helper(int(value + 1)));",
        "int(helper(float(3)));",
        "int(true);",
        "int(false);",
        "int(bool(int(value)));",
        "int(float(int(value)));",
    ] {
        validate(&format!(
            "__device__ int helper(int value) {{ return value; }}
            __global__ void kernel() {{ int value = 3; {statement} }}"
        ));
    }
    for statement in ["int(helper(int(value)));", "int(helper(float(value)));"] {
        let source = format!(
            "__device__ int helper(int value) {{ return value; }}
            __global__ void kernel() {{ int value = 3; {statement} }}"
        );
        let error = parse_str(&source, &Options::default()).unwrap_err();
        assert!(!error.message.is_empty());
        assert!(source.get(error.span.to_range().unwrap()).is_some());
    }
}
