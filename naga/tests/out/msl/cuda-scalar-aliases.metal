// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
};

typedef uint type_3[1];
typedef int type_4[1];
struct type_5 {
    uint value;
    char _pad1[12];
};
struct type_6 {
    uint inner[64];
};

int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

int transform(
    float value,
    float multiplier,
    int offset
) {
    float value_1 = {};
    float multiplier_1 = {};
    int offset_1 = {};
    value_1 = value;
    multiplier_1 = multiplier;
    offset_1 = offset;
    float _e6 = value_1;
    float _e7 = multiplier_1;
    int _e9 = offset_1;
    return naga_f2i32((_e6 * _e7) + static_cast<float>(_e9));
}
int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}

uint naga_f2u32(float value) {
    return static_cast<uint>(metal::clamp(value, 0.0, 4294967000.0));
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}


struct scalar_aliasesInput {
};
kernel void scalar_aliases(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, uint __local_invocation_index [[thread_index_in_threadgroup]]
, device type_3& out [[user(fake0)]]
, device type_4 const& input [[user(fake0)]]
, constant type_5& n_1 [[user(fake0)]]
, threadgroup type_6& shared_values
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    if (__local_invocation_index == 0u) {
        shared_values = {};
    }
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    uint n = {};
    uint i = {};
    uint base = {};
    uint lane = {};
    int signed_count = {};
    float fraction = {};
    int after_block = {};
    float coefficient = {};
    bool selected = {};
    int total = {};
    int current = {};
    float after_loop = {};
    uint _e4 = n_1.value;
    n = _e4;
    i = (blockIdx.x * 64u) + threadIdx.x;
    uint _e15 = i;
    uint _e16 = n;
    if (_e15 < _e16) {
        uint _e20 = i;
        base = _e20 * 10u;
        lane = threadIdx.x;
        uint _e27 = i;
        uint _e24 = lane;
        shared_values.inner[_e24] = _e27 + 10u;
        uint _e31 = lane;
        uint _e33 = shared_values.inner[_e31];
        uint _e29 = base;
        out[_e29] = _e33;
        uint _e38 = i;
        int _e40 = input[_e38];
        int _e44 = transform(static_cast<float>(_e40), 2.0, 1);
        uint _e35 = base;
        out[_e35 + 1u] = static_cast<uint>(_e44);
        signed_count = naga_neg(3);
        int _e53 = signed_count;
        uint _e50 = base;
        out[_e50 + 2u] = static_cast<uint>(_e53);
        {
            fraction = 3.75;
            float _e61 = fraction;
            uint _e58 = base;
            out[_e58 + 3u] = naga_f2u32(_e61);
        }
        after_block = naga_neg(5);
        int _e70 = after_block;
        uint _e67 = base;
        out[_e67 + 4u] = static_cast<uint>(_e70);
        coefficient = 1.25;
        float _e79 = coefficient;
        uint _e75 = base;
        out[_e75 + 5u] = naga_f2u32(_e79 * 4.0);
        uint _e84 = i;
        selected = naga_mod(_e84, 2u) == 0u;
        bool _e92 = selected;
        uint _e89 = base;
        out[_e89 + 6u] = static_cast<uint>(_e92);
        total = 0;
        {
            uint2 loop_bound = uint2(4294967295u);
            while(true) {
                if (metal::all(loop_bound == uint2(0u))) { break; }
                loop_bound -= uint2(loop_bound.y == 0u, 1u);
                int _e97 = total;
                if (_e97 < 3) {
                } else {
                    break;
                }
                int _e100 = total;
                current = _e100;
                int _e102 = current;
                total = as_type<int>(as_type<uint>(_e102) + as_type<uint>(1));
            }
        }
        int _e108 = total;
        uint _e105 = base;
        out[_e105 + 7u] = static_cast<uint>(_e108);
        after_loop = 1.5;
        float _e117 = after_loop;
        uint _e113 = base;
        out[_e113 + 8u] = naga_f2u32(_e117 * 2.0);
        uint _e124 = n;
        uint _e121 = base;
        out[_e121 + 9u] = naga_f2u32(static_cast<float>(_e124));
    }
}
