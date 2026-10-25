use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

const EDGES: [(u32, u32, u32, u32); 20] = [
    (0, 0, 0, 0),
    (1, 1, 1, 1),
    (0xffff_ffff, 1, 0xffff_ffff, 0x00ff_ffff),
    (0xffff_ffff, 0xffff_ffff, 1, 0xfe00_0001),
    (0x0080_0000, 1, 0xff80_0000, 0x0080_0000),
    (0x007f_ffff, 1, 0x007f_ffff, 0x007f_ffff),
    (0x0080_0000, 2, 0xff00_0000, 0x0100_0000),
    (0x007f_ffff, 2, 0x00ff_fffe, 0x00ff_fffe),
    (0x0080_0000, 0xffff_ffff, 0x0080_0000, 0xff80_0000),
    (0x0080_0000, 0x0080_0000, 0, 0),
    (0x007f_ffff, 0x007f_ffff, 0xff00_0001, 0xff00_0001),
    (0x0080_0000, 0x007f_ffff, 0x0080_0000, 0xff80_0000),
    (0x00ff_ffff, 2, 0xffff_fffe, 0x01ff_fffe),
    (0x0100_0001, 3, 3, 3),
    (0xff00_0001, 3, 3, 3),
    (0x0180_0000, 1, 0xff80_0000, 0x0080_0000),
    (0x8000_0000, 0x00ff_ffff, 0, 0),
    (0x0001_0000, 0x0001_0000, 0, 0),
    (0x0000_ffff, 0x0000_ffff, 0xfffe_0001, 0xfffe_0001),
    (0x0080_0001, 2, 0xff00_0002, 0x0100_0002),
];

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![word_pairs(), conversions_and_overloads(), operand_effects()]
}

fn signed_reference(first: u32, second: u32) -> u32 {
    fn signed_value(word: u32) -> i64 {
        let value = i64::from(word & 0x00ff_ffff);
        if word & 0x0080_0000 != 0 {
            value - 0x0100_0000
        } else {
            value
        }
    }
    (signed_value(first) * signed_value(second)) as u32
}

fn unsigned_reference(first: u32, second: u32) -> u32 {
    (u64::from(first & 0x00ff_ffff) * u64::from(second & 0x00ff_ffff)) as u32
}

fn word_pairs() -> ShaderTest {
    let mut pairs: Vec<_> = EDGES
        .iter()
        .map(|&(first, second, _, _)| (first, second))
        .collect();
    for bit in 0..32 {
        pairs.push((1_u32 << bit, 0x00ff_fffd));
        pairs.push((!(1_u32 << bit), 0x0080_0003));
    }
    for low_bits in [0, 1, 0x007f_ffff, 0x0080_0001] {
        for high_byte in [0, 1, 0x80, 0xff] {
            pairs.push((low_bits | (high_byte << 24), 0x8a00_0003));
        }
    }
    assert_eq!(pairs.len(), 100);
    for &(first, second, signed, unsigned) in &EDGES {
        assert_eq!(signed_reference(first, second), signed);
        assert_eq!(unsigned_reference(first, second), unsigned);
    }
    let mut input = Vec::new();
    let mut expected = Vec::new();
    for &(first, second) in &pairs {
        input.extend([first, second]);
        expected.extend([
            signed_reference(first, second),
            unsigned_reference(first, second),
        ]);
    }
    input.extend([SENTINEL; 8]);
    expected.extend([SENTINEL; 8]);
    module_case(
        "signed and unsigned 24-bit products, every bit, high-byte invariance and low 32-bit results",
        "__global__ void words(const unsigned *input, unsigned *out) {
            for (unsigned index = 0u; index < 100u; ++index) {
                unsigned first = input[index * 2u];
                unsigned second = input[index * 2u + 1u];
                out[index * 2u] = unsigned(__mul24(int(first), int(second)));
                out[index * 2u + 1u] = __umul24(first, second);
            }
        }", [1, 1, 1], &expected,
    ).module_buffers(vec![
        ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: true }, &input),
        ShaderBuffer::new(1, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 208]),
    ], 1, [1, 1, 1])
}

