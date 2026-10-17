#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

mod cuda_helpers;
use cuda_helpers::{execute, validate};

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert!(!error.message.is_empty());
    assert!(source.get(error.span.to_range().unwrap()).is_some());
    error
}

fn storage_loads(module: &naga::Module, block: &naga::Block, nested: bool) -> usize {
    let function = &module.entry_points[0].function;
    let mut count = 0;
    for statement in block {
        match statement {
            naga::Statement::Emit(range) => {
                for handle in range.clone() {
                    if let naga::Expression::Load { mut pointer } = function.expressions[handle] {
                        loop {
                            match function.expressions[pointer] {
                                naga::Expression::Access { base, .. }
                                | naga::Expression::AccessIndex { base, .. } => pointer = base,
                                naga::Expression::GlobalVariable(global) => {
                                    count += usize::from(matches!(
                                        module.global_variables[global].space,
                                        naga::AddressSpace::Storage { .. }
                                    ));
                                    break;
                                }
                                _ => break,
                            }
                        }
                    }
                }
            }
            naga::Statement::Block(body) => count += storage_loads(module, body, nested),
            naga::Statement::If { accept, reject, .. } if nested => {
                count += storage_loads(module, accept, nested);
                count += storage_loads(module, reject, nested);
            }
            _ => {}
        }
    }
    count
}

fn statements(block: &naga::Block) -> Vec<&naga::Statement> {
    let mut result = Vec::new();
    for statement in block {
        result.push(statement);
        match statement {
            naga::Statement::Block(body) => result.extend(statements(body)),
            naga::Statement::If { accept, reject, .. } => {
                result.extend(statements(accept));
                result.extend(statements(reject));
            }
            naga::Statement::Loop {
                body, continuing, ..
            } => {
                result.extend(statements(body));
                result.extend(statements(continuing));
            }
            _ => {}
        }
    }
    result
}

fn local_pointer(function: &naga::Function, name: &str) -> naga::Handle<naga::Expression> {
    let local = function
        .local_variables
        .iter()
        .find(|(_, local)| local.name.as_deref() == Some(name))
        .unwrap()
        .0;
    function
        .expressions
        .iter()
        .find_map(|(handle, expression)| {
            matches!(expression, naga::Expression::LocalVariable(candidate) if *candidate == local)
                .then_some(handle)
        })
        .unwrap()
}

fn assert_snapshot_before_mutation(
    function: &naga::Function,
    value: naga::Handle<naga::Expression>,
    pointer: naga::Handle<naga::Expression>,
) {
    let body = statements(&function.body);
    let snapshot = body.iter().position(|statement| {
        matches!(statement, naga::Statement::Emit(range) if range.clone().any(|handle| handle == value))
    }).expect("operand must be evaluated");
    let mutation = body.iter().rposition(|statement| {
        matches!(statement, naga::Statement::Store { pointer: target, .. } if *target == pointer)
    }).unwrap();
    assert!(
        snapshot < mutation,
        "operand must be captured before the later mutation"
    );
}

#[test]
fn cuda_conditional_precedence_and_right_associativity_match_cpp() {
    let source = "__global__ void kernel(int *out) {
        int value = 0;
        out[0] = false ? 1 : true ? 2 : 3;
        out[1] = true ? false ? 4 : 5 : 6;
        out[2] = false || true ? 7 : 8;
        out[3] = true ? value = 9 : value = 10;
        out[4] = value;
        out[5] = false ? 11 : value = 12;
        out[6] = value;
    }";
    assert_eq!(execute(source, &[0; 7]), [2, 5, 7, 9, 9, 12, 12]);
}

#[test]
fn cuda_conditional_conditions_use_scalar_truthiness() {
    let source = "__global__ void kernel(int *out) {
        out[0] = -1 ? 1 : 2;
        out[1] = 0u ? 3 : 4;
        out[2] = -0.0f ? 5 : 6;
        out[3] = 0.5f ? 7 : 8;
        out[4] = false ? 9 : 10;
    }";
    assert_eq!(execute(source, &[0; 5]), [1, 4, 6, 7, 10]);
}

