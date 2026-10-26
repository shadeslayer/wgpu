use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![local_projections_and_aliases(), shared_and_storage_roots()]
}

fn local_projections_and_aliases() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        2, 5, 17, 18, 20, 58, 17, 23, 75, 20, 16, 10, 10, 61, 20, 98, 5, 1, 23, 24, 2, 27, 45, 79,
        103, 111, 28, 62, 5, 16, 37, 2,
    ]);
    module_case(
        "arrow projections, offsets, captured operands and live aliases",
        include_str!("../../../../../naga/tests/in/cuda/arrow-members.cu"),
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

fn shared_and_storage_roots() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        2, 12, 14, 4, 3, 6, 4, 20, 4, 15, 19, 5, 4, 8, 6, 21, 6, 18, 24, 6, 5, 10, 8, 22, 8, 21,
        29, 7, 6, 12, 2, 23,
    ]);
    module_case(
        "arrow fields across packed readonly and shared storage",
        r#"struct Sample { int3 point; int counts[2]; };
__device__ void update(Sample *value, const Sample *view, unsigned lane) {
    value->point.x += int(lane) + 1;
    value->counts[0] += view->point.x;
    value->counts[1] = value->point.x + value->counts[0];
}
__device__ void bump(int &value) { value += 2; }
__device__ int forward(Sample *value) {
    bump(value->point.y);
    return value->counts[1];
}
__device__ int previous_z(const Sample *value) { return (value - 1)->point.z; }
__global__ void arrow_spaces(unsigned *out, const Sample *input) {
    unsigned lane = threadIdx.x;
    __shared__ Sample shared[4];
    shared[lane] = input[lane];
    __syncthreads();
    update(&shared[lane], &shared[lane], lane);
    int result = forward(&shared[lane]);
    (&shared[lane].point)->z += int(lane) + 3;
    __syncthreads();
    unsigned slot = lane * 8u;
    out[slot] = unsigned((&shared[lane])->point.x);
    out[slot + 1u] = unsigned((&shared[lane])->counts[0]);
    out[slot + 2u] = unsigned(result);
    out[slot + 3u] = unsigned((&shared[lane])->point.y);
    out[slot + 4u] = unsigned(previous_z(&input[lane + 1u]));
    out[slot + 5u] = unsigned((&shared[lane].point)->z);
    out[slot + 6u] = unsigned((&shared[(lane + 1u) % 4u])->point.x);
    out[slot + 7u] = unsigned((&input[lane])->counts[1]);
}
"#,
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
                &[
                    1, 2, 3, 10, 20, 2, 3, 4, 11, 21, 3, 4, 5, 12, 22, 4, 5, 6, 13, 23, SENTINEL,
                    SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL, SENTINEL,
                ],
            ),
        ],
        0,
        [1, 1, 1],
    )
}
