use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::local_declarators::cuda_local_declarator_lists_do_not_add_expression_nesting/1",
            r#"__global__ void kernel(int *out) { int value0 = 0, value1 = value0 + 1, value2 = value1 + 1, value3 = value2 + 1, value4 = value3 + 1, value5 = value4 + 1, value6 = value5 + 1, value7 = value6 + 1, value8 = value7 + 1, value9 = value8 + 1, value10 = value9 + 1, value11 = value10 + 1, value12 = value11 + 1, value13 = value12 + 1, value14 = value13 + 1, value15 = value14 + 1, value16 = value15 + 1, value17 = value16 + 1, value18 = value17 + 1, value19 = value18 + 1, value20 = value19 + 1, value21 = value20 + 1, value22 = value21 + 1, value23 = value22 + 1, value24 = value23 + 1, value25 = value24 + 1, value26 = value25 + 1, value27 = value26 + 1, value28 = value27 + 1, value29 = value28 + 1, value30 = value29 + 1, value31 = value30 + 1, value32 = value31 + 1, value33 = value32 + 1, value34 = value33 + 1, value35 = value34 + 1, value36 = value35 + 1, value37 = value36 + 1, value38 = value37 + 1, value39 = value38 + 1, value40 = value39 + 1, value41 = value40 + 1, value42 = value41 + 1, value43 = value42 + 1, value44 = value43 + 1, value45 = value44 + 1, value46 = value45 + 1, value47 = value46 + 1, value48 = value47 + 1, value49 = value48 + 1, value50 = value49 + 1, value51 = value50 + 1, value52 = value51 + 1, value53 = value52 + 1, value54 = value53 + 1, value55 = value54 + 1, value56 = value55 + 1, value57 = value56 + 1, value58 = value57 + 1, value59 = value58 + 1, value60 = value59 + 1, value61 = value60 + 1, value62 = value61 + 1, value63 = value62 + 1, value64 = value63 + 1, value65 = value64 + 1, value66 = value65 + 1, value67 = value66 + 1, value68 = value67 + 1, value69 = value68 + 1, value70 = value69 + 1, value71 = value70 + 1, value72 = value71 + 1, value73 = value72 + 1, value74 = value73 + 1, value75 = value74 + 1, value76 = value75 + 1, value77 = value76 + 1, value78 = value77 + 1, value79 = value78 + 1, value80 = value79 + 1, value81 = value80 + 1, value82 = value81 + 1, value83 = value82 + 1, value84 = value83 + 1, value85 = value84 + 1, value86 = value85 + 1, value87 = value86 + 1, value88 = value87 + 1, value89 = value88 + 1, value90 = value89 + 1, value91 = value90 + 1, value92 = value91 + 1, value93 = value92 + 1, value94 = value93 + 1, value95 = value94 + 1, value96 = value95 + 1, value97 = value96 + 1, value98 = value97 + 1, value99 = value98 + 1, value100 = value99 + 1, value101 = value100 + 1, value102 = value101 + 1, value103 = value102 + 1, value104 = value103 + 1, value105 = value104 + 1, value106 = value105 + 1, value107 = value106 + 1, value108 = value107 + 1, value109 = value108 + 1, value110 = value109 + 1, value111 = value110 + 1, value112 = value111 + 1, value113 = value112 + 1, value114 = value113 + 1, value115 = value114 + 1, value116 = value115 + 1, value117 = value116 + 1, value118 = value117 + 1, value119 = value118 + 1, value120 = value119 + 1, value121 = value120 + 1, value122 = value121 + 1, value123 = value122 + 1, value124 = value123 + 1, value125 = value124 + 1, value126 = value125 + 1, value127 = value126 + 1; out[0] = value127; }"#,
            &[0],
            &[127],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarator_names_enter_scope_only_when_reached/1",
            r#"__global__ void kernel(int *out) {
                int value = 9;
                { int first = value, value = 3; out[0] = first; out[1] = value; }
                out[2] = value;
            }"#,
            &[0, 0, 0],
            &[9, 3, 9],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_allow_comma_expressions_in_full_expression_contexts/1",
            r#"__global__ void kernel(int *out) { int first = (1, 2); out[0] = first; }"#,
            &[0, 0],
            &[2, 0],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_allow_comma_expressions_in_full_expression_contexts/2",
            r#"__global__ void kernel(int *out) { int first = true ? 1, 2 : 3; out[0] = first; }"#,
            &[0, 0],
            &[2, 0],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_allow_comma_expressions_in_full_expression_contexts/3",
            r#"__global__ void kernel(int *out) { out[0] = 1, out[1] = 2; }"#,
            &[0, 0],
            &[1, 2],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_allow_mixed_omitted_and_initialized_values/1",
            r#"__global__ void kernel(int *out) { int first, second = 3, third; first = second + 1; third = first + second; out[0] = first; out[1] = second; out[2] = third; }"#,
            &[0, 0, 0],
            &[4, 3, 7],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_allow_mixed_omitted_and_initialized_values/2",
            r#"__global__ void kernel(int *out) { int first = 4, second, third = first + 3; second = third - first; out[0] = first; out[1] = second; out[2] = third; }"#,
            &[0, 0, 0],
            &[4, 3, 7],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_allow_mixed_omitted_and_initialized_values/3",
            r#"__global__ void kernel(int *out) { int first, second, third; first = 4; second = 3; third = 7; out[0] = first; out[1] = second; out[2] = third; }"#,
            &[0, 0, 0],
            &[4, 3, 7],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_apply_const_to_every_item/1",
            r#"using Frozen = const int; __global__ void kernel(int *out) {
                const int first = 3, second = first + 1; out[0] = second;
            }"#,
            &[0],
            &[4],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_apply_const_to_every_item/2",
            r#"using Frozen = const int; __global__ void kernel(int *out) {
                int const first = 3, second = first + 1; out[0] = second;
            }"#,
            &[0],
            &[4],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_apply_const_to_every_item/3",
            r#"using Frozen = const int; __global__ void kernel(int *out) {
                Frozen first = 3, second = first + 1; out[0] = second;
            }"#,
            &[0],
            &[4],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_cache_the_type_before_a_global_alias_is_hidden/1",
            r#"using Value = int; __global__ void kernel(int *out) { Value Value = 3, next = Value + 1; out[0] = Value; out[1] = next; }"#,
            &[0, 0],
            &[3, 4],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_cache_the_type_before_a_global_alias_is_hidden/2",
            r#"using Value = int; __global__ void kernel(int *out) { Value Value, next; Value = 3; next = Value + 1; out[0] = Value; out[1] = next; }"#,
            &[0, 0],
            &[3, 4],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_finish_each_initializer_before_the_next/1",
            r#"__global__ void kernel(int *out) { int first = 1, second = first + 2, third = second + 3;
            out[0] = first; out[1] = second; out[2] = third; }"#,
            &[0, 0, 0],
            &[1, 3, 6],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_finish_each_initializer_before_the_next/2",
            r#"__global__ void kernel(int *out) { int counter = 1; int first = counter++, second = counter++, third = counter;
            out[0] = first; out[1] = second; out[2] = third; }"#,
            &[0, 0, 0],
            &[1, 2, 3],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_finish_each_initializer_before_the_next/3",
            r#"__global__ void kernel(int *out) { int value = 5; int first = value, second = (value = 9), third = first;
            out[0] = first; out[1] = second; out[2] = third; }"#,
            &[0, 0, 0],
            &[5, 9, 5],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_load_conditional_references_before_later_side_effects/1",
            r#"__global__ void kernel(int *out) {
                int first = 3, second = 9;
                int selected = true ? first : second, changed = (first = 7);
                out[0] = selected; out[1] = changed;
            }"#,
            &[0, 0],
            &[3, 7],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_share_each_supported_scalar_alias_type/1",
            r#"using Value = int; __global__ void kernel(int *out) {
                Value first = -2, second; second = 7; out[0] = int(first + second);
            }"#,
            &[0],
            &[5],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_share_each_supported_scalar_alias_type/2",
            r#"using Value = unsigned; __global__ void kernel(int *out) {
                Value first = 3u, second; second = 5u; out[0] = int(first + second);
            }"#,
            &[0],
            &[8],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_share_each_supported_scalar_alias_type/3",
            r#"using Value = float; __global__ void kernel(int *out) {
                Value first = 2.5f, second; second = 3.5f; out[0] = int(first + second);
            }"#,
            &[0],
            &[6],
        ),
        case(
            "cuda::local_declarators::cuda_local_declarators_share_each_supported_scalar_alias_type/4",
            r#"using Value = bool; __global__ void kernel(int *out) {
                Value first = true, second; second = false; out[0] = int(first + second);
            }"#,
            &[0],
            &[1],
        ),
    ]
}
