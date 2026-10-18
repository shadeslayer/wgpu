#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct _6
{
    int _m0;
};

struct _7
{
    uint _m0;
};

layout(set = 0, binding = 0, std430) buffer _10_9
{
    uint _m0[];
} _9;

layout(set = 0, binding = 1, std140) uniform _13_12
{
    _6 _m0;
} _12;

layout(set = 0, binding = 2, std140) uniform _16_15
{
    _7 _m0;
} _15;

int _18(int _20, int _21)
{
    int _33 = ((_21 == 0) || ((_20 == int(0x80000000)) && (_21 == (-1)))) ? 1 : _21;
    return _20 - ((_20 / _33) * _33);
}

uint _37(uint _39, uint _40)
{
    return _39 % ((_40 == 0u) ? 1u : _40);
}

void main()
{
    uint _74 = 0u;
    uint _67 = 0u;
    uint _72 = 0u;
    int _64 = 0;
    int _70 = 0;
    _64 = _12._m0._m0;
    _67 = _15._m0._m0;
    int _83 = -3;
    _70 = _18(_64, _83);
    _72 = uint(_70) & 15u;
    _72 |= 8u;
    _72 ^= 1u;
    _74 = _67;
    _72 = _72 << _74;
    _72 = _72 >> 1u;
    _72 = _37(_72, 97u);
    _72 &= 255u;
    _9._m0[gl_LocalInvocationID.x] = ~_72;
}

