// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
};

struct type_1 {
    int inner[2];
};
struct Record_cuda_record_0_ {
    int value;
    type_1 history;
};
typedef metal::int2 type_4[1];
typedef uint type_6[1];

metal::int2 operator_u002b(
    metal::int2 value
) {
    metal::int2 value_1 = {};
    value_1 = value;
    int _e3 = value_1.x;
    int _e5 = value_1.y;
    return metal::int2(_e3, _e5);
}

int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}

metal::int2 operator_u002d(
    metal::int2 value_2
) {
    metal::int2 value_3 = {};
    value_3 = value_2;
    int _e3 = value_3.x;
    int _e6 = value_3.y;
    return metal::int2(naga_neg(_e3), naga_neg(_e6));
}

metal::int2 operator_u007e(
    metal::int2 value_4
) {
    metal::int2 value_5 = {};
    value_5 = value_4;
    int _e3 = value_5.x;
    int _e6 = value_5.y;
    return metal::int2(~(_e3), ~(_e6));
}

bool operator_u0021_(
    metal::int2 value_6
) {
    metal::int2 value_7 = {};
    bool local_1 = {};
    value_7 = value_6;
    int _e4 = value_7.x;
    bool _e5 = _e4 == 0;
    local_1 = _e5;
    if (_e5) {
        int _e9 = value_7.y;
        local_1 = _e9 == 0;
    }
    bool _e11 = local_1;
    return _e11;
}

int operator_u002a(
    metal::int2 value_8
) {
    metal::int2 value_9 = {};
    value_9 = value_8;
    int _e4 = value_9.x;
    int _e7 = value_9.y;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e4) * as_type<uint>(10))) + as_type<uint>(_e7));
}

metal::int2 operator_u0026_(
    metal::int2 value_10
) {
    metal::int2 value_11 = {};
    value_11 = value_10;
    int _e3 = value_11.y;
    int _e5 = value_11.x;
    return metal::int2(_e3, _e5);
}

metal::int2 operator_u002b_1(
    metal::int2 left_1,
    metal::int2 right_1
) {
    metal::int2 left_2 = {};
    metal::int2 right_2 = {};
    left_2 = left_1;
    right_2 = right_1;
    int _e6 = left_2.x;
    int _e7 = right_2.x;
    int _e11 = left_2.y;
    int _e12 = right_2.y;
    return metal::int2(as_type<int>(as_type<uint>(_e6) + as_type<uint>(_e7)), as_type<int>(as_type<uint>(_e11) + as_type<uint>(_e12)));
}

metal::int2 operator_u002d_1(
    metal::int2 left_3,
    metal::int2 right_3
) {
    metal::int2 left_4 = {};
    metal::int2 right_4 = {};
    left_4 = left_3;
    right_4 = right_3;
    int _e6 = left_4.x;
    int _e7 = right_4.x;
    int _e11 = left_4.y;
    int _e12 = right_4.y;
    return metal::int2(as_type<int>(as_type<uint>(_e6) - as_type<uint>(_e7)), as_type<int>(as_type<uint>(_e11) - as_type<uint>(_e12)));
}

metal::int2 operator_u002a_1(
    metal::int2 left_5,
    metal::int2 right_5
) {
    metal::int2 left_6 = {};
    metal::int2 right_6 = {};
    left_6 = left_5;
    right_6 = right_5;
    int _e6 = left_6.x;
    int _e7 = right_6.x;
    int _e11 = left_6.y;
    int _e12 = right_6.y;
    return metal::int2(as_type<int>(as_type<uint>(_e6) * as_type<uint>(_e7)), as_type<int>(as_type<uint>(_e11) * as_type<uint>(_e12)));
}

int naga_div(int lhs, int rhs) {
    return lhs / metal::select(rhs, 1, (lhs == (-2147483647 - 1) & rhs == -1) | (rhs == 0));
}

metal::int2 operator_u002f(
    metal::int2 left_7,
    metal::int2 right_7
) {
    metal::int2 left_8 = {};
    metal::int2 right_8 = {};
    left_8 = left_7;
    right_8 = right_7;
    int _e6 = left_8.x;
    int _e7 = right_8.x;
    int _e11 = left_8.y;
    int _e12 = right_8.y;
    return metal::int2(naga_div(_e6, _e7), naga_div(_e11, _e12));
}

int naga_mod(int lhs, int rhs) {
    int divisor = metal::select(rhs, 1, (lhs == (-2147483647 - 1) & rhs == -1) | (rhs == 0));
    return lhs - (lhs / divisor) * divisor;
}

