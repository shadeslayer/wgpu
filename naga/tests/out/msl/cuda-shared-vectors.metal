// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

struct float3_ {
    float x;
    float y;
    float z;
};
typedef float type_1[1];
struct type_2 {
    float3_ inner[8];
};

float3_ shift(
    float3_ value
) {
    float3_ value_1 = {};
    value_1 = value;
    float _e4 = value_1.x;
    value_1.x = _e4 + 1.0;
    float _e9 = value_1.z;
    float _e10 = value_1.y;
    value_1.y = _e9 - _e10;
    float3_ _e12 = value_1;
    return _e12;
}
uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}


struct shared_vectorsInput {
};
kernel void shared_vectors(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, uint __local_invocation_index [[thread_index_in_threadgroup]]
, device type_1& output [[user(fake0)]]
, threadgroup type_2& values
, threadgroup float3_& broadcast
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    if (__local_invocation_index == 0u) {
        values = {};
        broadcast = {};
    }
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    uint lane = {};
    uint group = {};
    float base = {};
    uint next = {};
    float3_ copied = {};
    int offset = {};
    bool choose_shared = {};
    uint row = {};
    lane = threadIdx.x;
    group = blockIdx.x;
    uint _e11 = group;
    uint _e13 = lane;
    base = static_cast<float>((_e11 * 16u) + _e13);
    float _e19 = base;
    float _e21 = base;
    float _e26 = base;
    uint _e16 = lane;
    values.inner[_e16] = float3_ {_e19 + 0.5, -(_e21) - 1.25, _e26 + 2.5};
    uint _e30 = lane;
    if (_e30 == 0u) {
        uint _e32 = group;
        uint _e36 = group;
        uint _e40 = group;
        broadcast = float3_ {static_cast<float>(_e32) + 0.25, static_cast<float>(_e36) + 0.5, static_cast<float>(_e40) + 0.75};
    }
    metal::threadgroup_barrier(metal::mem_flags::mem_device);
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    uint _e47 = lane;
    next = naga_mod(_e47 + 1u, 8u);
    uint _e52 = next;
    float3_ _e54 = values.inner[_e52];
    float3_ _e55 = shift(_e54);
    copied = _e55;
    {
        offset = 1;
        int _e59 = offset;
        float _e60 = copied.x;
        copied.x = _e60 + static_cast<float>(_e59);
    }
    metal::threadgroup_barrier(metal::mem_flags::mem_device);
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    uint _e65 = lane;
    choose_shared = (_e65 & 1u) != 0u;
    float _e75 = broadcast.x;
    bool _e69 = choose_shared;
    if (_e69) {
        uint _e70 = lane;
        float _e76 = values.inner[_e70].y;
        values.inner[_e70].y = _e76 + _e75;
    } else {
        float _e78 = copied.y;
        copied.y = _e78 + _e75;
    }
    float _e84 = copied.x;
    uint _e80 = lane;
    values.inner[_e80].z = _e84;
    uint _e87 = group;
    uint _e89 = lane;
    row = ((_e87 * 8u) + _e89) * 9u;
    float _e96 = copied.x;
    uint _e93 = row;
    output[_e93] = _e96;
    float _e102 = copied.y;
    uint _e98 = row;
    output[_e98 + 1u] = _e102;
    float _e108 = copied.z;
    uint _e104 = row;
    output[_e104 + 2u] = _e108;
    metal::threadgroup_barrier(metal::mem_flags::mem_device);
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    uint _e113 = next;
    float _e116 = values.inner[_e113].x;
    uint _e110 = row;
    output[_e110 + 3u] = _e116;
    uint _e121 = next;
    float _e124 = values.inner[_e121].y;
    uint _e118 = row;
    output[_e118 + 4u] = _e124;
    uint _e129 = next;
    float _e132 = values.inner[_e129].z;
    uint _e126 = row;
    output[_e126 + 5u] = _e132;
    float _e138 = broadcast.x;
    uint _e134 = row;
    output[_e134 + 6u] = _e138;
    float _e144 = broadcast.y;
    uint _e140 = row;
    output[_e140 + 7u] = _e144;
    float _e150 = broadcast.z;
    uint _e146 = row;
    output[_e146 + 8u] = _e150;
}
