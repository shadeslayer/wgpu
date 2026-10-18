@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

fn count(value: u32) -> i32 {
    var value_1: u32;

    value_1 = value;
    let _e2 = value_1;
    return i32(countOneBits(_e2));
}

fn scan(value_2: i32) -> i32 {
    var value_3: i32;

    value_3 = value_2;
    let _e2 = value_3;
    return (firstTrailingBit(_e2) + 1i);
}

fn _clz(first: i32, second: i32) -> i32 {
    var first_1: i32;
    var second_1: i32;

    first_1 = first;
    second_1 = second;
    let _e4 = first_1;
    let _e5 = second_1;
    return (_e4 + _e5);
}

@compute @workgroup_size(1, 1, 1)
fn integer_intrinsics() {
    out[0i] = u32(countLeadingZeros(i32(true)));
    out[1i] = u32(countLeadingZeros(8i));
    out[2i] = u32(countLeadingZeros(i32(2147483648u)));
    out[3i] = u32(countLeadingZeros(i32(8.75f)));
    out[4i] = u32((firstTrailingBit(i32(true)) + 1i));
    out[5i] = u32((firstTrailingBit(8i) + 1i));
    out[6i] = u32((firstTrailingBit(i32(2147483648u)) + 1i));
    out[7i] = u32((firstTrailingBit(i32(8.75f)) + 1i));
    out[8i] = u32(i32(countOneBits(u32(true))));
    out[9i] = u32(i32(countOneBits(u32(-(1i)))));
    out[10i] = u32(i32(countOneBits(2147483649u)));
    out[11i] = u32(i32(countOneBits(u32(3.75f))));
    out[12i] = reverseBits(u32(true));
    out[13i] = reverseBits(u32(-(1i)));
    out[14i] = reverseBits(19088743u);
    out[15i] = reverseBits(u32(2.75f));
    let _e106 = count(2147483649u);
    out[16i] = u32(_e106);
    let _e112 = scan(i32(2147483648u));
    out[17i] = u32(_e112);
    out[18i] = u32(countLeadingZeros(0i));
    out[19i] = u32((firstTrailingBit(0i) + 1i));
    out[20i] = u32(i32(countOneBits(4294967295u)));
    out[21i] = reverseBits(0u);
    out[22i] = reverseBits(1u);
    out[23i] = reverseBits(2147483648u);
    out[24i] = u32(countLeadingZeros(-(2i)));
    out[25i] = u32((firstTrailingBit(-(2i)) + 1i));
    out[26i] = u32(i32(countOneBits(2863311530u)));
    out[27i] = reverseBits(2863311530u);
    let _e172 = _clz(3i, 4i);
    out[28i] = u32(_e172);
    out[29i] = u32(((i32(countOneBits(3u)) - 3i) < 0i));
}
