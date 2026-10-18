#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_4_block_0Compute { uint _group_0_binding_0_cs[]; };


uint _naga_sqrtf(uint input_bits) {
    uint significand = 0u;
    uint adjusted_exponent = 0u;
    uint upper_word = 0u;
    uint lower_word = 0u;
    uint root = 0u;
    uint remainder = 0u;
    uint iteration = 0u;
    uint _e5 = (input_bits & 2147483647u);
    if (((_e5 == 0u) || (input_bits == 2139095040u))) {
        return input_bits;
    }
    if ((((input_bits & 2147483648u) != 0u) || (_e5 > 2139095040u))) {
        return 2143289344u;
    }
    uint _e19 = (input_bits & 8388607u);
    uint _e20 = (_e5 >> 23u);
    significand = (_e19 | 8388608u);
    adjusted_exponent = (_e20 + 127u);
    if ((_e20 == 0u)) {
        uint _e28 = (uint(31 - findMSB(_e19)) - 8u);
        significand = (_e19 << _e28);
        adjusted_exponent = (128u - _e28);
    }
    uint _e32 = significand;
    uint _e33 = adjusted_exponent;
    uint _e34 = (_e33 & 1u);
    upper_word = (_e32 >> (9u - _e34));
    lower_word = (_e32 << (23u + _e34));
    root = 0u;
    remainder = 0u;
    iteration = 0u;
    bool loop_init = true;
    while(true) {
        if (!loop_init) {
            uint _e72 = iteration;
            iteration = (_e72 + 1u);
        }
        loop_init = false;
        uint _e50 = iteration;
        if ((_e50 >= 24u)) {
            break;
        }
        uint _e52 = upper_word;
        uint _e53 = lower_word;
        uint _e54 = root;
        uint _e55 = remainder;
        uint _e63 = ((_e55 << 2u) | (_e52 >> 14u));
        uint _e65 = ((_e54 << 2u) | 1u);
        bool _e66 = (_e63 >= _e65);
        uint _e69 = (_e54 << 1u);
        upper_word = (((_e52 << 2u) | (_e53 >> 30u)) & 65535u);
        lower_word = (_e53 << 2u);
        remainder = (_e66 ? (_e63 - _e65) : _e63);
        root = (_e66 ? (_e69 | 1u) : _e69);
    }
    uint _e74 = root;
    uint _e75 = remainder;
    return (((_e33 >> 1u) << 23u) | (((_e75 > _e74) ? (_e74 + 1u) : _e74) & 8388607u));
}

float root_1(float value_1) {
    float value_2 = 0.0;
    value_2 = value_1;
    float _e2 = value_2;
    uint _e4 = _naga_sqrtf(floatBitsToUint(_e2));
    return uintBitsToFloat(_e4);
}

float root_2(int value_3) {
    int value_4 = 0;
    value_4 = value_3;
    int _e2 = value_4;
    uint _e5 = _naga_sqrtf(floatBitsToUint(float(_e2)));
    return uintBitsToFloat(_e5);
}

float root_3(uint value_5) {
    uint value_6 = 0u;
    value_6 = value_5;
    uint _e2 = value_6;
    uint _e5 = _naga_sqrtf(floatBitsToUint(float(_e2)));
    return uintBitsToFloat(_e5);
}

float root_4(bool value_7) {
    bool value_8 = false;
    value_8 = value_7;
    bool _e2 = value_8;
    uint _e5 = _naga_sqrtf(floatBitsToUint(float(_e2)));
    return uintBitsToFloat(_e5);
}

int sqrtf(bool value_9) {
    bool value_10 = false;
    int local_10 = 0;
    value_10 = value_9;
    bool _e2 = value_10;
    if (_e2) {
        local_10 = 7;
    } else {
        local_10 = 9;
    }
    int _e6 = local_10;
    return _e6;
}

