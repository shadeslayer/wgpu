#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

const GUARDED_ADD: &str = include_str!("../../in/cuda/guarded-control-flow.cu");

use super::helpers::validate;

fn global_name<'a>(
    module: &'a naga::Module,
    function: &naga::Function,
    mut handle: naga::Handle<naga::Expression>,
) -> Option<&'a str> {
    loop {
        match function.expressions[handle] {
            naga::Expression::Access { base, .. } | naga::Expression::AccessIndex { base, .. } => {
                handle = base
            }
            naga::Expression::GlobalVariable(global) => {
                return module.global_variables[global].name.as_deref();
            }
            _ => return None,
        }
    }
}

fn buffer_loads(module: &naga::Module, block: &naga::Block, name: &str) -> usize {
    let function = &module.entry_points[0].function;
    block
        .iter()
        .filter_map(|statement| match statement {
            naga::Statement::Emit(range) => Some(range.clone()),
            _ => None,
        })
        .flatten()
        .filter(|&handle| match function.expressions[handle] {
            naga::Expression::Load { pointer } => {
                global_name(module, function, pointer) == Some(name)
            }
            _ => false,
        })
        .count()
}

fn all_buffer_loads(module: &naga::Module, block: &naga::Block, name: &str) -> usize {
    buffer_loads(module, block, name)
        + block
            .iter()
            .map(|statement| match statement {
                naga::Statement::Block(inner) => all_buffer_loads(module, inner, name),
                naga::Statement::If { accept, reject, .. } => {
                    all_buffer_loads(module, accept, name) + all_buffer_loads(module, reject, name)
                }
                _ => 0,
            })
            .sum::<usize>()
}

fn branch(block: &naga::Block) -> (naga::Handle<naga::Expression>, &naga::Block, &naga::Block) {
    block
        .iter()
        .find_map(|statement| match statement {
            naga::Statement::If {
                condition,
                accept,
                reject,
            } => Some((*condition, accept, reject)),
            _ => None,
        })
        .expect("expected an if statement")
}

#[test]
fn cuda_guarded_vector_add_keeps_buffer_reads_in_the_branch() {
    let (module, _) = validate(GUARDED_ADD);
    let function = &module.entry_points[0].function;
    let (_, accept, reject) = branch(&function.body);
    assert!(reject.is_empty());
    for name in ["a", "b"] {
        assert_eq!(buffer_loads(&module, &function.body, name), 0);
        assert_eq!(all_buffer_loads(&module, accept, name), 1);
    }
    assert!(accept.iter().any(|statement| match *statement {
        naga::Statement::Store { pointer, .. } => {
            global_name(&module, function, pointer) == Some("out")
        }
        _ => false,
    }));
}

#[test]
fn cuda_branch_scopes_restore_shadowed_locals() {
    let (module, _) = validate(
        "__global__ void scopes(int *out) { int value = 1;\n\
         if (out[0]) { int value = out[1]; out[2] = value; }\n\
         else { int value = out[3]; out[4] = value; }\n\
         out[5] = value; }",
    );
    let function = &module.entry_points[0].function;
    let locals: Vec<_> = function
        .local_variables
        .iter()
        .filter(|(_, local)| local.name.as_deref() == Some("value"))
        .map(|(handle, _)| handle)
        .collect();
    assert_eq!(locals.len(), 3);
    let value = function
        .body
        .iter()
        .filter_map(|statement| match *statement {
            naga::Statement::Store { pointer, value }
                if global_name(&module, function, pointer) == Some("out") =>
            {
                Some(value)
            }
            _ => None,
        })
        .next_back()
        .unwrap();
    let naga::Expression::Load { pointer } = function.expressions[value] else {
        panic!("expected final read of outer local");
    };
    assert_eq!(
        function.expressions[pointer],
        naga::Expression::LocalVariable(locals[0])
    );
}