metal::int2 operator_u0025_(
    metal::int2 left_9,
    metal::int2 right_9
) {
    metal::int2 left_10 = {};
    metal::int2 right_10 = {};
    left_10 = left_9;
    right_10 = right_9;
    int _e6 = left_10.x;
    int _e7 = right_10.x;
    int _e11 = left_10.y;
    int _e12 = right_10.y;
    return metal::int2(naga_mod(_e6, _e7), naga_mod(_e11, _e12));
}

metal::int2 operator_u0026_1(
    metal::int2 left_11,
    metal::int2 right_11
) {
    metal::int2 left_12 = {};
    metal::int2 right_12 = {};
    left_12 = left_11;
    right_12 = right_11;
    int _e6 = left_12.x;
    int _e7 = right_12.x;
    int _e11 = left_12.y;
    int _e12 = right_12.y;
    return metal::int2(_e6 & _e7, _e11 & _e12);
}

metal::int2 operator_u007c(
    metal::int2 left_13,
    metal::int2 right_13
) {
    metal::int2 left_14 = {};
    metal::int2 right_14 = {};
    left_14 = left_13;
    right_14 = right_13;
    int _e6 = left_14.x;
    int _e7 = right_14.x;
    int _e11 = left_14.y;
    int _e12 = right_14.y;
    return metal::int2(_e6 | _e7, _e11 | _e12);
}

metal::int2 operator_u005e(
    metal::int2 left_15,
    metal::int2 right_15
) {
    metal::int2 left_16 = {};
    metal::int2 right_16 = {};
    left_16 = left_15;
    right_16 = right_15;
    int _e6 = left_16.x;
    int _e7 = right_16.x;
    int _e11 = left_16.y;
    int _e12 = right_16.y;
    return metal::int2(_e6 ^ _e7, _e11 ^ _e12);
}

metal::int2 operator_(
    metal::int2 left_17,
    metal::int2 right_17
) {
    metal::int2 left_18 = {};
    metal::int2 right_18 = {};
    uint local_2 = {};
    uint local_3 = {};
    left_18 = left_17;
    right_18 = right_17;
    int _e5 = left_18.x;
    int _e7 = right_18.x;
    local_2 = static_cast<uint>(_e7);
    uint _e10 = local_2;
    int _e13 = left_18.y;
    int _e15 = right_18.y;
    local_3 = static_cast<uint>(_e15);
    uint _e18 = local_3;
    return metal::int2(_e5 << _e10, _e13 << _e18);
}

metal::int2 operator_1(
    metal::int2 left_19,
    metal::int2 right_19
) {
    metal::int2 left_20 = {};
    metal::int2 right_20 = {};
    left_20 = left_19;
    right_20 = right_19;
    int _e5 = left_20.x;
    int _e7 = right_20.x;
    int _e11 = left_20.y;
    int _e13 = right_20.y;
    return metal::int2(_e5 >> static_cast<uint>(_e7), _e11 >> static_cast<uint>(_e13));
}

Record_cuda_record_0_ operator_u002a_2(
    Record_cuda_record_0_ value_12
) {
    Record_cuda_record_0_ value_13 = {};
    value_13 = value_12;
    int _e3 = value_13.value;
    value_13.value = as_type<int>(as_type<uint>(_e3) + as_type<uint>(1));
    Record_cuda_record_0_ _e6 = value_13;
    return _e6;
}

int operator_u0026_2(
    Record_cuda_record_0_ value_14
) {
    Record_cuda_record_0_ value_15 = {};
    value_15 = value_14;
    int _e3 = value_15.value;
    return _e3;
}

Record_cuda_record_0_ operator_u002d_2(
    Record_cuda_record_0_ value_16
) {
    Record_cuda_record_0_ value_17 = {};
    value_17 = value_16;
    int _e4 = value_17.value;
    value_17.value = naga_neg(_e4);
    Record_cuda_record_0_ _e6 = value_17;
    return _e6;
}

Record_cuda_record_0_ operator_u002a_3(
    Record_cuda_record_0_ value_18,
    int factor
) {
    Record_cuda_record_0_ value_19 = {};
    int factor_1 = {};
    value_19 = value_18;
    factor_1 = factor;
    int _e5 = factor_1;
    int _e6 = value_19.value;
    value_19.value = as_type<int>(as_type<uint>(_e6) * as_type<uint>(_e5));
    int _e11 = factor_1;
    int _e12 = value_19.history.inner[1];
    value_19.history.inner[1] = as_type<int>(as_type<uint>(_e12) + as_type<uint>(_e11));
    Record_cuda_record_0_ _e14 = value_19;
    return _e14;
}