#[test]
fn cuda_conditional_arithmetic_arms_use_a_common_type() {
    for condition in ["false", "true"] {
        let source = format!(
            "__global__ void kernel(int *out) {{
            out[0] = ({condition} ? 1 : 2.5f) * 2.0f;
            out[1] = 0 < ({condition} ? -1 : 1u);
            out[2] = ({condition} ? true : 4) + 2;
            out[3] = {condition} ? true : false;
        }}"
        );
        let expected = if condition == "true" {
            [2, 1, 3, 1]
        } else {
            [5, 1, 6, 0]
        };
        assert_eq!(execute(&source, &[0; 4]), expected);
    }
}

#[test]
fn cuda_conditional_shift_lhs_is_captured_before_rhs_side_effects() {
    for (operator, expected) in [
        ("<<", naga::BinaryOperator::ShiftLeft),
        (">>", naga::BinaryOperator::ShiftRight),
    ] {
        let (module, _) = validate(&format!(
            "__global__ void kernel(int *out, int condition) {{
            int a = 8; int b = 12;
            out[0] = (condition ? a : b) {operator} (a = 1);
        }}"
        ));
        let function = &module.entry_points[0].function;
        let left = function
            .expressions
            .iter()
            .find_map(|(_, expression)| match *expression {
                naga::Expression::Binary { op, left, .. } if op == expected => Some(left),
                _ => None,
            })
            .unwrap();
        assert_snapshot_before_mutation(function, left, local_pointer(function, "a"));
    }
}

#[test]
fn cuda_conditional_evaluates_the_condition_once_before_its_selected_arm() {
    for initial in [0, 1] {
        let source = format!(
            "__global__ void kernel(int *out) {{
            int condition = {initial}; int a = 10; int b = 20;
            int result = condition++ ? ++a : ++b;
            out[0] = result; out[1] = condition; out[2] = a; out[3] = b;
        }}"
        );
        let expected = if initial == 0 {
            [21, 1, 10, 21]
        } else {
            [11, 2, 11, 20]
        };
        assert_eq!(execute(&source, &[0; 4]), expected);
    }
}

#[test]
fn cuda_conditional_buffer_reads_stay_within_the_selected_branch() {
    let (module, _) = validate(
        "__global__ void kernel(int *out, const int *a, const int *b, int condition) {
        out[0] = condition ? a[0] : b[0];
    }",
    );
    let body = &module.entry_points[0].function.body;
    assert_eq!(storage_loads(&module, body, false), 0);
    assert!(storage_loads(&module, body, true) > 0);
}

#[test]
fn cuda_discarded_conditional_references_have_effects_without_storage_reads() {
    for condition in ["false", "true"] {
        let source = format!(
            "__global__ void kernel(int *out) {{
            int a = 0; int b = 1;
            {condition} ? out[a++] : out[b++];
            out[2] = a; out[3] = b;
        }}"
        );
        let expected = if condition == "true" {
            [-9, -9, 1, 1]
        } else {
            [-9, -9, 0, 2]
        };
        assert_eq!(execute(&source, &[-9; 4]), expected);
        let (module, _) = validate(&source);
        assert_eq!(
            storage_loads(&module, &module.entry_points[0].function.body, true),
            0
        );
    }
}

#[test]
fn cuda_discarded_conditional_assignment_results_do_not_load_storage() {
    for condition in ["false", "true"] {
        let source = format!(
            "__global__ void kernel(int *out) {{
            {condition} ? (out[0] = 3) : (out[1] = 4);
        }}"
        );
        let expected = if condition == "true" {
            [3, -9]
        } else {
            [-9, 4]
        };
        assert_eq!(execute(&source, &[-9; 2]), expected);
        let (module, _) = validate(&source);
        assert_eq!(
            storage_loads(&module, &module.entry_points[0].function.body, true),
            0
        );
    }
}

#[test]
fn cuda_matching_conditional_references_accept_simple_assignment() {
    for condition in ["false", "true"] {
        let source = format!(
            "__global__ void kernel(int *out) {{
            int local = 3; ({condition} ? local : out[0]) = 7;
            out[1] = local;
        }}"
        );
        let expected = if condition == "true" { [-9, 7] } else { [7, 3] };
        assert_eq!(execute(&source, &[-9; 2]), expected);
    }
}

