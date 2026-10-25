use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![canonical(), expansion_and_boundaries(), lexical_phases()]
}

fn canonical() -> ShaderTest {
    let mut expected = [SENTINEL; 56];
    for lane in 0..2 {
        expected[lane * 24..lane * 24 + 24].copy_from_slice(&[
            7,
            11,
            22,
            14,
            5,
            12,
            lane as u32 + 5,
            lane as u32 + 1,
            lane as u32 + 2,
            17,
            19,
            23,
            1,
            2,
            2,
            31,
            if lane == 0 { 2 } else { 4 },
            2,
            1,
            1,
            61,
            lane as u32,
            0x8000_0000,
            0xffff_ffff,
        ]);
    }
    module_case(
        "object macros preserve source order, helper prototypes, class copies and exact literal bits",
        include_str!("../../../../../naga/tests/in/cuda/preprocessing.cu"),
        [2, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 56])],
        0,
        [1, 1, 1],
    )
}

fn expansion_and_boundaries() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..24].copy_from_slice(&[
        5,
        3,
        0,
        1,
        2,
        0,
        2,
        1,
        2,
        3,
        3,
        4,
        4,
        1,
        2,
        17,
        19,
        23,
        0xffff_ffff,
        0x4480_1000,
        0x42fa_0000,
        9,
        15,
        0x8000_0000,
    ]);
    module_case(
        "rescanning preserves token boundaries and each source evaluation while scalar branches stay lazy",
        r#"
#define PLUS +
#define STEP counter++
#define EFFECT (counter += 3, counter)
#define SELF SELF
#define LEFT RIGHT
#define RIGHT LEFT
#define NAME ABSENT
#define MAXIMUM 0xffff'ffffU
#define FRACTION 1'024.5f
#define EXPONENT 1.25e+0'2f
__global__ void expansion_and_boundaries(unsigned *out) {
    unsigned value = 3u;
    out[0] = value PLUS+ 2u;
    out[1] = value;
    unsigned counter = 0u;
    out[2] = STEP;
    out[3] = STEP;
    out[4] = counter;
    out[5] = false && EFFECT;
    out[6] = counter;
    out[7] = true || EFFECT;
    out[8] = counter;
    out[9] = (counter++, counter);
    out[10] = counter;
    {
#define BLOCK_VALUE 4u
        out[11] = BLOCK_VALUE;
    }
    out[12] = BLOCK_VALUE;
#ifdef NAME
    out[13] = 1u;
#else
    out[13] = 99u;
#endif
#ifndef ABSENT
    out[14] = 2u;
#else
    out[14] = 99u;
#endif
    unsigned SELF = 17u, LEFT = 19u, RIGHT = 23u;
    out[15] = SELF;
    out[16] = LEFT;
    out[17] = RIGHT;
    out[18] = MAXIMUM;
    out[19] = __float_as_uint(FRACTION);
    out[20] = __float_as_uint(EXPONENT);
    out[21] = 0b10'01u;
    out[22] = 017;
    out[23] = __float_as_uint(-0.0f);
}
"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 32])],
        0,
        [1, 1, 1],
    )
}

fn lexical_phases() -> ShaderTest {
    let mut expected = [SENTINEL; 20];
    expected[..12].copy_from_slice(&[23, 5, 3, 4, 37, 41, 43, 47, 53, 59, 61, 67]);
    module_case(
        "line splicing and opaque literals preserve directive boundaries and skipped branches",
        concat!(
            "\u{feff}",
            r###"
#defi\
ne FIRST 23u
#define COMBINED 2u + \
3u
#define MULTILINE 1u /* comment
*/ + 2u
#define RAW R"tag(begin
)ta\
g"
#endif
#define SHADOW 99u
end)tag"
#define QUOTED "FIRST\" #endif // /*"
#define CHARACTER '\''
#define PREFIXED u8"#else"
#ifdef ABSENT
u8R"tag(
#endif
#define HIDDEN 99u
)tag"
#if invalid(())
#include "never-read.h"
#endif
#endif
#define __CUDACC__ 1
extern "C" __global__ void lexical_phases(unsigned *out) {
    unsigned val\
ue = 4u;
    out[0] = FIRST;
    out[1] = COMBINED;
    out[2] = MULTILINE;
    out[3] = value;
#ifndef SHADOW
    out[4] = 37u;
#else
    out[4] = 99u;
#endif
#ifndef HIDDEN
    out[5] = 41u;
#else
    out[5] = 99u;
#endif
    out[6] = 43u;
    /\
/ out[6] = 99u;
    /\
* hidden store: out[7] = 99u; */
    out[7] = 47u;
    out[8] = 53u;
#define SPACED 5\ 	
9u
    out[9] = SPACED;
#ifdef __CUDACC__
    out[10] = 61u;
#else
    out[10] = 99u;
#endif
    out[11] = 67u;
}
"###
        ),
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 20],
        )],
        0,
        [1, 1, 1],
    )
}
