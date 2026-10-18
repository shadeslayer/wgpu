#version 460
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct _8
{
    uint _m0;
};

layout(set = 0, binding = 0, std430) buffer _11_10
{
    int _m0[];
} _10;

layout(set = 0, binding = 1, std430) readonly buffer _14_13
{
    int _m0[];
} _13;

layout(set = 0, binding = 2, std140) uniform _17_16
{
    _8 _m0;
} _16;

uint _60(uint _62, uint _63)
{
    return _62 % ((_63 == 0u) ? 1u : _63);
}

int _22(int _20, bool _21)
{
    int _26 = 0;
    bool _29 = false;
    int _32 = 0;
    _26 = _20;
    _29 = _21;
    if (_29)
    {
        _32 = _26 + 3;
    }
    else
    {
        _32 = _26 - 2;
    }
    return _32;
}

uint _72(uint _71)
{
    uint _79 = 0u;
    uint _83 = 0u;
    uint _76 = 0u;
    uvec2 _98 = uvec2(4294967295u);
    _76 = _71;
    _79 = 0u;
    uint _81 = 0u;
    for (;;)
    {
        if (all(equal(uvec2(0u), _98)))
        {
            break;
        }
        _98 -= uvec2(uint(_98.y == 0u), 1u);
        if (!(_81 < _76))
        {
            break;
        }
        if (_60(_81, 2u) == 0u)
        {
            _83 = _81 + 1u;
        }
        else
        {
            _83 = _81 * 2u;
        }
        _79 += _83;
        _81++;
        continue;
    }
    return _79;
}

float _46(float _45)
{
    float _50 = 0.0;
    float _53 = 0.0;
    _50 = _45;
    _53 = (_50 * 2.0) + 4.0;
    return _53;
}

void main()
{
    int _193 = 0;
    int _187 = 0;
    int _181 = 0;
    uint _175 = 0u;
    int _169 = 0;
    uint _163 = 0u;
    int _197 = 0;
    int _191 = 0;
    bool _173 = false;
    int _167 = 0;
    uint _161 = 0u;
    int _195 = 0;
    int _189 = 0;
    int _183 = 0;
    int _177 = 0;
    float _171 = 0.0;
    uint _165 = 0u;
    uvec2 _280 = uvec2(4294967295u);
    uvec2 _313 = uvec2(4294967295u);
    uvec2 _345 = uvec2(4294967295u);
    _161 = _16._m0._m0;
    _163 = (gl_WorkGroupID.x * 64u) + gl_LocalInvocationID.x;
    if (_163 < _161)
    {
        _165 = _163 * 12u;
        _167 = _13._m0[_163];
        if (_60(_163, 2u) == 0u)
        {
            _169 = _167 + 7;
        }
        else
        {
            _169 = _167 - 4;
        }
        _10._m0[_165] = _169;
        _10._m0[_165 + 1u] = _22(_167, _60(_163, 3u) == 0u);
        _171 = (float(_167) * 1.5) + 2.0;
        _10._m0[_165 + 2u] = int(clamp(_171, -2147483648.0, 2147483520.0));
        _173 = _167 < 0;
        _10._m0[_165 + 3u] = int(_173);
        _175 = uint(_167 + 5);
        _10._m0[_165 + 4u] = int(_175 * 3u);
        _10._m0[_165 + 5u] = int(_72(_60(_163, 5u)));
        _177 = 0;
        int _179 = 0;
        for (;;)
        {
            if (all(equal(uvec2(0u), _280)))
            {
                break;
            }
            _280 -= uvec2(uint(_280.y == 0u), 1u);
            if (!(_179 < 3))
            {
                break;
            }
            _181 = _167 + _179;
            _177 += _181;
            _179++;
            continue;
        }
        _10._m0[_165 + 6u] = _177;
        _183 = 0;
        int _185 = 0;
        for (;;)
        {
            if (all(equal(uvec2(0u), _313)))
            {
                break;
            }
            _313 -= uvec2(uint(_313.y == 0u), 1u);
            if (!(_185 < 3))
            {
                break;
            }
            _187 = _167 - _185;
            _183 += _187;
            _185++;
            continue;
        }
        _10._m0[_165 + 7u] = _183;
        _189 = 2;
        _191 = _167;
        for (;;)
        {
            if (all(equal(uvec2(0u), _345)))
            {
                break;
            }
            _345 -= uvec2(uint(_345.y == 0u), 1u);
            if (!(_189 > 0))
            {
                break;
            }
            _193 = _191 + _189;
            _191 = _193;
            _189--;
            continue;
        }
        _10._m0[_165 + 8u] = _191;
        _195 = _167;
        _197 = _167 + 10;
        _10._m0[_165 + 9u] = _197;
        _10._m0[_165 + 10u] = _195;
        _10._m0[_165 + 11u] = int(clamp(_46(float(_167)), -2147483648.0, 2147483520.0));
    }
}

