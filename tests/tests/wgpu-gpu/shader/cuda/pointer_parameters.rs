use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        offsets_and_live_aliases(),
        argument_capture_and_returns(),
        shared_and_packed_storage_roots(),
        own_function_pointer_lowering(),
    ]
}

fn offsets_and_live_aliases() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        1, 13, 8, 4, 138, 56, 9, 11, 12, 12, 23, 5, 4, 18, 6, 13, 8, 9, 2, 11, 8, 7, 23, 5,
        0x3fc00000, 0x40800000, 0x41000000, 1, 0, 10, 20, 23,
    ]);
    module_case(
        "borrowed pointer aliases, signed offsets and nested references",
        include_str!("../../../../../naga/tests/in/cuda/pointer-parameters.cu"),
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 40],
        )],
        0,
        [1, 1, 1],
    )
}

fn argument_capture_and_returns() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        11, 22, 2, 43, 30, 43, 1, 14, 14, 22, 0, 99, 99, 3, 9, 25, 25, 4, 50, 6, 33, 33, 5, 28, 10,
        10, 17, 17, 3, 3, 0, 2,
    ]);
    module_case("captured pointer offsets and isolated helper returns", r#"__device__ void add_pair(int *first, int *second) { first[0] += 1; second[0] += 2; }
__device__ int live(const int *value, int ignored) { return value[0]; }
__device__ int copied(int value, int *destination) { destination[0] = 9; return value; }
__device__ int inspect(int *value, int ignored) { value[0] += 3; return value[0]; }
__device__ int previous(const int *value) { int index = -1; return index[(index = 0, value)]; }
__device__ int nested_choice(int *first, int *second, bool choose) { return inspect(choose ? first : second, 0); }
__device__ int step(int *value, int mode) {
    for (int iteration = 0; iteration < 3; ++iteration) {
        if (mode == 0) { value[0] += 2; return value[0]; }
        if (iteration == 1) { continue; }
        ++value[0];
    }
    switch (mode) { case 1: value[0] += 3; break; default: value[0] += 5; }
    return value[0];
}
__device__ void advance(int *value) { ++value[0]; }
__global__ void pointer_capture(unsigned *out, const unsigned *selection) {
    int values[5]{10, 20, 30, 40, 50}; int index = 0; add_pair(values + index++, index++ + values);
    out[0] = unsigned(values[0]); out[1] = unsigned(values[1]); out[2] = unsigned(index);
    bool choose = bool(selection[0]); out[3] = unsigned(inspect(&(choose ? values[2] : values[3]), (choose = true)));
    out[4] = unsigned(values[2]); out[5] = unsigned(values[3]); out[6] = unsigned(choose);
    choose = bool(selection[1]); out[7] = unsigned(inspect(choose ? &values[0] : &values[1], (choose = false)));
    out[8] = unsigned(values[0]); out[9] = unsigned(values[1]); out[10] = unsigned(choose);
    out[11] = unsigned(live(&values[0], (values[0] = 99))); out[12] = unsigned(values[0]);
    values[0] = 3; out[13] = unsigned(copied(values[0], &values[0])); out[14] = unsigned(values[0]);
    index = 1; out[15] = unsigned(inspect(values + index++, (index = 4))); out[16] = unsigned(values[1]); out[17] = unsigned(index);
    index = 5; out[18] = unsigned(previous((&values[index++]) - 0)); out[19] = unsigned(index);
    int other[2]{30, 40}; out[20] = unsigned(nested_choice(&values[0], other, bool(selection[0]))); out[21] = unsigned(other[0]);
    int current = 1, total = 0; for (int iteration = 0; iteration < 2; ++iteration) { total += step(&current, 0); total += 10; }
    out[22] = unsigned(current); out[23] = unsigned(total); out[24] = unsigned(step(&current, 1)); out[25] = unsigned(current);
    out[26] = unsigned(step(&current, 2)); out[27] = unsigned(current);
    int cursor = 0, sum = 0; for (; cursor < 3; advance(&cursor)) { sum += cursor; } out[28] = unsigned(sum); out[29] = unsigned(cursor);
    index = 0; bool skipped = false && inspect(&values[index++], 0); bool kept = true || inspect(&values[index++], 0);
    out[30] = unsigned(index); out[31] = unsigned(skipped) + 2u * unsigned(kept);
}"#, [1, 1, 1], &expected)
        .module_buffers(vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 40]), ShaderBuffer::new(1, wgpu::BufferBindingType::Storage { read_only: true }, &[0, 1, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL])], 0, [1, 1, 1])
}

