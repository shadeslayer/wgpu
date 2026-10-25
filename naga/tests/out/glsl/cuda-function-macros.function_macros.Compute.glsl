#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 2, local_size_y = 1, local_size_z = 1) in;

struct Pair_cuda_record_0_ {
    uint first;
    uint second;
};
layout(std430) buffer type_1_block_0Compute { uint _group_0_binding_0_cs[]; };


uint advance(uint value) {
    uint value_1 = 0u;
    value_1 = value;
    uint _e3 = value_1;
    return (_e3 + 3u);
}

void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    uint lane = 0u;
    uint start = 0u;
    Pair_cuda_record_0_ pair = Pair_cuda_record_0_(0u, 0u);
    Pair_cuda_record_0_ local = Pair_cuda_record_0_(0u, 0u);
    uint values[3] = uint[3](0u, 0u, 0u);
    uvec2 vector = uvec2(0u);
    uint counter = 0u;
    uint local_1 = 0u;
    uint declared = 0u;
    lane = threadIdx.x;
    uint _e5 = lane;
    if ((_e5 >= 2u)) {
        return;
    }
    uint _e9 = lane;
    start = (_e9 * 24u);
    local.first = 11u;
    local.second = 13u;
    Pair_cuda_record_0_ _e17 = local;
    pair = _e17;
    values[0] = 2u;
    values[1] = 3u;
    values[2] = 5u;
    vector = uvec2(17u, 19u);
    counter = 0u;
    uint _e31 = start;
    _group_0_binding_0_cs[_e31] = 7u;
    uint _e35 = start;
    _group_0_binding_0_cs[(_e35 + 1u)] = 11u;
    uint _e40 = start;
    _group_0_binding_0_cs[(_e40 + 2u)] = (2u + 3u);
    uint _e47 = start;
    _group_0_binding_0_cs[(_e47 + 3u)] = ((1u + 2u) * 3u);
    uint _e56 = start;
    _group_0_binding_0_cs[(_e56 + 4u)] = (5u + 6u);
    uint _e63 = start;
    _group_0_binding_0_cs[(_e63 + 5u)] = (7u + 8u);
    uint _e74 = pair.first;
    uint _e70 = start;
    _group_0_binding_0_cs[(_e70 + 6u)] = _e74;
    uint _e80 = pair.second;
    uint _e76 = start;
    _group_0_binding_0_cs[(_e76 + 7u)] = _e80;
    uint _e82 = start;
    _group_0_binding_0_cs[(_e82 + 8u)] = uint(3);
    uint _e91 = lane;
    uint _e93 = values[_e91];
    uint _e88 = start;
    _group_0_binding_0_cs[(_e88 + 9u)] = _e93;
    uint _e99 = vector.x;
    uint _e95 = start;
    _group_0_binding_0_cs[(_e95 + 10u)] = _e99;
    uint _e105 = vector.y;
    uint _e101 = start;
    _group_0_binding_0_cs[(_e101 + 11u)] = _e105;
    uint _e110 = counter;
    counter = (_e110 + uint(1));
    uint _e114 = counter;
    counter = (_e114 + uint(1));
    uint _e107 = start;
    _group_0_binding_0_cs[(_e107 + 12u)] = _e114;
    uint _e122 = counter;
    uint _e119 = start;
    _group_0_binding_0_cs[(_e119 + 13u)] = _e122;
    uint _e124 = start;
    _group_0_binding_0_cs[(_e124 + 14u)] = 19u;
    uint _e132 = counter;
    uint _e129 = start;
    _group_0_binding_0_cs[(_e129 + 15u)] = _e132;
    values[0] = 23u;
    uint _e140 = values[0];
    uint _e134 = start;
    _group_0_binding_0_cs[(_e134 + 16u)] = _e140;
    uint _e147 = values[0];
    uint _e142 = start;
    _group_0_binding_0_cs[(_e142 + 17u)] = _e147;
    uint _e153 = lane;
    if ((_e153 == 0u)) {
        local_1 = 29u;
    } else {
        local_1 = 31u;
    }
    uint _e158 = local_1;
    uint _e149 = start;
    _group_0_binding_0_cs[(_e149 + 18u)] = _e158;
    uint _e163 = lane;
    uint _e164 = advance(_e163);
    uint _e160 = start;
    _group_0_binding_0_cs[(_e160 + 19u)] = _e164;
    declared = 37u;
    uint _e171 = declared;
    uint _e168 = start;
    _group_0_binding_0_cs[(_e168 + 20u)] = _e171;
    uint _e173 = start;
    _group_0_binding_0_cs[(_e173 + 21u)] = floatBitsToUint(-(0.0));
    uint _e180 = start;
    _group_0_binding_0_cs[(_e180 + 22u)] = 4294967295u;
    uint _e189 = lane;
    uint _e185 = start;
    _group_0_binding_0_cs[(_e185 + 23u)] = (41u + _e189);
}

