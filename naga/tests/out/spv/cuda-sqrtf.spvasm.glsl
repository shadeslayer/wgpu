#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0, std430) buffer _9_8
{
    uint _m0[];
} _8;

uint _13(uint _12)
{
    uint _42 = 0u;
    uint _36 = 0u;
    uint _40 = 0u;
    uint _33 = 0u;
    uint _44 = 0u;
    uint _38 = 0u;
    uvec2 _91 = uvec2(4294967295u);
    uint _49 = _12 & 2147483647u;
    if ((_49 == 0u) || (_12 == 2139095040u))
    {
        return _12;
    }
    if (((_12 & 2147483648u) != 0u) || (_49 > 2139095040u))
    {
        return 2143289344u;
    }
    uint _61 = _12 & 8388607u;
    uint _62 = _49 >> 23u;
    _33 = _61 | 8388608u;
    _36 = _62 + 127u;
    if (_62 == 0u)
    {
        uint _71 = (31u - uint(findMSB(_61))) - 8u;
        _33 = _61 << _71;
        _36 = 128u - _71;
    }
    uint _76 = _36 & 1u;
    _38 = _33 >> (9u - _76);
    _40 = _33 << (23u + _76);
    _42 = 0u;
    _44 = 0u;
    uint _46 = 0u;
    for (;;)
    {
        if (all(equal(uvec2(0u), _91)))
        {
            break;
        }
        _91 -= uvec2(uint(_91.y == 0u), 1u);
        if (_46 >= 24u)
        {
            break;
        }
        uint _106 = _38;
        uint _109 = _44;
        uint _117 = (_109 << 2u) | (_106 >> 14u);
        uint _119 = (_42 << 2u) | 1u;
        bool _120 = _117 >= _119;
        uint _123 = _42 << 1u;
        _38 = ((_106 << 2u) | (_40 >> 30u)) & 65535u;
        _40 = _40 << 2u;
        _44 = _120 ? (_117 - _119) : _117;
        _42 = _120 ? (_123 | 1u) : _123;
        _46++;
        continue;
    }
    return ((_36 >> 1u) << 23u) | (((_44 > _42) ? (_42 + 1u) : _42) & 8388607u);
}

float _151(int _150)
{
    int _153 = 0;
    _153 = _150;
    return uintBitsToFloat(_13(floatBitsToUint(float(_153))));
}

float _164(uint _163)
{
    uint _166 = 0u;
    _166 = _163;
    return uintBitsToFloat(_13(floatBitsToUint(float(_166))));
}

float _139(float _138)
{
    float _141 = 0.0;
    _141 = _138;
    return uintBitsToFloat(_13(floatBitsToUint(_141)));
}

int _191(bool _190)
{
    bool _195 = false;
    int _197 = 0;
    _195 = _190;
    if (_195)
    {
        _197 = 7;
    }
    else
    {
        _197 = 9;
    }
    return _197;
}

float _208(float _206, float _207)
{
    float _210 = 0.0;
    float _212 = 0.0;
    _210 = _206;
    _212 = _207;
    return _210 + _212;
}

float _176(bool _175)
{
    bool _178 = false;
    _178 = _175;
    return uintBitsToFloat(_13(floatBitsToUint(float(_178))));
}

void main()
{
    uint _284 = 0u;
    float _278 = 0.0;
    uint _272 = 0u;
    uint _266 = 0u;
    uint _286 = 0u;
    uint _280 = 0u;
    uint _274 = 0u;
    uint _268 = 0u;
    uint _282 = 0u;
    uint _276 = 0u;
    uint _270 = 0u;
    _8._m0[0u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(0.0))));
    float _295 = -0.0;
    _8._m0[1u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(_295))));
    _8._m0[2u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(1.0))));
    _8._m0[3u] = floatBitsToUint(_151(4));
    _8._m0[4u] = floatBitsToUint(_164(9u));
    _8._m0[5u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(6.25))));
    _8._m0[6u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(2.0))));
    _266 = 1u;
    _8._m0[7u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(uintBitsToFloat(_266)))));
    _268 = 8388607u;
    _8._m0[8u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(uintBitsToFloat(_268)))));
    _270 = 8388608u;
    _8._m0[9u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(uintBitsToFloat(_270)))));
    _272 = 2139095039u;
    _8._m0[10u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(uintBitsToFloat(_272)))));
    _274 = 2139095040u;
    _8._m0[11u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(uintBitsToFloat(_274)))));
    float _364 = -1.0;
    _8._m0[12u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(_364))));
    _276 = 2139095041u;
    _8._m0[13u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(uintBitsToFloat(_276)))));
    _278 = 4.0;
    float _379 = _278;
    float _380 = float(1);
    _278 = _379 + _380;
    _8._m0[14u] = floatBitsToUint(_139(_379));
    _8._m0[15u] = floatBitsToUint(_278);
    _8._m0[16u] = uint(_191(true));
    _8._m0[17u] = uint(_191(false));
    _8._m0[18u] = floatBitsToUint(_208(2.0, 3.0));
    _280 = 1065353217u;
    _8._m0[19u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(uintBitsToFloat(_280)))));
    _282 = 1065353218u;
    _8._m0[20u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(uintBitsToFloat(_282)))));
    _284 = 1065353215u;
    _8._m0[21u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(uintBitsToFloat(_284)))));
    _8._m0[22u] = floatBitsToUint(_164(4294967295u));
    int _429 = -4;
    _8._m0[23u] = floatBitsToUint(_151(_429));
    _8._m0[24u] = floatBitsToUint(_139(16.0));
    _8._m0[25u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(0.25))));
    _286 = 2147483649u;
    _8._m0[26u] = floatBitsToUint(uintBitsToFloat(_13(floatBitsToUint(uintBitsToFloat(_286)))));
    _8._m0[27u] = floatBitsToUint(_176(false));
    _8._m0[28u] = floatBitsToUint(_176(true));
    _8._m0[29u] = floatBitsToUint(_139(0.5));
}

