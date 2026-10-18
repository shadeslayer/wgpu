#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::{execute, validate};

fn rejected_at(source: &str, token: &str, occurrence: usize) {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let start = source.match_indices(token).nth(occurrence).unwrap().0;
    assert!(!error.message.is_empty(), "{source}");
    assert_eq!(
        error.span,
        naga::Span::from(start..start + token.len()),
        "{source}: {error}"
    );
    assert!(source.get(error.span.to_range().unwrap()).is_some());
}

#[test]
fn cuda_local_declarators_finish_each_initializer_before_the_next() {
    for (body, expected) in [
        (
            "int first = 1, second = first + 2, third = second + 3;
            out[0] = first; out[1] = second; out[2] = third;",
            [1, 3, 6],
        ),
        (
            "int counter = 1; int first = counter++, second = counter++, third = counter;
            out[0] = first; out[1] = second; out[2] = third;",
            [1, 2, 3],
        ),
        (
            "int value = 5; int first = value, second = (value = 9), third = first;
            out[0] = first; out[1] = second; out[2] = third;",
            [5, 9, 5],
        ),
    ] {
        let source = format!("__global__ void kernel(int *out) {{ {body} }}");
        assert_eq!(execute(&source, &[0; 3]), expected);
    }
}

#[test]
fn cuda_local_declarators_allow_mixed_omitted_and_initialized_values() {
    for body in [
        "int first, second = 3, third; first = second + 1; third = first + second;",
        "int first = 4, second, third = first + 3; second = third - first;",
        "int first, second, third; first = 4; second = 3; third = 7;",
    ] {
        let source = format!(
            "__global__ void kernel(int *out) {{ {body} out[0] = first; out[1] = second; out[2] = third; }}"
        );
        assert_eq!(execute(&source, &[0; 3]), [4, 3, 7]);
    }
}

#[test]
fn cuda_local_declarators_share_each_supported_scalar_alias_type() {
    for (ty, first, second, scalar, expected) in [
        ("int", "-2", "7", naga::Scalar::I32, 5),
        ("unsigned", "3u", "5u", naga::Scalar::U32, 8),
        ("float", "2.5f", "3.5f", naga::Scalar::F32, 6),
        ("bool", "true", "false", naga::Scalar::BOOL, 1),
    ] {
        let source = format!(
            "using Value = {ty}; __global__ void kernel(int *out) {{
                Value first = {first}, second; second = {second}; out[0] = int(first + second);
            }}"
        );
        assert_eq!(execute(&source, &[0]), [expected]);
        let (module, _) = validate(&source);
        for (_, local) in module.entry_points[0].function.local_variables.iter() {
            if matches!(local.name.as_deref(), Some("first" | "second")) {
                assert_eq!(
                    module.types[local.ty].inner,
                    naga::TypeInner::Scalar(scalar)
                );
            }
        }
    }
}

#[test]
fn cuda_local_declarators_cache_the_type_before_a_global_alias_is_hidden() {
    for body in [
        "Value Value = 3, next = Value + 1;",
        "Value Value, next; Value = 3; next = Value + 1;",
    ] {
        let source = format!(
            "using Value = int; __global__ void kernel(int *out) {{ {body} out[0] = Value; out[1] = next; }}"
        );
        assert_eq!(execute(&source, &[0; 2]), [3, 4]);
    }
}

#[test]
fn cuda_local_declarators_apply_const_to_every_item() {
    for prefix in ["const int", "int const", "Frozen"] {
        let source = format!(
            "using Frozen = const int; __global__ void kernel(int *out) {{
                {prefix} first = 3, second = first + 1; out[0] = second;
            }}"
        );
        assert_eq!(execute(&source, &[0]), [4]);
    }
    rejected_at(
        "__global__ void kernel() { const int first, second = 2; }",
        ",",
        0,
    );
    rejected_at(
        "__global__ void kernel() { const int first = 1, second; }",
        ";",
        0,
    );
    rejected_at(
        "using Frozen = const int; __global__ void kernel() { Frozen first = 1, second; }",
        ";",
        1,
    );
    rejected_at(
        "using Frozen = const int; __global__ void kernel() { Frozen first = 1, second = 2; second = 3; }",
        "second",
        1,
    );
}

#[test]
fn cuda_local_declarator_names_enter_scope_only_when_reached() {
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                int value = 9;
                { int first = value, value = 3; out[0] = first; out[1] = value; }
                out[2] = value;
            }",
            &[0; 3]
        ),
        [9, 3, 9]
    );
}

#[test]
fn cuda_local_declarator_initializers_bind_their_own_names_in_ir() {
    let (module, _) = validate(
        "__global__ void kernel() { int value = 7; { int first = value, value = value; } }",
    );
    let function = &module.entry_points[0].function;
    let locals: Vec<_> = function
        .local_variables
        .iter()
        .filter_map(|(handle, local)| (local.name.as_deref() == Some("value")).then_some(handle))
        .collect();
    let loaded: Vec<_> = function
        .expressions
        .iter()
        .filter_map(|(_, expression)| {
            let naga::Expression::Load { pointer } = *expression else {
                return None;
            };
            let naga::Expression::LocalVariable(local) = function.expressions[pointer] else {
                return None;
            };
            Some(local)
        })
        .collect();
    assert_eq!(locals.len(), 2);
    assert_eq!(loaded, locals);
}

#[test]
fn cuda_local_declarators_load_conditional_references_before_later_side_effects() {
    assert_eq!(
        execute(
            "__global__ void kernel(int *out) {
                int first = 3, second = 9;
                int selected = true ? first : second, changed = (first = 7);
                out[0] = selected; out[1] = changed;
            }",
            &[0; 2]
        ),
        [3, 7]
    );
}

