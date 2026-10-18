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
    int _37 = 0;
    int _41 = 0;
    int _34 = 0;
    uvec2 _61 = uvec2(4294967295u);
    uvec2 _94 = uvec2(4294967295u);
    uvec2 _123 = uvec2(4294967295u);
    _34 = _11._m0._m0;
    _37 = 0;
    int _39 = 0;
    for (;;)
    {
        if (all(equal(uvec2(0u), _61)))
        {
            break;
        }
        _61 -= uvec2(uint(_61.y == 0u), 1u);
        if (!(_39 < _34))
        {
            break;
        }
        if (_39 == 2)
        {
            int _88 = _39;
            _39 = _88 + 1;
            continue;
        }
        if (_39 == 7)
        {
            break;
        }
        _37 += _39;
        int _88 = _39;
        _39 = _88 + 1;
        continue;
    }
    _41 = 0;
    for (;;)
    {
        if (all(equal(uvec2(0u), _94)))
        {
            break;
        }
        _94 -= uvec2(uint(_94.y == 0u), 1u);
        if (!(_41 < 3))
        {
            break;
        }
        _41++;
        if (_41 == 2)
        {
            continue;
        }
        _37 += 10;
        continue;
    }
    do
    {
        if (all(equal(uvec2(0u), _123)))
        {
            break;
        }
        _123 -= uvec2(uint(_123.y == 0u), 1u);
        _37 += 100;
        if (_37 > 200)
        {
            break;
        }
    } while (!(!(_37 < 150)));
    _8._m0[gl_LocalInvocationID.x] = _37;
}

