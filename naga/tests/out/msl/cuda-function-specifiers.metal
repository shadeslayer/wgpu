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

int increment(
    int value_1
) {
    int value_2 = {};
    value_2 = value_1;
    int _e3 = value_2;
    value_2 = as_type<int>(as_type<uint>(_e3) + as_type<uint>(2));
    int _e5 = value_2;
    return _e5;
}

int twice(
    int value_3
) {
    int value_4 = {};
    value_4 = value_3;
    int _e2 = value_4;
    int _e3 = increment(_e2);
    return as_type<int>(as_type<uint>(_e3) * as_type<uint>(2));
}

int advance(
    int value_5
) {
    int value_6 = {};
    value_6 = value_5;
    int _e2 = value_6;
    int _e3 = twice(_e2);
    return as_type<int>(as_type<uint>(_e3) + as_type<uint>(1));
}

int retreat(
    int value_7
) {
    int value_8 = {};
    value_8 = value_7;
    int _e3 = value_8;
    return as_type<int>(as_type<uint>(_e3) - as_type<uint>(3));
}

float scaled(
    float value_9
) {
    float value_10 = {};
    value_10 = value_9;
    float _e3 = value_10;
    return (_e3 * 1.5) + 0.5;
}

bool differs(
    int value_11,
    int other
) {
    int value_12 = {};
    int other_1 = {};
    value_12 = value_11;
    other_1 = other;
    int _e4 = value_12;
    int _e5 = other_1;
    return _e4 != _e5;
}

void consume(
    bool stop,
    int value_13
) {
    bool stop_1 = {};
    int value_14 = {};
    stop_1 = stop;
    value_14 = value_13;
    bool _e4 = stop_1;
    if (_e4) {
        return;
    }
    int _e5 = value_14;
    int _e6 = advance(_e5);
    value_14 = _e6;
    return;
}

void forward(
    bool stop_2,
    int value_15
) {
    bool stop_3 = {};
    int value_16 = {};
    stop_3 = stop_2;
    value_16 = value_15;
    bool _e4 = stop_3;
    int _e5 = value_16;
    int _e6 = retreat(_e5);
    consume(_e4, _e6);
    return;
}

uint plus_one(
    uint value_17
) {
    uint value_18 = {};
    value_18 = value_17;
    uint _e3 = value_18;
    return _e3 + 1u;
}
uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}

int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}


struct function_specifiersInput {
};
kernel void function_specifiers(
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
        int _e33 = value;
        int _e34 = increment(_e33);
        uint _e31 = base;
        out[_e31] = _e34;
        int _e39 = value;
        int _e40 = twice(_e39);
        uint _e36 = base;
        out[_e36 + 1u] = _e40;
        int _e45 = value;
        int _e46 = advance(_e45);
        uint _e42 = base;
        out[_e42 + 2u] = _e46;
        int _e51 = value;
        int _e52 = retreat(_e51);
        uint _e48 = base;
        out[_e48 + 3u] = _e52;
        int _e57 = value;
        float _e59 = scaled(static_cast<float>(_e57));
        uint _e54 = base;
        out[_e54 + 4u] = naga_f2i32(_e59);
        int _e65 = value;
        uint _e66 = i;
        int _e68 = input[_e66];
        bool _e69 = differs(_e65, _e68);
        uint _e62 = base;
        out[_e62 + 5u] = static_cast<int>(_e69);
        int _e72 = value;
        effect = _e72;
        bool _e73 = choose;
        int _e74 = effect;
        effect = as_type<int>(as_type<uint>(_e74) + as_type<uint>(1));
        consume(_e73, _e74);
        int _e81 = effect;
        uint _e78 = base;
        out[_e78 + 6u] = _e81;
        bool _e82 = choose;
        int _e84 = effect;
        effect = as_type<int>(as_type<uint>(_e84) + as_type<uint>(1));
        forward(!(_e82), _e84);
        int _e91 = effect;
        uint _e88 = base;
        out[_e88 + 7u] = _e91;
        int _e96 = effect;
        int _e97 = advance(_e96);
        uint _e93 = base;
        out[_e93 + 8u] = _e97;
        int _e103 = effect;
        uint _e106 = plus_one(static_cast<uint>(as_type<int>(as_type<uint>(_e103) + as_type<uint>(16))));
        uint _e99 = base;
        out[_e99 + 9u] = static_cast<int>(_e106);
        int _e112 = effect;
        uint _e113 = i;
        int _e115 = input[_e113];
        bool _e116 = differs(_e112, _e115);
        uint _e109 = base;
        out[_e109 + 10u] = static_cast<int>(_e116);
        int _e122 = effect;
        uint _e119 = base;
        out[_e119 + 11u] = _e122;
    }
}
