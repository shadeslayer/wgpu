@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

fn signed_conversion(first_1: f32, second_1: f32) -> i32 {
    var first_2: f32;
    var second_2: f32;

    first_2 = first_1;
    second_2 = second_1;
    let _e4 = first_2;
    let _e5 = second_2;
    return bitcast<i32>((bitcast<u32>(extractBits(i32(_e4), 0u, 24u)) * bitcast<u32>(extractBits(i32(_e5), 0u, 24u))));
}

fn unsigned_conversion(first_3: f32, second_3: f32) -> u32 {
    var first_4: f32;
    var second_4: f32;

    first_4 = first_3;
    second_4 = second_3;
    let _e4 = first_4;
    let _e5 = second_4;
    return (extractBits(u32(_e4), 0u, 24u) * extractBits(u32(_e5), 0u, 24u));
}

fn _mul24_(first_5: bool, second_5: bool) -> i32 {
    var first_6: bool;
    var second_6: bool;
    var local_2: i32;
    var local_3: i32;

    first_6 = first_5;
    second_6 = second_5;
    let _e4 = first_6;
    if _e4 {
        let _e5 = second_6;
        if _e5 {
            local_2 = 71i;
        } else {
            local_2 = 73i;
        }
        let _e9 = local_2;
        local_3 = _e9;
    } else {
        local_3 = 79i;
    }
    let _e12 = local_3;
    return _e12;
}

fn _mul24_1(first_7: f32, second_7: f32) -> i32 {
    var first_8: f32;
    var second_8: f32;

    first_8 = first_7;
    second_8 = second_7;
    let _e5 = first_8;
    let _e7 = second_8;
    return i32(((_e5 * 10f) + _e7));
}

fn _umul24_(first_9: i32, second_9: i32) -> u32 {
    var first_10: i32;
    var second_10: i32;

    first_10 = first_9;
    second_10 = second_9;
    let _e4 = first_10;
    let _e5 = second_10;
    return u32(((_e4 + _e5) + 100i));
}

fn signed_pair(first_11: i32, second_11: i32) -> i32 {
    var first_12: i32;
    var second_12: i32;

    first_12 = first_11;
    second_12 = second_11;
    let _e4 = first_12;
    let _e5 = second_12;
    return bitcast<i32>((bitcast<u32>(extractBits(_e4, 0u, 24u)) * bitcast<u32>(extractBits(_e5, 0u, 24u))));
}

