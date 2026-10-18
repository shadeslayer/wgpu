#version 460
layout(local_size_x = 8, local_size_y = 1, local_size_z = 1) in;

struct _4
{
    float _m0;
    float _m1;
    float _m2;
};

layout(set = 0, binding = 0, std430) buffer _13_12
{
    float _m0[];
} _12;

shared _4 _15[8];
shared _4 _17;

uint _43(uint _45, uint _46)
{
    return _45 % ((_46 == 0u) ? 1u : _46);
}

_4 _21(_4 _20)
{
    _4 _24 = _4(0.0, 0.0, 0.0);
    _24 = _20;
    _24._m0 += 1.0;
    _24._m1 = _24._m2 - _24._m1;
    return _24;
}

void main()
{
    bool _88 = false;
    uint _81 = 0u;
    uint _74 = 0u;
    uint _91 = 0u;
    _4 _83 = _4(0.0, 0.0, 0.0);
    uint _77 = 0u;
    int _85 = 0;
    float _79 = 0.0;
    if (gl_LocalInvocationIndex == 0u)
    {
        _15 = _4[](_4(0.0, 0.0, 0.0), _4(0.0, 0.0, 0.0), _4(0.0, 0.0, 0.0), _4(0.0, 0.0, 0.0), _4(0.0, 0.0, 0.0), _4(0.0, 0.0, 0.0), _4(0.0, 0.0, 0.0), _4(0.0, 0.0, 0.0));
        _17 = _4(0.0, 0.0, 0.0);
    }
    barrier();
    _74 = gl_LocalInvocationID.x;
    _77 = gl_WorkGroupID.x;
    _79 = float((_77 * 16u) + _74);
    _15[_74] = _4(_79 + 0.5, (-_79) - 1.25, _79 + 2.5);
    if (_74 == 0u)
    {
        _17 = _4(float(_77) + 0.25, float(_77) + 0.5, float(_77) + 0.75);
    }
    memoryBarrierShared();
    memoryBarrierBuffer();
    barrier();
    _81 = _43(_74 + 1u, 8u);
    _83 = _21(_15[_81]);
    _85 = 1;
    _83._m0 += float(_85);
    memoryBarrierShared();
    memoryBarrierBuffer();
    barrier();
    _88 = (_74 & 1u) != 0u;
    if (_88)
    {
        _15[_74]._m1 += _17._m0;
    }
    else
    {
        _83._m1 += _17._m0;
    }
    _15[_74]._m2 = _83._m0;
    _91 = ((_77 * 8u) + _74) * 9u;
    _12._m0[_91] = _83._m0;
    _12._m0[_91 + 1u] = _83._m1;
    _12._m0[_91 + 2u] = _83._m2;
    memoryBarrierShared();
    memoryBarrierBuffer();
    barrier();
    _12._m0[_91 + 3u] = _15[_81]._m0;
    _12._m0[_91 + 4u] = _15[_81]._m1;
    _12._m0[_91 + 5u] = _15[_81]._m2;
    _12._m0[_91 + 6u] = _17._m0;
    _12._m0[_91 + 7u] = _17._m1;
    _12._m0[_91 + 8u] = _17._m2;
}

