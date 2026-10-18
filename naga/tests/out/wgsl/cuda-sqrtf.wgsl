@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

fn gen__naga_sqrtf(input_bits: u32) -> u32 {
    var significand: u32;
    var adjusted_exponent: u32;
    var upper_word: u32;
    var lower_word: u32;
    var root: u32;
    var remainder: u32;
    var iteration: u32;

    let _e5 = (input_bits & 2147483647u);
    if ((_e5 == 0u) || (input_bits == 2139095040u)) {
        return input_bits;
    }
    if (((input_bits & 2147483648u) != 0u) || (_e5 > 2139095040u)) {
        return 2143289344u;
    }
    let _e19 = (input_bits & 8388607u);
    let _e20 = (_e5 >> 23u);
    significand = (_e19 | 8388608u);
    adjusted_exponent = (_e20 + 127u);
    if (_e20 == 0u) {
        let _e28 = (countLeadingZeros(_e19) - 8u);
        significand = (_e19 << _e28);
        adjusted_exponent = (128u - _e28);
    }
    let _e32 = significand;
    let _e33 = adjusted_exponent;
    let _e34 = (_e33 & 1u);
    upper_word = (_e32 >> (9u - _e34));
    lower_word = (_e32 << (23u + _e34));
    root = 0u;
    remainder = 0u;
    iteration = 0u;
    loop {
        let _e50 = iteration;
        if (_e50 >= 24u) {
            break;
        }
        let _e52 = upper_word;
        let _e53 = lower_word;
        let _e54 = root;
        let _e55 = remainder;
        let _e63 = ((_e55 << 2u) | (_e52 >> 14u));
        let _e65 = ((_e54 << 2u) | 1u);
        let _e66 = (_e63 >= _e65);
        let _e69 = (_e54 << 1u);
        upper_word = (((_e52 << 2u) | (_e53 >> 30u)) & 65535u);
        lower_word = (_e53 << 2u);
        remainder = select(_e63, (_e63 - _e65), _e66);
        root = select(_e69, (_e69 | 1u), _e66);
        continuing {
            let _e72 = iteration;
            iteration = (_e72 + 1u);
        }
    }
    let _e74 = root;
    let _e75 = remainder;
    return (((_e33 >> 1u) << 23u) | (select(_e74, (_e74 + 1u), (_e75 > _e74)) & 8388607u));
}

fn root_1(value_1: f32) -> f32 {
    var value_2: f32;

    value_2 = value_1;
    let _e2 = value_2;
    let _e4 = gen__naga_sqrtf(bitcast<u32>(_e2));
    return bitcast<f32>(_e4);
}

fn root_2(value_3: i32) -> f32 {
    var value_4: i32;

    value_4 = value_3;
    let _e2 = value_4;
    let _e5 = gen__naga_sqrtf(bitcast<u32>(f32(_e2)));
    return bitcast<f32>(_e5);
}

fn root_3(value_5: u32) -> f32 {
    var value_6: u32;

    value_6 = value_5;
    let _e2 = value_6;
    let _e5 = gen__naga_sqrtf(bitcast<u32>(f32(_e2)));
    return bitcast<f32>(_e5);
}

fn root_4(value_7: bool) -> f32 {
    var value_8: bool;

    value_8 = value_7;
    let _e2 = value_8;
    let _e5 = gen__naga_sqrtf(bitcast<u32>(f32(_e2)));
    return bitcast<f32>(_e5);
}

fn sqrtf(value_9: bool) -> i32 {
    var value_10: bool;
    var local_10: i32;

    value_10 = value_9;
    let _e2 = value_10;
    if _e2 {
        local_10 = 7i;
    } else {
        local_10 = 9i;
    }
    let _e6 = local_10;
    return _e6;
}

fn sqrtf_1(first: f32, second: f32) -> f32 {
    var first_1: f32;
    var second_1: f32;

    first_1 = first;
    second_1 = second;
    let _e4 = first_1;
    let _e5 = second_1;
    return (_e4 + _e5);
}