#[test]
fn cuda_conditional_compounds_modify_only_the_selected_location() {
    for condition in ["false", "true"] {
        let source = format!(
            "__global__ void kernel(int *out) {{
            int a = 10; int b = 20;
            ({condition} ? a : b) += 3;
            ({condition} ? a : b) *= 2;
            out[0] = a; out[1] = b;
        }}"
        );
        let expected = if condition == "true" {
            [26, 20]
        } else {
            [10, 46]
        };
        assert_eq!(execute(&source, &[0; 2]), expected);
    }
    for operator in ["-=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="] {
        validate(&format!(
            "__global__ void kernel(int *out, int condition) {{
            (condition ? out[0] : out[1]) {operator} 1;
        }}"
        ));
    }
}

#[test]
fn cuda_conditional_prefix_and_postfix_results_keep_their_value_categories() {
    let source = "__global__ void kernel(int *out) {
        int a = 3; int b = 7;
        int old = (true ? a : b)++;
        int current = ++(false ? a : b);
        (++(true ? a : b)) = 9;
        out[0] = old; out[1] = current; out[2] = a; out[3] = b;
    }";
    assert_eq!(execute(source, &[0; 4]), [3, 8, 9, 8]);
}

#[test]
fn cuda_conditional_assignment_and_prefix_arms_remain_references() {
    let source = "__global__ void kernel(int *out) {
        int a = 1; int b = 2;
        (true ? ++a : ++b) = 7;
        (false ? (a = 3) : (b = 4)) += 2;
        out[0] = a; out[1] = b;
    }";
    assert_eq!(execute(source, &[0; 2]), [7, 6]);
}

#[test]
fn cuda_conditional_assignment_result_keeps_its_original_selection() {
    let source = "__global__ void kernel(int *out) {
        int condition = 1; int other = 20;
        int old = ((condition ? condition : other) = 0)++;
        out[0] = condition; out[1] = other; out[2] = old;
    }";
    assert_eq!(execute(source, &[0; 3]), [1, 20, 0]);
}

#[test]
fn cuda_conditional_assignment_and_update_evaluate_selected_index_once() {
    for condition in ["false", "true"] {
        let source = format!(
            "__global__ void kernel(int *out) {{
            int a = 0; int b = 1;
            int old = (({condition} ? out[a++] : out[b++]) = 7)++;
            out[2] = a; out[3] = b; out[4] = old;
        }}"
        );
        let expected = if condition == "true" {
            [8, -9, 1, 1, 7]
        } else {
            [-9, 8, 0, 2, 7]
        };
        assert_eq!(execute(&source, &[-9; 5]), expected);
    }
}

#[test]
fn cuda_nested_conditional_references_preserve_the_selected_address() {
    for (outer, inner, expected) in [
        ("false", "false", [1, 2, 10]),
        ("true", "false", [1, 10, 3]),
        ("true", "true", [10, 2, 3]),
    ] {
        let source = format!(
            "__global__ void kernel(int *out) {{
            int a = 1; int b = 2; int c = 3;
            (({outer} ? ({inner} ? a : b) : c) = 9)++;
            out[0] = a; out[1] = b; out[2] = c;
        }}"
        );
        assert_eq!(execute(&source, &[0; 3]), expected);
    }
}

#[test]
fn cuda_conditional_assignment_rhs_precedes_condition_and_target_evaluation() {
    for (operator, expected) in [("=", [1, 20, 2]), ("+=", [11, 20, 2])] {
        let source = format!(
            "__global__ void kernel(int *out) {{
            out[0] = 10; out[1] = 20; int index = 0;
            ((index++ != 0) ? out[0] : out[1]) {operator} ++index;
            out[2] = index;
        }}"
        );
        assert_eq!(execute(&source, &[-9; 3]), expected);
    }
}

