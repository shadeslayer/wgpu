use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::uninitialized::cuda_bare_local_shadowing_keeps_inner_and_outer_assignments_separate/1",
            r#"__global__ void kernel(int *out) {
                int value = 7;
                { int value; value = 3; out[0] = value; }
                out[1] = value;
            }"#,
            &[0, 0],
            &[3, 7],
        ),
        case(
            "cuda::uninitialized::cuda_bare_local_shadowing_keeps_inner_and_outer_assignments_separate/2",
            r#"using Value = int;
            __global__ void kernel(int *out) { Value Value; Value = 9; out[0] = Value; }"#,
            &[0],
            &[9],
        ),
        case(
            "cuda::uninitialized::cuda_declared_locals_can_be_assigned_on_each_branch_before_use/1",
            r#"__global__ void kernel(int *out) {
                int value;
                if (true) { value = 7; } else { value = 11; }
                out[0] = value;
            }"#,
            &[0],
            &[7],
        ),
        case(
            "cuda::uninitialized::cuda_declared_locals_can_be_assigned_on_each_branch_before_use/2",
            r#"__global__ void kernel(int *out) {
                int value;
                if (false) { value = 7; } else { value = 11; }
                out[0] = value;
            }"#,
            &[0],
            &[11],
        ),
        case(
            "cuda::uninitialized::cuda_mutable_alias_locals_allow_later_initialization/1",
            r#"using Sample = float; using Value = Sample;
            __global__ void kernel(int *out) { Value value; value = 5.5f; out[0] = int(value); }"#,
            &[0],
            &[5],
        ),
        case(
            "cuda::uninitialized::cuda_mutable_scalar_locals_can_be_assigned_after_declaration/1",
            r#"__global__ void kernel(int *out) { int value; value = -7; out[0] = value; }"#,
            &[0],
            &[-7],
        ),
        case(
            "cuda::uninitialized::cuda_mutable_scalar_locals_can_be_assigned_after_declaration/2",
            r#"__global__ void kernel(int *out) { unsigned value; value = 0x123u; out[0] = value; }"#,
            &[0],
            &[291],
        ),
        case(
            "cuda::uninitialized::cuda_mutable_scalar_locals_can_be_assigned_after_declaration/3",
            r#"__global__ void kernel(int *out) { float value; value = 3.75f; out[0] = value; }"#,
            &[0],
            &[3],
        ),
        case(
            "cuda::uninitialized::cuda_mutable_scalar_locals_can_be_assigned_after_declaration/4",
            r#"__global__ void kernel(int *out) { bool value; value = true; out[0] = value; }"#,
            &[0],
            &[1],
        ),
        case(
            "cuda::uninitialized::cuda_scalar_declaration_lists_accept_initialized_and_omitted_initializers/1",
            r#"__global__ void kernel(int *out) { int value, other; value = 1; other = 2; out[0] = value + other; }"#,
            &[0],
            &[3],
        ),
        case(
            "cuda::uninitialized::cuda_scalar_declaration_lists_accept_initialized_and_omitted_initializers/2",
            r#"__global__ void kernel(int *out) { int value = 1, other = 2; out[0] = value + other; }"#,
            &[0],
            &[3],
        ),
    ]
}
