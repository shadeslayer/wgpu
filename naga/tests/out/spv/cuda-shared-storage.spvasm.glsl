#version 460
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0, std430) buffer _11_10
{
    uint _m0[];
} _10;

shared uint _13[64];
shared int _15;
shared float _17;

void main()
{
    uint _38 = 0u;
    uint _41 = 0u;
    if (gl_LocalInvocationIndex == 0u)
    {
        _13 = uint[](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
        _15 = 0;
        _17 = 0.0;
    }
    barrier();
    _38 = gl_LocalInvocationID.x;
    _41 = gl_WorkGroupID.x * 128u;
    _13[_38] = ((gl_WorkGroupID.x * 1000u) + _38) + 10u;
    _10._m0[_41 + _38] = _13[_38];
    if (_38 == 0u)
    {
        int _80 = -7;
        _15 = _80;
        _17 = 3.5;
        _10._m0[_41 + 64u] = uint(_15 + 10);
        _10._m0[_41 + 65u] = uint(clamp(_17 * 2.0, 0.0, 4294967040.0));
    }
}

