#![cfg(feature = "cuda-in")]

mod cuda_helpers;
use cuda_helpers::{execute, validate};

#[test]
fn cuda_discarded_reference_evaluates_its_index_without_loading_storage() {
    let source = "__global__ void kernel(int *out) {
        int index = 0; out[index++]; out[2] = index;
    }";
    assert_eq!(execute(source, &[-9; 3]), [-9, -9, 1]);
    let (module, _) = validate(source);
    let function = &module.entry_points[0].function;
    for (_, expression) in function.expressions.iter() {
        if let naga::Expression::Load { mut pointer } = *expression {
            loop {
                match function.expressions[pointer] {
                    naga::Expression::Access { base, .. }
                    | naga::Expression::AccessIndex { base, .. } => pointer = base,
                    naga::Expression::GlobalVariable(_) => panic!("discarded storage load"),
                    _ => break,
                }
            }
        }
    }
}

#[test]
fn cuda_assignment_result_keeps_the_original_indexed_reference() {
    let source = "__global__ void kernel(int *out) {
        int index = 0; (out[index++] = 7) = 9; out[2] = index;
    }";
    assert_eq!(execute(source, &[-9; 3]), [9, -9, 1]);
}

#[test]
fn cuda_prefix_result_keeps_the_original_indexed_reference() {
    let source = "__global__ void kernel(int *out) {
        out[0] = 3; int index = 0; (++out[index++]) = 8; out[2] = index;
    }";
    assert_eq!(execute(source, &[-9; 3]), [8, -9, 1]);
}

#[test]
fn cuda_postfix_result_is_the_old_value_of_the_selected_location() {
    let source = "__global__ void kernel(int *out) {
        out[0] = 3; int index = 0; int previous = out[index++]++;
        out[1] = previous; out[2] = index;
    }";
    assert_eq!(execute(source, &[-9; 3]), [4, 3, 1]);
}

#[test]
fn cuda_reference_assignment_and_compound_evaluate_rhs_before_index() {
    for (operator, expected) in [("=", [10, 1, 2]), ("+=", [10, 21, 2])] {
        let source = format!(
            "__global__ void kernel(int *out) {{
            out[0] = 10; out[1] = 20; int index = 0;
            out[index++] {operator} ++index; out[2] = index;
        }}"
        );
        assert_eq!(execute(&source, &[-9; 3]), expected);
    }
}
