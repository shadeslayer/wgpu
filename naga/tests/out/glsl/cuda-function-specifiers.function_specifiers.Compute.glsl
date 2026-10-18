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


int increment(int value_1) {
    int value_2 = 0;
    value_2 = value_1;
    int _e3 = value_2;
    value_2 = (_e3 + 2);
    int _e5 = value_2;
    return _e5;
}

int twice(int value_3) {
    int value_4 = 0;
    value_4 = value_3;
    int _e2 = value_4;
    int _e3 = increment(_e2);
    return (_e3 * 2);
}

int advance(int value_5) {
    int value_6 = 0;
    value_6 = value_5;
    int _e2 = value_6;
    int _e3 = twice(_e2);
    return (_e3 + 1);
}

int retreat(int value_7) {
    int value_8 = 0;
    value_8 = value_7;
    int _e3 = value_8;
    return (_e3 - 3);
}

float scaled(float value_9) {
    float value_10 = 0.0;
    value_10 = value_9;
    float _e3 = value_10;
    return ((_e3 * 1.5) + 0.5);
}

bool differs(int value_11, int other) {
    int value_12 = 0;
    int other_1 = 0;
    value_12 = value_11;
    other_1 = other;
    int _e4 = value_12;
    int _e5 = other_1;
    return (_e4 != _e5);
}

void consume(bool stop, int value_13) {
    bool stop_1 = false;
    int value_14 = 0;
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

void forward(bool stop_2, int value_15) {
    bool stop_3 = false;
    int value_16 = 0;
    stop_3 = stop_2;
    value_16 = value_15;
    bool _e4 = stop_3;
    int _e5 = value_16;
    int _e6 = retreat(_e5);
    consume(_e4, _e6);
    return;
}

uint plus_one(uint value_17) {
    uint value_18 = 0u;
    value_18 = value_17;
    uint _e3 = value_18;
    return (_e3 + 1u);
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
    uint _e4 = _group_0_binding_2_cs.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e14 = i;
    uint _e15 = n;
    if ((_e14 < _e15)) {
        uint _e19 = i;
        base = (_e19 * 12u);
        uint _e22 = i;
        int _e24 = _group_0_binding_1_cs[_e22];
        value = _e24;
        uint _e27 = i;
        choose = ((_e27 - 2u * (_e27 / 2u)) == 0u);
        int _e33 = value;
        int _e34 = increment(_e33);
        uint _e31 = base;
        _group_0_binding_0_cs[_e31] = _e34;
        int _e39 = value;
        int _e40 = twice(_e39);
        uint _e36 = base;
        _group_0_binding_0_cs[(_e36 + 1u)] = _e40;
        int _e45 = value;
        int _e46 = advance(_e45);
        uint _e42 = base;
        _group_0_binding_0_cs[(_e42 + 2u)] = _e46;
        int _e51 = value;
        int _e52 = retreat(_e51);
        uint _e48 = base;
        _group_0_binding_0_cs[(_e48 + 3u)] = _e52;
        int _e57 = value;
        float _e59 = scaled(float(_e57));
        uint _e54 = base;
        _group_0_binding_0_cs[(_e54 + 4u)] = int(_e59);
        int _e65 = value;
        uint _e66 = i;
        int _e68 = _group_0_binding_1_cs[_e66];
        bool _e69 = differs(_e65, _e68);
        uint _e62 = base;
        _group_0_binding_0_cs[(_e62 + 5u)] = int(_e69);
        int _e72 = value;
        effect = _e72;
        bool _e73 = choose;
        int _e74 = effect;
        effect = (_e74 + 1);
        consume(_e73, _e74);
        int _e81 = effect;
        uint _e78 = base;
        _group_0_binding_0_cs[(_e78 + 6u)] = _e81;
        bool _e82 = choose;
        int _e84 = effect;
        effect = (_e84 + 1);
        forward(!(_e82), _e84);
        int _e91 = effect;
        uint _e88 = base;
        _group_0_binding_0_cs[(_e88 + 7u)] = _e91;
        int _e96 = effect;
        int _e97 = advance(_e96);
        uint _e93 = base;
        _group_0_binding_0_cs[(_e93 + 8u)] = _e97;
        int _e103 = effect;
        uint _e106 = plus_one(uint((_e103 + 16)));
        uint _e99 = base;
        _group_0_binding_0_cs[(_e99 + 9u)] = int(_e106);
        int _e112 = effect;
        uint _e113 = i;
        int _e115 = _group_0_binding_1_cs[_e113];
        bool _e116 = differs(_e112, _e115);
        uint _e109 = base;
        _group_0_binding_0_cs[(_e109 + 10u)] = int(_e116);
        int _e122 = effect;
        uint _e119 = base;
        _group_0_binding_0_cs[(_e119 + 11u)] = _e122;
    }
}

