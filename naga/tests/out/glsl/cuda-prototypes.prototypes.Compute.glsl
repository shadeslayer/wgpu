#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 2, local_size_y = 1, local_size_z = 1) in;

struct Packet_cuda_record_0_ {
    int key;
    int values[2];
};
layout(std430) buffer type_4_block_0Compute { uint _group_0_binding_0_cs[]; };

uvec3 blockIdx_1 = uvec3(0u);

uvec3 threadIdx_1 = uvec3(0u);


int route(float value) {
    float value_1 = 0.0;
    value_1 = value;
    float _e3 = value_1;
    return (10 + int(_e3));
}

int earlier(int value_2) {
    int value_3 = 0;
    value_3 = value_2;
    int _e2 = value_3;
    int _e4 = route(float(_e2));
    return _e4;
}

int route_1(int value_4) {
    int value_5 = 0;
    value_5 = value_4;
    int _e3 = value_5;
    return (20 + _e3);
}

int forward(int value_6) {
    int value_7 = 0;
    value_7 = value_6;
    int _e2 = value_7;
    int _e3 = route_1(_e2);
    int _e4 = value_7;
    int _e5 = earlier(_e4);
    return (_e3 + _e5);
}

Packet_cuda_record_0_ packet(int value_8) {
    int value_9 = 0;
    Packet_cuda_record_0_ local = Packet_cuda_record_0_(0, int[2](0, 0));
    value_9 = value_8;
    int _e3 = value_9;
    local.key = _e3;
    int _e7 = value_9;
    local.values[0] = (_e7 + 1);
    int _e12 = value_9;
    local.values[1] = (_e12 + 2);
    Packet_cuda_record_0_ _e14 = local;
    return _e14;
}

Packet_cuda_record_0_ adjust(Packet_cuda_record_0_ value_10) {
    Packet_cuda_record_0_ value_11 = Packet_cuda_record_0_(0, int[2](0, 0));
    value_11 = value_10;
    int _e4 = value_11.key;
    value_11.key = (_e4 + 4);
    int _e10 = value_11.values[1];
    value_11.values[1] = (_e10 + 6);
    Packet_cuda_record_0_ _e12 = value_11;
    return _e12;
}

Packet_cuda_record_0_ operator_u002b(Packet_cuda_record_0_ left, Packet_cuda_record_0_ right) {
    Packet_cuda_record_0_ left_1 = Packet_cuda_record_0_(0, int[2](0, 0));
    Packet_cuda_record_0_ right_1 = Packet_cuda_record_0_(0, int[2](0, 0));
    Packet_cuda_record_0_ local_1 = Packet_cuda_record_0_(0, int[2](0, 0));
    left_1 = left;
    right_1 = right;
    int _e7 = left_1.key;
    int _e8 = right_1.key;
    local_1.key = (_e7 + _e8);
    int _e18 = left_1.values[0];
    int _e19 = right_1.values[0];
    local_1.values[0] = (_e18 + _e19);
    int _e29 = left_1.values[1];
    int _e30 = right_1.values[1];
    local_1.values[1] = (_e29 + _e30);
    Packet_cuda_record_0_ _e32 = local_1;
    return _e32;
}

int pair(int first_1, int second_1) {
    int first_2 = 0;
    int second_2 = 0;
    first_2 = first_1;
    second_2 = second_1;
    int _e5 = first_2;
    int _e7 = second_2;
    return ((_e5 * 10) + _e7);
}

void consume(int value_12) {
    int value_13 = 0;
    value_13 = value_12;
    int _e2 = value_13;
    if ((_e2 != 0)) {
        return;
    } else {
        return;
    }
}

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

