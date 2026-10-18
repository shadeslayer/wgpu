use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    let mut cases = vec![float3_phases()];
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            if scalar != "float" || width != 3 {
                cases.push(neighbor_and_broadcast(scalar, width));
            }
        }
    }
    cases
}

fn float3_phases() -> ShaderTest {
    let mut expected = vec![SENTINEL; 16 * 9 + 8];
    for group in 0..2 {
        let broadcast = group as f32 + 0.25;
        for lane in 0..8 {
            let next = (lane + 1) % 8;
            let after_next = (lane + 2) % 8;
            let neighbor = (group * 16 + next) as f32;
            let mut copied_y = neighbor * 2.0 + 3.75;
            if lane % 2 == 0 {
                copied_y += broadcast;
            }
            let mut shared_y = -neighbor - 1.25;
            if next % 2 != 0 {
                shared_y += broadcast;
            }
            let values = [
                neighbor + 2.5,
                copied_y,
                neighbor + 2.5,
                neighbor + 0.5,
                shared_y,
                (group * 16 + after_next) as f32 + 2.5,
                broadcast,
                group as f32 + 0.5,
                group as f32 + 0.75,
            ];
            let row = (group * 8 + lane) * 9;
            for (word, value) in expected[row..row + 9].iter_mut().zip(values) {
                *word = value.to_bits();
            }
        }
    }
    module_case(
        "float3 shared snapshot, selected fields, helper copies and three uniform phases",
        include_str!("../../../../../naga/tests/in/cuda/shared-vectors.cu"),
        [8, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &vec![SENTINEL; expected.len()],
        )],
        0,
        [2, 1, 1],
    )
}

fn neighbor_and_broadcast(scalar: &str, width: usize) -> ShaderTest {
    let kind = format!("{scalar}{width}");
    let arguments = match scalar {
        "float" => [
            "base + 0.5f",
            "-base - 1.25f",
            "base + 2.5f",
            "-base - 3.75f",
        ],
        "int" => ["base + 3", "-base - 5", "base + 7", "-base - 9"],
        "uint" => ["base + 3u", "base + 5u", "base + 7u", "base + 9u"],
        _ => unreachable!(),
    };
    let make = format!("make_{kind}({})", arguments[..width].join(", "));
    let scalar_type = if scalar == "uint" { "unsigned" } else { scalar };
    let base = if scalar == "uint" {
        "0x80000000u + group * 16u + lane"
    } else {
        "group * 16u + lane"
    };
    let broadcast_base = if scalar == "uint" {
        "0x80000000u + group * 16u + 8u"
    } else {
        "group * 16u + 8u"
    };
    let source = format!(
        "using Value = {kind};
__global__ void shared_values(Value *output) {{
    __shared__ Value values[8];
    __shared__ Value broadcast;
    unsigned lane = threadIdx.x;
    unsigned group = blockIdx.x;
    {scalar_type} base = {base};
    values[lane] = {make};
    if (lane == 0u) {{
        base = {broadcast_base}; broadcast = {make};
    }}
    __syncthreads();
    unsigned row = (group * 8u + lane) * 2u;
    output[row] = values[(lane + 1u) % 8u];
    output[row + 1u] = broadcast;
}}"
    );
    let mut expected = vec![SENTINEL; 35 * width];
    for group in 0..2 {
        for lane in 0..8 {
            let neighbor = components(scalar, group * 16 + (lane + 1) % 8);
            let broadcast = components(scalar, group * 16 + 8);
            let row = (group * 8 + lane) * 2 * width;
            expected[row..row + width].copy_from_slice(&neighbor[..width]);
            expected[row + width..row + width * 2].copy_from_slice(&broadcast[..width]);
        }
    }
    module_case(
        &format!("{kind} shared neighbor array and per-workgroup broadcast"),
        &source,
        [8, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &vec![SENTINEL; expected.len()],
        )],
        0,
        [2, 1, 1],
    )
}

fn components(scalar: &str, base: usize) -> [u32; 4] {
    match scalar {
        "float" => {
            let base = base as f32;
            [base + 0.5, -base - 1.25, base + 2.5, -base - 3.75].map(f32::to_bits)
        }
        "int" => {
            let base = base as i32;
            [base + 3, -base - 5, base + 7, -base - 9].map(|value| value as u32)
        }
        "uint" => {
            let base = 0x8000_0000 + base as u32;
            [base + 3, base + 5, base + 7, base + 9]
        }
        _ => unreachable!(),
    }
}
