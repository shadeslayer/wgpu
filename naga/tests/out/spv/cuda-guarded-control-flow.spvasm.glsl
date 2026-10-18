#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

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
    uint _36 = 0u;
    uint _39 = 0u;
    _36 = _17._m0._m0;
    _39 = (gl_WorkGroupID.x * 1u) + gl_LocalInvocationID.x;
    if (_39 < _36)
    {
        _14._m0[_39] = _8._m0[_39] + _11._m0[_39];
    }
}

