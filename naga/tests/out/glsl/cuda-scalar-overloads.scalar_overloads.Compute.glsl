#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct type_5 {
    uint value;
};
layout(std430) buffer type_4_block_0Compute { int _group_0_binding_0_cs[]; };

layout(std430) readonly buffer type_4_block_1Compute { int _group_0_binding_1_cs[]; };

layout(std140) uniform type_5_block_2Compute { type_5 _group_0_binding_2_cs; };


bool classify(bool value_1) {
    bool value_2 = false;
    value_2 = value_1;
    bool _e2 = value_2;
    return !(_e2);
}

int classify_1(int value_3) {
    int value_4 = 0;
    value_4 = value_3;
    int _e3 = value_4;
    return (_e3 + 10);
}

uint classify_2(uint value_5) {
    uint value_6 = 0u;
    value_6 = value_5;
    uint _e3 = value_6;
    return (_e3 + 20u);
}

float classify_3(float value_7) {
    float value_8 = 0.0;
    value_8 = value_7;
    float _e3 = value_8;
    return ((_e3 * 1.5) + 0.5);
}

int rank(int left, float right) {
    int left_1 = 0;
    float right_1 = 0.0;
    left_1 = left;
    right_1 = right;
    int _e5 = left_1;
    float _e7 = right_1;
    return ((_e5 * 10) + int(_e7));
}

int rank_1(float left_2, int right_2) {
    float left_3 = 0.0;
    int right_3 = 0;
    left_3 = left_2;
    right_3 = right_2;
    float _e5 = left_3;
    int _e10 = right_3;
    return ((100 + (int(_e5) * 10)) + _e10);
}

int pair(float left_4, int right_4) {
    float left_5 = 0.0;
    int right_5 = 0;
    left_5 = left_4;
    right_5 = right_4;
    float _e4 = left_5;
    int _e8 = right_5;
    return ((int(_e4) * 10) + _e8);
}

int promote(uint left_6, float right_6) {
    uint left_7 = 0u;
    float right_7 = 0.0;
    left_7 = left_6;
    right_7 = right_6;
    uint _e5 = left_7;
    float _e8 = right_7;
    return ((200 + int(_e5)) + int(_e8));
}

void touch(bool stop, int value_9) {
    bool stop_1 = false;
    int value_10 = 0;
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

int touch_1(int value_11) {
    int value_12 = 0;
    value_12 = value_11;
    int _e2 = value_12;
    int _e3 = classify_1(_e2);
    return (_e3 + 3);
}

int stage(int value_13) {
    int value_14 = 0;
    value_14 = value_13;
    int _e3 = value_14;
    return (_e3 + 2);
}

int frozen(float value_15) {
    float value_16 = 0.0;
    value_16 = value_15;
    float _e2 = value_16;
    int _e4 = stage(int(_e2));
    return _e4;
}

int stage_1(float value_17) {
    float value_18 = 0.0;
    value_18 = value_17;
    float _e2 = value_18;
    return (int(_e2) + 100);
}

void main() {
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint base = 0u;
    int value = 0;
    bool choose = false;
    int effect = 0;
    int nested = 0;
    int argument = 0;
    int captured = 0;
    uint _e4 = _group_0_binding_2_cs.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e14 = i;
    uint _e15 = n;
    if ((_e14 < _e15)) {
        uint _e19 = i;
        base = (_e19 * 13u);
        uint _e22 = i;
        int _e24 = _group_0_binding_1_cs[_e22];
        value = _e24;
        uint _e27 = i;
        choose = ((_e27 - 2u * (_e27 / 2u)) == 0u);
        bool _e33 = choose;
        bool _e34 = classify(_e33);
        uint _e31 = base;
        _group_0_binding_0_cs[_e31] = int(_e34);
        int _e40 = value;
        int _e41 = classify_1(_e40);
        uint _e37 = base;
        _group_0_binding_0_cs[(_e37 + 1u)] = _e41;
        int _e47 = value;
        uint _e50 = classify_2(uint((_e47 + 16)));
        uint _e43 = base;
        _group_0_binding_0_cs[(_e43 + 2u)] = int(_e50);
        int _e56 = value;
        float _e58 = classify_3(float(_e56));
        uint _e53 = base;
        _group_0_binding_0_cs[(_e53 + 3u)] = int(_e58);
        bool _e64 = choose;
        int _e65 = value;
        int _e70 = rank(int(_e64), (float(_e65) + 0.5));
        uint _e61 = base;
        _group_0_binding_0_cs[(_e61 + 4u)] = _e70;
        int _e75 = value;
        bool _e79 = choose;
        int _e81 = rank_1((float(_e75) + 0.5), int(_e79));
        uint _e72 = base;
        _group_0_binding_0_cs[(_e72 + 5u)] = _e81;
        int _e87 = value;
        int _e90 = value;
        int _e92 = promote(uint((_e87 + 16)), float(_e90));
        uint _e83 = base;
        _group_0_binding_0_cs[(_e83 + 6u)] = _e92;
        int _e94 = value;
        effect = _e94;
        bool _e95 = choose;
        int _e96 = effect;
        effect = (_e96 + 1);
        touch(_e95, _e96);
        int _e103 = effect;
        uint _e100 = base;
        _group_0_binding_0_cs[(_e100 + 7u)] = _e103;
        int _e108 = effect;
        int _e109 = touch_1(_e108);
        uint _e105 = base;
        _group_0_binding_0_cs[(_e105 + 8u)] = _e109;
        int _e114 = value;
        int _e118 = frozen((float(_e114) + 0.75));
        uint _e111 = base;
        _group_0_binding_0_cs[(_e111 + 9u)] = _e118;
        int _e123 = value;
        int _e127 = stage_1((float(_e123) + 0.75));
        uint _e120 = base;
        _group_0_binding_0_cs[(_e120 + 10u)] = _e127;
        int _e129 = effect;
        effect = (_e129 + 1);
        int _e132 = classify_1(_e129);
        int _e133 = touch_1(_e132);
        nested = _e133;
        int _e139 = nested;
        int _e141 = effect;
        uint _e135 = base;
        _group_0_binding_0_cs[(_e135 + 11u)] = ((_e139 * 10) + _e141);
        int _e145 = value;
        argument = (_e145 + 8);
        int _e148 = argument;
        int _e149 = argument;
        argument = (_e149 + 1);
        int _e153 = pair(float(_e148), _e149);
        captured = _e153;
        int _e159 = captured;
        int _e161 = argument;
        uint _e155 = base;
        _group_0_binding_0_cs[(_e155 + 12u)] = ((_e159 * 100) + _e161);
    }
}

