#version 460
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct _8
{
    uint _m0;
};

layout(set = 0, binding = 0, std430) buffer _14_13
{
    uint _m0[];
} _13;

layout(set = 0, binding = 1, std430) readonly buffer _17_16
{
    int _m0[];
} _16;

layout(set = 0, binding = 2, std140) uniform _20_19
{
    _8 _m0;
} _19;

shared uint _22[64];

int _28(float _25, float _26, int _27)
{
    float _30 = 0.0;
    float _33 = 0.0;
    int _35 = 0;
    _30 = _25;
    _33 = _26;
    _35 = _27;
    return int(clamp((_30 * _33) + float(_35), -2147483648.0, 2147483520.0));
}

uint _49(uint _51, uint _52)
{
    return _51 % ((_52 == 0u) ? 1u : _52);
}

void main()
{
    int _111 = 0;
    int _104 = 0;
    uint _98 = 0u;
    uint _91 = 0u;
    int _113 = 0;
    float _106 = 0.0;
    int _100 = 0;
    uint _94 = 0u;
    float _115 = 0.0;
    bool _108 = false;
    float _102 = 0.0;
    uint _96 = 0u;
    uvec2 _212 = uvec2(4294967295u);
    if (gl_LocalInvocationIndex == 0u)
    {
        _22 = uint[](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    }
    barrier();
    _91 = _19._m0._m0;
    _94 = (gl_WorkGroupID.x * 64u) + gl_LocalInvocationID.x;
    if (_94 < _91)
    {
        _96 = _94 * 10u;
        _98 = gl_LocalInvocationID.x;
        _22[_98] = _94 + 10u;
        _13._m0[_96] = _22[_98];
        _13._m0[_96 + 1u] = uint(_28(float(_16._m0[_94]), 2.0, 1));
        int _163 = -3;
        _100 = _163;
        _13._m0[_96 + 2u] = uint(_100);
        _102 = 3.75;
        _13._m0[_96 + 3u] = uint(clamp(_102, 0.0, 4294967040.0));
        int _179 = -5;
        _104 = _179;
        _13._m0[_96 + 4u] = uint(_104);
        _106 = 1.25;
        _13._m0[_96 + 5u] = uint(clamp(_106 * 4.0, 0.0, 4294967040.0));
        _108 = _49(_94, 2u) == 0u;
        _13._m0[_96 + 6u] = uint(_108);
        _111 = 0;
        for (;;)
        {
            if (all(equal(uvec2(0u), _212)))
            {
                break;
            }
            _212 -= uvec2(uint(_212.y == 0u), 1u);
            if (!(_111 < 3))
            {
                break;
            }
            _113 = _111;
            _111 = _113 + 1;
            continue;
        }
        _13._m0[_96 + 7u] = uint(_111);
        _115 = 1.5;
        _13._m0[_96 + 8u] = uint(clamp(_115 * 2.0, 0.0, 4294967040.0));
        _13._m0[_96 + 9u] = uint(clamp(float(_91), 0.0, 4294967040.0));
    }
}

