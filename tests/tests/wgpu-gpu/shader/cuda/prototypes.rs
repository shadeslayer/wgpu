use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![canonical(), late_builtins(), builtin_overloads()]
}

fn canonical() -> ShaderTest {
    let mut expected = vec![SENTINEL; 40];
    for lane in 0..2 {
        expected[lane * 16..lane * 16 + 16].copy_from_slice(&[
            13,
            23,
            38,
            1,
            3,
            5,
            9,
            6,
            4,
            12,
            23,
            5,
            3.0_f32.to_bits(),
            42,
            lane as u32,
            42,
        ]);
    }
    module_case(
        "prototypes retain overload binding, record copies, call sequencing and generated helper dependencies",
        include_str!("../../../../../naga/tests/in/cuda/prototypes.cu"),
        [2, 1, 1],
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

const LATE_BUILTINS: &str = r#"
__device__ unsigned coordinate(unsigned);
__global__ void late_builtins(unsigned *out) {
    unsigned x = coordinate(3u) * coordinate(6u) + coordinate(0u);
    unsigned y = coordinate(4u) * coordinate(7u) + coordinate(1u);
    unsigned z = coordinate(5u) * coordinate(8u) + coordinate(2u);
    unsigned width = coordinate(6u) * coordinate(9u);
    unsigned height = coordinate(7u) * coordinate(10u);
    unsigned base = ((z * height + y) * width + x) * 12u;
    for (unsigned component = 0u; component < 12u; ++component) {
        out[base + component] = coordinate(component);
    }
}
__device__ unsigned coordinate(unsigned component) {
    switch (component) {
        case 0u: return threadIdx.x;
        case 1u: return threadIdx.y;
        case 2u: return threadIdx.z;
        case 3u: return blockIdx.x;
        case 4u: return blockIdx.y;
        case 5u: return blockIdx.z;
        case 6u: return blockDim.x;
        case 7u: return blockDim.y;
        case 8u: return blockDim.z;
        case 9u: return gridDim.x;
        case 10u: return gridDim.y;
        default: return gridDim.z;
    }
}
"#;

fn late_builtins() -> ShaderTest {
    let mut expected = vec![SENTINEL; 784];
    for z in 0..4 {
        for y in 0..4 {
            for x in 0..4 {
                let base = ((z * 4 + y) * 4 + x) * 12;
                expected[base..base + 12].copy_from_slice(&[
                    (x % 2) as u32,
                    (y % 2) as u32,
                    (z % 2) as u32,
                    (x / 2) as u32,
                    (y / 2) as u32,
                    (z / 2) as u32,
                    2,
                    2,
                    2,
                    2,
                    2,
                    2,
                ]);
            }
        }
    }
    module_case(
        "late helper definitions initialize every invocation, group, size and dispatch coordinate",
        LATE_BUILTINS,
        [2, 2, 2],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 784],
        )],
        0,
        [2, 2, 2],
    )
}

fn builtin_overloads() -> ShaderTest {
    let mut expected = [SENTINEL; 20];
    expected[..12].copy_from_slice(&[
        5,
        4,
        103,
        203,
        12,
        22,
        2.0_f32.to_bits(),
        2.0_f32.to_bits(),
        2.0_f32.to_bits(),
        0,
        1.0_f32.to_bits(),
        10,
    ]);
    module_case(
        "ordinary forward overloads coexist with declared intrinsics and const parameter copies",
        r#"
using Whole = int;
__device__ int advance(const Whole);
__device__ int fabs(int);
__device__ int floorf(int);
__device__ float fabs(float);
__device__ float floorf(float);
__device__ float sqrtf(float);
__device__ float sinf(float);
__device__ float cosf(float);
__device__ int choose(float);
__device__ int earlier(int value) { return choose(value); }
__device__ int choose(int);
__global__ void builtin_overloads(unsigned *out) {
    int value = 4;
    out[0] = advance(value);
    out[1] = value;
    out[2] = fabs(-3);
    out[3] = floorf(3);
    out[4] = earlier(2);
    out[5] = choose(2);
    out[6] = __float_as_uint(fabs(-2.0f));
    out[7] = __float_as_uint(floorf(2.5f));
    out[8] = __float_as_uint(sqrtf(4.0f));
    out[9] = __float_as_uint(sinf(0.0f));
    out[10] = __float_as_uint(cosf(0.0f));
    int advanced = advance(value++);
    out[11] = advanced + value;
}
__device__ int advance(Whole value) { return ++value; }
__device__ int fabs(int value) { return 100 - value; }
__device__ int floorf(int value) { return 200 + value; }
__device__ int choose(float value) { return 10 + int(value); }
__device__ int choose(int value) { return 20 + value; }
"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 20],
        )],
        0,
        [1, 1, 1],
    )
}
