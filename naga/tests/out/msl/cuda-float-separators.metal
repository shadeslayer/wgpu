// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
};

typedef float type_1[1];
struct type_3 {
    uint value;
    char _pad1[12];
};

struct float_separatorsInput {
};
kernel void float_separators(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, device type_1 const& input [[user(fake0)]]
, constant type_3& n_1 [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint n = {};
    uint i = {};
    uint base = {};
    uint _e4 = n_1.value;
    n = _e4;
    i = (blockIdx.x * 64u) + threadIdx.x;
    uint _e14 = i;
    uint _e15 = n;
    if (_e14 < _e15) {
        uint _e19 = i;
        base = _e19 * 12u;
        uint _e21 = base;
        out[_e21] = 1024.5;
        uint _e25 = base;
        out[_e25 + 1u] = 0.125;
        uint _e30 = base;
        out[_e30 + 2u] = 12.0;
        uint _e35 = base;
        out[_e35 + 3u] = 25.0;
        uint _e40 = base;
        out[_e40 + 4u] = 0.5;
        uint _e45 = base;
        out[_e45 + 5u] = 6.25;
        uint _e50 = base;
        out[_e50 + 6u] = -(1024.5);
        uint _e56 = base;
        out[_e56 + 7u] = 0.125;
        uint _e61 = base;
        out[_e61 + 8u] = 3.1415927;
        uint _e66 = base;
        out[_e66 + 9u] = 1.0;
        uint _e71 = base;
        out[_e71 + 10u] = 1.0000002;
        uint _e79 = i;
        float _e82 = input[_e79];
        uint _e76 = base;
        out[_e76 + 11u] = (_e82 * 0.25) + 10.0;
    }
}
