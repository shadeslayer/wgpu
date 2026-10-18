// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef uint type_4[1];

uint _naga_sqrtf(
    uint input_bits
) {
    uint significand = {};
    uint adjusted_exponent = {};
    uint upper_word = {};
    uint lower_word = {};
    uint root = {};
    uint remainder = {};
    uint iteration = {};
    uint _e5 = input_bits & 2147483647u;
    if ((_e5 == 0u) || (input_bits == 2139095040u)) {
        return input_bits;
    }
    if (((input_bits & 2147483648u) != 0u) || (_e5 > 2139095040u)) {
        return 2143289344u;
    }
    uint _e19 = input_bits & 8388607u;
    uint _e20 = _e5 >> 23u;
    significand = _e19 | 8388608u;
    adjusted_exponent = _e20 + 127u;
    if (_e20 == 0u) {
        uint _e28 = metal::clz(_e19) - 8u;
        significand = _e19 << _e28;
        adjusted_exponent = 128u - _e28;
    }
    uint _e32 = significand;
    uint _e33 = adjusted_exponent;
    uint _e34 = _e33 & 1u;
    upper_word = _e32 >> (9u - _e34);
    lower_word = _e32 << (23u + _e34);
    root = 0u;
    remainder = 0u;
    iteration = 0u;
    uint2 loop_bound = uint2(4294967295u);
    bool loop_init = true;
    while(true) {
        if (metal::all(loop_bound == uint2(0u))) { break; }
        loop_bound -= uint2(loop_bound.y == 0u, 1u);
        if (!loop_init) {
            uint _e72 = iteration;
            iteration = _e72 + 1u;
        }
        loop_init = false;
        uint _e50 = iteration;
        if (_e50 >= 24u) {
            break;
        }
        uint _e52 = upper_word;
        uint _e53 = lower_word;
        uint _e54 = root;
        uint _e55 = remainder;
        uint _e63 = (_e55 << 2u) | (_e52 >> 14u);
        uint _e65 = (_e54 << 2u) | 1u;
        bool _e66 = _e63 >= _e65;
        uint _e69 = _e54 << 1u;
        upper_word = ((_e52 << 2u) | (_e53 >> 30u)) & 65535u;
        lower_word = _e53 << 2u;
        remainder = _e66 ? (_e63 - _e65) : _e63;
        root = _e66 ? (_e69 | 1u) : _e69;
    }
    uint _e74 = root;
    uint _e75 = remainder;
    return ((_e33 >> 1u) << 23u) | (((_e75 > _e74) ? (_e74 + 1u) : _e74) & 8388607u);
}

float root_1(
    float value_1
) {
    float value_2 = {};
    value_2 = value_1;
    float _e2 = value_2;
    uint _e4 = _naga_sqrtf(as_type<uint>(_e2));
    return as_type<float>(_e4);
}

float root_2(
    int value_3
) {
    int value_4 = {};
    value_4 = value_3;
    int _e2 = value_4;
    uint _e5 = _naga_sqrtf(as_type<uint>(static_cast<float>(_e2)));
    return as_type<float>(_e5);
}

float root_3(
    uint value_5
) {
    uint value_6 = {};
    value_6 = value_5;
    uint _e2 = value_6;
    uint _e5 = _naga_sqrtf(as_type<uint>(static_cast<float>(_e2)));
    return as_type<float>(_e5);
}

float root_4(
    bool value_7
) {
    bool value_8 = {};
    value_8 = value_7;
    bool _e2 = value_8;
    uint _e5 = _naga_sqrtf(as_type<uint>(static_cast<float>(_e2)));
    return as_type<float>(_e5);
}

int sqrtf(
    bool value_9
) {
    bool value_10 = {};
    int local_10 = {};
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

float sqrtf_1(
    float first,
    float second
) {
    float first_1 = {};
    float second_1 = {};
    first_1 = first;
    second_1 = second;
    float _e4 = first_1;
    float _e5 = second_1;
    return _e4 + _e5;
}
int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}


