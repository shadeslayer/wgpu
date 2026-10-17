#version 460
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct _6
{
    uint _m0;
};

layout(set = 0, binding = 0, std430) readonly buffer _9_8
{
    float _m0[];
} _8;

layout(set = 0, binding = 1, std430) readonly buffer _12_11
{
    float _m0[];
} _11;

layout(set = 0, binding = 2, std430) buffer _15_14
{
    float _m0[];
} _14;

layout(set = 0, binding = 3, std140) uniform _18_17
{
    _6 _m0;
} _17;

void main()
{
    uint _37 = 0u;
    uint _40 = 0u;
    _37 = _17._m0._m0;
    _40 = (gl_WorkGroupID.x * 64u) + gl_LocalInvocationID.x;
    if (_40 < _37)
    {
        _14._m0[_40] = _8._m0[_40] + (_11._m0[_40] * 2.0);
    }
}

