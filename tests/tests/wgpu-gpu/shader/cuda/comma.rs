use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    let flat_sequence_source = format!(
        "__global__ void kernel(int *out) {{ int count = 0; out[0] = ({}count); }}",
        "count += 1, ".repeat(128)
    );
    vec![
        case(
            "cuda::comma::cuda_comma_assignment_evaluates_rhs_before_the_complete_left_address/1",
            r#"__global__ void kernel(int *out) {
                int index = 0;
                (index++, out[index]) = (index += 2, 9);
                out[0] = index;
            }"#,
            &[0, 10, 20, 30],
            &[3, 10, 20, 9],
        ),
        case(
            "cuda::comma::cuda_comma_assignment_evaluates_rhs_before_the_complete_left_address/2",
            r#"__global__ void kernel(int *out) {
                int index = 0;
                (index++, out[index]) += (index += 2, 9);
                out[0] = index;
            }"#,
            &[0, 10, 20, 30],
            &[3, 10, 20, 39],
        ),
        case(
            "cuda::comma::cuda_comma_conditional_middle_is_full_expression_and_reject_is_assignment/1",
            r#"__global__ void kernel(int *out) {
                int value = 0;
                out[0] = true ? value = 1, value + 2 : 8;
                out[1] = false ? 1 : value = 4, value + 10;
                out[2] = (false ? 1 : value = 5, value + 10);
                out[3] = value;
            }"#,
            &[0, 0, 0, 0],
            &[3, 4, 15, 5],
        ),
        case(
            "cuda::comma::cuda_comma_discarded_references_evaluate_addresses_without_storage_loads/1",
            r#"__global__ void kernel(int *out) {
        int index = 0;
        (out[index++], true ? out[index++] : out[++index]);
        out[3] = index;
    }"#,
            &[-9, -9, -9, -9],
            &[-9, -9, -9, 2],
        ),
        case(
            "cuda::comma::cuda_comma_distinguishes_functional_cast_lists_from_full_expressions/1",
            r#"using Number = int;
            __global__ void kernel(int *out) {
                out[0] = int((1, 2));
                out[1] = Number((3, 4));
                out[2] = static_cast<int>(5, 6);
                out[(0, 3)] = 7;
                out[true ? 8, 3 : 2] = 7;
            }"#,
            &[0, 0, 0, 0],
            &[2, 4, 6, 7],
        ),
        case(
            "cuda::comma::cuda_comma_flat_sequences_do_not_exhaust_the_expression_nesting_limit/1",
            &flat_sequence_source,
            &[0],
            &[128],
        ),
        case(
            "cuda::comma::cuda_comma_has_lower_precedence_than_assignment/1",
            r#"__global__ void bad(int *out) { out[0] = 1, 2; }"#,
            &[0],
            &[1],
        ),
        case(
            "cuda::comma::cuda_comma_has_lower_precedence_than_assignment/2",
            r#"__global__ void kernel(int *out) {
                int a = 0, b = 1, c = 2;
                a = b, c; out[0] = a;
                a = (b, c); out[1] = a;
                a = b = 3, c = 4; out[2] = a; out[3] = c;
            }"#,
            &[0, 0, 0, 0],
            &[1, 2, 3, 4],
        ),
        case(
            "cuda::comma::cuda_comma_keeps_declarator_separators_outside_parentheses/1",
            r#"__global__ void kernel(int *out) {
                int counter = 0;
                int first = (counter += 1, counter + 2), second = (counter += 3, counter);
                out[0] = first; out[1] = second; out[2] = counter;
            }"#,
            &[0, 0, 0],
            &[3, 4, 4],
        ),
        case(
            "cuda::comma::cuda_comma_keeps_only_the_right_type_and_const_qualification/1",
            r#"__global__ void kernel(int *out) {
                const int fixed = 7; int value = 1;
                (fixed, value) = 3;
                float floating = (1, 2.75f);
                bool flag = (3u, false); unsigned natural = (false, 4u);
                out[0] = value; out[1] = floating + 0.5f;
                out[2] = flag; out[3] = natural;
            }"#,
            &[0, 0, 0, 0],
            &[3, 3, 0, 4],
        ),
        case(
            "cuda::comma::cuda_comma_preserves_nested_conditional_reference_identity/1",
            r#"__global__ void kernel(int *out) {
                int condition = 1, index = 0;
                ((condition = 0, condition ? out[index++] : out[++index]) = 7)++;
                out[3] = index;
            }"#,
            &[-9, -9, -9, -9],
            &[-9, 8, -9, 1],
        ),
        case(
            "cuda::comma::cuda_comma_preserves_nested_conditional_reference_identity/2",
            r#"__global__ void kernel(int *out) {
                int condition = 1, index = 0;
                (index++, condition ? out[1] : out[2]) += (condition = 0, 3);
                out[0] = index;
            }"#,
            &[0, 10, 20],
            &[1, 10, 23],
        ),
        case(
            "cuda::comma::cuda_comma_preserves_right_reference_assignment_and_updates/1",
            r#"__global__ void kernel(int *out) {
                int x = 2, y = 5;
                (x++, y) = 7;
                out[0] = (x++, y)++;
                out[1] = ++(x++, y);
                out[2] = x; out[3] = y;
            }"#,
            &[0, 0, 0, 0],
            &[7, 9, 5, 9],
        ),
        case(
            "cuda::comma::cuda_comma_sequences_each_left_operand_before_the_next/1",
            r#"__global__ void kernel(int *out) {
                int value = 0;
                out[0] = (value = 1, value += 2, value++);
                out[1] = value;
            }"#,
            &[0, 0],
            &[3, 4],
        ),
        case(
            "cuda::comma::cuda_comma_stays_inside_selected_short_circuit_and_conditional_branches/1",
            r#"__global__ void kernel(int *out) {
                int value = 0;
                false && (value = 2, true);
                true || (value = 3, false);
                out[0] = value;
                out[1] = true && (value = 4, value < 5);
                out[2] = value;
                out[3] = false ? (value = 9, value) : (value += 2, value);
            }"#,
            &[0, 0, 0, 0],
            &[0, 1, 4, 6],
        ),
    ]
}