fn conversions_and_overloads() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        0xff80_0000,
        0xfe00_0001,
        0xff00_0001,
        0,
        0xffff_fffa,
        0xffff_fffa,
        6,
        0x00ff_ffff,
        71,
        79,
        3,
        3,
        39,
        107,
        12,
        102,
        12,
        0xffff_ffeb,
        60,
        0x01ff_fffc,
        0xffff_fff1,
        0x04ff_fff1,
        0xff80_0000,
        0x0080_0000,
        0xffff_ffeb,
        18,
        2,
        6,
        4,
        12,
        9,
        0xffff_fffd,
    ]);
    output_case(
        "multiply24 conversions, source-point overloads, literal edges, aliases and forward helpers",
        include_str!("../../../../../naga/tests/in/cuda/multiply24.cu"), &expected,
    )
}

fn operand_effects() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        6,
        4,
        20,
        6,
        42,
        8,
        72,
        10,
        21,
        7,
        45,
        9,
        14,
        1,
        7,
        11,
        91,
        13,
        187,
        17,
        0,
        10,
        1,
        10,
        20,
        11,
        0,
        12,
        13,
        13,
        0x01ff_fffe,
        0xff00_0002,
    ]);
    output_case(
        "multiply24 arguments execute once with left-to-right snapshots and lazy selected branches",
        "__device__ int next(int value) { return value; }
        __global__ void effects(unsigned *out) {
            int counter = 2;
            out[0] = unsigned(__mul24(counter++, counter++)); out[1] = unsigned(counter);
            out[2] = __umul24(unsigned(counter++), unsigned(counter++)); out[3] = unsigned(counter);
            out[4] = unsigned(__mul24(next(counter++), next(counter++))); out[5] = unsigned(counter);
            out[6] = __umul24(unsigned(next(counter++)), unsigned(next(counter++))); out[7] = unsigned(counter);
            int value = 3;
            out[8] = unsigned(__mul24(value, (value = 7))); out[9] = unsigned(value);
            value = 5;
            out[10] = __umul24(unsigned(value), unsigned(value = 9)); out[11] = unsigned(value);
            int values[2] = {2, 11}; int index = 0;
            out[12] = unsigned(__mul24(values[index++], (values[0] = 7)));
            out[13] = unsigned(index); out[14] = unsigned(values[0]); out[15] = unsigned(values[1]);
            bool choose = true;
            out[16] = unsigned(__mul24((choose ? values[0] : values[1]), (values[0] = 13)));
            out[17] = unsigned(values[0]);
            choose = false;
            out[18] = __umul24(unsigned(choose ? values[0] : values[1]), unsigned(values[1] = 17));
            out[19] = unsigned(values[1]);
            out[20] = unsigned(false && __mul24(counter++, 2)); out[21] = unsigned(counter);
            out[22] = unsigned(true || __umul24(unsigned(counter++), 2u)); out[23] = unsigned(counter);
            out[24] = unsigned(true ? __mul24(counter++, 2) : __mul24(counter++, 3)); out[25] = unsigned(counter);
            out[26] = __umul24(unsigned(counter++), 0u); out[27] = unsigned(counter);
            out[28] = unsigned((__mul24(counter++, 1), counter)); out[29] = unsigned(counter);
            out[30] = __umul24(0xffffffffu, 0x01000002u);
            out[31] = unsigned(__mul24(0x01800001, 0xff000002u));
        }", &expected,
    )
}

fn output_case(name: &str, source: &str, expected: &[u32; 40]) -> ShaderTest {
    module_case(name, source, [1, 1, 1], expected).module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 40],
        )],
        0,
        [1, 1, 1],
    )
}
