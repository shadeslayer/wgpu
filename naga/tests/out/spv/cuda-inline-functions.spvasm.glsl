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

int _21(int _20)
{
    int _24 = 0;
    _24 = _20;
    _24 += 2;
    return _24;
}

int _33(int _32)
{
    int _34 = 0;
    _34 = _32;
    return _21(_34) * 2;
}

uint _42(uint _41)
{
    uint _45 = 0u;
    _45 = _41;
    return _45 + 3u;
}

uint _53(uint _52)
{
    uint _55 = 0u;
    _55 = _52;
    return _55 - 1u;
}

float _63(float _61, int _62)
{
    float _66 = 0.0;
    int _69 = 0;
    _66 = _61;
    _69 = _62;
    return (_66 * 1.5) + float(_69);
}

bool _80(int _78, int _79)
{
    int _82 = 0;
    int _84 = 0;
    _82 = _78;
    _84 = _79;
    return _82 == _84;
}

uint _119(uint _121, uint _122)
{
    return _121 % ((_122 == 0u) ? 1u : _122);
}

void _93(bool _91, int _92)
{
    bool _95 = false;
    int _98 = 0;
    _95 = _91;
    _98 = _92;
    if (_95)
    {
        return;
    }
    _98 = _33(_98);
}

void _109(bool _107, int _108)
{
    bool _110 = false;
    int _112 = 0;
    _110 = _107;
    _112 = _108;
    _93(_110, _21(_112));
}

void main()
{
    uint _155 = 0u;
    int _159 = 0;
    uint _153 = 0u;
    uint _157 = 0u;
    _153 = _16._m0._m0;
    _155 = (gl_WorkGroupID.x * 64u) + gl_LocalInvocationID.x;
    if (_155 < _153)
    {
        _157 = _155 * 12u;
        _159 = _13._m0[_155];
        _10._m0[_157] = _21(_159);
        _10._m0[_157 + 1u] = _159;
        _10._m0[_157 + 2u] = _33(_159);
        _10._m0[_157 + 3u] = int(_42(uint(_159)));
        _10._m0[_157 + 4u] = int(_53(uint(_159)));
        _10._m0[_157 + 5u] = int(clamp(_63(float(_159), 2), -2147483648.0, 2147483520.0));
        _10._m0[_157 + 6u] = int(_80(_159, _13._m0[_155]));
        int _230 = _159;
        _159 = _230 + 1;
        _93(_119(_155, 2u) == 0u, _230);
        _10._m0[_157 + 7u] = _159;
        int _240 = _159;
        _159 = _240 + 1;
        _109(_119(_155, 2u) != 0u, _240);
        _10._m0[_157 + 8u] = _159;
        _10._m0[_157 + 9u] = int(_80(_159, _13._m0[_155]));
        _10._m0[_157 + 10u] = _21(_21(_159));
        _10._m0[_157 + 11u] = _159;
    }
}

