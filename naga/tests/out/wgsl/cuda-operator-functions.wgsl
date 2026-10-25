struct Record_cuda_record_0_ {
    value: i32,
    history: array<i32, 2>,
}

@group(0) @binding(0)
var<storage> input: array<vec2<i32>>;
@group(0) @binding(1)
var<storage, read_write> out: array<u32>;

fn operator_u002b(value: vec2<i32>) -> vec2<i32> {
    var value_1: vec2<i32>;

    value_1 = value;
    let _e3 = value_1.x;
    let _e5 = value_1.y;
    return vec2<i32>(_e3, _e5);
}

fn operator_u002d(value_2: vec2<i32>) -> vec2<i32> {
    var value_3: vec2<i32>;

    value_3 = value_2;
    let _e3 = value_3.x;
    let _e6 = value_3.y;
    return vec2<i32>(-(_e3), -(_e6));
}

fn operator_u007e(value_4: vec2<i32>) -> vec2<i32> {
    var value_5: vec2<i32>;

    value_5 = value_4;
    let _e3 = value_5.x;
    let _e6 = value_5.y;
    return vec2<i32>(~(_e3), ~(_e6));
}

fn operator_u0021_(value_6: vec2<i32>) -> bool {
    var value_7: vec2<i32>;
    var local_1: bool;

    value_7 = value_6;
    let _e4 = value_7.x;
    let _e5 = (_e4 == 0i);
    local_1 = _e5;
    if _e5 {
        let _e9 = value_7.y;
        local_1 = (_e9 == 0i);
    }
    let _e11 = local_1;
    return _e11;
}

fn operator_u002a(value_8: vec2<i32>) -> i32 {
    var value_9: vec2<i32>;

    value_9 = value_8;
    let _e4 = value_9.x;
    let _e7 = value_9.y;
    return ((_e4 * 10i) + _e7);
}

fn operator_u0026_(value_10: vec2<i32>) -> vec2<i32> {
    var value_11: vec2<i32>;

    value_11 = value_10;
    let _e3 = value_11.y;
    let _e5 = value_11.x;
    return vec2<i32>(_e3, _e5);
}

fn operator_u002b_1(left_1: vec2<i32>, right_1: vec2<i32>) -> vec2<i32> {
    var left_2: vec2<i32>;
    var right_2: vec2<i32>;

    left_2 = left_1;
    right_2 = right_1;
    let _e6 = left_2.x;
    let _e7 = right_2.x;
    let _e11 = left_2.y;
    let _e12 = right_2.y;
    return vec2<i32>((_e6 + _e7), (_e11 + _e12));
}

fn operator_u002d_1(left_3: vec2<i32>, right_3: vec2<i32>) -> vec2<i32> {
    var left_4: vec2<i32>;
    var right_4: vec2<i32>;

    left_4 = left_3;
    right_4 = right_3;
    let _e6 = left_4.x;
    let _e7 = right_4.x;
    let _e11 = left_4.y;
    let _e12 = right_4.y;
    return vec2<i32>((_e6 - _e7), (_e11 - _e12));
}

fn operator_u002a_1(left_5: vec2<i32>, right_5: vec2<i32>) -> vec2<i32> {
    var left_6: vec2<i32>;
    var right_6: vec2<i32>;

    left_6 = left_5;
    right_6 = right_5;
    let _e6 = left_6.x;
    let _e7 = right_6.x;
    let _e11 = left_6.y;
    let _e12 = right_6.y;
    return vec2<i32>((_e6 * _e7), (_e11 * _e12));
}

fn operator_u002f(left_7: vec2<i32>, right_7: vec2<i32>) -> vec2<i32> {
    var left_8: vec2<i32>;
    var right_8: vec2<i32>;

    left_8 = left_7;
    right_8 = right_7;
    let _e6 = left_8.x;
    let _e7 = right_8.x;
    let _e11 = left_8.y;
    let _e12 = right_8.y;
    return vec2<i32>((_e6 / _e7), (_e11 / _e12));
}

