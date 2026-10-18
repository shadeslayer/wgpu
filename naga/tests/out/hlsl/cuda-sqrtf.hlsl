RWByteAddressBuffer out_ : register(u0);

uint _naga_sqrtf(uint input_bits)
{
    uint significand = (uint)0;
    uint adjusted_exponent = (uint)0;
    uint upper_word = (uint)0;
    uint lower_word = (uint)0;
    uint root = (uint)0;
    uint remainder = (uint)0;
    uint iteration = (uint)0;

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
        uint _e28 = ((31u - firstbithigh(_e19)) - 8u);
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
    uint2 loop_bound = uint2(4294967295u, 4294967295u);
    bool loop_init = true;
    while(true) {
        if (all(loop_bound == uint2(0u, 0u))) { break; }
        loop_bound -= uint2(loop_bound.y == 0u, 1u);
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

float root_1(float value_1)
{
    float value_2 = (float)0;

    value_2 = value_1;
    float _e2 = value_2;
    const uint _e4 = _naga_sqrtf(asuint(_e2));
    return asfloat(_e4);
}

float root_2(int value_3)
{
    int value_4 = (int)0;

    value_4 = value_3;
    int _e2 = value_4;
    const uint _e5 = _naga_sqrtf(asuint(float(_e2)));
    return asfloat(_e5);
}

float root_3(uint value_5)
{
    uint value_6 = (uint)0;

    value_6 = value_5;
    uint _e2 = value_6;
    const uint _e5 = _naga_sqrtf(asuint(float(_e2)));
    return asfloat(_e5);
}

float root_4(bool value_7)
{
    bool value_8 = (bool)0;

    value_8 = value_7;
    bool _e2 = value_8;
    const uint _e5 = _naga_sqrtf(asuint(float(_e2)));
    return asfloat(_e5);
}

int sqrtf(bool value_9)
{
    bool value_10 = (bool)0;
    int local_10 = (int)0;

    value_10 = value_9;
    bool _e2 = value_10;
    if (_e2) {
        local_10 = int(7);
    } else {
        local_10 = int(9);
    }
    int _e6 = local_10;
    return _e6;
}

float sqrtf_1(float first, float second)
{
    float first_1 = (float)0;
    float second_1 = (float)0;

    first_1 = first;
    second_1 = second;
    float _e4 = first_1;
    float _e5 = second_1;
    return (_e4 + _e5);
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

[numthreads(1, 1, 1)]
void roots()
{
    uint local = (uint)0;
    uint local_1 = (uint)0;
    uint local_2 = (uint)0;
    uint local_3 = (uint)0;
    uint local_4 = (uint)0;
    uint local_5 = (uint)0;
    float value = (float)0;
    uint local_6 = (uint)0;
    uint local_7 = (uint)0;
    uint local_8 = (uint)0;
    uint local_9 = (uint)0;

    const uint _e5 = _naga_sqrtf(asuint(0.0));
    out_.Store(int(0)*4, asuint(asuint(asfloat(_e5))));
    const uint _e13 = _naga_sqrtf(asuint(-(0.0)));
    out_.Store(int(1)*4, asuint(asuint(asfloat(_e13))));
    const uint _e20 = _naga_sqrtf(asuint(1.0));
    out_.Store(int(2)*4, asuint(asuint(asfloat(_e20))));
    const float _e26 = root_2(int(4));
    out_.Store(int(3)*4, asuint(asuint(_e26)));
    const float _e31 = root_3(9u);
    out_.Store(int(4)*4, asuint(asuint(_e31)));
    const uint _e37 = _naga_sqrtf(asuint(6.25));
    out_.Store(int(5)*4, asuint(asuint(asfloat(_e37))));
    const uint _e44 = _naga_sqrtf(asuint(2.0));
    out_.Store(int(6)*4, asuint(asuint(asfloat(_e44))));
    local = 1u;
    uint _e51 = local;
    const uint _e54 = _naga_sqrtf(asuint(asfloat(_e51)));
    out_.Store(int(7)*4, asuint(asuint(asfloat(_e54))));
    local_1 = 8388607u;
    uint _e61 = local_1;
    const uint _e64 = _naga_sqrtf(asuint(asfloat(_e61)));
    out_.Store(int(8)*4, asuint(asuint(asfloat(_e64))));
    local_2 = 8388608u;
    uint _e71 = local_2;
    const uint _e74 = _naga_sqrtf(asuint(asfloat(_e71)));
    out_.Store(int(9)*4, asuint(asuint(asfloat(_e74))));
    local_3 = 2139095039u;
    uint _e81 = local_3;
    const uint _e84 = _naga_sqrtf(asuint(asfloat(_e81)));
    out_.Store(int(10)*4, asuint(asuint(asfloat(_e84))));
    local_4 = 2139095040u;
    uint _e91 = local_4;
    const uint _e94 = _naga_sqrtf(asuint(asfloat(_e91)));
    out_.Store(int(11)*4, asuint(asuint(asfloat(_e94))));
    const uint _e102 = _naga_sqrtf(asuint(-(1.0)));
    out_.Store(int(12)*4, asuint(asuint(asfloat(_e102))));
    local_5 = 2139095041u;
    uint _e109 = local_5;
    const uint _e112 = _naga_sqrtf(asuint(asfloat(_e109)));
    out_.Store(int(13)*4, asuint(asuint(asfloat(_e112))));
    value = 4.0;
    float _e119 = value;
    value = (_e119 + float(int(1)));
    const float _e123 = root_1(_e119);
    out_.Store(int(14)*4, asuint(asuint(_e123)));
    float _e127 = value;
    out_.Store(int(15)*4, asuint(asuint(_e127)));
    const int _e132 = sqrtf(true);
    out_.Store(int(16)*4, asuint(uint(_e132)));
    const int _e137 = sqrtf(false);
    out_.Store(int(17)*4, asuint(uint(_e137)));
    const float _e143 = sqrtf_1(2.0, 3.0);
    out_.Store(int(18)*4, asuint(asuint(_e143)));
    local_6 = 1065353217u;
    uint _e149 = local_6;
    const uint _e152 = _naga_sqrtf(asuint(asfloat(_e149)));
    out_.Store(int(19)*4, asuint(asuint(asfloat(_e152))));
    local_7 = 1065353218u;
    uint _e159 = local_7;
    const uint _e162 = _naga_sqrtf(asuint(asfloat(_e159)));
    out_.Store(int(20)*4, asuint(asuint(asfloat(_e162))));
    local_8 = 1065353215u;
    uint _e169 = local_8;
    const uint _e172 = _naga_sqrtf(asuint(asfloat(_e169)));
    out_.Store(int(21)*4, asuint(asuint(asfloat(_e172))));
    const float _e178 = root_3(4294967295u);
    out_.Store(int(22)*4, asuint(asuint(_e178)));
    const float _e184 = root_2(naga_neg(int(4)));
    out_.Store(int(23)*4, asuint(asuint(_e184)));
    const float _e189 = root_1(16.0);
    out_.Store(int(24)*4, asuint(asuint(_e189)));
    const uint _e195 = _naga_sqrtf(asuint(0.25));
    out_.Store(int(25)*4, asuint(asuint(asfloat(_e195))));
    local_9 = 2147483649u;
    uint _e202 = local_9;
    const uint _e205 = _naga_sqrtf(asuint(asfloat(_e202)));
    out_.Store(int(26)*4, asuint(asuint(asfloat(_e205))));
    const float _e211 = root_4(false);
    out_.Store(int(27)*4, asuint(asuint(_e211)));
    const float _e216 = root_4(true);
    out_.Store(int(28)*4, asuint(asuint(_e216)));
    const float _e221 = root_1(0.5);
    out_.Store(int(29)*4, asuint(asuint(_e221)));
}
