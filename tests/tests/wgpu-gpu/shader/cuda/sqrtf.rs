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
    if input_bits & 0x7fff_ffff == 0 || input_bits == 0x7f80_0000 {
        input_bits
    } else if input_bits & 0x8000_0000 != 0 || input_bits & 0x7f80_0000 == 0x7f80_0000 {
        CANONICAL_NAN
    } else {
        f32::from_bits(input_bits).sqrt().to_bits()
    }
}

fn rounded_results() -> ShaderTest {
    let inputs = input_bits();
    assert_eq!(inputs.len(), 1732);
    let mut input_buffer = [0x0123_4567_u32; 2048];
    input_buffer[..inputs.len()].copy_from_slice(&inputs);
    let mut expected = [SENTINEL; 2048];
    for (output, &input) in expected.iter_mut().zip(&inputs) {
        *output = expected_bits(input);
    }
    module_case(
        "sqrtf all exponent bands, subnormal normalization, rounding boundaries and exceptional classes",
        "__global__ void roots(const unsigned *input, unsigned *out) {
            unsigned index = blockIdx.x * blockDim.x + threadIdx.x;
            if (index >= 1732u) return;
            out[index] = __float_as_uint(sqrtf(__uint_as_float(input[index])));
        }",
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: true }, &input_buffer),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 2048]),
        ],
        1,
        [32, 1, 1],
    )
}

fn conversions_and_side_effects() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..30].copy_from_slice(&[
        0x0000_0000,
        0x8000_0000,
        0x3f80_0000,
        0x4000_0000,
        0x4040_0000,
        0x4020_0000,
        0x3fb5_04f3,
        0x1a35_04f3,
        0x1fff_ffff,
        0x2000_0000,
        0x5f7f_ffff,
        0x7f80_0000,
        CANONICAL_NAN,
        CANONICAL_NAN,
        0x4000_0000,
        0x40a0_0000,
        7,
        9,
        0x40a0_0000,
        0x3f80_0000,
        0x3f80_0001,
        0x3f7f_ffff,
        0x4780_0000,
        CANONICAL_NAN,
        0x4080_0000,
        0x3f00_0000,
        CANONICAL_NAN,
        0x0000_0000,
        0x3f80_0000,
        0x3f35_04f3,
    ]);
    output_case(
        "sqrtf scalar conversions, aliases, helper returns, overloads and argument-once updates",
        include_str!("../../../../../naga/tests/in/cuda/sqrtf.cu"),
        &expected,
    )
}

fn selected_calls() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..20].copy_from_slice(&[
        2.0_f32.to_bits(),
        5.0_f32.to_bits(),
        40.0_f32.to_bits(),
        11.0_f32.to_bits(),
        11.0_f32.sqrt().to_bits(),
        12.0_f32.to_bits(),
        12.0_f32.sqrt().to_bits(),
        13.0_f32.to_bits(),
        0,
        13.0_f32.to_bits(),
        1,
        13.0_f32.to_bits(),
        17,
        4.0_f32.to_bits(),
        17.0_f32.sqrt().to_bits(),
        18.0_f32.to_bits(),
        18.0_f32.sqrt().to_bits(),
        19.0_f32.to_bits(),
        1,
        20.0_f32.to_bits(),
    ]);
    output_case(
        "sqrtf compiler left-to-right argument snapshots, selected branches, comma sequencing and continue",
        "__device__ float identity(float value) { return value; }
        __device__ float pair(float first, float second) { return first * 10.0f + second; }
        __device__ int sqrtf(bool value) { return value ? 17 : 19; }
        __global__ void effects(unsigned *out) {
            float value = 4.0f;
            out[0] = __float_as_uint(sqrtf(value++));
            out[1] = __float_as_uint(value);
            value = 9.0f;
            out[2] = __float_as_uint(pair(sqrtf(value++), value++));
            out[3] = __float_as_uint(value);
            out[4] = __float_as_uint(sqrtf(identity(value++)));
            out[5] = __float_as_uint(value);
            bool choose = true;
            out[6] = __float_as_uint(choose ? sqrtf(value++) : sqrtf(value++));
            out[7] = __float_as_uint(value);
            bool run = false;
            out[8] = unsigned(run && sqrtf(value++));
            out[9] = __float_as_uint(value);
            run = true;
            out[10] = unsigned(run || sqrtf(value++));
            out[11] = __float_as_uint(value);
            out[12] = unsigned(sqrtf(true));
            out[13] = __float_as_uint(sqrtf(16.0f));
            value = 16.0f;
            out[14] = __float_as_uint((sqrtf(value++), sqrtf(value++)));
            out[15] = __float_as_uint(value);
            for (int index = 0; index < 2; ++index) {
                if (index == 0) continue;
                out[16] = __float_as_uint(sqrtf(value++));
            }
            out[17] = __float_as_uint(value);
            out[18] = unsigned(run && sqrtf(value++));
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
        0xbf80_0000,
        0xc080_0000,
        0x8000_0001,
        0x807f_ffff,
        0x8080_0000,
        0xff7f_ffff,
    ] {
        add(bits);
    }
    for exponent in 1..255 {
        for fraction in [0, 1, 0x003f_ffff, 0x007f_fffe, 0x007f_ffff] {
            add((exponent << 23) | fraction);
        }
    }
    for bits in 1..=16 {
        add(bits);
    }
    for shift in 0..23 {
        for offset in [-1, 0, 1] {
            let bits = (1_i32 << shift) + offset;
            if bits > 0 && bits < 0x0080_0000 {
                add(bits as u32);
            }
        }
    }
    for bits in [
        0x007f_fffd,
        0x007f_fffe,
        0x007f_ffff,
        0x0080_0000,
        0x0080_0001,
        0x3f80_0000,
        0x4000_0000,
        0x4040_0000,
        0x4080_0000,
        0x3e80_0000,
        0x7f7f_ffff,
    ] {
        add(bits);
    }
    for significand in [
        0x0080_0000,
        0x0080_0001,
        0x0080_0002,
        0x008a_bcde,
        0x00aa_aaaa,
        0x00b5_04f3,
        0x00c0_0000,
        0x00ff_ffff,
    ] {
        for exponent in [-74, -63, -1, 0, 1, 31, 63] {
            let midpoint = f64::from(2 * significand + 1) * 2.0_f64.powi(exponent - 24);
            let squared_midpoint = midpoint * midpoint;
            let mut below = (squared_midpoint as f32).to_bits();
            if below >= 0x7f80_0000 {
                continue;
            }
            while below > 0 && f64::from(f32::from_bits(below)) > squared_midpoint {
                below -= 1;
            }
            while below < 0x7f7f_ffff && f64::from(f32::from_bits(below + 1)) <= squared_midpoint {
                below += 1;
            }
            for offset in [-1, 0, 1, 2] {
                let bits = i64::from(below) + offset;
                if bits > 0 && bits < 0x7f80_0000 {
                    add(bits as u32);
                }
            }
        }
    }
    let mut state = 0x51f1_5e5d_u32;
    for _ in 0..256 {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let bits = state & 0x7fff_ffff;
        if bits > 0 && bits < 0x7f80_0000 {
            add(bits);
        }
    }
    inputs
}
