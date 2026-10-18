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

bool classify(
    bool value_1
) {
    bool value_2 = {};
    value_2 = value_1;
    bool _e2 = value_2;
    return !(_e2);
}

int classify_1(
    int value_3
) {
    int value_4 = {};
    value_4 = value_3;
    int _e3 = value_4;
    return as_type<int>(as_type<uint>(_e3) + as_type<uint>(10));
}

uint classify_2(
    uint value_5
) {
    uint value_6 = {};
    value_6 = value_5;
    uint _e3 = value_6;
    return _e3 + 20u;
}

float classify_3(
    float value_7
) {
    float value_8 = {};
    value_8 = value_7;
    float _e3 = value_8;
    return (_e3 * 1.5) + 0.5;
}

int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

int rank(
    int left,
    float right
) {
    int left_1 = {};
    float right_1 = {};
    left_1 = left;
    right_1 = right;
    int _e5 = left_1;
    float _e7 = right_1;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e5) * as_type<uint>(10))) + as_type<uint>(naga_f2i32(_e7)));
}

int rank_1(
    float left_2,
    int right_2
) {
    float left_3 = {};
    int right_3 = {};
    left_3 = left_2;
    right_3 = right_2;
    float _e5 = left_3;
    int _e10 = right_3;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(100) + as_type<uint>(as_type<int>(as_type<uint>(naga_f2i32(_e5)) * as_type<uint>(10))))) + as_type<uint>(_e10));
}

int pair(
    float left_4,
    int right_4
) {
    float left_5 = {};
    int right_5 = {};
    left_5 = left_4;
    right_5 = right_4;
    float _e4 = left_5;
    int _e8 = right_5;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(naga_f2i32(_e4)) * as_type<uint>(10))) + as_type<uint>(_e8));
}

int promote(
    uint left_6,
    float right_6
) {
    uint left_7 = {};
    float right_7 = {};
    left_7 = left_6;
    right_7 = right_6;
    uint _e5 = left_7;
    float _e8 = right_7;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(200) + as_type<uint>(static_cast<int>(_e5)))) + as_type<uint>(naga_f2i32(_e8)));
}

void touch(
    bool stop,
    int value_9
) {
    bool stop_1 = {};
    int value_10 = {};
    stop_1 = stop;
    value_10 = value_9;
    bool _e4 = stop_1;
    if (_e4) {
        return;
    }
    int _e5 = value_10;
    int _e6 = classify_1(_e5);
    value_10 = _e6;
    return;
}

int touch_1(
    int value_11
) {
    int value_12 = {};
    value_12 = value_11;
    int _e2 = value_12;
    int _e3 = classify_1(_e2);
    return as_type<int>(as_type<uint>(_e3) + as_type<uint>(3));
}

int stage(
    int value_13
) {
    int value_14 = {};
    value_14 = value_13;
    int _e3 = value_14;
    return as_type<int>(as_type<uint>(_e3) + as_type<uint>(2));
}

int frozen(
    float value_15
) {
    float value_16 = {};
    value_16 = value_15;
    float _e2 = value_16;
    int _e4 = stage(naga_f2i32(_e2));
    return _e4;
}

int stage_1(
    float value_17
) {
    float value_18 = {};
    value_18 = value_17;
    float _e2 = value_18;
    return as_type<int>(as_type<uint>(naga_f2i32(_e2)) + as_type<uint>(100));
}
uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}


struct scalar_overloadsInput {
};
kernel void scalar_overloads(
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
    bool choose = {};
    int effect = {};
    int nested = {};
    int argument = {};
    int captured = {};
    uint _e4 = n_1.value;
    n = _e4;
    i = (blockIdx.x * 64u) + threadIdx.x;
    uint _e14 = i;
    uint _e15 = n;
    if (_e14 < _e15) {
        uint _e19 = i;
        base = _e19 * 13u;
        uint _e22 = i;
        int _e24 = input[_e22];
        value = _e24;
        uint _e27 = i;
        choose = naga_mod(_e27, 2u) == 0u;
        bool _e33 = choose;
        bool _e34 = classify(_e33);
        uint _e31 = base;
        out[_e31] = static_cast<int>(_e34);
        int _e40 = value;
        int _e41 = classify_1(_e40);
        uint _e37 = base;
        out[_e37 + 1u] = _e41;
        int _e47 = value;
        uint _e50 = classify_2(static_cast<uint>(as_type<int>(as_type<uint>(_e47) + as_type<uint>(16))));
        uint _e43 = base;
        out[_e43 + 2u] = static_cast<int>(_e50);
        int _e56 = value;
        float _e58 = classify_3(static_cast<float>(_e56));
        uint _e53 = base;
        out[_e53 + 3u] = naga_f2i32(_e58);
        bool _e64 = choose;
        int _e65 = value;
        int _e70 = rank(static_cast<int>(_e64), static_cast<float>(_e65) + 0.5);
        uint _e61 = base;
        out[_e61 + 4u] = _e70;
        int _e75 = value;
        bool _e79 = choose;
        int _e81 = rank_1(static_cast<float>(_e75) + 0.5, static_cast<int>(_e79));
        uint _e72 = base;
        out[_e72 + 5u] = _e81;
        int _e87 = value;
        int _e90 = value;
        int _e92 = promote(static_cast<uint>(as_type<int>(as_type<uint>(_e87) + as_type<uint>(16))), static_cast<float>(_e90));
        uint _e83 = base;
        out[_e83 + 6u] = _e92;
        int _e94 = value;
        effect = _e94;
        bool _e95 = choose;
        int _e96 = effect;
        effect = as_type<int>(as_type<uint>(_e96) + as_type<uint>(1));
        touch(_e95, _e96);
        int _e103 = effect;
        uint _e100 = base;
        out[_e100 + 7u] = _e103;
        int _e108 = effect;
        int _e109 = touch_1(_e108);
        uint _e105 = base;
        out[_e105 + 8u] = _e109;
        int _e114 = value;
        int _e118 = frozen(static_cast<float>(_e114) + 0.75);
        uint _e111 = base;
        out[_e111 + 9u] = _e118;
        int _e123 = value;
        int _e127 = stage_1(static_cast<float>(_e123) + 0.75);
        uint _e120 = base;
        out[_e120 + 10u] = _e127;
        int _e129 = effect;
        effect = as_type<int>(as_type<uint>(_e129) + as_type<uint>(1));
        int _e132 = classify_1(_e129);
        int _e133 = touch_1(_e132);
        nested = _e133;
        int _e139 = nested;
        int _e141 = effect;
        uint _e135 = base;
        out[_e135 + 11u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e139) * as_type<uint>(10))) + as_type<uint>(_e141));
        int _e145 = value;
        argument = as_type<int>(as_type<uint>(_e145) + as_type<uint>(8));
        int _e148 = argument;
        int _e149 = argument;
        argument = as_type<int>(as_type<uint>(_e149) + as_type<uint>(1));
        int _e153 = pair(static_cast<float>(_e148), _e149);
        captured = _e153;
        int _e159 = captured;
        int _e161 = argument;
        uint _e155 = base;
        out[_e155 + 12u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e159) * as_type<uint>(100))) + as_type<uint>(_e161));
    }
}
