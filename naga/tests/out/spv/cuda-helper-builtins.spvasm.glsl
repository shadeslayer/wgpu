#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct _8
{
    float _m0;
};

struct _9
{
    float _m0;
    float _m1;
    float _m2;
};

struct _11
{
    uint _m0;
};

layout(set = 0, binding = 0, std430) readonly buffer _19_18
{
    int _m0[];
} _18;

layout(set = 0, binding = 1, std140) uniform _22_21
{
    _8 _m0;
} _21;

layout(set = 0, binding = 2, std430) readonly buffer _25_24
{
    _9 _m0[];
} _24;

layout(set = 0, binding = 3, std140) uniform _28_27
{
    _11 _m0;
} _27;

layout(set = 0, binding = 4, std430) buffer _31_30
{
    uint _m0[];
} _30;

uvec3 _13 = uvec3(0u);
uvec3 _16 = uvec3(0u);
uvec3 _17 = uvec3(0u);

uint _60(uint _59)
{
    uint _61 = 0u;
    _61 = _59;
    if (_61 == 0u)
    {
        return _16.x;
    }
    if (_61 == 1u)
    {
        return _16.y;
    }
    return _16.z;
}

uint _80(uint _79)
{
    uint _81 = 0u;
    _81 = _79;
    if (_81 == 0u)
    {
        return 1u;
    }
    if (_81 == 1u)
    {
        return 1u;
    }
    return 1u;
}

uint _35(uint _34)
{
    uint _39 = 0u;
    _39 = _34;
    if (_39 == 0u)
    {
        return _13.x;
    }
    if (_39 == 1u)
    {
        return _13.y;
    }
    return _13.z;
}

uint _114(uint _113)
{
    uint _115 = 0u;
    _115 = _113;
    return _35(_115);
}

uint _134()
{
    return (_60(0u) * _80(0u)) + _114(0u);
}

void _150()
{
    if (_17.z != 0u)
    {
        return;
    }
    else
    {
        return;
    }
}

uint _94(uint _93)
{
    uint _95 = 0u;
    _95 = _93;
    if (_95 == 0u)
    {
        return _17.x;
    }
    if (_95 == 1u)
    {
        return _17.y;
    }
    return _17.z;
}

float _122(float _121)
{
    float _124 = 0.0;
    _124 = _121;
    return float(_16.x) + _124;
}

uint _144(uint _143)
{
    uint _146 = 0u;
    _146 = _143;
    return 11u;
}

void main()
{
    float _210 = 0.0;
    uint _212 = 0u;
    _210 = _21._m0._m0;
    _212 = _27._m0._m0;
    _13 = gl_LocalInvocationID;
    _16 = gl_WorkGroupID;
    _17 = gl_NumWorkGroups;
    if (_134() >= _212)
    {
        return;
    }
    _150();
    _30._m0[0u] = _35(0u);
    _30._m0[1u] = _35(1u);
    _30._m0[2u] = _35(2u);
    _30._m0[3u] = _60(0u);
    _30._m0[4u] = _60(1u);
    _30._m0[5u] = _60(2u);
    _30._m0[6u] = _80(0u);
    _30._m0[7u] = _80(1u);
    _30._m0[8u] = _80(2u);
    _30._m0[9u] = _94(0u);
    _30._m0[10u] = _94(1u);
    _30._m0[11u] = _94(2u);
    _30._m0[12u] = _134();
    _30._m0[13u] = _114(0u);
    _30._m0[14u] = floatBitsToUint(_122(2.5));
    _30._m0[15u] = gl_LocalInvocationID.x;
    _30._m0[16u] = gl_WorkGroupID.y;
    _30._m0[17u] = gl_NumWorkGroups.z;
    _30._m0[18u] = 1u;
    _30._m0[19u] = uint(_18._m0[0u]);
    _30._m0[20u] = floatBitsToUint(_210);
    _30._m0[21u] = floatBitsToUint(_24._m0[0u]._m0);
    _30._m0[22u] = floatBitsToUint(_24._m0[0u]._m1);
    _30._m0[23u] = floatBitsToUint(_24._m0[0u]._m2);
    _30._m0[24u] = _212;
    _30._m0[25u] = ((_35(0u) + _60(1u)) + _80(2u)) + _94(0u);
    _30._m0[26u] = _134() + _134();
    _30._m0[27u] = floatBitsToUint((float(_18._m0[0u]) * _210) + _24._m0[0u]._m0);
    _30._m0[28u] = _144(gl_LocalInvocationID.x);
    _30._m0[29u] = _94(2u);
}

