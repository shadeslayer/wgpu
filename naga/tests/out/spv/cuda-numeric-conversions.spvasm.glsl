#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct _6
{
    uint _m0;
};

layout(set = 0, binding = 0, std430) readonly buffer _10_9
{
    float _m0[];
} _9;

layout(set = 0, binding = 1, std430) buffer _13_12
{
    float _m0[];
} _12;

layout(set = 0, binding = 2, std140) uniform _16_15
{
    _6 _m0;
} _15;

void main()
{
    uint _33 = 0u;
    int _36 = 0;
    float _39 = 0.0;
    _33 = _15._m0._m0;
    _36 = int((gl_WorkGroupID.x * 1u) + gl_LocalInvocationID.x);
    if (uint(_36) < _33)
    {
        _39 = float(_36);
        _12._m0[_36] = _9._m0[_36] + _39;
    }
}

