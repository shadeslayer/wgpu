#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

mod cuda_helpers;
use cuda_helpers::{execute, validate};

const UPDATES: &str = "__global__ void updates(int *out, int value) {\n\
    int previous = value++;\n\
    out[threadIdx.x] += previous;\n\
    out[threadIdx.x] = ++value;\n\
}";

#[test]
fn cuda_prefix_and_postfix_updates_return_new_and_old_values() {
    let source = "__global__ void update(int *out) { int x = 3; int post = x++; int pre = ++x;\n\
        int old = x--; int last = --x; out[0] = post; out[1] = pre; out[2] = old; out[3] = last; out[4] = x; }";
    assert_eq!(execute(source, &[0; 5]), [3, 5, 5, 3, 3]);
}

#[test]
fn cuda_assignment_evaluates_rhs_before_the_lhs_index() {
    let source = "__global__ void update(int *out) { int i = 0; out[i++] = i++; out[2] = i; }";
    assert_eq!(execute(source, &[-999; 3]), [-999, 0, 2]);
}

#[test]
fn cuda_assignment_loads_rhs_before_the_lhs_changes_it() {
    let source = "__global__ void update(int *out) { int i = 0; out[i++] = i; out[2] = i; }";
    assert_eq!(execute(source, &[-999; 3]), [0, -999, 1]);
}

#[test]
fn cuda_compound_assignment_loads_rhs_before_the_lhs_changes_it() {
    let source = "__global__ void update(int *out) { int i = 1; out[i++] += i; out[2] = i; }";
    assert_eq!(execute(source, &[-999, 10, -999]), [-999, 11, 2]);
}

#[test]
fn cuda_compound_assignment_evaluates_its_index_once_after_the_rhs() {
    let source = "__global__ void update(int *out) { int i = 0; out[i++] += ++i; out[2] = i; }";
    assert_eq!(execute(source, &[-999, 10, -999]), [-999, 11, 2]);
}

#[test]
fn cuda_chained_assignments_associate_right_and_return_an_lvalue() {
    let source =
        "__global__ void update(int *out) { int a = 1; int b = 2; int value = ++(a = b = 7);\n\
        (a = b) = 9; out[0] = value; out[1] = a; out[2] = b; }";
    assert_eq!(execute(source, &[0; 3]), [8, 9, 7]);
}

#[test]
fn cuda_assignments_read_postfix_values_before_the_lhs_is_modified() {
    let source = "__global__ void update(int *out) { int i = 4; i = i++; out[0] = i;\n\
        i = 4; i = ++i; out[1] = i; }";
    assert_eq!(execute(source, &[0; 2]), [4, 5]);
}

#[test]
fn cuda_logical_operators_keep_updates_in_the_selected_branch() {
    let source = "__global__ void update(int *out) { int x = 0; bool a = false && ++x;\n\
        bool b = true || ++x; bool c = true && ++x; bool d = false || ++x;\n\
        out[0] = x; out[1] = a; out[2] = b; out[3] = c; out[4] = d; }";
    assert_eq!(execute(source, &[0; 5]), [2, 0, 1, 1, 1]);
}

#[test]
fn cuda_nested_assignment_indices_preserve_rhs_order() {
    let source =
        "__global__ void update(int *out) { int i = 0; out[(i = 2)] = (i = 1); out[3] = i; }";
    assert_eq!(execute(source, &[-999; 4]), [-999, -999, 1, 2]);
}

#[test]
fn cuda_discarded_updates_still_apply_their_side_effects() {
    let source = "__global__ void update(int *out) { int i = 0; i++; ++i; out[0] = i; }";
    assert_eq!(execute(source, &[0]), [2]);
}

#[test]
fn cuda_compound_assignments_convert_arithmetic_back_to_the_lhs_type() {
    let source = "__global__ void update(int *out) { int value = 3; value *= 1.5f;\n\
        value += 2u; value /= 2; value -= 1; out[0] = value; }";
    assert_eq!(execute(source, &[0]), [2]);
}

