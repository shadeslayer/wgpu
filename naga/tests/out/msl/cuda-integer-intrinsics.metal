// language: metal1.2
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef uint type_2[1];

int count(
    uint value
) {
    uint value_1 = {};
    value_1 = value;
    uint _e2 = value_1;
    return static_cast<int>(metal::popcount(_e2));
}

int scan(
    int value_2
) {
    int value_3 = {};
    value_3 = value_2;
    int _e2 = value_3;
    return as_type<int>(as_type<uint>((((metal::ctz(_e2) + 1) % 33) - 1)) + as_type<uint>(1));
}

int _clz(
    int first,
    int second
) {
    int first_1 = {};
    int second_1 = {};
    first_1 = first;
    second_1 = second;
    int _e4 = first_1;
    int _e5 = second_1;
    return as_type<int>(as_type<uint>(_e4) + as_type<uint>(_e5));
}
int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}

uint naga_f2u32(float value) {
    return static_cast<uint>(metal::clamp(value, 0.0, 4294967000.0));
}


kernel void integer_intrinsics(
  device type_2& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    out[0] = static_cast<uint>(metal::clz(static_cast<int>(true)));
    out[1] = static_cast<uint>(metal::clz(8));
    out[2] = static_cast<uint>(metal::clz(static_cast<int>(2147483648u)));
    out[3] = static_cast<uint>(metal::clz(naga_f2i32(8.75)));
    out[4] = static_cast<uint>(as_type<int>(as_type<uint>((((metal::ctz(static_cast<int>(true)) + 1) % 33) - 1)) + as_type<uint>(1)));
    out[5] = static_cast<uint>(as_type<int>(as_type<uint>((((metal::ctz(8) + 1) % 33) - 1)) + as_type<uint>(1)));
    out[6] = static_cast<uint>(as_type<int>(as_type<uint>((((metal::ctz(static_cast<int>(2147483648u)) + 1) % 33) - 1)) + as_type<uint>(1)));
    out[7] = static_cast<uint>(as_type<int>(as_type<uint>((((metal::ctz(naga_f2i32(8.75)) + 1) % 33) - 1)) + as_type<uint>(1)));
    out[8] = static_cast<uint>(static_cast<int>(metal::popcount(static_cast<uint>(true))));
    out[9] = static_cast<uint>(static_cast<int>(metal::popcount(static_cast<uint>(naga_neg(1)))));
    out[10] = static_cast<uint>(static_cast<int>(metal::popcount(2147483649u)));
    out[11] = static_cast<uint>(static_cast<int>(metal::popcount(naga_f2u32(3.75))));
    out[12] = metal::reverse_bits(static_cast<uint>(true));
    out[13] = metal::reverse_bits(static_cast<uint>(naga_neg(1)));
    out[14] = metal::reverse_bits(19088743u);
    out[15] = metal::reverse_bits(naga_f2u32(2.75));
    int _e106 = count(2147483649u);
    out[16] = static_cast<uint>(_e106);
    int _e112 = scan(static_cast<int>(2147483648u));
    out[17] = static_cast<uint>(_e112);
    out[18] = static_cast<uint>(metal::clz(0));
    out[19] = static_cast<uint>(as_type<int>(as_type<uint>((((metal::ctz(0) + 1) % 33) - 1)) + as_type<uint>(1)));
    out[20] = static_cast<uint>(static_cast<int>(metal::popcount(4294967295u)));
    out[21] = metal::reverse_bits(0u);
    out[22] = metal::reverse_bits(1u);
    out[23] = metal::reverse_bits(2147483648u);
    out[24] = static_cast<uint>(metal::clz(naga_neg(2)));
    out[25] = static_cast<uint>(as_type<int>(as_type<uint>((((metal::ctz(naga_neg(2)) + 1) % 33) - 1)) + as_type<uint>(1)));
    out[26] = static_cast<uint>(static_cast<int>(metal::popcount(2863311530u)));
    out[27] = metal::reverse_bits(2863311530u);
    int _e172 = _clz(3, 4);
    out[28] = static_cast<uint>(_e172);
    out[29] = static_cast<uint>(as_type<int>(as_type<uint>(static_cast<int>(metal::popcount(3u))) - as_type<uint>(3)) < 0);
}
