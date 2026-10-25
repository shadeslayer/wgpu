use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        canonical(),
        type_and_lazy_values(),
        source_state_and_characters(),
    ]
}

fn canonical() -> ShaderTest {
    let mut expected = [SENTINEL; 56];
    for lane in 0..2 {
        expected[lane * 24..lane * 24 + 24].copy_from_slice(&[
            7,
            11,
            13,
            17,
            19,
            23,
            29,
            31,
            37,
            41,
            43,
            47,
            53,
            59,
            61,
            67,
            71,
            lane as u32 + 73,
            0,
            1,
            1,
            lane as u32,
            0x8000_0000,
            0xffff_ffff,
        ]);
    }
    module_case(
        "conditional inclusion preserves source helper and storage semantics without runtime wide integers",
        include_str!("../../../../../naga/tests/in/cuda/preprocessing-conditions.cu"),
        [2, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 56])],
        0,
        [1, 1, 1],
    )
}

fn type_and_lazy_values() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..24].copy_from_slice(&[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ]);
    module_case(
        "integer types, precedence and lazy values select exact independent branch markers",
        r#"
__global__ void type_and_lazy_values(unsigned *out) {
#if (0 ? 1u : -1) > 0
    out[0] = 1u;
#else
    out[0] = 99u;
#endif
#if (1 ? -1 : 0u) > 0
    out[1] = 2u;
#else
    out[1] = 99u;
#endif
#if (1 ? -1 : 0u) == 0xffffffffffffffffu
    out[2] = 3u;
#else
    out[2] = 99u;
#endif
#if (1 ? 0u, -1 : 0) < 0
    out[3] = 4u;
#else
    out[3] = 99u;
#endif
#if !(0 && (1 / 0))
    out[4] = 5u;
#else
    out[4] = 99u;
#endif
#if 1 || (9223372036854775807 + 1)
    out[5] = 6u;
#else
    out[5] = 99u;
#endif
#if (0 ? ((-9223372036854775807 - 1) / -1) : 17) == 17
    out[6] = 7u;
#else
    out[6] = 99u;
#endif
#if (1 ? 19 : (1 / 0)) == 19
    out[7] = 8u;
#else
    out[7] = 99u;
#endif
#if (0 ? (1 << 64) : 23) == 23
    out[8] = 9u;
#else
    out[8] = 99u;
#endif
#if 1 + 2 * 3 == 7
    out[9] = 10u;
#else
    out[9] = 99u;
#endif
#if 12 / 3 * 2 == 8
    out[10] = 11u;
#else
    out[10] = 99u;
#endif
#if 17 % 5 == 2
    out[11] = 12u;
#else
    out[11] = 99u;
#endif
#if (1 + 2 << 3) == 24
    out[12] = 13u;
#else
    out[12] = 99u;
#endif
#if (8 >> 1 + 1) == 2
    out[13] = 14u;
#else
    out[13] = 99u;
#endif
#if 1 < 2 == 1
    out[14] = 15u;
#else
    out[14] = 99u;
#endif
#if (1 | 2 ^ 3 & 1) == 3
    out[15] = 16u;
#else
    out[15] = 99u;
#endif
#if (~0u >> 63) == 1
    out[16] = 17u;
#else
    out[16] = 99u;
#endif
#if (-3 >> 1) == -2
    out[17] = 18u;
#else
    out[17] = 99u;
#endif
#if (-1 << 1) == -2
    out[18] = 19u;
#else
    out[18] = 99u;
#endif
#if (1LL << 63) < 0
    out[19] = 20u;
#else
    out[19] = 99u;
#endif
#if 0xffffffffffffffffu + 1u == 0u
    out[20] = 21u;
#else
    out[20] = 99u;
#endif
#if 0u - 1u == 0xffffffffffffffffu
    out[21] = 22u;
#else
    out[21] = 99u;
#endif
#if -5 / 2 == -2 && -5 % 2 == -1
    out[22] = 23u;
#else
    out[22] = 99u;
#endif
#if (1 ? 0 ? 3 : 5 : 7) == 5
    out[23] = 24u;
#else
    out[23] = 99u;
#endif
}
"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 32],
        )],
        0,
        [1, 1, 1],
    )
}

fn source_state_and_characters() -> ShaderTest {
    let mut expected = [SENTINEL; 24];
    expected[..16].copy_from_slice(&[3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 0, 1, 3, 3]);
    module_case(
        "directive eligibility, ASCII values and ordinary macro effects preserve exact outputs and guards",
        r#"
#define MODE 2
#define ALIAS ABSENT
#define SELF SELF
#define LEFT RIGHT
#define RIGHT LEFT
#define STEP counter++
#define EFFECT (counter += 2u)
__global__ void source_state_and_characters(unsigned *out) {
#if MODE == 1
    out[0] = 99u;
#elif MODE == 2
    out[0] = 3u;
#elif __LINE__ + bad((
    out[0] = 99u;
#endif
#undef MODE
#if MODE
    out[1] = 99u;
#else
    out[1] = 5u;
#endif
#if defined(ALIAS) && !defined(ABSENT)
    out[2] = 7u;
#else
    out[2] = 99u;
#endif
#if 0
    out[3] = 99u;
#elifdef ABSENT
    out[3] = 99u;
#elifndef MODE
    out[3] = 11u;
#else
    out[3] = 99u;
#endif
#if SELF || LEFT || RIGHT
    out[4] = 99u;
#else
    out[4] = 13u;
#endif
#if !int && !sizeof && true && !false
    out[5] = 17u;
#else
    out[5] = 99u;
#endif
#if '\\' == 92 && '\'' == 39
    out[6] = 19u;
#else
    out[6] = 99u;
#endif
#if '\0' == 0 && '\n' == 10 && '\t' == 9
    out[7] = 23u;
#else
    out[7] = 99u;
#endif
#if '\101' == 65 && '\x7f' == 127
    out[8] = 29u;
#else
    out[8] = 99u;
#endif
#if not false and ((compl 0 bitand 3) == 3) and ((1 bitor 2) == 3) and ((3 xor 1) not_eq 0) or false
    out[9] = 31u;
#else
    out[9] = 99u;
#endif
#if !__has_embed && !defined(__has_embed)
    out[10] = 37u;
#else
    out[10] = 99u;
#endif
#if 1
    out[11] = 41u;
#elifdef __has_include
    out[11] = 99u;
#else
    out[11] = 99u;
#endif
    unsigned counter = 0u;
    out[12] = STEP;
    out[13] = counter;
    out[14] = EFFECT;
    out[15] = counter;
}
"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 24])],
        0,
        [1, 1, 1],
    )
}
