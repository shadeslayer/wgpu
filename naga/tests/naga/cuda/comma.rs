#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::{execute, validate};

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert!(!error.message.is_empty(), "{source}");
    assert!(source.get(error.span.to_range().unwrap()).is_some());
    error
}

fn rejected_at(source: &str, token: &str, occurrence: usize) {
    let error = rejected(source);
    let start = source.match_indices(token).nth(occurrence).unwrap().0;
    assert_eq!(
        error.span,
        naga::Span::from(start..start + token.len()),
        "{source}: {error}"
    );
}

#[test]
fn cuda_comma_sequences_each_left_operand_before_the_next() {
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                int value = 0;
                out[0] = (value = 1, value += 2, value++);
                out[1] = value;
            }",
            &[0; 2],
        ),
        [3, 4]
    );
}

#[test]
fn cuda_comma_has_lower_precedence_than_assignment() {
    assert_eq!(
        execute("__global__ void bad(int *out) { out[0] = 1, 2; }", &[0]),
        [1]
    );
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                int a = 0, b = 1, c = 2;
                a = b, c; out[0] = a;
                a = (b, c); out[1] = a;
                a = b = 3, c = 4; out[2] = a; out[3] = c;
            }",
            &[0; 4],
        ),
        [1, 2, 3, 4]
    );
}

#[test]
fn cuda_comma_keeps_declarator_separators_outside_parentheses() {
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                int counter = 0;
                int first = (counter += 1, counter + 2), second = (counter += 3, counter);
                out[0] = first; out[1] = second; out[2] = counter;
            }",
            &[0; 3],
        ),
        [3, 4, 4]
    );
}

#[test]
fn cuda_comma_preserves_right_reference_assignment_and_updates() {
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                int x = 2, y = 5;
                (x++, y) = 7;
                out[0] = (x++, y)++;
                out[1] = ++(x++, y);
                out[2] = x; out[3] = y;
            }",
            &[0; 4],
        ),
        [7, 9, 5, 9]
    );
}

#[test]
fn cuda_comma_assignment_evaluates_rhs_before_the_complete_left_address() {
    for (operator, expected) in [("=", [3, 10, 20, 9]), ("+=", [3, 10, 20, 39])] {
        let source = format!(
            "__global__ void kernel(int *out) {{
                int index = 0;
                (index++, out[index]) {operator} (index += 2, 9);
                out[0] = index;
            }}"
        );
        assert_eq!(execute(&source, &[0, 10, 20, 30]), expected);
    }
}

#[test]
fn cuda_comma_preserves_nested_conditional_reference_identity() {
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                int condition = 1, index = 0;
                ((condition = 0, condition ? out[index++] : out[++index]) = 7)++;
                out[3] = index;
            }",
            &[-9; 4],
        ),
        [-9, 8, -9, 1]
    );
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                int condition = 1, index = 0;
                (index++, condition ? out[1] : out[2]) += (condition = 0, 3);
                out[0] = index;
            }",
            &[0, 10, 20],
        ),
        [1, 10, 23]
    );
}

#[test]
fn cuda_comma_discarded_references_evaluate_addresses_without_storage_loads() {
    let source = "__global__ void kernel(int *out) {
        int index = 0;
        (out[index++], true ? out[index++] : out[++index]);
        out[3] = index;
    }";
    assert_eq!(execute(source, &[-9; 4]), [-9, -9, -9, 2]);
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
fn cuda_comma_keeps_only_the_right_type_and_const_qualification() {
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                const int fixed = 7; int value = 1;
                (fixed, value) = 3;
                float floating = (1, 2.75f);
                bool flag = (3u, false); unsigned natural = (false, 4u);
                out[0] = value; out[1] = floating + 0.5f;
                out[2] = flag; out[3] = natural;
            }",
            &[0; 4],
        ),
        [3, 3, 0, 4]
    );
    for expression in [
        "(value, fixed)",
        "(0, true ? value : fixed)",
        "(0, value + 1)",
    ] {
        rejected(&format!(
            "__global__ void kernel() {{ int value = 1; const int fixed = 2; {expression} = 3; }}"
        ));
    }
}

#[test]
fn cuda_comma_conditional_middle_is_full_expression_and_reject_is_assignment() {
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                int value = 0;
                out[0] = true ? value = 1, value + 2 : 8;
                out[1] = false ? 1 : value = 4, value + 10;
                out[2] = (false ? 1 : value = 5, value + 10);
                out[3] = value;
            }",
            &[0; 4],
        ),
        [3, 4, 15, 5]
    );
}

#[test]
fn cuda_comma_stays_inside_selected_short_circuit_and_conditional_branches() {
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                int value = 0;
                false && (value = 2, true);
                true || (value = 3, false);
                out[0] = value;
                out[1] = true && (value = 4, value < 5);
                out[2] = value;
                out[3] = false ? (value = 9, value) : (value += 2, value);
            }",
            &[0; 4],
        ),
        [0, 1, 4, 6]
    );
}

#[test]
fn cuda_comma_keeps_call_argument_separators_and_captures_frontend_argument_order() {
    let (module, _) = validate(
        "__device__ int pair(int first, int second) { return first * 10 + second; }
        __global__ void kernel(int *out) {
            int value = 1; out[0] = pair((value += 1, value), value++);
        }",
    );
    let function = &module.entry_points[0].function;
    let (call_index, arguments) = function
        .body
        .iter()
        .enumerate()
        .find_map(|(index, statement)| match statement {
            naga::Statement::Call { arguments, .. } => Some((index, arguments)),
            _ => None,
        })
        .unwrap();
    assert_eq!(arguments.len(), 2);
    let naga::Expression::Load { pointer } = function.expressions[arguments[0]] else {
        panic!("first argument must capture its value")
    };
    let load_index = function.body.iter().position(|statement| {
        matches!(statement, naga::Statement::Emit(range) if range.clone().any(|handle| handle == arguments[0]))
    }).unwrap();
    assert!(function.body.iter().enumerate().any(|(index, statement)| {
        index > load_index && index < call_index
            && matches!(statement, naga::Statement::Store { pointer: stored, .. } if *stored == pointer)
    }));
}

