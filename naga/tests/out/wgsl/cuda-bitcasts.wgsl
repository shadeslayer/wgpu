@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

fn from_bits(value_1: u32) -> f32 {
    var value_2: u32;
    var local_13: u32;

    value_2 = value_1;
    let _e2 = value_2;
    local_13 = _e2;
    let _e4 = local_13;
    return bitcast<f32>(_e4);
}

fn _float_as_int(first: i32, second: i32) -> i32 {
    var first_1: i32;
    var second_1: i32;

    first_1 = first;
    second_1 = second;
    let _e4 = first_1;
    let _e5 = second_1;
    return (_e4 + _e5);
}

@compute @workgroup_size(1, 1, 1)
fn bitcasts() {
    var local: i32;
    var local_1: i32;
    var local_2: i32;
    var local_3: i32;
    var local_4: u32;
    var local_5: u32;
    var local_6: u32;
    var local_7: u32;
    var value: f32;
    var signed_bits: i32;
    var local_8: i32;
    var unsigned_bits: u32;
    var local_9: u32;
    var copied: f32;
    var local_10: i32;
    var local_11: u32;
    var local_12: u32;

    out[0i] = u32(bitcast<i32>(f32(true)));
    out[1i] = u32(bitcast<i32>(f32(3i)));
    out[2i] = u32(bitcast<i32>(f32(5u)));
    out[3i] = u32(bitcast<i32>(2.75f));
    out[4i] = bitcast<u32>(f32(true));
    out[5i] = bitcast<u32>(f32(3i));
    out[6i] = bitcast<u32>(f32(5u));
    out[7i] = bitcast<u32>(2.75f);
    local = i32(true);
    let _e48 = local;
    out[8i] = bitcast<u32>(bitcast<f32>(_e48));
    local_1 = 3i;
    let _e55 = local_1;
    out[9i] = bitcast<u32>(bitcast<f32>(_e55));
    local_2 = i32(5u);
    let _e63 = local_2;
    out[10i] = bitcast<u32>(bitcast<f32>(_e63));
    local_3 = i32(2.75f);
    let _e71 = local_3;
    out[11i] = bitcast<u32>(bitcast<f32>(_e71));
    local_4 = u32(true);
    let _e79 = local_4;
    out[12i] = bitcast<u32>(bitcast<f32>(_e79));
    local_5 = u32(3i);
    let _e87 = local_5;
    out[13i] = bitcast<u32>(bitcast<f32>(_e87));
    local_6 = 5u;
    let _e94 = local_6;
    out[14i] = bitcast<u32>(bitcast<f32>(_e94));
    local_7 = u32(2.75f);
    let _e102 = local_7;
    out[15i] = bitcast<u32>(bitcast<f32>(_e102));
    value = 1.5f;
    let _e109 = value;
    value = (_e109 + f32(1i));
    out[16i] = u32(bitcast<i32>(_e109));
    let _e117 = value;
    out[17i] = bitcast<u32>(_e117);
    let _e121 = value;
    value = (_e121 + f32(1i));
    out[18i] = bitcast<u32>(_e121);
    let _e128 = value;
    out[19i] = bitcast<u32>(_e128);
    signed_bits = 1065353216i;
    let _e134 = signed_bits;
    signed_bits = (_e134 + 1i);
    local_8 = _e134;
    let _e138 = local_8;
    out[20i] = bitcast<u32>(bitcast<f32>(_e138));
    let _e143 = signed_bits;
    out[21i] = u32(_e143);
    unsigned_bits = 1073741824u;
    let _e149 = unsigned_bits;
    unsigned_bits = (_e149 + u32(1i));
    local_9 = _e149;
    let _e154 = local_9;
    out[22i] = bitcast<u32>(bitcast<f32>(_e154));
    let _e159 = unsigned_bits;
    out[23i] = _e159;
    let _e164 = _float_as_int(3i, 4i);
    out[24i] = u32(_e164);
    let _e168 = from_bits(3212836864u);
    copied = _e168;
    let _e171 = copied;
    out[25i] = bitcast<u32>(_e171);
    local_10 = 2139095040i;
    let _e177 = local_10;
    out[26i] = bitcast<u32>(bitcast<f32>(_e177));
    local_11 = 2143363909u;
    let _e184 = local_11;
    out[27i] = bitcast<u32>(bitcast<f32>(_e184));
    local_12 = u32(1065353200f);
    let _e192 = local_12;
    out[28i] = bitcast<u32>(bitcast<f32>(_e192));
    out[29i] = u32(bitcast<i32>(f32(1u)));
}
