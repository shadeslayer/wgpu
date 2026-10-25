// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef uint type_1[1];

uint later(
    uint value
) {
    uint value_1 = {};
    value_1 = value;
    uint _e3 = value_1;
    return _e3 + 73u;
}

struct preprocessing_conditionsInput {
};
kernel void preprocessing_conditions(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint lane = {};
    uint start = {};
    uint counter = {};
    lane = threadIdx.x;
    uint _e5 = lane;
    if (_e5 >= 2u) {
        return;
    }
    uint _e9 = lane;
    start = _e9 * 24u;
    uint _e11 = start;
    out[_e11] = 7u;
    uint _e15 = start;
    out[_e15 + 1u] = 11u;
    uint _e20 = start;
    out[_e20 + 2u] = 13u;
    uint _e25 = start;
    out[_e25 + 3u] = 17u;
    uint _e30 = start;
    out[_e30 + 4u] = 19u;
    uint _e35 = start;
    out[_e35 + 5u] = 23u;
    uint _e40 = start;
    out[_e40 + 6u] = 29u;
    uint _e45 = start;
    out[_e45 + 7u] = 31u;
    uint _e50 = start;
    out[_e50 + 8u] = 37u;
    uint _e55 = start;
    out[_e55 + 9u] = 41u;
    uint _e60 = start;
    out[_e60 + 10u] = 43u;
    uint _e65 = start;
    out[_e65 + 11u] = 47u;
    uint _e70 = start;
    out[_e70 + 12u] = 53u;
    uint _e75 = start;
    out[_e75 + 13u] = 59u;
    uint _e80 = start;
    out[_e80 + 14u] = 61u;
    uint _e85 = start;
    out[_e85 + 15u] = 67u;
    uint _e90 = start;
    out[_e90 + 16u] = 71u;
    uint _e98 = lane;
    uint _e99 = later(_e98);
    uint _e95 = start;
    out[_e95 + 17u] = _e99;
    counter = 0u;
    uint _e106 = counter;
    counter = _e106 + static_cast<uint>(1);
    uint _e103 = start;
    out[_e103 + 18u] = _e106;
    uint _e114 = counter;
    uint _e111 = start;
    out[_e111 + 19u] = _e114;
    uint _e116 = start;
    out[_e116 + 20u] = static_cast<uint>(true);
    uint _e125 = lane;
    uint _e122 = start;
    out[_e122 + 21u] = _e125;
    uint _e127 = start;
    out[_e127 + 22u] = as_type<uint>(-(0.0));
    uint _e134 = start;
    out[_e134 + 23u] = 4294967295u;
}
