use super::{case, module_case, ShaderBuffer, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![local_initialization(), index_effects(), shared_exchange()]
}

fn local_initialization() -> ShaderTest {
    let expected = [
        2, 5, 0, 3, 5, 0, 4, 5, 0, 2, 5, 0, 3, 5, 0, 4, 5, 0, 2, 5, 0, 3, 5, 0, 4, 5, 0, 9, 2, 3,
        -1, -1,
    ];
    module_case(
        "all nine nested vector arrays preserve copies, elision, zero fields and packed shared rows",
        include_str!("../../../../../naga/tests/in/cuda/multidimensional-arrays.cu"),
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[-1_i32; 32],
        )],
        0,
        [1, 1, 1],
    )
}

fn index_effects() -> ShaderTest {
    case(
        "nested lists zero rows before later effects and index each dimension once after assignment values",
        r#"using Count = int;
__device__ int next(int value) {
    return value + 10;
}
__global__ void multidimensional_effects(int *out) {
    int zeroed[2][2]{{1}, {zeroed[0][1] + 2, 3}};
    out[0] = zeroed[0][0];
    out[1] = zeroed[0][1];
    out[2] = zeroed[1][0];
    out[3] = zeroed[1][1];
    int changed[2][2]{{1}, {(changed[0][1] = 9, 2), 3}};
    out[4] = changed[0][1];
    out[5] = changed[1][0];
    int self[2][2]{{1, (self[0][0] = 7, 2)}, {self[0][0], 4}};
    out[6] = self[0][0];
    out[7] = self[1][0];
    int inferred[][2]{{1}, 2, 3, 4};
    out[8] = inferred[0][1];
    out[9] = inferred[1][1];
    out[10] = inferred[2][0];
    out[11] = inferred[2][1];
    unsigned counts[2][2]{{3u}, {5u, 6u}};
    out[12] = int(counts[0][0] + counts[1][0] + counts[1][1]);
    float samples[2][2]{{1.5f}, {2.5f, 3.5f}};
    out[13] = int(samples[0][0] + samples[1][0] + samples[1][1]);
    bool flags[2][2]{{true}, {false, true}};
    out[14] = int(flags[0][0]) + int(flags[0][1]) + int(flags[1][1]);
    int row = 0;
    int column = 0;
    int matrix[3][3]{};
    matrix[row++][column++] = row + column + 5;
    out[15] = matrix[0][0];
    out[16] = row * 10 + column;
    matrix[row++][column++] += row + column;
    out[17] = matrix[1][1];
    out[18] = row * 10 + column;
    matrix[row++][0] = row;
    out[19] = matrix[2][0];
    int counter = 0;
    int called[2][2]{{next(counter++), next(counter++)}, {next(counter++), next(counter++)}};
    out[20] = called[0][0] + called[0][1] + called[1][0] + called[1][1];
    out[21] = counter;
    ++(counter ? matrix[0][0] : matrix[2][0]);
    out[22] = matrix[0][0];
    int total = 0;
    for (int iteration = 0; iteration < 3; ++iteration) {
        int reset[2][2]{{iteration}, {}};
        total += reset[0][1] + reset[1][1];
        reset[1][1] = 9;
        if (iteration == 1) continue;
        total += reset[0][0];
    }
    out[23] = total;
    constexpr Count Width = (called[1][1], 2);
    int scratch[Width][2]{};
    out[24] = scratch[1][1];
    float pending[][2]{{1.0f, (pending[0][1], 2)}};
    out[25] = int(pending[0][1]);
    int3 points[2][2]{{{1, (points[0][0].x = 9, 2), points[0][0].x}}, {{3}}};
    out[26] = points[0][0].x;
    out[27] = points[0][0].z;
    int3 copies[2][2]{{points[0][0]}, {points[1][0]}};
    points[0][0].x = 100;
    out[28] = copies[0][0].x;
    out[29] = copies[1][0].x;
}"#,
        &[0; 30],
        &[
            1, 0, 2, 3, 9, 2, 7, 7, 0, 3, 4, 0, 14, 7, 2, 5, 11, 2, 22, 2, 46, 4, 6, 2, 0, 2,
            9, 9, 9, 3,
        ],
    )
}

fn shared_exchange() -> ShaderTest {
    let mut expected = [-1_i32; 32];
    for group in 0..2 {
        for column in 0..3 {
            for row in 0..2 {
                let peer = group * 100 + (1 - row) * 10 + (column + 1) % 3;
                let output = (group * 6 + column * 2 + row) * 2;
                expected[output] = peer as i32;
                expected[output + 1] = (3 * peer + 1) as i32;
            }
        }
    }
    module_case(
        "two non-square workgroups exchange separate nested integer planes and packed float3 rows",
        r#"__global__ void multidimensional_shared(int *out) {
    __shared__ int planes[2][2][3];
    __shared__ float3 points[2][3];
    unsigned row = threadIdx.x;
    unsigned column = threadIdx.y;
    unsigned group = blockIdx.x;
    unsigned plane = group % 2u;
    int value = int(group * 100u + row * 10u + column);
    planes[plane][row][column] = value;
    points[row][column] = make_float3(float(value) + 0.25f, float(value) + 0.5f, float(value) + 0.75f);
    __syncthreads();
    unsigned peer_row = 1u - row;
    unsigned peer_column = (column + 1u) % 3u;
    unsigned output = (group * 6u + column * 2u + row) * 2u;
    out[output] = planes[plane][peer_row][peer_column];
    float3 copied = points[peer_row][peer_column];
    out[output + 1u] = int(copied.x + copied.y + copied.z);
}"#,
        [2, 3, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[-1_i32; 32],
        )],
        0,
        [2, 1, 1],
    )
}