#[test]
fn cuda_comma_distinguishes_functional_cast_lists_from_full_expressions() {
    assert_eq!(
        execute(
            "using Number = int;
            __global__ void kernel(int *out) {
                out[0] = int((1, 2));
                out[1] = Number((3, 4));
                out[2] = static_cast<int>(5, 6);
                out[(0, 3)] = 7;
                out[true ? 8, 3 : 2] = 7;
            }",
            &[0; 4],
        ),
        [2, 4, 6, 7]
    );
    for expression in ["int(1, 2)", "Number(1, 2)"] {
        rejected_at(
            &format!("using Number = int; __global__ void kernel() {{ {expression}; }}"),
            ",",
            0,
        );
    }
    rejected_at(
        "__global__ void kernel(int *out) { out[0, 1] = 2; }",
        ",",
        0,
    );
}

#[test]
fn cuda_comma_accepts_void_left_operands_and_void_right_discard_or_return() {
    let (module, _) = validate(
        "__device__ void sink(int value) {}
        __device__ int scalar(int value) { return sink(value), value + 1; }
        __device__ void consume(int value) { return value += 1, sink(value); }
        __global__ void kernel(int *out) {
            sink(0), out[0] = scalar((sink(1), 3));
            consume(2), sink(3);
            return out[0], sink(4);
        }",
    );
    assert_eq!(module.functions.len(), 3);
    let function = &module.entry_points[0].function;
    assert!(function.body.iter().any(|statement| matches!(
        statement,
        naga::Statement::Call {
            result: Some(_),
            ..
        }
    )));
    assert!(function
        .body
        .iter()
        .any(|statement| matches!(statement, naga::Statement::Call { result: None, .. })));
}

#[test]
fn cuda_comma_void_right_operands_still_reject_value_contexts() {
    for statement in [
        "int value = (1, sink());",
        "out[0] = (1, sink());",
        "if (1, sink()) {}",
        "scalar((1, sink()));",
        "(1, sink()) + 2;",
    ] {
        let source = format!(
            "__device__ void sink() {{}}
            __device__ int scalar(int value) {{ return value; }}
            __global__ void kernel(int *out) {{ {statement} }}"
        );
        rejected_at(&source, "sink()", 1);
    }
    rejected_at(
        "__device__ void sink() {} __device__ int scalar() { return 1, sink(); }
        __global__ void kernel() {}",
        "sink()",
        1,
    );
}

#[test]
fn cuda_comma_for_clauses_and_loop_conditions_validate_with_continue() {
    validate(
        "__global__ void kernel(int *out) {
            int i = 0, j = 0, checks = 0, total = 0;
            for (i = 0, j = 1; checks += 1, i < 4; ++i, j += 2) {
                if (i == 1) continue;
                total += j;
            }
            while (checks += 1, i < 6) ++i;
            do { ++i; } while (checks += 1, i < 8);
            out[0] = total; out[1] = i; out[2] = checks;
        }",
    );
}

#[test]
fn cuda_comma_unreachable_operands_are_checked_and_dead_calls_compact() {
    let (module, _) = validate(
        "__device__ int helper() { return 2; }
        __device__ void sink() {}
        __global__ void kernel() { return; helper(), sink(), helper(); }",
    );
    assert!(!module.entry_points[0]
        .function
        .body
        .iter()
        .any(|statement| matches!(statement, naga::Statement::Call { .. })));
    rejected_at(
        "__global__ void kernel() { return; 1, missing; }",
        "missing",
        0,
    );
}

#[test]
fn cuda_comma_rejects_malformed_operands_with_utf8_safe_spans() {
    for (statement, token, occurrence) in [
        ("(1,);", ")", 1),
        ("(,1);", ",", 0),
        ("(1,,2);", ",", 1),
        ("out[0] = 1,;", ";", 0),
        ("if (1,) {}", ")", 1),
        ("true ? 1, : 2;", ":", 0),
        ("(1, λ);", "λ", 0),
    ] {
        let source = format!("/* café */ __global__ void kernel(int *out) {{ {statement} }}");
        rejected_at(&source, token, occurrence);
    }
}

#[test]
fn cuda_comma_flat_sequences_do_not_exhaust_the_expression_nesting_limit() {
    let source = format!(
        "__global__ void kernel(int *out) {{ int count = 0; out[0] = ({}count); }}",
        "count += 1, ".repeat(128)
    );
    assert_eq!(execute(&source, &[0]), [128]);
    let source = format!(
        "__global__ void kernel() {{ {}1{}; }}",
        "(0,".repeat(128),
        ")".repeat(128)
    );
    rejected(&source);
}

#[test]
#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
fn cuda_comma_roundtrips_through_wgsl() {
    let (module, info) = validate(
        "__device__ void sink(int value) {}
        __device__ int next(int value) { return value += 1, value; }
        __global__ void kernel(int *out) {
            int first = (sink(1), 2), second = 3;
            for (int i = 0; i < 4; ++i, ++first) {
                if (i == 1) continue;
                (sink(i), i < 2 ? first : second) += next(i);
            }
            out[0] = first; out[1] = second;
        }",
    );
    let text =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let reparsed = naga::front::wgsl::parse_str(&text).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&reparsed)
    .unwrap();
}
