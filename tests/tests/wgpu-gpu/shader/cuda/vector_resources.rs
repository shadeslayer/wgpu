use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    let mut cases = vec![float3_references()];
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            if scalar != "float" || width != 3 {
                cases.push(adjacent_elements(scalar, width));
            }
        }
    }
    cases
}

fn float3_references() -> ShaderTest {
    let values = [0.5_f32, -1.25, 2.5, -4.5, 5.25, -6.5, 8.5, -9.25, 10.5];
    let mut input = [0x0123_4567_u32; 32];
    for (word, value) in input.iter_mut().zip(values) {
        *word = value.to_bits();
    }
    let values = [
        -3.5_f32, 5.25, -6.5, 8.5, 5.25, -1.25, 9.5, -9.25, 1.25, 0.5, -3.5, 2.5, 1.25, 0.5, 2.0,
        -3.5, 5.25, -6.5, -1.25, 30.0, 7.0, -3.5, 8.5, 9.5, 9.0, 9.0, 10.5,
    ];
    let mut expected = [SENTINEL; 32];
    for (word, value) in expected.iter_mut().zip(values) {
        *word = value.to_bits();
    }
    module_case(
        "packed float3 resources, helper copies, conditional fields and compiler left-to-right snapshots",
        include_str!("../../../../../naga/tests/in/cuda/vector-resources.cu"),
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: true }, &input),
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

fn adjacent_elements(scalar: &str, width: usize) -> ShaderTest {
    let kind = format!("{scalar}{width}");
    let last = ["x", "y", "z", "w"][width - 1];
    let source = format!(
        "__device__ {kind} increment({kind} value) {{ value.x += 1; return value; }}
__global__ void resources(const {kind} *input, {kind} *output) {{
    for (int i = 0; i < 3; ++i) {{
        {kind} copied = input[i]; output[i] = increment(copied);
        output[i].{last} = input[2 - i].{last};
    }}
    output[3] = input[0];
    int index = 0;
    output[index++].x = (index = 1, input[1].x);
    output[3].y = index;
    bool choose_first = input[0].x < input[1].x;
    (choose_first ? output[0] : output[2]).y += 2;
}}"
    );
    let rows = match scalar {
        "float" => [
            [0.5_f32, -1.25, 2.5, 3.75].map(f32::to_bits),
            [-4.5_f32, 5.25, -6.5, 7.75].map(f32::to_bits),
            [8.5_f32, -9.25, 10.5, -11.75].map(f32::to_bits),
        ],
        "int" => [
            [-7_i32, 11, -13, 17].map(|value| value as u32),
            [19_i32, -23, 29, -31].map(|value| value as u32),
            [-37_i32, 41, -43, 47].map(|value| value as u32),
        ],
        "uint" => [
            [0xffff_ffff, 0x8000_0001, 0xf000_0003, 0x8000_0005],
            [0x8000_0011, 0xffff_fffe, 0xa000_0017, 0xb000_001d],
            [0x8000_0021, 0xc000_0023, 0xd000_0029, 0xe000_002f],
        ],
        _ => unreachable!(),
    };
    let mut input = [0x0123_4567_u32; 32];
    let mut expected = [SENTINEL; 32];
    for (index, row) in rows.iter().enumerate() {
        let base = index * width;
        input[base..base + width].copy_from_slice(&row[..width]);
        expected[base..base + width].copy_from_slice(&row[..width]);
        expected[base] = add(scalar, row[0], 1);
        expected[base + width - 1] = rows[2 - index][width - 1];
    }
    expected[3 * width..4 * width].copy_from_slice(&rows[0][..width]);
    expected[width] = rows[1][0];
    expected[3 * width + 1] = if scalar == "float" {
        2.0_f32.to_bits()
    } else {
        2
    };
    let choose_first = match scalar {
        "float" => f32::from_bits(rows[0][0]) < f32::from_bits(rows[1][0]),
        "int" => (rows[0][0] as i32) < rows[1][0] as i32,
        "uint" => rows[0][0] < rows[1][0],
        _ => unreachable!(),
    };
    let selected = if choose_first { 0 } else { 2 };
    expected[selected * width + 1] = add(scalar, expected[selected * width + 1], 2);
    module_case(
        &format!("adjacent {kind} storage elements and full tails"),
        &source,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
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

fn add(scalar: &str, bits: u32, amount: u32) -> u32 {
    if scalar == "float" {
        (f32::from_bits(bits) + amount as f32).to_bits()
    } else {
        bits.wrapping_add(amount)
    }
}
