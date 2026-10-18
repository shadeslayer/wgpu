#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct type_1 {
    float value;
};
layout(std140) uniform type_1_block_0Compute { type_1 _group_0_binding_0_cs; };

layout(std430) readonly buffer type_2_block_1Compute { float _group_0_binding_1_cs[]; };

layout(std430) buffer type_2_block_2Compute { float _group_0_binding_2_cs[]; };


void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    float factor = 0.0;
    float _e2 = _group_0_binding_0_cs.value;
    factor = _e2;
    float _e7 = factor;
    factor = (_e7 * 2.0);
    float _e14 = _group_0_binding_1_cs[threadIdx.x];
    float _e15 = factor;
    _group_0_binding_2_cs[threadIdx.x] = (_e14 * _e15);
}

