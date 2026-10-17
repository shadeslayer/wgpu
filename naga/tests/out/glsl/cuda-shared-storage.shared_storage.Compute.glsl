#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_1_block_0Compute { uint _group_0_binding_0_cs[]; };

shared uint values[64];

shared int signed_value;

shared float float_value;


void main() {
    if (gl_LocalInvocationID == uvec3(0u)) {
        values = uint[64](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
        signed_value = 0;
        float_value = 0.0;
    }
    memoryBarrierShared();
    barrier();
    uvec3 threadIdx = gl_LocalInvocationID;
    uvec3 blockIdx = gl_WorkGroupID;
    uint lane = 0u;
    uint base = 0u;
    lane = threadIdx.x;
    base = (blockIdx.x * 128u);
    uint _e17 = lane;
    uint _e12 = lane;
    values[_e12] = (((blockIdx.x * 1000u) + _e17) + 10u);
    uint _e25 = lane;
    uint _e27 = values[_e25];
    uint _e21 = base;
    uint _e22 = lane;
    _group_0_binding_0_cs[(_e21 + _e22)] = _e27;
    uint _e29 = lane;
    if ((_e29 == 0u)) {
        signed_value = -(7);
        float_value = 3.5;
        int _e39 = signed_value;
        uint _e35 = base;
        _group_0_binding_0_cs[(_e35 + 64u)] = uint((_e39 + 10));
        float _e47 = float_value;
        uint _e43 = base;
        _group_0_binding_0_cs[(_e43 + 65u)] = uint((_e47 * 2.0));
    }
}

