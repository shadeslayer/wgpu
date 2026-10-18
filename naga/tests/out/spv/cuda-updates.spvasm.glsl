#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct _5
{
    int _m0;
};

layout(set = 0, binding = 0, std430) buffer _9_8
{
    int _m0[];
} _8;

layout(set = 0, binding = 1, std140) uniform _12_11
{
    _5 _m0;
} _11;

void main()
{
    int _26 = 0;
    int _29 = 0;
    _26 = _11._m0._m0;
    int _35 = _26;
    _26 = _35 + 1;
    _29 = _35;
    _8._m0[gl_LocalInvocationID.x] += _29;
    _26++;
    _8._m0[gl_LocalInvocationID.x] = _26;
}