fn operator_u0025_(left_9: vec2<i32>, right_9: vec2<i32>) -> vec2<i32> {
    var left_10: vec2<i32>;
    var right_10: vec2<i32>;

    left_10 = left_9;
    right_10 = right_9;
    let _e6 = left_10.x;
    let _e7 = right_10.x;
    let _e11 = left_10.y;
    let _e12 = right_10.y;
    return vec2<i32>((_e6 % _e7), (_e11 % _e12));
}

fn operator_u0026_1(left_11: vec2<i32>, right_11: vec2<i32>) -> vec2<i32> {
    var left_12: vec2<i32>;
    var right_12: vec2<i32>;

    left_12 = left_11;
    right_12 = right_11;
    let _e6 = left_12.x;
    let _e7 = right_12.x;
    let _e11 = left_12.y;
    let _e12 = right_12.y;
    return vec2<i32>((_e6 & _e7), (_e11 & _e12));
}

fn operator_u007c(left_13: vec2<i32>, right_13: vec2<i32>) -> vec2<i32> {
    var left_14: vec2<i32>;
    var right_14: vec2<i32>;

    left_14 = left_13;
    right_14 = right_13;
    let _e6 = left_14.x;
    let _e7 = right_14.x;
    let _e11 = left_14.y;
    let _e12 = right_14.y;
    return vec2<i32>((_e6 | _e7), (_e11 | _e12));
}

fn operator_u005e(left_15: vec2<i32>, right_15: vec2<i32>) -> vec2<i32> {
    var left_16: vec2<i32>;
    var right_16: vec2<i32>;

    left_16 = left_15;
    right_16 = right_15;
    let _e6 = left_16.x;
    let _e7 = right_16.x;
    let _e11 = left_16.y;
    let _e12 = right_16.y;
    return vec2<i32>((_e6 ^ _e7), (_e11 ^ _e12));
}

fn operator_(left_17: vec2<i32>, right_17: vec2<i32>) -> vec2<i32> {
    var left_18: vec2<i32>;
    var right_18: vec2<i32>;
    var local_2: u32;
    var local_3: u32;

    left_18 = left_17;
    right_18 = right_17;
    let _e5 = left_18.x;
    let _e7 = right_18.x;
    local_2 = u32(_e7);
    let _e10 = local_2;
    let _e13 = left_18.y;
    let _e15 = right_18.y;
    local_3 = u32(_e15);
    let _e18 = local_3;
    return vec2<i32>((_e5 << _e10), (_e13 << _e18));
}

fn operator_1(left_19: vec2<i32>, right_19: vec2<i32>) -> vec2<i32> {
    var left_20: vec2<i32>;
    var right_20: vec2<i32>;

    left_20 = left_19;
    right_20 = right_19;
    let _e5 = left_20.x;
    let _e7 = right_20.x;
    let _e11 = left_20.y;
    let _e13 = right_20.y;
    return vec2<i32>((_e5 >> u32(_e7)), (_e11 >> u32(_e13)));
}

fn operator_u002a_2(value_12: Record_cuda_record_0_) -> Record_cuda_record_0_ {
    var value_13: Record_cuda_record_0_;

    value_13 = value_12;
    let _e3 = value_13.value;
    value_13.value = (_e3 + 1i);
    let _e6 = value_13;
    return _e6;
}

fn operator_u0026_2(value_14: Record_cuda_record_0_) -> i32 {
    var value_15: Record_cuda_record_0_;

    value_15 = value_14;
    let _e3 = value_15.value;
    return _e3;
}

fn operator_u002d_2(value_16: Record_cuda_record_0_) -> Record_cuda_record_0_ {
    var value_17: Record_cuda_record_0_;

    value_17 = value_16;
    let _e4 = value_17.value;
    value_17.value = -(_e4);
    let _e6 = value_17;
    return _e6;
}

