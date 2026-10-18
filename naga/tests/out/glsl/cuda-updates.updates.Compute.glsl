#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct type_2 {
    int value;
};
layout(std430) buffer type_1_block_0Compute { int _group_0_binding_0_cs[]; };

layout(std140) uniform type_2_block_1Compute { type_2 _group_0_binding_1_cs; };


void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    int value = 0;
    int previous = 0;
    int _e3 = _group_0_binding_1_cs.value;
    value = _e3;
    int _e6 = value;
    value = (_e6 + 1);
    previous = _e6;
    int _e12 = previous;
    int _e13 = _group_0_binding_0_cs[threadIdx.x];
    _group_0_binding_0_cs[threadIdx.x] = (_e13 + _e12);
    int _e17 = value;
    value = (_e17 + 1);
    int _e20 = value;
    _group_0_binding_0_cs[threadIdx.x] = _e20;
}

