// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef uint type_1[1];
struct type_3 {
    int value;
    char _pad1[12];
};
struct type_4 {
    uint value;
    char _pad1[12];
};
int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}

int naga_mod(int lhs, int rhs) {
    int divisor = metal::select(rhs, 1, (lhs == (-2147483647 - 1) & rhs == -1) | (rhs == 0));
    return lhs - (lhs / divisor) * divisor;
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}


struct integer_opsInput {
};
kernel void integer_ops(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, constant type_3& value_1 [[user(fake0)]]
, constant type_4& amount_1 [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int value = {};
    uint amount = {};
    int remainder = {};
    uint bits = {};
    uint local = {};
    int _e3 = value_1.value;
    value = _e3;
    uint _e7 = amount_1.value;
    amount = _e7;
    int _e12 = value;
    remainder = naga_mod(_e12, naga_neg(3));
    int _e15 = remainder;
    bits = static_cast<uint>(_e15) & 15u;
    uint _e20 = bits;
    bits = _e20 | 8u;
    uint _e23 = bits;
    bits = _e23 ^ 1u;
    uint _e25 = amount;
    uint _e26 = bits;
    local = _e25;
    uint _e28 = local;
    bits = _e26 << _e28;
    uint _e31 = bits;
    bits = _e31 >> 1u;
    uint _e34 = bits;
    bits = naga_mod(_e34, 97u);
    uint _e37 = bits;
    bits = _e37 & 255u;
    uint _e42 = bits;
    out[threadIdx.x] = ~(_e42);
}
