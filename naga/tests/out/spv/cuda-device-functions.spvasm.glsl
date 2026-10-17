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

bool _40(int _39)
{
    int _43 = 0;
    _43 = _39;
    return _43 >= 0;
}

float _20(float _17, float _18, int _19)
{
    float _22 = 0.0;
    float _25 = 0.0;
    int _27 = 0;
    _22 = _17;
    _25 = _18;
    _27 = _19;
    _22 *= _25;
    return _22 + float(_27);
}

int _124(int _123)
{
    int _127 = 0;
    _127 = _123;
    if (_40(_127))
    {
        return int(clamp(_20(float(_127), 2.0, 1), -2147483648.0, 2147483520.0));
    }
    return -_127;
}

uint _48(uint _50, uint _51)
{
    return _50 % ((_51 == 0u) ? 1u : _51);
}

uint _61(uint _59, bool _60)
{
    bool _75 = false;
    bool _68 = false;
    uint _65 = 0u;
    uint _71 = 0u;
    uvec2 _90 = uvec2(4294967295u);
    _65 = _59;
    _68 = _60;
    _71 = 0u;
    uint _73 = 0u;
    for (;;)
    {
        if (all(equal(uvec2(0u), _90)))
        {
            break;
        }
        _90 -= uvec2(uint(_90.y == 0u), 1u);
        if (!(_73 < _65))
        {
            break;
        }
        _75 = _68;
        if (!_68)
        {
            _75 = _48(_73, 2u) != 0u;
        }
        if (_75)
        {
            _71 += _73;
        }
        _73++;
        continue;
    }
    return _71;
}

int _146(int _144, int _145)
{
    int _149 = 0;
    int _151 = 0;
    _149 = _144;
    _151 = _145;
    return (_149 * 10) + _151;
}

void main()
{
    int _193 = 0;
    uint _187 = 0u;
    int _191 = 0;
    uint _185 = 0u;
    uint _189 = 0u;
    _185 = _13._m0._m0;
    _187 = (gl_WorkGroupID.x * 64u) + gl_LocalInvocationID.x;
    if (_187 < _185)
    {
        _189 = _187 * 10u;
        _191 = int(_187) - 35;
        _193 = 3;
        _10._m0[_189] = _124(_191);
        _10._m0[_189 + 1u] = int(clamp(_20(float(_191), 1.5, 2), -2147483648.0, 2147483520.0));
        _10._m0[_189 + 2u] = int(_61(uint(5), true));
        _10._m0[_189 + 3u] = int(_61(5u, 0 != 0));
        float _240 = float(2u);
        _10._m0[_189 + 4u] = int(clamp(_20(float(_191), _240, int(true)), -2147483648.0, 2147483520.0));
        _10._m0[_189 + 5u] = _191;
        _10._m0[_189 + 6u] = _124(_191) + _124(_191 + 1);
        _10._m0[_189 + 7u] = int(_40(_191));
        int _267 = _193;
        int _268 = _193;
        _193 = _268 + 1;
        _10._m0[_189 + 8u] = _146(_267, _268);
        _10._m0[_189 + 9u] = _193;
    }
}

