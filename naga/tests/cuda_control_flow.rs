#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

const GUARDED_ADD: &str = r#"
__global__ void add(const float *a, const float *b, float *out, unsigned int n) {
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        out[i] = a[i] + b[i];
    }
}
"#;

fn validate(source: &str) -> (naga::Module, naga::valid::ModuleInfo) {
    let module = parse_str(source, &Options::default()).unwrap();
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    (module, info)
}

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

fn condition_value(
    module: &naga::Module,
    handle: naga::Handle<naga::Expression>,
    input: f64,
) -> f64 {
    let function = &module.entry_points[0].function;
    let evaluate = |handle| condition_value(module, handle, input);
    match function.expressions[handle] {
        naga::Expression::Literal(naga::Literal::I32(value)) => f64::from(value),
        naga::Expression::Literal(naga::Literal::U32(value)) => f64::from(value),
        naga::Expression::Literal(naga::Literal::F32(value)) => f64::from(value),
        naga::Expression::Literal(naga::Literal::Bool(value)) => f64::from(u8::from(value)),
        naga::Expression::Load { pointer }
            if global_name(module, function, pointer) == Some("lhs") =>
        {
            input
        }
        naga::Expression::Unary { op, expr } => match op {
            naga::UnaryOperator::Negate => -evaluate(expr),
            naga::UnaryOperator::LogicalNot => f64::from(u8::from(evaluate(expr) == 0.0)),
            _ => panic!("unexpected unary operator: {op:?}"),
        },
        naga::Expression::Binary { op, left, right } => {
            let left = evaluate(left);
            let right = evaluate(right);
            let result = match op {
                naga::BinaryOperator::Equal => left == right,
                naga::BinaryOperator::NotEqual => left != right,
                naga::BinaryOperator::Less => left < right,
                naga::BinaryOperator::LessEqual => left <= right,
                naga::BinaryOperator::Greater => left > right,
                naga::BinaryOperator::GreaterEqual => left >= right,
                _ => panic!("unexpected condition operator: {op:?}"),
            };
            f64::from(u8::from(result))
        }
        ref expression => panic!("unexpected condition expression: {expression:?}"),
    }
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
fn cuda_scalar_conditions_compare_against_zero() {
    for (expression, expected) in [
        ("0", false),
        ("-3", true),
        ("0u", false),
        ("2u", true),
        ("0.0f", false),
        ("-0.0f", false),
        ("-0.5f", true),
        ("!0", true),
        ("!3u", false),
        ("!-0.5f", false),
    ] {
        let source = format!("__global__ void truth(int *out) {{ if ({expression}) out[0] = 1; }}");
        let (module, _) = validate(&source);
        let (condition, _, _) = branch(&module.entry_points[0].function.body);
        assert_eq!(
            condition_value(&module, condition, 0.0) != 0.0,
            expected,
            "{expression}"
        );
    }
}

#[test]
fn cuda_comparisons_produce_boolean_conditions() {
    for (operator, expected) in [
        ("<", true),
        ("<=", true),
        (">", false),
        (">=", false),
        ("==", false),
        ("!=", true),
    ] {
        let source =
            format!("__global__ void compare(int *out) {{ if (2 {operator} 3) out[0] = 1; }}");
        let (module, _) = validate(&source);
        let (condition, _, _) = branch(&module.entry_points[0].function.body);
        assert_eq!(
            condition_value(&module, condition, 0.0) != 0.0,
            expected,
            "{operator}"
        );
    }
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
        let (condition, accept, reject) = branch(body);
        for lhs in [0.0, 1.0] {
            let selected = if condition_value(&module, condition, lhs) != 0.0 {
                accept
            } else {
                reject
            };
            let evaluates_rhs = if operator == "&&" {
                lhs != 0.0
            } else {
                lhs == 0.0
            };
            assert_eq!(
                all_buffer_loads(&module, selected, "rhs"),
                usize::from(evaluates_rhs)
            );
        }
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
        "__global__ void bad(int *out) { if (true ? 1 : 0) out[0] = 1; }",
        "__global__ void bad(int *out) { if (true << false) out[0] = 1; }",
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
    let output =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let reparsed = naga::front::wgsl::parse_str(&output).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&reparsed)
    .unwrap();
}

#[cfg(feature = "spv-out")]
#[test]
fn cuda_guarded_kernel_emits_spirv() {
    let (module, info) = validate(GUARDED_ADD);
    let words = naga::back::spv::write_vec(&module, &info, &Default::default(), None).unwrap();
    rspirv::dr::load_words(words).unwrap();
}

#[cfg(feature = "glsl-out")]
#[test]
fn cuda_guarded_kernel_emits_glsl() {
    let (module, info) = validate(GUARDED_ADD);
    let mut output = String::new();
    let pipeline = naga::back::glsl::PipelineOptions {
        shader_stage: naga::ShaderStage::Compute,
        entry_point: "add".into(),
        multiview: None,
    };
    naga::back::glsl::Writer::new(
        &mut output,
        &module,
        &info,
        &Default::default(),
        &pipeline,
        Default::default(),
    )
    .unwrap()
    .write()
    .unwrap();
}

#[cfg(feature = "hlsl-out")]
#[test]
fn cuda_guarded_kernel_emits_hlsl() {
    let (module, info) = validate(GUARDED_ADD);
    let mut output = String::new();
    let reflection =
        naga::back::hlsl::Writer::new(&mut output, &Default::default(), &Default::default())
            .write(&module, &info, None)
            .unwrap();
    for name in reflection.entry_point_names {
        name.unwrap();
    }
}

#[cfg(feature = "msl-out")]
#[test]
fn cuda_guarded_kernel_emits_msl() {
    let (module, info) = validate(GUARDED_ADD);
    let (_, reflection) =
        naga::back::msl::write_string(&module, &info, &Default::default(), &Default::default())
            .unwrap();
    for name in reflection.entry_point_names {
        name.unwrap();
    }
}
