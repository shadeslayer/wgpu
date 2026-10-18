use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::declarations::cuda_unambiguous_constructor_statements_keep_their_expression_meaning/1",
            r#"__global__ void kernel(int *out) {
        int value = 3;
        int(value) + 1;
        int((value)) + 1;
        int(value = 1);
        (int(value));
        int(value + 1);
        static_cast<int>(value);
        out[0] = value;
    }"#,
            &[0],
            &[1],
        ),
    ]
}
