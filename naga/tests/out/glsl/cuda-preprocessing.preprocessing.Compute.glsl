#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 2, local_size_y = 1, local_size_z = 1) in;

struct Packet_cuda_record_0_ {
    uint first;
    uint second;
};
layout(std430) buffer type_1_block_0Compute { uint _group_0_binding_0_cs[]; };


uint later(uint value) {
    uint value_1 = 0u;
    value_1 = value;
    uint _e3 = value_1;
    return (_e3 + 5u);
}

void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    uint lane = 0u;
    uint start = 0u;
    uint local = 0u;
    Packet_cuda_record_0_ original = Packet_cuda_record_0_(0u, 0u);
    Packet_cuda_record_0_ copied = Packet_cuda_record_0_(0u, 0u);
    uint SELF = 0u;
    uint LEFT = 0u;
    uint RIGHT = 0u;
    uint counter = 0u;
    Packet_cuda_record_0_ ordered = Packet_cuda_record_0_(0u, 0u);
    uint values[3] = uint[3](0u, 0u, 0u);
    uint index = 0u;
    lane = threadIdx.x;
    uint _e5 = lane;
    if ((_e5 >= 2u)) {
        return;
    }
    uint _e9 = lane;
    start = (_e9 * 24u);
    uint _e12 = start;
    _group_0_binding_0_cs[(_e12 + 0u)] = 7u;
    uint _e17 = start;
    _group_0_binding_0_cs[(_e17 + 1u)] = 11u;
    uint _e22 = start;
    _group_0_binding_0_cs[(_e22 + 2u)] = 22u;
    uint _e27 = start;
    _group_0_binding_0_cs[(_e27 + 3u)] = (2u + (3u * 4u));
    uint _e36 = start;
    _group_0_binding_0_cs[(_e36 + 4u)] = (2u + 3u);
    local = 2u;
    uint _e49 = local;
    uint _e43 = start;
    _group_0_binding_0_cs[(_e43 + 5u)] = (3u << _e49);
    uint _e55 = lane;
    uint _e56 = later(_e55);
    uint _e52 = start;
    _group_0_binding_0_cs[(_e52 + 6u)] = _e56;
    uint _e59 = lane;
    original.first = (_e59 + 1u);
    uint _e64 = lane;
    original.second = (_e64 + 2u);
    Packet_cuda_record_0_ _e67 = original;
    copied = _e67;
    uint _e73 = copied.first;
    uint _e69 = start;
    _group_0_binding_0_cs[(_e69 + 7u)] = _e73;
    uint _e79 = copied.second;
    uint _e75 = start;
    _group_0_binding_0_cs[(_e75 + 8u)] = _e79;
    SELF = 17u;
    LEFT = 19u;
    RIGHT = 23u;
    uint _e90 = SELF;
    uint _e87 = start;
    _group_0_binding_0_cs[(_e87 + 9u)] = _e90;
    uint _e95 = LEFT;
    uint _e92 = start;
    _group_0_binding_0_cs[(_e92 + 10u)] = _e95;
    uint _e100 = RIGHT;
    uint _e97 = start;
    _group_0_binding_0_cs[(_e97 + 11u)] = _e100;
    counter = 0u;
    uint _e104 = counter;
    counter = (_e104 + uint(1));
    uint _e108 = counter;
    ordered.first = _e108;
    uint _e111 = counter;
    counter = (_e111 + uint(1));
    uint _e115 = counter;
    ordered.second = _e115;
    uint _e121 = ordered.first;
    uint _e117 = start;
    _group_0_binding_0_cs[(_e117 + 12u)] = _e121;
    uint _e127 = ordered.second;
    uint _e123 = start;
    _group_0_binding_0_cs[(_e123 + 13u)] = _e127;
    uint _e132 = counter;
    uint _e129 = start;
    _group_0_binding_0_cs[(_e129 + 14u)] = _e132;
    uint _e134 = start;
    _group_0_binding_0_cs[(_e134 + 15u)] = 31u;
    values[0] = 2u;
    values[1] = 4u;
    values[2] = 6u;
    uint _e149 = lane;
    uint _e151 = values[_e149];
    uint _e146 = start;
    _group_0_binding_0_cs[(_e146 + 16u)] = _e151;
    index = 0u;
    uint _e158 = index;
    index = (_e158 + uint(1));
    uint _e163 = values[_e158];
    uint _e155 = start;
    _group_0_binding_0_cs[(_e155 + 17u)] = _e163;
    uint _e168 = index;
    uint _e165 = start;
    _group_0_binding_0_cs[(_e165 + 18u)] = _e168;
    uint _e170 = start;
    _group_0_binding_0_cs[(_e170 + 19u)] = uint(true);
    uint _e176 = start;
    _group_0_binding_0_cs[(_e176 + 20u)] = 61u;
    uint _e184 = lane;
    uint _e181 = start;
    _group_0_binding_0_cs[(_e181 + 21u)] = _e184;
    uint _e186 = start;
    _group_0_binding_0_cs[(_e186 + 22u)] = floatBitsToUint(-(0.0));
    uint _e193 = start;
    _group_0_binding_0_cs[(_e193 + 23u)] = 4294967295u;
}

