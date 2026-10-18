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

struct convertInput {
};
kernel void convert(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1 const& input [[user(fake0)]]
, device type_1& output [[user(fake0)]]
, constant type_3& n_1 [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint n = {};
    int i = {};
    float offset = {};
    uint _e4 = n_1.value;
    n = _e4;
    i = static_cast<int>((blockIdx.x * 1u) + threadIdx.x);
    int _e15 = i;
    uint _e17 = n;
    if (static_cast<uint>(_e15) < _e17) {
        int _e20 = i;
        offset = static_cast<float>(_e20);
        int _e24 = i;
        float _e26 = input[_e24];
        float _e27 = offset;
        int _e22 = i;
        output[_e22] = _e26 + _e27;
    }
}
