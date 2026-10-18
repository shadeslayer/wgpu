use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::reference_categories::cuda_assignment_result_keeps_the_original_indexed_reference/1",
            r#"__global__ void kernel(int *out) {
        int index = 0; (out[index++] = 7) = 9; out[2] = index;
    }"#,
            &[-9, -9, -9],
            &[9, -9, 1],
        ),
        case(
            "cuda::reference_categories::cuda_discarded_reference_evaluates_its_index_without_loading_storage/1",
            r#"__global__ void kernel(int *out) {
        int index = 0; out[index++]; out[2] = index;
    }"#,
            &[-9, -9, -9],
            &[-9, -9, 1],
        ),
        case(
            "cuda::reference_categories::cuda_postfix_result_is_the_old_value_of_the_selected_location/1",
            r#"__global__ void kernel(int *out) {
        out[0] = 3; int index = 0; int previous = out[index++]++;
        out[1] = previous; out[2] = index;
    }"#,
            &[-9, -9, -9],
            &[4, 3, 1],
        ),
        case(
            "cuda::reference_categories::cuda_prefix_result_keeps_the_original_indexed_reference/1",
            r#"__global__ void kernel(int *out) {
        out[0] = 3; int index = 0; (++out[index++]) = 8; out[2] = index;
    }"#,
            &[-9, -9, -9],
            &[8, -9, 1],
        ),
        case(
            "cuda::reference_categories::cuda_reference_assignment_and_compound_evaluate_rhs_before_index/1",
            r#"__global__ void kernel(int *out) {
            out[0] = 10; out[1] = 20; int index = 0;
            out[index++] = ++index; out[2] = index;
        }"#,
            &[-9, -9, -9],
            &[10, 1, 2],
        ),
        case(
            "cuda::reference_categories::cuda_reference_assignment_and_compound_evaluate_rhs_before_index/2",
            r#"__global__ void kernel(int *out) {
            out[0] = 10; out[1] = 20; int index = 0;
            out[index++] += ++index; out[2] = index;
        }"#,
            &[-9, -9, -9],
            &[10, 21, 2],
        ),
    ]
}
