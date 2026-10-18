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
struct type_3 {
    uint value;
    char _pad1[12];
};

struct addInput {
};
kernel void add(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1 const& a [[user(fake0)]]
, device type_1 const& b [[user(fake0)]]
, device type_1& out [[user(fake0)]]
, constant type_3& n_1 [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint n = {};
    uint i = {};
    uint _e5 = n_1.value;
    n = _e5;
    i = (blockIdx.x * 1u) + threadIdx.x;
    uint _e15 = i;
    uint _e16 = n;
    if (_e15 < _e16) {
        uint _e20 = i;
        uint _e22 = i;
        float _e24 = a[_e20];
        float _e25 = b[_e22];
        uint _e18 = i;
        out[_e18] = _e24 + _e25;
    }
}
