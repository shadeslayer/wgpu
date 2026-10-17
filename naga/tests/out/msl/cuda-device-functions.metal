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

float affine(
    float value_1,
    float scale,
    int offset
) {
    float value_2 = {};
    float scale_1 = {};
    int offset_1 = {};
    value_2 = value_1;
    scale_1 = scale;
    offset_1 = offset;
    float _e6 = scale_1;
    float _e7 = value_2;
    value_2 = _e7 * _e6;
    float _e9 = value_2;
    int _e10 = offset_1;
    return _e9 + static_cast<float>(_e10);
}

bool nonnegative(
    int value_3
) {
    int value_4 = {};
    value_4 = value_3;
    int _e3 = value_4;
    return _e3 >= 0;
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}

uint sum_steps(
    uint count,
    bool include_even
) {
    uint count_1 = {};
    bool include_even_1 = {};
    uint total = {};
    uint i_1 = {};
    bool local = {};
    count_1 = count;
    include_even_1 = include_even;
    total = 0u;
    {
        i_1 = 0u;
        uint2 loop_bound = uint2(4294967295u);
        bool loop_init = true;
        while(true) {
            if (metal::all(loop_bound == uint2(0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                uint _e11 = i_1;
                i_1 = _e11 + static_cast<uint>(1);
            }
            loop_init = false;
            uint _e8 = i_1;
            uint _e9 = count_1;
            if (_e8 < _e9) {
            } else {
                break;
            }
            bool _e15 = include_even_1;
            local = _e15;
            if (_e15) {
            } else {
                uint _e18 = i_1;
                local = naga_mod(_e18, 2u) != 0u;
            }
            bool _e22 = local;
            if (_e22) {
                uint _e23 = i_1;
                uint _e24 = total;
                total = _e24 + _e23;
            }
        }
    }
    uint _e26 = total;
    return _e26;
}

int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}

int choose(
    int value_5
) {
    int value_6 = {};
    value_6 = value_5;
    int _e2 = value_6;
    bool _e3 = nonnegative(_e2);
    if (_e3) {
        int _e4 = value_6;
        float _e8 = affine(static_cast<float>(_e4), 2.0, 1);
        return naga_f2i32(_e8);
    }
    int _e10 = value_6;
    return naga_neg(_e10);
}

int pair(
    int a,
    int b
) {
    int a_1 = {};
    int b_1 = {};
    a_1 = a;
    b_1 = b;
    int _e5 = a_1;
    int _e7 = b_1;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e5) * as_type<uint>(10))) + as_type<uint>(_e7));
}

struct device_functionsInput {
};
kernel void device_functions(
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
    int argument = {};
    uint _e3 = n_1.value;
    n = _e3;
    i = (blockIdx.x * 64u) + threadIdx.x;
    uint _e13 = i;
    uint _e14 = n;
    if (_e13 < _e14) {
        uint _e18 = i;
        base = _e18 * 10u;
        uint _e21 = i;
        value = as_type<int>(as_type<uint>(static_cast<int>(_e21)) - as_type<uint>(35));
        argument = 3;
        int _e29 = value;
        int _e30 = choose(_e29);
        uint _e27 = base;
        out[_e27] = _e30;
        int _e35 = value;
        float _e39 = affine(static_cast<float>(_e35), 1.5, 2);
        uint _e32 = base;
        out[_e32 + 1u] = naga_f2i32(_e39);
        uint _e48 = sum_steps(static_cast<uint>(5), true);
        uint _e42 = base;
        out[_e42 + 2u] = static_cast<int>(_e48);
        uint _e58 = sum_steps(5u, 0 != 0);
        uint _e51 = base;
        out[_e51 + 3u] = static_cast<int>(_e58);
        int _e64 = value;
        float _e70 = affine(static_cast<float>(_e64), static_cast<float>(2u), static_cast<int>(true));
        uint _e61 = base;
        out[_e61 + 4u] = naga_f2i32(_e70);
        int _e76 = value;
        uint _e73 = base;
        out[_e73 + 5u] = _e76;
        int _e81 = value;
        int _e82 = choose(_e81);
        int _e84 = value;
        int _e86 = choose(as_type<int>(as_type<uint>(_e84) + as_type<uint>(1)));
        uint _e78 = base;
        out[_e78 + 6u] = as_type<int>(as_type<uint>(_e82) + as_type<uint>(_e86));
        int _e92 = value;
        bool _e93 = nonnegative(_e92);
        uint _e89 = base;
        out[_e89 + 7u] = static_cast<int>(_e93);
        int _e99 = argument;
        int _e100 = argument;
        argument = as_type<int>(as_type<uint>(_e100) + as_type<uint>(1));
        int _e103 = pair(_e99, _e100);
        uint _e96 = base;
        out[_e96 + 8u] = _e103;
        int _e108 = argument;
        uint _e105 = base;
        out[_e105 + 9u] = _e108;
    }
}
