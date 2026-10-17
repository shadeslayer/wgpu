#version 460
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0, std430) buffer _9_8
{
    uint _m0[];
} _8;

shared uint _11[64];
shared uint _13;

uint _15(uint _17, uint _18)
{
    return _17 % ((_18 == 0u) ? 1u : _18);
}

void main()
{
    uint _45 = 0u;
    uint _48 = 0u;
    if (gl_LocalInvocationIndex == 0u)
    {
        _11 = uint[](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
        _13 = 0u;
    }
    barrier();
    _45 = gl_LocalInvocationID.x;
    _48 = gl_WorkGroupID.x * 256u;
    _11[_45] = ((gl_WorkGroupID.x * 2000u) + _45) + 7u;
    _8._m0[_48 + _45] = (gl_WorkGroupID.x * 1000u) + _45;
    if (_45 == 0u)
    {
        _13 = (gl_WorkGroupID.x * 100u) + 11u;
    }
    memoryBarrierShared();
    memoryBarrierBuffer();
    barrier();
    _8._m0[(_48 + 64u) + _45] = _11[63u - _45];
    _8._m0[(_48 + 128u) + _45] = _8._m0[_48 + _15(_45 + 1u, 64u)];
    _8._m0[(_48 + 192u) + _45] = _13;
}

