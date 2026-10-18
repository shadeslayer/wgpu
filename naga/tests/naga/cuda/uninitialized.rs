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
fn cuda_bare_scalar_declaration_has_no_explicit_initializer_or_store() {
    let (module, _) = validate("__global__ void kernel() { int unused; }");
    let function = &module.entry_points[0].function;
    let local = function.local_variables.iter().next().unwrap().1;
    assert_eq!(local.name.as_deref(), Some("unused"));
    assert_eq!(local.init, None);
    assert_eq!(
        module.types[local.ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::I32)
    );
    assert!(!function
        .body
        .iter()
        .any(|statement| matches!(statement, naga::Statement::Store { .. })));
}

#[test]
fn cuda_mutable_alias_locals_allow_later_initialization() {
    for source in [
        "__global__ void kernel() { const int value; }",
        "__global__ void kernel() { int const value; }",
        "using Frozen = const float; __global__ void kernel() { Frozen value; }",
        "using Frozen = const int; using Copy = Frozen; __global__ void kernel() { Copy value; }",
        "__global__ void kernel() { const int value; value = 1; }",
    ] {
        let occurrence = source.match_indices(';').count() - 1;
        let occurrence = if source.contains("value = 1") {
            0
        } else {
            occurrence
        };
        rejected_at(source, ";", occurrence);
    }
}

#[test]
fn cuda_bare_local_name_binds_inner_storage_without_executing_an_uninitialized_read() {
    let (module, _) =
        validate("__global__ void kernel() { int value = 7; { int value; int copy = value; } }");
    let function = &module.entry_points[0].function;
    let locals: Vec<_> = function
        .local_variables
        .iter()
        .filter_map(|(handle, local)| (local.name.as_deref() == Some("value")).then_some(handle))
        .collect();
    assert_eq!(locals.len(), 2);
    let loaded_local = function.expressions.iter().find_map(|(_, expression)| {
        let naga::Expression::Load { pointer } = *expression else {
            return None;
        };
        let naga::Expression::LocalVariable(local) = function.expressions[pointer] else {
            return None;
        };
        Some(local)
    });
    assert_eq!(loaded_local, Some(locals[1]));
}

#[test]
fn cuda_bare_locals_work_in_scalar_and_void_helpers() {
    let (module, _) = validate(
        "__device__ int increment(int value) { int result; result = value + 1; return result; }
        __host__ __device__ void consume(int value) { float copy; copy = value; if (copy) return; }
        __global__ void kernel(int *out) { int value; value = increment(3); consume(value); out[0] = value; }",
    );
    assert_eq!(module.functions.len(), 2);
    assert!(module.functions.iter().all(|(_, function)| function
        .arguments
        .iter()
        .all(|argument| argument.binding.is_none())));
}

#[test]
fn cuda_bare_loop_locals_are_assigned_before_each_use() {
    validate(
        "__global__ void kernel(int *out) {
            int sum = 0;
            for (int index = 0; index < 3; ++index) {
                int value; value = index + 1;
                if (index == 1) continue;
                sum += value;
            }
            while (sum < 6) { int value; value = sum + 1; sum = value; }
            do { int value; value = sum + 1; sum = value; } while (sum < 7);
            out[0] = sum;
        }",
    );
    for source in [
        "__global__ void kernel() { for (int unused; false;) {} }",
        "__global__ void kernel(int *out) { for (int index; (index = 0) < 1; ++index) { out[0] = index; break; } }",
    ] {
        validate(source);
    }
    rejected_at(
        "__global__ void kernel(int *out) { for (int hidden; false;) {} out[0] = hidden; }",
        "hidden",
        1,
    );
}

#[test]
fn cuda_bare_declarations_keep_duplicate_reserved_and_hidden_name_errors() {
    for (source, token, occurrence) in [
        (
            "/* 漢 */ __global__ void kernel() { int value; int value = 1; }",
            "value",
            1,
        ),
        (
            "__global__ void kernel(int value) { int value; }",
            "value",
            1,
        ),
        (
            "__global__ void kernel() { int class; }",
            "class",
            0,
        ),
        (
            "__device__ int helper() { return 1; } __global__ void kernel() { int helper; helper(); }",
            "helper",
            2,
        ),
    ] {
        rejected_at(source, token, occurrence);
    }
}

#[test]
fn cuda_scalar_declaration_lists_accept_initialized_and_omitted_initializers() {
    validate("__global__ void kernel() { return; int value, other; }");
}

#[test]
fn cuda_bare_declarations_do_not_enable_other_declarators_or_malformed_syntax() {
    for (statement, token) in [
        ("int value trailing;", "trailing"),
        ("int value = ;", ";"),
        ("int value = 1 extra;", "extra"),
        ("int *value;", "*"),
        ("int value[];", "]"),
        ("int(value);", "("),
        ("int value(2);", "("),
        ("int value{};", "{"),
        ("int value = {};", "{"),
    ] {
        let prefix = "/* café 漢 */ __global__ void kernel() { ";
        let source = format!("{prefix}{statement} }}");
        let occurrence = prefix.match_indices(token).count();
        rejected_at(&source, token, occurrence);
    }
    rejected_at("__global__ void kernel() { int value }", "}", 0);
}

#[test]
fn cuda_bare_declarations_in_unreachable_source_remain_checked() {
    validate("__global__ void kernel() { return; { int unused; } }");
    rejected_at(
        "__global__ void kernel() { return; const int value; }",
        ";",
        1,
    );
    rejected_at("__global__ void kernel() { return; int value,; }", ";", 1);
}

#[test]
#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
fn cuda_bare_scalar_locals_roundtrip_through_wgsl() {
    let (module, info) = validate(
        "__device__ float scale(float input) { float result; result = input * 2.0f; return result; }
        __global__ void kernel(float *out) {
            unsigned index; index = threadIdx.x;
            bool enabled; enabled = index < 8;
            float value; if (enabled) value = scale(index); else value = -1.0f;
            out[index] = value;
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
