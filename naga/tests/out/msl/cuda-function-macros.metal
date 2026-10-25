// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

struct Pair_cuda_record_0_ {
    uint first;
    uint second;
};
typedef uint type_1[1];
struct type_3 {
    uint inner[3];
};

uint advance(
    uint value
) {
    uint value_1 = {};
    value_1 = value;
    uint _e3 = value_1;
    return _e3 + 3u;
}

struct function_macrosInput {
};
kernel void function_macros(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint lane = {};
    uint start = {};
    Pair_cuda_record_0_ pair = {};
    Pair_cuda_record_0_ local = {};
    type_3 values = {};
    metal::uint2 vector = {};
    uint counter = {};
    uint local_1 = {};
    uint declared = {};
    lane = threadIdx.x;
    uint _e5 = lane;
    if (_e5 >= 2u) {
        return;
    }
    uint _e9 = lane;
    start = _e9 * 24u;
    local.first = 11u;
    local.second = 13u;
    Pair_cuda_record_0_ _e17 = local;
    pair = _e17;
    values.inner[0] = 2u;
    values.inner[1] = 3u;
    values.inner[2] = 5u;
    vector = metal::uint2(17u, 19u);
    counter = 0u;
    uint _e31 = start;
    out[_e31] = 7u;
    uint _e35 = start;
    out[_e35 + 1u] = 11u;
    uint _e40 = start;
    out[_e40 + 2u] = 2u + 3u;
    uint _e47 = start;
    out[_e47 + 3u] = (1u + 2u) * 3u;
    uint _e56 = start;
    out[_e56 + 4u] = 5u + 6u;
    uint _e63 = start;
    out[_e63 + 5u] = 7u + 8u;
    uint _e74 = pair.first;
    uint _e70 = start;
    out[_e70 + 6u] = _e74;
    uint _e80 = pair.second;
    uint _e76 = start;
    out[_e76 + 7u] = _e80;
    uint _e82 = start;
    out[_e82 + 8u] = static_cast<uint>(3);
    uint _e91 = lane;
    uint _e93 = values.inner[_e91];
    uint _e88 = start;
    out[_e88 + 9u] = _e93;
    uint _e99 = vector.x;
    uint _e95 = start;
    out[_e95 + 10u] = _e99;
    uint _e105 = vector.y;
    uint _e101 = start;
    out[_e101 + 11u] = _e105;
    uint _e110 = counter;
    counter = _e110 + static_cast<uint>(1);
    uint _e114 = counter;
    counter = _e114 + static_cast<uint>(1);
    uint _e107 = start;
    out[_e107 + 12u] = _e114;
    uint _e122 = counter;
    uint _e119 = start;
    out[_e119 + 13u] = _e122;
    uint _e124 = start;
    out[_e124 + 14u] = 19u;
    uint _e132 = counter;
    uint _e129 = start;
    out[_e129 + 15u] = _e132;
    values.inner[0] = 23u;
    uint _e140 = values.inner[0];
    uint _e134 = start;
    out[_e134 + 16u] = _e140;
    uint _e147 = values.inner[0];
    uint _e142 = start;
    out[_e142 + 17u] = _e147;
    uint _e153 = lane;
    if (_e153 == 0u) {
        local_1 = 29u;
    } else {
        local_1 = 31u;
    }
    uint _e158 = local_1;
    uint _e149 = start;
    out[_e149 + 18u] = _e158;
    uint _e163 = lane;
    uint _e164 = advance(_e163);
    uint _e160 = start;
    out[_e160 + 19u] = _e164;
    declared = 37u;
    uint _e171 = declared;
    uint _e168 = start;
    out[_e168 + 20u] = _e171;
    uint _e173 = start;
    out[_e173 + 21u] = as_type<uint>(-(0.0));
    uint _e180 = start;
    out[_e180 + 22u] = 4294967295u;
    uint _e189 = lane;
    uint _e185 = start;
    out[_e185 + 23u] = 41u + _e189;
}