fn operator_u002a_3(value_18: Record_cuda_record_0_, factor: i32) -> Record_cuda_record_0_ {
    var value_19: Record_cuda_record_0_;
    var factor_1: i32;

    value_19 = value_18;
    factor_1 = factor;
    let _e5 = factor_1;
    let _e6 = value_19.value;
    value_19.value = (_e6 * _e5);
    let _e11 = factor_1;
    let _e12 = value_19.history[1i];
    value_19.history[1i] = (_e12 + _e11);
    let _e14 = value_19;
    return _e14;
}

@compute @workgroup_size(2, 1, 1)
fn operator_functions(@builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var base: u32;
    var left: vec2<i32>;
    var right: vec2<i32>;
    var original: Record_cuda_record_0_;
    var scaled: Record_cuda_record_0_;
    var local: Record_cuda_record_0_;

    base = (threadIdx.x * 36u);
    let _e10 = input[threadIdx.x];
    left = _e10;
    right = vec2<i32>(3i, 2i);
    let _e19 = left;
    let _e20 = operator_u002b(_e19);
    let _e16 = base;
    out[(_e16 + 0u)] = u32(_e20.x);
    let _e27 = left;
    let _e28 = operator_u002d(_e27);
    let _e24 = base;
    out[(_e24 + 1u)] = u32(_e28.y);
    let _e35 = left;
    let _e36 = operator_u007e(_e35);
    let _e32 = base;
    out[(_e32 + 2u)] = u32(_e36.x);
    let _e43 = left;
    let _e44 = operator_u0021_(_e43);
    let _e40 = base;
    out[(_e40 + 3u)] = u32(_e44);
    let _e50 = left;
    let _e51 = operator_u002a(_e50);
    let _e47 = base;
    out[(_e47 + 4u)] = u32(_e51);
    let _e57 = left;
    let _e58 = operator_u0026_(_e57);
    let _e54 = base;
    out[(_e54 + 5u)] = u32(_e58.x);
    let _e65 = left;
    let _e66 = right;
    let _e67 = operator_u002b_1(_e65, _e66);
    let _e62 = base;
    out[(_e62 + 6u)] = u32(_e67.x);
    let _e74 = left;
    let _e75 = right;
    let _e76 = operator_u002d_1(_e74, _e75);
    let _e71 = base;
    out[(_e71 + 7u)] = u32(_e76.y);
    let _e83 = left;
    let _e84 = right;
    let _e85 = operator_u002a_1(_e83, _e84);
    let _e80 = base;
    out[(_e80 + 8u)] = u32(_e85.x);
    let _e92 = left;
    let _e93 = right;
    let _e94 = operator_u002f(_e92, _e93);
    let _e89 = base;
    out[(_e89 + 9u)] = u32(_e94.y);
    let _e101 = left;
    let _e102 = right;
    let _e103 = operator_u0025_(_e101, _e102);
    let _e98 = base;
    out[(_e98 + 10u)] = u32(_e103.y);
    let _e110 = left;
    let _e111 = right;
    let _e112 = operator_u0026_1(_e110, _e111);
    let _e107 = base;
    out[(_e107 + 11u)] = u32(_e112.x);
    let _e119 = left;
    let _e120 = right;
    let _e121 = operator_u007c(_e119, _e120);
    let _e116 = base;
    out[(_e116 + 12u)] = u32(_e121.y);
    let _e128 = left;
    let _e129 = right;
    let _e130 = operator_u005e(_e128, _e129);
    let _e125 = base;
    out[(_e125 + 13u)] = u32(_e130.x);
    let _e137 = left;
    let _e138 = right;
    let _e139 = operator_(_e137, _e138);
    let _e134 = base;
    out[(_e134 + 14u)] = u32(_e139.x);
    let _e146 = left;
    let _e147 = right;
    let _e148 = operator_1(_e146, _e147);
    let _e143 = base;
    out[(_e143 + 15u)] = u32(_e148.y);
    let _e155 = left;
    let _e156 = operator_u002b(_e155);
    let _e152 = base;
    out[(_e152 + 16u)] = u32(_e156.y);
    let _e163 = left;
    let _e164 = right;
    let _e165 = operator_u002d_1(_e163, _e164);
    let _e160 = base;
    out[(_e160 + 17u)] = u32(_e165.x);
    let _e172 = left;
    let _e173 = operator_u002a(_e172);
    let _e169 = base;
    out[(_e169 + 18u)] = u32(_e173);
    let _e179 = left;
    let _e180 = right;
    let _e181 = operator_u0026_1(_e179, _e180);
    let _e176 = base;
    out[(_e176 + 19u)] = u32(_e181.y);
    let _e188 = left;
    let _e189 = operator_u0026_(_e188);
    let _e190 = operator_u002a(_e189);
    let _e185 = base;
    out[(_e185 + 20u)] = u32(_e190);
    let _e196 = left;
    let _e197 = operator_u0026_(_e196);
    let _e198 = operator_u0026_(_e197);
    let _e193 = base;
    out[(_e193 + 21u)] = u32(_e198.x);
    let _e208 = left;
    let _e205 = right;
    let _e206 = right;
    let _e207 = operator_u002a_1(_e205, _e206);
    let _e209 = operator_u002b_1(_e208, _e207);
    let _e202 = base;
    out[(_e202 + 22u)] = u32(_e209.x);
    let _e216 = left;
    let _e220 = right;
    let _e221 = operator_u002b_1(_e220, vec2<i32>(1i, 1i));
    let _e222 = operator_(_e216, _e221);
    let _e213 = base;
    out[(_e213 + 23u)] = u32(_e222.x);
    let _e234 = left;
    let _e232 = right;
    let _e233 = operator_u0026_1(_e232, vec2<i32>(1i, 1i));
    let _e235 = operator_u007c(_e234, _e233);
    let _e226 = base;
    out[(_e226 + 24u)] = u32(_e235.y);
    let _e242 = left;
    let _e243 = right;
    let _e244 = operator_u002d_1(_e242, _e243);
    let _e245 = right;
    let _e246 = operator_u002d_1(_e244, _e245);
    let _e239 = base;
    out[(_e239 + 25u)] = u32(_e246.x);
    original.value = 9i;
    original.history[0] = 1i;
    original.history[1] = 2i;
    let _e259 = original;
    let _e260 = operator_u002a_3(_e259, 3i);
    scaled = _e260;
    local.value = 9i;
    local.history[0] = 1i;
    local.history[1] = 2i;
    let _e273 = local;
    let _e274 = operator_u0026_2(_e273);
    let _e262 = base;
    out[(_e262 + 26u)] = u32(_e274);
    let _e280 = original;
    let _e281 = operator_u002a_2(_e280);
    let _e282 = operator_u0026_2(_e281);
    let _e277 = base;
    out[(_e277 + 27u)] = u32(_e282);
    let _e289 = scaled.value;
    let _e285 = base;
    out[(_e285 + 28u)] = u32(_e289);
    let _e298 = scaled.history[1i];
    let _e292 = base;
    out[(_e292 + 29u)] = u32(_e298);
    let _e305 = original.value;
    let _e301 = base;
    out[(_e301 + 30u)] = u32(_e305);
    let _e314 = original.history[1i];
    let _e308 = base;
    out[(_e308 + 31u)] = u32(_e314);
    let _e320 = original;
    let _e321 = operator_u002d_2(_e320);
    let _e317 = base;
    out[(_e317 + 32u)] = u32(_e321.value);
    let _e328 = original;
    let _e330 = operator_u002a_3(_e328, 2i);
    let _e325 = base;
    out[(_e325 + 33u)] = u32(_e330.value);
    let _e337 = original;
    let _e338 = operator_u002a_2(_e337);
    let _e339 = operator_u0026_2(_e338);
    let _e334 = base;
    out[(_e334 + 34u)] = u32(_e339);
    let _e348 = operator_u0021_(vec2<i32>(0i, 0i));
    let _e342 = base;
    out[(_e342 + 35u)] = u32(_e348);
}
