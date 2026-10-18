#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

fn validate(source: &str) -> naga::Module {
    super::helpers::validate_with_options(
        source,
        &Options {
            workgroup_size: [64, 1, 1],
        },
    )
    .0
}

#[test]
fn cuda_syncthreads_synchronizes_shared_and_global_memory() {
    let source = "__global__ void sync(int *out) { out[threadIdx.x] = 1; { __syncthreads(); } __syncthreads(); out[threadIdx.x] += 1; }";
    let module = validate(source);
    let mut barriers = Vec::new();
    naga::proc::any_statement(&module.entry_points[0].function.body, &mut |statement| {
        if let naga::Statement::ControlBarrier(flags) = *statement {
            barriers.push(flags);
        }
        false
    });
    assert_eq!(
        barriers,
        [naga::Barrier::WORK_GROUP | naga::Barrier::STORAGE; 2]
    );
}

#[test]
fn cuda_syncthreads_allows_reconverged_control_flow_without_returns() {
    validate("__global__ void sync(int *out) { if (threadIdx.x < 32u) out[threadIdx.x] = 1; else out[threadIdx.x] = 2; for (int j = 0; j < 3; ++j) { if (threadIdx.x == 0u) break; } __syncthreads(); if (threadIdx.x > 3u) return; out[threadIdx.x] += 1; }");
}

#[test]
fn cuda_syncthreads_empty_kernel_and_unreachable_call_validate() {
    validate("__global__ void sync() { __syncthreads /* gap */ ( /* no args */ ); return; }");
    validate("__global__ void sync() { return; __syncthreads(); }");
}

#[test]
fn cuda_syncthreads_rejects_conditional_and_loop_placement() {
    for statement in [
        "if (threadIdx.x == 0u) __syncthreads();",
        "if (true) { __syncthreads(); }",
        "if (false) {} else { { __syncthreads(); } }",
        "for (int j = 0; j < 3; ++j) __syncthreads();",
        "while (false) { __syncthreads(); }",
        "do { __syncthreads(); } while (false);",
    ] {
        assert_placement_error(statement);
    }
}

#[test]
fn cuda_syncthreads_rejects_prior_potential_returns() {
    for statement in [
        "if (threadIdx.x == 0u) return; __syncthreads();",
        "{ if (threadIdx.x < 8u) { return; } } __syncthreads();",
        "while (threadIdx.x > 0u) { if (threadIdx.x == 1u) return; break; } __syncthreads();",
        "do { if (threadIdx.x == 0u) return; } while (false); __syncthreads();",
        "__syncthreads(); if (threadIdx.x == 0u) return; __syncthreads();",
    ] {
        assert_placement_error(statement);
    }
}

fn assert_placement_error(statement: &str) {
    let source = format!("__global__ void sync() {{ {statement} }}");
    let error = parse_str(&source, &Options::default()).unwrap_err();
    assert!(error.message.contains("straight-line"), "{source}: {error}");
    assert_eq!(&source[error.span.to_range().unwrap()], "__syncthreads();");
}

#[test]
fn cuda_syncthreads_requires_a_zero_argument_statement() {
    for statement in [
        "__syncthreads(1);",
        "__syncthreads;",
        "__syncthreads()",
        "int x = __syncthreads();",
        "bool x = true && __syncthreads();",
        "(__syncthreads());",
        "__syncthreads_count(1);",
        "__threadfence_block();",
    ] {
        let source = format!("__global__ void bad() {{ {statement} }}");
        assert!(parse_str(&source, &Options::default()).is_err(), "{source}");
    }
}

#[test]
fn cuda_syncthreads_cannot_be_shadowed() {
    for source in [
        "__global__ void __syncthreads() {}",
        "__global__ void bad(int __syncthreads) {}",
        "__global__ void bad(int *__syncthreads) {}",
        "__global__ void bad() { int __syncthreads = 1; }",
    ] {
        assert!(parse_str(source, &Options::default()).is_err(), "{source}");
    }
}
