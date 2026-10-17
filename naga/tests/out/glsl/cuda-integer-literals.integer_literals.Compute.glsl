#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct type_2 {
    uint value;
};
layout(std430) buffer type_1_block_0Compute { uint _group_0_binding_0_cs[]; };

layout(std430) readonly buffer type_1_block_1Compute { uint _group_0_binding_1_cs[]; };

layout(std140) uniform type_2_block_2Compute { type_2 _group_0_binding_2_cs; };

shared uint shared_values[64];


void main() {
    if (gl_LocalInvocationID == uvec3(0u)) {
        shared_values = uint[64](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    }
    memoryBarrierShared();
    barrier();
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint value = 0u;
    uint shift = 0u;
    uint base = 0u;
    uint _e4 = _group_0_binding_2_cs.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e15 = i;
    uint _e16 = n;
    if ((_e15 < _e16)) {
        uint _e19 = i;
        uint _e21 = _group_0_binding_1_cs[_e19];
        value = _e21;
        uint _e24 = value;
        shift = (_e24 & 3u);
        uint _e28 = i;
        base = (_e28 * 18u);
        uint _e33 = value;
        shared_values[threadIdx.x] = (_e33 ^ uint(255));
        uint _e40 = shared_values[threadIdx.x];
        uint _e36 = base;
        _group_0_binding_0_cs[_e36] = _e40;
        uint _e46 = value;
        uint _e42 = base;
        _group_0_binding_0_cs[(_e42 + 1u)] = (_e46 & uint(255));
        uint _e54 = value;
        uint _e50 = base;
        _group_0_binding_0_cs[(_e50 + 2u)] = (_e54 & 240u);
        uint _e61 = value;
        uint _e57 = base;
        _group_0_binding_0_cs[(_e57 + 3u)] = (_e61 + 1000u);
        uint _e68 = shift;
        uint _e64 = base;
        _group_0_binding_0_cs[(_e64 + 4u)] = (2147483648u >> _e68);
        uint _e76 = shift;
        uint _e71 = base;
        _group_0_binding_0_cs[(_e71 + 5u)] = uint((int(2147483648u) >> _e76));
        uint _e84 = value;
        uint _e80 = base;
        _group_0_binding_0_cs[(_e80 + 6u)] = (4294967295u ^ _e84);
        uint _e91 = value;
        uint _e87 = base;
        _group_0_binding_0_cs[(_e87 + 7u)] = uint((_e91 < 2147483648u));
        uint _e98 = value;
        uint _e95 = base;
        _group_0_binding_0_cs[(_e95 + 8u)] = uint((int(_e98) < 2147483647));
        uint _e108 = shift;
        uint _e104 = base;
        _group_0_binding_0_cs[(_e104 + 9u)] = (2147483648u >> _e108);
        uint _e115 = value;
        uint _e111 = base;
        _group_0_binding_0_cs[(_e111 + 10u)] = (255u + _e115);
        uint _e122 = value;
        uint _e118 = base;
        _group_0_binding_0_cs[(_e118 + 11u)] = (((_e122 + 10u) + 8u) + 10u);
        uint _e133 = value;
        uint _e129 = base;
        _group_0_binding_0_cs[(_e129 + 12u)] = (_e133 + uint(15));
        uint _e141 = value;
        uint _e137 = base;
        _group_0_binding_0_cs[(_e137 + 13u)] = (4294967295u - _e141);
        uint _e148 = value;
        uint _e144 = base;
        _group_0_binding_0_cs[(_e144 + 14u)] = (3735928559u ^ _e148);
        uint _e156 = value;
        uint _e151 = base;
        _group_0_binding_0_cs[(_e151 + 15u)] = uint((~(2147483647) < int(_e156)));
        uint _e165 = value;
        uint _e161 = base;
        _group_0_binding_0_cs[(_e161 + 16u)] = ((_e165 + uint(30)) + uint(2));
        uint _e176 = value;
        uint _e172 = base;
        _group_0_binding_0_cs[(_e172 + 17u)] = ((_e176 + uint(30)) - uint(2));
    }
}