@compute @workgroup_size(1, 1, 1)
fn multiply24_() {
    var first: i32;
    var second: u32;
    var local: i32;
    var values: array<i32, 2>;
    var Count: i32;
    var local_1: i32;
    var scratch: array<i32, 2>;
    var counter: i32;
    var total: i32;
    var index: i32;

    out[0i] = u32(bitcast<i32>((bitcast<u32>(extractBits(8388608i, 0u, 24u)) * bitcast<u32>(extractBits(1i, 0u, 24u)))));
    out[1i] = (extractBits(16777215u, 0u, 24u) * extractBits(16777215u, 0u, 24u));
    out[2i] = u32(bitcast<i32>((bitcast<u32>(extractBits(8388607i, 0u, 24u)) * bitcast<u32>(extractBits(8388607i, 0u, 24u)))));
    out[3i] = (extractBits(16777216u, 0u, 24u) * extractBits(13u, 0u, 24u));
    let _e50 = signed_conversion(3.75f, -(2.75f));
    out[4i] = u32(_e50);
    let _e57 = signed_conversion(-(3.75f), 2.75f);
    out[5i] = u32(_e57);
    let _e63 = unsigned_conversion(3.75f, 2.75f);
    out[6i] = _e63;
    let _e68 = unsigned_conversion(16777215f, 1f);
    out[7i] = _e68;
    let _e73 = _mul24_(true, true);
    out[8i] = u32(_e73);
    let _e79 = _mul24_(false, true);
    out[9i] = u32(_e79);
    out[10i] = u32(bitcast<i32>((bitcast<u32>(extractBits(3i, 0u, 24u)) * bitcast<u32>(extractBits(i32(true), 0u, 24u)))));
    out[11i] = u32(bitcast<i32>((bitcast<u32>(extractBits(i32(true), 0u, 24u)) * bitcast<u32>(extractBits(3i, 0u, 24u)))));
    let _e113 = _mul24_1(3.75f, 2f);
    out[12i] = u32(_e113);
    let _e119 = _umul24_(3i, 4i);
    out[13i] = _e119;
    out[14i] = (extractBits(3u, 0u, 24u) * extractBits(4u, 0u, 24u));
    let _e135 = _umul24_(i32(true), i32(true));
    out[15i] = _e135;
    out[16i] = (extractBits(3u, 0u, 24u) * extractBits(u32(4i), 0u, 24u));
    let _e151 = signed_pair(-(7i), 3i);
    out[17i] = u32(_e151);
    out[18i] = u32(bitcast<i32>((bitcast<u32>(extractBits(bitcast<i32>((bitcast<u32>(extractBits(3i, 0u, 24u)) * bitcast<u32>(extractBits(4i, 0u, 24u)))), 0u, 24u)) * bitcast<u32>(extractBits(5i, 0u, 24u)))));
    out[19i] = (extractBits((extractBits(16777215u, 0u, 24u) * extractBits(2u, 0u, 24u)), 0u, 24u) * extractBits(2u, 0u, 24u));
    first = -3i;
    second = 5u;
    let _e196 = first;
    let _e197 = second;
    out[20i] = u32(bitcast<i32>((bitcast<u32>(extractBits(_e196, 0u, 24u)) * bitcast<u32>(extractBits(i32(_e197), 0u, 24u)))));
    let _e210 = first;
    let _e212 = second;
    out[21i] = (extractBits(u32(_e210), 0u, 24u) * extractBits(_e212, 0u, 24u));
    out[22i] = u32(bitcast<i32>((bitcast<u32>(extractBits(25165824i, 0u, 24u)) * bitcast<u32>(extractBits(16777217i, 0u, 24u)))));
    out[23i] = (extractBits(4286578688u, 0u, 24u) * extractBits(4278190081u, 0u, 24u));
    if true {
        local = bitcast<i32>((bitcast<u32>(extractBits(-(3i), 0u, 24u)) * bitcast<u32>(extractBits(7i, 0u, 24u))));
    } else {
        local = bitcast<i32>((bitcast<u32>(extractBits(0i, 0u, 24u)) * bitcast<u32>(extractBits(0i, 0u, 24u))));
    }
    let _e265 = local;
    out[24i] = u32(_e265);
    values[0] = bitcast<i32>((bitcast<u32>(extractBits(2i, 0u, 24u)) * bitcast<u32>(extractBits(3i, 0u, 24u))));
    values[1] = bitcast<i32>((bitcast<u32>(extractBits(3i, 0u, 24u)) * bitcast<u32>(extractBits(4i, 0u, 24u))));
    let _e296 = values[0i];
    let _e297 = values[1i];
    out[25i] = u32((_e296 + _e297));
    Count = 2i;
    scratch = array<i32, 2>();
    let _e308 = Count;
    let _e309 = scratch[0i];
    out[26i] = u32((_e308 + _e309));
    counter = 2i;
    let _e316 = counter;
    counter = (_e316 + 1i);
    let _e319 = counter;
    counter = (_e319 + 1i);
    out[27i] = u32(bitcast<i32>((bitcast<u32>(extractBits(_e316, 0u, 24u)) * bitcast<u32>(extractBits(_e319, 0u, 24u)))));
    let _e333 = counter;
    out[28i] = u32(_e333);
    total = 0i;
    {
        index = 0i;
        loop {
            let _e340 = index;
            if (_e340 < 3i) {
            } else {
                break;
            }
            let _e346 = index;
            let _e357 = total;
            total = (_e357 + bitcast<i32>((bitcast<u32>(extractBits((_e346 + 1i), 0u, 24u)) * bitcast<u32>(extractBits(2i, 0u, 24u)))));
            continuing {
                let _e342 = index;
                index = (_e342 + 1i);
            }
        }
    }
    let _e361 = total;
    out[29i] = u32(_e361);
    {
        out[30i] = u32(i32(9.75f));
    }
    out[31i] = u32(bitcast<i32>((bitcast<u32>(extractBits(16777215i, 0u, 24u)) * bitcast<u32>(extractBits(3i, 0u, 24u)))));
}
