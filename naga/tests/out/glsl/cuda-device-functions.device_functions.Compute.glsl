#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct type_5 {
    uint value;
};
layout(std430) buffer type_4_block_0Compute { int _group_0_binding_0_cs[]; };

layout(std140) uniform type_5_block_1Compute { type_5 _group_0_binding_1_cs; };


float affine(float value_1, float scale, int offset) {
    float value_2 = 0.0;
    float scale_1 = 0.0;
    int offset_1 = 0;
    value_2 = value_1;
    scale_1 = scale;
    offset_1 = offset;
    float _e6 = scale_1;
    float _e7 = value_2;
    value_2 = (_e7 * _e6);
    float _e9 = value_2;
    int _e10 = offset_1;
    return (_e9 + float(_e10));
}

bool nonnegative(int value_3) {
    int value_4 = 0;
    value_4 = value_3;
    int _e3 = value_4;
    return (_e3 >= 0);
}

uint sum_steps(uint count, bool include_even) {
    uint count_1 = 0u;
    bool include_even_1 = false;
    uint total = 0u;
    uint i_1 = 0u;
    bool local = false;
    count_1 = count;
    include_even_1 = include_even;
    total = 0u;
    {
        i_1 = 0u;
        bool loop_init = true;
        while(true) {
            if (!loop_init) {
                uint _e11 = i_1;
                i_1 = (_e11 + uint(1));
            }
            loop_init = false;
            uint _e8 = i_1;
            uint _e9 = count_1;
            if ((_e8 < _e9)) {
            } else {
                break;
            }
            bool _e15 = include_even_1;
            local = _e15;
            if (_e15) {
            } else {
                uint _e18 = i_1;
                local = ((_e18 - 2u * (_e18 / 2u)) != 0u);
            }
            bool _e22 = local;
            if (_e22) {
                uint _e23 = i_1;
                uint _e24 = total;
                total = (_e24 + _e23);
            }
        }
    }
    uint _e26 = total;
    return _e26;
}

int choose(int value_5) {
    int value_6 = 0;
    value_6 = value_5;
    int _e2 = value_6;
    bool _e3 = nonnegative(_e2);
    if (_e3) {
        int _e4 = value_6;
        float _e8 = affine(float(_e4), 2.0, 1);
        return int(_e8);
    }
    int _e10 = value_6;
    return -(_e10);
}

int pair(int a, int b) {
    int a_1 = 0;
    int b_1 = 0;
    a_1 = a;
    b_1 = b;
    int _e5 = a_1;
    int _e7 = b_1;
    return ((_e5 * 10) + _e7);
}

void main() {
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint base = 0u;
    int value = 0;
    int argument = 0;
    uint _e3 = _group_0_binding_1_cs.value;
    n = _e3;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e13 = i;
    uint _e14 = n;
    if ((_e13 < _e14)) {
        uint _e18 = i;
        base = (_e18 * 10u);
        uint _e21 = i;
        value = (int(_e21) - 35);
        argument = 3;
        int _e29 = value;
        int _e30 = choose(_e29);
        uint _e27 = base;
        _group_0_binding_0_cs[_e27] = _e30;
        int _e35 = value;
        float _e39 = affine(float(_e35), 1.5, 2);
        uint _e32 = base;
        _group_0_binding_0_cs[(_e32 + 1u)] = int(_e39);
        uint _e48 = sum_steps(uint(5), true);
        uint _e42 = base;
        _group_0_binding_0_cs[(_e42 + 2u)] = int(_e48);
        uint _e58 = sum_steps(5u, (0 != 0));
        uint _e51 = base;
        _group_0_binding_0_cs[(_e51 + 3u)] = int(_e58);
        int _e64 = value;
        float _e70 = affine(float(_e64), float(2u), int(true));
        uint _e61 = base;
        _group_0_binding_0_cs[(_e61 + 4u)] = int(_e70);
        int _e76 = value;
        uint _e73 = base;
        _group_0_binding_0_cs[(_e73 + 5u)] = _e76;
        int _e81 = value;
        int _e82 = choose(_e81);
        int _e84 = value;
        int _e86 = choose((_e84 + 1));
        uint _e78 = base;
        _group_0_binding_0_cs[(_e78 + 6u)] = (_e82 + _e86);
        int _e92 = value;
        bool _e93 = nonnegative(_e92);
        uint _e89 = base;
        _group_0_binding_0_cs[(_e89 + 7u)] = int(_e93);
        int _e99 = argument;
        int _e100 = argument;
        argument = (_e100 + 1);
        int _e103 = pair(_e99, _e100);
        uint _e96 = base;
        _group_0_binding_0_cs[(_e96 + 8u)] = _e103;
        int _e108 = argument;
        uint _e105 = base;
        _group_0_binding_0_cs[(_e105 + 9u)] = _e108;
    }
}

