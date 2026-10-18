use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::float_separators::cuda_decimal_float_separators_preserve_exponent_signs_and_adjacent_operators/1",
            r#"__global__ void kernel(int *out) {
        out[0] = 1'2.5f * 2.0f;
        out[1] = 1.0e+0'2f-2.0f;
        out[2] = 1'0.0e-0'1f+2.0f;
        out[3] = int(1'0.5F);
    }"#,
            &[0, 0, 0, 0],
            &[25, 98, 3, 10],
        ),
    ]
}
