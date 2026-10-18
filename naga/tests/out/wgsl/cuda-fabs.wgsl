struct float3_ {
    x: f32,
    y: f32,
    z: f32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

fn fabs(value_1: vec2<f32>) -> vec2<f32> {
    var value_2: vec2<f32>;
    var local_23: u32;
    var local_24: u32;

    value_2 = value_1;
    let _e3 = value_2.x;
    let _e6 = (bitcast<u32>(_e3) & 2147483647u);
    local_23 = select(_e6, 2143289344u, (_e6 > 2139095040u));
    let _e12 = local_23;
    let _e15 = value_2.y;
    let _e18 = (bitcast<u32>(_e15) & 2147483647u);
    local_24 = select(_e18, 2143289344u, (_e18 > 2139095040u));
    let _e24 = local_24;
    return vec2<f32>(bitcast<f32>(_e12), bitcast<f32>(_e24));
}

fn fabs_1(value_3: float3_) -> float3_ {
    var value_4: float3_;
    var local_25: u32;
    var local_26: u32;
    var local_27: u32;

    value_4 = value_3;
    let _e3 = value_4.x;
    let _e6 = (bitcast<u32>(_e3) & 2147483647u);
    local_25 = select(_e6, 2143289344u, (_e6 > 2139095040u));
    let _e12 = local_25;
    let _e15 = value_4.y;
    let _e18 = (bitcast<u32>(_e15) & 2147483647u);
    local_26 = select(_e18, 2143289344u, (_e18 > 2139095040u));
    let _e24 = local_26;
    let _e27 = value_4.z;
    let _e30 = (bitcast<u32>(_e27) & 2147483647u);
    local_27 = select(_e30, 2143289344u, (_e30 > 2139095040u));
    let _e36 = local_27;
    return float3_(bitcast<f32>(_e12), bitcast<f32>(_e24), bitcast<f32>(_e36));
}

fn fabs_2(value_5: vec4<f32>) -> vec4<f32> {
    var value_6: vec4<f32>;
    var local_28: u32;
    var local_29: u32;
    var local_30: u32;
    var local_31: u32;

    value_6 = value_5;
    let _e3 = value_6.x;
    let _e6 = (bitcast<u32>(_e3) & 2147483647u);
    local_28 = select(_e6, 2143289344u, (_e6 > 2139095040u));
    let _e12 = local_28;
    let _e15 = value_6.y;
    let _e18 = (bitcast<u32>(_e15) & 2147483647u);
    local_29 = select(_e18, 2143289344u, (_e18 > 2139095040u));
    let _e24 = local_29;
    let _e27 = value_6.z;
    let _e30 = (bitcast<u32>(_e27) & 2147483647u);
    local_30 = select(_e30, 2143289344u, (_e30 > 2139095040u));
    let _e36 = local_30;
    let _e39 = value_6.w;
    let _e42 = (bitcast<u32>(_e39) & 2147483647u);
    local_31 = select(_e42, 2143289344u, (_e42 > 2139095040u));
    let _e48 = local_31;
    return vec4<f32>(bitcast<f32>(_e12), bitcast<f32>(_e24), bitcast<f32>(_e36), bitcast<f32>(_e48));
}

fn absolute(value_7: f32) -> f32 {
    var value_8: f32;
    var local_32: u32;

    value_8 = value_7;
    let _e2 = value_8;
    let _e5 = (bitcast<u32>(_e2) & 2147483647u);
    local_32 = select(_e5, 2143289344u, (_e5 > 2139095040u));
    let _e11 = local_32;
    return bitcast<f32>(_e11);
}

fn converted_absolute(value_9: i32) -> f32 {
    var value_10: i32;
    var local_33: u32;

    value_10 = value_9;
    let _e2 = value_10;
    let _e6 = (bitcast<u32>(f32(_e2)) & 2147483647u);
    local_33 = select(_e6, 2143289344u, (_e6 > 2139095040u));
    let _e12 = local_33;
    return bitcast<f32>(_e12);
}

fn converted_absolute_1(value_11: u32) -> f32 {
    var value_12: u32;
    var local_34: u32;

    value_12 = value_11;
    let _e2 = value_12;
    let _e6 = (bitcast<u32>(f32(_e2)) & 2147483647u);
    local_34 = select(_e6, 2143289344u, (_e6 > 2139095040u));
    let _e12 = local_34;
    return bitcast<f32>(_e12);
}

fn converted_absolute_2(value_13: bool) -> f32 {
    var value_14: bool;
    var local_35: u32;

    value_14 = value_13;
    let _e2 = value_14;
    let _e6 = (bitcast<u32>(f32(_e2)) & 2147483647u);
    local_35 = select(_e6, 2143289344u, (_e6 > 2139095040u));
    let _e12 = local_35;
    return bitcast<f32>(_e12);
}

fn fabs_3(value_15: i32) -> i32 {
    var value_16: i32;

    value_16 = value_15;
    let _e3 = value_16;
    return (_e3 + 40i);
}

fn fabs_4(value_17: u32) -> u32 {
    var value_18: u32;

    value_18 = value_17;
    let _e3 = value_18;
    return (_e3 + 50u);
}

fn fabs_5(value_19: bool) -> i32 {
    var value_20: bool;
    var local_36: i32;

    value_20 = value_19;
    let _e2 = value_20;
    if _e2 {
        local_36 = 7i;
    } else {
        local_36 = 9i;
    }
    let _e6 = local_36;
    return _e6;
}

fn fabsf(value_21: bool) -> i32 {
    var value_22: bool;
    var local_37: i32;

    value_22 = value_21;
    let _e2 = value_22;
    if _e2 {
        local_37 = 11i;
    } else {
        local_37 = 13i;
    }
    let _e6 = local_37;
    return _e6;
}

fn fabsf_1(first: f32, second: f32) -> f32 {
    var first_1: f32;
    var second_1: f32;

    first_1 = first;
    second_1 = second;
    let _e4 = first_1;
    let _e5 = second_1;
    return (_e4 + _e5);
}

@compute @workgroup_size(1, 1, 1)
fn absolutes() {
    var local: u32;
    var local_1: u32;
    var local_2: u32;
    var local_3: u32;
    var local_4: u32;
    var local_5: u32;
    var local_6: u32;
    var local_7: u32;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var local_11: u32;
    var local_12: u32;
    var local_13: u32;
    var local_14: u32;
    var local_15: u32;
    var local_16: u32;
    var local_17: u32;
    var local_18: u32;
    var local_19: u32;
    var local_20: u32;
    var value: f32;
    var local_21: u32;
    var triple: float3_;
    var local_22: u32;
    var pair: vec2<f32>;
    var quadruple: vec4<f32>;

    let _e6 = (bitcast<u32>(0f) & 2147483647u);
    local = select(_e6, 2143289344u, (_e6 > 2139095040u));
    let _e12 = local;
    out[0i] = bitcast<u32>(bitcast<f32>(_e12));
    let _e21 = (bitcast<u32>(-(0f)) & 2147483647u);
    local_1 = select(_e21, 2143289344u, (_e21 > 2139095040u));
    let _e27 = local_1;
    out[1i] = bitcast<u32>(bitcast<f32>(_e27));
    let _e36 = (bitcast<u32>(-(1.75f)) & 2147483647u);
    local_2 = select(_e36, 2143289344u, (_e36 > 2139095040u));
    let _e42 = local_2;
    out[2i] = bitcast<u32>(bitcast<f32>(_e42));
    let _e49 = absolute(-(1.25f));
    out[3i] = bitcast<u32>(_e49);
    let _e55 = converted_absolute(-(3i));
    out[4i] = bitcast<u32>(_e55);
    let _e60 = converted_absolute_1(4294967295u);
    out[5i] = bitcast<u32>(_e60);
    let _e65 = converted_absolute_2(true);
    out[6i] = bitcast<u32>(_e65);
    let _e70 = converted_absolute_2(false);
    out[7i] = bitcast<u32>(_e70);
    local_3 = 1u;
    let _e76 = local_3;
    let _e80 = (bitcast<u32>(bitcast<f32>(_e76)) & 2147483647u);
    local_4 = select(_e80, 2143289344u, (_e80 > 2139095040u));
    let _e86 = local_4;
    out[8i] = bitcast<u32>(bitcast<f32>(_e86));
    local_5 = 2147483649u;
    let _e93 = local_5;
    let _e97 = (bitcast<u32>(bitcast<f32>(_e93)) & 2147483647u);
    local_6 = select(_e97, 2143289344u, (_e97 > 2139095040u));
    let _e103 = local_6;
    out[9i] = bitcast<u32>(bitcast<f32>(_e103));
    local_7 = 8388607u;
    let _e110 = local_7;
    let _e114 = (bitcast<u32>(bitcast<f32>(_e110)) & 2147483647u);
    local_8 = select(_e114, 2143289344u, (_e114 > 2139095040u));
    let _e120 = local_8;
    out[10i] = bitcast<u32>(bitcast<f32>(_e120));
    local_9 = 2155872255u;
    let _e127 = local_9;
    let _e131 = (bitcast<u32>(bitcast<f32>(_e127)) & 2147483647u);
    local_10 = select(_e131, 2143289344u, (_e131 > 2139095040u));
    let _e137 = local_10;
    out[11i] = bitcast<u32>(bitcast<f32>(_e137));
    let _e145 = (bitcast<u32>(340282350000000000000000000000000000000f) & 2147483647u);
    local_11 = select(_e145, 2143289344u, (_e145 > 2139095040u));
    let _e151 = local_11;
    out[12i] = bitcast<u32>(bitcast<f32>(_e151));
    let _e160 = (bitcast<u32>(-(340282350000000000000000000000000000000f)) & 2147483647u);
    local_12 = select(_e160, 2143289344u, (_e160 > 2139095040u));
    let _e166 = local_12;
    out[13i] = bitcast<u32>(bitcast<f32>(_e166));
    local_13 = 2139095040u;
    let _e173 = local_13;
    let _e177 = (bitcast<u32>(bitcast<f32>(_e173)) & 2147483647u);
    local_14 = select(_e177, 2143289344u, (_e177 > 2139095040u));
    let _e183 = local_14;
    out[14i] = bitcast<u32>(bitcast<f32>(_e183));
    local_15 = 4286578688u;
    let _e190 = local_15;
    let _e194 = (bitcast<u32>(bitcast<f32>(_e190)) & 2147483647u);
    local_16 = select(_e194, 2143289344u, (_e194 > 2139095040u));
    let _e200 = local_16;
    out[15i] = bitcast<u32>(bitcast<f32>(_e200));
    local_17 = 2143363909u;
    let _e207 = local_17;
    let _e211 = (bitcast<u32>(bitcast<f32>(_e207)) & 2147483647u);
    local_18 = select(_e211, 2143289344u, (_e211 > 2139095040u));
    let _e217 = local_18;
    out[16i] = bitcast<u32>(bitcast<f32>(_e217));
    local_19 = 2139095041u;
    let _e224 = local_19;
    let _e228 = (bitcast<u32>(bitcast<f32>(_e224)) & 2147483647u);
    local_20 = select(_e228, 2143289344u, (_e228 > 2139095040u));
    let _e234 = local_20;
    out[17i] = bitcast<u32>(bitcast<f32>(_e234));
    value = -(2.75f);
    let _e242 = value;
    value = (_e242 + f32(1i));
    let _e248 = (bitcast<u32>(_e242) & 2147483647u);
    local_21 = select(_e248, 2143289344u, (_e248 > 2139095040u));
    let _e254 = local_21;
    out[18i] = bitcast<u32>(bitcast<f32>(_e254));
    let _e259 = value;
    out[19i] = bitcast<u32>(_e259);
    let _e265 = fabs_3(-(3i));
    out[20i] = u32(_e265);
    let _e270 = fabs_4(3u);
    out[21i] = _e270;
    let _e274 = fabs_5(true);
    out[22i] = u32(_e274);
    let _e279 = fabs_5(false);
    out[23i] = u32(_e279);
    let _e284 = fabsf(true);
    out[24i] = u32(_e284);
    local_22 = 2147483649u;
    let _e293 = local_22;
    let _e296 = fabs_1(float3_(-(2.5f), -(0f), bitcast<f32>(_e293)));
    triple = _e296;
    let _e300 = triple.x;
    out[25i] = bitcast<u32>(_e300);
    let _e305 = triple.y;
    out[26i] = bitcast<u32>(_e305);
    let _e310 = triple.z;
    out[27i] = bitcast<u32>(_e310);
    let _e317 = fabsf_1(-(2f), 3f);
    out[28i] = bitcast<u32>(_e317);
    let _e325 = fabs(vec2<f32>(-(3f), -(4f)));
    pair = _e325;
    let _e336 = fabs_2(vec4<f32>(-(1f), -(2f), -(3f), -(4f)));
    quadruple = _e336;
    let _e341 = pair.x;
    let _e342 = pair.y;
    let _e345 = quadruple.x;
    let _e348 = quadruple.w;
    out[29i] = bitcast<u32>((((_e341 + _e342) + _e345) + _e348));
}
