#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 2, local_size_y = 1, local_size_z = 1) in;

struct Record_cuda_record_0_ {
    int value;
    int history[2];
};
layout(std430) readonly buffer type_4_block_0Compute { ivec2 _group_0_binding_0_cs[]; };

layout(std430) buffer type_6_block_1Compute { uint _group_0_binding_1_cs[]; };


ivec2 operator_u002b(ivec2 value) {
    ivec2 value_1 = ivec2(0);
    value_1 = value;
    int _e3 = value_1.x;
    int _e5 = value_1.y;
    return ivec2(_e3, _e5);
}

ivec2 operator_u002d(ivec2 value_2) {
    ivec2 value_3 = ivec2(0);
    value_3 = value_2;
    int _e3 = value_3.x;
    int _e6 = value_3.y;
    return ivec2(-(_e3), -(_e6));
}

ivec2 operator_u007e(ivec2 value_4) {
    ivec2 value_5 = ivec2(0);
    value_5 = value_4;
    int _e3 = value_5.x;
    int _e6 = value_5.y;
    return ivec2(~(_e3), ~(_e6));
}

bool operator_u0021_(ivec2 value_6) {
    ivec2 value_7 = ivec2(0);
    bool local_1 = false;
    value_7 = value_6;
    int _e4 = value_7.x;
    bool _e5 = (_e4 == 0);
    local_1 = _e5;
    if (_e5) {
        int _e9 = value_7.y;
        local_1 = (_e9 == 0);
    }
    bool _e11 = local_1;
    return _e11;
}

int operator_u002a(ivec2 value_8) {
    ivec2 value_9 = ivec2(0);
    value_9 = value_8;
    int _e4 = value_9.x;
    int _e7 = value_9.y;
    return ((_e4 * 10) + _e7);
}

ivec2 operator_u0026_(ivec2 value_10) {
    ivec2 value_11 = ivec2(0);
    value_11 = value_10;
    int _e3 = value_11.y;
    int _e5 = value_11.x;
    return ivec2(_e3, _e5);
}

ivec2 operator_u002b_1(ivec2 left_1, ivec2 right_1) {
    ivec2 left_2 = ivec2(0);
    ivec2 right_2 = ivec2(0);
    left_2 = left_1;
    right_2 = right_1;
    int _e6 = left_2.x;
    int _e7 = right_2.x;
    int _e11 = left_2.y;
    int _e12 = right_2.y;
    return ivec2((_e6 + _e7), (_e11 + _e12));
}

ivec2 operator_u002d_1(ivec2 left_3, ivec2 right_3) {
    ivec2 left_4 = ivec2(0);
    ivec2 right_4 = ivec2(0);
    left_4 = left_3;
    right_4 = right_3;
    int _e6 = left_4.x;
    int _e7 = right_4.x;
    int _e11 = left_4.y;
    int _e12 = right_4.y;
    return ivec2((_e6 - _e7), (_e11 - _e12));
}

ivec2 operator_u002a_1(ivec2 left_5, ivec2 right_5) {
    ivec2 left_6 = ivec2(0);
    ivec2 right_6 = ivec2(0);
    left_6 = left_5;
    right_6 = right_5;
    int _e6 = left_6.x;
    int _e7 = right_6.x;
    int _e11 = left_6.y;
    int _e12 = right_6.y;
    return ivec2((_e6 * _e7), (_e11 * _e12));
}

ivec2 operator_u002f(ivec2 left_7, ivec2 right_7) {
    ivec2 left_8 = ivec2(0);
    ivec2 right_8 = ivec2(0);
    left_8 = left_7;
    right_8 = right_7;
    int _e6 = left_8.x;
    int _e7 = right_8.x;
    int _e11 = left_8.y;
    int _e12 = right_8.y;
    return ivec2((_e6 / _e7), (_e11 / _e12));
}

ivec2 operator_u0025_(ivec2 left_9, ivec2 right_9) {
    ivec2 left_10 = ivec2(0);
    ivec2 right_10 = ivec2(0);
    left_10 = left_9;
    right_10 = right_9;
    int _e6 = left_10.x;
    int _e7 = right_10.x;
    int _e11 = left_10.y;
    int _e12 = right_10.y;
    return ivec2((_e6 - _e7 * (_e6 / _e7)), (_e11 - _e12 * (_e11 / _e12)));
}

