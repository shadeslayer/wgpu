#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct type_3 {
    uint value;
};
layout(std430) readonly buffer type_1_block_0Compute { float _group_0_binding_0_cs[]; };

layout(std430) buffer type_1_block_1Compute { float _group_0_binding_1_cs[]; };

layout(std140) uniform type_3_block_2Compute { type_3 _group_0_binding_2_cs; };


void main() {
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    int i = 0;
    float offset = 0.0;
    uint _e4 = _group_0_binding_2_cs.value;
    n = _e4;
    i = int(((blockIdx.x * 1u) + threadIdx.x));
    int _e15 = i;
    uint _e17 = n;
    if ((uint(_e15) < _e17)) {
        int _e20 = i;
        offset = float(_e20);
        int _e24 = i;
        float _e26 = _group_0_binding_0_cs[_e24];
        float _e27 = offset;
        int _e22 = i;
        _group_0_binding_1_cs[_e22] = (_e26 + _e27);
    }
}

