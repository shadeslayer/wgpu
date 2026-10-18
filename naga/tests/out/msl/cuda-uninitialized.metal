// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
};

typedef int type_4[1];
struct type_5 {
    uint value;
    char _pad1[12];
};

int select_value(
    int value_1,
    bool choose_first
) {
    int value_2 = {};
    bool choose_first_1 = {};
    int result = {};
    value_2 = value_1;
    choose_first_1 = choose_first;
    bool _e5 = choose_first_1;
    if (_e5) {
        int _e7 = value_2;
        result = as_type<int>(as_type<uint>(_e7) + as_type<uint>(3));
    } else {
        int _e10 = value_2;
        result = as_type<int>(as_type<uint>(_e10) - as_type<uint>(2));
    }
    int _e12 = result;
    return _e12;
}

float scaled(
    float value_3
) {
    float value_4 = {};
    float result_1 = {};
    value_4 = value_3;
    float _e4 = value_4;
    result_1 = (_e4 * 2.0) + 4.0;
    float _e8 = result_1;
    return _e8;
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}

uint accumulate(
    uint limit
) {
    uint limit_1 = {};
    uint sum = {};
    uint i_1 = {};
    uint term = {};
    limit_1 = limit;
    sum = 0u;
    {
        i_1 = 0u;
        uint2 loop_bound = uint2(4294967295u);
        bool loop_init = true;
        while(true) {
            if (metal::all(loop_bound == uint2(0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                uint _e9 = i_1;
                i_1 = _e9 + static_cast<uint>(1);
            }
            loop_init = false;
            uint _e6 = i_1;
            uint _e7 = limit_1;
            if (_e6 < _e7) {
            } else {
                break;
            }
            uint _e15 = i_1;
            if (naga_mod(_e15, 2u) == 0u) {
                uint _e20 = i_1;
                term = _e20 + 1u;
            } else {
                uint _e23 = i_1;
                term = _e23 * 2u;
            }
            uint _e25 = term;
            uint _e26 = sum;
            sum = _e26 + _e25;
        }
    }
    uint _e28 = sum;
    return _e28;
}
int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}


struct uninitializedInput {
};
kernel void uninitialized(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_4& out [[user(fake0)]]
, device type_4 const& input [[user(fake0)]]
, constant type_5& n_1 [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint n = {};
    uint i = {};
    uint base = {};
    int value = {};
    int chosen = {};
    float weighted = {};
    bool negative = {};
    uint magnitude = {};
    int total = {};
    int iteration = {};
    int copied = {};
    int descending = {};
    int iteration_1 = {};
    int staged = {};
    int remaining = {};
    int partial = {};
    int next = {};
    int shadow = {};
    int shadow_1 = {};
    uint _e4 = n_1.value;
    n = _e4;
    i = (blockIdx.x * 64u) + threadIdx.x;
    uint _e14 = i;
    uint _e15 = n;
    if (_e14 < _e15) {
        uint _e19 = i;
        base = _e19 * 12u;
        uint _e22 = i;
        int _e24 = input[_e22];
        value = _e24;
        uint _e27 = i;
        if (naga_mod(_e27, 2u) == 0u) {
            int _e32 = value;
            chosen = as_type<int>(as_type<uint>(_e32) + as_type<uint>(7));
        } else {
            int _e35 = value;
            chosen = as_type<int>(as_type<uint>(_e35) - as_type<uint>(4));
        }
        int _e39 = chosen;
        uint _e37 = base;
        out[_e37] = _e39;
        int _e44 = value;
        uint _e46 = i;
        int _e50 = select_value(_e44, naga_mod(_e46, 3u) == 0u);
        uint _e41 = base;
        out[_e41 + 1u] = _e50;
        int _e52 = value;
        weighted = (static_cast<float>(_e52) * 1.5) + 2.0;
        float _e62 = weighted;
        uint _e59 = base;
        out[_e59 + 2u] = naga_f2i32(_e62);
        int _e66 = value;
        negative = _e66 < 0;
        bool _e72 = negative;
        uint _e69 = base;
        out[_e69 + 3u] = static_cast<int>(_e72);
        int _e76 = value;
        magnitude = static_cast<uint>(as_type<int>(as_type<uint>(_e76) + as_type<uint>(5)));
        uint _e84 = magnitude;
        uint _e80 = base;
        out[_e80 + 4u] = static_cast<int>(_e84 * 3u);
        uint _e92 = i;
        uint _e94 = accumulate(naga_mod(_e92, 5u));
        uint _e88 = base;
        out[_e88 + 5u] = static_cast<int>(_e94);
        total = 0;
        {
            iteration = 0;
            uint2 loop_bound_1 = uint2(4294967295u);
            bool loop_init_1 = true;
            while(true) {
                if (metal::all(loop_bound_1 == uint2(0u))) { break; }
                loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
                if (!loop_init_1) {
                    int _e103 = iteration;
                    iteration = as_type<int>(as_type<uint>(_e103) + as_type<uint>(1));
                }
                loop_init_1 = false;
                int _e101 = iteration;
                if (_e101 < 3) {
                } else {
                    break;
                }
                int _e107 = value;
                int _e108 = iteration;
                copied = as_type<int>(as_type<uint>(_e107) + as_type<uint>(_e108));
                int _e110 = copied;
                int _e111 = total;
                total = as_type<int>(as_type<uint>(_e111) + as_type<uint>(_e110));
            }
        }
        int _e117 = total;
        uint _e114 = base;
        out[_e114 + 6u] = _e117;
        descending = 0;
        iteration_1 = 0;
        {
            uint2 loop_bound_2 = uint2(4294967295u);
            bool loop_init_2 = true;
            while(true) {
                if (metal::all(loop_bound_2 == uint2(0u))) { break; }
                loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
                if (!loop_init_2) {
                    int _e126 = iteration_1;
                    iteration_1 = as_type<int>(as_type<uint>(_e126) + as_type<uint>(1));
                }
                loop_init_2 = false;
                int _e124 = iteration_1;
                if (_e124 < 3) {
                } else {
                    break;
                }
                int _e129 = value;
                int _e130 = iteration_1;
                staged = as_type<int>(as_type<uint>(_e129) - as_type<uint>(_e130));
                int _e132 = staged;
                int _e133 = descending;
                descending = as_type<int>(as_type<uint>(_e133) + as_type<uint>(_e132));
            }
        }
        int _e139 = descending;
        uint _e136 = base;
        out[_e136 + 7u] = _e139;
        remaining = 2;
        int _e143 = value;
        partial = _e143;
        uint2 loop_bound_3 = uint2(4294967295u);
        while(true) {
            if (metal::all(loop_bound_3 == uint2(0u))) { break; }
            loop_bound_3 -= uint2(loop_bound_3.y == 0u, 1u);
            int _e145 = remaining;
            if (_e145 > 0) {
            } else {
                break;
            }
            {
                int _e148 = partial;
                int _e149 = remaining;
                next = as_type<int>(as_type<uint>(_e148) + as_type<uint>(_e149));
                int _e151 = next;
                partial = _e151;
                int _e152 = remaining;
                remaining = as_type<int>(as_type<uint>(_e152) - as_type<uint>(1));
            }
        }
        int _e159 = partial;
        uint _e156 = base;
        out[_e156 + 8u] = _e159;
        int _e161 = value;
        shadow = _e161;
        {
            int _e164 = value;
            shadow_1 = as_type<int>(as_type<uint>(_e164) + as_type<uint>(10));
            int _e170 = shadow_1;
            uint _e167 = base;
            out[_e167 + 9u] = _e170;
        }
        int _e175 = shadow;
        uint _e172 = base;
        out[_e172 + 10u] = _e175;
        int _e180 = value;
        float _e182 = scaled(static_cast<float>(_e180));
        uint _e177 = base;
        out[_e177 + 11u] = naga_f2i32(_e182);
    }
}
