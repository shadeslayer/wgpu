#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 2, local_size_y = 1, local_size_z = 1) in;

struct float3_ {
    float x;
    float y;
    float z;
};
struct Sample_cuda_record_0_ {
    float3_ position;
    uint tag;
};
struct Packet_cuda_record_1_ {
    uint serial;
    vec2 uv;
    Sample_cuda_record_0_ samples[2];
    float weights[2][2];
    vec4 colour;
};
struct Packet_cuda_record_2_ {
    int w;
    int x;
};
layout(std430) readonly buffer type_8_block_0Compute { Packet_cuda_record_1_ _group_0_binding_0_cs[]; };

layout(std430) buffer type_8_block_1Compute { Packet_cuda_record_1_ _group_0_binding_1_cs[]; };


Packet_cuda_record_1_ adjust(Packet_cuda_record_1_ value, int bias) {
    Packet_cuda_record_1_ value_1 = Packet_cuda_record_1_(0u, vec2(0.0), Sample_cuda_record_0_[2](Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u), Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u)), float[2][2](float[2](0.0, 0.0), float[2](0.0, 0.0)), vec4(0.0));
    int bias_1 = 0;
    value_1 = value;
    bias_1 = bias;
    int _e5 = bias_1;
    uint _e7 = value_1.serial;
    value_1.serial = (_e7 + uint(_e5));
    float _e19 = value_1.weights[0][1];
    float _e20 = value_1.samples[1].position.x;
    value_1.samples[1].position.x = (_e20 + _e19);
    uint _e31 = value_1.samples[0].tag;
    value_1.weights[1][0] = float(_e31);
    Packet_cuda_record_1_ _e33 = value_1;
    return _e33;
}

Packet_cuda_record_1_ operator_u002b(Packet_cuda_record_1_ left, Packet_cuda_record_1_ right) {
    Packet_cuda_record_1_ left_1 = Packet_cuda_record_1_(0u, vec2(0.0), Sample_cuda_record_0_[2](Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u), Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u)), float[2][2](float[2](0.0, 0.0), float[2](0.0, 0.0)), vec4(0.0));
    Packet_cuda_record_1_ right_1 = Packet_cuda_record_1_(0u, vec2(0.0), Sample_cuda_record_0_[2](Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u), Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u)), float[2][2](float[2](0.0, 0.0), float[2](0.0, 0.0)), vec4(0.0));
    left_1 = left;
    right_1 = right;
    uint _e6 = right_1.serial;
    uint _e7 = left_1.serial;
    left_1.serial = (_e7 + _e6);
    float _e13 = right_1.uv.y;
    float _e14 = left_1.uv.x;
    left_1.uv.x = (_e14 + _e13);
    Packet_cuda_record_1_ _e16 = left_1;
    return _e16;
}

void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    uint lane = 0u;
    Packet_cuda_record_1_ original = Packet_cuda_record_1_(0u, vec2(0.0), Sample_cuda_record_0_[2](Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u), Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u)), float[2][2](float[2](0.0, 0.0), float[2](0.0, 0.0)), vec4(0.0));
    Packet_cuda_record_1_ changed = Packet_cuda_record_1_(0u, vec2(0.0), Sample_cuda_record_0_[2](Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u), Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u)), float[2][2](float[2](0.0, 0.0), float[2](0.0, 0.0)), vec4(0.0));
    Packet_cuda_record_2_ local = Packet_cuda_record_2_(0, 0);
    Packet_cuda_record_1_ combined = Packet_cuda_record_1_(0u, vec2(0.0), Sample_cuda_record_0_[2](Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u), Sample_cuda_record_0_(float3_(0.0, 0.0, 0.0), 0u)), float[2][2](float[2](0.0, 0.0), float[2](0.0, 0.0)), vec4(0.0));
    uint slot = 0u;
    int sample_ = 0;
    int row = 0;
    int column = 0;
    lane = threadIdx.x;
    uint _e6 = lane;
    if ((_e6 >= 2u)) {
        return;
    }
    uint _e9 = lane;
    Packet_cuda_record_1_ _e11 = _group_0_binding_0_cs[_e9];
    original = _e11;
    Packet_cuda_record_1_ _e13 = original;
    uint _e15 = lane;
    Packet_cuda_record_1_ _e18 = adjust(_e13, (3 + int(_e15)));
    changed = _e18;
    {
        local.w = 7;
        local.x = 11;
        int _e30 = local.w;
        int _e31 = local.x;
        uint _e34 = changed.samples[0].tag;
        changed.samples[0].tag = (_e34 + uint((_e30 + _e31)));
    }
    Packet_cuda_record_1_ _e37 = changed;
    Packet_cuda_record_1_ _e38 = original;
    Packet_cuda_record_1_ _e39 = operator_u002b(_e37, _e38);
    combined = _e39;
    uint _e46 = original.serial;
    float _e48 = combined.weights[0][0];
    combined.weights[0][0] = (_e48 + float(_e46));
    uint _e53 = combined.serial;
    combined.colour.w = float(_e53);
    uint _e57 = lane;
    slot = (_e57 + 1u);
    uint _e63 = combined.serial;
    uint _e59 = slot;
    _group_0_binding_1_cs[_e59].serial = _e63;
    vec2 _e68 = combined.uv;
    uint _e64 = slot;
    _group_0_binding_1_cs[_e64].uv = _e68;
    {
        sample_ = 0;
        bool loop_init = true;
        while(true) {
            if (!loop_init) {
                int _e74 = sample_;
                sample_ = (_e74 + 1);
            }
            loop_init = false;
            int _e72 = sample_;
            if ((_e72 < 2)) {
            } else {
                break;
            }
            int _e83 = sample_;
            Sample_cuda_record_0_ _e85 = combined.samples[_e83];
            uint _e77 = slot;
            int _e80 = sample_;
            _group_0_binding_1_cs[_e77].samples[_e80] = _e85;
        }
    }
    {
        row = 0;
        bool loop_init_1 = true;
        while(true) {
            if (!loop_init_1) {
                int _e91 = row;
                row = (_e91 + 1);
            }
            loop_init_1 = false;
            int _e89 = row;
            if ((_e89 < 2)) {
            } else {
                break;
            }
            {
                column = 0;
                bool loop_init_2 = true;
                while(true) {
                    if (!loop_init_2) {
                        int _e99 = column;
                        column = (_e99 + 1);
                    }
                    loop_init_2 = false;
                    int _e97 = column;
                    if ((_e97 < 2)) {
                    } else {
                        break;
                    }
                    int _e110 = row;
                    int _e112 = column;
                    float _e114 = combined.weights[_e110][_e112];
                    uint _e102 = slot;
                    int _e105 = row;
                    int _e107 = column;
                    _group_0_binding_1_cs[_e102].weights[_e105][_e107] = _e114;
                }
            }
        }
    }
    vec4 _e119 = combined.colour;
    uint _e115 = slot;
    _group_0_binding_1_cs[_e115].colour = _e119;
}

