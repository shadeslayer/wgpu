use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::updates::cuda_assignment_evaluates_rhs_before_the_lhs_index/1",
            r#"__global__ void update(int *out) { int i = 0; out[i++] = i++; out[2] = i; }"#,
            &[-999, -999, -999],
            &[-999, 0, 2],
        ),
        case(
            "cuda::updates::cuda_assignment_loads_rhs_before_the_lhs_changes_it/1",
            r#"__global__ void update(int *out) { int i = 0; out[i++] = i; out[2] = i; }"#,
            &[-999, -999, -999],
            &[0, -999, 1],
        ),
        case(
            "cuda::updates::cuda_assignments_read_postfix_values_before_the_lhs_is_modified/1",
            r#"__global__ void update(int *out) { int i = 4; i = i++; out[0] = i;
i = 4; i = ++i; out[1] = i; }"#,
            &[0, 0],
            &[4, 5],
        ),
        case(
            "cuda::updates::cuda_buffer_element_updates_preserve_old_values/1",
            r#"__global__ void update(int *out) { out[0] = 4; int old = out[0]++;
out[1] = old; out[2] = ++out[0]; }"#,
            &[0, 0, 0],
            &[6, 4, 6],
        ),
        case(
            "cuda::updates::cuda_chained_assignments_associate_right_and_return_an_lvalue/1",
            r#"__global__ void update(int *out) { int a = 1; int b = 2; int value = ++(a = b = 7);
(a = b) = 9; out[0] = value; out[1] = a; out[2] = b; }"#,
            &[0, 0, 0],
            &[8, 9, 7],
        ),
        case(
            "cuda::updates::cuda_compound_assignment_evaluates_its_index_once_after_the_rhs/1",
            r#"__global__ void update(int *out) { int i = 0; out[i++] += ++i; out[2] = i; }"#,
            &[-999, 10, -999],
            &[-999, 11, 2],
        ),
        case(
            "cuda::updates::cuda_compound_assignment_loads_rhs_before_the_lhs_changes_it/1",
            r#"__global__ void update(int *out) { int i = 1; out[i++] += i; out[2] = i; }"#,
            &[-999, 10, -999],
            &[-999, 11, 2],
        ),
        case(
            "cuda::updates::cuda_compound_assignments_convert_arithmetic_back_to_the_lhs_type/1",
            r#"__global__ void update(int *out) { int value = 3; value *= 1.5f;
value += 2u; value /= 2; value -= 1; out[0] = value; }"#,
            &[0],
            &[2],
        ),
        case(
            "cuda::updates::cuda_discarded_updates_still_apply_their_side_effects/1",
            r#"__global__ void update(int *out) { int i = 0; i++; ++i; out[0] = i; }"#,
            &[0],
            &[2],
        ),
        case(
            "cuda::updates::cuda_logical_operators_keep_updates_in_the_selected_branch/1",
            r#"__global__ void update(int *out) { int x = 0; bool a = false && ++x;
bool b = true || ++x; bool c = true && ++x; bool d = false || ++x;
out[0] = x; out[1] = a; out[2] = b; out[3] = c; out[4] = d; }"#,
            &[0, 0, 0, 0, 0],
            &[2, 0, 1, 1, 1],
        ),
        case(
            "cuda::updates::cuda_nested_assignment_indices_preserve_rhs_order/1",
            r#"__global__ void update(int *out) { int i = 0; out[(i = 2)] = (i = 1); out[3] = i; }"#,
            &[-999, -999, -999, -999],
            &[-999, -999, 1, 2],
        ),
        case(
            "cuda::updates::cuda_prefix_and_postfix_updates_return_new_and_old_values/1",
            r#"__global__ void update(int *out) { int x = 3; int post = x++; int pre = ++x;
int old = x--; int last = --x; out[0] = post; out[1] = pre; out[2] = old; out[3] = last; out[4] = x; }"#,
            &[0, 0, 0, 0, 0],
            &[3, 5, 5, 3, 3],
        ),
    ]
}
