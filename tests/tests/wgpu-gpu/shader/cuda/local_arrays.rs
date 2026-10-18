use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "local array types, aliases, scopes, initialization and element references",
            include_str!("../../../../../naga/tests/in/cuda/local-arrays.cu"),
            &[
                3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            &[
                5, 1, 15, 101, 0, 7, 3, 0, 6, 7, 6, 3, 44, 5, 2, 31, 0, 16, 3, 5, 32, 1, 532, 9,
                1013,
            ],
        ),
        case(
            "empty local arrays reset at every declaration and helpers own their storage",
            r#"__device__ int local_sum(int seed) {
    int values[3]{};
    values[0] = seed;
    values[1] = seed + 1;
    return values[0] + values[1] + values[2];
}
__global__ void array_lifetimes(int *out) {
    for (int i = 0; i < 4; ++i) {
        int values[2]{};
        bool flags[2] = {};
        float samples[2]{};
        unsigned counts[2] = {};
        out[0] += values[0];
        out[1] += int(flags[1]);
        out[2] += int(samples[1]);
        out[3] += int(counts[1]);
        values[0] = 7;
        flags[1] = true;
        samples[1] = 1.5f;
        counts[1] = 9u;
    }
    out[4] = local_sum(3);
    out[5] = local_sum(7);
    const int zeroes[2]{};
    out[6] = zeroes[0] + zeroes[1];
}"#,
            &[0; 7],
            &[0, 0, 0, 0, 7, 15, 0],
        ),
        case(
            "local array aliases, right-first indexed assignments and lazy references",
            r#"using Count = int;
__global__ void array_updates(int *out) {
    Count Count[3]{}, next[2] = {}, scalar = 4;
    Count[0] = scalar;
    Count[1] = 6;
    Count[2] = 8;
    next[0] = 10;
    next[1] = 11;
    int index = 0;
    out[0] = (Count[index++] = ++index);
    out[1] = index;
    out[2] = Count[0];
    out[3] = Count[1];
    out[4] = Count[2];
    index = 0;
    out[5] = (Count[index++] += next[index++]);
    out[6] = index;
    out[7] = Count[1];
    int choose = 0;
    out[8] = ++(choose ? Count[index++] : next[choose++]);
    out[9] = choose;
    out[10] = index;
    out[11] = next[0];
    out[12] = next[1];
}"#,
            &[0; 13],
            &[1, 2, 4, 1, 8, 11, 2, 11, 11, 1, 2, 11, 11],
        ),
    ]
}
