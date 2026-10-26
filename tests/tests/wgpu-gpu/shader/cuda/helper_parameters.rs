use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        live_aliases(),
        argument_capture(),
        address_spaces(),
        pointer_branch_captures(),
    ]
}

fn live_aliases() -> ShaderTest {
    let mut expected = [SENTINEL; 48];
    expected[..40].copy_from_slice(&[
        14, 9, 4, 14, 7, 7, 0x40e00000, 0x40e00000, 23, 14, 14, 12, 9, 11, 5, 7, 0x41500000,
        0x3fc00000, 1, 8, 6, 7, 8, 7, 5, 5, 10, 10, 10, 8, 20, 20, 707, 7, 705, 7, 1, 1, 3, 2,
    ]);
    module_case(
        "live aliases, overlapping objects and source operator parameters",
        include_str!("../../../../../naga/tests/in/cuda/helper-parameters.cu"),
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 48],
        )],
        0,
        [1, 1, 1],
    )
}

fn argument_capture() -> ShaderTest {
    let mut expected = [SENTINEL; 48];
    expected[..40].copy_from_slice(&[
        9, 9, 3, 9, 0x40400000, 9, 11, 22, 2, 3, 6, 6, 1, 5, 5, 5, 6, 43, 43, 1, 7, 5, 102, 3, 5,
        28, 10, 10, 17, 17, 3, 3, 3, 5, 9, 9, 10909, 13, 12, 1,
    ]);
    module_case(
        "argument captures, frozen projections and callee return isolation",
        r#"struct Record { int x; int y; int values[2]; };
__device__ int live_read(const int &value, int ignored) { return value; }
__device__ int value_read(int value, int ignored) { return value; }
__device__ float converted_read(const float &value, int ignored) { return value; }
__device__ void add_pair(int &first, int &second) { ++first; second += 2; }
__device__ int inspect(int &value, int ignored) { value += 2; return value; }
__device__ void nested_project(Record &first, Record &second, bool choose, int &result) {
    Record &whole = choose ? first : second; int &x = whole.x; int &y = whole.y;
    add_pair(choose ? x : y, x); int index = 0;
    inspect(whole.values[index++], (index = 1));
    result = whole.x * 100 + whole.y + index * 10000;
}
__device__ int nested(int &value) { return inspect(value, 0) + 2; }
__device__ int temporary_pair(const int &first, const int &second) { return first * 100 + second; }
__device__ int step(int &value, int mode) {
    for (int iteration = 0; iteration < 3; ++iteration) {
        if (mode == 0) { value += 2; return value; }
        if (iteration == 1) { continue; }
        ++value;
    }
    switch (mode) { case 1: value += 3; break; default: value += 5; }
    return value;
}
__device__ void advance(int &value) { ++value; }
__global__ void capture(unsigned *out) {
    int value = 3; out[0] = unsigned(live_read(value, (value = 9))); out[1] = unsigned(value);
    value = 3; out[2] = unsigned(value_read(value, (value = 9))); out[3] = unsigned(value);
    value = 3; out[4] = __float_as_uint(converted_read(value, (value = 9))); out[5] = unsigned(value);
    int values[2] = {10, 20}; int index = 0; add_pair(values[index++], values[index++]); out[6] = unsigned(values[0]); out[7] = unsigned(values[1]); out[8] = unsigned(index);
    bool choose = false; int left = 3, right = 4; int selected = inspect(choose ? left : right, (choose = true)); out[9] = unsigned(left); out[10] = unsigned(right); out[11] = unsigned(selected); out[12] = unsigned(choose);
    Record first{3, 4, {31, 37}}, second{5, 6, {41, 47}};
    Record &whole = choose ? first : second; int &x = whole.x; int &y = whole.y;
    choose = false; add_pair(choose ? x : y, x); out[13] = unsigned(first.x); out[14] = unsigned(first.y); out[15] = unsigned(second.x); out[16] = unsigned(second.y);
    Record &array_record = choose ? first : second; index = 0;
    out[17] = unsigned(inspect(array_record.values[index++], (index = 1))); out[18] = unsigned(second.values[0]); out[19] = unsigned(index);
    value = 3; out[20] = unsigned(nested(value)); out[21] = unsigned(value);
    int counter = 1; out[22] = unsigned(temporary_pair(counter++, counter++)); out[23] = unsigned(counter);
    value = 1; int total = 0; for (int iteration = 0; iteration < 2; ++iteration) { total += step(value, 0); total += 10; }
    out[24] = unsigned(value); out[25] = unsigned(total);
    out[26] = unsigned(step(value, 1)); out[27] = unsigned(value); out[28] = unsigned(step(value, 2)); out[29] = unsigned(value);
    int cursor = 0; int sum = 0; for (; cursor < 3; advance(cursor)) { sum += cursor; } out[30] = unsigned(sum); out[31] = unsigned(cursor);
    Record selected_first{3, 4, {1, 2}}, selected_second{5, 6, {3, 4}}, third{7, 8, {11, 12}};
    bool first_choice = false, second_choice = true;
    Record &first_view = first_choice ? selected_first : selected_second;
    Record &second_view = second_choice ? third : selected_first;
    int projected = 0; nested_project(first_view, second_view, false, projected);
    out[32] = unsigned(selected_first.x); out[33] = unsigned(selected_second.x);
    out[34] = unsigned(third.x); out[35] = unsigned(third.y); out[36] = unsigned(projected);
    out[37] = unsigned(third.values[0]); out[38] = unsigned(third.values[1]); out[39] = unsigned(second_choice);
}"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[SENTINEL; 48],
            ),
        ],
        0,
        [1, 1, 1],
    )
}