#[test]
fn cuda_conditional_const_references_allow_read_and_discard() {
    let source = "__global__ void kernel(int *out) {
        int mutable_value = 3; const int fixed = 7;
        out[0] = true ? mutable_value : fixed;
        out[1] = false ? fixed : mutable_value;
        true ? fixed : mutable_value;
    }";
    assert_eq!(execute(source, &[0; 2]), [3, 3]);
}

#[test]
fn cuda_conditional_const_join_rejects_modification_for_either_arm_order() {
    for expression in [
        "true ? mutable_value : fixed",
        "false ? fixed : mutable_value",
    ] {
        for modification in [
            format!("({expression}) = 9"),
            format!("({expression}) += 1"),
            format!("++({expression})"),
            format!("({expression})--"),
        ] {
            let source = format!(
                "__global__ void kernel() {{
                int mutable_value = 3; const int fixed = 7; {modification};
            }}"
            );
            rejected(&source);
        }
    }
    rejected(
        "__global__ void kernel(int *out, const int *input) { (true ? out[0] : input[0]) = 1; }",
    );
}

#[test]
fn cuda_conditional_prvalue_and_bool_results_reject_invalid_updates() {
    for expression in [
        "true ? value : 1",
        "false ? 1 : value",
        "true ? value : floating",
        "true ? value : natural",
        "true ? value++ : value++",
        "true ? 1 : 2",
    ] {
        rejected(&format!(
            "__global__ void kernel() {{
            int value = 1; float floating = 2.0f; unsigned natural = 3u;
            ({expression}) = 4;
        }}"
        ));
    }
    for update in ["++(true ? a : b)", "(false ? a : b)--"] {
        rejected(&format!(
            "__global__ void kernel() {{ bool a = false; bool b = true; {update}; }}"
        ));
    }
}

#[test]
fn cuda_conditional_bool_references_can_be_assigned() {
    let source = "__global__ void kernel(int *out) {
        bool a = false; bool b = false;
        (true ? a : b) = true;
        out[0] = a; out[1] = b;
    }";
    assert_eq!(execute(source, &[0; 2]), [1, 0]);
}

#[test]
fn cuda_conditional_helpers_keep_calls_and_arguments_in_selected_arms() {
    validate(
        "__device__ int next(int value) { return value + 1; }
        __device__ int choose(bool condition, int value) {
            return condition ? next(value++) : next(++value);
        }
        __global__ void kernel(int *out) {
            int value = 3; out[0] = true ? choose(false, value++) : next(++value);
        }",
    );
}

#[test]
fn cuda_conditional_call_argument_is_captured_before_the_following_argument() {
    let (module, _) = validate(
        "__device__ int pair(int first, int second) { return first * 10 + second; }
        __global__ void kernel(int *out, int condition) {
            int value = 3; int other = 7;
            out[0] = pair(condition ? value : other, value++);
            out[1] = value;
        }",
    );
    let function = &module.entry_points[0].function;
    let argument = statements(&function.body)
        .into_iter()
        .find_map(|statement| match statement {
            naga::Statement::Call { arguments, .. } => Some(arguments[0]),
            _ => None,
        })
        .unwrap();
    assert_snapshot_before_mutation(function, argument, local_pointer(function, "value"));
}

#[test]
fn cuda_conditional_reference_for_update_remains_in_continuing_after_continue() {
    let (module, _) = validate(
        "__global__ void kernel(int *out, int condition) {
        int i = 0; int j = 0; int count = 0;
        for (; count < 3; condition ? ++i : ++j) {
            ++count; if (count == 2) continue;
        }
        out[0] = i; out[1] = j; out[2] = count;
    }",
    );
    let function = &module.entry_points[0].function;
    let (body, continuing) = statements(&function.body)
        .into_iter()
        .find_map(|statement| match statement {
            naga::Statement::Loop {
                body, continuing, ..
            } => Some((body, continuing)),
            _ => None,
        })
        .unwrap();
    let body = statements(body);
    assert!(body
        .iter()
        .any(|statement| matches!(statement, naga::Statement::Continue)));
    let continuing = statements(continuing);
    for name in ["i", "j"] {
        let pointer = local_pointer(function, name);
        assert!(continuing.iter().any(|statement| matches!(statement, naga::Statement::Store { pointer: target, .. } if *target == pointer)));
        assert!(!body.iter().any(|statement| matches!(statement, naga::Statement::Store { pointer: target, .. } if *target == pointer)));
    }
}