kernel void roots(
  device type_4& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint local = {};
    uint local_1 = {};
    uint local_2 = {};
    uint local_3 = {};
    uint local_4 = {};
    uint local_5 = {};
    float value = {};
    uint local_6 = {};
    uint local_7 = {};
    uint local_8 = {};
    uint local_9 = {};
    uint _e5 = _naga_sqrtf(as_type<uint>(0.0));
    out[0] = as_type<uint>(as_type<float>(_e5));
    uint _e13 = _naga_sqrtf(as_type<uint>(-(0.0)));
    out[1] = as_type<uint>(as_type<float>(_e13));
    uint _e20 = _naga_sqrtf(as_type<uint>(1.0));
    out[2] = as_type<uint>(as_type<float>(_e20));
    float _e26 = root_2(4);
    out[3] = as_type<uint>(_e26);
    float _e31 = root_3(9u);
    out[4] = as_type<uint>(_e31);
    uint _e37 = _naga_sqrtf(as_type<uint>(6.25));
    out[5] = as_type<uint>(as_type<float>(_e37));
    uint _e44 = _naga_sqrtf(as_type<uint>(2.0));
    out[6] = as_type<uint>(as_type<float>(_e44));
    local = 1u;
    uint _e51 = local;
    uint _e54 = _naga_sqrtf(as_type<uint>(as_type<float>(_e51)));
    out[7] = as_type<uint>(as_type<float>(_e54));
    local_1 = 8388607u;
    uint _e61 = local_1;
    uint _e64 = _naga_sqrtf(as_type<uint>(as_type<float>(_e61)));
    out[8] = as_type<uint>(as_type<float>(_e64));
    local_2 = 8388608u;
    uint _e71 = local_2;
    uint _e74 = _naga_sqrtf(as_type<uint>(as_type<float>(_e71)));
    out[9] = as_type<uint>(as_type<float>(_e74));
    local_3 = 2139095039u;
    uint _e81 = local_3;
    uint _e84 = _naga_sqrtf(as_type<uint>(as_type<float>(_e81)));
    out[10] = as_type<uint>(as_type<float>(_e84));
    local_4 = 2139095040u;
    uint _e91 = local_4;
    uint _e94 = _naga_sqrtf(as_type<uint>(as_type<float>(_e91)));
    out[11] = as_type<uint>(as_type<float>(_e94));
    uint _e102 = _naga_sqrtf(as_type<uint>(-(1.0)));
    out[12] = as_type<uint>(as_type<float>(_e102));
    local_5 = 2139095041u;
    uint _e109 = local_5;
    uint _e112 = _naga_sqrtf(as_type<uint>(as_type<float>(_e109)));
    out[13] = as_type<uint>(as_type<float>(_e112));
    value = 4.0;
    float _e119 = value;
    value = _e119 + static_cast<float>(1);
    float _e123 = root_1(_e119);
    out[14] = as_type<uint>(_e123);
    float _e127 = value;
    out[15] = as_type<uint>(_e127);
    int _e132 = sqrtf(true);
    out[16] = static_cast<uint>(_e132);
    int _e137 = sqrtf(false);
    out[17] = static_cast<uint>(_e137);
    float _e143 = sqrtf_1(2.0, 3.0);
    out[18] = as_type<uint>(_e143);
    local_6 = 1065353217u;
    uint _e149 = local_6;
    uint _e152 = _naga_sqrtf(as_type<uint>(as_type<float>(_e149)));
    out[19] = as_type<uint>(as_type<float>(_e152));
    local_7 = 1065353218u;
    uint _e159 = local_7;
    uint _e162 = _naga_sqrtf(as_type<uint>(as_type<float>(_e159)));
    out[20] = as_type<uint>(as_type<float>(_e162));
    local_8 = 1065353215u;
    uint _e169 = local_8;
    uint _e172 = _naga_sqrtf(as_type<uint>(as_type<float>(_e169)));
    out[21] = as_type<uint>(as_type<float>(_e172));
    float _e178 = root_3(4294967295u);
    out[22] = as_type<uint>(_e178);
    float _e184 = root_2(naga_neg(4));
    out[23] = as_type<uint>(_e184);
    float _e189 = root_1(16.0);
    out[24] = as_type<uint>(_e189);
    uint _e195 = _naga_sqrtf(as_type<uint>(0.25));
    out[25] = as_type<uint>(as_type<float>(_e195));
    local_9 = 2147483649u;
    uint _e202 = local_9;
    uint _e205 = _naga_sqrtf(as_type<uint>(as_type<float>(_e202)));
    out[26] = as_type<uint>(as_type<float>(_e205));
    float _e211 = root_4(false);
    out[27] = as_type<uint>(_e211);
    float _e216 = root_4(true);
    out[28] = as_type<uint>(_e216);
    float _e221 = root_1(0.5);
    out[29] = as_type<uint>(_e221);
}
