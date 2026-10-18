use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;
const CANONICAL_NAN: u32 = 0x7fc0_0000;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        rounded_results(),
        conversions_and_side_effects(),
        selected_calls(),
    ]
}

fn expected_bits(input_bits: u32) -> u32 {
    let magnitude = input_bits & 0x7fff_ffff;
    if magnitude == 0 || magnitude == 0x7f80_0000 {
        input_bits
    } else if magnitude > 0x7f80_0000 {
        CANONICAL_NAN
    } else {
        f32::from_bits(input_bits).floor().to_bits()
    }
}

fn rounded_results() -> ShaderTest {
    let inputs = input_bits();
    assert_eq!(inputs.len(), 3037);
    let mut input_buffer = [0x0123_4567_u32; 4096];
    input_buffer[..inputs.len()].copy_from_slice(&inputs);
    let mut expected = [SENTINEL; 4096];
    for (output, &input) in expected.iter_mut().zip(&inputs) {
        *output = expected_bits(input);
    }
    module_case(
        "floorf both signs and all exponent bands, integer boundaries, subnormals and exceptional classes",
        "__global__ void floors(const unsigned *input, unsigned *out) {
            unsigned index = blockIdx.x * blockDim.x + threadIdx.x;
            if (index >= 3037u) return;
            out[index] = __float_as_uint(floorf(__uint_as_float(input[index])));
        }",
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: true }, &input_buffer),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 4096]),
        ],
        1,
        [64, 1, 1],
    )
}

fn conversions_and_side_effects() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..30].copy_from_slice(&[
        0x0000_0000,
        0x8000_0000,
        0x3f80_0000,
        0xc000_0000,
        0x4040_0000,
        0x40a0_0000,
        0x3f80_0000,
        0x0000_0000,
        0x0000_0000,
        0xbf80_0000,
        0x0000_0000,
        0xbf80_0000,
        0x7f7f_ffff,
        0xff7f_ffff,
        0x7f7f_ffff,
        0xff7f_ffff,
        0x7f80_0000,
        0xff80_0000,
        CANONICAL_NAN,
        CANONICAL_NAN,
        0x4000_0000,
        0x4070_0000,
        7,
        9,
        0x40a0_0000,
        0x4aff_fffe,
        0xcb00_0000,
        0x4f80_0000,
        0xcf00_0000,
        0x5015_02f9,
    ]);
    output_case(
        "floorf scalar conversions, aliases, ordinary large literals, overloads and argument-once updates",
        include_str!("../../../../../naga/tests/in/cuda/floorf.cu"),
        &expected,
    )
}

fn selected_calls() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..20].copy_from_slice(&[
        1.0_f32.to_bits(),
        2.75_f32.to_bits(),
        (-20.25_f32).to_bits(),
        0.75_f32.to_bits(),
        0.0_f32.to_bits(),
        1.75_f32.to_bits(),
        1.0_f32.to_bits(),
        2.75_f32.to_bits(),
        0,
        2.75_f32.to_bits(),
        1,
        2.75_f32.to_bits(),
        17,
        4.0_f32.to_bits(),
        0.0_f32.to_bits(),
        1.75_f32.to_bits(),
        1.0_f32.to_bits(),
        2.75_f32.to_bits(),
        1,
        3.75_f32.to_bits(),
    ]);
    output_case(
        "floorf compiler left-to-right argument snapshots, selected branches, comma sequencing and continue",
        "__device__ float identity(float value) { return value; }
        __device__ float pair(float first, float second) { return first * 10.0f + second; }
        __device__ int floorf(bool value) { return value ? 17 : 19; }
        __global__ void effects(unsigned *out) {
            float value = 1.75f;
            out[0] = __float_as_uint(floorf(value++));
            out[1] = __float_as_uint(value);
            value = -1.25f;
            out[2] = __float_as_uint(pair(floorf(value++), value++));
            out[3] = __float_as_uint(value);
            out[4] = __float_as_uint(floorf(identity(value++)));
            out[5] = __float_as_uint(value);
            bool choose = true;
            out[6] = __float_as_uint(choose ? floorf(value++) : floorf(value++));
            out[7] = __float_as_uint(value);
            bool run = false;
            out[8] = unsigned(run && floorf(value++));
            out[9] = __float_as_uint(value);
            run = true;
            out[10] = unsigned(run || floorf(value++));
            out[11] = __float_as_uint(value);
            out[12] = unsigned(floorf(true));
            out[13] = __float_as_uint(floorf(4.75f));
            value = -0.25f;
            out[14] = __float_as_uint((floorf(value++), floorf(value++)));
            out[15] = __float_as_uint(value);
            for (int index = 0; index < 2; ++index) {
                if (index == 0) continue;
                out[16] = __float_as_uint(floorf(value++));
            }
            out[17] = __float_as_uint(value);
            out[18] = unsigned(run && floorf(value++));
            out[19] = __float_as_uint(value);
        }",
        &expected,
    )
}

fn output_case(name: &str, source: &str, expected: &[u32; 32]) -> ShaderTest {
    module_case(name, source, [1, 1, 1], expected).module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 32],
        )],
        0,
        [1, 1, 1],
    )
}

fn input_bits() -> Vec<u32> {
    let mut inputs = Vec::new();
    let mut add = |bits| {
        if !inputs.contains(&bits) {
            inputs.push(bits);
        }
    };
    for bits in [
        0x0000_0000,
        0x8000_0000,
        0x7f80_0000,
        0xff80_0000,
        0x7fc0_0000,
        0xffc0_0000,
        0x7fc1_2345,
        0xffc1_2345,
        0x7f80_0001,
        0xff80_0001,
        0x7fa0_0000,
        0xffa0_0000,
    ] {
        add(bits);
    }
    for exponent in 1..255 {
        for fraction in [0, 1, 0x003f_ffff, 0x007f_fffe, 0x007f_ffff] {
            for sign in [0, 0x8000_0000] {
                add((exponent << 23) | fraction | sign);
            }
        }
    }
    for bits in 1..=16 {
        for sign in [0, 0x8000_0000] {
            add(bits | sign);
        }
    }
    for shift in 0..23 {
        for offset in [-1, 0, 1] {
            let bits = (1_i32 << shift) + offset;
            if bits > 0 && bits < 0x0080_0000 {
                for sign in [0, 0x8000_0000] {
                    add(bits as u32 | sign);
                }
            }
        }
    }
    for integer in [
        1, 2, 3, 7, 15, 31, 127, 255, 1023, 4095, 65535, 65536, 8388607, 8388608, 8388609,
        16777215, 16777216,
    ] {
        let center = (integer as f32).to_bits();
        for offset in [-1, 0, 1] {
            let bits = (i64::from(center) + offset) as u32;
            for sign in [0, 0x8000_0000] {
                add(bits | sign);
            }
        }
    }
    for center in [
        0x3e80_0000,
        0x3f00_0000,
        0x3f40_0000,
        0x3fa0_0000,
        0x3fc0_0000,
        0x3fe0_0000,
        0x4010_0000,
        0x4020_0000,
    ] {
        for offset in [-1, 0, 1] {
            let bits = (i64::from(center) + offset) as u32;
            for sign in [0, 0x8000_0000] {
                add(bits | sign);
            }
        }
    }
    let mut state = 0x51f1_5e5d_u32;
    for _ in 0..256 {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        if state & 0x7f80_0000 != 0x7f80_0000 {
            add(state);
        }
    }
    inputs
}
