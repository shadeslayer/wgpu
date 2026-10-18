#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

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
layout(std430) buffer type_5_block_0Compute { int _group_0_binding_0_cs[]; };


int choose(ivec2 value) {
    ivec2 value_1 = ivec2(0);
    value_1 = value;
    int _e4 = value_1.x;
    int _e7 = value_1.y;
    return ((_e4 * 10) + _e7);
}

int choose_1(uint3_ value_2) {
    uint3_ value_3 = uint3_(0u, 0u, 0u);
    value_3 = value_2;
    uint _e3 = value_3.z;
    return int(_e3);
}

int choose_2(vec4 value_4) {
    vec4 value_5 = vec4(0.0);
    value_5 = value_4;
    float _e3 = value_5.w;
    return int(_e3);
}

int3_ bump(int3_ value_6) {
    int3_ value_7 = int3_(0, 0, 0);
    value_7 = value_6;
    int _e4 = value_7.y;
    value_7.y = (_e4 + 1);
    int3_ _e6 = value_7;
    return _e6;
}

float3_ shift(float3_ value_8) {
    float3_ value_9 = float3_(0.0, 0.0, 0.0);
    value_9 = value_8;
    float _e4 = value_9.x;
    value_9.x = (_e4 + 2.0);
    float _e9 = value_9.x;
    float _e10 = value_9.y;
    value_9.z = (_e9 + _e10);
    float3_ _e12 = value_9;
    return _e12;
}

int pair(float3_ first, int second) {
    float3_ first_1 = float3_(0.0, 0.0, 0.0);
    int second_1 = 0;
    first_1 = first;
    second_1 = second;
    float _e5 = first_1.x;
    int _e9 = second_1;
    return ((int(_e5) * 10) + _e9);
}

