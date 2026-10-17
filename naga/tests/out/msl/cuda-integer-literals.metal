// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
};

typedef uint type_1[1];
struct type_2 {
    uint value;
    char _pad1[12];
};
struct type_3 {
    uint inner[64];
};

struct integer_literalsInput {
};
kernel void integer_literals(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, uint __local_invocation_index [[thread_index_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, device type_1 const& input [[user(fake0)]]
, constant type_2& n_1 [[user(fake0)]]
, threadgroup type_3& shared_values
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    if (__local_invocation_index == 0u) {
        shared_values = {};
    }
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    uint n = {};
    uint i = {};
    uint value = {};
    uint shift = {};
    uint base = {};
    uint _e4 = n_1.value;
    n = _e4;
    i = (blockIdx.x * 64u) + threadIdx.x;
    uint _e15 = i;
    uint _e16 = n;
    if (_e15 < _e16) {
        uint _e19 = i;
        uint _e21 = input[_e19];
        value = _e21;
        uint _e24 = value;
        shift = _e24 & 3u;
        uint _e28 = i;
        base = _e28 * 18u;
        uint _e33 = value;
        shared_values.inner[threadIdx.x] = _e33 ^ static_cast<uint>(255);
        uint _e40 = shared_values.inner[threadIdx.x];
        uint _e36 = base;
        out[_e36] = _e40;
        uint _e46 = value;
        uint _e42 = base;
        out[_e42 + 1u] = _e46 & static_cast<uint>(255);
        uint _e54 = value;
        uint _e50 = base;
        out[_e50 + 2u] = _e54 & 240u;
        uint _e61 = value;
        uint _e57 = base;
        out[_e57 + 3u] = _e61 + 1000u;
        uint _e68 = shift;
        uint _e64 = base;
        out[_e64 + 4u] = 2147483648u >> _e68;
        uint _e76 = shift;
        uint _e71 = base;
        out[_e71 + 5u] = static_cast<uint>(static_cast<int>(2147483648u) >> _e76);
        uint _e84 = value;
        uint _e80 = base;
        out[_e80 + 6u] = 4294967295u ^ _e84;
        uint _e91 = value;
        uint _e87 = base;
        out[_e87 + 7u] = static_cast<uint>(_e91 < 2147483648u);
        uint _e98 = value;
        uint _e95 = base;
        out[_e95 + 8u] = static_cast<uint>(static_cast<int>(_e98) < 2147483647);
        uint _e108 = shift;
        uint _e104 = base;
        out[_e104 + 9u] = 2147483648u >> _e108;
        uint _e115 = value;
        uint _e111 = base;
        out[_e111 + 10u] = 255u + _e115;
        uint _e122 = value;
        uint _e118 = base;
        out[_e118 + 11u] = ((_e122 + 10u) + 8u) + 10u;
        uint _e133 = value;
        uint _e129 = base;
        out[_e129 + 12u] = _e133 + static_cast<uint>(15);
        uint _e141 = value;
        uint _e137 = base;
        out[_e137 + 13u] = 4294967295u - _e141;
        uint _e148 = value;
        uint _e144 = base;
        out[_e144 + 14u] = 3735928559u ^ _e148;
        uint _e156 = value;
        uint _e151 = base;
        out[_e151 + 15u] = static_cast<uint>(~(2147483647) < static_cast<int>(_e156));
        uint _e165 = value;
        uint _e161 = base;
        out[_e161 + 16u] = (_e165 + static_cast<uint>(30)) + static_cast<uint>(2);
        uint _e176 = value;
        uint _e172 = base;
        out[_e172 + 17u] = (_e176 + static_cast<uint>(30)) - static_cast<uint>(2);
    }
}
