#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

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
fn cuda_local_declarators_share_each_supported_scalar_alias_type() {
    for (ty, first, second, scalar) in [
        ("int", "-2", "7", naga::Scalar::I32),
        ("unsigned", "3u", "5u", naga::Scalar::U32),
        ("float", "2.5f", "3.5f", naga::Scalar::F32),
        ("bool", "true", "false", naga::Scalar::BOOL),
    ] {
        let source = format!(
            "using Value = {ty}; __global__ void kernel(int *out) {{
                Value first = {first}, second; second = {second}; out[0] = int(first + second);
            }}"
        );
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
fn cuda_local_declarators_apply_const_to_every_item() {
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
