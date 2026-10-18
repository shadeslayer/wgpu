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

int doubled(
    int value_3
) {
    int value_4 = {};
    value_4 = value_3;
    int _e2 = value_4;
    int _e3 = increment(_e2);
    return as_type<int>(as_type<uint>(_e3) * as_type<uint>(2));
}

uint advance(
    uint value_5
) {
    uint value_6 = {};
    value_6 = value_5;
    uint _e3 = value_6;
    return _e3 + 3u;
}

uint retreat(
    uint value_7
) {
    uint value_8 = {};
    value_8 = value_7;
    uint _e3 = value_8;
    return _e3 - 1u;
}

float scale(
    float value_9,
    int bias
) {
    float value_10 = {};
    int bias_1 = {};
    value_10 = value_9;
    bias_1 = bias;
    float _e5 = value_10;
    int _e7 = bias_1;
    return (_e5 * 1.5) + static_cast<float>(_e7);
}

bool same(
    int left,
    int right
) {
    int left_1 = {};
    int right_1 = {};
    left_1 = left;
    right_1 = right;
    int _e4 = left_1;
    int _e5 = right_1;
    return _e4 == _e5;
}

void consume(
    bool stop,
    int value_11
) {
    bool stop_1 = {};
    int value_12 = {};
    stop_1 = stop;
    value_12 = value_11;
    bool _e4 = stop_1;
    if (_e4) {
        return;
    }
    int _e5 = value_12;
    int _e6 = doubled(_e5);
    value_12 = _e6;
    return;
}

void forward(
    bool stop_2,
    int value_13
) {
    bool stop_3 = {};
    int value_14 = {};
    stop_3 = stop_2;
    value_14 = value_13;
    bool _e4 = stop_3;
    int _e5 = value_14;
    int _e6 = increment(_e5);
    consume(_e4, _e6);
    return;
}
int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}


struct inline_functionsInput {
};
kernel void inline_functions(
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
        int _e27 = value;
        int _e28 = increment(_e27);
        uint _e25 = base;
        out[_e25] = _e28;
        int _e33 = value;
        uint _e30 = base;
        out[_e30 + 1u] = _e33;
        int _e38 = value;
        int _e39 = doubled(_e38);
        uint _e35 = base;
        out[_e35 + 2u] = _e39;
        int _e44 = value;
        uint _e46 = advance(static_cast<uint>(_e44));
        uint _e41 = base;
        out[_e41 + 3u] = static_cast<int>(_e46);
        int _e52 = value;
        uint _e54 = retreat(static_cast<uint>(_e52));
        uint _e49 = base;
        out[_e49 + 4u] = static_cast<int>(_e54);
        int _e60 = value;
        float _e63 = scale(static_cast<float>(_e60), 2);
        uint _e57 = base;
        out[_e57 + 5u] = naga_f2i32(_e63);
        int _e69 = value;
        uint _e70 = i;
        int _e72 = input[_e70];
        bool _e73 = same(_e69, _e72);
        uint _e66 = base;
        out[_e66 + 6u] = static_cast<int>(_e73);
        uint _e76 = i;
        int _e80 = value;
        value = as_type<int>(as_type<uint>(_e80) + as_type<uint>(1));
        consume(naga_mod(_e76, 2u) == 0u, _e80);
        int _e87 = value;
        uint _e84 = base;
        out[_e84 + 7u] = _e87;
        uint _e89 = i;
        int _e93 = value;
        value = as_type<int>(as_type<uint>(_e93) + as_type<uint>(1));
        forward(naga_mod(_e89, 2u) != 0u, _e93);
        int _e100 = value;
        uint _e97 = base;
        out[_e97 + 8u] = _e100;
        int _e105 = value;
        uint _e106 = i;
        int _e108 = input[_e106];
        bool _e109 = same(_e105, _e108);
        uint _e102 = base;
        out[_e102 + 9u] = static_cast<int>(_e109);
        int _e115 = value;
        int _e116 = increment(_e115);
        int _e117 = increment(_e116);
        uint _e112 = base;
        out[_e112 + 10u] = _e117;
        int _e122 = value;
        uint _e119 = base;
        out[_e119 + 11u] = _e122;
    }
}
