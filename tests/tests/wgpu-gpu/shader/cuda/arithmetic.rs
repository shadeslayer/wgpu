use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![case(
        "cuda::adversarial::cuda_arithmetic_preserves_precedence_and_left_associativity/1",
        r#"__global__ void arithmetic(int *out) {
out[0] = 20 - 3 - 2;
out[1] = 24 / 3 / 2;
out[2] = 2 + 3 * 4;
out[3] = -(2 + 3) * 4;
out[4] = 20 - (3 - 2);
}"#,
        &[0, 0, 0, 0, 0],
        &[15, 4, 14, -20, 19],
    )]
}
