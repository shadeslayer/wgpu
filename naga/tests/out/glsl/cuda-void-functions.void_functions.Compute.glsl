#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct type_5 {
    uint value;
};
layout(std430) buffer type_4_block_0Compute { int _group_0_binding_0_cs[]; };

layout(std140) uniform type_5_block_1Compute { type_5 _group_0_binding_1_cs; };


int next_value(int value_1) {
    int value_2 = 0;
    value_2 = value_1;
    int _e3 = value_2;
    return (_e3 + 1);
}

void consume(bool stop, int value_3, uint count, float scale) {
    bool stop_1 = false;
    int value_4 = 0;
    uint count_1 = 0u;
    float scale_1 = 0.0;
    stop_1 = stop;
    value_4 = value_3;
    count_1 = count;
    scale_1 = scale;
    uint _e8 = count_1;
    int _e9 = value_4;
    value_4 = int((uint(_e9) + _e8));
    bool _e13 = stop_1;
    if (_e13) {
        return;
    }
    float _e14 = scale_1;
    int _e15 = value_4;
    value_4 = int((float(_e15) * _e14));
    return;
}

void forward(bool stop_2, int value_5) {
    bool stop_3 = false;
    int value_6 = 0;
    stop_3 = stop_2;
    value_6 = value_5;
    bool _e4 = stop_3;
    int _e5 = value_6;
    int _e6 = next_value(_e5);
    consume(_e4, _e6, 2u, 0.5);
    return;
}

void main() {
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint base = 0u;
    int value = 0;
    int iterations = 0;
    uint _e3 = _group_0_binding_1_cs.value;
    n = _e3;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e13 = i;
    uint _e14 = n;
    if ((_e13 < _e14)) {
        uint _e18 = i;
        base = (_e18 * 6u);
        value = 2;
        int _e23 = value;
        value = (_e23 + 1);
        consume(false, _e23, 1u, 1.5);
        int _e30 = value;
        uint _e28 = base;
        _group_0_binding_0_cs[_e28] = _e30;
        int _e32 = value;
        value = (_e32 + 1);
        int _e35 = next_value(_e32);
        consume(true, _e35, 2u, 0.5);
        int _e42 = value;
        uint _e39 = base;
        _group_0_binding_0_cs[(_e39 + 1u)] = _e42;
        uint _e44 = i;
        int _e48 = value;
        value = (_e48 + 1);
        forward(((_e44 - 2u * (_e44 / 2u)) == 0u), _e48);
        int _e55 = value;
        uint _e52 = base;
        _group_0_binding_0_cs[(_e52 + 2u)] = _e55;
        iterations = 0;
        {
            int _e61 = value;
            value = (_e61 + 1);
            consume((0 != 0), _e61, uint(0), float(1u));
            bool loop_init = true;
            while(true) {
                if (!loop_init) {
                    int _e72 = value;
                    value = (_e72 + 1);
                    int _e75 = value;
                    consume(false, _e75, 0u, 1.0);
                }
                loop_init = false;
                int _e69 = iterations;
                if ((_e69 < 3)) {
                } else {
                    break;
                }
                int _e78 = iterations;
                iterations = (_e78 + 1);
                int _e82 = iterations;
                if ((_e82 == 1)) {
                    continue;
                }
            }
        }
        int _e88 = value;
        uint _e85 = base;
        _group_0_binding_0_cs[(_e85 + 3u)] = _e88;
        int _e93 = iterations;
        uint _e90 = base;
        _group_0_binding_0_cs[(_e90 + 4u)] = _e93;
        int _e98 = value;
        int _e99 = next_value(_e98);
        uint _e95 = base;
        _group_0_binding_0_cs[(_e95 + 5u)] = _e99;
    }
}