#[test]
fn cuda_buffer_element_updates_preserve_old_values() {
    let source = "__global__ void update(int *out) { out[0] = 4; int old = out[0]++;\n\
        out[1] = old; out[2] = ++out[0]; }";
    assert_eq!(execute(source, &[0; 3]), [6, 4, 6]);
}

#[test]
fn cuda_updates_reject_const_bool_and_non_lvalues() {
    for source in [
        "__global__ void bad(int *out) { const int i = 0; out[0] = i++; }",
        "__global__ void bad(int *out) { const int i = 0; out[0] = ++i; }",
        "__global__ void bad(const int *out) { int value = out[0]++; }",
        "__global__ void bad(int *out) { bool flag = false; out[0] = flag++; }",
        "__global__ void bad(int *out) { bool flag = false; out[0] = --flag; }",
        "__global__ void bad(int *out) { out[0] = 1++; }",
        "__global__ void bad(int *out) { int i = 1; out[0] = (i + 1)++; }",
        "__global__ void bad(int *out) { int i = 1; out[0] = int(i)++; }",
        "__global__ void bad(int *out) { int i = 1; (i++) = 2; }",
        "__global__ void bad(int *out) { int i = 1; out[0] = ++(i++); }",
        "__global__ void bad(int *out) { int i = 1; (+i) = 2; }",
        "__global__ void bad(int *out) { int i = 1; (i + 1) = 2; }",
        "__global__ void bad(int *out) { out[0] = threadIdx.x++; }",
        "__global__ void bad(int *out) { out++; }",
    ] {
        assert!(parse_str(source, &Options::default()).is_err(), "{source}");
    }
}

#[test]
fn cuda_updates_reject_excessive_assignment_and_prefix_nesting() {
    for expression in [
        format!("{}1", "value = ".repeat(128)),
        format!("{}value", "++".repeat(128)),
    ] {
        let source = format!("__global__ void deep() {{ int value = 0; value = {expression}; }}");
        assert!(parse_str(&source, &Options::default()).is_err());
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_updates_roundtrip_through_wgsl() {
    let (module, info) = validate(UPDATES);
    let source =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let module = naga::front::wgsl::parse_str(&source).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
}

#[cfg(feature = "spv-out")]
#[test]
fn cuda_updates_emit_spirv() {
    let (module, info) = validate(UPDATES);
    let words = naga::back::spv::write_vec(&module, &info, &Default::default(), None).unwrap();
    rspirv::dr::load_words(words).unwrap();
}

#[cfg(feature = "glsl-out")]
#[test]
fn cuda_updates_emit_glsl() {
    let (module, info) = validate(UPDATES);
    let mut output = String::new();
    naga::back::glsl::Writer::new(
        &mut output,
        &module,
        &info,
        &Default::default(),
        &naga::back::glsl::PipelineOptions {
            shader_stage: naga::ShaderStage::Compute,
            entry_point: "updates".into(),
            multiview: None,
        },
        Default::default(),
    )
    .unwrap()
    .write()
    .unwrap();
}

#[cfg(feature = "hlsl-out")]
#[test]
fn cuda_updates_emit_hlsl() {
    let (module, info) = validate(UPDATES);
    let mut output = String::new();
    let options = naga::back::hlsl::Options::default();
    let pipeline = naga::back::hlsl::PipelineOptions::default();
    let reflection = naga::back::hlsl::Writer::new(&mut output, &options, &pipeline)
        .write(&module, &info, None)
        .unwrap();
    for entry in reflection.entry_point_names {
        entry.unwrap();
    }
}

#[cfg(feature = "msl-out")]
#[test]
fn cuda_updates_emit_msl() {
    let (module, info) = validate(UPDATES);
    let (_, reflection) =
        naga::back::msl::write_string(&module, &info, &Default::default(), &Default::default())
            .unwrap();
    for entry in reflection.entry_point_names {
        entry.unwrap();
    }
}
