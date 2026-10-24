#![cfg(all(feature = "glsl-in", feature = "wgsl-in", feature = "wgsl-out"))]

use naga::valid::{Capabilities, ValidationFlags, Validator};

const FIXTURE: &str = include_str!("../in/glsl/switch-fallthrough.comp");

fn has_nonempty_fallthrough(module: &naga::Module) -> bool {
    let mut contains_fallthrough = |statement: &naga::Statement| matches!(statement, naga::Statement::Switch { cases, .. } if cases.iter().any(|case| case.fall_through && !case.body.is_empty()));
    module
        .functions
        .iter()
        .any(|(_, function)| naga::proc::any_statement(&function.body, &mut contains_fallthrough))
        || module
            .entry_points
            .iter()
            .any(|entry| naga::proc::any_statement(&entry.function.body, &mut contains_fallthrough))
}

fn roundtrip(source: &str, expect_fallthrough: bool) {
    let module = naga::front::glsl::Frontend::default()
        .parse(
            &naga::front::glsl::Options::from(naga::ShaderStage::Compute),
            source,
        )
        .unwrap_or_else(|error| panic!("{error:?}\n{source}"));
    assert_eq!(has_nonempty_fallthrough(&module), expect_fallthrough);
    let info = Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&module)
        .unwrap();
    let output = naga::back::wgsl::write_string(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    )
    .unwrap();
    let reparsed = naga::front::wgsl::parse_str(&output)
        .unwrap_or_else(|error| panic!("{}\n{output}", error.emit_to_string(&output)));
    Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&reparsed)
        .unwrap_or_else(|error| panic!("{error:?}\n{output}"));
    assert!(!has_nonempty_fallthrough(&reparsed));
}

fn shader(body: &str) -> String {
    format!("#version 450\nlayout(local_size_x = 1) in;\nlayout(set = 0, binding = 0, std430) buffer Output {{ int values[]; }};\nvoid main() {{ {body} }}")
}

#[test]
fn wgsl_switch_fallthrough_roundtrips_the_common_glsl_snapshot() {
    roundtrip(FIXTURE, true);
}

#[test]
fn wgsl_switch_fallthrough_keeps_default_position_and_grouped_labels() {
    for body in [
        "int selector = values[0]; switch (selector) { case -1: values[1] = 1; default: values[2] = 2; case 3: values[3] = 3; break; }",
        "uint selector = uint(values[0]); switch (selector) { case 0u: case 1u: values[1] = 3; case 2u: values[2] = 5; break; default: values[3] = 7; break; }",
        "switch (values[0]) { default: values[1] = 1; case 2: case 3: values[2] = 2; break; }",
    ] {
        roundtrip(&shader(body), true);
    }
}

#[test]
fn wgsl_switch_fallthrough_keeps_expression_snapshots_and_call_results_scoped() {
    let source = "#version 450
        layout(local_size_x = 1) in;
        layout(set = 0, binding = 0, std430) buffer Output { int values[]; };
        int update(int value) { values[4] += 1; return value + values[4]; }
        void main() {
            int selector = values[0];
            switch (selector) {
                case 0: { int named_value = update(values[1]); values[2] = named_value + named_value; }
                case 1: { int named_value = update(values[2]); values[3] = named_value * named_value; }
                default: { int named_value = update(values[3]); values[5] = named_value + named_value; break; }
            }
            values[6] = update(values[5]);
        }";
    roundtrip(source, true);
}

#[test]
fn wgsl_switch_fallthrough_preserves_nested_switch_break_targets() {
    roundtrip(&shader("switch (values[0]) { case 0: values[1] += 1; switch (values[2]) { case 0: values[1] += 10; default: values[1] += 20; break; } case 1: values[1] += 100; break; default: values[1] = 900; break; }"), true);
}

#[test]
fn wgsl_switch_fallthrough_preserves_loop_continue_and_conditional_break() {
    roundtrip(&shader("for (int iteration = 0; iteration < 4; ++iteration) { switch (values[0]) { case 0: if (iteration == 1) continue; values[1] += iteration + 1; case 1: if (iteration == 2) break; values[1] += 10; break; default: values[1] += 100; continue; } values[1] += 1000; }"), true);
}

#[test]
fn wgsl_switch_fallthrough_handles_terminal_return_branches_without_invalid_tails() {
    for first_case in [
        "return 3;",
        "{ return 3; }",
        "if (flag != 0) { return 3; } else { return 4; }",
        "{ if (flag != 0) { return 3; } else { return 4; } }",
        "if (flag != 0) return 3; values[1] += 1;",
    ] {
        let source = format!("#version 450\nlayout(local_size_x = 1) in;\nlayout(set = 0, binding = 0, std430) buffer Output {{ int values[]; }};\nint route(int selector, int flag) {{ switch (selector) {{ case 0: {first_case} case 1: values[2] += 5; return 9; default: return 10; }} return 11; }}\nvoid main() {{ values[3] = route(values[0], values[4]); }}");
        roundtrip(&source, true);
    }
}

#[test]
fn wgsl_switch_fallthrough_handles_terminal_loop_branches_without_invalid_tails() {
    for first_case in [
        "continue;",
        "{ continue; }",
        "if (iteration == 1) { continue; } else { continue; }",
        "{ if (iteration == 1) { continue; } else { continue; } }",
    ] {
        roundtrip(&shader(&format!("for (int iteration = 0; iteration < 3; ++iteration) {{ switch (values[0]) {{ case 0: {first_case} case 1: values[1] += 7; break; default: values[2] += 9; break; }} values[3] += 11; }}")), true);
    }
}

#[test]
fn wgsl_switch_fallthrough_preserves_previously_supported_grouping_and_empty_switches() {
    for body in [
        "switch (values[0]) {}",
        "switch (values[0]) { case 0: case 1: values[1] = 7; break; default: values[1] = 9; break; }",
        "switch (values[0]) { default: case 0: case 1: break; }",
    ] {
        roundtrip(&shader(body), false);
    }
}
