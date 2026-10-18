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


int cached_type(int value_1) {
    int value_2 = 0;
    int Number = 0;
    int next = 0;
    int result = 0;
    value_2 = value_1;
    int _e3 = value_2;
    Number = _e3;
    int _e6 = Number;
    next = (_e6 + 1);
    int _e10 = next;
    result = (_e10 * 2);
    int _e12 = result;
    return _e12;
}

int pair_add(int left, int right) {
    int left_1 = 0;
    int right_1 = 0;
    left_1 = left;
    right_1 = right;
    int _e4 = left_1;
    int _e5 = right_1;
    return (_e4 + _e5);
}

void main() {
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint base = 0u;
    int value = 0;
    int first = 0;
    int second = 0;
    int third = 0;
    int omitted = 0;
    int initialized = 0;
    int final = 0;
    int fixed_ = 0;
    int fixed_next = 0;
    int alias_first = 0;
    int alias_next = 0;
    bool positive = false;
    bool same = false;
    bool local = false;
    float low = 0.0;
    float high = 0.0;
    uint start = 0u;
    uint finish = 0u;
    int outside = 0;
    int outside_1 = 0;
    int derived = 0;
    int total = 0;
    int iteration = 0;
    int seed = 0;
    int term = 0;
    int copied = 0;
    int from_call = 0;
    int from_prior = 0;
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
        int _e26 = value;
        value = (_e26 + 1);
        first = _e26;
        int _e30 = value;
        value = (_e30 + 1);
        second = _e30;
        int _e34 = value;
        third = _e34;
        int _e38 = first;
        int _e41 = second;
        int _e44 = third;
        uint _e35 = base;
        _group_0_binding_0_cs[_e35] = (((_e38 * 100) + (_e41 * 10)) + _e44);
        int _e50 = value;
        uint _e47 = base;
        _group_0_binding_0_cs[(_e47 + 1u)] = _e50;
        int _e54 = value;
        initialized = (_e54 + 3);
        int _e58 = initialized;
        omitted = (_e58 + 2);
        int _e60 = omitted;
        int _e61 = initialized;
        final = (_e60 + _e61);
        int _e67 = final;
        uint _e64 = base;
        _group_0_binding_0_cs[(_e64 + 2u)] = _e67;
        int _e70 = value;
        fixed_ = (_e70 + 1);
        int _e74 = fixed_;
        fixed_next = (_e74 + 2);
        int _e80 = fixed_next;
        uint _e77 = base;
        _group_0_binding_0_cs[(_e77 + 3u)] = _e80;
        int _e83 = value;
        alias_first = (_e83 + 4);
        int _e87 = alias_first;
        alias_next = (_e87 + 1);
        int _e93 = alias_next;
        uint _e90 = base;
        _group_0_binding_0_cs[(_e90 + 4u)] = _e93;
        int _e96 = value;
        positive = (_e96 > 0);
        bool _e99 = positive;
        local = _e99;
        if (_e99) {
            uint _e101 = i;
            int _e104 = _group_0_binding_1_cs[_e101];
            int _e106 = value;
            local = (_e106 == (_e104 + 2));
        }
        bool _e108 = local;
        same = _e108;
        bool _e113 = same;
        uint _e110 = base;
        _group_0_binding_0_cs[(_e110 + 5u)] = int(_e113);
        int _e116 = value;
        low = (float(_e116) * 0.5);
        float _e122 = low;
        high = (_e122 + 1.5);
        float _e129 = high;
        uint _e125 = base;
        _group_0_binding_0_cs[(_e125 + 6u)] = int((_e129 * 2.0));
        uint _e134 = i;
        start = (_e134 - 5u * (_e134 / 5u));
        uint _e138 = start;
        finish = (_e138 + 3u);
        uint _e144 = finish;
        uint _e141 = base;
        _group_0_binding_0_cs[(_e141 + 7u)] = int(_e144);
        int _e150 = value;
        int _e151 = cached_type(_e150);
        uint _e147 = base;
        _group_0_binding_0_cs[(_e147 + 8u)] = _e151;
        int _e153 = value;
        outside = _e153;
        {
            int _e156 = value;
            outside_1 = (_e156 + 5);
            int _e160 = outside_1;
            derived = (_e160 + 2);
            int _e166 = derived;
            uint _e163 = base;
            _group_0_binding_0_cs[(_e163 + 9u)] = _e166;
        }
        total = 0;
        {
            iteration = 0;
            int _e172 = value;
            seed = _e172;
            bool loop_init = true;
            while(true) {
                if (!loop_init) {
                    int _e176 = iteration;
                    iteration = (_e176 + 1);
                }
                loop_init = false;
                int _e174 = iteration;
                if ((_e174 < 4)) {
                } else {
                    break;
                }
                int _e180 = iteration;
                if ((_e180 == 1)) {
                    continue;
                }
                int _e183 = seed;
                int _e184 = iteration;
                term = (_e183 + _e184);
                int _e187 = term;
                copied = _e187;
                int _e188 = copied;
                int _e189 = total;
                total = (_e189 + _e188);
            }
        }
        int _e195 = total;
        uint _e192 = base;
        _group_0_binding_0_cs[(_e192 + 10u)] = _e195;
        int _e197 = value;
        int _e199 = pair_add(_e197, 2);
        from_call = _e199;
        int _e201 = from_call;
        int _e203 = pair_add(_e201, 3);
        from_prior = _e203;
        int _e208 = from_prior;
        int _e209 = outside;
        uint _e205 = base;
        _group_0_binding_0_cs[(_e205 + 11u)] = (_e208 + _e209);
    }
}

