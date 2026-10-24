use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/switch.cu");

fn kernel(body: &str) -> String {
    format!("__global__ void kernel(int *out, int selector) {{ {body} }}")
}

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty() || range.start == source.len(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

fn case_shapes(module: &naga::Module) -> Vec<(Vec<naga::SwitchValue>, Vec<bool>)> {
    let mut shapes = Vec::new();
    naga::proc::any_statement(&module.entry_points[0].function.body, &mut |statement| {
        if let naga::Statement::Switch { cases, .. } = statement {
            shapes.push((
                cases.iter().map(|case| case.value).collect(),
                cases.iter().map(|case| case.fall_through).collect(),
            ));
        }
        false
    });
    shapes
}

#[test]
fn cuda_switch_selectors_promote_boolean_and_preserve_signedness() {
    use naga::SwitchValue as CaseValue;
    for (selector, labels, expected) in [
        (
            "true",
            "case false: case true: case 2:",
            vec![
                CaseValue::I32(0),
                CaseValue::I32(1),
                CaseValue::I32(2),
                CaseValue::Default,
            ],
        ),
        (
            "selector",
            "case -1: case 0: case 2147483647u:",
            vec![
                CaseValue::I32(-1),
                CaseValue::I32(0),
                CaseValue::I32(i32::MAX),
                CaseValue::Default,
            ],
        ),
        (
            "unsigned(selector)",
            "case 0: case 2147483648u: case 0xffffffffu:",
            vec![
                CaseValue::U32(0),
                CaseValue::U32(0x80000000),
                CaseValue::U32(u32::MAX),
                CaseValue::Default,
            ],
        ),
    ] {
        let (module, _) = validate(&kernel(&format!(
            "switch ({selector}) {{ {labels} out[0] = 1; break; }}"
        )));
        let shapes = case_shapes(&module);
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].0, expected, "{selector}");
        assert_eq!(shapes[0].1.last(), Some(&false));
    }
}

#[test]
fn cuda_switch_evaluates_selector_calls_once_and_accepts_comma_expressions() {
    let (module, _) = validate("__device__ int select(int value) { return value; } __global__ void kernel(int *out, int selector) { int counter = 0; switch (counter++, select(selector)) { case 0: out[0] = counter; case 1: out[1] = counter; break; } out[2] = counter; }");
    let mut calls = 0;
    naga::proc::any_statement(&module.entry_points[0].function.body, &mut |statement| {
        if matches!(statement, naga::Statement::Call { .. }) {
            calls += 1;
        }
        false
    });
    assert_eq!(calls, 1);
}

#[test]
fn cuda_switch_case_constants_use_aliases_constexpr_and_checked_conversions() {
    use naga::SwitchValue as CaseValue;
    let (module, _) = validate("using Count = unsigned; constexpr int First = -2; constexpr Count Last = 0xffffffffu; __global__ void kernel(int *out, int selector) { const int Local = 3; constexpr float Exact = 2.0f; switch (selector) { case First: case int(Exact): case Local: case unsigned(4): out[0] = 1; break; } switch (Count(selector)) { case false: case 2147483648u: case Last: out[1] = 2; break; } }");
    let shapes = case_shapes(&module);
    assert_eq!(shapes.len(), 2);
    assert_eq!(
        shapes[0].0,
        [
            CaseValue::I32(-2),
            CaseValue::I32(2),
            CaseValue::I32(3),
            CaseValue::I32(4),
            CaseValue::Default
        ]
    );
    assert_eq!(
        shapes[1].0,
        [
            CaseValue::U32(0),
            CaseValue::U32(0x80000000),
            CaseValue::U32(u32::MAX),
            CaseValue::Default
        ]
    );
    let (module, _) = validate(&kernel("switch (selector) { case (selector, 2): out[0] = 1; break; } switch (unsigned(selector)) { case static_cast<unsigned>(-1): out[1] = 2; break; }"));
    let shapes = case_shapes(&module);
    assert_eq!(shapes.len(), 2);
    assert_eq!(shapes[0].0, [CaseValue::I32(2), CaseValue::Default]);
    assert_eq!(shapes[1].0, [CaseValue::U32(u32::MAX), CaseValue::Default]);
    let (module, _) = validate(&kernel("switch (selector) { case true ? 2 : (1 / 0): out[0] = 1; break; } switch (selector) { case (0xffffffffu + 1u): out[1] = 2; break; }"));
    let shapes = case_shapes(&module);
    assert_eq!(shapes.len(), 2);
    assert_eq!(shapes[0].0, [CaseValue::I32(2), CaseValue::Default]);
    assert_eq!(shapes[1].0, [CaseValue::I32(0), CaseValue::Default]);
}

