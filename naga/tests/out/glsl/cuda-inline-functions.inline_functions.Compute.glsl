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

int doubled(int value_3) {
    int value_4 = 0;
    value_4 = value_3;
    int _e2 = value_4;
    int _e3 = increment(_e2);
    return (_e3 * 2);
}

uint advance(uint value_5) {
    uint value_6 = 0u;
    value_6 = value_5;
    uint _e3 = value_6;
    return (_e3 + 3u);
}

uint retreat(uint value_7) {
    uint value_8 = 0u;
    value_8 = value_7;
    uint _e3 = value_8;
    return (_e3 - 1u);
}

float scale(float value_9, int bias) {
    float value_10 = 0.0;
    int bias_1 = 0;
    value_10 = value_9;
    bias_1 = bias;
    float _e5 = value_10;
    int _e7 = bias_1;
    return ((_e5 * 1.5) + float(_e7));
}

bool same(int left, int right) {
    int left_1 = 0;
    int right_1 = 0;
    left_1 = left;
    right_1 = right;
    int _e4 = left_1;
    int _e5 = right_1;
    return (_e4 == _e5);
}

void consume(bool stop, int value_11) {
    bool stop_1 = false;
    int value_12 = 0;
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

void forward(bool stop_2, int value_13) {
    bool stop_3 = false;
    int value_14 = 0;
    stop_3 = stop_2;
    value_14 = value_13;
    bool _e4 = stop_3;
    int _e5 = value_14;
    int _e6 = increment(_e5);
    consume(_e4, _e6);
    return;
}

void main() {
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint base = 0u;
    int value = 0;
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
        int _e27 = value;
        int _e28 = increment(_e27);
        uint _e25 = base;
        _group_0_binding_0_cs[_e25] = _e28;
        int _e33 = value;
        uint _e30 = base;
        _group_0_binding_0_cs[(_e30 + 1u)] = _e33;
        int _e38 = value;
        int _e39 = doubled(_e38);
        uint _e35 = base;
        _group_0_binding_0_cs[(_e35 + 2u)] = _e39;
        int _e44 = value;
        uint _e46 = advance(uint(_e44));
        uint _e41 = base;
        _group_0_binding_0_cs[(_e41 + 3u)] = int(_e46);
        int _e52 = value;
        uint _e54 = retreat(uint(_e52));
        uint _e49 = base;
        _group_0_binding_0_cs[(_e49 + 4u)] = int(_e54);
        int _e60 = value;
        float _e63 = scale(float(_e60), 2);
        uint _e57 = base;
        _group_0_binding_0_cs[(_e57 + 5u)] = int(_e63);
        int _e69 = value;
        uint _e70 = i;
        int _e72 = _group_0_binding_1_cs[_e70];
        bool _e73 = same(_e69, _e72);
        uint _e66 = base;
        _group_0_binding_0_cs[(_e66 + 6u)] = int(_e73);
        uint _e76 = i;
        int _e80 = value;
        value = (_e80 + 1);
        consume(((_e76 - 2u * (_e76 / 2u)) == 0u), _e80);
        int _e87 = value;
        uint _e84 = base;
        _group_0_binding_0_cs[(_e84 + 7u)] = _e87;
        uint _e89 = i;
        int _e93 = value;
        value = (_e93 + 1);
        forward(((_e89 - 2u * (_e89 / 2u)) != 0u), _e93);
        int _e100 = value;
        uint _e97 = base;
        _group_0_binding_0_cs[(_e97 + 8u)] = _e100;
        int _e105 = value;
        uint _e106 = i;
        int _e108 = _group_0_binding_1_cs[_e106];
        bool _e109 = same(_e105, _e108);
        uint _e102 = base;
        _group_0_binding_0_cs[(_e102 + 9u)] = int(_e109);
        int _e115 = value;
        int _e116 = increment(_e115);
        int _e117 = increment(_e116);
        uint _e112 = base;
        _group_0_binding_0_cs[(_e112 + 10u)] = _e117;
        int _e122 = value;
        uint _e119 = base;
        _group_0_binding_0_cs[(_e119 + 11u)] = _e122;
    }
}

