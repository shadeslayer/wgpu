// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef int type_4[1];
struct type_5 {
    uint value;
    char _pad1[12];
};

int next_value(
    int value_1
) {
    int value_2 = {};
    value_2 = value_1;
    int _e3 = value_2;
    return as_type<int>(as_type<uint>(_e3) + as_type<uint>(1));
}

int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

void consume(
    bool stop,
    int value_3,
    uint count,
    float scale
) {
    bool stop_1 = {};
    int value_4 = {};
    uint count_1 = {};
    float scale_1 = {};
    stop_1 = stop;
    value_4 = value_3;
    count_1 = count;
    scale_1 = scale;
    uint _e8 = count_1;
    int _e9 = value_4;
    value_4 = static_cast<int>(static_cast<uint>(_e9) + _e8);
    bool _e13 = stop_1;
    if (_e13) {
        return;
    }
    float _e14 = scale_1;
    int _e15 = value_4;
    value_4 = naga_f2i32(static_cast<float>(_e15) * _e14);
    return;
}

void forward(
    bool stop_2,
    int value_5
) {
    bool stop_3 = {};
    int value_6 = {};
    stop_3 = stop_2;
    value_6 = value_5;
    bool _e4 = stop_3;
    int _e5 = value_6;
    int _e6 = next_value(_e5);
    consume(_e4, _e6, 2u, 0.5);
    return;
}
uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}


struct void_functionsInput {
};
kernel void void_functions(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_4& out [[user(fake0)]]
, constant type_5& n_1 [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint n = {};
    uint i = {};
    uint base = {};
    int value = {};
    int iterations = {};
    uint _e3 = n_1.value;
    n = _e3;
    i = (blockIdx.x * 64u) + threadIdx.x;
    uint _e13 = i;
    uint _e14 = n;
    if (_e13 < _e14) {
        uint _e18 = i;
        base = _e18 * 6u;
        value = 2;
        int _e23 = value;
        value = as_type<int>(as_type<uint>(_e23) + as_type<uint>(1));
        consume(false, _e23, 1u, 1.5);
        int _e30 = value;
        uint _e28 = base;
        out[_e28] = _e30;
        int _e32 = value;
        value = as_type<int>(as_type<uint>(_e32) + as_type<uint>(1));
        int _e35 = next_value(_e32);
        consume(true, _e35, 2u, 0.5);
        int _e42 = value;
        uint _e39 = base;
        out[_e39 + 1u] = _e42;
        uint _e44 = i;
        int _e48 = value;
        value = as_type<int>(as_type<uint>(_e48) + as_type<uint>(1));
        forward(naga_mod(_e44, 2u) == 0u, _e48);
        int _e55 = value;
        uint _e52 = base;
        out[_e52 + 2u] = _e55;
        iterations = 0;
        {
            int _e61 = value;
            value = as_type<int>(as_type<uint>(_e61) + as_type<uint>(1));
            consume(0 != 0, _e61, static_cast<uint>(0), static_cast<float>(1u));
            uint2 loop_bound = uint2(4294967295u);
            bool loop_init = true;
            while(true) {
                if (metal::all(loop_bound == uint2(0u))) { break; }
                loop_bound -= uint2(loop_bound.y == 0u, 1u);
                if (!loop_init) {
                    int _e72 = value;
                    value = as_type<int>(as_type<uint>(_e72) + as_type<uint>(1));
                    int _e75 = value;
                    consume(false, _e75, 0u, 1.0);
                }
                loop_init = false;
                int _e69 = iterations;
                if (_e69 < 3) {
                } else {
                    break;
                }
                int _e78 = iterations;
                iterations = as_type<int>(as_type<uint>(_e78) + as_type<uint>(1));
                int _e82 = iterations;
                if (_e82 == 1) {
                    continue;
                }
            }
        }
        int _e88 = value;
        uint _e85 = base;
        out[_e85 + 3u] = _e88;
        int _e93 = iterations;
        uint _e90 = base;
        out[_e90 + 4u] = _e93;
        int _e98 = value;
        int _e99 = next_value(_e98);
        uint _e95 = base;
        out[_e95 + 5u] = _e99;
    }
}
