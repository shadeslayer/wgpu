#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct type_3 {
    int value;
};
struct type_4 {
    uint value;
};
layout(std430) buffer type_1_block_0Compute { uint _group_0_binding_0_cs[]; };

layout(std140) uniform type_3_block_1Compute { type_3 _group_0_binding_1_cs; };

layout(std140) uniform type_4_block_2Compute { type_4 _group_0_binding_2_cs; };


void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    int value = 0;
    uint amount = 0u;
    int remainder = 0;
    uint bits = 0u;
    uint local = 0u;
    int _e3 = _group_0_binding_1_cs.value;
    value = _e3;
    uint _e7 = _group_0_binding_2_cs.value;
    amount = _e7;
    int _e11 = -(3);
    int _e12 = value;
    remainder = (_e12 - _e11 * (_e12 / _e11));
    int _e15 = remainder;
    bits = (uint(_e15) & 15u);
    uint _e20 = bits;
    bits = (_e20 | 8u);
    uint _e23 = bits;
    bits = (_e23 ^ 1u);
    uint _e25 = amount;
    uint _e26 = bits;
    local = _e25;
    uint _e28 = local;
    bits = (_e26 << _e28);
    uint _e31 = bits;
    bits = (_e31 >> 1u);
    uint _e34 = bits;
    bits = (_e34 - 97u * (_e34 / 97u));
    uint _e37 = bits;
    bits = (_e37 & 255u);
    uint _e42 = bits;
    _group_0_binding_0_cs[threadIdx.x] = ~(_e42);
}

