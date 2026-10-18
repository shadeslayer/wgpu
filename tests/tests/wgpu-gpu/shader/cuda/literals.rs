use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::literals::cuda_integer_bases_and_separators_preserve_arithmetic_precedence/1",
            r#"__global__ void kernel(int *out) {
        out[0] = 010 + 0x10 * 0b10 - 1'0;
        out[1] = 0x1e + 2;
        out[2] = 0x1E/* separated */+2;
    }"#,
            &[0, 0, 0],
            &[30, 32, 32],
        ),
        case(
            "cuda::literals::cuda_nondecimal_signedness_controls_arithmetic_and_comparisons/1",
            r#"__global__ void kernel(int *out) {
        out[0] = -1 < 0xffffffff;
        out[1] = -1 < 0x7fffffff;
        out[2] = (037777777777 + 1u) == 0u;
        out[3] = (-0b11111111111111111111111111111111) == 1u;
    }"#,
            &[9, 9, 9, 9],
            &[0, 1, 1, 1],
        ),
        case(
            "cuda::literals::cuda_quoted_hexadecimal_exponent_digits_do_not_absorb_adjacent_operators/1",
            r#"__global__ void kernel(int *out) {
        out[0] = 0x1'e+2;
        out[1] = 0x1'E-2;
    }"#,
            &[0, 0],
            &[32, 28],
        ),
    ]
}
