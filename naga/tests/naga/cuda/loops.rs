#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

const LOOPS: &str = r#"
__global__ void loops(int *out, int n) {
    int sum = 0;
    for (int i = 0; i < n; i = i + 1) {
        if (i == 2) continue;
        if (i == 7) break;
        sum = sum + i;
    }
    int i = 0;
    while (i < 3) {
        i = i + 1;
        if (i == 2) continue;
        sum = sum + 10;
    }
    do {
        sum = sum + 100;
        if (sum > 200) break;
    } while (sum < 150);
    out[threadIdx.x] = sum;
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

fn loops<'a>(block: &'a naga::Block, result: &mut Vec<&'a naga::Statement>) {
    for statement in block {
        match statement {
            naga::Statement::Loop { body, .. } => {
                result.push(statement);
                loops(body, result);
            }
            naga::Statement::Block(block) => loops(block, result),
            naga::Statement::If { accept, reject, .. } => {
                loops(accept, result);
                loops(reject, result);
            }
            _ => {}
        }
    }
}

#[test]
fn cuda_for_continue_runs_the_step_and_do_continue_checks_the_condition() {
    let (module, _) = validate(LOOPS);
    let mut statements = Vec::new();
    loops(&module.entry_points[0].function.body, &mut statements);
    assert_eq!(statements.len(), 3);
    let naga::Statement::Loop {
        continuing,
        break_if,
        ..
    } = statements[0]
    else {
        unreachable!();
    };
    assert!(continuing
        .iter()
        .any(|statement| matches!(statement, naga::Statement::Store { .. })));
    assert!(break_if.is_none());
    let naga::Statement::Loop {
        continuing,
        break_if,
        ..
    } = statements[1]
    else {
        unreachable!();
    };
    assert!(continuing.is_empty());
    assert!(break_if.is_none());
    let naga::Statement::Loop {
        continuing,
        break_if,
        ..
    } = statements[2]
    else {
        unreachable!();
    };
    assert!(!continuing.is_empty());
    assert!(break_if.is_some());
}

#[test]
fn cuda_empty_loop_clauses_and_single_statement_bodies_validate() {
    validate("__global__ void empty(int *out) { for (;;) { break; } for (;out[0];) break; while (out[0]) break; do continue; while (false); }");
    validate(
        "__global__ void simple(int *out) { int i = 0; for (i = 1; i < 4; i = i + 1) out[i] = i; }",
    );
}

#[test]
fn cuda_nested_loops_preserve_jump_targets() {
    validate("__global__ void nested(int *out) { for (int i = 0; i < 4; i = i + 1) { for (int j = 0; j < 3; j = j + 1) { if (j == 1) continue; if (i == 2) break; out[i * 3 + j] = j; } if (i == 3) return; } }");
}

#[test]
fn cuda_loop_scopes_restore_outer_variables() {
    validate("__global__ void scopes(int *out) { int i = 9; for (int i = 0; i < 2; i = i + 1) { { int i = 4; out[0] = i; } } out[1] = i; while (false) int hidden = 1; do { int local = 3; out[2] = local; } while (false); }");
    for source in [
        "__global__ void bad(int *out) { for (int i = 0; i < 2; i = i + 1) {} out[0] = i; }",
        "__global__ void bad() { for (int i = 0; i < 2; i = i + 1) { int i = 3; } }",
        "__global__ void bad() { for (int i = 0; i < 2; i = i + 1) int i = 3; }",
        "__global__ void bad(int *out) { while (false) int hidden = 1; out[0] = hidden; }",
        "__global__ void bad() { do { int local = 1; } while (local); }",
        "__global__ void bad() { for (int i = 0; i < 2; i = local) { int local = 1; } }",
    ] {
        assert!(parse_str(source, &Options::default()).is_err(), "{source}");
    }
}

#[test]
fn cuda_loop_jumps_require_an_enclosing_loop() {
    for statement in [
        "break;",
        "continue;",
        "if (true) break;",
        "return; continue;",
    ] {
        let source = format!("__global__ void bad() {{ {statement} }}");
        let error = parse_str(&source, &Options::default()).unwrap_err();
        assert!(error.message.contains("loop"), "{error}");
        assert!(error.span.to_range().is_some());
    }
}

#[test]
fn cuda_unreachable_loop_statements_are_checked() {
    validate("__global__ void done(int *out) { while (true) { break; out[0] = 1; } do { continue; out[0] = 2; } while (false); }");
    assert!(parse_str(
        "__global__ void bad() { for (;;) { break; missing = 1; } }",
        &Options::default()
    )
    .is_err());
}

#[test]
fn cuda_rejects_malformed_and_excessively_nested_loops() {
    for statement in [
        "for (int i = 0; i < 2; i = i + 1 {}",
        "while () {}",
        "do {} while (false)",
        "do {} (false);",
        "for (int i : 3) {}",
    ] {
        let source = format!("__global__ void bad() {{ {statement} }}");
        assert!(parse_str(&source, &Options::default()).is_err(), "{source}");
    }
    for prefix in ["for (;;) {", "while (true) {", "do {"] {
        let suffix = if prefix == "do {" {
            "} while (false);"
        } else {
            "}"
        };
        let source = format!(
            "__global__ void deep() {{ {}break;{} }}",
            prefix.repeat(256),
            suffix.repeat(256)
        );
        assert!(parse_str(&source, &Options::default()).is_err());
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_loops_roundtrip_through_wgsl() {
    let (module, info) = validate(LOOPS);
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
fn cuda_loops_emit_spirv() {
    let (module, info) = validate(LOOPS);
    let words = naga::back::spv::write_vec(&module, &info, &Default::default(), None).unwrap();
    rspirv::dr::load_words(words).unwrap();
}

#[cfg(feature = "glsl-out")]
#[test]
fn cuda_loops_emit_glsl() {
    let (module, info) = validate(LOOPS);
    let mut output = String::new();
    let pipeline = naga::back::glsl::PipelineOptions {
        shader_stage: naga::ShaderStage::Compute,
        entry_point: "loops".into(),
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
fn cuda_loops_emit_hlsl() {
    let (module, info) = validate(LOOPS);
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
fn cuda_loops_emit_msl() {
    let (module, info) = validate(LOOPS);
    let (_, reflection) =
        naga::back::msl::write_string(&module, &info, &Default::default(), &Default::default())
            .unwrap();
    for name in reflection.entry_point_names {
        name.unwrap();
    }
}
