use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "inferred scalar and vector arrays, aliases, copies, completed bounds and zero fields",
            include_str!("../../../../../naga/tests/in/cuda/inferred-arrays.cu"),
            &[
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0,
            ],
            &[
                6,
                4,
                0,
                2,
                8,
                4,
                123,
                7,
                7,
                3,
                0,
                3,
                4,
                5,
                0,
                3,
                0,
                3,
                6,
                123,
                3,
                38,
                4,
                0,
                5,
                9,
                5,
                i32::MIN,
                0,
                17,
            ],
        ),
        vector_array_forms(),
        bound_facts_and_sequencing(),
    ]
}

fn vector_array_forms() -> ShaderTest {
    let mut source = String::from("__global__ void inferred_vectors(int *out) {\n");
    let mut expected = Vec::new();
    for base in ["int", "uint", "float"] {
        for width in [2, 3, 4] {
            let vector_name = format!("{base}{width}");
            let original_components = (1..=width)
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let elided_components = (width + 1..=width * 2)
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let last_field = ["x", "y", "z", "w"][width - 1];
            let output = expected.len();
            source.push_str(&format!(
                "{{\n    {vector_name} original{{{original_components}}};\n    const {vector_name} copied[]{{{{original}}, {elided_components}, 9}};\n    original.x = 99;\n    out[{output}] = int(copied[0].x);\n    out[{}] = int(copied[1].{last_field});\n    out[{}] = int(copied[2].y);\n}}\n",
                output + 1,
                output + 2,
            ));
            expected.extend([1, (width * 2) as i32, 0]);
        }
    }
    source.push_str("}\n");
    case(
        "all nine inferred vector array types count copies and elided partial elements",
        &source,
        &[0; 27],
        &expected,
    )
}

fn bound_facts_and_sequencing() -> ShaderTest {
    case(
        "pending bounds preserve constant facts while list stores and helper calls remain ordered",
        r#"using Count = int;
__device__ int pair(int first, int second) {
    return first * 10 + second;
}
__device__ int3 triple(int seed) {
    int3 value{seed, seed + 1, seed + 2};
    return value;
}
__device__ int sum(int seed) {
    int values[]{seed, seed + 1};
    return values[0] + values[1];
}
__global__ void inferred_effects(int *out) {
    int self[]{1, self[0] + 1, self[1] + 1};
    out[0] = self[0];
    out[1] = self[1];
    out[2] = self[2];
    int changed[]{1, (changed[0] = 7, 2), changed[0]};
    out[3] = changed[0];
    out[4] = changed[2];
    float pending[]{1.0f, (pending[0], 2)};
    out[5] = int(pending[0] + pending[1]);
    float converted[]{1.0f, unsigned((converted[0], 2))};
    out[6] = int(converted[1]);
    float skipped_and[]{1.0f, false && (skipped_and[100], true)};
    float skipped_or[]{1.0f, true || (skipped_or[100], false)};
    out[7] = int(skipped_and[1]);
    out[8] = int(skipped_or[1]);
    float conditional[]{1.0f, ((true ? conditional[0] : conditional[100]), 2)};
    out[9] = int(conditional[0] + conditional[1]);
    int counter = 1;
    int calls[]{pair(counter++, counter++), counter++, counter};
    out[10] = calls[0];
    out[11] = calls[1];
    out[12] = counter;
    int3 copied[]{triple(5), triple(9)};
    out[13] = copied[0].x;
    out[14] = copied[1].z;
    int total = 0;
    for (int iteration = 0; iteration < 4; ++iteration) {
        int2 reset[]{{iteration}, {}};
        total += reset[0].y + reset[1].x;
        reset[0].y = 9;
        reset[1].x = 8;
        if (iteration == 1) continue;
        total += reset[0].x;
    }
    out[15] = total;
    bool flags[]{true, {}};
    out[16] = int(flags[1]);
    const int constants[]{3, 4};
    out[17] = constants[0] + constants[1];
    Count Count[]{4, 5}, next[]{Count[1]};
    out[18] = next[0];
    out[19] = sum(3);
    int3 changed_vector[]{{1, (changed_vector[0].x = 9, 2), changed_vector[0].x}};
    out[20] = changed_vector[0].x;
    out[21] = changed_vector[0].z;
    int2 nested[]{{1}, {2}};
    out[22] = nested[1].x;
    out[23] = nested[1].y;
}"#,
        &[0; 24],
        &[
            1, 2, 3, 7, 7, 3, 2, 0, 1, 3, 12, 3, 4, 5, 11, 5, 0, 7, 5, 7, 9, 9, 2, 0,
        ],
    )
}