#[test]
fn cuda_conditional_void_arms_work_in_discard_return_and_for_contexts() {
    let (module, _) = validate("__device__ void sink(int value) { if (value) return; }
        __device__ void forward(bool condition) { return condition ? sink(1) : (sink(2)); }
        __global__ void kernel() {
            bool condition = true;
            condition ? sink(1) : sink(2);
            for (condition ? sink(3) : sink(4); condition; condition ? sink(5) : sink(6)) condition = false;
            return condition ? forward(true) : forward(false);
        }");
    for (_, function) in module.functions.iter() {
        assert!(function.result.is_none());
        assert!(!function
            .expressions
            .iter()
            .any(|(_, expression)| matches!(expression, naga::Expression::CallResult(_))));
    }
}

#[test]
fn cuda_conditional_void_results_reject_value_contexts_with_complete_spans() {
    let expression = "condition ? sink(1) : sink(2)";
    for statement in [
        format!("int value = ({expression});"),
        format!("if (({expression})) return;"),
        format!("int value = 1 + ({expression});"),
        format!("sink(({expression}));"),
    ] {
        let source = format!(
            "__device__ void sink(int value) {{}}
            __global__ void kernel() {{ bool condition = true; {statement} }}"
        );
        let error = rejected(&source);
        assert_eq!(&source[error.span.to_range().unwrap()], expression);
    }
}

#[test]
fn cuda_conditional_rejects_mixed_void_arms_and_void_conditions() {
    for expression in [
        "true ? sink(1) : 2",
        "false ? 1 : sink(2)",
        "sink(1) ? 2 : 3",
    ] {
        rejected(&format!(
            "__device__ void sink(int value) {{}}
            __global__ void kernel() {{ {expression}; }}"
        ));
    }
}

#[test]
fn cuda_conditional_dead_calls_are_compacted_but_source_errors_are_checked() {
    validate(
        "__device__ int next(int value) { return value + 1; }
        __device__ int done(bool condition) {
            return condition ? next(1) : next(2);
            { condition ? next(3) : next(4); }
        }
        __global__ void kernel() { return; { true ? done(true) : next(5); } }",
    );
    for statement in [
        "true ? 1 : missing",
        "false ? missing() : 2",
        "true ? (1 = 2) : 3",
    ] {
        rejected(&format!(
            "__global__ void kernel() {{ return; {statement}; }}"
        ));
    }
}

#[test]
fn cuda_conditional_reference_state_is_private_to_each_function() {
    let (module, _) = validate("__device__ int first(bool condition, int value) {
            int other = 2; (condition ? value : other) += 3; return value;
        }
        __device__ float second(bool condition, float value) {
            float other = 2.0f; (condition ? value : other) *= 3.0f; return value;
        }
        __global__ void kernel(int *out, int count) { out[0] = first(true, count) + second(false, count); }");
    assert_eq!(module.global_variables.len(), 2);
    assert_eq!(module.functions.len(), 2);
}

#[test]
fn cuda_conditional_rejects_malformed_and_excessively_nested_source() {
    for expression in ["true ? : 1", "true ? 1 :", "true ? 1", "true ? 1 : 2 : 3"] {
        rejected(&format!("__global__ void kernel() {{ {expression}; }}"));
    }
    let expression = format!("{}1", "false ? 0 : ".repeat(512));
    rejected(&format!(
        "__global__ void kernel() {{ int value = {expression}; }}"
    ));
    let expression = format!("{}1", "false ? 0 : ".repeat(8));
    validate(&format!(
        "__global__ void kernel() {{ int value = {expression}; }}"
    ));
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_conditionals_roundtrip_through_wgsl() {
    let (module, info) = validate(include_str!("in/cuda/conditionals.cu"));
    let source =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let module = naga::front::wgsl::parse_str(&source).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
}
