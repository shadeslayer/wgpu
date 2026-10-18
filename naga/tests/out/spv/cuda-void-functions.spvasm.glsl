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

layout(set = 0, binding = 1, std140) uniform _14_13
{
    _8 _m0;
} _13;

void _32(bool _28, int _29, uint _30, float _31)
{
    int _37 = 0;
    float _42 = 0.0;
    bool _34 = false;
    uint _39 = 0u;
    _34 = _28;
    _37 = _29;
    _39 = _30;
    _42 = _31;
    _37 = int(uint(_37) + _39);
    if (_34)
    {
        return;
    }
    _37 = int(clamp(float(_37) * _42, -2147483648.0, 2147483520.0));
}

int _18(int _17)
{
    int _21 = 0;
    _21 = _17;
    return _21 + 1;
}

uint _78(uint _80, uint _81)
{
    return _80 % ((_81 == 0u) ? 1u : _81);
}

void _65(bool _63, int _64)
{
    bool _69 = false;
    int _71 = 0;
    _69 = _63;
    _71 = _64;
    _32(_69, _18(_71), 2u, 0.5);
}

void main()
{
    int _120 = 0;
    uint _114 = 0u;
    int _118 = 0;
    uint _112 = 0u;
    uint _116 = 0u;
    uvec2 _180 = uvec2(4294967295u);
    _112 = _13._m0._m0;
    _114 = (gl_WorkGroupID.x * 64u) + gl_LocalInvocationID.x;
    if (_114 < _112)
    {
        _116 = _114 * 6u;
        _118 = 2;
        int _137 = _118;
        _118 = _137 + 1;
        _32(false, _137, 1u, 1.5);
        _10._m0[_116] = _118;
        int _144 = _118;
        _118 = _144 + 1;
        _32(true, _18(_144), 2u, 0.5);
        _10._m0[_116 + 1u] = _118;
        int _155 = _118;
        _118 = _155 + 1;
        _65(_78(_114, 2u) == 0u, _155);
        _10._m0[_116 + 2u] = _118;
        _120 = 0;
        int _164 = _118;
        _118 = _164 + 1;
        float _168 = float(1u);
        _32(0 != 0, _164, uint(0), _168);
        for (;;)
        {
            if (all(equal(uvec2(0u), _180)))
            {
                break;
            }
            _180 -= uvec2(uint(_180.y == 0u), 1u);
            if (!(_120 < 3))
            {
                break;
            }
            _120++;
            if (_120 == 1)
            {
                int _201 = _118;
                _118 = _201 + 1;
                int _203 = _118;
                _32(false, _203, 0u, 1.0);
                continue;
            }
            int _201 = _118;
            _118 = _201 + 1;
            int _203 = _118;
            _32(false, _203, 0u, 1.0);
            continue;
        }
        _10._m0[_116 + 3u] = _118;
        _10._m0[_116 + 4u] = _120;
        _10._m0[_116 + 5u] = _18(_118);
    }
}

