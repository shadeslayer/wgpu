#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 8, local_size_y = 1, local_size_z = 1) in;

struct float3_ {
    float x;
    float y;
    float z;
};
layout(std430) buffer type_1_block_0Compute { float _group_0_binding_0_cs[]; };

shared float3_ values[8];

shared float3_ broadcast;


float3_ shift(float3_ value) {
    float3_ value_1 = float3_(0.0, 0.0, 0.0);
    value_1 = value;
    float _e4 = value_1.x;
    value_1.x = (_e4 + 1.0);
    float _e9 = value_1.z;
    float _e10 = value_1.y;
    value_1.y = (_e9 - _e10);
    float3_ _e12 = value_1;
    return _e12;
}

void main() {
    if (gl_LocalInvocationID == uvec3(0u)) {
        values = float3_[8](float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0));
        broadcast = float3_(0.0, 0.0, 0.0);
    }
    memoryBarrierShared();
    barrier();
    uvec3 threadIdx = gl_LocalInvocationID;
    uvec3 blockIdx = gl_WorkGroupID;
    uint lane = 0u;
    uint group = 0u;
    float base = 0.0;
    uint next = 0u;
    float3_ copied = float3_(0.0, 0.0, 0.0);
    int offset = 0;
    bool choose_shared = false;
    uint row = 0u;
    lane = threadIdx.x;
    group = blockIdx.x;
    uint _e11 = group;
    uint _e13 = lane;
    base = float(((_e11 * 16u) + _e13));
    float _e19 = base;
    float _e21 = base;
    float _e26 = base;
    uint _e16 = lane;
    values[_e16] = float3_((_e19 + 0.5), (-(_e21) - 1.25), (_e26 + 2.5));
    uint _e30 = lane;
    if ((_e30 == 0u)) {
        uint _e32 = group;
        uint _e36 = group;
        uint _e40 = group;
        broadcast = float3_((float(_e32) + 0.25), (float(_e36) + 0.5), (float(_e40) + 0.75));
    }
    memoryBarrierBuffer();
    memoryBarrierShared();
    barrier();
    uint _e47 = lane;
    uint _e48 = (_e47 + 1u);
    next = (_e48 - 8u * (_e48 / 8u));
    uint _e52 = next;
    float3_ _e54 = values[_e52];
    float3_ _e55 = shift(_e54);
    copied = _e55;
    {
        offset = 1;
        int _e59 = offset;
        float _e60 = copied.x;
        copied.x = (_e60 + float(_e59));
    }
    memoryBarrierBuffer();
    memoryBarrierShared();
    barrier();
    uint _e65 = lane;
    choose_shared = ((_e65 & 1u) != 0u);
    float _e75 = broadcast.x;
    bool _e69 = choose_shared;
    if (_e69) {
        uint _e70 = lane;
        float _e76 = values[_e70].y;
        values[_e70].y = (_e76 + _e75);
    } else {
        float _e78 = copied.y;
        copied.y = (_e78 + _e75);
    }
    float _e84 = copied.x;
    uint _e80 = lane;
    values[_e80].z = _e84;
    uint _e87 = group;
    uint _e89 = lane;
    row = (((_e87 * 8u) + _e89) * 9u);
    float _e96 = copied.x;
    uint _e93 = row;
    _group_0_binding_0_cs[_e93] = _e96;
    float _e102 = copied.y;
    uint _e98 = row;
    _group_0_binding_0_cs[(_e98 + 1u)] = _e102;
    float _e108 = copied.z;
    uint _e104 = row;
    _group_0_binding_0_cs[(_e104 + 2u)] = _e108;
    memoryBarrierBuffer();
    memoryBarrierShared();
    barrier();
    uint _e113 = next;
    float _e116 = values[_e113].x;
    uint _e110 = row;
    _group_0_binding_0_cs[(_e110 + 3u)] = _e116;
    uint _e121 = next;
    float _e124 = values[_e121].y;
    uint _e118 = row;
    _group_0_binding_0_cs[(_e118 + 4u)] = _e124;
    uint _e129 = next;
    float _e132 = values[_e129].z;
    uint _e126 = row;
    _group_0_binding_0_cs[(_e126 + 5u)] = _e132;
    float _e138 = broadcast.x;
    uint _e134 = row;
    _group_0_binding_0_cs[(_e134 + 6u)] = _e138;
    float _e144 = broadcast.y;
    uint _e140 = row;
    _group_0_binding_0_cs[(_e140 + 7u)] = _e144;
    float _e150 = broadcast.z;
    uint _e146 = row;
    _group_0_binding_0_cs[(_e146 + 8u)] = _e150;
}

