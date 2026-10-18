use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::inline::cuda_inline_cv_void_kernels_remain_void_compute_entry_points/1",
            r#"inline __global__ const void kernel(int *out) { out[0] = 9; }"#,
            &[0],
            &[9],
        ),
        case(
            "cuda::inline::cuda_inline_cv_void_kernels_remain_void_compute_entry_points/2",
            r#"const inline void __global__ kernel(int *out) { out[0] = 9; }"#,
            &[0],
            &[9],
        ),
        case(
            "cuda::inline::cuda_inline_cv_void_kernels_remain_void_compute_entry_points/3",
            r#"void const __global__ inline kernel(int *out) { out[0] = 9; }"#,
            &[0],
            &[9],
        ),
        case(
            "cuda::inline::cuda_inline_cv_void_kernels_remain_void_compute_entry_points/4",
            r#"__global__ void inline const kernel(int *out) { out[0] = 9; }"#,
            &[0],
            &[9],
        ),
        case(
            "cuda::inline::cuda_inline_kernel_specifiers_accept_each_header_order_and_keep_the_body/1",
            r#"inline __global__ void kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::inline::cuda_inline_kernel_specifiers_accept_each_header_order_and_keep_the_body/2",
            r#"inline void __global__ kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::inline::cuda_inline_kernel_specifiers_accept_each_header_order_and_keep_the_body/3",
            r#"__global__ inline void kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::inline::cuda_inline_kernel_specifiers_accept_each_header_order_and_keep_the_body/4",
            r#"__global__ void inline kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::inline::cuda_inline_kernel_specifiers_accept_each_header_order_and_keep_the_body/5",
            r#"void inline __global__ kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::inline::cuda_inline_kernel_specifiers_accept_each_header_order_and_keep_the_body/6",
            r#"void __global__ inline kernel(int *out) { out[0] = 7; }"#,
            &[0],
            &[7],
        ),
    ]
}
