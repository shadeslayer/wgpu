use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::specifiers::cuda_static_kernels_accept_each_header_order/1",
            r#"static __global__ void kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::specifiers::cuda_static_kernels_accept_each_header_order/2",
            r#"static void __global__ kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::specifiers::cuda_static_kernels_accept_each_header_order/3",
            r#"__global__ static void kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::specifiers::cuda_static_kernels_accept_each_header_order/4",
            r#"__global__ void static kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::specifiers::cuda_static_kernels_accept_each_header_order/5",
            r#"void static __global__ kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::specifiers::cuda_static_kernels_accept_each_header_order/6",
            r#"void __global__ static kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
    ]
}
