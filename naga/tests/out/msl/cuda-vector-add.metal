// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
    uint size2;
};

typedef float type_1[1];

struct addInput {
};
kernel void add(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1 const& a [[user(fake0)]]
, device type_1 const& b [[user(fake0)]]
, device type_1& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint i = {};
    i = (blockIdx.x * 64u) + threadIdx.x;
    uint _e13 = i;
    uint _e15 = i;
    float _e18 = b[_e15];
    float _e20 = a[_e13];
    uint _e11 = i;
    out[_e11] = _e20 + (_e18 * 2.0);
}
