use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![canonical(), sequencing(), lazy_and_void()]
}

fn canonical() -> ShaderTest {
    let mut expected = [SENTINEL; 56];
    for lane in 0..2 {
        expected[lane * 24..lane * 24 + 24].copy_from_slice(&[
            25,
            10,
            205,
            10,
            2,
            4,
            29,
            5,
            25,
            10,
            24,
            305,
            242,
            353,
            2042,
            21,
            25,
            200,
            1,
            0,
            1,
            1,
            1,
            lane as u32,
        ]);
    }
    module_case(
        "logical value overloads preserve record copies, vector results, precedence and builtin scalar logic",
        include_str!("../../../../../naga/tests/in/cuda/logical-operators.cu"),
        [2, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 56])],
        0,
        [1, 1, 1],
    )
}

fn sequencing() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        1, 1, 1002, 2, 203, 7, 2003, 7, 203, 7, 2003, 7, 5, 1, 1005, 0, 509, 5, 307, 7, 3008, 1, 8,
        102, 3004, 203, 4, 4005, 6, 0, 1, 6,
    ]);
    module_case(
        "eager logical calls snapshot original scalar and class operands before right effects and evaluate each operand once",
        r#"
struct Box { int value; };
__device__ int operator&&(Box left, Box right) { return left.value * 100 + right.value; }
__device__ int operator&&(int left, Box right) { return left * 100 + right.value; }
__device__ int operator&&(Box left, int right) { return left.value * 100 + right; }
__device__ int operator||(Box left, Box right) { return left.value * 1000 + right.value; }
__device__ int operator||(int left, Box right) { return left * 1000 + right.value; }
__device__ int operator||(Box left, int right) { return left.value * 1000 + right; }
__global__ void sequencing(unsigned *out) {
    int counter = 0;
    out[0] = false && Box{++counter};
    out[1] = counter;
    out[2] = true || Box{++counter};
    out[3] = counter;
    int scalar = 2;
    out[4] = scalar && (scalar = 7, Box{3});
    out[5] = scalar;
    scalar = 2;
    out[6] = scalar || (scalar = 7, Box{3});
    out[7] = scalar;
    Box first{2};
    out[8] = first && (first.value = 7, Box{3});
    out[9] = first.value;
    first.value = 2;
    out[10] = first || (first.value = 7, Box{3});
    out[11] = first.value;
    bool false_left = false;
    out[12] = false_left && (false_left = true, Box{5});
    out[13] = false_left;
    bool true_left = true;
    out[14] = true_left || (true_left = false, Box{5});
    out[15] = true_left;
    const Box frozen{5};
    out[16] = frozen && 9;
    out[17] = frozen.value;
    Box chosen{3}, other{6};
    bool choose = true;
    out[18] = (choose ? chosen : other) && (chosen.value = 7);
    out[19] = chosen.value;
    int index = 0;
    Box values[2]{Box{3}, Box{4}};
    out[20] = values[index++] || (values[0].value = 8);
    out[21] = index;
    out[22] = values[0].value;
    out[23] = operator&&(Box{1}, 2);
    out[24] = operator||(3, Box{4});
    out[25] = Box{counter++} && Box{counter++};
    out[26] = counter;
    out[27] = Box{counter++} || Box{counter++};
    out[28] = counter;
    out[29] = false && counter++;
    out[30] = true || counter++;
    out[31] = counter;
}
"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 40])],
        0,
        [1, 1, 1],
    )
}

fn lazy_and_void() -> ShaderTest {
    let mut expected = [SENTINEL; 30];
    expected[..22].copy_from_slice(&[
        2, 4, 6, 8, 8, 1, 10, 1, 12, 17, 14, 18, 20, 0, 1, 20, 0, 0, 1, 0, 1, 1,
    ]);
    module_case(
        "void logical calls stay eager while enclosing builtin scalar operators keep their short circuit",
        r#"
struct Box { int value; };
__device__ void operator&&(Box left, Box right) { left.value += right.value; }
__device__ void operator||(Box left, Box right) { left.value += right.value; }
__device__ int operator&&(int2 left, int2 right) { return left.x * 10 + right.x; }
__device__ int operator||(int2 left, int2 right) { return left.x * 100 + right.x; }
__global__ void lazy_and_void(unsigned *out) {
    int counter = 0;
    Box{counter++} && Box{counter++};
    out[0] = counter;
    Box{counter++} || Box{counter++};
    out[1] = counter;
    true ? (Box{counter++} && Box{counter++}) : (Box{counter++} || Box{counter++});
    out[2] = counter;
    false ? (Box{counter++} && Box{counter++}) : (Box{counter++} || Box{counter++});
    out[3] = counter;
    false && (make_int2(counter++, 0) && make_int2(counter++, 0));
    true || (make_int2(counter++, 0) || make_int2(counter++, 0));
    out[4] = counter;
    out[5] = true && (make_int2(counter++, 0) && make_int2(counter++, 0));
    out[6] = counter;
    out[7] = false || (make_int2(counter++, 0) || make_int2(counter++, 0));
    out[8] = counter;
    out[9] = (Box{counter++} && Box{counter++}, 17);
    out[10] = counter;
    for (int iteration = 0; iteration < 3; ++iteration) {
        if (iteration == 1) continue;
        Box{counter++} || Box{counter++};
    }
    out[11] = counter;
    operator&&(Box{counter++}, Box{counter++});
    out[12] = counter;
    out[13] = false && (counter = 99);
    out[14] = true || (counter = 99);
    out[15] = counter;
    int index = 0;
    Box values[2]{Box{4}, Box{5}};
    bool choose = true;
    out[16] = false && (choose ? values[index++].value : values[index++].value);
    out[17] = index;
    out[18] = true || (choose ? values[index++].value : values[index++].value);
    out[19] = index;
    out[20] = true && (choose ? values[index++].value : values[index++].value);
    out[21] = index;
}
"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 30])],
        0,
        [1, 1, 1],
    )
}
