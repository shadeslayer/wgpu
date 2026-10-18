#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

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
        let (module, _) = validate(&source);
        assert_eq!(
            storage_loads(&module, &module.entry_points[0].function.body, true),
            0
        );
    }
}

#[test]
fn cuda_conditional_compounds_modify_only_the_selected_location() {
    for operator in ["-=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="] {
        validate(&format!(
            "__global__ void kernel(int *out, int condition) {{
            (condition ? out[0] : out[1]) {operator} 1;
        }}"
        ));
    }
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
    let (module, info) = validate(include_str!("../../in/cuda/conditionals.cu"));
    super::helpers::wgsl_roundtrip(&module, &info, naga::back::wgsl::WriterFlags::empty());
}