#[test]
fn cuda_local_declarator_initializers_keep_nested_call_argument_commas() {
    let (module, _) = validate(
        "__device__ int add(int a, int b) { int first = a, second = b, result; result = first + second; return result; }
        __host__ __device__ void consume(int value) { int first = value, second; second = first; if (second) return; }
        __global__ void kernel(int *out) {
            int first = add(1, add(2, 3)), second = add(first, 4);
            consume(second); out[0] = second;
        }",
    );
    let calls = module.entry_points[0]
        .function
        .body
        .iter()
        .filter(|statement| matches!(statement, naga::Statement::Call { .. }))
        .count();
    assert_eq!(module.functions.len(), 2);
    assert_eq!(calls, 4);
    validate(
        "__device__ int helper() { return 4; }
        __global__ void kernel(int *out) { int first = helper(), helper = 7; out[0] = first + helper; }",
    );
    rejected_at(
        "__device__ int helper() { return 4; }
        __global__ void kernel() { int first = 1, helper = helper(); }",
        "helper",
        2,
    );
}

#[test]
fn cuda_local_declarators_work_in_for_initializers_and_loop_bodies() {
    validate(
        "__global__ void kernel(int *out) {
            int sum = 0;
            for (int index = 0, limit = 3; index < limit; ++index) {
                int first = index + 1, second; second = first;
                if (index == 1) continue;
                sum += second;
            }
            for (int staged, value = 3; value < 5; ++value) { staged = value; sum += staged; }
            out[0] = sum;
        }",
    );
    rejected_at(
        "__global__ void kernel() { for (int first = 0, hidden = 1; first < hidden; ++first) {} hidden = 3; }",
        "hidden",
        2,
    );
}

#[test]
fn cuda_local_declarators_reject_duplicates_reserved_names_and_type_collisions() {
    for (source, token, occurrence) in [
        (
            "/* 漢 */ __global__ void kernel() { int value = 1, value = 2; }",
            "value",
            1,
        ),
        (
            "__global__ void kernel(int value) { int first = 1, value = 2; }",
            "value",
            1,
        ),
        (
            "__global__ void kernel() { int first = 1, class = 2; }",
            "class",
            0,
        ),
        (
            "__global__ void kernel() { using Value = int; int first = 1, Value = 2; }",
            "Value",
            1,
        ),
    ] {
        rejected_at(source, token, occurrence);
    }
}

#[test]
fn cuda_local_declarators_keep_other_declarator_forms_unsupported() {
    for (statement, token, occurrence) in [
        ("int first = 1,;", ";", 0),
        ("int first,;", ";", 0),
        ("int first = 1,, second = 2;", ",", 1),
        ("int first = 1, second = ;", ";", 0),
        ("int first = 1, *second;", "*", 0),
        ("int first = 1, second[2];", "[", 0),
        ("int first = 1, (second);", "(", 0),
        ("int first = 1, second(2);", "(", 0),
        ("int first = 1, second{};", "{", 0),
        ("int first = 1, float second = 2;", "float", 0),
        ("int first = 1, const second = 2;", "const", 0),
        ("__shared__ int first, second;", ",", 0),
    ] {
        let prefix = "/* café 漢 */ __global__ void kernel() { ";
        let source = format!("{prefix}{statement} }}");
        rejected_at(
            &source,
            token,
            prefix.match_indices(token).count() + occurrence,
        );
    }
}

#[test]
fn cuda_local_declarators_allow_comma_expressions_in_full_expression_contexts() {
    for (statement, expected) in [
        ("int first = (1, 2); out[0] = first;", [2, 0]),
        ("int first = true ? 1, 2 : 3; out[0] = first;", [2, 0]),
        ("out[0] = 1, out[1] = 2;", [1, 2]),
    ] {
        let source = format!("__global__ void kernel(int *out) {{ {statement} }}");
        assert_eq!(execute(&source, &[0; 2]), expected);
    }
    validate("__global__ void kernel() { for (int first = 0; first < 1; ++first, ++first) {} }");
}

#[test]
fn cuda_local_declarators_in_unreachable_source_are_still_checked() {
    validate(
        "__device__ int helper() { return 1; }
        __global__ void kernel() { return; { int first = helper(), second = helper(), unused; } }",
    );
    rejected_at(
        "__global__ void kernel() { return; int first = 1, second = ; }",
        ";",
        1,
    );
    rejected_at(
        "__device__ void consume() {} __global__ void kernel() { int first = 1, second = consume(); }",
        "consume()",
        1,
    );
}

#[test]
fn cuda_local_declarator_lists_do_not_add_expression_nesting() {
    let declarators = (0..128)
        .map(|index| {
            if index == 0 {
                String::from("value0 = 0")
            } else {
                format!("value{index} = value{} + 1", index - 1)
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    let source =
        format!("__global__ void kernel(int *out) {{ int {declarators}; out[0] = value127; }}");
    assert_eq!(execute(&source, &[0]), [127]);
}

#[test]
#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
fn cuda_local_declarators_roundtrip_through_wgsl() {
    let (module, info) = validate(
        "using Weight = float;
        __device__ Weight add(Weight a, Weight b) { Weight first = a, second = b; return first + second; }
        __global__ void kernel(float *out) {
            unsigned index = threadIdx.x, limit = 8;
            bool enabled = index < limit, selected = !enabled;
            Weight first, second = add(index, 2.5f); first = enabled ? second : -1.0f;
            out[index] = selected ? 0.0f : first;
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
