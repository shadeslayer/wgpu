// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

struct uint3_ {
    uint x;
    uint y;
    uint z;
};
struct int3_ {
    int x;
    int y;
    int z;
};
struct float3_ {
    float x;
    float y;
    float z;
};
typedef int type_5[1];
struct type_10 {
    float3_ inner[2];
};

int choose(
    metal::int2 value
) {
    metal::int2 value_1 = {};
    value_1 = value;
    int _e4 = value_1.x;
    int _e7 = value_1.y;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e4) * as_type<uint>(10))) + as_type<uint>(_e7));
}

int choose_1(
    uint3_ value_2
) {
    uint3_ value_3 = {};
    value_3 = value_2;
    uint _e3 = value_3.z;
    return static_cast<int>(_e3);
}

int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

int choose_2(
    metal::float4 value_4
) {
    metal::float4 value_5 = {};
    value_5 = value_4;
    float _e3 = value_5.w;
    return naga_f2i32(_e3);
}

int3_ bump(
    int3_ value_6
) {
    int3_ value_7 = {};
    value_7 = value_6;
    int _e4 = value_7.y;
    value_7.y = as_type<int>(as_type<uint>(_e4) + as_type<uint>(1));
    int3_ _e6 = value_7;
    return _e6;
}

float3_ shift(
    float3_ value_8
) {
    float3_ value_9 = {};
    value_9 = value_8;
    float _e4 = value_9.x;
    value_9.x = _e4 + 2.0;
    float _e9 = value_9.x;
    float _e10 = value_9.y;
    value_9.z = _e9 + _e10;
    float3_ _e12 = value_9;
    return _e12;
}

int pair(
    float3_ first,
    int second
) {
    float3_ first_1 = {};
    int second_1 = {};
    first_1 = first;
    second_1 = second;
    float _e5 = first_1.x;
    int _e9 = second_1;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(naga_f2i32(_e5)) * as_type<uint>(10))) + as_type<uint>(_e9));
}
int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}


