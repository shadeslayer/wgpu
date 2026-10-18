use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        coordinate_fixture(),
        three_dimensional_coordinates(),
        mixed_resources_and_early_return(),
    ]
}

fn coordinate_fixture() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..30].copy_from_slice(&[
        0,
        0,
        0,
        0,
        0,
        0,
        1,
        1,
        1,
        1,
        1,
        1,
        0,
        0,
        2.5_f32.to_bits(),
        0,
        0,
        1,
        1,
        7,
        2.0_f32.to_bits(),
        1.25_f32.to_bits(),
        (-2.5_f32).to_bits(),
        4.0_f32.to_bits(),
        1,
        2,
        0,
        15.25_f32.to_bits(),
        11,
        1,
    ]);
    module_case(
        "helper coordinate components, overloads, direct reads and mixed source parameter bindings",
        include_str!("../../../../../naga/tests/in/cuda/helper-builtins.cu"),
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: true },
                &[7_i32],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Uniform,
                &[2.0_f32.to_bits(), 0, 0, 0],
            ),
            ShaderBuffer::new(
                2,
                wgpu::BufferBindingType::Storage { read_only: true },
                &[1.25_f32, -2.5, 4.0],
            ),
            ShaderBuffer::new(3, wgpu::BufferBindingType::Uniform, &[1_u32, 0, 0, 0]),
            ShaderBuffer::new(
                4,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[SENTINEL; 32],
            ),
        ],
        4,
        [1, 1, 1],
    )
}

fn three_dimensional_coordinates() -> ShaderTest {
    let mut expected = [SENTINEL; 2048];
    for global_z in 0..4 {
        for global_y in 0..6 {
            for global_x in 0..6 {
                let index = (global_z * 6 + global_y) * 6 + global_x;
                expected[index * 12..index * 12 + 12].copy_from_slice(&[
                    (global_x % 2) as u32,
                    (global_y % 3) as u32,
                    (global_z % 2) as u32,
                    (global_x / 2) as u32,
                    (global_y / 3) as u32,
                    (global_z / 2) as u32,
                    2,
                    3,
                    2,
                    3,
                    2,
                    2,
                ]);
            }
        }
    }
    module_case(
        "all helper compute components in three dimensional workgroups and dispatches",
        "__device__ unsigned local_component(unsigned axis) {
            if (axis == 0u) return threadIdx.x;
            if (axis == 1u) return threadIdx.y;
            return threadIdx.z;
        }
        __device__ unsigned group_component(unsigned axis) {
            if (axis == 0u) return blockIdx.x;
            if (axis == 1u) return blockIdx.y;
            return blockIdx.z;
        }
        __device__ unsigned shape_component(unsigned axis) {
            if (axis == 0u) return blockDim.x;
            if (axis == 1u) return blockDim.y;
            return blockDim.z;
        }
        __device__ unsigned grid_component(unsigned axis) {
            if (axis == 0u) return gridDim.x;
            if (axis == 1u) return gridDim.y;
            return gridDim.z;
        }
        __device__ unsigned position(unsigned axis) {
            return group_component(axis) * shape_component(axis) + local_component(axis);
        }
        __device__ unsigned linear_index() {
            unsigned width = grid_component(0u) * shape_component(0u);
            unsigned height = grid_component(1u) * shape_component(1u);
            return (position(2u) * height + position(1u)) * width + position(0u);
        }
        __global__ void coordinates(unsigned *out) {
            unsigned offset = linear_index() * 12u;
            out[offset] = local_component(0u);
            out[offset + 1u] = local_component(1u);
            out[offset + 2u] = local_component(2u);
            out[offset + 3u] = group_component(0u);
            out[offset + 4u] = group_component(1u);
            out[offset + 5u] = group_component(2u);
            out[offset + 6u] = shape_component(0u);
            out[offset + 7u] = shape_component(1u);
            out[offset + 8u] = shape_component(2u);
            out[offset + 9u] = grid_component(0u);
            out[offset + 10u] = grid_component(1u);
            out[offset + 11u] = grid_component(2u);
        }",
        [2, 3, 2],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 2048],
        )],
        0,
        [3, 2, 2],
    )
}

fn mixed_resources_and_early_return() -> ShaderTest {
    let input: Vec<i32> = (0..128).map(|index| index % 11 - 5).collect();
    let points: Vec<f32> = (0..128)
        .flat_map(|index| {
            [
                index as f32 / 4.0,
                (index % 7) as f32 / 2.0 - 1.5,
                -(index as f32) / 8.0,
            ]
        })
        .collect();
    let mut expected = [SENTINEL; 384];
    for index in 0..70 {
        expected[index * 3..index * 3 + 3].copy_from_slice(&[
            (input[index] as f32 * 2.0 + points[index * 3]).to_bits(),
            (points[index * 3 + 1] + (index % 12 % 4) as f32).to_bits(),
            (points[index * 3 + 2] + (index / 48) as f32).to_bits(),
        ]);
    }
    module_case(
        "helper input initialization before early return with mixed scalar and packed vector resource bindings",
        "__device__ unsigned thread_x() { return threadIdx.x; }
        __device__ unsigned group_z() { return blockIdx.z; }
        __device__ unsigned width() { return gridDim.x * blockDim.x; }
        __device__ unsigned height() { return gridDim.y * blockDim.y; }
        __host__ __device__ unsigned linear_index() {
            unsigned global_x = blockIdx.x * blockDim.x + thread_x();
            unsigned global_y = blockIdx.y * blockDim.y + threadIdx.y;
            unsigned global_z = group_z() * blockDim.z + threadIdx.z;
            return (global_z * height() + global_y) * width() + global_x;
        }
        __device__ float add_lane(float value) { return value + float(thread_x()); }
        __global__ void resources(const int *input, float scale, const float3 *points, unsigned limit, float3 *out) {
            unsigned index = linear_index();
            if (index >= limit) return;
            float3 point = points[index];
            out[index] = make_float3(float(input[index]) * scale + point.x, add_lane(point.y), point.z + float(group_z()));
        }",
        [4, 2, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: true }, &input),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Uniform, &[2.0_f32.to_bits(), 0, 0, 0]),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Storage { read_only: true }, &points),
            ShaderBuffer::new(3, wgpu::BufferBindingType::Uniform, &[70_u32, 0, 0, 0]),
            ShaderBuffer::new(4, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 384]),
        ],
        4,
        [3, 2, 2],
    )
}
