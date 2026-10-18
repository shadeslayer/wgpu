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


int select_value(int value_1, bool choose_first) {
    int value_2 = 0;
    bool choose_first_1 = false;
    int result = 0;
    value_2 = value_1;
    choose_first_1 = choose_first;
    bool _e5 = choose_first_1;
    if (_e5) {
        int _e7 = value_2;
        result = (_e7 + 3);
    } else {
        int _e10 = value_2;
        result = (_e10 - 2);
    }
    int _e12 = result;
    return _e12;
}

float scaled(float value_3) {
    float value_4 = 0.0;
    float result_1 = 0.0;
    value_4 = value_3;
    float _e4 = value_4;
    result_1 = ((_e4 * 2.0) + 4.0);
    float _e8 = result_1;
    return _e8;
}

uint accumulate(uint limit) {
    uint limit_1 = 0u;
    uint sum = 0u;
    uint i_1 = 0u;
    uint term = 0u;
    limit_1 = limit;
    sum = 0u;
    {
        i_1 = 0u;
        bool loop_init = true;
        while(true) {
            if (!loop_init) {
                uint _e9 = i_1;
                i_1 = (_e9 + uint(1));
            }
            loop_init = false;
            uint _e6 = i_1;
            uint _e7 = limit_1;
            if ((_e6 < _e7)) {
            } else {
                break;
            }
            uint _e15 = i_1;
            if (((_e15 - 2u * (_e15 / 2u)) == 0u)) {
                uint _e20 = i_1;
                term = (_e20 + 1u);
            } else {
                uint _e23 = i_1;
                term = (_e23 * 2u);
            }
            uint _e25 = term;
            uint _e26 = sum;
            sum = (_e26 + _e25);
        }
    }
    uint _e28 = sum;
    return _e28;
}

void main() {
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint base = 0u;
    int value = 0;
    int chosen = 0;
    float weighted = 0.0;
    bool negative = false;
    uint magnitude = 0u;
    int total = 0;
    int iteration = 0;
    int copied = 0;
    int descending = 0;
    int iteration_1 = 0;
    int staged = 0;
    int remaining = 0;
    int partial = 0;
    int next = 0;
    int shadow = 0;
    int shadow_1 = 0;
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
        if (((_e27 - 2u * (_e27 / 2u)) == 0u)) {
            int _e32 = value;
            chosen = (_e32 + 7);
        } else {
            int _e35 = value;
            chosen = (_e35 - 4);
        }
        int _e39 = chosen;
        uint _e37 = base;
        _group_0_binding_0_cs[_e37] = _e39;
        int _e44 = value;
        uint _e46 = i;
        int _e50 = select_value(_e44, ((_e46 - 3u * (_e46 / 3u)) == 0u));
        uint _e41 = base;
        _group_0_binding_0_cs[(_e41 + 1u)] = _e50;
        int _e52 = value;
        weighted = ((float(_e52) * 1.5) + 2.0);
        float _e62 = weighted;
        uint _e59 = base;
        _group_0_binding_0_cs[(_e59 + 2u)] = int(_e62);
        int _e66 = value;
        negative = (_e66 < 0);
        bool _e72 = negative;
        uint _e69 = base;
        _group_0_binding_0_cs[(_e69 + 3u)] = int(_e72);
        int _e76 = value;
        magnitude = uint((_e76 + 5));
        uint _e84 = magnitude;
        uint _e80 = base;
        _group_0_binding_0_cs[(_e80 + 4u)] = int((_e84 * 3u));
        uint _e92 = i;
        uint _e94 = accumulate((_e92 - 5u * (_e92 / 5u)));
        uint _e88 = base;
        _group_0_binding_0_cs[(_e88 + 5u)] = int(_e94);
        total = 0;
        {
            iteration = 0;
            bool loop_init_1 = true;
            while(true) {
                if (!loop_init_1) {
                    int _e103 = iteration;
                    iteration = (_e103 + 1);
                }
                loop_init_1 = false;
                int _e101 = iteration;
                if ((_e101 < 3)) {
                } else {
                    break;
                }
                int _e107 = value;
                int _e108 = iteration;
                copied = (_e107 + _e108);
                int _e110 = copied;
                int _e111 = total;
                total = (_e111 + _e110);
            }
        }
        int _e117 = total;
        uint _e114 = base;
        _group_0_binding_0_cs[(_e114 + 6u)] = _e117;
        descending = 0;
        iteration_1 = 0;
        {
            bool loop_init_2 = true;
            while(true) {
                if (!loop_init_2) {
                    int _e126 = iteration_1;
                    iteration_1 = (_e126 + 1);
                }
                loop_init_2 = false;
                int _e124 = iteration_1;
                if ((_e124 < 3)) {
                } else {
                    break;
                }
                int _e129 = value;
                int _e130 = iteration_1;
                staged = (_e129 - _e130);
                int _e132 = staged;
                int _e133 = descending;
                descending = (_e133 + _e132);
            }
        }
        int _e139 = descending;
        uint _e136 = base;
        _group_0_binding_0_cs[(_e136 + 7u)] = _e139;
        remaining = 2;
        int _e143 = value;
        partial = _e143;
        while(true) {
            int _e145 = remaining;
            if ((_e145 > 0)) {
            } else {
                break;
            }
            {
                int _e148 = partial;
                int _e149 = remaining;
                next = (_e148 + _e149);
                int _e151 = next;
                partial = _e151;
                int _e152 = remaining;
                remaining = (_e152 - 1);
            }
        }
        int _e159 = partial;
        uint _e156 = base;
        _group_0_binding_0_cs[(_e156 + 8u)] = _e159;
        int _e161 = value;
        shadow = _e161;
        {
            int _e164 = value;
            shadow_1 = (_e164 + 10);
            int _e170 = shadow_1;
            uint _e167 = base;
            _group_0_binding_0_cs[(_e167 + 9u)] = _e170;
        }
        int _e175 = shadow;
        uint _e172 = base;
        _group_0_binding_0_cs[(_e172 + 10u)] = _e175;
        int _e180 = value;
        float _e182 = scaled(float(_e180));
        uint _e177 = base;
        _group_0_binding_0_cs[(_e177 + 11u)] = int(_e182);
    }
}