uint[22] _naga_trig_reduce_multiply(uint left_2[2], uint right_2[20]) {
    uint left_digits[2] = uint[2](0u, 0u);
    uint right_digits[20] = uint[20](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint product[22] = uint[22](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint carry = 0u;
    uint left_index = 0u;
    uint right_index = 0u;
    uint result[22] = uint[22](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint result_index = 0u;
    left_digits = left_2;
    right_digits = right_2;
    left_index = 0u;
    bool loop_init_1 = true;
    while(true) {
        if (!loop_init_1) {
            uint _e46 = left_index;
            left_index = (_e46 + 1u);
        }
        loop_init_1 = false;
        uint _e13 = left_index;
        if ((_e13 >= 2u)) {
            break;
        }
        carry = 0u;
        uint _e18 = left_digits[_e13];
        right_index = 0u;
        bool loop_init_2 = true;
        while(true) {
            if (!loop_init_2) {
                uint _e39 = right_index;
                right_index = (_e39 + 1u);
            }
            loop_init_2 = false;
            uint _e22 = right_index;
            if ((_e22 >= 20u)) {
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
        product[(_e13 + 20u)] = _e44;
    }
    result_index = 0u;
    bool loop_init_3 = true;
    while(true) {
        if (!loop_init_3) {
            uint _e62 = result_index;
            result_index = (_e62 + 1u);
        }
        loop_init_3 = false;
        uint _e54 = result_index;
        if ((_e54 >= 22u)) {
            break;
        }
        uint _e60 = product[(_e54 + 0u)];
        result[_e54] = _e60;
    }
    uint _e65[22] = result;
    return _e65;
}

uint _naga_trig_extract_product(uint digits[22], uint offset) {
    uint source[22] = uint[22](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint low = 0u;
    uint high = 0u;
    source = digits;
    uint _e5 = (offset / 16u);
    uint _e7 = (offset - 16u * (offset / 16u));
    uint _e13 = (_e5 + 0u);
    if ((_e13 < 22u)) {
        uint _e17 = source[_e13];
        low = _e17;
    }
    uint _e19 = (_e5 + 1u);
    if ((_e19 < 22u)) {
        uint _e23 = source[_e19];
        high = _e23;
    }
    uint _e24 = low;
    uint _e25 = high;
    return (((_e24 >> _e7) | (_e25 << (16u - _e7))) & 65535u);
}

uint[12] _naga_trig_multiply(uint left_3[12], uint right_3[12]) {
    uint left_digits_1[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint right_digits_1[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint product_1[24] = uint[24](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint carry_1 = 0u;
    uint left_index_1 = 0u;
    uint right_index_1 = 0u;
    uint result_1[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint result_index_1 = 0u;
    left_digits_1 = left_3;
    right_digits_1 = right_3;
    left_index_1 = 0u;
    bool loop_init_4 = true;
    while(true) {
        if (!loop_init_4) {
            uint _e46 = left_index_1;
            left_index_1 = (_e46 + 1u);
        }
        loop_init_4 = false;
        uint _e13 = left_index_1;
        if ((_e13 >= 12u)) {
            break;
        }
        carry_1 = 0u;
        uint _e18 = left_digits_1[_e13];
        right_index_1 = 0u;
        bool loop_init_5 = true;
        while(true) {
            if (!loop_init_5) {
                uint _e39 = right_index_1;
                right_index_1 = (_e39 + 1u);
            }
            loop_init_5 = false;
            uint _e22 = right_index_1;
            if ((_e22 >= 12u)) {
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
        product_1[(_e13 + 12u)] = _e44;
    }
    result_index_1 = 0u;
    bool loop_init_6 = true;
    while(true) {
        if (!loop_init_6) {
            uint _e62 = result_index_1;
            result_index_1 = (_e62 + 1u);
        }
        loop_init_6 = false;
        uint _e54 = result_index_1;
        if ((_e54 >= 12u)) {
            break;
        }
        uint _e60 = product_1[(_e54 + 11u)];
        result_1[_e54] = _e60;
    }
    uint _e65[12] = result_1;
    return _e65;
}

uint[12] _naga_trig_divide(uint value_14[12], uint divisor) {
    uint source_1[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint result_2[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint remainder_1 = 0u;
    uint step_ = 0u;
    source_1 = value_14;
    step_ = 0u;
    bool loop_init_7 = true;
    while(true) {
        if (!loop_init_7) {
            uint _e25 = step_;
            step_ = (_e25 + 1u);
        }
        loop_init_7 = false;
        uint _e11 = step_;
        if ((_e11 >= 12u)) {
            break;
        }
        uint _e15 = (11u - _e11);
        uint _e16 = remainder_1;
        uint _e20 = source_1[_e15];
        uint _e21 = ((_e16 << 16u) | _e20);
        result_2[_e15] = (_e21 / divisor);
        remainder_1 = (_e21 - divisor * (_e21 / divisor));
    }
    uint _e28[12] = result_2;
    return _e28;
}

uint[12] _naga_trig_combine(uint left_4[12], uint right_4[12], uint subtract) {
    uint left_digits_2[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint right_digits_2[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint result_3[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint carry_2 = 0u;
    uint digit_index = 0u;
    left_digits_2 = left_4;
    right_digits_2 = right_4;
    bool _e12 = (subtract != 0u);
    digit_index = 0u;
    bool loop_init_8 = true;
    while(true) {
        if (!loop_init_8) {
            uint _e38 = digit_index;
            digit_index = (_e38 + 1u);
        }
        loop_init_8 = false;
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

uint[12] _naga_trig_kernel(uint angle[12], uint sine) {
    uint term[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint total_1[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint step_1 = 0u;
    bool _e7 = (sine != 0u);
    if (_e7) {
        term = angle;
    } else {
        term = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 1u);
    }
    uint _e21[12] = term;
    total_1 = _e21;
    uint _e22[12] = _naga_trig_multiply(angle, angle);
    step_1 = 0u;
    bool loop_init_9 = true;
    while(true) {
        if (!loop_init_9) {
            uint _e46 = step_1;
            step_1 = (_e46 + 1u);
        }
        loop_init_9 = false;
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
        uint _e42[12] = total_1;
        uint _e45[12] = _naga_trig_combine(_e42, _e41, (_e30 & 1u));
        total_1 = _e45;
    }
    uint _e49[12] = total_1;
    return _e49;
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

uint _naga_trig_round(uint digits_2[12]) {
    uint source_3[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint highest_bit = 0u;
    uint nonzero = 0u;
    uint digit_index_1 = 0u;
    uint sticky = 0u;
    uint sticky_index = 0u;
    source_3 = digits_2;
    digit_index_1 = 0u;
    bool loop_init_10 = true;
    while(true) {
        if (!loop_init_10) {
            uint _e25 = digit_index_1;
            digit_index_1 = (_e25 + 1u);
        }
        loop_init_10 = false;
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
    bool loop_init_11 = true;
    while(true) {
        if (!loop_init_11) {
            uint _e80 = sticky_index;
            sticky_index = (_e80 + 1u);
        }
        loop_init_11 = false;
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

uint _naga_trigonometry(uint input_bits_1, uint cosine) {
    uint quadrant = 0u;
    uint fraction[12] = uint[12](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    uint fraction_index = 0u;
    uint _e3 = (input_bits_1 & 2147483647u);
    if ((_e3 >= 2139095040u)) {
        return 2143289344u;
    }
    bool _e8 = (cosine != 0u);
    if ((_e3 <= 956301312u)) {
        return (_e8 ? 1065353216u : input_bits_1);
    }
    uint _e16 = (470u - (_e3 >> 23u));
    uint _e20 = ((_e3 & 8388607u) | 8388608u);
    uint _e47[22] = _naga_trig_reduce_multiply(uint[2]((_e20 & 65535u), (_e20 >> 16u)), uint[20](53984u, 16973u, 28218u, 46884u, 50529u, 57019u, 25515u, 65105u, 36929u, 15427u, 38297u, 56162u, 56768u, 62772u, 22481u, 64551u, 5417u, 20036u, 33646u, 41721u));
    uint _e50 = _naga_trig_extract_product(_e47, _e16);
    quadrant = (_e50 & 3u);
    fraction_index = 0u;
    bool loop_init_12 = true;
    while(true) {
        if (!loop_init_12) {
            uint _e68 = fraction_index;
            fraction_index = (_e68 + 1u);
        }
        loop_init_12 = false;
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
    return (_e120 | ((((_e75 && _e115) ? 2147483648u : 0u) ^ (((_e8 ? (_e112 + 1u) : _e112) & 2u) << 30u)) ^ (_e8 ? 0u : (input_bits_1 & 2147483648u))));
}

float generated(float value_15) {
    float value_16 = 0.0;
    value_16 = value_15;
    uint _e4 = _naga_sqrtf(floatBitsToUint(4.0));
    float _e6 = value_16;
    uint _e9 = _naga_trigonometry(floatBitsToUint(_e6), 0u);
    float _e12 = value_16;
    uint _e15 = _naga_trigonometry(floatBitsToUint(_e12), 1u);
    return ((uintBitsToFloat(_e4) + uintBitsToFloat(_e9)) + uintBitsToFloat(_e15));
}

int control(int count) {
    int count_1 = 0;
    int result_4 = 0;
    int index = 0;
    count_1 = count;
    result_4 = 0;
    {
        index = 0;
        bool loop_init_13 = true;
        while(true) {
            if (!loop_init_13) {
                int _e9 = index;
                index = (_e9 + 1);
            }
            loop_init_13 = false;
            int _e6 = index;
            int _e7 = count_1;
            if ((_e6 < _e7)) {
            } else {
                break;
            }
            int _e13 = index;
            if ((_e13 == 1)) {
                continue;
            }
            int _e15 = index;
            switch(_e15) {
                case 0: {
                    int _e16 = index;
                    int _e17 = route_1(_e16);
                    int _e18 = result_4;
                    result_4 = (_e18 + _e17);
                    break;
                }
                default: {
                    int _e20 = index;
                    int _e21 = forward(_e20);
                    int _e22 = index;
                    int _e23 = earlier(_e22);
                    int _e25 = result_4;
                    result_4 = (_e25 + (_e21 - _e23));
                    break;
                }
            }
        }
    }
    int _e27 = result_4;
    return _e27;
}

uint late_index() {
    uvec3 _e1 = blockIdx_1;
    uvec3 _e6 = threadIdx_1;
    return ((_e1.x * 2u) + _e6.x);
}

void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    uvec3 blockIdx = gl_WorkGroupID;
    uint lane = 0u;
    uint base = 0u;
    int counter = 0;
    Packet_cuda_record_0_ first = Packet_cuda_record_0_(0, int[2](0, 0));
    Packet_cuda_record_0_ second = Packet_cuda_record_0_(0, int[2](0, 0));
    Packet_cuda_record_0_ total = Packet_cuda_record_0_(0, int[2](0, 0));
    int first_argument = 0;
    threadIdx_1 = threadIdx;
    blockIdx_1 = blockIdx;
    uint _e2 = late_index();
    lane = _e2;
    uint _e5 = lane;
    base = (_e5 * 16u);
    counter = 1;
    int _e10 = counter;
    counter = (_e10 + 1);
    Packet_cuda_record_0_ _e13 = packet(_e10);
    first = _e13;
    Packet_cuda_record_0_ _e15 = first;
    Packet_cuda_record_0_ _e16 = adjust(_e15);
    second = _e16;
    Packet_cuda_record_0_ _e18 = first;
    Packet_cuda_record_0_ _e19 = second;
    Packet_cuda_record_0_ _e20 = operator_u002b(_e18, _e19);
    total = _e20;
    int _e24 = earlier(3);
    uint _e21 = base;
    _group_0_binding_0_cs[_e21] = uint(_e24);
    int _e32 = route_1(3);
    uint _e27 = base;
    _group_0_binding_0_cs[(_e27 + uint(1))] = uint(_e32);
    int _e40 = forward(4);
    uint _e35 = base;
    _group_0_binding_0_cs[(_e35 + uint(2))] = uint(_e40);
    int _e48 = first.key;
    uint _e43 = base;
    _group_0_binding_0_cs[(_e43 + uint(3))] = uint(_e48);
    int _e58 = first.values[1];
    uint _e51 = base;
    _group_0_binding_0_cs[(_e51 + uint(4))] = uint(_e58);
    int _e66 = second.key;
    uint _e61 = base;
    _group_0_binding_0_cs[(_e61 + uint(5))] = uint(_e66);
    int _e76 = second.values[1];
    uint _e69 = base;
    _group_0_binding_0_cs[(_e69 + uint(6))] = uint(_e76);
    int _e84 = total.key;
    uint _e79 = base;
    _group_0_binding_0_cs[(_e79 + uint(7))] = uint(_e84);
    int _e94 = total.values[0];
    uint _e87 = base;
    _group_0_binding_0_cs[(_e87 + uint(8))] = uint(_e94);
    int _e104 = total.values[1];
    uint _e97 = base;
    _group_0_binding_0_cs[(_e97 + uint(9))] = uint(_e104);
    int _e107 = counter;
    counter = (_e107 + 1);
    first_argument = _e107;
    int _e115 = first_argument;
    int _e116 = counter;
    counter = (_e116 + 1);
    int _e119 = pair(_e115, _e116);
    uint _e111 = base;
    _group_0_binding_0_cs[(_e111 + uint(10))] = uint(_e119);
    int _e121 = counter;
    counter = (_e121 + 1);
    consume(_e121);
    int _e129 = counter;
    uint _e125 = base;
    _group_0_binding_0_cs[(_e125 + uint(11))] = uint(_e129);
    float _e137 = generated(0.0);
    uint _e132 = base;
    _group_0_binding_0_cs[(_e132 + uint(12))] = floatBitsToUint(_e137);
    int _e145 = control(3);
    uint _e140 = base;
    _group_0_binding_0_cs[(_e140 + uint(13))] = uint(_e145);
    uint _e152 = lane;
    uint _e148 = base;
    _group_0_binding_0_cs[(_e148 + uint(14))] = _e152;
    int _e158 = counter;
    counter = (_e158 + 1);
    int _e161 = counter;
    int _e162 = forward(_e161);
    uint _e154 = base;
    _group_0_binding_0_cs[(_e154 + uint(15))] = uint(_e162);
}