float sqrtf_1(float first, float second) {
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
    float value = 0.0;
    uint local_6 = 0u;
    uint local_7 = 0u;
    uint local_8 = 0u;
    uint local_9 = 0u;
    uint _e5 = _naga_sqrtf(floatBitsToUint(0.0));
    _group_0_binding_0_cs[0] = floatBitsToUint(uintBitsToFloat(_e5));
    uint _e13 = _naga_sqrtf(floatBitsToUint(-(0.0)));
    _group_0_binding_0_cs[1] = floatBitsToUint(uintBitsToFloat(_e13));
    uint _e20 = _naga_sqrtf(floatBitsToUint(1.0));
    _group_0_binding_0_cs[2] = floatBitsToUint(uintBitsToFloat(_e20));
    float _e26 = root_2(4);
    _group_0_binding_0_cs[3] = floatBitsToUint(_e26);
    float _e31 = root_3(9u);
    _group_0_binding_0_cs[4] = floatBitsToUint(_e31);
    uint _e37 = _naga_sqrtf(floatBitsToUint(6.25));
    _group_0_binding_0_cs[5] = floatBitsToUint(uintBitsToFloat(_e37));
    uint _e44 = _naga_sqrtf(floatBitsToUint(2.0));
    _group_0_binding_0_cs[6] = floatBitsToUint(uintBitsToFloat(_e44));
    local = 1u;
    uint _e51 = local;
    uint _e54 = _naga_sqrtf(floatBitsToUint(uintBitsToFloat(_e51)));
    _group_0_binding_0_cs[7] = floatBitsToUint(uintBitsToFloat(_e54));
    local_1 = 8388607u;
    uint _e61 = local_1;
    uint _e64 = _naga_sqrtf(floatBitsToUint(uintBitsToFloat(_e61)));
    _group_0_binding_0_cs[8] = floatBitsToUint(uintBitsToFloat(_e64));
    local_2 = 8388608u;
    uint _e71 = local_2;
    uint _e74 = _naga_sqrtf(floatBitsToUint(uintBitsToFloat(_e71)));
    _group_0_binding_0_cs[9] = floatBitsToUint(uintBitsToFloat(_e74));
    local_3 = 2139095039u;
    uint _e81 = local_3;
    uint _e84 = _naga_sqrtf(floatBitsToUint(uintBitsToFloat(_e81)));
    _group_0_binding_0_cs[10] = floatBitsToUint(uintBitsToFloat(_e84));
    local_4 = 2139095040u;
    uint _e91 = local_4;
    uint _e94 = _naga_sqrtf(floatBitsToUint(uintBitsToFloat(_e91)));
    _group_0_binding_0_cs[11] = floatBitsToUint(uintBitsToFloat(_e94));
    uint _e102 = _naga_sqrtf(floatBitsToUint(-(1.0)));
    _group_0_binding_0_cs[12] = floatBitsToUint(uintBitsToFloat(_e102));
    local_5 = 2139095041u;
    uint _e109 = local_5;
    uint _e112 = _naga_sqrtf(floatBitsToUint(uintBitsToFloat(_e109)));
    _group_0_binding_0_cs[13] = floatBitsToUint(uintBitsToFloat(_e112));
    value = 4.0;
    float _e119 = value;
    value = (_e119 + float(1));
    float _e123 = root_1(_e119);
    _group_0_binding_0_cs[14] = floatBitsToUint(_e123);
    float _e127 = value;
    _group_0_binding_0_cs[15] = floatBitsToUint(_e127);
    int _e132 = sqrtf(true);
    _group_0_binding_0_cs[16] = uint(_e132);
    int _e137 = sqrtf(false);
    _group_0_binding_0_cs[17] = uint(_e137);
    float _e143 = sqrtf_1(2.0, 3.0);
    _group_0_binding_0_cs[18] = floatBitsToUint(_e143);
    local_6 = 1065353217u;
    uint _e149 = local_6;
    uint _e152 = _naga_sqrtf(floatBitsToUint(uintBitsToFloat(_e149)));
    _group_0_binding_0_cs[19] = floatBitsToUint(uintBitsToFloat(_e152));
    local_7 = 1065353218u;
    uint _e159 = local_7;
    uint _e162 = _naga_sqrtf(floatBitsToUint(uintBitsToFloat(_e159)));
    _group_0_binding_0_cs[20] = floatBitsToUint(uintBitsToFloat(_e162));
    local_8 = 1065353215u;
    uint _e169 = local_8;
    uint _e172 = _naga_sqrtf(floatBitsToUint(uintBitsToFloat(_e169)));
    _group_0_binding_0_cs[21] = floatBitsToUint(uintBitsToFloat(_e172));
    float _e178 = root_3(4294967295u);
    _group_0_binding_0_cs[22] = floatBitsToUint(_e178);
    float _e184 = root_2(-(4));
    _group_0_binding_0_cs[23] = floatBitsToUint(_e184);
    float _e189 = root_1(16.0);
    _group_0_binding_0_cs[24] = floatBitsToUint(_e189);
    uint _e195 = _naga_sqrtf(floatBitsToUint(0.25));
    _group_0_binding_0_cs[25] = floatBitsToUint(uintBitsToFloat(_e195));
    local_9 = 2147483649u;
    uint _e202 = local_9;
    uint _e205 = _naga_sqrtf(floatBitsToUint(uintBitsToFloat(_e202)));
    _group_0_binding_0_cs[26] = floatBitsToUint(uintBitsToFloat(_e205));
    float _e211 = root_4(false);
    _group_0_binding_0_cs[27] = floatBitsToUint(_e211);
    float _e216 = root_4(true);
    _group_0_binding_0_cs[28] = floatBitsToUint(_e216);
    float _e221 = root_1(0.5);
    _group_0_binding_0_cs[29] = floatBitsToUint(_e221);
}

