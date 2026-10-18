@group(0) @binding(0)
var<storage, read_write> out: array<i32>;

fn min_(first_1: i32, second_1: i32, third: i32) -> i32 {
    var first_2: i32;
    var second_2: i32;
    var third_1: i32;

    first_2 = first_1;
    second_2 = second_1;
    third_1 = third;
    let _e6 = first_2;
    let _e7 = second_2;
    let _e9 = third_1;
    return (((_e6 + _e7) + _e9) + 100i);
}

fn bounded(value_1: i32) -> i32 {
    var value_2: i32;

    value_2 = value_1;
    let _e2 = value_2;
    return min(max(_e2, -(2i)), 2i);
}

@compute @workgroup_size(1, 1, 1)
fn integer_minmax() {
    var seed: i32;
    var first: i32;
    var second: i32;
    var value: i32;

    let _e4 = out[0i];
    seed = _e4;
    let _e7 = seed;
    out[0i] = min(_e7, -(7i));
    let _e13 = seed;
    out[1i] = max(_e13, -(7i));
    out[2i] = i32(min(3u, 9u));
    out[3i] = i32(max(3u, 9u));
    out[4i] = i32((min(u32(-(1i)), 2u) == 2u));
    out[5i] = i32((max(u32(-(1i)), 2u) == 4294967295u));
    out[6i] = i32((min(2u, u32(-(1i))) == 2u));
    out[7i] = i32((max(2u, u32(-(1i))) == 4294967295u));
    out[8i] = min(i32(true), i32(false));
    out[9i] = max(i32(false), i32(true));
    let _e85 = seed;
    out[10i] = min(max(_e85, -(5i)), 2i);
    out[11i] = i32((min(u32(-(2i)), 4u) == 4u));
    let _e103 = seed;
    let _e106 = min_(_e103, 4i, 5i);
    out[12i] = _e106;
    let _e109 = seed;
    let _e110 = bounded(_e109);
    out[13i] = _e110;
    first = 3i;
    second = 7i;
    let _e117 = first;
    first = (_e117 + 1i);
    let _e120 = second;
    second = (_e120 + 1i);
    out[14i] = max(_e117, _e120);
    let _e126 = first;
    out[15i] = _e126;
    let _e129 = second;
    out[16i] = _e129;
    let _e131 = seed;
    value = _e131;
    let _e134 = value;
    let _e135 = value;
    value = (_e135 + 1i);
    out[17i] = max(_e134, _e135);
    let _e141 = value;
    out[18i] = _e141;
    out[19i] = i32((min((-(2147483647i) - 1i), 2147483647i) == (-(2147483647i) - 1i)));
    out[20i] = i32((max(4294967295u, 0u) == 4294967295u));
    out[21i] = i32((min(u32(false), 3u) == 0u));
    out[22i] = i32((max(u32(true), 0u) == 1u));
    let _e184 = seed;
    let _e185 = seed;
    out[23i] = min(_e184, _e185);
}
