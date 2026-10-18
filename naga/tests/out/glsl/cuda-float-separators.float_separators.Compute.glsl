#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct type_3 {
    uint value;
};
layout(std430) buffer type_1_block_0Compute { float _group_0_binding_0_cs[]; };

layout(std430) readonly buffer type_1_block_1Compute { float _group_0_binding_1_cs[]; };

layout(std140) uniform type_3_block_2Compute { type_3 _group_0_binding_2_cs; };


void main() {
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint base = 0u;
    uint _e4 = _group_0_binding_2_cs.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e14 = i;
    uint _e15 = n;
    if ((_e14 < _e15)) {
        uint _e19 = i;
        base = (_e19 * 12u);
        uint _e21 = base;
        _group_0_binding_0_cs[_e21] = 1024.5;
        uint _e25 = base;
        _group_0_binding_0_cs[(_e25 + 1u)] = 0.125;
        uint _e30 = base;
        _group_0_binding_0_cs[(_e30 + 2u)] = 12.0;
        uint _e35 = base;
        _group_0_binding_0_cs[(_e35 + 3u)] = 25.0;
        uint _e40 = base;
        _group_0_binding_0_cs[(_e40 + 4u)] = 0.5;
        uint _e45 = base;
        _group_0_binding_0_cs[(_e45 + 5u)] = 6.25;
        uint _e50 = base;
        _group_0_binding_0_cs[(_e50 + 6u)] = -(1024.5);
        uint _e56 = base;
        _group_0_binding_0_cs[(_e56 + 7u)] = 0.125;
        uint _e61 = base;
        _group_0_binding_0_cs[(_e61 + 8u)] = 3.1415927;
        uint _e66 = base;
        _group_0_binding_0_cs[(_e66 + 9u)] = 1.0;
        uint _e71 = base;
        _group_0_binding_0_cs[(_e71 + 10u)] = 1.0000002;
        uint _e79 = i;
        float _e82 = _group_0_binding_1_cs[_e79];
        uint _e76 = base;
        _group_0_binding_0_cs[(_e76 + 11u)] = ((_e82 * 0.25) + 10.0);
    }
}