#[test]
fn cuda_switch_rejects_runtime_float_vector_and_narrowing_case_labels() {
    for body in [
        "switch (selector) { case selector: break; }",
        "switch (selector) { case threadIdx.x: break; }",
        "switch (selector) { case blockDim.x: break; }",
        "switch (selector) { case 1.0f: break; }",
        "switch (selector) { case make_int2(1, 2): break; }",
        "switch (selector) { case 2147483648u: break; }",
        "switch (selector) { case 0xffffffff: break; }",
        "switch (unsigned(selector)) { case -1: break; }",
        "switch (selector) { case 2147483647 + 1: break; }",
        "switch (selector) { case 1 / 0: break; }",
        "switch (selector) { case (selector++, 1): break; }",
    ] {
        rejected(&kernel(body));
    }
    rejected("__device__ int ordinary() { return 1; } __global__ void kernel() { switch (1) { case ordinary(): break; } }");
    for selector in ["1.0f", "make_int2(1, 2)"] {
        rejected(&kernel(&format!(
            "switch ({selector}) {{ default: break; }}"
        )));
    }
    rejected("__device__ void ignored() {} __global__ void kernel() { switch (ignored()) { default: break; } }");
}

#[test]
fn cuda_switch_detects_duplicate_values_after_integral_conversion() {
    for labels in [
        "case 1: break; case 1u: break;",
        "case true: break; case 1: break;",
        "case false: break; case 0u: break;",
        "case 1 + 1: break; case 2: break;",
        "case (0xffffffffu + 1u): break; case 0: break;",
        "default: break; default: break;",
    ] {
        rejected(&kernel(&format!("switch (selector) {{ {labels} }}")));
    }
    rejected(&kernel(
        "switch (unsigned(selector)) { case 1: break; case 1u: break; }",
    ));
    validate(&kernel("switch (selector) { case 1: break; default: break; } switch (selector) { case 1u: break; default: break; }"));
}

#[test]
fn cuda_switch_preserves_default_position_grouping_and_nonempty_fallthrough() {
    for body in [
        "switch (selector) { default: out[0] += 1; case -1: out[0] += 2; case 2: out[0] += 4; break; }",
        "switch (selector) { case 0: out[0] += 1; default: out[0] += 2; case 2: out[0] += 4; }",
        "switch (selector) { case 0: case 1: out[0] += 1; case 2: out[0] += 2; default: out[0] += 4; }",
        "switch (selector) { case 0: out[0] = 1; }",
        "switch (selector) {}",
    ] {
        let (module, _) = validate(&kernel(body));
        for (values, fallthrough) in case_shapes(&module) {
            assert_eq!(values.iter().filter(|value| matches!(value, naga::SwitchValue::Default)).count(), 1);
            assert_eq!(fallthrough.last(), Some(&false));
        }
    }
    let (module, _) = validate(&kernel(
        "switch (selector) { default: out[0] += 1; case 3: out[0] += 2; break; }",
    ));
    let shapes = case_shapes(&module);
    assert_eq!(
        shapes[0].0,
        [naga::SwitchValue::Default, naga::SwitchValue::I32(3)]
    );
    assert_eq!(shapes[0].1, [true, false]);
    validate(
        "__global__ void bad(unsigned int *out) { switch (threadIdx.x) { case 0: out[0] = 1u; } }",
    );
}

#[test]
fn cuda_switch_accepts_direct_noncompound_bodies_and_label_chains() {
    for body in [
        "switch (selector);",
        "switch (selector) out[0] = 99;",
        "switch (selector) int vacuous;",
        "switch (selector) int initialized = 1;",
        "switch (selector) case 0: out[0] = 1;",
        "switch (selector) case 0: case 1: out[0] = 1;",
        "switch (selector) default: out[0] = 2;",
        "switch (selector) case 0: int initialized = 1;",
    ] {
        validate(&kernel(body));
    }
    validate("__device__ int route(int selector) { switch (selector) default: return 7; } __global__ void kernel(int *out) { out[0] = route(1); }");
    rejected("__device__ int route(int selector) { switch (selector) case 0: return 7; } __global__ void kernel() {}");
}

