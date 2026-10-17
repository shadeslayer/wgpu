#version 460
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct _5
{
    uint _m0;
};

layout(set = 0, binding = 0, std430) buffer _10_9
{
    uint _m0[];
} _9;

layout(set = 0, binding = 1, std430) readonly buffer _13_12
{
    uint _m0[];
} _12;

layout(set = 0, binding = 2, std140) uniform _16_15
{
    _5 _m0;
} _15;

shared uint _18[64];

void main()
{
    uint _73 = 0u;
    uint _67 = 0u;
    uint _71 = 0u;
    uint _64 = 0u;
    uint _69 = 0u;
    if (gl_LocalInvocationIndex == 0u)
    {
        _18 = uint[](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    }
    barrier();
    _64 = _15._m0._m0;
    _67 = (gl_WorkGroupID.x * 64u) + gl_LocalInvocationID.x;
    if (_67 < _64)
    {
        _69 = _12._m0[_67];
        _71 = _69 & 3u;
        _73 = _67 * 18u;
        _18[gl_LocalInvocationID.x] = _69 ^ uint(255);
        _9._m0[_73] = _18[gl_LocalInvocationID.x];
        _9._m0[_73 + 1u] = _69 & uint(255);
        _9._m0[_73 + 2u] = _69 & 240u;
        _9._m0[_73 + 3u] = _69 + 1000u;
        _9._m0[_73 + 4u] = 2147483648u >> _71;
        _9._m0[_73 + 5u] = uint(int(2147483648u) >> int(_71));
        _9._m0[_73 + 6u] = 4294967295u ^ _69;
        _9._m0[_73 + 7u] = uint(_69 < 2147483648u);
        _9._m0[_73 + 8u] = uint(int(_69) < 2147483647);
        _9._m0[_73 + 9u] = 2147483648u >> _71;
        _9._m0[_73 + 10u] = 255u + _69;
        _9._m0[_73 + 11u] = ((_69 + 10u) + 8u) + 10u;
        _9._m0[_73 + 12u] = _69 + uint(15);
        _9._m0[_73 + 13u] = 4294967295u - _69;
        _9._m0[_73 + 14u] = 3735928559u ^ _69;
        int _196 = ~2147483647;
        _9._m0[_73 + 15u] = uint(_196 < int(_69));
        _9._m0[_73 + 16u] = (_69 + uint(30)) + uint(2);
        _9._m0[_73 + 17u] = (_69 + uint(30)) - uint(2);
    }
}

