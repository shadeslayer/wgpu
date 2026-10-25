#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_10_block_0Compute { uint _group_0_binding_0_cs[]; };


uint[12] _naga_trig_multiply(uint left[12], uint right[12]) {
    uint left_digits[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint right_digits[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint product[24] = uint[24](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint carry = 0u;
    uint left_index = 0u;
    uint right_index = 0u;
    uint result[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint result_index = 0u;
    left_digits = left;
    right_digits = right;
    left_index = 0u;
    bool loop_init = true;
    while(true) {
        if (!loop_init) {
            uint _e46 = left_index;
            left_index = (_e46 + 1u);
        }
        loop_init = false;
        uint _e13 = left_index;
        if ((_e13 >= 12u)) {
            break;
        }
        carry = 0u;
        uint _e18 = left_digits[_e13];
        right_index = 0u;
        bool loop_init_1 = true;
        while(true) {
            if (!loop_init_1) {
                uint _e39 = right_index;
                right_index = (_e39 + 1u);
            }
            loop_init_1 = false;
            uint _e22 = right_index;
            if ((_e22 >= 12u)) {
                break;
            }
            uint _e26 = right_digits[_e22];
            uint _e27 = (_e13 + _e22);
            uint _e29 = product[_e27];
            uint _e30 = carry;
            uint _e33 = (((_e18 * _e26) + _e29) + _e30);
            product[_e27] = (_e33 & 65535u);
            carry = (_e33 >> 16u);
        }
        uint _e44 = carry;
        product[(_e13 + 12u)] = _e44;
    }
    result_index = 0u;
    bool loop_init_2 = true;
    while(true) {
        if (!loop_init_2) {
            uint _e62 = result_index;
            result_index = (_e62 + 1u);
        }
        loop_init_2 = false;
        uint _e54 = result_index;
        if ((_e54 >= 12u)) {
            break;
        }
        uint _e60 = product[(_e54 + 11u)];
        result[_e54] = _e60;
    }
    uint _e65[12] = result;
    return _e65;
}

uint[22] _naga_trig_reduce_multiply(uint left_1[2], uint right_1[20]) {
    uint left_digits_1[2] = uint[2](0u, 0u);
    uint right_digits_1[20] = uint[20](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint product_1[22] = uint[22](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint carry_1 = 0u;
    uint left_index_1 = 0u;
    uint right_index_1 = 0u;
    uint result_1[22] = uint[22](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint result_index_1 = 0u;
    left_digits_1 = left_1;
    right_digits_1 = right_1;
    left_index_1 = 0u;
    bool loop_init_3 = true;
    while(true) {
        if (!loop_init_3) {
            uint _e46 = left_index_1;
            left_index_1 = (_e46 + 1u);
        }
        loop_init_3 = false;
        uint _e13 = left_index_1;
        if ((_e13 >= 2u)) {
            break;
        }
        carry_1 = 0u;
        uint _e18 = left_digits_1[_e13];
        right_index_1 = 0u;
        bool loop_init_4 = true;
        while(true) {
            if (!loop_init_4) {
                uint _e39 = right_index_1;
                right_index_1 = (_e39 + 1u);
            }
            loop_init_4 = false;
            uint _e22 = right_index_1;
            if ((_e22 >= 20u)) {
                break;
            }
            uint _e26 = right_digits_1[_e22];
            uint _e27 = (_e13 + _e22);
            uint _e29 = product_1[_e27];
            uint _e30 = carry_1;
            uint _e33 = (((_e18 * _e26) + _e29) + _e30);
            product_1[_e27] = (_e33 & 65535u);
            carry_1 = (_e33 >> 16u);
        }
        uint _e44 = carry_1;
        product_1[(_e13 + 20u)] = _e44;
    }
    result_index_1 = 0u;
    bool loop_init_5 = true;
    while(true) {
        if (!loop_init_5) {
            uint _e62 = result_index_1;
            result_index_1 = (_e62 + 1u);
        }
        loop_init_5 = false;
        uint _e54 = result_index_1;
        if ((_e54 >= 22u)) {
            break;
        }
        uint _e60 = product_1[(_e54 + 0u)];
        result_1[_e54] = _e60;
    }
    uint _e65[22] = result_1;
    return _e65;
}

uint[12] _naga_trig_divide(uint value_1[12], uint divisor) {
    uint source[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint result_2[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint remainder = 0u;
    uint step_ = 0u;
    source = value_1;
    step_ = 0u;
    bool loop_init_6 = true;
    while(true) {
        if (!loop_init_6) {
            uint _e25 = step_;
            step_ = (_e25 + 1u);
        }
        loop_init_6 = false;
        uint _e11 = step_;
        if ((_e11 >= 12u)) {
            break;
        }
        uint _e15 = (11u - _e11);
        uint _e16 = remainder;
        uint _e20 = source[_e15];
        uint _e21 = ((_e16 << 16u) | _e20);
        result_2[_e15] = (_e21 / divisor);
        remainder = (_e21 - divisor * (_e21 / divisor));
    }
    uint _e28[12] = result_2;
    return _e28;
}

uint[12] _naga_trig_combine(uint left_2[12], uint right_2[12], uint subtract) {
    uint left_digits_2[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint right_digits_2[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint result_3[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint carry_2 = 0u;
    uint digit_index = 0u;
    left_digits_2 = left_2;
    right_digits_2 = right_2;
    bool _e12 = (subtract != 0u);
    digit_index = 0u;
    bool loop_init_7 = true;
    while(true) {
        if (!loop_init_7) {
            uint _e38 = digit_index;
            digit_index = (_e38 + 1u);
        }
        loop_init_7 = false;
        uint _e16 = digit_index;
        if ((_e16 >= 12u)) {
            break;
        }
        uint _e20 = left_digits_2[_e16];
        uint _e22 = right_digits_2[_e16];
        uint _e23 = carry_2;
        uint _e24 = (_e22 + _e23);
        uint _e26 = (_e20 + _e24);
        result_3[_e16] = ((_e12 ? (_e20 - _e24) : _e26) & 65535u);
        carry_2 = (_e12 ? ((_e20 < _e24) ? 1u : 0u) : (_e26 >> 16u));
    }
    uint _e41[12] = result_3;
    return _e41;
}

uint _naga_trig_extract_product(uint digits[22], uint offset) {
    uint source_1[22] = uint[22](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint low = 0u;
    uint high = 0u;
    source_1 = digits;
    uint _e5 = (offset / 16u);
    uint _e7 = (offset - 16u * (offset / 16u));
    uint _e13 = (_e5 + 0u);
    if ((_e13 < 22u)) {
        uint _e17 = source_1[_e13];
        low = _e17;
    }
    uint _e19 = (_e5 + 1u);
    if ((_e19 < 22u)) {
        uint _e23 = source_1[_e19];
        high = _e23;
    }
    uint _e24 = low;
    uint _e25 = high;
    return (((_e24 >> _e7) | (_e25 << (16u - _e7))) & 65535u);
}

uint _naga_trig_extract_fixed(uint digits_1[12], uint offset_1) {
    uint source_2[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint low_1 = 0u;
    uint high_1 = 0u;
    source_2 = digits_1;
    uint _e5 = (offset_1 / 16u);
    uint _e7 = (offset_1 - 16u * (offset_1 / 16u));
    uint _e13 = (_e5 + 0u);
    if ((_e13 < 12u)) {
        uint _e17 = source_2[_e13];
        low_1 = _e17;
    }
    uint _e19 = (_e5 + 1u);
    if ((_e19 < 12u)) {
        uint _e23 = source_2[_e19];
        high_1 = _e23;
    }
    uint _e24 = low_1;
    uint _e25 = high_1;
    return (((_e24 >> _e7) | (_e25 << (16u - _e7))) & 65535u);
}

uint[12] _naga_trig_kernel(uint angle[12], uint sine) {
    uint term[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint total[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint step_1 = 0u;
    bool _e7 = (sine != 0u);
    if (_e7) {
        term = angle;
    } else {
        term = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 1u);
    }
    uint _e21[12] = term;
    total = _e21;
    uint _e22[12] = _naga_trig_multiply(angle, angle);
    step_1 = 0u;
    bool loop_init_8 = true;
    while(true) {
        if (!loop_init_8) {
            uint _e46 = step_1;
            step_1 = (_e46 + 1u);
        }
        loop_init_8 = false;
        uint _e26 = step_1;
        if ((_e26 >= 8u)) {
            break;
        }
        uint _e30 = (_e26 + 1u);
        uint _e32 = (_e30 * 2u);
        uint _e39[12] = term;
        uint _e40[12] = _naga_trig_multiply(_e39, _e22);
        uint _e41[12] = _naga_trig_divide(_e40, (_e32 * (_e7 ? (_e32 + 1u) : (_e32 - 1u))));
        term = _e41;
        uint _e42[12] = total;
        uint _e45[12] = _naga_trig_combine(_e42, _e41, (_e30 & 1u));
        total = _e45;
    }
    uint _e49[12] = total;
    return _e49;
}

uint _naga_trig_round(uint digits_2[12]) {
    uint source_3[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint highest_bit = 0u;
    uint nonzero = 0u;
    uint digit_index_1 = 0u;
    uint sticky = 0u;
    uint sticky_index = 0u;
    source_3 = digits_2;
    digit_index_1 = 0u;
    bool loop_init_9 = true;
    while(true) {
        if (!loop_init_9) {
            uint _e25 = digit_index_1;
            digit_index_1 = (_e25 + 1u);
        }
        loop_init_9 = false;
        uint _e10 = digit_index_1;
        if ((_e10 >= 12u)) {
            break;
        }
        uint _e14 = source_3[_e10];
        uint _e15 = nonzero;
        nonzero = (_e15 | _e14);
        if ((_e14 != 0u)) {
            highest_bit = ((_e10 * 16u) + (31u - uint(31 - findMSB(_e14))));
        }
    }
    uint _e28 = nonzero;
    if ((_e28 == 0u)) {
        return 0u;
    }
    uint _e32 = highest_bit;
    uint _e38 = ((_e32 >= 50u) ? (_e32 - 23u) : 27u);
    uint _e39 = _naga_trig_extract_fixed(digits_2, _e38);
    uint _e42 = _naga_trig_extract_fixed(digits_2, (_e38 + 16u));
    uint _e45 = (_e39 | (_e42 << 16u));
    uint _e47 = (_e38 - 1u);
    uint _e48 = _naga_trig_extract_fixed(digits_2, _e47);
    uint _e54 = (_e47 / 16u);
    sticky_index = 0u;
    bool loop_init_10 = true;
    while(true) {
        if (!loop_init_10) {
            uint _e80 = sticky_index;
            sticky_index = (_e80 + 1u);
        }
        loop_init_10 = false;
        uint _e66 = sticky_index;
        if ((_e66 >= 12u)) {
            break;
        }
        uint _e76 = source_3[_e66];
        uint _e78 = sticky;
        sticky = (_e78 | (_e76 & ((_e66 < _e54) ? 65535u : ((_e66 == _e54) ? ((1u << (_e47 - 16u * (_e47 / 16u))) - 1u) : 0u))));
    }
    uint _e83 = sticky;
    uint _e93 = (_e45 + ((((_e48 & 1u) != 0u) && ((_e83 | (_e45 & 1u)) != 0u)) ? 1u : 0u));
    if ((_e32 < 50u)) {
        return _e93;
    }
    bool _e97 = (_e93 == 16777216u);
    uint _e102 = (_e32 - 49u);
    return (((_e97 ? (_e102 + 1u) : _e102) << 23u) | ((_e97 ? (_e93 >> 1u) : _e93) & 8388607u));
}

uint _naga_trigonometry(uint input_bits, uint cosine) {
    uint quadrant = 0u;
    uint fraction[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint fraction_index = 0u;
    uint _e3 = (input_bits & 2147483647u);
    if ((_e3 >= 2139095040u)) {
        return 2143289344u;
    }
    bool _e8 = (cosine != 0u);
    if ((_e3 <= 956301312u)) {
        return (_e8 ? 1065353216u : input_bits);
    }
    uint _e16 = (470u - (_e3 >> 23u));
    uint _e20 = ((_e3 & 8388607u) | 8388608u);
    uint _e47[22] = _naga_trig_reduce_multiply(uint[2]((_e20 & 65535u), (_e20 >> 16u)), uint[20](53984u, 16973u, 28218u, 46884u, 50529u, 57019u, 25515u, 65105u, 36929u, 15427u, 38297u, 56162u, 56768u, 62772u, 22481u, 64551u, 5417u, 20036u, 33646u, 41721u));
    uint _e50 = _naga_trig_extract_product(_e47, _e16);
    quadrant = (_e50 & 3u);
    fraction_index = 0u;
    bool loop_init_11 = true;
    while(true) {
        if (!loop_init_11) {
            uint _e68 = fraction_index;
            fraction_index = (_e68 + 1u);
        }
        loop_init_11 = false;
        uint _e60 = fraction_index;
        if ((_e60 >= 11u)) {
            break;
        }
        uint _e66 = _naga_trig_extract_product(_e47, ((_e16 - 176u) + (_e60 * 16u)));
        fraction[_e60] = _e66;
    }
    uint _e73 = fraction[10u];
    bool _e75 = (_e73 >= 32768u);
    if (_e75) {
        uint _e89 = quadrant;
        quadrant = ((_e89 + 1u) & 3u);
        uint _e94[12] = fraction;
        uint _e96[12] = _naga_trig_combine(uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 1u), _e94, 1u);
        fraction = _e96;
    }
    uint _e97[12] = fraction;
    uint _e111[12] = _naga_trig_multiply(_e97, uint[12](5327u, 39953u, 20996u, 14754u, 440u, 50455u, 35212u, 33897u, 17105u, 46404u, 37407u, 1u));
    uint _e112 = quadrant;
    bool _e115 = ((_e112 & 1u) == cosine);
    uint _e119[12] = _naga_trig_kernel(_e111, (_e115 ? 1u : 0u));
    uint _e120 = _naga_trig_round(_e119);
    return (_e120 | ((((_e75 && _e115) ? 2147483648u : 0u) ^ (((_e8 ? (_e112 + 1u) : _e112) & 2u) << 30u)) ^ (_e8 ? 0u : (input_bits & 2147483648u))));
}

float sine_value(float value_2) {
    float value_3 = 0.0;
    value_3 = value_2;
    float _e2 = value_3;
    uint _e5 = _naga_trigonometry(floatBitsToUint(_e2), 0u);
    return uintBitsToFloat(_e5);
}

float cosine_value(float value_4) {
    float value_5 = 0.0;
    value_5 = value_4;
    float _e2 = value_5;
    uint _e5 = _naga_trigonometry(floatBitsToUint(_e2), 1u);
    return uintBitsToFloat(_e5);
}

int sinf(bool value_6) {
    bool value_7 = false;
    int local_4 = 0;
    value_7 = value_6;
    bool _e2 = value_7;
    if (_e2) {
        local_4 = 17;
    } else {
        local_4 = 19;
    }
    int _e6 = local_4;
    return _e6;
}

int cosf(int value_8) {
    int value_9 = 0;
    value_9 = value_8;
    int _e3 = value_9;
    return (_e3 + 23);
}

float sinf_1(float first, float second) {
    float first_1 = 0.0;
    float second_1 = 0.0;
    first_1 = first;
    second_1 = second;
    float _e4 = first_1;
    float _e5 = second_1;
    return (_e4 + _e5);
}

float cosf_1(vec2 value_10) {
    vec2 value_11 = vec2(0.0);
    value_11 = value_10;
    float _e4 = value_11.x;
    float _e5 = value_11.y;
    return (_e4 + _e5);
}

float identity(float value_12) {
    float value_13 = 0.0;
    value_13 = value_12;
    float _e2 = value_13;
    return _e2;
}

float pair(float first_2, float second_2) {
    float first_3 = 0.0;
    float second_3 = 0.0;
    first_3 = first_2;
    second_3 = second_2;
    float _e5 = first_3;
    float _e7 = second_3;
    return ((_e5 * 10.0) + _e7);
}

void main() {
    float value = 0.0;
    bool choose = false;
    int counter = 0;
    bool local = false;
    bool local_1 = false;
    float local_2 = 0.0;
    float values[2] = float[2](0.0, 0.0);
    int Count = 0;
    float scratch[2] = float[2](0.0, 0.0);
    int repeated = 0;
    int index = 0;
    float local_3 = 0.0;
    uint _e6 = _naga_trigonometry(floatBitsToUint(0.0), 0u);
    _group_0_binding_0_cs[0] = floatBitsToUint(uintBitsToFloat(_e6));
    uint _e15 = _naga_trigonometry(floatBitsToUint(-(0.0)), 0u);
    _group_0_binding_0_cs[1] = floatBitsToUint(uintBitsToFloat(_e15));
    uint _e23 = _naga_trigonometry(floatBitsToUint(0.0), 1u);
    _group_0_binding_0_cs[2] = floatBitsToUint(uintBitsToFloat(_e23));
    uint _e32 = _naga_trigonometry(floatBitsToUint(-(0.0)), 1u);
    _group_0_binding_0_cs[3] = floatBitsToUint(uintBitsToFloat(_e32));
    float _e39 = sine_value(float(0));
    _group_0_binding_0_cs[4] = floatBitsToUint(_e39);
    float _e45 = cosine_value(float(0u));
    _group_0_binding_0_cs[5] = floatBitsToUint(_e45);
    int _e50 = sinf(true);
    _group_0_binding_0_cs[6] = uint(_e50);
    int _e55 = sinf(false);
    _group_0_binding_0_cs[7] = uint(_e55);
    int _e60 = cosf(4);
    _group_0_binding_0_cs[8] = uint(_e60);
    uint _e67 = _naga_trigonometry(floatBitsToUint(0.0), 1u);
    _group_0_binding_0_cs[9] = floatBitsToUint(uintBitsToFloat(_e67));
    float _e74 = sinf_1(2.0, 3.0);
    _group_0_binding_0_cs[10] = floatBitsToUint(_e74);
    float _e81 = cosf_1(vec2(2.0, 3.0));
    _group_0_binding_0_cs[11] = floatBitsToUint(_e81);
    value = 0.0;
    float _e87 = value;
    value = (_e87 + float(1));
    uint _e93 = _naga_trigonometry(floatBitsToUint(_e87), 0u);
    _group_0_binding_0_cs[12] = floatBitsToUint(uintBitsToFloat(_e93));
    float _e98 = value;
    _group_0_binding_0_cs[13] = floatBitsToUint(_e98);
    value = -(1.0);
    float _e104 = value;
    value = (_e104 + float(1));
    float _e108 = value;
    uint _e111 = _naga_trigonometry(floatBitsToUint(_e108), 1u);
    _group_0_binding_0_cs[14] = floatBitsToUint(uintBitsToFloat(_e111));
    float _e116 = value;
    _group_0_binding_0_cs[15] = floatBitsToUint(_e116);
    float _e120 = value;
    value = (_e120 + float(1));
    float _e124 = identity(_e120);
    uint _e127 = _naga_trigonometry(floatBitsToUint(_e124), 0u);
    float _e129 = value;
    value = (_e129 + float(1));
    float _e133 = pair(uintBitsToFloat(_e127), _e129);
    _group_0_binding_0_cs[16] = floatBitsToUint(_e133);
    float _e137 = value;
    _group_0_binding_0_cs[17] = floatBitsToUint(_e137);
    choose = false;
    counter = 0;
    bool _e145 = choose;
    local = _e145;
    if (_e145) {
        int _e147 = counter;
        counter = (_e147 + 1);
        uint _e153 = _naga_trigonometry(floatBitsToUint(float(_e147)), 0u);
        local = (uintBitsToFloat(_e153) != 0.0);
    }
    bool _e157 = local;
    _group_0_binding_0_cs[18] = uint(_e157);
    local_1 = true;
    if (true) {
    } else {
        int _e163 = counter;
        counter = (_e163 + 1);
        uint _e169 = _naga_trigonometry(floatBitsToUint(float(_e163)), 1u);
        local_1 = (uintBitsToFloat(_e169) != 0.0);
    }
    bool _e173 = local_1;
    _group_0_binding_0_cs[19] = uint(_e173);
    bool _e177 = choose;
    if (_e177) {
        int _e178 = counter;
        counter = (_e178 + 1);
        int _e181 = counter;
        uint _e185 = _naga_trigonometry(floatBitsToUint(float(_e181)), 0u);
        local_2 = uintBitsToFloat(_e185);
    } else {
        int _e187 = counter;
        uint _e191 = _naga_trigonometry(floatBitsToUint(float(_e187)), 1u);
        local_2 = uintBitsToFloat(_e191);
    }
    float _e194 = local_2;
    _group_0_binding_0_cs[20] = uint(_e194);
    int _e198 = counter;
    _group_0_binding_0_cs[21] = uint(_e198);
    uint _e205 = _naga_trigonometry(floatBitsToUint(0.0), 0u);
    values[0] = uintBitsToFloat(_e205);
    uint _e211 = _naga_trigonometry(floatBitsToUint(0.0), 1u);
    values[1] = uintBitsToFloat(_e211);
    float _e219 = values[0];
    float _e220 = values[1];
    _group_0_binding_0_cs[22] = floatBitsToUint((_e219 + _e220));
    Count = 2;
    scratch = float[2](0.0, 0.0);
    float _e231 = scratch[1];
    _group_0_binding_0_cs[23] = floatBitsToUint(_e231);
    repeated = 0;
    {
        index = 0;
        bool loop_init_12 = true;
        while(true) {
            if (!loop_init_12) {
                int _e240 = index;
                index = (_e240 + 1);
            }
            loop_init_12 = false;
            int _e238 = index;
            if ((_e238 < 3)) {
            } else {
                break;
            }
            int _e244 = index;
            if ((_e244 == 0)) {
                continue;
            }
            uint _e249 = _naga_trigonometry(floatBitsToUint(0.0), 1u);
            int _e252 = repeated;
            repeated = (_e252 + int(uintBitsToFloat(_e249)));
        }
    }
    int _e256 = repeated;
    _group_0_binding_0_cs[24] = uint(_e256);
    value = 0.0;
    float _e261 = value;
    value = (_e261 + float(1));
    uint _e267 = _naga_trigonometry(floatBitsToUint(_e261), 0u);
    float _e268 = value;
    value = (_e268 - float(1));
    float _e272 = value;
    uint _e275 = _naga_trigonometry(floatBitsToUint(_e272), 1u);
    _group_0_binding_0_cs[25] = floatBitsToUint(uintBitsToFloat(_e275));
    float _e280 = value;
    _group_0_binding_0_cs[26] = floatBitsToUint(_e280);
    if (true) {
        uint _e288 = _naga_trigonometry(floatBitsToUint(0.0), 1u);
        local_3 = uintBitsToFloat(_e288);
    } else {
        uint _e293 = _naga_trigonometry(floatBitsToUint(99.0), 0u);
        local_3 = uintBitsToFloat(_e293);
    }
    float _e296 = local_3;
    _group_0_binding_0_cs[27] = uint(_e296);
    float _e302 = cosine_value(float(false));
    _group_0_binding_0_cs[28] = floatBitsToUint(_e302);
}

