#version 460
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct _6
{
    uint _m0;
};

layout(set = 0, binding = 0, std430) buffer _9_8
{
    float _m0[];
} _8;

layout(set = 0, binding = 1, std430) readonly buffer _12_11
{
    float _m0[];
} _11;

layout(set = 0, binding = 2, std140) uniform _15_14
{
    _6 _m0;
} _14;

void main()
{
    uint _55 = 0u;
    uint _58 = 0u;
    uint _60 = 0u;
    _55 = _14._m0._m0;
    _58 = (gl_WorkGroupID.x * 64u) + gl_LocalInvocationID.x;
    if (_58 < _55)
    {
        _60 = _58 * 12u;
        _8._m0[_60] = 1024.5;
        _8._m0[_60 + 1u] = 0.125;
        _8._m0[_60 + 2u] = 12.0;
        _8._m0[_60 + 3u] = 25.0;
        _8._m0[_60 + 4u] = 0.5;
        _8._m0[_60 + 5u] = 6.25;
        float _96 = -1024.5;
        _8._m0[_60 + 6u] = _96;
        _8._m0[_60 + 7u] = 0.125;
        _8._m0[_60 + 8u] = 3.1415927410125732421875;
        _8._m0[_60 + 9u] = 1.0;
        _8._m0[_60 + 10u] = 1.0000002384185791015625;
        _8._m0[_60 + 11u] = (_11._m0[_58] * 0.25) + 10.0;
    }
}