#[test]
fn cuda_switch_has_one_lexical_scope_for_vacuous_declarations_and_aliases() {
    validate(&kernel("switch (selector) { int prelude; using Count = unsigned; case 0: int value; int3 point; int matrix[2][2]; value = 1; point = make_int3(2, 3, 4); matrix[1][1] = 5; prelude = 6; case 1: Count next; next = 7u; value = 8; point.x = 9; matrix[0][0] = 10; prelude = 11; out[0] = value + point.x + matrix[0][0] + prelude + int(next); break; }"));
    validate(&kernel("switch (selector) { case 0: using Count = unsigned; break; case 1: Count value; value = 1u; out[0] = int(value); break; }"));
    for body in [
        "switch (selector) { case 0: int value; break; case 1: int value; break; }",
        "switch (selector) { case 0: using Count = int; break; case 1: using Count = unsigned; break; }",
        "switch (selector) { case 0: int hidden; break; } out[0] = hidden;",
        "switch (selector) { using Hidden = int; default: break; } Hidden value;",
        "switch (selector) int hidden; out[0] = hidden;",
    ] {
        rejected(&kernel(body));
    }
}

#[test]
fn cuda_switch_checks_but_does_not_execute_prelude_or_unreachable_statements() {
    validate(&kernel("switch (selector) { out[0] = 91; int bare; case 0: bare = 3; out[1] = bare; break; out[2] = 92; case 1: bare = 4; out[3] = bare; break; }"));
    validate(&kernel("switch (selector) { case 0: break; using Count = int; case 1: Count value; value = 4; out[0] = value; }"));
    for body in [
        "switch (selector) { missing = 1; case 0: break; }",
        "switch (selector) { case 0: break; missing = 1; }",
        "return; switch (selector) { case missing: break; }",
        "switch (selector) { case 0: break; int initialized = 1; case 1: break; }",
    ] {
        rejected(&kernel(body));
    }
}

#[test]
fn cuda_switch_rejects_case_entry_past_any_active_nonvacuous_initializer() {
    for declaration in [
        "int value = 1;",
        "int value{};",
        "float value = 1.0f;",
        "const int value = 1;",
        "constexpr int value = 1;",
        "int first, second = 2;",
        "int first = 1, second;",
        "int2 point{};",
        "const int3 point{1, 2, 3};",
        "int values[2]{};",
        "int values[2]{1};",
        "int values[]{1, 2};",
        "int matrix[2][2]{};",
    ] {
        rejected(&kernel(&format!(
            "switch (selector) {{ {declaration} case 0: break; }}"
        )));
        rejected(&kernel(&format!(
            "switch (selector) {{ case 0: {declaration} case 1: break; }}"
        )));
        rejected(&kernel(&format!(
            "switch (selector) {{ case 0: {declaration} default: break; }}"
        )));
    }
}

#[test]
fn cuda_switch_allows_initializers_outside_or_inside_closed_scopes() {
    validate(&kernel("int outer = 7; switch (selector) { case 0: { int value = outer; out[0] = value; } case 1: { int2 point{}; out[1] = point.x; } default: break; }"));
    validate(&kernel("switch (selector) { case 0: for (int iteration = 0; iteration < 2; ++iteration) { int inside = iteration; } case 1: out[0] = 1; break; }"));
    validate(&kernel("switch (selector) { case 0: if (selector) int scoped = 1; case 1: while (false) { int scoped = 2; } default: break; }"));
    validate(&kernel(
        "switch (selector) { case 0: int initialized = 1; out[0] = initialized; }",
    ));
}

#[test]
fn cuda_switch_keeps_nested_switch_loop_break_and_continue_ownership() {
    validate(&kernel("for (int iteration = 0; iteration < 4; ++iteration) { switch (selector) { case 0: if (iteration == 1) continue; out[0] += 1; case 1: if (iteration == 2) break; out[0] += 10; break; default: continue; } out[0] += 100; }"));
    validate(&kernel("switch (selector) { case 0: for (int iteration = 0; iteration < 3; ++iteration) { if (iteration == 1) continue; if (iteration == 2) break; out[0] += iteration; } case 1: switch (selector + 1) { case 2: out[0] += 3; break; default: out[0] += 4; } out[0] += 5; break; default: break; }"));
    rejected(&kernel("switch (selector) { default: continue; }"));
    rejected(&kernel("switch (selector) { default: break; } break;"));
    rejected(&kernel("case 0: out[0] = 1;"));
    rejected(&kernel("default: out[0] = 1;"));
}

