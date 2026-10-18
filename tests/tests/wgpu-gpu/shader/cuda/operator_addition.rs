use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        all_vector_components(),
        precedence_and_effects(),
        mixed_vector_resources(),
    ]
}

fn all_vector_components() -> ShaderTest {
    let mut expected = Vec::new();
    for width in [2, 3, 4] {
        let left = [1.25_f32, -2.5, 3.75, -4.0];
        let right = [0.5_f32, 1.5, -2.0, 0.25];
        expected.extend(
            left[..width]
                .iter()
                .zip(&right)
                .map(|(left, right)| (left + right).to_bits()),
        );
        expected.extend(left[..width].iter().map(|value| (value + 2.0).to_bits()));
        expected.extend(right[..width].iter().map(|value| (2.0 + value).to_bits()));
    }
    for width in [2, 3, 4] {
        let left = [1_i32, -2, 3, -4];
        let right = [5_i32, 6, -7, 8];
        expected.extend(
            left[..width]
                .iter()
                .zip(&right)
                .map(|(left, right)| (left + right) as u32),
        );
        expected.extend(left[..width].iter().map(|value| (value + 3) as u32));
        expected.extend(right[..width].iter().map(|value| (3 + value) as u32));
    }
    for width in [2, 3, 4] {
        let left = [0xffff_fffe_u32, 0x8000_0001, 3, 0xffff_ffff];
        let right = [5_u32, 2, 0xffff_fffd, 1];
        expected.extend(
            left[..width]
                .iter()
                .zip(&right)
                .map(|(left, right)| left.wrapping_add(*right)),
        );
        expected.extend(left[..width].iter().map(|value| value.wrapping_add(4)));
        expected.extend(
            right[..width]
                .iter()
                .map(|value| 4_u32.wrapping_add(*value)),
        );
    }
    assert_eq!(expected.len(), 81);
    expected.resize(96, SENTINEL);
    module_case(
        "all components of float int uint widths two three four in all three operator addition operand forms",
        include_str!("../../../../../naga/tests/in/cuda/operator-addition.cu"),
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 96])],
        0,
        [1, 1, 1],
    )
}

fn precedence_and_effects() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..30].copy_from_slice(&[
        13, 24, 131, 41, 7, 61, 1, 4, 12, 3, 3, 3, 31, 0, 1, 42, 31, 33, 101, 7, 4, 5, 6, 20, 15,
        24, 2, 1, 3, 6,
    ]);
    module_case(
        "operator addition precedence parameter copies void results lazy branches and explicit call left to right snapshots",
        "__device__ int2 operator+(int2 left, int2 right) {
            return make_int2(left.x * 10 + right.x, left.y * 10 + right.y);
        }
        __device__ int2 operator+(int2 left, int right) {
            left.x += right; left.y += right; return left;
        }
        __device__ int2 operator+(int left, int2 right) {
            return make_int2(left * 10 + right.x, left * 10 + right.y);
        }
        __device__ int operator+(int2 left, bool right) { return left.x + (right ? 100 : 0); }
        __device__ void operator+(uint2 left, unsigned right) {}
        __device__ void consume(uint2 left, unsigned right) { return operator+(left, right); }
        __device__ int2 shifted(int2 value) { return value + 2; }
        __global__ void effects(unsigned *out) {
            const int2 left = make_int2(1, 2), right = make_int2(3, 4);
            int2 combined = left + right;
            out[0] = unsigned(combined.x);
            out[1] = unsigned(combined.y);
            out[2] = unsigned((left + right + left).x);
            out[3] = unsigned((left + (right + left)).x);
            out[4] = unsigned((left + 2 * 3).x);
            out[5] = unsigned((2 * 3 + left).x);
            out[6] = unsigned(left.x);
            out[7] = unsigned((left + 2).y);
            int counter = 1;
            out[8] = unsigned(operator+(counter++, make_int2(counter++, 9)).x);
            out[9] = unsigned(counter);
            counter = 1;
            out[10] = unsigned(operator+(make_int2(counter++, 5), counter++).x);
            out[11] = unsigned(counter);
            bool choose = false;
            out[12] = unsigned((choose ? (left + counter++) : (right + left)).x);
            bool run = false;
            out[13] = unsigned(run && (left + counter++).x);
            run = true;
            out[14] = unsigned(run || (left + counter++).x);
            out[15] = unsigned((left + right, right + left).y);
            int2 values[2]{left, right};
            int total = 0;
            for (int index = 0; index < 2; ++index) {
                if (index == 0) continue;
                total += (values[index] + left).x;
            }
            out[16] = unsigned(total);
            out[17] = unsigned((shifted(left) + right).x);
            out[18] = unsigned(left + true);
            out[19] = unsigned(operator+(left, false) + 2 * 3);
            operator+(make_uint2(1u, 2u), unsigned(counter++));
            out[20] = unsigned(counter);
            make_uint2(1u, 2u) + unsigned(counter++);
            out[21] = unsigned(counter);
            consume(make_uint2(1u, 2u), unsigned(counter++));
            out[22] = unsigned(counter);
            out[23] = unsigned(1 + 2 * 3 + (left + right).x);
            out[24] = unsigned((left + right).x + (left + 1).x);
            out[25] = unsigned((left + right).y);
            int iterations = 0;
            for (int index = 0; index < 2; left + right) { ++index; ++iterations; }
            out[26] = unsigned(iterations);
            out[27] = unsigned(left.x);
            out[28] = unsigned(right.x);
            out[29] = unsigned(counter);
        }",
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 32])],
        0,
        [1, 1, 1],
    )
}

fn mixed_vector_resources() -> ShaderTest {
    let left: Vec<f32> = (0..128)
        .flat_map(|index| {
            [
                index as f32 / 4.0,
                (index % 7) as f32 / 2.0 - 1.5,
                -(index as f32) / 8.0,
            ]
        })
        .collect();
    let right = [2.0_f32, -0.5, 0.25].repeat(128);
    let mut expected = [SENTINEL; 384];
    for index in 0..70 {
        expected[index * 3..index * 3 + 3].copy_from_slice(&[
            (left[index * 3] + 2.5).to_bits(),
            (left[index * 3 + 1] + (index % 64) as f32).to_bits(),
            (left[index * 3 + 2] + 0.75).to_bits(),
        ]);
    }
    module_case(
        "operator addition of packed vector resources helper builtin reads and source parameter bindings",
        "__device__ float3 operator+(float3 left, float3 right) {
            left.x += right.x;
            left.y = left.y + right.y + float(threadIdx.x);
            left.z += right.z;
            return left;
        }
        __host__ __device__ float3 operator+(float3 left, float right) {
            return make_float3(left.x + right, left.y + right, left.z + right);
        }
        __device__ unsigned position() { return blockIdx.x * blockDim.x + threadIdx.x; }
        __global__ void resources(const float3 *left, float bias, const float3 *right, unsigned count, float3 *out) {
            unsigned index = position();
            if (index >= count) return;
            out[index] = (left[index] + right[index]) + bias;
        }",
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: true }, &left),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Uniform, &[0.5_f32.to_bits(), 0, 0, 0]),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Storage { read_only: true }, &right),
            ShaderBuffer::new(3, wgpu::BufferBindingType::Uniform, &[70_u32, 0, 0, 0]),
            ShaderBuffer::new(4, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 384]),
        ],
        4,
        [2, 1, 1],
    )
}
