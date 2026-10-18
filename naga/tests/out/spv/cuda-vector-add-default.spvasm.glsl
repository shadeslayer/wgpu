#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0, std430) readonly buffer _8_7
{
    float _m0[];
} _7;

layout(set = 0, binding = 1, std430) readonly buffer _11_10
{
    float _m0[];
} _10;

layout(set = 0, binding = 2, std430) buffer _14_13
{
    float _m0[];
} _13;

void main()
{
    uint _31 = 0u;
    _31 = (gl_WorkGroupID.x * 1u) + gl_LocalInvocationID.x;
    _13._m0[_31] = _7._m0[_31] + (_10._m0[_31] * 2.0);
}

