// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef int type_1[1];
struct type_2 {
    int value;
    char _pad1[12];
};

struct updatesInput {
};
kernel void updates(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, constant type_2& value_1 [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int value = {};
    int previous = {};
    int _e3 = value_1.value;
    value = _e3;
    int _e6 = value;
    value = as_type<int>(as_type<uint>(_e6) + as_type<uint>(1));
    previous = _e6;
    int _e12 = previous;
    int _e13 = out[threadIdx.x];
    out[threadIdx.x] = as_type<int>(as_type<uint>(_e13) + as_type<uint>(_e12));
    int _e17 = value;
    value = as_type<int>(as_type<uint>(_e17) + as_type<uint>(1));
    int _e20 = value;
    out[threadIdx.x] = _e20;
}