fn shared_and_packed_storage_roots() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        14, 12, 14, 14, 3, 5, 200, 5, 19, 15, 19, 19, 4, 7, 202, 6, 24, 18, 24, 24, 5, 9, 204, 7,
        29, 21, 29, 29, 6, 11, 206, 8,
    ]);
    module_case("shared and packed readonly record pointer roots", r#"struct Record { int3 point; int samples[2]; };
__device__ void update(Record *value, int *sample, const Record *view, unsigned lane) {
    value[0].point.x += int(lane) + 1; sample[0] += view[0].point.x; value[0].samples[1] = value[0].point.x + sample[0];
}
__device__ int forward(Record *value, const Record *view) { int &alias = value[0].point.x; alias += view[0].samples[0]; return alias; }
__device__ void vector_update(int3 *value, const int3 *view) { value[0].y += view[0].z; }
__device__ int previous_z(const Record *value) { return value[-1].point.z; }
__device__ void storage_alias(unsigned *value, const unsigned *view) { value[0] += view[0]; }
__device__ unsigned position(unsigned *value) { value[0] = threadIdx.x + blockDim.x + gridDim.x; return value[0]; }
__global__ void pointer_spaces(unsigned *out, const Record *input) {
    unsigned lane = threadIdx.x; __shared__ Record shared[4]; shared[lane] = input[lane]; __syncthreads();
    update(&shared[lane], &shared[lane].samples[0], &shared[lane], lane); vector_update(&shared[lane].point, &shared[lane].point);
    int result = forward(&shared[lane], &shared[lane]); __syncthreads();
    unsigned slot = lane * 8u; out[slot] = unsigned(shared[lane].point.x); out[slot + 1u] = unsigned(shared[lane].samples[0]);
    out[slot + 2u] = unsigned(shared[lane].samples[1]); out[slot + 3u] = unsigned(result);
    out[slot + 4u] = unsigned(previous_z(&input[lane + 1u])); out[slot + 5u] = unsigned(shared[lane].point.y);
    out[slot + 6u] = 100u + lane; storage_alias(&out[slot + 6u], &out[slot + 6u]); position(&out[slot + 7u]);
}"#, [4, 1, 1], &expected)
        .module_buffers(vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 40]), ShaderBuffer::new(1, wgpu::BufferBindingType::Storage { read_only: true }, &[1, 2, 3, 10, 20, 2, 3, 4, 11, 21, 3, 4, 5, 12, 22, 4, 5, 6, 13, 23, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL])], 0, [1, 1, 1])
}

fn own_function_pointer_lowering() -> ShaderTest {
    let mut expected = [SENTINEL; 16];
    expected[..8].copy_from_slice(&[11, 8, 8, 5, 10, 5, 15, 15]);
    module_case(
        "pointer lowering in a kernel without device helpers",
        r#"struct Record { int value; int samples[2]; };
__global__ void pointer_standalone(unsigned *out) {
    int values[4]{3, 5, 7, 11};
    *(&values[4] - 2) += 1;
    out[0] = unsigned(*(&values[4] - 1));
    out[1] = unsigned((1 + values)[1]);
    out[2] = unsigned(2[values]);
    int3 point{2, 4, 6};
    (*(&point)).x += 3;
    out[3] = unsigned((*(&point)).x);
    int &alias = values[1];
    *(&alias + 1) += 2;
    out[4] = unsigned(*(&alias + 1));
    out[5] = unsigned(values[1]);
    Record record{9, {12, 15}};
    (*(&record)).samples[0] += 3;
    out[6] = unsigned((*(&record)).samples[0]);
    out[7] = unsigned(*(&record.samples[2] - 1));
}
"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 16],
        )],
        0,
        [1, 1, 1],
    )
}
