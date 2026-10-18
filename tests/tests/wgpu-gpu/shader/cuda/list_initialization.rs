use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    let mut cases = vec![case(
        "list initialization, constant bounds, ordered subobjects, copies and zero tails",
        include_str!("../../../../../naga/tests/in/cuda/list-initialization.cu"),
        &[
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,
        ],
        &[
            0,
            0,
            1,
            7,
            3,
            5,
            0,
            6,
            7,
            7,
            3,
            9,
            9,
            3,
            6,
            2,
            2,
            0,
            123,
            3,
            34,
            0,
            3,
            5,
            3,
            i32::MIN,
            0,
            14,
            5,
            9,
        ],
    )];
    cases.push(vector_array_forms());
    cases.push(sequencing_and_reinitialization());
    cases
}

fn vector_array_forms() -> ShaderTest {
    let mut source = String::from("__global__ void vector_lists(int *out) {\n");
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
                "{{\n    {vector_name} original{{{original_components}}};\n    const {vector_name} copied{{original}};\n    {vector_name} values[3]{{{{original}}, {elided_components}, 9}};\n    original.x = 99;\n    out[{output}] = int(copied.x);\n    out[{}] = int(values[1].{last_field});\n    out[{}] = int(values[2].y);\n}}\n",
                output + 1,
                output + 2,
            ));
            expected.extend([1, (width * 2) as i32, 0]);
        }
    }
    source.push_str("}\n");
    case(
        "all nine vector types copy whole values before nested and flat brace elision",
        &source,
        &[0; 27],
        &expected,
    )
}

fn sequencing_and_reinitialization() -> ShaderTest {
    case(
        "earlier element mutations survive later clauses and loop declarations reset every time",
        r#"__device__ int pair(int first, int second) {
    return first * 10 + second;
}
__global__ void ordered_lists(int *out) {
    int values[3]{1, values[0] + 1, values[1] + 1};
    out[0] = values[0];
    out[1] = values[1];
    out[2] = values[2];
    int changed[3]{1, (changed[0] = 7, 2), changed[0]};
    out[3] = changed[0];
    out[4] = changed[1];
    out[5] = changed[2];
    int3 point{1, point.x + 1, point.y + 1};
    out[6] = point.x;
    out[7] = point.y;
    out[8] = point.z;
    int3 changed_point{1, (changed_point.x = 9, 2), changed_point.x};
    out[9] = changed_point.x;
    out[10] = changed_point.y;
    out[11] = changed_point.z;
    int counter = 1;
    int called[3]{pair(counter++, counter++), counter++, counter};
    out[12] = called[0];
    out[13] = called[1];
    out[14] = called[2];
    out[15] = counter;
    int total{0};
    for (int iteration{0}; iteration < 4; ++iteration) {
        int reset[3]{iteration};
        int3 reset_point{iteration};
        total += reset[2] + reset_point.y;
        reset[2] = 9;
        reset_point.y = 8;
        if (iteration == 1) continue;
        total += reset[0];
    }
    out[16] = total;
    int2 nested[2]{{1}, {2}};
    int2 elided[2]{1, 0, 2};
    out[17] = nested[0].x + elided[0].x;
    out[18] = nested[1].x + elided[1].x;
    out[19] = nested[1].y + elided[1].y;
}"#,
        &[0; 20],
        &[1, 2, 3, 7, 2, 7, 1, 2, 3, 9, 2, 9, 12, 3, 4, 4, 5, 2, 4, 0],
    )
}