ivec2 operator_u0026_1(ivec2 left_11, ivec2 right_11) {
    ivec2 left_12 = ivec2(0);
    ivec2 right_12 = ivec2(0);
    left_12 = left_11;
    right_12 = right_11;
    int _e6 = left_12.x;
    int _e7 = right_12.x;
    int _e11 = left_12.y;
    int _e12 = right_12.y;
    return ivec2((_e6 & _e7), (_e11 & _e12));
}

ivec2 operator_u007c(ivec2 left_13, ivec2 right_13) {
    ivec2 left_14 = ivec2(0);
    ivec2 right_14 = ivec2(0);
    left_14 = left_13;
    right_14 = right_13;
    int _e6 = left_14.x;
    int _e7 = right_14.x;
    int _e11 = left_14.y;
    int _e12 = right_14.y;
    return ivec2((_e6 | _e7), (_e11 | _e12));
}

ivec2 operator_u005e(ivec2 left_15, ivec2 right_15) {
    ivec2 left_16 = ivec2(0);
    ivec2 right_16 = ivec2(0);
    left_16 = left_15;
    right_16 = right_15;
    int _e6 = left_16.x;
    int _e7 = right_16.x;
    int _e11 = left_16.y;
    int _e12 = right_16.y;
    return ivec2((_e6 ^ _e7), (_e11 ^ _e12));
}

ivec2 operator(ivec2 left_17, ivec2 right_17) {
    ivec2 left_18 = ivec2(0);
    ivec2 right_18 = ivec2(0);
    uint local_2 = 0u;
    uint local_3 = 0u;
    left_18 = left_17;
    right_18 = right_17;
    int _e5 = left_18.x;
    int _e7 = right_18.x;
    local_2 = uint(_e7);
    uint _e10 = local_2;
    int _e13 = left_18.y;
    int _e15 = right_18.y;
    local_3 = uint(_e15);
    uint _e18 = local_3;
    return ivec2((_e5 << _e10), (_e13 << _e18));
}

ivec2 operator_1(ivec2 left_19, ivec2 right_19) {
    ivec2 left_20 = ivec2(0);
    ivec2 right_20 = ivec2(0);
    left_20 = left_19;
    right_20 = right_19;
    int _e5 = left_20.x;
    int _e7 = right_20.x;
    int _e11 = left_20.y;
    int _e13 = right_20.y;
    return ivec2((_e5 >> uint(_e7)), (_e11 >> uint(_e13)));
}

Record_cuda_record_0_ operator_u002a_2(Record_cuda_record_0_ value_12) {
    Record_cuda_record_0_ value_13 = Record_cuda_record_0_(0, int[2](0, 0));
    value_13 = value_12;
    int _e3 = value_13.value;
    value_13.value = (_e3 + 1);
    Record_cuda_record_0_ _e6 = value_13;
    return _e6;
}

int operator_u0026_2(Record_cuda_record_0_ value_14) {
    Record_cuda_record_0_ value_15 = Record_cuda_record_0_(0, int[2](0, 0));
    value_15 = value_14;
    int _e3 = value_15.value;
    return _e3;
}

Record_cuda_record_0_ operator_u002d_2(Record_cuda_record_0_ value_16) {
    Record_cuda_record_0_ value_17 = Record_cuda_record_0_(0, int[2](0, 0));
    value_17 = value_16;
    int _e4 = value_17.value;
    value_17.value = -(_e4);
    Record_cuda_record_0_ _e6 = value_17;
    return _e6;
}

Record_cuda_record_0_ operator_u002a_3(Record_cuda_record_0_ value_18, int factor) {
    Record_cuda_record_0_ value_19 = Record_cuda_record_0_(0, int[2](0, 0));
    int factor_1 = 0;
    value_19 = value_18;
    factor_1 = factor;
    int _e5 = factor_1;
    int _e6 = value_19.value;
    value_19.value = (_e6 * _e5);
    int _e11 = factor_1;
    int _e12 = value_19.history[1];
    value_19.history[1] = (_e12 + _e11);
    Record_cuda_record_0_ _e14 = value_19;
    return _e14;
}

