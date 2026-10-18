use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

const RAW_BITS: [u32; 24] = [
    0x0000_0000,
    0x8000_0000,
    0x3f80_0000,
    0xbf80_0000,
    0x3fc0_0000,
    0xc020_0000,
    0x0000_0001,
    0x8000_0001,
    0x007f_ffff,
    0x807f_ffff,
    0x0080_0000,
    0x8080_0000,
    0x7f7f_ffff,
    0xff7f_ffff,
    0x7f80_0000,
    0xff80_0000,
    0x7fc1_2345,
    0xffc5_4321,
    0x7f80_0001,
    0xff80_0001,
    0x7fff_ffff,
    0xffff_ffff,
    0x3f00_0001,
    0xbeff_ffff,
];

pub(super) fn cases() -> Vec<ShaderTest> {
    let mut cases = vec![conversions_and_side_effects(), overloads_and_control_flow()];
    for (name, input_type, output_type) in [
        ("__float_as_int", "float", "int"),
        ("__float_as_uint", "float", "unsigned"),
        ("__int_as_float", "int", "float"),
        ("__uint_as_float", "unsigned", "float"),
    ] {
        let mut input = [0x0123_4567_u32; 32];
        input[..RAW_BITS.len()].copy_from_slice(&RAW_BITS);
        let mut expected = [SENTINEL; 32];
        expected[..RAW_BITS.len()].copy_from_slice(&RAW_BITS);
        let source = format!(
            "__global__ void reinterpret(const {input_type} *input, {output_type} *out) {{
                for (unsigned i = 0u; i < 24u; ++i) {{ out[i] = {name}(input[i]); }}
            }}"
        );
        cases.push(
            module_case(
                &format!("{name} one-way storage bits, exceptional values and full tails"),
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
            ),
        );
    }
    cases
}

fn conversions_and_side_effects() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..30].copy_from_slice(&[
        0x3f80_0000,
        0x4040_0000,
        0x40a0_0000,
        0x4030_0000,
        0x3f80_0000,
        0x4040_0000,
        0x40a0_0000,
        0x4030_0000,
        1,
        3,
        5,
        2,
        1,
        3,
        5,
        2,
        0x3fc0_0000,
        0x4020_0000,
        0x4020_0000,
        0x4060_0000,
        0x3f80_0000,
        0x3f80_0001,
        0x4000_0000,
        0x4000_0001,
        7,
        0xbf80_0000,
        0x7f80_0000,
        0x7fc1_2345,
        0x3f80_0000,
        0x3f80_0000,
    ]);
    output_case(
        "bitcast parameter conversions, aliases, helper returns and argument-once updates",
        include_str!("../../../../../naga/tests/in/cuda/bitcasts.cu"),
        &expected,
    )
}

fn overloads_and_control_flow() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..10].copy_from_slice(&[
        0x3fc0_0001,
        0x4020_0000,
        7,
        0x3fc0_0000,
        0x4020_0000,
        0x4060_0000,
        0x4060_0000,
        0x4090_0000,
        0,
        0x4090_0000,
    ]);
    output_case(
        "bitcast builtin and user overloads, compiler left-to-right argument snapshots and selected branches",
        "__device__ int __float_as_uint(bool value) { return value ? 7 : 9; }
        __device__ float next(float value) { return value; }
        __device__ int pair(int first, int second) { return first + second; }
        __global__ void effects(unsigned *out) {
            float value = 1.5f;
            out[0] = unsigned(pair(__float_as_int(value), int(value++)));
            out[1] = __float_as_uint(value);
            out[2] = unsigned(__float_as_uint(true));
            out[3] = __float_as_uint(1.5f);
            out[4] = unsigned(__float_as_int(next(value++)));
            out[5] = __float_as_uint(value);
            bool choose = true;
            out[6] = choose ? __float_as_uint(value++) : __float_as_uint(value++);
            out[7] = __float_as_uint(value);
            bool run = false;
            bool got = run && __float_as_uint(value++);
            out[8] = unsigned(got);
            out[9] = __float_as_uint(value);
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
