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
uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}


struct barriersInput {
};
kernel void barriers(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, uint __local_invocation_index [[thread_index_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, threadgroup type_2& values
, threadgroup uint& broadcast
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    if (__local_invocation_index == 0u) {
        values = {};
        broadcast = {};
    }
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    uint lane = {};
    uint base = {};
    lane = threadIdx.x;
    base = blockIdx.x * 256u;
    uint _e16 = lane;
    uint _e11 = lane;
    values.inner[_e11] = ((blockIdx.x * 2000u) + _e16) + 7u;
    uint _e27 = lane;
    uint _e20 = base;
    uint _e21 = lane;
    out[_e20 + _e21] = (blockIdx.x * 1000u) + _e27;
    uint _e30 = lane;
    if (_e30 == 0u) {
        broadcast = (blockIdx.x * 100u) + 11u;
    }
    metal::threadgroup_barrier(metal::mem_flags::mem_device);
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    uint _e44 = lane;
    uint _e47 = values.inner[63u - _e44];
    uint _e38 = base;
    uint _e40 = lane;
    out[(_e38 + 64u) + _e40] = _e47;
    uint _e55 = lane;
    uint _e59 = base;
    uint _e62 = out[_e59 + naga_mod(_e55 + 1u, 64u)];
    uint _e49 = base;
    uint _e51 = lane;
    out[(_e49 + 128u) + _e51] = _e62;
    uint _e69 = broadcast;
    uint _e64 = base;
    uint _e66 = lane;
    out[(_e64 + 192u) + _e66] = _e69;
}
