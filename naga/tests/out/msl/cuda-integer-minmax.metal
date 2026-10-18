// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef int type_1[1];

int min(
    int first_1,
    int second_1,
    int third
) {
    int first_2 = {};
    int second_2 = {};
    int third_1 = {};
    first_2 = first_1;
    second_2 = second_1;
    third_1 = third;
    int _e6 = first_2;
    int _e7 = second_2;
    int _e9 = third_1;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e6) + as_type<uint>(_e7))) + as_type<uint>(_e9))) + as_type<uint>(100));
}

int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}

int bounded(
    int value_1
) {
    int value_2 = {};
    value_2 = value_1;
    int _e2 = value_2;
    return metal::min(metal::max(_e2, naga_neg(2)), 2);
}

kernel void integer_minmax(
  device type_1& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int seed = {};
    int first = {};
    int second = {};
    int value = {};
    int _e4 = out[0];
    seed = _e4;
    int _e7 = seed;
    out[0] = metal::min(_e7, naga_neg(7));
    int _e13 = seed;
    out[1] = metal::max(_e13, naga_neg(7));
    out[2] = static_cast<int>(metal::min(3u, 9u));
    out[3] = static_cast<int>(metal::max(3u, 9u));
    out[4] = static_cast<int>(metal::min(static_cast<uint>(naga_neg(1)), 2u) == 2u);
    out[5] = static_cast<int>(metal::max(static_cast<uint>(naga_neg(1)), 2u) == 4294967295u);
    out[6] = static_cast<int>(metal::min(2u, static_cast<uint>(naga_neg(1))) == 2u);
    out[7] = static_cast<int>(metal::max(2u, static_cast<uint>(naga_neg(1))) == 4294967295u);
    out[8] = metal::min(static_cast<int>(true), static_cast<int>(false));
    out[9] = metal::max(static_cast<int>(false), static_cast<int>(true));
    int _e85 = seed;
    out[10] = metal::min(metal::max(_e85, naga_neg(5)), 2);
    out[11] = static_cast<int>(metal::min(static_cast<uint>(naga_neg(2)), 4u) == 4u);
    int _e103 = seed;
    int _e106 = min(_e103, 4, 5);
    out[12] = _e106;
    int _e109 = seed;
    int _e110 = bounded(_e109);
    out[13] = _e110;
    first = 3;
    second = 7;
    int _e117 = first;
    first = as_type<int>(as_type<uint>(_e117) + as_type<uint>(1));
    int _e120 = second;
    second = as_type<int>(as_type<uint>(_e120) + as_type<uint>(1));
    out[14] = metal::max(_e117, _e120);
    int _e126 = first;
    out[15] = _e126;
    int _e129 = second;
    out[16] = _e129;
    int _e131 = seed;
    value = _e131;
    int _e134 = value;
    int _e135 = value;
    value = as_type<int>(as_type<uint>(_e135) + as_type<uint>(1));
    out[17] = metal::max(_e134, _e135);
    int _e141 = value;
    out[18] = _e141;
    out[19] = static_cast<int>(metal::min(as_type<int>(as_type<uint>(naga_neg(2147483647)) - as_type<uint>(1)), 2147483647) == as_type<int>(as_type<uint>(naga_neg(2147483647)) - as_type<uint>(1)));
    out[20] = static_cast<int>(metal::max(4294967295u, 0u) == 4294967295u);
    out[21] = static_cast<int>(metal::min(static_cast<uint>(false), 3u) == 0u);
    out[22] = static_cast<int>(metal::max(static_cast<uint>(true), 0u) == 1u);
    int _e184 = seed;
    int _e185 = seed;
    out[23] = metal::min(_e184, _e185);
}
