#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 2, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_1_block_0Compute { uint _group_0_binding_0_cs[]; };


uint later(uint value) {
    uint value_1 = 0u;
    value_1 = value;
    uint _e3 = value_1;
    return (_e3 + 73u);
}

void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    uint lane = 0u;
    uint start = 0u;
    uint counter = 0u;
    lane = threadIdx.x;
    uint _e5 = lane;
    if ((_e5 >= 2u)) {
        return;
    }
    uint _e9 = lane;
    start = (_e9 * 24u);
    uint _e11 = start;
    _group_0_binding_0_cs[_e11] = 7u;
    uint _e15 = start;
    _group_0_binding_0_cs[(_e15 + 1u)] = 11u;
    uint _e20 = start;
    _group_0_binding_0_cs[(_e20 + 2u)] = 13u;
    uint _e25 = start;
    _group_0_binding_0_cs[(_e25 + 3u)] = 17u;
    uint _e30 = start;
    _group_0_binding_0_cs[(_e30 + 4u)] = 19u;
    uint _e35 = start;
    _group_0_binding_0_cs[(_e35 + 5u)] = 23u;
    uint _e40 = start;
    _group_0_binding_0_cs[(_e40 + 6u)] = 29u;
    uint _e45 = start;
    _group_0_binding_0_cs[(_e45 + 7u)] = 31u;
    uint _e50 = start;
    _group_0_binding_0_cs[(_e50 + 8u)] = 37u;
    uint _e55 = start;
    _group_0_binding_0_cs[(_e55 + 9u)] = 41u;
    uint _e60 = start;
    _group_0_binding_0_cs[(_e60 + 10u)] = 43u;
    uint _e65 = start;
    _group_0_binding_0_cs[(_e65 + 11u)] = 47u;
    uint _e70 = start;
    _group_0_binding_0_cs[(_e70 + 12u)] = 53u;
    uint _e75 = start;
    _group_0_binding_0_cs[(_e75 + 13u)] = 59u;
    uint _e80 = start;
    _group_0_binding_0_cs[(_e80 + 14u)] = 61u;
    uint _e85 = start;
    _group_0_binding_0_cs[(_e85 + 15u)] = 67u;
    uint _e90 = start;
    _group_0_binding_0_cs[(_e90 + 16u)] = 71u;
    uint _e98 = lane;
    uint _e99 = later(_e98);
    uint _e95 = start;
    _group_0_binding_0_cs[(_e95 + 17u)] = _e99;
    counter = 0u;
    uint _e106 = counter;
    counter = (_e106 + uint(1));
    uint _e103 = start;
    _group_0_binding_0_cs[(_e103 + 18u)] = _e106;
    uint _e114 = counter;
    uint _e111 = start;
    _group_0_binding_0_cs[(_e111 + 19u)] = _e114;
    uint _e116 = start;
    _group_0_binding_0_cs[(_e116 + 20u)] = uint(true);
    uint _e125 = lane;
    uint _e122 = start;
    _group_0_binding_0_cs[(_e122 + 21u)] = _e125;
    uint _e127 = start;
    _group_0_binding_0_cs[(_e127 + 22u)] = floatBitsToUint(-(0.0));
    uint _e134 = start;
    _group_0_binding_0_cs[(_e134 + 23u)] = 4294967295u;
}