fn address_spaces() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        14, 12, 14, 14, 1, 20, 200, 5, 19, 15, 19, 19, 2, 21, 202, 6, 24, 18, 24, 24, 3, 22, 204,
        7, 29, 21, 29, 29, 4, 23, 206, 8,
    ]);
    module_case(
        "WorkGroup and storage roots, readonly record input and late builtin helper",
        r#"struct Record { int value; int samples[2]; };
__device__ void update(Record &record, int &sample, const int &read, unsigned lane) {
    record.value += int(lane) + 1; sample += read; record.samples[1] = record.value + sample;
}
__device__ int forward(Record &record, const Record &view) { int &alias = record.value; alias += view.samples[0]; return alias; }
__device__ int readonly(const int &value) { return value; }
__device__ void storage_alias(unsigned &write, const unsigned &read) { write += read; }
__device__ unsigned position(unsigned &destination);
__global__ void spaces(unsigned *out, const Record *input) {
    unsigned lane = threadIdx.x; __shared__ Record shared[4]; shared[lane] = input[lane]; __syncthreads();
    update(shared[lane], shared[lane].samples[0], shared[lane].value, lane);
    int result = forward(shared[lane], shared[lane]); __syncthreads();
    unsigned slot = lane * 8u;
    out[slot] = unsigned(shared[lane].value); out[slot + 1u] = unsigned(shared[lane].samples[0]); out[slot + 2u] = unsigned(shared[lane].samples[1]); out[slot + 3u] = unsigned(result);
    out[slot + 4u] = unsigned(input[lane].value); out[slot + 5u] = unsigned(readonly(input[lane].samples[1]));
    out[slot + 6u] = 100u + lane; storage_alias(out[slot + 6u], out[slot + 6u]);
    position(out[slot + 7u]);
}
__device__ unsigned position(unsigned &destination) { destination = threadIdx.x + blockDim.x + gridDim.x; return destination; }"#,
        [4, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[SENTINEL; 40],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &[1, 10, 20, 2, 11, 21, 3, 12, 22, 4, 13, 23, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL],
            ),
        ],
        0,
        [1, 1, 1],
    )
}

fn pointer_branch_captures() -> ShaderTest {
    let expected = [5, 5, 19, 11, SENTINEL, SENTINEL, SENTINEL, SENTINEL];
    module_case(
        "saved pointer references across branches and selected fields",
        r#"struct Record { int first; int second; };

__device__ int read_saved(int *values, bool choose) {
    int &saved = values[1];
    if (choose) {
        return saved;
    } else {
        return saved;
    }
}

__device__ int read_value(const int *value) { return *value; }

__device__ int choose_field(Record *values, bool choose) {
    Record &saved = values[1];
    int &selected = choose ? saved.first : saved.second;
    return read_value(&selected);
}

__global__ void pointer_branch_captures(unsigned *out, const unsigned *selection) {
    int values[3]{3, 5, 7};
    out[0] = unsigned(read_saved(values, bool(selection[0])));
    out[1] = unsigned(read_saved(values, bool(selection[1])));
    Record records[2]{{3, 5}, {11, 19}};
    out[2] = unsigned(choose_field(records, bool(selection[0])));
    out[3] = unsigned(choose_field(records, bool(selection[1])));
}
"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[SENTINEL; 8],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &[0, 1, SENTINEL, SENTINEL],
            ),
        ],
        0,
        [1, 1, 1],
    )
}
