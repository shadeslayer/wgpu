use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        declarations_and_bounds(),
        source_eligibility(),
        shared_bounds(),
    ]
}

fn declarations_and_bounds() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..30].copy_from_slice(&[
        2,
        0x8000_0000,
        0x3f00_0000,
        3,
        18,
        7,
        0,
        2,
        0x8000_0000,
        0xffff_fffe,
        0xffff_ffff,
        0,
        0,
        0x8000_0000,
        0xffff_ff00,
        1,
        4,
        6,
        5,
        0,
        1,
        5,
        7,
        0,
        0,
        0,
        0,
        24,
        3,
        12,
    ]);
    input_output_case(
        "scalar constant declarations, conversions, fixed bounds, loop resets and full tails",
        include_str!("../../../../../naga/tests/in/cuda/constant-expressions.cu"),
        &expected,
    )
}

fn source_eligibility() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..20].copy_from_slice(&[
        3,
        4,
        5,
        6,
        7,
        5,
        0,
        1,
        5,
        9,
        10,
        0,
        0,
        0,
        0,
        16_777_216,
        0xffff_fffd,
        7,
        7,
        12,
    ]);
    input_output_case(
        "constant eligibility, discarded addresses, skipped side effects, pending self names and runtime inputs",
        "constexpr int Global = 4;
        __device__ int forbidden() { return 99; }
        __global__ void eligibility(const int *input, unsigned *out) {
            int runtime = input[0], other = input[1];
            int local[2]{};
            constexpr int discarded = (runtime, 3);
            constexpr int selected_address = ((true ? runtime : other), 4);
            constexpr int array_address = (local[0], 5);
            constexpr int selected_array = ((true ? local[0] : input[0]), 6);
            out[0] = unsigned(discarded); out[1] = unsigned(selected_address);
            out[2] = unsigned(array_address); out[3] = unsigned(selected_array);
            constexpr int selected = true ? 7 : (runtime = 100);
            out[4] = unsigned(selected); out[5] = unsigned(runtime);
            constexpr bool no_call = false && forbidden();
            constexpr bool no_increment = true || runtime++;
            out[6] = unsigned(no_call); out[7] = unsigned(no_increment); out[8] = unsigned(runtime);
            constexpr int self = true ? 9 : self;
            constexpr int Global = true ? 10 : Global;
            out[9] = unsigned(self); out[10] = unsigned(Global);
            {
                constexpr int Inner = 2; const int Count = Inner + 1;
                for (int i = 0; i < 3; ++i) {
                    constexpr int Size = Count; int scratch[Size]{};
                    out[11 + i] = unsigned(scratch[0]); scratch[0] = runtime + i;
                }
            }
            const bool Ready = true; int true_count[Ready]{};
            out[14] = unsigned(true_count[0]);
            constexpr float rounded = 16777217;
            out[15] = unsigned(rounded);
            constexpr int negative = -3; out[16] = unsigned(negative);
            const int Count = 2;
            { int Count[Count]{}; Count[1] = other; out[17] = unsigned(Count[1]); }
            out[18] = unsigned(other); out[19] = unsigned(runtime + other);
        }",
        &expected,
    )
}

fn input_output_case(name: &str, source: &str, expected: &[u32; 32]) -> ShaderTest {
    let mut input = [0x0123_4567_i32; 32];
    input[..2].copy_from_slice(&[5, 7]);
    module_case(name, source, [1, 1, 1], expected).module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[SENTINEL; 32],
            ),
        ],
        1,
        [1, 1, 1],
    )
}

fn shared_bounds() -> ShaderTest {
    let mut input = [0x0123_4567_i32; 20];
    for (index, value) in input[..16].iter_mut().enumerate() {
        *value = index as i32 + 10;
    }
    let mut expected = [SENTINEL; 72];
    for group in 0..2 {
        for lane in 0..8 {
            let index = group * 8 + lane;
            let peer = group * 8 + (lane + 1) % 8;
            expected[index * 4..index * 4 + 4].copy_from_slice(&[
                input[peer] as u32,
                input[peer] as u32 + 1,
                input[index] as u32 + 2,
                8,
            ]);
        }
    }
    module_case(
        "constant shared scalar and packed-vector array bounds, neighboring lanes and complete tails",
        "constexpr int Width = 4 + 4;
        __global__ void shared_constants(const int *input, unsigned *out) {
            constexpr int Count = Width;
            __shared__ int values[Count]; __shared__ int3 vectors[Count];
            unsigned lane = threadIdx.x;
            unsigned index = blockIdx.x * unsigned(Count) + lane;
            values[lane] = input[index];
            vectors[lane] = make_int3(input[index], input[index] + 1, input[index] + 2);
            __syncthreads();
            unsigned peer = (lane + 1u) % unsigned(Count);
            out[index * 4u] = unsigned(values[peer]);
            out[index * 4u + 1u] = unsigned(vectors[peer].y);
            out[index * 4u + 2u] = unsigned(vectors[lane].z);
            out[index * 4u + 3u] = unsigned(Count);
        }", [8, 1, 1], &expected,
    ).module_buffers(
        vec![
            ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: true }, &input),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 72]),
        ], 1, [2, 1, 1],
    )
}
