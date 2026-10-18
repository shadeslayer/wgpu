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
    _24 += 3;
    return _24;
}

int _33(int _32)
{
    int _35 = 0;
    _35 = _32;
    return _21(_35) * 2;
}

uint _43(uint _42)
{
    uint _46 = 0u;
    _46 = _42;
    return _46 + 5u;
}

uint _54(uint _53)
{
    uint _56 = 0u;
    _56 = _53;
    return _56 - 2u;
}

float _64(float _62, int _63)
{
    float _67 = 0.0;
    int _70 = 0;
    _67 = _62;
    _70 = _63;
    return (_67 * 2.0) + float(_70);
}

bool _81(int _79, int _80)
{
    int _83 = 0;
    int _85 = 0;
    _83 = _79;
    _85 = _80;
    return _83 == _85;
}

uint _130(uint _132, uint _133)
{
    return _132 % ((_133 == 0u) ? 1u : _133);
}

void _94(bool _92, int _93)
{
    bool _96 = false;
    int _99 = 0;
    _96 = _92;
    _99 = _93;
    if (_96)
    {
        return;
    }
    _99 = _33(_99);
}

void _110(bool _108, int _109)
{
    bool _111 = false;
    int _113 = 0;
    _111 = _108;
    _113 = _109;
    _94(_111, _21(_113));
}

int _122(int _121)
{
    int _124 = 0;
    _124 = _121;
    return _21(_124) + 1;
}

void main()
{
    uint _166 = 0u;
    int _170 = 0;
    uint _164 = 0u;
    uint _168 = 0u;
    _164 = _16._m0._m0;
    _166 = (gl_WorkGroupID.x * 64u) + gl_LocalInvocationID.x;
    if (_166 < _164)
    {
        _168 = _166 * 12u;
        _170 = _13._m0[_166];
        _10._m0[_168] = _21(_170);
        _10._m0[_168 + 1u] = _170;
        _10._m0[_168 + 2u] = _33(_170);
        _10._m0[_168 + 3u] = int(_43(uint(_170)));
        _10._m0[_168 + 4u] = int(_54(uint(_170)));
        _10._m0[_168 + 5u] = int(clamp(_64(float(_170), 4), -2147483648.0, 2147483520.0));
        _10._m0[_168 + 6u] = int(_81(_170, _13._m0[_166]));
        int _241 = _170;
        _170 = _241 + 1;
        _94(_130(_166, 2u) == 0u, _241);
        _10._m0[_168 + 7u] = _170;
        int _251 = _170;
        _170 = _251 + 1;
        _110(_130(_166, 2u) != 0u, _251);
        _10._m0[_168 + 8u] = _170;
        _10._m0[_168 + 9u] = int(_81(_170, _13._m0[_166]));
        _10._m0[_168 + 10u] = _122(_21(_170));
        _10._m0[_168 + 11u] = _170;
    }
}