#[test]
fn cuda_switch_return_analysis_covers_every_effective_case_entry() {
    for definition in [
        "__device__ int route(int selector) { switch (selector) { case 0: return 1; default: return 2; } }",
        "__device__ int route(int selector) { switch (selector) { default: case 0: case 1: return 3; } }",
        "__device__ int route(int selector) { int result = 0; switch (selector) { default: result = 2; case 1: return result + 3; } }",
        "__device__ int route(int selector) { switch (selector) { case 0: if (selector) return 1; else return 2; default: return 3; } }",
        "__device__ int route(int selector) { switch (selector) { case 0: switch (selector) { default: break; } return 1; default: return 2; } }",
        "__device__ int route(int selector) { switch (selector) default: return 5; }",
    ] {
        validate(&format!("{definition} __global__ void kernel(int *out) {{ out[0] = route(0); }}"));
    }
    for definition in [
        "__device__ int route(int selector) { switch (selector) { case 0: return 1; } }",
        "__device__ int route(int selector) { switch (selector) { case 0: break; default: return 2; } }",
        "__device__ int route(int selector) { switch (selector) { case 0: if (selector) break; return 1; default: return 2; } }",
        "__device__ int route(int selector) { switch (selector) { case 0: while (selector) { return 1; } break; default: return 2; } }",
    ] {
        rejected(&format!("{definition} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_switch_does_not_discard_reachable_statements_after_case_breaks() {
    validate("__device__ int route(int selector) { switch (selector) { case 0: break; default: return 2; } return 3; } __global__ void kernel(int *out, int selector) { switch (selector) { default: break; } out[0] = route(selector); }");
    rejected(&kernel("while (selector) { switch (selector) { case 0: continue; default: continue; } missing = 1; }"));
}

#[test]
fn cuda_switch_barriers_retain_conservative_conditional_and_return_rules() {
    for body in [
        "switch (selector) { case 0: __syncthreads(); break; }",
        "switch (0) { default: { __syncthreads(); } }",
        "switch (selector) { case 0: for (int iteration = 0; iteration < 1; ++iteration) { __syncthreads(); } break; }",
        "switch (selector) { case 0: switch (selector) { default: __syncthreads(); } break; }",
        "switch (selector) { case 0: return; default: break; } __syncthreads();",
        "switch (selector) { default: switch (selector) { case 0: return; default: break; } } __syncthreads();",
    ] {
        rejected(&kernel(body));
    }
    validate(&kernel("__syncthreads(); switch (selector) { case 0: out[0] = 1; break; default: out[0] = 2; } __syncthreads();"));
    validate(&kernel(
        "switch (selector) { default: return; } __syncthreads();",
    ));
    rejected("__device__ void helper(int selector) { switch (selector) { default: __syncthreads(); } } __global__ void kernel() {}");
}

#[test]
fn cuda_switch_rejects_advanced_nested_entry_labels_and_init_statements() {
    for body in [
        "switch (selector) { if (true) { case 0: out[0] = 1; } }",
        "switch (selector) { while (false) { case 0: out[0] = 1; } }",
        "switch (selector) { do { case 0: out[0] = 1; } while (false); }",
        "switch (selector) { for (;;) { default: break; } }",
        "switch (selector) if (true) case 0: out[0] = 1;",
        "switch (int value = selector; value) { default: break; }",
        "switch (int value = selector) { default: break; }",
    ] {
        rejected(&kernel(body));
    }
}

#[test]
fn cuda_switch_errors_keep_original_utf8_spans_and_check_dead_source() {
    let source = "/* café 漢 */ __global__ void kernel(int selector) { switch (selector) { case missing: break; } }";
    assert_eq!(
        &source[rejected(source).span.to_range().unwrap()],
        "missing"
    );
    for body in [
        "switch () {}",
        "switch selector {}",
        "switch (selector { case 0: break; }",
        "switch (selector) { case: break; }",
        "switch (selector) { case 0; break; }",
        "switch (selector) { default break; }",
        "switch (selector) { case 0: break;",
        "return; switch (selector) { case 0: missing = 1; }",
        "return; switch (selector) { case 0: break; case 0: break; }",
    ] {
        rejected(&kernel(body));
    }
}

#[test]
fn cuda_switch_nesting_uses_existing_frontend_depth_guards() {
    let prefix = "switch (selector) { case 0:";
    validate(&kernel(
        &(prefix.repeat(8) + "out[0] = 1;" + &"}".repeat(8)),
    ));
    rejected(&kernel(
        &(prefix.repeat(256) + "out[0] = 1;" + &"}".repeat(256)),
    ));
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_switch_roundtrips_nonempty_fallthrough_through_common_wgsl_helpers() {
    let (module, info) = validate(FIXTURE);
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}

#[cfg(spv_out)]
#[cfg_attr(miri, ignore)]
#[test]
fn cuda_switch_fixture_emits_valid_spirv_binary() {
    let (module, info) = validate(FIXTURE);
    let options = naga::back::spv::Options {
        lang_version: (1, 1),
        ..Default::default()
    };
    let pipeline_options = naga::back::spv::PipelineOptions {
        entry_point: "switches".to_string(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let words =
        naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline_options)).unwrap();
    crate::helpers::validate_spirv(&words);
}
