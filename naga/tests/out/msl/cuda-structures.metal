// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
};

struct float3_ {
    float x;
    float y;
    float z;
};
struct Sample_cuda_record_0_ {
    float3_ position;
    uint tag;
};
struct type_3 {
    Sample_cuda_record_0_ inner[2];
};
struct type_4 {
    float inner[2];
};
struct type_5 {
    type_4 inner[2];
};
struct Packet_cuda_record_1_ {
    uint serial;
    char _pad1[4];
    metal::float2 uv;
    type_3 samples;
    type_5 weights;
    metal::float4 colour;
};
typedef Packet_cuda_record_1_ type_8[1];
struct Packet_cuda_record_2_ {
    int w;
    int x;
};

Packet_cuda_record_1_ adjust(
    Packet_cuda_record_1_ value,
    int bias
) {
    Packet_cuda_record_1_ value_1 = {};
    int bias_1 = {};
    value_1 = value;
    bias_1 = bias;
    int _e5 = bias_1;
    uint _e7 = value_1.serial;
    value_1.serial = _e7 + static_cast<uint>(_e5);
    float _e19 = value_1.weights.inner[0].inner[1];
    float _e20 = value_1.samples.inner[1].position.x;
    value_1.samples.inner[1].position.x = _e20 + _e19;
    uint _e31 = value_1.samples.inner[0].tag;
    value_1.weights.inner[1].inner[0] = static_cast<float>(_e31);
    Packet_cuda_record_1_ _e33 = value_1;
    return _e33;
}

Packet_cuda_record_1_ operator_u002b(
    Packet_cuda_record_1_ left,
    Packet_cuda_record_1_ right
) {
    Packet_cuda_record_1_ left_1 = {};
    Packet_cuda_record_1_ right_1 = {};
    left_1 = left;
    right_1 = right;
    uint _e6 = right_1.serial;
    uint _e7 = left_1.serial;
    left_1.serial = _e7 + _e6;
    float _e13 = right_1.uv.y;
    float _e14 = left_1.uv.x;
    left_1.uv.x = _e14 + _e13;
    Packet_cuda_record_1_ _e16 = left_1;
    return _e16;
}

struct structuresInput {
};
kernel void structures(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_8 const& input [[user(fake0)]]
, device type_8& output [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint lane = {};
    Packet_cuda_record_1_ original = {};
    Packet_cuda_record_1_ changed = {};
    Packet_cuda_record_2_ local = {};
    Packet_cuda_record_1_ combined = {};
    uint slot = {};
    int sample = {};
    int row = {};
    int column = {};
    lane = threadIdx.x;
    uint _e6 = lane;
    if (_e6 >= 2u) {
        return;
    }
    uint _e9 = lane;
    Packet_cuda_record_1_ _e11 = input[_e9];
    original = _e11;
    Packet_cuda_record_1_ _e13 = original;
    uint _e15 = lane;
    Packet_cuda_record_1_ _e18 = adjust(_e13, as_type<int>(as_type<uint>(3) + as_type<uint>(static_cast<int>(_e15))));
    changed = _e18;
    {
        local.w = 7;
        local.x = 11;
        int _e30 = local.w;
        int _e31 = local.x;
        uint _e34 = changed.samples.inner[0].tag;
        changed.samples.inner[0].tag = _e34 + static_cast<uint>(as_type<int>(as_type<uint>(_e30) + as_type<uint>(_e31)));
    }
    Packet_cuda_record_1_ _e37 = changed;
    Packet_cuda_record_1_ _e38 = original;
    Packet_cuda_record_1_ _e39 = operator_u002b(_e37, _e38);
    combined = _e39;
    uint _e46 = original.serial;
    float _e48 = combined.weights.inner[0].inner[0];
    combined.weights.inner[0].inner[0] = _e48 + static_cast<float>(_e46);
    uint _e53 = combined.serial;
    combined.colour.w = static_cast<float>(_e53);
    uint _e57 = lane;
    slot = _e57 + 1u;
    uint _e63 = combined.serial;
    uint _e59 = slot;
    output[_e59].serial = _e63;
    metal::float2 _e68 = combined.uv;
    uint _e64 = slot;
    output[_e64].uv = _e68;
    {
        sample = 0;
        uint2 loop_bound = uint2(4294967295u);
        bool loop_init = true;
        while(true) {
            if (metal::all(loop_bound == uint2(0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e74 = sample;
                sample = as_type<int>(as_type<uint>(_e74) + as_type<uint>(1));
            }
            loop_init = false;
            int _e72 = sample;
            if (_e72 < 2) {
            } else {
                break;
            }
            int _e83 = sample;
            Sample_cuda_record_0_ _e85 = combined.samples.inner[_e83];
            uint _e77 = slot;
            int _e80 = sample;
            output[_e77].samples.inner[_e80] = _e85;
        }
    }
    {
        row = 0;
        uint2 loop_bound_1 = uint2(4294967295u);
        bool loop_init_1 = true;
        while(true) {
            if (metal::all(loop_bound_1 == uint2(0u))) { break; }
            loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
            if (!loop_init_1) {
                int _e91 = row;
                row = as_type<int>(as_type<uint>(_e91) + as_type<uint>(1));
            }
            loop_init_1 = false;
            int _e89 = row;
            if (_e89 < 2) {
            } else {
                break;
            }
            {
                column = 0;
                uint2 loop_bound_2 = uint2(4294967295u);
                bool loop_init_2 = true;
                while(true) {
                    if (metal::all(loop_bound_2 == uint2(0u))) { break; }
                    loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
                    if (!loop_init_2) {
                        int _e99 = column;
                        column = as_type<int>(as_type<uint>(_e99) + as_type<uint>(1));
                    }
                    loop_init_2 = false;
                    int _e97 = column;
                    if (_e97 < 2) {
                    } else {
                        break;
                    }
                    int _e110 = row;
                    int _e112 = column;
                    float _e114 = combined.weights.inner[_e110].inner[_e112];
                    uint _e102 = slot;
                    int _e105 = row;
                    int _e107 = column;
                    output[_e102].weights.inner[_e105].inner[_e107] = _e114;
                }
            }
        }
    }
    metal::float4 _e119 = combined.colour;
    uint _e115 = slot;
    output[_e115].colour = _e119;
}
