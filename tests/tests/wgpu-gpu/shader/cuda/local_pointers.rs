use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "cuda::local_pointers::captures_live_local_pointer_views",
            include_str!("../../../../../naga/tests/in/cuda/local-pointers.cu"),
            &[
                101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, -97,
                179, -101,
            ],
            &[
                26, 16, 22, 29, 3, 0, 26, 33, 16, 20, 28, 41, 17, 31, 31, -97, 37, -101,
            ],
        ),
        case(
            "cuda::local_pointers::helper_reentry_and_declarator_scopes",
            r#"__device__ int advance(int *values, int slot, bool stop) {
    int *selected = values + slot;
    int *same{selected};
    const int *view = {same};
    *selected += 2;
    if (stop) return *view;
    same[1] += *view;
    return same[1];
}

__device__ void accumulate(int *destination, const int *source) {
    int *write = destination;
    const int *read = source;
    *write += *read;
}

__device__ int read(const int *value) {
    const int *local = value;
    return *local;
}

__global__ void kernel(int *out) {
    int values[5]{1, 3, 5, 7, 11};
    int total = 0;
    for (int iteration = 0; iteration < 3; ++iteration) {
        total += advance(values, iteration, iteration == 1);
    }
    accumulate(values + 1, values + 2);
    int *first = values, count = *first, &alias = *first, *second = first + 1;
    *second += count;
    alias += 2;

    int value = 19;
    int *outer = &value;
    {
        int value = 23;
        int *outer = &value;
        *outer += 6;
        out[10] = *outer;
    }
    out[11] = *outer;

    int cursor = 0;
    for (int *head = values, limit = 2; cursor < limit; ++cursor) {
        int *current = head + cursor;
        *current += 1;
    }
    switch (cursor) {
        case 2: {
            int *within = values + 2;
            *within += 3;
            out[13] = *within;
            break;
        }
        default: out[13] = -7;
    }
    out[0] = total;
    out[1] = values[0];
    out[2] = values[1];
    out[3] = values[2];
    out[4] = values[3];
    out[5] = values[4];
    out[6] = count;
    out[7] = alias;
    out[8] = *second;
    out[9] = read(first);
    out[12] = cursor;
}
"#,
            &[
                101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, -103, -107,
            ],
            &[
                28, 6, 19, 10, 14, 11, 3, 6, 19, 6, 29, 19, 2, 10, -103, -107,
            ],
        ),
    ]
}
