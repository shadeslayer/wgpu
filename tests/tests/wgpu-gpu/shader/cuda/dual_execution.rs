use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::dual_execution::cuda_dual_execution_helpers_preserve_kernel_bindings_and_workgroup_size/1",
            r#"__host__ __device__ int unused(int value) { return value; } __global__ void kernel(int *out) { int value = 3; out[0] = ++value * 2; }"#,
            &[0],
            &[8],
        ),
    ]
}