void main() {
    int seed = 0;
    vec2 f2_ = vec2(0.0);
    float3_ f3_ = float3_(0.0, 0.0, 0.0);
    vec4 f4_ = vec4(0.0);
    ivec2 i2_ = ivec2(0);
    int3_ i3_ = int3_(0, 0, 0);
    ivec4 i4_ = ivec4(0);
    uvec2 u2_ = uvec2(0u);
    uint3_ u3_ = uint3_(0u, 0u, 0u);
    uvec4 u4_ = uvec4(0u);
    float3_ frozen = float3_(0.0, 0.0, 0.0);
    float3_ copied = float3_(0.0, 0.0, 0.0);
    int3_ changed = int3_(0, 0, 0);
    float3_ samples[2] = float3_[2](float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0));
    int index = 0;
    bool choose_first = false;
    float local = 0.0;
    float local_1 = 0.0;
    float3_ local_2 = float3_(0.0, 0.0, 0.0);
    int local_3 = 0;
    int argument = 0;
    int3_ captured = int3_(0, 0, 0);
    int _e4 = _group_0_binding_0_cs[0];
    seed = _e4;
    int _e6 = seed;
    f2_ = vec2(float(_e6), 1.5);
    f3_ = float3_(1.0, 2.0, 3.0);
    f4_ = vec4(float(1), float(2), float(3), float(4));
    int _e26 = seed;
    i2_ = ivec2(_e26, -(2));
    i3_ = int3_(1, 2, 3);
    i4_ = ivec4(1, 2, 3, 4);
    u2_ = uvec2(uint(-(1)), uint(2));
    u3_ = uint3_(uint(1), uint(2), uint(3));
    u4_ = uvec4(uint(1), uint(2), uint(3), uint(4));
    float _e70 = f2_.x;
    float _e71 = f2_.y;
    _group_0_binding_0_cs[0] = int((_e70 + _e71));
    float _e78 = f3_.x;
    float _e79 = f3_.y;
    float _e82 = f3_.z;
    _group_0_binding_0_cs[1] = int(((_e78 + _e79) + _e82));
    float _e88 = f4_.w;
    _group_0_binding_0_cs[2] = int(_e88);
    int _e94 = i2_.x;
    int _e95 = i2_.y;
    _group_0_binding_0_cs[3] = (_e94 + _e95);
    int _e100 = i3_.z;
    _group_0_binding_0_cs[4] = _e100;
    int _e105 = i4_.x;
    int _e106 = i4_.w;
    _group_0_binding_0_cs[5] = (_e105 + _e106);
    uint _e112 = u2_.x;
    _group_0_binding_0_cs[6] = int((_e112 == 4294967295u));
    uint _e119 = u3_.x;
    uint _e120 = u3_.z;
    _group_0_binding_0_cs[7] = int((_e119 + _e120));
    uint _e126 = u4_.w;
    _group_0_binding_0_cs[8] = int(_e126);
    float3_ _e129 = f3_;
    frozen = _e129;
    float3_ _e131 = frozen;
    copied = _e131;
    copied.x = 9.0;
    float _e138 = frozen.x;
    float _e141 = copied.x;
    _group_0_binding_0_cs[9] = int(((_e138 * 10.0) + _e141));
    int3_ _e145 = i3_;
    int3_ _e146 = bump(_e145);
    changed = _e146;
    int _e151 = changed.y;
    int _e154 = i3_.y;
    _group_0_binding_0_cs[10] = ((_e151 * 10) + _e154);
    float3_ _e158 = f3_;
    float3_ _e159 = shift(_e158);
    _group_0_binding_0_cs[11] = int(_e159.z);
    ivec2 _e164 = i2_;
    int _e165 = choose(_e164);
    uint3_ _e166 = u3_;
    int _e167 = choose_1(_e166);
    vec4 _e169 = f4_;
    int _e170 = choose_2(_e169);
    _group_0_binding_0_cs[12] = ((_e165 + _e167) + _e170);
    float3_ _e175 = f3_;
    samples[0] = _e175;
    samples[1] = float3_(float(5), float(6), float(7));
    index = 0;
    int _e194 = index;
    index = (_e194 + 1);
    int _e197 = index;
    int _e189 = index;
    index = (_e189 + 1);
    float _e198 = samples[_e189].x;
    samples[_e189].x = (_e198 + float(_e197));
    float _e201 = samples[_e189].x;
    _group_0_binding_0_cs[13] = int(_e201);
    int _e206 = index;
    float _e211 = samples[0].x;
    float _e219 = samples[1].x;
    _group_0_binding_0_cs[14] = (((_e206 * 100) + (int(_e211) * 10)) + int(_e219));
    choose_first = false;
    bool _e226 = choose_first;
    if (_e226) {
        float _e234 = samples[0].y;
        samples[0].y = (_e234 + float(1));
        local = _e234;
    } else {
        float _e238 = samples[1].y;
        samples[1].y = (_e238 + float(1));
        local = _e238;
    }
    float _e242 = local;
    _group_0_binding_0_cs[15] = int(_e242);
    float _e249 = samples[1].y;
    _group_0_binding_0_cs[16] = int(_e249);
    float3_ _e258 = f3_;
    bool _e253 = choose_first;
    if (_e253) {
        samples[0] = _e258;
        float _e262 = samples[0].z;
        local_1 = _e262;
    } else {
        samples[1] = _e258;
        float _e263 = samples[1].z;
        local_1 = _e263;
    }
    float _e264 = local_1;
    _group_0_binding_0_cs[17] = int(_e264);
    if (true) {
        local_2 = float3_(float(7), float(8), float(9));
    } else {
        float3_ _e277 = f3_;
        local_2 = _e277;
    }
    float3_ _e278 = local_2;
    _group_0_binding_0_cs[18] = int(_e278.y);
    {
        int _e283 = seed;
        local_3 = (_e283 + 2);
        int _e287 = local_3;
        _group_0_binding_0_cs[19] = _e287;
    }
    argument = 3;
    int _e291 = argument;
    int _e292 = argument;
    argument = (_e292 + 1);
    int _e295 = argument;
    captured = int3_(_e291, _e292, _e295);
    int _e301 = captured.x;
    int _e305 = captured.y;
    int _e309 = captured.z;
    _group_0_binding_0_cs[20] = (((_e301 * 100) + (_e305 * 10)) + _e309);
    int _e313 = argument;
    _group_0_binding_0_cs[21] = _e313;
    float3_ _e316 = f3_;
    f3_.x = 9.0;
    int _e320 = pair(_e316, 4);
    _group_0_binding_0_cs[22] = _e320;
    float _e324 = f3_.x;
    _group_0_binding_0_cs[23] = int(_e324);
}