struct operator_functionsInput {
};
kernel void operator_functions(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_4 const& input [[user(fake0)]]
, device type_6& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint base = {};
    metal::int2 left = {};
    metal::int2 right = {};
    Record_cuda_record_0_ original = {};
    Record_cuda_record_0_ scaled = {};
    Record_cuda_record_0_ local = {};
    base = threadIdx.x * 36u;
    metal::int2 _e10 = input[threadIdx.x];
    left = _e10;
    right = metal::int2(3, 2);
    metal::int2 _e19 = left;
    metal::int2 _e20 = operator_u002b(_e19);
    uint _e16 = base;
    out[_e16 + 0u] = static_cast<uint>(_e20.x);
    metal::int2 _e27 = left;
    metal::int2 _e28 = operator_u002d(_e27);
    uint _e24 = base;
    out[_e24 + 1u] = static_cast<uint>(_e28.y);
    metal::int2 _e35 = left;
    metal::int2 _e36 = operator_u007e(_e35);
    uint _e32 = base;
    out[_e32 + 2u] = static_cast<uint>(_e36.x);
    metal::int2 _e43 = left;
    bool _e44 = operator_u0021_(_e43);
    uint _e40 = base;
    out[_e40 + 3u] = static_cast<uint>(_e44);
    metal::int2 _e50 = left;
    int _e51 = operator_u002a(_e50);
    uint _e47 = base;
    out[_e47 + 4u] = static_cast<uint>(_e51);
    metal::int2 _e57 = left;
    metal::int2 _e58 = operator_u0026_(_e57);
    uint _e54 = base;
    out[_e54 + 5u] = static_cast<uint>(_e58.x);
    metal::int2 _e65 = left;
    metal::int2 _e66 = right;
    metal::int2 _e67 = operator_u002b_1(_e65, _e66);
    uint _e62 = base;
    out[_e62 + 6u] = static_cast<uint>(_e67.x);
    metal::int2 _e74 = left;
    metal::int2 _e75 = right;
    metal::int2 _e76 = operator_u002d_1(_e74, _e75);
    uint _e71 = base;
    out[_e71 + 7u] = static_cast<uint>(_e76.y);
    metal::int2 _e83 = left;
    metal::int2 _e84 = right;
    metal::int2 _e85 = operator_u002a_1(_e83, _e84);
    uint _e80 = base;
    out[_e80 + 8u] = static_cast<uint>(_e85.x);
    metal::int2 _e92 = left;
    metal::int2 _e93 = right;
    metal::int2 _e94 = operator_u002f(_e92, _e93);
    uint _e89 = base;
    out[_e89 + 9u] = static_cast<uint>(_e94.y);
    metal::int2 _e101 = left;
    metal::int2 _e102 = right;
    metal::int2 _e103 = operator_u0025_(_e101, _e102);
    uint _e98 = base;
    out[_e98 + 10u] = static_cast<uint>(_e103.y);
    metal::int2 _e110 = left;
    metal::int2 _e111 = right;
    metal::int2 _e112 = operator_u0026_1(_e110, _e111);
    uint _e107 = base;
    out[_e107 + 11u] = static_cast<uint>(_e112.x);
    metal::int2 _e119 = left;
    metal::int2 _e120 = right;
    metal::int2 _e121 = operator_u007c(_e119, _e120);
    uint _e116 = base;
    out[_e116 + 12u] = static_cast<uint>(_e121.y);
    metal::int2 _e128 = left;
    metal::int2 _e129 = right;
    metal::int2 _e130 = operator_u005e(_e128, _e129);
    uint _e125 = base;
    out[_e125 + 13u] = static_cast<uint>(_e130.x);
    metal::int2 _e137 = left;
    metal::int2 _e138 = right;
    metal::int2 _e139 = operator_(_e137, _e138);
    uint _e134 = base;
    out[_e134 + 14u] = static_cast<uint>(_e139.x);
    metal::int2 _e146 = left;
    metal::int2 _e147 = right;
    metal::int2 _e148 = operator_1(_e146, _e147);
    uint _e143 = base;
    out[_e143 + 15u] = static_cast<uint>(_e148.y);
    metal::int2 _e155 = left;
    metal::int2 _e156 = operator_u002b(_e155);
    uint _e152 = base;
    out[_e152 + 16u] = static_cast<uint>(_e156.y);
    metal::int2 _e163 = left;
    metal::int2 _e164 = right;
    metal::int2 _e165 = operator_u002d_1(_e163, _e164);
    uint _e160 = base;
    out[_e160 + 17u] = static_cast<uint>(_e165.x);
    metal::int2 _e172 = left;
    int _e173 = operator_u002a(_e172);
    uint _e169 = base;
    out[_e169 + 18u] = static_cast<uint>(_e173);
    metal::int2 _e179 = left;
    metal::int2 _e180 = right;
    metal::int2 _e181 = operator_u0026_1(_e179, _e180);
    uint _e176 = base;
    out[_e176 + 19u] = static_cast<uint>(_e181.y);
    metal::int2 _e188 = left;
    metal::int2 _e189 = operator_u0026_(_e188);
    int _e190 = operator_u002a(_e189);
    uint _e185 = base;
    out[_e185 + 20u] = static_cast<uint>(_e190);
    metal::int2 _e196 = left;
    metal::int2 _e197 = operator_u0026_(_e196);
    metal::int2 _e198 = operator_u0026_(_e197);
    uint _e193 = base;
    out[_e193 + 21u] = static_cast<uint>(_e198.x);
    metal::int2 _e208 = left;
    metal::int2 _e205 = right;
    metal::int2 _e206 = right;
    metal::int2 _e207 = operator_u002a_1(_e205, _e206);
    metal::int2 _e209 = operator_u002b_1(_e208, _e207);
    uint _e202 = base;
    out[_e202 + 22u] = static_cast<uint>(_e209.x);
    metal::int2 _e216 = left;
    metal::int2 _e220 = right;
    metal::int2 _e221 = operator_u002b_1(_e220, metal::int2(1, 1));
    metal::int2 _e222 = operator_(_e216, _e221);
    uint _e213 = base;
    out[_e213 + 23u] = static_cast<uint>(_e222.x);
    metal::int2 _e234 = left;
    metal::int2 _e232 = right;
    metal::int2 _e233 = operator_u0026_1(_e232, metal::int2(1, 1));
    metal::int2 _e235 = operator_u007c(_e234, _e233);
    uint _e226 = base;
    out[_e226 + 24u] = static_cast<uint>(_e235.y);
    metal::int2 _e242 = left;
    metal::int2 _e243 = right;
    metal::int2 _e244 = operator_u002d_1(_e242, _e243);
    metal::int2 _e245 = right;
    metal::int2 _e246 = operator_u002d_1(_e244, _e245);
    uint _e239 = base;
    out[_e239 + 25u] = static_cast<uint>(_e246.x);
    original.value = 9;
    original.history.inner[0] = 1;
    original.history.inner[1] = 2;
    Record_cuda_record_0_ _e259 = original;
    Record_cuda_record_0_ _e260 = operator_u002a_3(_e259, 3);
    scaled = _e260;
    local.value = 9;
    local.history.inner[0] = 1;
    local.history.inner[1] = 2;
    Record_cuda_record_0_ _e273 = local;
    int _e274 = operator_u0026_2(_e273);
    uint _e262 = base;
    out[_e262 + 26u] = static_cast<uint>(_e274);
    Record_cuda_record_0_ _e280 = original;
    Record_cuda_record_0_ _e281 = operator_u002a_2(_e280);
    int _e282 = operator_u0026_2(_e281);
    uint _e277 = base;
    out[_e277 + 27u] = static_cast<uint>(_e282);
    int _e289 = scaled.value;
    uint _e285 = base;
    out[_e285 + 28u] = static_cast<uint>(_e289);
    int _e298 = scaled.history.inner[1];
    uint _e292 = base;
    out[_e292 + 29u] = static_cast<uint>(_e298);
    int _e305 = original.value;
    uint _e301 = base;
    out[_e301 + 30u] = static_cast<uint>(_e305);
    int _e314 = original.history.inner[1];
    uint _e308 = base;
    out[_e308 + 31u] = static_cast<uint>(_e314);
    Record_cuda_record_0_ _e320 = original;
    Record_cuda_record_0_ _e321 = operator_u002d_2(_e320);
    uint _e317 = base;
    out[_e317 + 32u] = static_cast<uint>(_e321.value);
    Record_cuda_record_0_ _e328 = original;
    Record_cuda_record_0_ _e330 = operator_u002a_3(_e328, 2);
    uint _e325 = base;
    out[_e325 + 33u] = static_cast<uint>(_e330.value);
    Record_cuda_record_0_ _e337 = original;
    Record_cuda_record_0_ _e338 = operator_u002a_2(_e337);
    int _e339 = operator_u0026_2(_e338);
    uint _e334 = base;
    out[_e334 + 34u] = static_cast<uint>(_e339);
    bool _e348 = operator_u0021_(metal::int2(0, 0));
    uint _e342 = base;
    out[_e342 + 35u] = static_cast<uint>(_e348);
}
