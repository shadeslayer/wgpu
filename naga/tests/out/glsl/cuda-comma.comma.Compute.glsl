#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct type_3 {
    uint value;
};
layout(std430) buffer type_1_block_0Compute { int _group_0_binding_0_cs[]; };

layout(std430) readonly buffer type_1_block_1Compute { int _group_0_binding_1_cs[]; };

layout(std140) uniform type_3_block_2Compute { type_3 _group_0_binding_2_cs; };


void consume(int value_1) {
    int value_2 = 0;
    value_2 = value_1;
    int _e3 = value_2;
    if ((_e3 < 0)) {
        return;
    } else {
        return;
    }
}

int pair_add(int left_1, int right_1) {
    int left_2 = 0;
    int right_2 = 0;
    left_2 = left_1;
    right_2 = right_1;
    int _e4 = left_2;
    int _e5 = right_2;
    return (_e4 + _e5);
}

void main() {
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint base = 0u;
    int value = 0;
    bool choose = false;
    int x = 0;
    int sequenced = 0;
    int target = 0;
    int rhs = 0;
    int middle = 0;
    int selected = 0;
    int local = 0;
    int left = 0;
    int right = 0;
    int identity = 0;
    uint position = 0u;
    int effect = 0;
    int total = 0;
    int iteration = 0;
    int current = 0;
    int ticks = 0;
    int passes = 0;
    int argument = 0;
    int first = 0;
    int second = 0;
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
        int _e32 = value;
        x = _e32;
        int _e35 = x;
        x = (_e35 + 2);
        int _e38 = x;
        x = (_e38 * 3);
        int _e41 = x;
        sequenced = (_e41 - 1);
        int _e45 = sequenced;
        uint _e43 = base;
        _group_0_binding_0_cs[_e43] = _e45;
        int _e50 = x;
        uint _e47 = base;
        _group_0_binding_0_cs[(_e47 + 1u)] = _e50;
        target = 0;
        int _e53 = value;
        target = _e53;
        int _e55 = target;
        target = (_e55 + 4);
        int _e61 = target;
        uint _e58 = base;
        _group_0_binding_0_cs[(_e58 + 2u)] = _e61;
        int _e64 = value;
        target = (_e64 + 1);
        int _e67 = target;
        rhs = (_e67 + 3);
        int _e74 = rhs;
        int _e76 = target;
        uint _e70 = base;
        _group_0_binding_0_cs[(_e70 + 3u)] = ((_e74 * 10) + _e76);
        middle = 0;
        bool _e81 = choose;
        if (_e81) {
            int _e83 = value;
            middle = (_e83 + 2);
            int _e86 = middle;
            local = (_e86 + 3);
        } else {
            int _e89 = value;
            local = (_e89 - 2);
        }
        int _e92 = local;
        selected = _e92;
        int _e98 = selected;
        int _e100 = middle;
        uint _e94 = base;
        _group_0_binding_0_cs[(_e94 + 4u)] = ((_e98 * 10) + _e100);
        int _e103 = value;
        left = _e103;
        int _e106 = value;
        right = (_e106 + 10);
        int _e108 = value;
        consume(_e108);
        bool _e109 = choose;
        if (_e109) {
            int _e111 = left;
            left = (_e111 + 3);
        } else {
            int _e113 = right;
            right = (_e113 + 3);
        }
        int _e120 = left;
        int _e122 = right;
        uint _e116 = base;
        _group_0_binding_0_cs[(_e116 + 5u)] = ((_e120 * 100) + _e122);
        int _e125 = value;
        identity = _e125;
        int _e126 = identity;
        identity = (_e126 + 1);
        consume(_e126);
        int _e129 = identity;
        identity = (_e129 + 1);
        int _e136 = identity;
        uint _e133 = base;
        _group_0_binding_0_cs[(_e133 + 6u)] = _e136;
        uint _e139 = base;
        position = (_e139 + 4u);
        uint _e148 = position;
        position = (_e148 + 2u);
        int _e151 = value;
        uint _e141 = position;
        position = (_e141 + uint(1));
        uint _e145 = position;
        _group_0_binding_0_cs[_e145] = ((_e151 * 2) + 2);
        int _e156 = value;
        effect = _e156;
        int _e157 = effect;
        effect = (_e157 + 1);
        consume(_e157);
        int _e160 = effect;
        effect = (_e160 + 1);
        consume(_e160);
        int _e163 = effect;
        effect = (_e163 + 1);
        consume(_e163);
        int _e166 = effect;
        effect = (_e166 + 1);
        consume(_e166);
        int _e173 = effect;
        uint _e170 = base;
        _group_0_binding_0_cs[(_e170 + 8u)] = _e173;
        total = 0;
        {
            iteration = 0;
            int _e179 = value;
            current = _e179;
            bool loop_init = true;
            while(true) {
                if (!loop_init) {
                    int _e183 = iteration;
                    iteration = (_e183 + 1);
                    int _e187 = current;
                    current = (_e187 + 2);
                }
                loop_init = false;
                int _e181 = iteration;
                if ((_e181 < 4)) {
                } else {
                    break;
                }
                int _e190 = iteration;
                if ((_e190 != 1)) {
                    int _e192 = current;
                    int _e193 = total;
                    total = (_e193 + _e192);
                } else {
                    continue;
                }
            }
        }
        int _e199 = total;
        uint _e196 = base;
        _group_0_binding_0_cs[(_e196 + 9u)] = _e199;
        int _e201 = value;
        ticks = _e201;
        passes = 0;
        {
            int _e205 = value;
            ticks = (_e205 + 1);
            passes = 0;
            bool loop_init_1 = true;
            while(true) {
                if (!loop_init_1) {
                    int _e215 = ticks;
                    ticks = (_e215 + 2);
                }
                loop_init_1 = false;
                int _e209 = passes;
                passes = (_e209 + 1);
                int _e212 = passes;
                if ((_e212 < 4)) {
                } else {
                    break;
                }
            }
        }
        int _e222 = ticks;
        int _e224 = passes;
        uint _e218 = base;
        _group_0_binding_0_cs[(_e218 + 10u)] = ((_e222 * 10) + _e224);
        int _e227 = value;
        argument = _e227;
        int _e230 = argument;
        argument = (_e230 + 1);
        int _e232 = argument;
        int _e234 = pair_add(_e232, 2);
        first = _e234;
        int _e236 = first;
        int _e238 = pair_add(_e236, 3);
        second = _e238;
        int _e244 = second;
        int _e246 = argument;
        uint _e240 = base;
        _group_0_binding_0_cs[(_e240 + 11u)] = ((_e244 * 10) + _e246);
    }
}

