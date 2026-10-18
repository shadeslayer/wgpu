#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct float3_ {
    float x;
    float y;
    float z;
};
layout(std430) buffer type_6_block_0Compute { uint _group_0_binding_0_cs[]; };


vec2 fabs(vec2 value_1) {
    vec2 value_2 = vec2(0.0);
    uint local_23 = 0u;
    uint local_24 = 0u;
    value_2 = value_1;
    float _e3 = value_2.x;
    uint _e6 = (floatBitsToUint(_e3) & 2147483647u);
    local_23 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_23;
    float _e15 = value_2.y;
    uint _e18 = (floatBitsToUint(_e15) & 2147483647u);
    local_24 = ((_e18 > 2139095040u) ? 2143289344u : _e18);
    uint _e24 = local_24;
    return vec2(uintBitsToFloat(_e12), uintBitsToFloat(_e24));
}

float3_ fabs_1(float3_ value_3) {
    float3_ value_4 = float3_(0.0, 0.0, 0.0);
    uint local_25 = 0u;
    uint local_26 = 0u;
    uint local_27 = 0u;
    value_4 = value_3;
    float _e3 = value_4.x;
    uint _e6 = (floatBitsToUint(_e3) & 2147483647u);
    local_25 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_25;
    float _e15 = value_4.y;
    uint _e18 = (floatBitsToUint(_e15) & 2147483647u);
    local_26 = ((_e18 > 2139095040u) ? 2143289344u : _e18);
    uint _e24 = local_26;
    float _e27 = value_4.z;
    uint _e30 = (floatBitsToUint(_e27) & 2147483647u);
    local_27 = ((_e30 > 2139095040u) ? 2143289344u : _e30);
    uint _e36 = local_27;
    return float3_(uintBitsToFloat(_e12), uintBitsToFloat(_e24), uintBitsToFloat(_e36));
}

vec4 fabs_2(vec4 value_5) {
    vec4 value_6 = vec4(0.0);
    uint local_28 = 0u;
    uint local_29 = 0u;
    uint local_30 = 0u;
    uint local_31 = 0u;
    value_6 = value_5;
    float _e3 = value_6.x;
    uint _e6 = (floatBitsToUint(_e3) & 2147483647u);
    local_28 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_28;
    float _e15 = value_6.y;
    uint _e18 = (floatBitsToUint(_e15) & 2147483647u);
    local_29 = ((_e18 > 2139095040u) ? 2143289344u : _e18);
    uint _e24 = local_29;
    float _e27 = value_6.z;
    uint _e30 = (floatBitsToUint(_e27) & 2147483647u);
    local_30 = ((_e30 > 2139095040u) ? 2143289344u : _e30);
    uint _e36 = local_30;
    float _e39 = value_6.w;
    uint _e42 = (floatBitsToUint(_e39) & 2147483647u);
    local_31 = ((_e42 > 2139095040u) ? 2143289344u : _e42);
    uint _e48 = local_31;
    return vec4(uintBitsToFloat(_e12), uintBitsToFloat(_e24), uintBitsToFloat(_e36), uintBitsToFloat(_e48));
}

float absolute(float value_7) {
    float value_8 = 0.0;
    uint local_32 = 0u;
    value_8 = value_7;
    float _e2 = value_8;
    uint _e5 = (floatBitsToUint(_e2) & 2147483647u);
    local_32 = ((_e5 > 2139095040u) ? 2143289344u : _e5);
    uint _e11 = local_32;
    return uintBitsToFloat(_e11);
}

float converted_absolute(int value_9) {
    int value_10 = 0;
    uint local_33 = 0u;
    value_10 = value_9;
    int _e2 = value_10;
    uint _e6 = (floatBitsToUint(float(_e2)) & 2147483647u);
    local_33 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_33;
    return uintBitsToFloat(_e12);
}

float converted_absolute_1(uint value_11) {
    uint value_12 = 0u;
    uint local_34 = 0u;
    value_12 = value_11;
    uint _e2 = value_12;
    uint _e6 = (floatBitsToUint(float(_e2)) & 2147483647u);
    local_34 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_34;
    return uintBitsToFloat(_e12);
}

float converted_absolute_2(bool value_13) {
    bool value_14 = false;
    uint local_35 = 0u;
    value_14 = value_13;
    bool _e2 = value_14;
    uint _e6 = (floatBitsToUint(float(_e2)) & 2147483647u);
    local_35 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_35;
    return uintBitsToFloat(_e12);
}

int fabs_3(int value_15) {
    int value_16 = 0;
    value_16 = value_15;
    int _e3 = value_16;
    return (_e3 + 40);
}

