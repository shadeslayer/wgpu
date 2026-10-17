// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef uint type_1[1];
struct type_2 {
    uint inner[64];
};
int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}

uint naga_f2u32(float value) {
    return static_cast<uint>(metal::clamp(value, 0.0, 4294967000.0));
}


struct shared_storageInput {
};
kernel void shared_storage(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, uint __local_invocation_index [[thread_index_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, threadgroup type_2& values
, threadgroup int& signed_value
, threadgroup float& float_value
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    if (__local_invocation_index == 0u) {
        values = {};
        signed_value = {};
        float_value = {};
    }
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    uint lane = {};
    uint base = {};
    lane = threadIdx.x;
    base = blockIdx.x * 128u;
    uint _e17 = lane;
    uint _e12 = lane;
    values.inner[_e12] = ((blockIdx.x * 1000u) + _e17) + 10u;
    uint _e25 = lane;
    uint _e27 = values.inner[_e25];
    uint _e21 = base;
    uint _e22 = lane;
    out[_e21 + _e22] = _e27;
    uint _e29 = lane;
    if (_e29 == 0u) {
        signed_value = naga_neg(7);
        float_value = 3.5;
        int _e39 = signed_value;
        uint _e35 = base;
        out[_e35 + 64u] = static_cast<uint>(as_type<int>(as_type<uint>(_e39) + as_type<uint>(10)));
        float _e47 = float_value;
        uint _e43 = base;
        out[_e43 + 65u] = naga_f2u32(_e47 * 2.0);
    }
}