@compute @workgroup_size(1, 1, 1)
fn roots() {
    var local: u32;
    var local_1: u32;
    var local_2: u32;
    var local_3: u32;
    var local_4: u32;
    var local_5: u32;
    var value: f32;
    var local_6: u32;
    var local_7: u32;
    var local_8: u32;
    var local_9: u32;

    let _e5 = gen__naga_sqrtf(bitcast<u32>(0f));
    out[0i] = bitcast<u32>(bitcast<f32>(_e5));
    let _e13 = gen__naga_sqrtf(bitcast<u32>(-(0f)));
    out[1i] = bitcast<u32>(bitcast<f32>(_e13));
    let _e20 = gen__naga_sqrtf(bitcast<u32>(1f));
    out[2i] = bitcast<u32>(bitcast<f32>(_e20));
    let _e26 = root_2(4i);
    out[3i] = bitcast<u32>(_e26);
    let _e31 = root_3(9u);
    out[4i] = bitcast<u32>(_e31);
    let _e37 = gen__naga_sqrtf(bitcast<u32>(6.25f));
    out[5i] = bitcast<u32>(bitcast<f32>(_e37));
    let _e44 = gen__naga_sqrtf(bitcast<u32>(2f));
    out[6i] = bitcast<u32>(bitcast<f32>(_e44));
    local = 1u;
    let _e51 = local;
    let _e54 = gen__naga_sqrtf(bitcast<u32>(bitcast<f32>(_e51)));
    out[7i] = bitcast<u32>(bitcast<f32>(_e54));
    local_1 = 8388607u;
    let _e61 = local_1;
    let _e64 = gen__naga_sqrtf(bitcast<u32>(bitcast<f32>(_e61)));
    out[8i] = bitcast<u32>(bitcast<f32>(_e64));
    local_2 = 8388608u;
    let _e71 = local_2;
    let _e74 = gen__naga_sqrtf(bitcast<u32>(bitcast<f32>(_e71)));
    out[9i] = bitcast<u32>(bitcast<f32>(_e74));
    local_3 = 2139095039u;
    let _e81 = local_3;
    let _e84 = gen__naga_sqrtf(bitcast<u32>(bitcast<f32>(_e81)));
    out[10i] = bitcast<u32>(bitcast<f32>(_e84));
    local_4 = 2139095040u;
    let _e91 = local_4;
    let _e94 = gen__naga_sqrtf(bitcast<u32>(bitcast<f32>(_e91)));
    out[11i] = bitcast<u32>(bitcast<f32>(_e94));
    let _e102 = gen__naga_sqrtf(bitcast<u32>(-(1f)));
    out[12i] = bitcast<u32>(bitcast<f32>(_e102));
    local_5 = 2139095041u;
    let _e109 = local_5;
    let _e112 = gen__naga_sqrtf(bitcast<u32>(bitcast<f32>(_e109)));
    out[13i] = bitcast<u32>(bitcast<f32>(_e112));
    value = 4f;
    let _e119 = value;
    value = (_e119 + f32(1i));
    let _e123 = root_1(_e119);
    out[14i] = bitcast<u32>(_e123);
    let _e127 = value;
    out[15i] = bitcast<u32>(_e127);
    let _e132 = sqrtf(true);
    out[16i] = u32(_e132);
    let _e137 = sqrtf(false);
    out[17i] = u32(_e137);
    let _e143 = sqrtf_1(2f, 3f);
    out[18i] = bitcast<u32>(_e143);
    local_6 = 1065353217u;
    let _e149 = local_6;
    let _e152 = gen__naga_sqrtf(bitcast<u32>(bitcast<f32>(_e149)));
    out[19i] = bitcast<u32>(bitcast<f32>(_e152));
    local_7 = 1065353218u;
    let _e159 = local_7;
    let _e162 = gen__naga_sqrtf(bitcast<u32>(bitcast<f32>(_e159)));
    out[20i] = bitcast<u32>(bitcast<f32>(_e162));
    local_8 = 1065353215u;
    let _e169 = local_8;
    let _e172 = gen__naga_sqrtf(bitcast<u32>(bitcast<f32>(_e169)));
    out[21i] = bitcast<u32>(bitcast<f32>(_e172));
    let _e178 = root_3(4294967295u);
    out[22i] = bitcast<u32>(_e178);
    let _e184 = root_2(-(4i));
    out[23i] = bitcast<u32>(_e184);
    let _e189 = root_1(16f);
    out[24i] = bitcast<u32>(_e189);
    let _e195 = gen__naga_sqrtf(bitcast<u32>(0.25f));
    out[25i] = bitcast<u32>(bitcast<f32>(_e195));
    local_9 = 2147483649u;
    let _e202 = local_9;
    let _e205 = gen__naga_sqrtf(bitcast<u32>(bitcast<f32>(_e202)));
    out[26i] = bitcast<u32>(bitcast<f32>(_e205));
    let _e211 = root_4(false);
    out[27i] = bitcast<u32>(_e211);
    let _e216 = root_4(true);
    out[28i] = bitcast<u32>(_e216);
    let _e221 = root_1(0.5f);
    out[29i] = bitcast<u32>(_e221);
}