uint fabs_4(uint value_17) {
    uint value_18 = 0u;
    value_18 = value_17;
    uint _e3 = value_18;
    return (_e3 + 50u);
}

int fabs_5(bool value_19) {
    bool value_20 = false;
    int local_36 = 0;
    value_20 = value_19;
    bool _e2 = value_20;
    if (_e2) {
        local_36 = 7;
    } else {
        local_36 = 9;
    }
    int _e6 = local_36;
    return _e6;
}

int fabsf(bool value_21) {
    bool value_22 = false;
    int local_37 = 0;
    value_22 = value_21;
    bool _e2 = value_22;
    if (_e2) {
        local_37 = 11;
    } else {
        local_37 = 13;
    }
    int _e6 = local_37;
    return _e6;
}

float fabsf_1(float first, float second) {
    float first_1 = 0.0;
    float second_1 = 0.0;
    first_1 = first;
    second_1 = second;
    float _e4 = first_1;
    float _e5 = second_1;
    return (_e4 + _e5);
}

void main() {
    uint local = 0u;
    uint local_1 = 0u;
    uint local_2 = 0u;
    uint local_3 = 0u;
    uint local_4 = 0u;
    uint local_5 = 0u;
    uint local_6 = 0u;
    uint local_7 = 0u;
    uint local_8 = 0u;
    uint local_9 = 0u;
    uint local_10 = 0u;
    uint local_11 = 0u;
    uint local_12 = 0u;
    uint local_13 = 0u;
    uint local_14 = 0u;
    uint local_15 = 0u;
    uint local_16 = 0u;
    uint local_17 = 0u;
    uint local_18 = 0u;
    uint local_19 = 0u;
    uint local_20 = 0u;
    float value = 0.0;
    uint local_21 = 0u;
    float3_ triple = float3_(0.0, 0.0, 0.0);
    uint local_22 = 0u;
    vec2 pair = vec2(0.0);
    vec4 quadruple = vec4(0.0);
    uint _e6 = (floatBitsToUint(0.0) & 2147483647u);
    local = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local;
    _group_0_binding_0_cs[0] = floatBitsToUint(uintBitsToFloat(_e12));
    uint _e21 = (floatBitsToUint(-(0.0)) & 2147483647u);
    local_1 = ((_e21 > 2139095040u) ? 2143289344u : _e21);
    uint _e27 = local_1;
    _group_0_binding_0_cs[1] = floatBitsToUint(uintBitsToFloat(_e27));
    uint _e36 = (floatBitsToUint(-(1.75)) & 2147483647u);
    local_2 = ((_e36 > 2139095040u) ? 2143289344u : _e36);
    uint _e42 = local_2;
    _group_0_binding_0_cs[2] = floatBitsToUint(uintBitsToFloat(_e42));
    float _e49 = absolute(-(1.25));
    _group_0_binding_0_cs[3] = floatBitsToUint(_e49);
    float _e55 = converted_absolute(-(3));
    _group_0_binding_0_cs[4] = floatBitsToUint(_e55);
    float _e60 = converted_absolute_1(4294967295u);
    _group_0_binding_0_cs[5] = floatBitsToUint(_e60);
    float _e65 = converted_absolute_2(true);
    _group_0_binding_0_cs[6] = floatBitsToUint(_e65);
    float _e70 = converted_absolute_2(false);
    _group_0_binding_0_cs[7] = floatBitsToUint(_e70);
    local_3 = 1u;
    uint _e76 = local_3;
    uint _e80 = (floatBitsToUint(uintBitsToFloat(_e76)) & 2147483647u);
    local_4 = ((_e80 > 2139095040u) ? 2143289344u : _e80);
    uint _e86 = local_4;
    _group_0_binding_0_cs[8] = floatBitsToUint(uintBitsToFloat(_e86));
    local_5 = 2147483649u;
    uint _e93 = local_5;
    uint _e97 = (floatBitsToUint(uintBitsToFloat(_e93)) & 2147483647u);
    local_6 = ((_e97 > 2139095040u) ? 2143289344u : _e97);
    uint _e103 = local_6;
    _group_0_binding_0_cs[9] = floatBitsToUint(uintBitsToFloat(_e103));
    local_7 = 8388607u;
    uint _e110 = local_7;
    uint _e114 = (floatBitsToUint(uintBitsToFloat(_e110)) & 2147483647u);
    local_8 = ((_e114 > 2139095040u) ? 2143289344u : _e114);
    uint _e120 = local_8;
    _group_0_binding_0_cs[10] = floatBitsToUint(uintBitsToFloat(_e120));
    local_9 = 2155872255u;
    uint _e127 = local_9;
    uint _e131 = (floatBitsToUint(uintBitsToFloat(_e127)) & 2147483647u);
    local_10 = ((_e131 > 2139095040u) ? 2143289344u : _e131);
    uint _e137 = local_10;
    _group_0_binding_0_cs[11] = floatBitsToUint(uintBitsToFloat(_e137));
    uint _e145 = (floatBitsToUint(3.4028235e38) & 2147483647u);
    local_11 = ((_e145 > 2139095040u) ? 2143289344u : _e145);
    uint _e151 = local_11;
    _group_0_binding_0_cs[12] = floatBitsToUint(uintBitsToFloat(_e151));
    uint _e160 = (floatBitsToUint(-(3.4028235e38)) & 2147483647u);
    local_12 = ((_e160 > 2139095040u) ? 2143289344u : _e160);
    uint _e166 = local_12;
    _group_0_binding_0_cs[13] = floatBitsToUint(uintBitsToFloat(_e166));
    local_13 = 2139095040u;
    uint _e173 = local_13;
    uint _e177 = (floatBitsToUint(uintBitsToFloat(_e173)) & 2147483647u);
    local_14 = ((_e177 > 2139095040u) ? 2143289344u : _e177);
    uint _e183 = local_14;
    _group_0_binding_0_cs[14] = floatBitsToUint(uintBitsToFloat(_e183));
    local_15 = 4286578688u;
    uint _e190 = local_15;
    uint _e194 = (floatBitsToUint(uintBitsToFloat(_e190)) & 2147483647u);
    local_16 = ((_e194 > 2139095040u) ? 2143289344u : _e194);
    uint _e200 = local_16;
    _group_0_binding_0_cs[15] = floatBitsToUint(uintBitsToFloat(_e200));
    local_17 = 2143363909u;
    uint _e207 = local_17;
    uint _e211 = (floatBitsToUint(uintBitsToFloat(_e207)) & 2147483647u);
    local_18 = ((_e211 > 2139095040u) ? 2143289344u : _e211);
    uint _e217 = local_18;
    _group_0_binding_0_cs[16] = floatBitsToUint(uintBitsToFloat(_e217));
    local_19 = 2139095041u;
    uint _e224 = local_19;
    uint _e228 = (floatBitsToUint(uintBitsToFloat(_e224)) & 2147483647u);
    local_20 = ((_e228 > 2139095040u) ? 2143289344u : _e228);
    uint _e234 = local_20;
    _group_0_binding_0_cs[17] = floatBitsToUint(uintBitsToFloat(_e234));
    value = -(2.75);
    float _e242 = value;
    value = (_e242 + float(1));
    uint _e248 = (floatBitsToUint(_e242) & 2147483647u);
    local_21 = ((_e248 > 2139095040u) ? 2143289344u : _e248);
    uint _e254 = local_21;
    _group_0_binding_0_cs[18] = floatBitsToUint(uintBitsToFloat(_e254));
    float _e259 = value;
    _group_0_binding_0_cs[19] = floatBitsToUint(_e259);
    int _e265 = fabs_3(-(3));
    _group_0_binding_0_cs[20] = uint(_e265);
    uint _e270 = fabs_4(3u);
    _group_0_binding_0_cs[21] = _e270;
    int _e274 = fabs_5(true);
    _group_0_binding_0_cs[22] = uint(_e274);
    int _e279 = fabs_5(false);
    _group_0_binding_0_cs[23] = uint(_e279);
    int _e284 = fabsf(true);
    _group_0_binding_0_cs[24] = uint(_e284);
    local_22 = 2147483649u;
    uint _e293 = local_22;
    float3_ _e296 = fabs_1(float3_(-(2.5), -(0.0), uintBitsToFloat(_e293)));
    triple = _e296;
    float _e300 = triple.x;
    _group_0_binding_0_cs[25] = floatBitsToUint(_e300);
    float _e305 = triple.y;
    _group_0_binding_0_cs[26] = floatBitsToUint(_e305);
    float _e310 = triple.z;
    _group_0_binding_0_cs[27] = floatBitsToUint(_e310);
    float _e317 = fabsf_1(-(2.0), 3.0);
    _group_0_binding_0_cs[28] = floatBitsToUint(_e317);
    vec2 _e325 = fabs(vec2(-(3.0), -(4.0)));
    pair = _e325;
    vec4 _e336 = fabs_2(vec4(-(1.0), -(2.0), -(3.0), -(4.0)));
    quadruple = _e336;
    float _e341 = pair.x;
    float _e342 = pair.y;
    float _e345 = quadruple.x;
    float _e348 = quadruple.w;
    _group_0_binding_0_cs[29] = floatBitsToUint((((_e341 + _e342) + _e345) + _e348));
}

