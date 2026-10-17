#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_1_block_0Compute { uint _group_0_binding_0_cs[]; };

shared uint values[64];

shared uint broadcast;


void main() {
    if (gl_LocalInvocationID == uvec3(0u)) {
        values = uint[64](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
        broadcast = 0u;
    }
    memoryBarrierShared();
    barrier();
    uvec3 threadIdx = gl_LocalInvocationID;
    uvec3 blockIdx = gl_WorkGroupID;
    uint lane = 0u;
    uint base = 0u;
    lane = threadIdx.x;
    base = (blockIdx.x * 256u);
    uint _e16 = lane;
    uint _e11 = lane;
    values[_e11] = (((blockIdx.x * 2000u) + _e16) + 7u);
    uint _e27 = lane;
    uint _e20 = base;
    uint _e21 = lane;
    _group_0_binding_0_cs[(_e20 + _e21)] = ((blockIdx.x * 1000u) + _e27);
    uint _e30 = lane;
    if ((_e30 == 0u)) {
        broadcast = ((blockIdx.x * 100u) + 11u);
    }
    memoryBarrierBuffer();
    memoryBarrierShared();
    barrier();
    uint _e44 = lane;
    uint _e47 = values[(63u - _e44)];
    uint _e38 = base;
    uint _e40 = lane;
    _group_0_binding_0_cs[((_e38 + 64u) + _e40)] = _e47;
    uint _e55 = lane;
    uint _e56 = (_e55 + 1u);
    uint _e59 = base;
    uint _e62 = _group_0_binding_0_cs[(_e59 + (_e56 - 64u * (_e56 / 64u)))];
    uint _e49 = base;
    uint _e51 = lane;
    _group_0_binding_0_cs[((_e49 + 128u) + _e51)] = _e62;
    uint _e69 = broadcast;
    uint _e64 = base;
    uint _e66 = lane;
    _group_0_binding_0_cs[((_e64 + 192u) + _e66)] = _e69;
}

