#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct type_5 {
    uint value;
};
layout(std430) buffer type_3_block_0Compute { uint _group_0_binding_0_cs[]; };

layout(std430) readonly buffer type_4_block_1Compute { int _group_0_binding_1_cs[]; };

layout(std140) uniform type_5_block_2Compute { type_5 _group_0_binding_2_cs; };

shared uint shared_values[64];


int transform(float value, float multiplier, int offset) {
    float value_1 = 0.0;
    float multiplier_1 = 0.0;
    int offset_1 = 0;
    value_1 = value;
    multiplier_1 = multiplier;
    offset_1 = offset;
    float _e6 = value_1;
    float _e7 = multiplier_1;
    int _e9 = offset_1;
    return int(((_e6 * _e7) + float(_e9)));
}

void main() {
    if (gl_LocalInvocationID == uvec3(0u)) {
        shared_values = uint[64](0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    }
    memoryBarrierShared();
    barrier();
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint base = 0u;
    uint lane = 0u;
    int signed_count = 0;
    float fraction = 0.0;
    int after_block = 0;
    float coefficient = 0.0;
    bool selected = false;
    int total = 0;
    int current = 0;
    float after_loop = 0.0;
    uint _e4 = _group_0_binding_2_cs.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e15 = i;
    uint _e16 = n;
    if ((_e15 < _e16)) {
        uint _e20 = i;
        base = (_e20 * 10u);
        lane = threadIdx.x;
        uint _e27 = i;
        uint _e24 = lane;
        shared_values[_e24] = (_e27 + 10u);
        uint _e31 = lane;
        uint _e33 = shared_values[_e31];
        uint _e29 = base;
        _group_0_binding_0_cs[_e29] = _e33;
        uint _e38 = i;
        int _e40 = _group_0_binding_1_cs[_e38];
        int _e44 = transform(float(_e40), 2.0, 1);
        uint _e35 = base;
        _group_0_binding_0_cs[(_e35 + 1u)] = uint(_e44);
        signed_count = -(3);
        int _e53 = signed_count;
        uint _e50 = base;
        _group_0_binding_0_cs[(_e50 + 2u)] = uint(_e53);
        {
            fraction = 3.75;
            float _e61 = fraction;
            uint _e58 = base;
            _group_0_binding_0_cs[(_e58 + 3u)] = uint(_e61);
        }
        after_block = -(5);
        int _e70 = after_block;
        uint _e67 = base;
        _group_0_binding_0_cs[(_e67 + 4u)] = uint(_e70);
        coefficient = 1.25;
        float _e79 = coefficient;
        uint _e75 = base;
        _group_0_binding_0_cs[(_e75 + 5u)] = uint((_e79 * 4.0));
        uint _e84 = i;
        selected = ((_e84 - 2u * (_e84 / 2u)) == 0u);
        bool _e92 = selected;
        uint _e89 = base;
        _group_0_binding_0_cs[(_e89 + 6u)] = uint(_e92);
        total = 0;
        {
            while(true) {
                int _e97 = total;
                if ((_e97 < 3)) {
                } else {
                    break;
                }
                int _e100 = total;
                current = _e100;
                int _e102 = current;
                total = (_e102 + 1);
            }
        }
        int _e108 = total;
        uint _e105 = base;
        _group_0_binding_0_cs[(_e105 + 7u)] = uint(_e108);
        after_loop = 1.5;
        float _e117 = after_loop;
        uint _e113 = base;
        _group_0_binding_0_cs[(_e113 + 8u)] = uint((_e117 * 2.0));
        uint _e124 = n;
        uint _e121 = base;
        _group_0_binding_0_cs[(_e121 + 9u)] = uint(float(_e124));
    }
}

