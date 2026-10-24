use super::{case, module_case, ShaderBuffer, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        control_flow(),
        scopes_and_noncompound_bodies(),
        reconverged_shared_exchange(),
    ]
}

fn control_flow() -> ShaderTest {
    case(
        "CUDA switches preserve full fallthrough, nested jumps, helper returns, scoped call results and selector effects",
        include_str!("../../../../../naga/tests/in/cuda/switch.cu"),
        &[-1_i32; 32],
        &[
            111, 110, 100, 10000, 11000, 8, 8, 5, 7, 131, 121, 100, 900, 3028, 4030, 400,
            3, 6, 5, 19, 10, 1, 2, 1, 134, 82, 16, 6, 23, 7, -1, -1,
        ],
    )
}

fn scopes_and_noncompound_bodies() -> ShaderTest {
    case(
        "CUDA switch scope spans labels while direct noncompound bodies keep selector and entry semantics",
        r#"__device__ int single_default(int selector) {
    switch (selector) default: return 37;
}
__device__ int single_case(int selector) {
    switch (selector) case 0: return 5;
    return 9;
}
__global__ void switch_scopes(int *out) {
    int prelude_count = 0;
    switch (0) {
        ++prelude_count;
        int prelude;
        using Count = int;
        case 0:
            int value;
            int2 point;
            int matrix[2][2];
            prelude = 2;
            value = 3;
            point = make_int2(4, 5);
            matrix[1][1] = 6;
            out[1] = prelude;
            out[2] = value;
            out[3] = point.y;
            out[4] = matrix[1][1];
        case 1:
            Count later;
            later = 7;
            out[5] = later;
            {
                int value = 11;
                out[6] = value;
            }
            out[7] = value;
            break;
    }
    out[0] = prelude_count;
    int counter = 0;
    switch (counter++);
    out[8] = counter;
    switch (counter++) ++counter;
    out[9] = counter;
    switch (0) case 0: out[10] = 13;
    switch (1) case 0: case 1: out[11] = 17;
    switch (2) default: out[12] = 19;
    out[13] = single_default(8);
    out[14] = single_case(0);
    out[15] = single_case(8);
    int shadow = 23;
    switch (1) {
        case 0: {
            int shadow = 2;
            out[16] = shadow;
            break;
        }
        default: {
            int shadow = 29;
            out[16] = shadow;
            break;
        }
    }
    out[17] = shadow;
    switch (2) {
        case 0: {
            int initialized = 3;
        }
        case 2:
            out[18] = 31;
            break;
    }
    switch (0) {
        case 0:
            for (int iteration = 0; iteration < 2; ++iteration) {}
        case 1:
            out[19] = 41;
            break;
    }
    switch (false) case false: out[20] = 43;
    switch (0u) {
        case 0:
            out[21] = 47;
            break;
    }
    int unchanged = 53;
    switch (3) {
        case 0:
            unchanged = 99;
    }
    out[22] = unchanged;
    switch (1) {
        case 0:
            break;
            using Word = int;
        case 1:
            Word word;
            word = 61;
            out[23] = word;
    }
    switch (0x80000000u) {
        case 0x80000000u:
            out[24] = 67;
            break;
        default:
            out[24] = 99;
            break;
    }
    switch (0xffffffffu) {
        case static_cast<unsigned>(-1):
            out[25] = 71;
            break;
        default:
            out[25] = 99;
            break;
    }
    switch (true) {
        case true:
            out[26] = 73;
            break;
        case false:
            out[26] = 99;
            break;
    }
}"#,
        &[-1_i32; 32],
        &[
            0, 2, 3, 5, 6, 7, 11, 3, 1, 2, 13, 17, 19, 37, 5, 9, 29, 23, 31, 41, 43, 47,
            53, 61, 67, 71, 73, -1, -1, -1, -1, -1,
        ],
    )
}

fn reconverged_shared_exchange() -> ShaderTest {
    let mut expected = [-1_i32; 32];
    expected[0] = 22;
    expected[1] = 11;
    module_case(
        "CUDA barriers after a non-returning divergent switch preserve shared exchange",
        r#"__global__ void switch_shared(int *out) {
    __shared__ int values[2];
    unsigned index = threadIdx.x;
    values[index] = int(index + 1u) * 10;
    switch (index) {
        case 0u:
            values[0] += 1;
            break;
        case 1u:
            values[1] += 2;
            break;
    }
    __syncthreads();
    out[index] = values[1u - index];
}"#,
        [2, 1, 1],
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
