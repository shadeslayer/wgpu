// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size1;
    uint size2;
};

struct type_1 {
    float value;
    char _pad1[12];
};
typedef float type_2[1];

struct scaleInput {
};
kernel void scale(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, constant type_1& factor_1 [[user(fake0)]]
, device type_2 const& input [[user(fake0)]]
, device type_2& output [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    float factor = {};
    float _e2 = factor_1.value;
    factor = _e2;
    float _e7 = factor;
    factor = _e7 * 2.0;
    float _e14 = input[threadIdx.x];
    float _e15 = factor;
    output[threadIdx.x] = _e14 * _e15;
}