#[test]
fn cuda_if_else_binds_to_the_nearest_if() {
    let (module, _) = validate(
        "__global__ void nested(int *out) { if (out[0]) if (out[1]) out[2] = 1; else out[2] = 2; }",
    );
    let (_, outer_accept, outer_reject) = branch(&module.entry_points[0].function.body);
    assert!(outer_reject.is_empty());
    let (_, inner_accept, inner_reject) = branch(outer_accept);
    assert!(!inner_accept.is_empty());
    assert!(!inner_reject.is_empty());
}

#[test]
fn cuda_early_return_guards_later_buffer_access() {
    let (module, _) = validate(
        "__global__ void copy(const int *input, int *out, unsigned int n) {\n\
         unsigned int i = threadIdx.x; if (i >= n) return; out[i] = input[i]; }",
    );
    let body = &module.entry_points[0].function.body;
    let (_, accept, _) = branch(body);
    assert!(accept
        .iter()
        .any(|statement| matches!(statement, naga::Statement::Return { value: None })));
    assert_eq!(buffer_loads(&module, body, "input"), 1);
}

#[test]
fn cuda_unreachable_statements_after_return_are_checked_and_discarded() {
    let (module, _) = validate("__global__ void done(int *out) { return; out[0] = 1; }");
    assert!(!module.entry_points[0]
        .function
        .body
        .iter()
        .any(|statement| matches!(statement, naga::Statement::Store { .. })));
    assert!(parse_str(
        "__global__ void bad(int *out) { return; out[0] = missing; }",
        &Options::default(),
    )
    .is_err());
}

#[test]
fn cuda_boolean_locals_and_literals_preserve_their_type() {
    validate(
        "__global__ void flags(int *out) { bool valid = out[0] < 4; bool enabled = true;\n\
         if (valid && enabled) { valid = false; } if (!valid) out[1] = 1; }",
    );
}

#[test]
fn cuda_logical_operators_short_circuit_buffer_loads() {
    for operator in ["&&", "||"] {
        let source = format!("__global__ void logical(const int *lhs, const int *rhs, int *out) {{ if (lhs[0] {operator} rhs[0]) out[0] = 1; }}");
        let (module, _) = validate(&source);
        let body = &module.entry_points[0].function.body;
        assert_eq!(buffer_loads(&module, body, "lhs"), 1);
        assert_eq!(buffer_loads(&module, body, "rhs"), 0);
        assert_eq!(all_buffer_loads(&module, body, "rhs"), 1);
        let (_, accept, reject) = branch(body);
        let (evaluated, skipped) = if operator == "&&" {
            (accept, reject)
        } else {
            (reject, accept)
        };
        assert_eq!(all_buffer_loads(&module, evaluated, "rhs"), 1);
        assert_eq!(all_buffer_loads(&module, skipped, "rhs"), 0);
    }
}

#[test]
fn cuda_rejects_names_outside_their_block_scope() {
    for source in [
        "__global__ void bad(int *out) { if (out[0]) { int local = 1; } out[1] = local; }",
        "__global__ void bad(int *out) { if (out[0]) { int local = 1; int local = 2; } }",
        "__global__ void bad(int *out) { { int local = 1; } out[0] = local; }",
        "__global__ void bad(int *out) { else { out[0] = 1; } }",
        "__global__ void bad(int *out) { if (out[0]) { return 1; } }",
        "__global__ void bad(bool *out) {}",
        "__global__ void bad(bool value) {}",
        "__global__ void bad(int *out) { if (true ? 1) out[0] = 1; }",
        "__global__ void bad(int *out) { if (true ? : true) out[0] = 1; }",
    ] {
        assert!(parse_str(source, &Options::default()).is_err(), "{source}");
    }
}

#[test]
fn cuda_rejects_excessive_statement_nesting() {
    for prefix in ["{", "if (1) {"] {
        let source = format!(
            "__global__ void deep() {{ {}return;{} }}",
            prefix.repeat(256),
            "}".repeat(256)
        );
        assert!(parse_str(&source, &Options::default()).is_err());
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_guarded_kernel_roundtrips_through_wgsl() {
    let (module, info) = validate(GUARDED_ADD);
    super::helpers::wgsl_roundtrip(&module, &info, naga::back::wgsl::WriterFlags::empty());
}
