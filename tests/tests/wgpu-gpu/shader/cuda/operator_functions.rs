use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![canonical(), sequencing(), lazy_and_void()]
}

fn canonical() -> ShaderTest {
    let mut expected = [SENTINEL; 80];
    for lane in 0..2 {
        expected[lane * 36..lane * 36 + 36].copy_from_slice(&[
            12,
            (-5_i32) as u32,
            (-13_i32) as u32,
            0,
            125,
            5,
            15,
            3,
            36,
            2,
            1,
            0,
            7,
            15,
            96,
            1,
            5,
            9,
            125,
            0,
            62,
            12,
            21,
            192,
            5,
            6,
            9,
            10,
            27,
            5,
            9,
            2,
            (-9_i32) as u32,
            18,
            10,
            1,
        ]);
    }
    module_case(
        "all unary and binary operator functions preserve precedence, record copies and readonly vector inputs",
        include_str!("../../../../../naga/tests/in/cuda/operator-functions.cu"),
        [2, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: true }, &[12_i32, 5, 12, 5]),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 80]),
        ],
        1,
        [1, 1, 1],
    )
}

fn sequencing() -> ShaderTest {
    let mut expected = [SENTINEL; 19];
    expected[..11].copy_from_slice(&[101, 2, 307, 7, 307, 7, 10, 6, 7, 8, 3]);
    module_case(
        "shift notation snapshots scalar and record left operands before right effects and unary class values stay private",
        r#"
struct Box { int value; };
__device__ int operator<<(int left, Box right) { return left * 100 + right.value; }
__device__ int operator<<(Box left, int right) { return left.value * 100 + right; }
__device__ int operator>>(Box left, Box right) { return left.value * 100 + right.value; }
__device__ Box operator*(Box value) { ++value.value; return value; }
__device__ Box operator&(Box value) { value.value *= 2; return value; }
__global__ void sequencing(int *out) {
    int scalar = 1;
    int scalar_shift = scalar << Box{scalar++};
    Box first{3};
    int class_shift = first << (first.value = 7);
    Box second{3};
    int class_right_shift = second >> Box{second.value = 7};
    const Box frozen{3};
    Box addressed = &Box{5};
    Box dereferenced = *Box{5};
    Box first_nested = *&frozen;
    Box second_nested = &*frozen;
    out[0] = scalar_shift;
    out[1] = scalar;
    out[2] = class_shift;
    out[3] = first.value;
    out[4] = class_right_shift;
    out[5] = second.value;
    out[6] = addressed.value;
    out[7] = dereferenced.value;
    out[8] = first_nested.value;
    out[9] = second_nested.value;
    out[10] = frozen.value;
}
"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 19])],
        0,
        [1, 1, 1],
    )
}

fn lazy_and_void() -> ShaderTest {
    let mut expected = [SENTINEL; 38];
    expected[..16].copy_from_slice(&[
        2,
        4,
        1,
        3,
        6,
        9,
        2,
        4.5_f32.to_bits(),
        9,
        4,
        6,
        17,
        2.5_f32.to_bits(),
        7,
        23,
        5,
    ]);
    expected[24..30].copy_from_slice(&[1, 2, 3, 5, 6, 1]);
    module_case(
        "operator calls evaluate array operands once and preserve lazy branches, void results and scalar float results",
        r#"
struct Box { int value; };
__device__ void operator!(Box value) { value.value; }
__device__ void operator%(Box value, int right) { value.value += right; }
__device__ float operator~(Box value) { return float(value.value) + 0.5f; }
__device__ Box operator*(Box value) { ++value.value; return value; }
__device__ Box operator&(Box value) { value.value *= 2; return value; }
__device__ Box operator+(Box left, Box right) { left.value += right.value; return left; }
__global__ void lazy_and_void(unsigned *out) {
    int counter = 0;
    !Box{counter++};
    true ? !Box{counter++} : !Box{counter++};
    false && ((Box{counter++} % 1), true);
    true || ((!Box{counter++}), false);
    out[0] = counter;
    int index = 0;
    Box values[2]{Box{3}, Box{5}};
    Box first = *values[index++];
    out[1] = first.value;
    out[2] = index;
    out[3] = values[0].value;
    Box selected = &(true ? values[0] : values[1]);
    out[4] = selected.value;
    Box result = false ? *Box{counter++} : *Box{8};
    out[5] = result.value;
    out[6] = counter;
    out[7] = __float_as_uint(~Box{4});
    Box left = *Box{counter++};
    Box right = &Box{counter++};
    Box combined = left + right;
    out[8] = combined.value;
    out[9] = counter;
    for (int iteration = 0; iteration < 3; ++iteration) {
        if (iteration == 1) continue;
        !Box{counter++};
    }
    out[10] = counter;
    out[11] = (Box{0} % 2, 17);
    out[12] = __float_as_uint(operator~(Box{2}));
    operator!(Box{counter++});
    out[13] = counter;
    out[14] = (operator%(Box{1}, 3), 23);
    out[15] = values[1].value;
    using Alias = Box;
    int temporary_counter = 0;
    Box{temporary_counter++};
    out[24] = temporary_counter;
    Alias{temporary_counter++};
    out[25] = temporary_counter;
    if (true) Box{temporary_counter++};
    out[26] = temporary_counter;
    for (int iteration = 0; iteration < 2; ++iteration) Box{temporary_counter++};
    out[27] = temporary_counter;
    for (Alias{temporary_counter++}; false;) { ++temporary_counter; }
    out[28] = temporary_counter;
    int scalar = 0;
    int(scalar++);
    out[29] = scalar;
}
"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 38])],
        0,
        [1, 1, 1],
    )
}
