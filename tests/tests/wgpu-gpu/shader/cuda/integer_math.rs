use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "integer min/max types, nesting, aliases, helper arity and argument snapshots",
            include_str!("../../../../../naga/tests/in/cuda/integer-minmax.cu"),
            &[
                3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            &[
                -7, 3, 3, 9, 1, 1, 1, 1, 0, 1, 2, 1, 112, 2, 7, 4, 8, 3, 4, 1, 1, 1, 1, 3,
            ],
        ),
        case(
            "user bool overloads coexist with exact integer builtins",
            r#"__device__ int min(bool first, bool second) {
    return (first ? 11 : 12) + (second ? 1 : 0);
}
__device__ int max(bool first, bool second) {
    return (first ? 1 : 0) + (second ? 21 : 22);
}
__global__ void helper_math(int *out) {
    out[0] = min(true, false);
    out[1] = min(3, 4);
    out[2] = max(false, true);
    out[3] = max(3, 4);
}"#,
            &[0, 0, 0, 0],
            &[11, 3, 21, 4],
        ),
    ]
}