void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    uint base = 0u;
    ivec2 left = ivec2(0);
    ivec2 right = ivec2(0);
    Record_cuda_record_0_ original = Record_cuda_record_0_(0, int[2](0, 0));
    Record_cuda_record_0_ scaled = Record_cuda_record_0_(0, int[2](0, 0));
    Record_cuda_record_0_ local = Record_cuda_record_0_(0, int[2](0, 0));
    base = (threadIdx.x * 36u);
    ivec2 _e10 = _group_0_binding_0_cs[threadIdx.x];
    left = _e10;
    right = ivec2(3, 2);
    ivec2 _e19 = left;
    ivec2 _e20 = operator_u002b(_e19);
    uint _e16 = base;
    _group_0_binding_1_cs[(_e16 + 0u)] = uint(_e20.x);
    ivec2 _e27 = left;
    ivec2 _e28 = operator_u002d(_e27);
    uint _e24 = base;
    _group_0_binding_1_cs[(_e24 + 1u)] = uint(_e28.y);
    ivec2 _e35 = left;
    ivec2 _e36 = operator_u007e(_e35);
    uint _e32 = base;
    _group_0_binding_1_cs[(_e32 + 2u)] = uint(_e36.x);
    ivec2 _e43 = left;
    bool _e44 = operator_u0021_(_e43);
    uint _e40 = base;
    _group_0_binding_1_cs[(_e40 + 3u)] = uint(_e44);
    ivec2 _e50 = left;
    int _e51 = operator_u002a(_e50);
    uint _e47 = base;
    _group_0_binding_1_cs[(_e47 + 4u)] = uint(_e51);
    ivec2 _e57 = left;
    ivec2 _e58 = operator_u0026_(_e57);
    uint _e54 = base;
    _group_0_binding_1_cs[(_e54 + 5u)] = uint(_e58.x);
    ivec2 _e65 = left;
    ivec2 _e66 = right;
    ivec2 _e67 = operator_u002b_1(_e65, _e66);
    uint _e62 = base;
    _group_0_binding_1_cs[(_e62 + 6u)] = uint(_e67.x);
    ivec2 _e74 = left;
    ivec2 _e75 = right;
    ivec2 _e76 = operator_u002d_1(_e74, _e75);
    uint _e71 = base;
    _group_0_binding_1_cs[(_e71 + 7u)] = uint(_e76.y);
    ivec2 _e83 = left;
    ivec2 _e84 = right;
    ivec2 _e85 = operator_u002a_1(_e83, _e84);
    uint _e80 = base;
    _group_0_binding_1_cs[(_e80 + 8u)] = uint(_e85.x);
    ivec2 _e92 = left;
    ivec2 _e93 = right;
    ivec2 _e94 = operator_u002f(_e92, _e93);
    uint _e89 = base;
    _group_0_binding_1_cs[(_e89 + 9u)] = uint(_e94.y);
    ivec2 _e101 = left;
    ivec2 _e102 = right;
    ivec2 _e103 = operator_u0025_(_e101, _e102);
    uint _e98 = base;
    _group_0_binding_1_cs[(_e98 + 10u)] = uint(_e103.y);
    ivec2 _e110 = left;
    ivec2 _e111 = right;
    ivec2 _e112 = operator_u0026_1(_e110, _e111);
    uint _e107 = base;
    _group_0_binding_1_cs[(_e107 + 11u)] = uint(_e112.x);
    ivec2 _e119 = left;
    ivec2 _e120 = right;
    ivec2 _e121 = operator_u007c(_e119, _e120);
    uint _e116 = base;
    _group_0_binding_1_cs[(_e116 + 12u)] = uint(_e121.y);
    ivec2 _e128 = left;
    ivec2 _e129 = right;
    ivec2 _e130 = operator_u005e(_e128, _e129);
    uint _e125 = base;
    _group_0_binding_1_cs[(_e125 + 13u)] = uint(_e130.x);
    ivec2 _e137 = left;
    ivec2 _e138 = right;
    ivec2 _e139 = operator(_e137, _e138);
    uint _e134 = base;
    _group_0_binding_1_cs[(_e134 + 14u)] = uint(_e139.x);
    ivec2 _e146 = left;
    ivec2 _e147 = right;
    ivec2 _e148 = operator_1(_e146, _e147);
    uint _e143 = base;
    _group_0_binding_1_cs[(_e143 + 15u)] = uint(_e148.y);
    ivec2 _e155 = left;
    ivec2 _e156 = operator_u002b(_e155);
    uint _e152 = base;
    _group_0_binding_1_cs[(_e152 + 16u)] = uint(_e156.y);
    ivec2 _e163 = left;
    ivec2 _e164 = right;
    ivec2 _e165 = operator_u002d_1(_e163, _e164);
    uint _e160 = base;
    _group_0_binding_1_cs[(_e160 + 17u)] = uint(_e165.x);
    ivec2 _e172 = left;
    int _e173 = operator_u002a(_e172);
    uint _e169 = base;
    _group_0_binding_1_cs[(_e169 + 18u)] = uint(_e173);
    ivec2 _e179 = left;
    ivec2 _e180 = right;
    ivec2 _e181 = operator_u0026_1(_e179, _e180);
    uint _e176 = base;
    _group_0_binding_1_cs[(_e176 + 19u)] = uint(_e181.y);
    ivec2 _e188 = left;
    ivec2 _e189 = operator_u0026_(_e188);
    int _e190 = operator_u002a(_e189);
    uint _e185 = base;
    _group_0_binding_1_cs[(_e185 + 20u)] = uint(_e190);
    ivec2 _e196 = left;
    ivec2 _e197 = operator_u0026_(_e196);
    ivec2 _e198 = operator_u0026_(_e197);
    uint _e193 = base;
    _group_0_binding_1_cs[(_e193 + 21u)] = uint(_e198.x);
    ivec2 _e208 = left;
    ivec2 _e205 = right;
    ivec2 _e206 = right;
    ivec2 _e207 = operator_u002a_1(_e205, _e206);
    ivec2 _e209 = operator_u002b_1(_e208, _e207);
    uint _e202 = base;
    _group_0_binding_1_cs[(_e202 + 22u)] = uint(_e209.x);
    ivec2 _e216 = left;
    ivec2 _e220 = right;
    ivec2 _e221 = operator_u002b_1(_e220, ivec2(1, 1));
    ivec2 _e222 = operator(_e216, _e221);
    uint _e213 = base;
    _group_0_binding_1_cs[(_e213 + 23u)] = uint(_e222.x);
    ivec2 _e234 = left;
    ivec2 _e232 = right;
    ivec2 _e233 = operator_u0026_1(_e232, ivec2(1, 1));
    ivec2 _e235 = operator_u007c(_e234, _e233);
    uint _e226 = base;
    _group_0_binding_1_cs[(_e226 + 24u)] = uint(_e235.y);
    ivec2 _e242 = left;
    ivec2 _e243 = right;
    ivec2 _e244 = operator_u002d_1(_e242, _e243);
    ivec2 _e245 = right;
    ivec2 _e246 = operator_u002d_1(_e244, _e245);
    uint _e239 = base;
    _group_0_binding_1_cs[(_e239 + 25u)] = uint(_e246.x);
    original.value = 9;
    original.history[0] = 1;
    original.history[1] = 2;
    Record_cuda_record_0_ _e259 = original;
    Record_cuda_record_0_ _e260 = operator_u002a_3(_e259, 3);
    scaled = _e260;
    local.value = 9;
    local.history[0] = 1;
    local.history[1] = 2;
    Record_cuda_record_0_ _e273 = local;
    int _e274 = operator_u0026_2(_e273);
    uint _e262 = base;
    _group_0_binding_1_cs[(_e262 + 26u)] = uint(_e274);
    Record_cuda_record_0_ _e280 = original;
    Record_cuda_record_0_ _e281 = operator_u002a_2(_e280);
    int _e282 = operator_u0026_2(_e281);
    uint _e277 = base;
    _group_0_binding_1_cs[(_e277 + 27u)] = uint(_e282);
    int _e289 = scaled.value;
    uint _e285 = base;
    _group_0_binding_1_cs[(_e285 + 28u)] = uint(_e289);
    int _e298 = scaled.history[1];
    uint _e292 = base;
    _group_0_binding_1_cs[(_e292 + 29u)] = uint(_e298);
    int _e305 = original.value;
    uint _e301 = base;
    _group_0_binding_1_cs[(_e301 + 30u)] = uint(_e305);
    int _e314 = original.history[1];
    uint _e308 = base;
    _group_0_binding_1_cs[(_e308 + 31u)] = uint(_e314);
    Record_cuda_record_0_ _e320 = original;
    Record_cuda_record_0_ _e321 = operator_u002d_2(_e320);
    uint _e317 = base;
    _group_0_binding_1_cs[(_e317 + 32u)] = uint(_e321.value);
    Record_cuda_record_0_ _e328 = original;
    Record_cuda_record_0_ _e330 = operator_u002a_3(_e328, 2);
    uint _e325 = base;
    _group_0_binding_1_cs[(_e325 + 33u)] = uint(_e330.value);
    Record_cuda_record_0_ _e337 = original;
    Record_cuda_record_0_ _e338 = operator_u002a_2(_e337);
    int _e339 = operator_u0026_2(_e338);
    uint _e334 = base;
    _group_0_binding_1_cs[(_e334 + 34u)] = uint(_e339);
    bool _e348 = operator_u0021_(ivec2(0, 0));
    uint _e342 = base;
    _group_0_binding_1_cs[(_e342 + 35u)] = uint(_e348);
}

