#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct type_3 {
    uint value;
};
layout(std430) readonly buffer type_1_block_0Compute { float _group_0_binding_0_cs[]; };

layout(std430) readonly buffer type_1_block_1Compute { float _group_0_binding_1_cs[]; };

layout(std430) buffer type_1_block_2Compute { float _group_0_binding_2_cs[]; };

layout(std140) uniform type_3_block_3Compute { type_3 _group_0_binding_3_cs; };


void main() {
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint _e5 = _group_0_binding_3_cs.value;
    n = _e5;
    i = ((blockIdx.x * 1u) + threadIdx.x);
    uint _e15 = i;
    uint _e16 = n;
    if ((_e15 < _e16)) {
        uint _e20 = i;
        uint _e22 = i;
        float _e24 = _group_0_binding_0_cs[_e20];
        float _e25 = _group_0_binding_1_cs[_e22];
        uint _e18 = i;
        _group_0_binding_2_cs[_e18] = (_e24 + _e25);
    }
}

