#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const UPDATES: &str = "__global__ void updates(int *out, int value) {\n\
    int previous = value++;\n\
    out[threadIdx.x] += previous;\n\
    out[threadIdx.x] = ++value;\n\
}";

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
