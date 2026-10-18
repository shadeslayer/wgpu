use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

const EDGES: [(u32, [u32; 4]); 16] = [
    (0x0000_0000, [32, 0, 0, 0x0000_0000]),
    (0xffff_ffff, [0, 1, 32, 0xffff_ffff]),
    (0x8000_0000, [0, 32, 1, 0x0000_0001]),
    (0x7fff_ffff, [1, 1, 31, 0xffff_fffe]),
    (0xaaaa_aaaa, [0, 2, 16, 0x5555_5555]),
    (0x5555_5555, [1, 1, 16, 0xaaaa_aaaa]),
    (0x0123_4567, [7, 1, 12, 0xe6a2_c480]),
    (0x89ab_cdef, [0, 1, 20, 0xf7b3_d591]),
    (0x00ff_00ff, [8, 1, 16, 0xff00_ff00]),
    (0xff00_ff00, [0, 9, 16, 0x00ff_00ff]),
    (0x0000_ffff, [16, 1, 16, 0xffff_0000]),
    (0xffff_0000, [0, 17, 16, 0x0000_ffff]),
    (0x7fff_fffe, [1, 2, 30, 0x7fff_fffe]),
    (0x8000_0001, [0, 1, 2, 0x8000_0001]),
    (0x0000_0003, [30, 1, 2, 0xc000_0000]),
    (0xffff_fffe, [0, 2, 31, 0x7fff_ffff]),
];

pub(super) fn cases() -> Vec<ShaderTest> {
    let mut words: Vec<_> = EDGES.iter().map(|&(word, _)| word).collect();
    for bit in 0..32 {
        words.push(1_u32 << bit);
        words.push(!(1_u32 << bit));
    }
    let mut mixed = 0x9e37_79b9_u32;
    for _ in 0..16 {
        mixed = mixed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        words.push(mixed);
    }
    assert_eq!(words.len(), 96);
    let mut cases = vec![conversions_and_aliases(), overloads_and_side_effects()];
    for (operation, (name, input_type, output_type)) in [
        ("__clz", "int", "int"),
        ("__ffs", "int", "int"),
        ("__popc", "unsigned", "int"),
        ("__brev", "unsigned", "unsigned"),
    ]
    .into_iter()
    .enumerate()
    {
        let mut input = [0x0123_4567_u32; 128];
        input[..words.len()].copy_from_slice(&words);
        let mut expected = [SENTINEL; 128];
        for (output, &word) in expected.iter_mut().zip(&words) {
            *output = reference(operation, word);
        }
        for &(word, expected) in &EDGES {
            assert_eq!(reference(operation, word), expected[operation]);
        }
        let source = format!(
            "__global__ void integer_words(const {input_type} *input, {output_type} *out) {{
                for (unsigned i = 0u; i < 96u; ++i) {{ out[i] = {name}(input[i]); }}
            }}"
        );
        cases.push(module_case(
            &format!("{name} zero, signed edges, every single bit and complement, mixed words and full tails"),
            &source, [1, 1, 1], &expected,
        ).module_buffers(vec![
            ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: true }, &input),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 128]),
        ], 1, [1, 1, 1]));
    }
    cases
}

fn reference(operation: usize, word: u32) -> u32 {
    match operation {
        0 => word.leading_zeros(),
        1 => {
            if word == 0 {
                0
            } else {
                word.trailing_zeros() + 1
            }
        }
        2 => word.count_ones(),
        3 => word.reverse_bits(),
        _ => unreachable!(),
    }
}

fn conversions_and_aliases() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..30].copy_from_slice(&[
        31,
        28,
        0,
        28,
        1,
        4,
        32,
        4,
        1,
        32,
        2,
        2,
        0x8000_0000,
        0xffff_ffff,
        0xe6a2_c480,
        0x4000_0000,
        2,
        32,
        32,
        0,
        32,
        0,
        0x8000_0000,
        1,
        0,
        2,
        16,
        0x5555_5555,
        7,
        1,
    ]);
    output_case(
        "integer intrinsic parameter conversions, aliases, helper results, zero and signed popcount result",
        include_str!("../../../../../naga/tests/in/cuda/integer-intrinsics.cu"), &expected,
    )
}

fn overloads_and_side_effects() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..20].copy_from_slice(&[
        408,
        9,
        7,
        28,
        101,
        2,
        13,
        0x4000_0000,
        1,
        10,
        2,
        11,
        0,
        11,
        28,
        12,
        2,
        13,
        0xb000_0000,
        14,
    ]);
    output_case(
        "integer intrinsic user overloads, compiler left-to-right snapshots, argument-once helpers and selected branches",
        "__device__ int __clz(bool value) { return value ? 7 : 9; }
        __device__ int __popc(int value) { return value + 100; }
        __device__ unsigned __brev(bool value) { return value ? 13u : 17u; }
        __device__ int next(int value) { return value; }
        __device__ int pair(int first, int second) { return first * 100 + second; }
        __global__ void effects(unsigned *out) {
            int value = 8;
            out[0] = unsigned(pair(__ffs(value), value++)); out[1] = unsigned(value);
            out[2] = unsigned(__clz(true)); out[3] = unsigned(__clz(8));
            out[4] = unsigned(__popc(true)); out[5] = unsigned(__popc(3u));
            out[6] = __brev(true); out[7] = __brev(2u);
            out[8] = unsigned(__ffs(next(value++))); out[9] = unsigned(value);
            bool choose = true;
            out[10] = unsigned(choose ? __ffs(value++) : __ffs(value++)); out[11] = unsigned(value);
            bool run = false; bool got = run && __ffs(value++);
            out[12] = unsigned(got); out[13] = unsigned(value);
            out[14] = unsigned(__clz(next(value++))); out[15] = unsigned(value);
            out[16] = unsigned(__popc(unsigned(next(value++)))); out[17] = unsigned(value);
            out[18] = __brev(unsigned(next(value++))); out[19] = unsigned(value);
        }", &expected,
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