kernel void vector_values(
  device type_5& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int seed = {};
    metal::float2 f2_ = {};
    float3_ f3_ = {};
    metal::float4 f4_ = {};
    metal::int2 i2_ = {};
    int3_ i3_ = {};
    metal::int4 i4_ = {};
    metal::uint2 u2_ = {};
    uint3_ u3_ = {};
    metal::uint4 u4_ = {};
    float3_ frozen = {};
    float3_ copied = {};
    int3_ changed = {};
    type_10 samples = {};
    int index = {};
    bool choose_first = {};
    float local = {};
    float local_1 = {};
    float3_ local_2 = {};
    int local_3 = {};
    int argument = {};
    int3_ captured = {};
    int _e4 = out[0];
    seed = _e4;
    int _e6 = seed;
    f2_ = metal::float2(static_cast<float>(_e6), 1.5);
    f3_ = float3_ {1.0, 2.0, 3.0};
    f4_ = metal::float4(static_cast<float>(1), static_cast<float>(2), static_cast<float>(3), static_cast<float>(4));
    int _e26 = seed;
    i2_ = metal::int2(_e26, naga_neg(2));
    i3_ = int3_ {1, 2, 3};
    i4_ = metal::int4(1, 2, 3, 4);
    u2_ = metal::uint2(static_cast<uint>(naga_neg(1)), static_cast<uint>(2));
    u3_ = uint3_ {static_cast<uint>(1), static_cast<uint>(2), static_cast<uint>(3)};
    u4_ = metal::uint4(static_cast<uint>(1), static_cast<uint>(2), static_cast<uint>(3), static_cast<uint>(4));
    float _e70 = f2_.x;
    float _e71 = f2_.y;
    out[0] = naga_f2i32(_e70 + _e71);
    float _e78 = f3_.x;
    float _e79 = f3_.y;
    float _e82 = f3_.z;
    out[1] = naga_f2i32((_e78 + _e79) + _e82);
    float _e88 = f4_.w;
    out[2] = naga_f2i32(_e88);
    int _e94 = i2_.x;
    int _e95 = i2_.y;
    out[3] = as_type<int>(as_type<uint>(_e94) + as_type<uint>(_e95));
    int _e100 = i3_.z;
    out[4] = _e100;
    int _e105 = i4_.x;
    int _e106 = i4_.w;
    out[5] = as_type<int>(as_type<uint>(_e105) + as_type<uint>(_e106));
    uint _e112 = u2_.x;
    out[6] = static_cast<int>(_e112 == 4294967295u);
    uint _e119 = u3_.x;
    uint _e120 = u3_.z;
    out[7] = static_cast<int>(_e119 + _e120);
    uint _e126 = u4_.w;
    out[8] = static_cast<int>(_e126);
    float3_ _e129 = f3_;
    frozen = _e129;
    float3_ _e131 = frozen;
    copied = _e131;
    copied.x = 9.0;
    float _e138 = frozen.x;
    float _e141 = copied.x;
    out[9] = naga_f2i32((_e138 * 10.0) + _e141);
    int3_ _e145 = i3_;
    int3_ _e146 = bump(_e145);
    changed = _e146;
    int _e151 = changed.y;
    int _e154 = i3_.y;
    out[10] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e151) * as_type<uint>(10))) + as_type<uint>(_e154));
    float3_ _e158 = f3_;
    float3_ _e159 = shift(_e158);
    out[11] = naga_f2i32(_e159.z);
    metal::int2 _e164 = i2_;
    int _e165 = choose(_e164);
    uint3_ _e166 = u3_;
    int _e167 = choose_1(_e166);
    metal::float4 _e169 = f4_;
    int _e170 = choose_2(_e169);
    out[12] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e165) + as_type<uint>(_e167))) + as_type<uint>(_e170));
    float3_ _e175 = f3_;
    samples.inner[0] = _e175;
    samples.inner[1] = float3_ {static_cast<float>(5), static_cast<float>(6), static_cast<float>(7)};
    index = 0;
    int _e194 = index;
    index = as_type<int>(as_type<uint>(_e194) + as_type<uint>(1));
    int _e197 = index;
    int _e189 = index;
    index = as_type<int>(as_type<uint>(_e189) + as_type<uint>(1));
    float _e198 = samples.inner[_e189].x;
    samples.inner[_e189].x = _e198 + static_cast<float>(_e197);
    float _e201 = samples.inner[_e189].x;
    out[13] = naga_f2i32(_e201);
    int _e206 = index;
    float _e211 = samples.inner[0].x;
    float _e219 = samples.inner[1].x;
    out[14] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e206) * as_type<uint>(100))) + as_type<uint>(as_type<int>(as_type<uint>(naga_f2i32(_e211)) * as_type<uint>(10))))) + as_type<uint>(naga_f2i32(_e219)));
    choose_first = false;
    bool _e226 = choose_first;
    if (_e226) {
        float _e234 = samples.inner[0].y;
        samples.inner[0].y = _e234 + static_cast<float>(1);
        local = _e234;
    } else {
        float _e238 = samples.inner[1].y;
        samples.inner[1].y = _e238 + static_cast<float>(1);
        local = _e238;
    }
    float _e242 = local;
    out[15] = naga_f2i32(_e242);
    float _e249 = samples.inner[1].y;
    out[16] = naga_f2i32(_e249);
    float3_ _e258 = f3_;
    bool _e253 = choose_first;
    if (_e253) {
        samples.inner[0] = _e258;
        float _e262 = samples.inner[0].z;
        local_1 = _e262;
    } else {
        samples.inner[1] = _e258;
        float _e263 = samples.inner[1].z;
        local_1 = _e263;
    }
    float _e264 = local_1;
    out[17] = naga_f2i32(_e264);
    if (true) {
        local_2 = float3_ {static_cast<float>(7), static_cast<float>(8), static_cast<float>(9)};
    } else {
        float3_ _e277 = f3_;
        local_2 = _e277;
    }
    float3_ _e278 = local_2;
    out[18] = naga_f2i32(_e278.y);
    {
        int _e283 = seed;
        local_3 = as_type<int>(as_type<uint>(_e283) + as_type<uint>(2));
        int _e287 = local_3;
        out[19] = _e287;
    }
    argument = 3;
    int _e291 = argument;
    int _e292 = argument;
    argument = as_type<int>(as_type<uint>(_e292) + as_type<uint>(1));
    int _e295 = argument;
    captured = int3_ {_e291, _e292, _e295};
    int _e301 = captured.x;
    int _e305 = captured.y;
    int _e309 = captured.z;
    out[20] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e301) * as_type<uint>(100))) + as_type<uint>(as_type<int>(as_type<uint>(_e305) * as_type<uint>(10))))) + as_type<uint>(_e309));
    int _e313 = argument;
    out[21] = _e313;
    float3_ _e316 = f3_;
    f3_.x = 9.0;
    int _e320 = pair(_e316, 4);
    out[22] = _e320;
    float _e324 = f3_.x;
    out[23] = naga_f2i32(_e324);
}
