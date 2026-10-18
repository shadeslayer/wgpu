// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
};

typedef int type_1[1];
struct type_3 {
    uint value;
    char _pad1[12];
};

void consume(
    int value_1
) {
    int value_2 = {};
    value_2 = value_1;
    int _e3 = value_2;
    if (_e3 < 0) {
        return;
    } else {
        return;
    }
}

int pair_add(
    int left_1,
    int right_1
) {
    int left_2 = {};
    int right_2 = {};
    left_2 = left_1;
    right_2 = right_1;
    int _e4 = left_2;
    int _e5 = right_2;
    return as_type<int>(as_type<uint>(_e4) + as_type<uint>(_e5));
}
uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}


struct commaInput {
};
kernel void comma(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, device type_1 const& input [[user(fake0)]]
, constant type_3& n_1 [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint n = {};
    uint i = {};
    uint base = {};
    int value = {};
    bool choose = {};
    int x = {};
    int sequenced = {};
    int target = {};
    int rhs = {};
    int middle = {};
    int selected = {};
    int local = {};
    int left = {};
    int right = {};
    int identity = {};
    uint position = {};
    int effect = {};
    int total = {};
    int iteration = {};
    int current = {};
    int ticks = {};
    int passes = {};
    int argument = {};
    int first = {};
    int second = {};
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
        choose = naga_mod(_e27, 2u) == 0u;
        int _e32 = value;
        x = _e32;
        int _e35 = x;
        x = as_type<int>(as_type<uint>(_e35) + as_type<uint>(2));
        int _e38 = x;
        x = as_type<int>(as_type<uint>(_e38) * as_type<uint>(3));
        int _e41 = x;
        sequenced = as_type<int>(as_type<uint>(_e41) - as_type<uint>(1));
        int _e45 = sequenced;
        uint _e43 = base;
        out[_e43] = _e45;
        int _e50 = x;
        uint _e47 = base;
        out[_e47 + 1u] = _e50;
        target = 0;
        int _e53 = value;
        target = _e53;
        int _e55 = target;
        target = as_type<int>(as_type<uint>(_e55) + as_type<uint>(4));
        int _e61 = target;
        uint _e58 = base;
        out[_e58 + 2u] = _e61;
        int _e64 = value;
        target = as_type<int>(as_type<uint>(_e64) + as_type<uint>(1));
        int _e67 = target;
        rhs = as_type<int>(as_type<uint>(_e67) + as_type<uint>(3));
        int _e74 = rhs;
        int _e76 = target;
        uint _e70 = base;
        out[_e70 + 3u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e74) * as_type<uint>(10))) + as_type<uint>(_e76));
        middle = 0;
        bool _e81 = choose;
        if (_e81) {
            int _e83 = value;
            middle = as_type<int>(as_type<uint>(_e83) + as_type<uint>(2));
            int _e86 = middle;
            local = as_type<int>(as_type<uint>(_e86) + as_type<uint>(3));
        } else {
            int _e89 = value;
            local = as_type<int>(as_type<uint>(_e89) - as_type<uint>(2));
        }
        int _e92 = local;
        selected = _e92;
        int _e98 = selected;
        int _e100 = middle;
        uint _e94 = base;
        out[_e94 + 4u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e98) * as_type<uint>(10))) + as_type<uint>(_e100));
        int _e103 = value;
        left = _e103;
        int _e106 = value;
        right = as_type<int>(as_type<uint>(_e106) + as_type<uint>(10));
        int _e108 = value;
        consume(_e108);
        bool _e109 = choose;
        if (_e109) {
            int _e111 = left;
            left = as_type<int>(as_type<uint>(_e111) + as_type<uint>(3));
        } else {
            int _e113 = right;
            right = as_type<int>(as_type<uint>(_e113) + as_type<uint>(3));
        }
        int _e120 = left;
        int _e122 = right;
        uint _e116 = base;
        out[_e116 + 5u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e120) * as_type<uint>(100))) + as_type<uint>(_e122));
        int _e125 = value;
        identity = _e125;
        int _e126 = identity;
        identity = as_type<int>(as_type<uint>(_e126) + as_type<uint>(1));
        consume(_e126);
        int _e129 = identity;
        identity = as_type<int>(as_type<uint>(_e129) + as_type<uint>(1));
        int _e136 = identity;
        uint _e133 = base;
        out[_e133 + 6u] = _e136;
        uint _e139 = base;
        position = _e139 + 4u;
        uint _e148 = position;
        position = _e148 + 2u;
        int _e151 = value;
        uint _e141 = position;
        position = _e141 + static_cast<uint>(1);
        uint _e145 = position;
        out[_e145] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e151) * as_type<uint>(2))) + as_type<uint>(2));
        int _e156 = value;
        effect = _e156;
        int _e157 = effect;
        effect = as_type<int>(as_type<uint>(_e157) + as_type<uint>(1));
        consume(_e157);
        int _e160 = effect;
        effect = as_type<int>(as_type<uint>(_e160) + as_type<uint>(1));
        consume(_e160);
        int _e163 = effect;
        effect = as_type<int>(as_type<uint>(_e163) + as_type<uint>(1));
        consume(_e163);
        int _e166 = effect;
        effect = as_type<int>(as_type<uint>(_e166) + as_type<uint>(1));
        consume(_e166);
        int _e173 = effect;
        uint _e170 = base;
        out[_e170 + 8u] = _e173;
        total = 0;
        {
            iteration = 0;
            int _e179 = value;
            current = _e179;
            uint2 loop_bound = uint2(4294967295u);
            bool loop_init = true;
            while(true) {
                if (metal::all(loop_bound == uint2(0u))) { break; }
                loop_bound -= uint2(loop_bound.y == 0u, 1u);
                if (!loop_init) {
                    int _e183 = iteration;
                    iteration = as_type<int>(as_type<uint>(_e183) + as_type<uint>(1));
                    int _e187 = current;
                    current = as_type<int>(as_type<uint>(_e187) + as_type<uint>(2));
                }
                loop_init = false;
                int _e181 = iteration;
                if (_e181 < 4) {
                } else {
                    break;
                }
                int _e190 = iteration;
                if (_e190 != 1) {
                    int _e192 = current;
                    int _e193 = total;
                    total = as_type<int>(as_type<uint>(_e193) + as_type<uint>(_e192));
                } else {
                    continue;
                }
            }
        }
        int _e199 = total;
        uint _e196 = base;
        out[_e196 + 9u] = _e199;
        int _e201 = value;
        ticks = _e201;
        passes = 0;
        {
            int _e205 = value;
            ticks = as_type<int>(as_type<uint>(_e205) + as_type<uint>(1));
            passes = 0;
            uint2 loop_bound_1 = uint2(4294967295u);
            bool loop_init_1 = true;
            while(true) {
                if (metal::all(loop_bound_1 == uint2(0u))) { break; }
                loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
                if (!loop_init_1) {
                    int _e215 = ticks;
                    ticks = as_type<int>(as_type<uint>(_e215) + as_type<uint>(2));
                }
                loop_init_1 = false;
                int _e209 = passes;
                passes = as_type<int>(as_type<uint>(_e209) + as_type<uint>(1));
                int _e212 = passes;
                if (_e212 < 4) {
                } else {
                    break;
                }
            }
        }
        int _e222 = ticks;
        int _e224 = passes;
        uint _e218 = base;
        out[_e218 + 10u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e222) * as_type<uint>(10))) + as_type<uint>(_e224));
        int _e227 = value;
        argument = _e227;
        int _e230 = argument;
        argument = as_type<int>(as_type<uint>(_e230) + as_type<uint>(1));
        int _e232 = argument;
        int _e234 = pair_add(_e232, 2);
        first = _e234;
        int _e236 = first;
        int _e238 = pair_add(_e236, 3);
        second = _e238;
        int _e244 = second;
        int _e246 = argument;
        uint _e240 = base;
        out[_e240 + 11u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e244) * as_type<uint>(10))) + as_type<uint>(_e246));
    }
}
