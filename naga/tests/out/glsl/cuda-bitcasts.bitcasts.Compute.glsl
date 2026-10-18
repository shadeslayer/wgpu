#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_3_block_0Compute { uint _group_0_binding_0_cs[]; };


float from_bits(uint value_1) {
    uint value_2 = 0u;
    uint local_13 = 0u;
    value_2 = value_1;
    uint _e2 = value_2;
    local_13 = _e2;
    uint _e4 = local_13;
    return uintBitsToFloat(_e4);
}

int _float_as_int(int first, int second) {
    int first_1 = 0;
    int second_1 = 0;
    first_1 = first;
    second_1 = second;
    int _e4 = first_1;
    int _e5 = second_1;
    return (_e4 + _e5);
}

void main() {
    int local = 0;
    int local_1 = 0;
    int local_2 = 0;
    int local_3 = 0;
    uint local_4 = 0u;
    uint local_5 = 0u;
    uint local_6 = 0u;
    uint local_7 = 0u;
    float value = 0.0;
    int signed_bits = 0;
    int local_8 = 0;
    uint unsigned_bits = 0u;
    uint local_9 = 0u;
    float copied = 0.0;
    int local_10 = 0;
    uint local_11 = 0u;
    uint local_12 = 0u;
    _group_0_binding_0_cs[0] = uint(floatBitsToInt(float(true)));
    _group_0_binding_0_cs[1] = uint(floatBitsToInt(float(3)));
    _group_0_binding_0_cs[2] = uint(floatBitsToInt(float(5u)));
    _group_0_binding_0_cs[3] = uint(floatBitsToInt(2.75));
    _group_0_binding_0_cs[4] = floatBitsToUint(float(true));
    _group_0_binding_0_cs[5] = floatBitsToUint(float(3));
    _group_0_binding_0_cs[6] = floatBitsToUint(float(5u));
    _group_0_binding_0_cs[7] = floatBitsToUint(2.75);
    local = int(true);
    int _e48 = local;
    _group_0_binding_0_cs[8] = floatBitsToUint(intBitsToFloat(_e48));
    local_1 = 3;
    int _e55 = local_1;
    _group_0_binding_0_cs[9] = floatBitsToUint(intBitsToFloat(_e55));
    local_2 = int(5u);
    int _e63 = local_2;
    _group_0_binding_0_cs[10] = floatBitsToUint(intBitsToFloat(_e63));
    local_3 = int(2.75);
    int _e71 = local_3;
    _group_0_binding_0_cs[11] = floatBitsToUint(intBitsToFloat(_e71));
    local_4 = uint(true);
    uint _e79 = local_4;
    _group_0_binding_0_cs[12] = floatBitsToUint(uintBitsToFloat(_e79));
    local_5 = uint(3);
    uint _e87 = local_5;
    _group_0_binding_0_cs[13] = floatBitsToUint(uintBitsToFloat(_e87));
    local_6 = 5u;
    uint _e94 = local_6;
    _group_0_binding_0_cs[14] = floatBitsToUint(uintBitsToFloat(_e94));
    local_7 = uint(2.75);
    uint _e102 = local_7;
    _group_0_binding_0_cs[15] = floatBitsToUint(uintBitsToFloat(_e102));
    value = 1.5;
    float _e109 = value;
    value = (_e109 + float(1));
    _group_0_binding_0_cs[16] = uint(floatBitsToInt(_e109));
    float _e117 = value;
    _group_0_binding_0_cs[17] = floatBitsToUint(_e117);
    float _e121 = value;
    value = (_e121 + float(1));
    _group_0_binding_0_cs[18] = floatBitsToUint(_e121);
    float _e128 = value;
    _group_0_binding_0_cs[19] = floatBitsToUint(_e128);
    signed_bits = 1065353216;
    int _e134 = signed_bits;
    signed_bits = (_e134 + 1);
    local_8 = _e134;
    int _e138 = local_8;
    _group_0_binding_0_cs[20] = floatBitsToUint(intBitsToFloat(_e138));
    int _e143 = signed_bits;
    _group_0_binding_0_cs[21] = uint(_e143);
    unsigned_bits = 1073741824u;
    uint _e149 = unsigned_bits;
    unsigned_bits = (_e149 + uint(1));
    local_9 = _e149;
    uint _e154 = local_9;
    _group_0_binding_0_cs[22] = floatBitsToUint(uintBitsToFloat(_e154));
    uint _e159 = unsigned_bits;
    _group_0_binding_0_cs[23] = _e159;
    int _e164 = _float_as_int(3, 4);
    _group_0_binding_0_cs[24] = uint(_e164);
    float _e168 = from_bits(3212836864u);
    copied = _e168;
    float _e171 = copied;
    _group_0_binding_0_cs[25] = floatBitsToUint(_e171);
    local_10 = 2139095040;
    int _e177 = local_10;
    _group_0_binding_0_cs[26] = floatBitsToUint(intBitsToFloat(_e177));
    local_11 = 2143363909u;
    uint _e184 = local_11;
    _group_0_binding_0_cs[27] = floatBitsToUint(uintBitsToFloat(_e184));
    local_12 = uint(1065353200.0);
    uint _e192 = local_12;
    _group_0_binding_0_cs[28] = floatBitsToUint(uintBitsToFloat(_e192));
    _group_0_binding_0_cs[29] = uint(floatBitsToInt(float(1u)));
}

