use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::conditionals::cuda_conditional_arithmetic_arms_use_a_common_type/1",
            r#"__global__ void kernel(int *out) {
            out[0] = (false ? 1 : 2.5f) * 2.0f;
            out[1] = 0 < (false ? -1 : 1u);
            out[2] = (false ? true : 4) + 2;
            out[3] = false ? true : false;
        }"#,
            &[0, 0, 0, 0],
            &[5, 1, 6, 0],
        ),
        case(
            "cuda::conditionals::cuda_conditional_arithmetic_arms_use_a_common_type/2",
            r#"__global__ void kernel(int *out) {
            out[0] = (true ? 1 : 2.5f) * 2.0f;
            out[1] = 0 < (true ? -1 : 1u);
            out[2] = (true ? true : 4) + 2;
            out[3] = true ? true : false;
        }"#,
            &[0, 0, 0, 0],
            &[2, 1, 3, 1],
        ),
        case(
            "cuda::conditionals::cuda_conditional_assignment_and_prefix_arms_remain_references/1",
            r#"__global__ void kernel(int *out) {
        int a = 1; int b = 2;
        (true ? ++a : ++b) = 7;
        (false ? (a = 3) : (b = 4)) += 2;
        out[0] = a; out[1] = b;
    }"#,
            &[0, 0],
            &[7, 6],
        ),
        case(
            "cuda::conditionals::cuda_conditional_assignment_and_update_evaluate_selected_index_once/1",
            r#"__global__ void kernel(int *out) {
            int a = 0; int b = 1;
            int old = ((false ? out[a++] : out[b++]) = 7)++;
            out[2] = a; out[3] = b; out[4] = old;
        }"#,
            &[-9, -9, -9, -9, -9],
            &[-9, 8, 0, 2, 7],
        ),
        case(
            "cuda::conditionals::cuda_conditional_assignment_and_update_evaluate_selected_index_once/2",
            r#"__global__ void kernel(int *out) {
            int a = 0; int b = 1;
            int old = ((true ? out[a++] : out[b++]) = 7)++;
            out[2] = a; out[3] = b; out[4] = old;
        }"#,
            &[-9, -9, -9, -9, -9],
            &[8, -9, 1, 1, 7],
        ),
        case(
            "cuda::conditionals::cuda_conditional_assignment_result_keeps_its_original_selection/1",
            r#"__global__ void kernel(int *out) {
        int condition = 1; int other = 20;
        int old = ((condition ? condition : other) = 0)++;
        out[0] = condition; out[1] = other; out[2] = old;
    }"#,
            &[0, 0, 0],
            &[1, 20, 0],
        ),
        case(
            "cuda::conditionals::cuda_conditional_assignment_rhs_precedes_condition_and_target_evaluation/1",
            r#"__global__ void kernel(int *out) {
            out[0] = 10; out[1] = 20; int index = 0;
            ((index++ != 0) ? out[0] : out[1]) = ++index;
            out[2] = index;
        }"#,
            &[-9, -9, -9],
            &[1, 20, 2],
        ),
        case(
            "cuda::conditionals::cuda_conditional_assignment_rhs_precedes_condition_and_target_evaluation/2",
            r#"__global__ void kernel(int *out) {
            out[0] = 10; out[1] = 20; int index = 0;
            ((index++ != 0) ? out[0] : out[1]) += ++index;
            out[2] = index;
        }"#,
            &[-9, -9, -9],
            &[11, 20, 2],
        ),
        case(
            "cuda::conditionals::cuda_conditional_bool_references_can_be_assigned/1",
            r#"__global__ void kernel(int *out) {
        bool a = false; bool b = false;
        (true ? a : b) = true;
        out[0] = a; out[1] = b;
    }"#,
            &[0, 0],
            &[1, 0],
        ),
        case(
            "cuda::conditionals::cuda_conditional_compounds_modify_only_the_selected_location/1",
            r#"__global__ void kernel(int *out) {
            int a = 10; int b = 20;
            (false ? a : b) += 3;
            (false ? a : b) *= 2;
            out[0] = a; out[1] = b;
        }"#,
            &[0, 0],
            &[10, 46],
        ),
        case(
            "cuda::conditionals::cuda_conditional_compounds_modify_only_the_selected_location/2",
            r#"__global__ void kernel(int *out) {
            int a = 10; int b = 20;
            (true ? a : b) += 3;
            (true ? a : b) *= 2;
            out[0] = a; out[1] = b;
        }"#,
            &[0, 0],
            &[26, 20],
        ),
        case(
            "cuda::conditionals::cuda_conditional_conditions_use_scalar_truthiness/1",
            r#"__global__ void kernel(int *out) {
        out[0] = -1 ? 1 : 2;
        out[1] = 0u ? 3 : 4;
        out[2] = -0.0f ? 5 : 6;
        out[3] = 0.5f ? 7 : 8;
        out[4] = false ? 9 : 10;
    }"#,
            &[0, 0, 0, 0, 0],
            &[1, 4, 6, 7, 10],
        ),
        case(
            "cuda::conditionals::cuda_conditional_const_references_allow_read_and_discard/1",
            r#"__global__ void kernel(int *out) {
        int mutable_value = 3; const int fixed = 7;
        out[0] = true ? mutable_value : fixed;
        out[1] = false ? fixed : mutable_value;
        true ? fixed : mutable_value;
    }"#,
            &[0, 0],
            &[3, 3],
        ),
        case(
            "cuda::conditionals::cuda_conditional_evaluates_the_condition_once_before_its_selected_arm/1",
            r#"__global__ void kernel(int *out) {
            int condition = 0; int a = 10; int b = 20;
            int result = condition++ ? ++a : ++b;
            out[0] = result; out[1] = condition; out[2] = a; out[3] = b;
        }"#,
            &[0, 0, 0, 0],
            &[21, 1, 10, 21],
        ),
        case(
            "cuda::conditionals::cuda_conditional_evaluates_the_condition_once_before_its_selected_arm/2",
            r#"__global__ void kernel(int *out) {
            int condition = 1; int a = 10; int b = 20;
            int result = condition++ ? ++a : ++b;
            out[0] = result; out[1] = condition; out[2] = a; out[3] = b;
        }"#,
            &[0, 0, 0, 0],
            &[11, 2, 11, 20],
        ),
        case(
            "cuda::conditionals::cuda_conditional_precedence_and_right_associativity_match_cpp/1",
            r#"__global__ void kernel(int *out) {
        int value = 0;
        out[0] = false ? 1 : true ? 2 : 3;
        out[1] = true ? false ? 4 : 5 : 6;
        out[2] = false || true ? 7 : 8;
        out[3] = true ? value = 9 : value = 10;
        out[4] = value;
        out[5] = false ? 11 : value = 12;
        out[6] = value;
    }"#,
            &[0, 0, 0, 0, 0, 0, 0],
            &[2, 5, 7, 9, 9, 12, 12],
        ),
        case(
            "cuda::conditionals::cuda_conditional_prefix_and_postfix_results_keep_their_value_categories/1",
            r#"__global__ void kernel(int *out) {
        int a = 3; int b = 7;
        int old = (true ? a : b)++;
        int current = ++(false ? a : b);
        (++(true ? a : b)) = 9;
        out[0] = old; out[1] = current; out[2] = a; out[3] = b;
    }"#,
            &[0, 0, 0, 0],
            &[3, 8, 9, 8],
        ),
        case(
            "cuda::conditionals::cuda_discarded_conditional_assignment_results_do_not_load_storage/1",
            r#"__global__ void kernel(int *out) {
            false ? (out[0] = 3) : (out[1] = 4);
        }"#,
            &[-9, -9],
            &[-9, 4],
        ),
        case(
            "cuda::conditionals::cuda_discarded_conditional_assignment_results_do_not_load_storage/2",
            r#"__global__ void kernel(int *out) {
            true ? (out[0] = 3) : (out[1] = 4);
        }"#,
            &[-9, -9],
            &[3, -9],
        ),
        case(
            "cuda::conditionals::cuda_discarded_conditional_references_have_effects_without_storage_reads/1",
            r#"__global__ void kernel(int *out) {
            int a = 0; int b = 1;
            false ? out[a++] : out[b++];
            out[2] = a; out[3] = b;
        }"#,
            &[-9, -9, -9, -9],
            &[-9, -9, 0, 2],
        ),
        case(
            "cuda::conditionals::cuda_discarded_conditional_references_have_effects_without_storage_reads/2",
            r#"__global__ void kernel(int *out) {
            int a = 0; int b = 1;
            true ? out[a++] : out[b++];
            out[2] = a; out[3] = b;
        }"#,
            &[-9, -9, -9, -9],
            &[-9, -9, 1, 1],
        ),
        case(
            "cuda::conditionals::cuda_matching_conditional_references_accept_simple_assignment/1",
            r#"__global__ void kernel(int *out) {
            int local = 3; (false ? local : out[0]) = 7;
            out[1] = local;
        }"#,
            &[-9, -9],
            &[7, 3],
        ),
        case(
            "cuda::conditionals::cuda_matching_conditional_references_accept_simple_assignment/2",
            r#"__global__ void kernel(int *out) {
            int local = 3; (true ? local : out[0]) = 7;
            out[1] = local;
        }"#,
            &[-9, -9],
            &[-9, 7],
        ),
        case(
            "cuda::conditionals::cuda_nested_conditional_references_preserve_the_selected_address/1",
            r#"__global__ void kernel(int *out) {
            int a = 1; int b = 2; int c = 3;
            ((false ? (false ? a : b) : c) = 9)++;
            out[0] = a; out[1] = b; out[2] = c;
        }"#,
            &[0, 0, 0],
            &[1, 2, 10],
        ),
        case(
            "cuda::conditionals::cuda_nested_conditional_references_preserve_the_selected_address/2",
            r#"__global__ void kernel(int *out) {
            int a = 1; int b = 2; int c = 3;
            ((true ? (false ? a : b) : c) = 9)++;
            out[0] = a; out[1] = b; out[2] = c;
        }"#,
            &[0, 0, 0],
            &[1, 10, 3],
        ),
        case(
            "cuda::conditionals::cuda_nested_conditional_references_preserve_the_selected_address/3",
            r#"__global__ void kernel(int *out) {
            int a = 1; int b = 2; int c = 3;
            ((true ? (true ? a : b) : c) = 9)++;
            out[0] = a; out[1] = b; out[2] = c;
        }"#,
            &[0, 0, 0],
            &[10, 2, 3],
        ),
    ]
}
