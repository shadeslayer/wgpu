#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const UPDATES: &str = include_str!("../../in/cuda/updates.cu");

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
    super::helpers::wgsl_roundtrip(&module, &info, naga::back::wgsl::WriterFlags::empty());
}
