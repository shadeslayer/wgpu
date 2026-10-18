use super::{float_case, ShaderTest};

// Each row contains the two inputs, minimum, and maximum as binary32 bits.
const PAIRS: [(u32, u32, u32, u32); 48] = [
    (0x3fc00000, 0x40200000, 0x3fc00000, 0x40200000),
    (0x40200000, 0x3fc00000, 0x3fc00000, 0x40200000),
    (0xbfc00000, 0xc0200000, 0xc0200000, 0xbfc00000),
    (0xc0200000, 0xbfc00000, 0xc0200000, 0xbfc00000),
    (0xbfc00000, 0x40200000, 0xbfc00000, 0x40200000),
    (0x40200000, 0xbfc00000, 0xbfc00000, 0x40200000),
    (0x3fc00000, 0x3fc00000, 0x3fc00000, 0x3fc00000),
    (0xbfc00000, 0xbfc00000, 0xbfc00000, 0xbfc00000),
    (0x00000000, 0x80000000, 0x80000000, 0x00000000),
    (0x80000000, 0x00000000, 0x80000000, 0x00000000),
    (0x00000000, 0x00000000, 0x00000000, 0x00000000),
    (0x80000000, 0x80000000, 0x80000000, 0x80000000),
    (0x7f800000, 0x40200000, 0x40200000, 0x7f800000),
    (0x40200000, 0x7f800000, 0x40200000, 0x7f800000),
    (0xff800000, 0xc0200000, 0xff800000, 0xc0200000),
    (0xc0200000, 0xff800000, 0xff800000, 0xc0200000),
    (0xff800000, 0x7f800000, 0xff800000, 0x7f800000),
    (0x7f800000, 0xff800000, 0xff800000, 0x7f800000),
    (0x7f800000, 0x7f800000, 0x7f800000, 0x7f800000),
    (0xff800000, 0xff800000, 0xff800000, 0xff800000),
    (0x00000001, 0x007fffff, 0x00000001, 0x007fffff),
    (0x007fffff, 0x00000001, 0x00000001, 0x007fffff),
    (0x80000001, 0x807fffff, 0x807fffff, 0x80000001),
    (0x807fffff, 0x80000001, 0x807fffff, 0x80000001),
    (0x00800000, 0x007fffff, 0x007fffff, 0x00800000),
    (0x007fffff, 0x00800000, 0x007fffff, 0x00800000),
    (0x80800000, 0x807fffff, 0x80800000, 0x807fffff),
    (0x807fffff, 0x80800000, 0x80800000, 0x807fffff),
    (0x7fc12345, 0x3fc00000, 0x3fc00000, 0x3fc00000),
    (0x3fc00000, 0x7fc12345, 0x3fc00000, 0x3fc00000),
    (0xffc12345, 0xc0200000, 0xc0200000, 0xc0200000),
    (0xc0200000, 0xffc12345, 0xc0200000, 0xc0200000),
    (0x7f800001, 0x00000000, 0x00000000, 0x00000000),
    (0x80000000, 0xff800001, 0x80000000, 0x80000000),
    (0x7fc12345, 0xffc54321, 0x7fc00000, 0x7fc00000),
    (0x7f800001, 0xff800001, 0x7fc00000, 0x7fc00000),
    (0x7fc12345, 0x7f800000, 0x7f800000, 0x7f800000),
    (0xff800000, 0x7fc12345, 0xff800000, 0xff800000),
    (0xffc12345, 0x7fc54321, 0x7fc00000, 0x7fc00000),
    (0x7f800001, 0x7fc12345, 0x7fc00000, 0x7fc00000),
    (0x00000001, 0x00000000, 0x00000000, 0x00000001),
    (0x80000001, 0x80000000, 0x80000001, 0x80000000),
    (0x7f7fffff, 0xff7fffff, 0xff7fffff, 0x7f7fffff),
    (0x00800000, 0x80800000, 0x80800000, 0x00800000),
    (0x00000000, 0x80000001, 0x80000001, 0x00000000),
    (0x80000000, 0x00000001, 0x80000000, 0x00000001),
    (0xffc12345, 0x00000000, 0x00000000, 0x00000000),
    (0xffc12345, 0x80000000, 0x80000000, 0x80000000),
];

pub(super) fn cases() -> Vec<ShaderTest> {
    let initial = [
        3.25_f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
    ];
    let expected = [
        -7.5_f32,
        3.25,
        1.5,
        3.25,
        -3.0,
        5.0,
        0.0,
        1.0,
        2.5,
        -3.5,
        3.25,
        2.5,
        2.25,
        3.5,
        3.25,
        4.25,
        16_777_216.0,
        4_294_967_296.0,
        3.25,
        3.25,
    ];
    let mut cases = vec![float_case(
        "floating min/max conversions, nesting, overloads and argument snapshots",
        include_str!("../../../../../naga/tests/in/cuda/float-minmax.cu"),
        &initial.map(f32::to_bits),
        &expected.map(f32::to_bits),
    )];
    for (operation, maximum) in [
        ("min", false),
        ("max", true),
        ("fminf", false),
        ("fmaxf", true),
    ] {
        let source = format!(
            "__global__ void float_pairs(float *out) {{
    for (int i = 0; i < 12; ++i) {{
        out[i] = {operation}(out[i], out[16 + i]);
    }}
}}"
        );
        for (batch, pairs) in PAIRS.as_chunks::<12>().0.iter().enumerate() {
            let mut initial = [0x4f000001; 32];
            for (index, &(left, right, _, _)) in pairs.iter().enumerate() {
                initial[index] = left;
                initial[16 + index] = right;
            }
            let mut expected = initial;
            for (index, &(_, _, minimum, max)) in pairs.iter().enumerate() {
                expected[index] = if maximum { max } else { minimum };
            }
            cases.push(float_case(
                &format!("{operation} binary32 bit patterns, batch {batch}"),
                &source,
                &initial,
                &expected,
            ));
        }
    }
    cases
}
