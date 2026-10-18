#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct _4
{
    float _m0;
};

layout(set = 0, binding = 0, std140) uniform _9_8
{
    _4 _m0;
} _8;

layout(set = 0, binding = 1, std430) readonly buffer _12_11
{
    float _m0[];
} _11;

layout(set = 0, binding = 2, std430) buffer _15_14
{
    float _m0[];
} _14;

void main()
{
    float _30 = 0.0;
    _30 = _8._m0._m0;
    _30 *= 2.0;
    _14._m0[gl_LocalInvocationID.x] = _11._m0[gl_LocalInvocationID.x] * _30;
}

