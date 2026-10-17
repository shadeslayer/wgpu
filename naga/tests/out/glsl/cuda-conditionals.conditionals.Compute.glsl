#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct type_3 {
    uint value;
};
layout(std430) buffer type_1_block_0Compute { int _group_0_binding_0_cs[]; };

layout(std430) readonly buffer type_1_block_1Compute { int _group_0_binding_1_cs[]; };

layout(std430) readonly buffer type_1_block_2Compute { int _group_0_binding_2_cs[]; };

layout(std140) uniform type_3_block_3Compute { type_3 _group_0_binding_3_cs; };

shared int shared_values[64];


int shifted(int value_1) {
    int value_2 = 0;
    value_2 = value_1;
    int _e3 = value_2;
    return (_e3 + 10);
}

void discard_(int value_3) {
    int value_4 = 0;
    value_4 = value_3;
    int _e3 = value_4;
    value_4 = (_e3 + 1);
    return;
}

int pair(int x, int y) {
    int x_1 = 0;
    int y_1 = 0;
    x_1 = x;
    y_1 = y;
    int _e5 = x_1;
    int _e7 = y_1;
    return ((_e5 * 10) + _e7);
}

void main() {
    if (gl_LocalInvocationID == uvec3(0u)) {
        shared_values = int[64](0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    }
    memoryBarrierShared();
    barrier();
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 threadIdx = gl_LocalInvocationID;
    uint n = 0u;
    uint i = 0u;
    uint base = 0u;
    bool choose_a = false;
    int local = 0;
    int a = 0;
    int b = 0;
    int local_1 = 0;
    int index_a = 0;
    int index_b = 0;
    int local_2 = 0;
    int local_3 = 0;
    int c = 0;
    int local_4 = 0;
    int condition = 0;
    uint lane = 0u;
    int local_value = 0;
    float local_5 = 0.0;
    uint local_6 = 0u;
    bool local_7 = false;
    int local_8 = 0;
    uint local_9 = 0u;
    int shift_value = 0;
    int local_10 = 0;
    uint local_11 = 0u;
    int value = 0;
    int other = 0;
    int local_12 = 0;
    int count = 0;
    int left = 0;
    int right = 0;
    uint _e5 = _group_0_binding_3_cs.value;
    n = _e5;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e16 = i;
    uint _e17 = n;
    if ((_e16 < _e17)) {
        uint _e21 = i;
        base = (_e21 * 25u);
        uint _e25 = i;
        choose_a = ((_e25 - 2u * (_e25 / 2u)) == 0u);
        bool _e31 = choose_a;
        if (_e31) {
            uint _e32 = i;
            int _e37 = _group_0_binding_1_cs[_e32];
            local = _e37;
        } else {
            uint _e34 = i;
            int _e38 = _group_0_binding_2_cs[_e34];
            local = _e38;
        }
        int _e39 = local;
        uint _e29 = base;
        _group_0_binding_0_cs[_e29] = _e39;
        a = 3;
        b = 5;
        bool _e48 = choose_a;
        if (_e48) {
            int _e49 = a;
            a = (_e49 + 1);
            int _e52 = shifted(_e49);
            local_1 = _e52;
        } else {
            int _e53 = b;
            b = (_e53 + 1);
            int _e56 = shifted(_e53);
            local_1 = _e56;
        }
        int _e58 = local_1;
        uint _e45 = base;
        _group_0_binding_0_cs[(_e45 + 1u)] = _e58;
        int _e64 = a;
        int _e66 = b;
        uint _e60 = base;
        _group_0_binding_0_cs[(_e60 + 2u)] = ((_e64 * 10) + _e66);
        bool _e68 = choose_a;
        if (_e68) {
            int _e69 = a;
            a = (_e69 + 1);
            discard_(_e69);
        } else {
            int _e72 = b;
            b = (_e72 + 1);
            discard_(_e72);
        }
        int _e80 = a;
        int _e82 = b;
        uint _e76 = base;
        _group_0_binding_0_cs[(_e76 + 3u)] = ((_e80 * 10) + _e82);
        index_a = 0;
        index_b = 0;
        bool _e88 = choose_a;
        if (_e88) {
            int _e89 = index_a;
            index_a = (_e89 + 1);
        } else {
            int _e92 = index_b;
            index_b = (_e92 + 1);
        }
        int _e100 = index_a;
        int _e102 = index_b;
        uint _e96 = base;
        _group_0_binding_0_cs[(_e96 + 4u)] = ((_e100 * 10) + _e102);
        a = 3;
        b = 5;
        bool _e106 = choose_a;
        if (_e106) {
            a = 7;
        } else {
            b = 7;
        }
        int _e113 = a;
        int _e115 = b;
        uint _e109 = base;
        _group_0_binding_0_cs[(_e109 + 5u)] = ((_e113 * 10) + _e115);
        bool _e117 = choose_a;
        if (_e117) {
            int _e119 = a;
            a = (_e119 + 2);
        } else {
            int _e121 = b;
            b = (_e121 + 2);
        }
        int _e128 = a;
        int _e130 = b;
        uint _e124 = base;
        _group_0_binding_0_cs[(_e124 + 6u)] = ((_e128 * 10) + _e130);
        bool _e136 = choose_a;
        if (_e136) {
            int _e137 = a;
            a = (_e137 + 1);
            int _e144 = a;
            local_2 = _e144;
        } else {
            int _e140 = b;
            b = (_e140 + 1);
            int _e145 = b;
            local_2 = _e145;
        }
        int _e146 = local_2;
        uint _e133 = base;
        _group_0_binding_0_cs[(_e133 + 7u)] = _e146;
        bool _e151 = choose_a;
        if (_e151) {
            int _e153 = a;
            a = (_e153 + 1);
            local_3 = _e153;
        } else {
            int _e156 = b;
            b = (_e156 + 1);
            local_3 = _e156;
        }
        int _e159 = local_3;
        uint _e148 = base;
        _group_0_binding_0_cs[(_e148 + 8u)] = _e159;
        int _e165 = a;
        int _e167 = b;
        uint _e161 = base;
        _group_0_binding_0_cs[(_e161 + 9u)] = ((_e165 * 100) + _e167);
        bool _e170 = choose_a;
        c = int(_e170);
        b = 7;
        int _e177 = c;
        if ((_e177 != 0)) {
            c = 0;
            int _e182 = c;
            c = (_e182 + 1);
            local_4 = _e182;
        } else {
            b = 0;
            int _e185 = b;
            b = (_e185 + 1);
            local_4 = _e185;
        }
        int _e188 = local_4;
        uint _e174 = base;
        _group_0_binding_0_cs[(_e174 + 10u)] = _e188;
        int _e194 = c;
        int _e196 = b;
        uint _e190 = base;
        _group_0_binding_0_cs[(_e190 + 11u)] = ((_e194 * 10) + _e196);
        condition = 0;
        a = 2;
        b = 4;
        condition = 1;
        int _e206 = condition;
        int _e202 = condition;
        if ((_e202 != 0)) {
            a = _e206;
        } else {
            b = _e206;
        }
        int _e212 = a;
        int _e214 = b;
        uint _e208 = base;
        _group_0_binding_0_cs[(_e208 + 12u)] = ((_e212 * 10) + _e214);
        a = 2;
        b = 4;
        c = 6;
        bool _e219 = choose_a;
        if (_e219) {
            uint _e221 = i;
            if (((_e221 - 4u * (_e221 / 4u)) == 0u)) {
                int _e226 = a;
                a = (_e226 + 3);
            } else {
                int _e228 = b;
                b = (_e228 + 3);
            }
        } else {
            int _e230 = c;
            c = (_e230 + 3);
        }
        int _e237 = a;
        int _e240 = b;
        int _e243 = c;
        uint _e233 = base;
        _group_0_binding_0_cs[(_e233 + 13u)] = (((_e237 * 100) + (_e240 * 10)) + _e243);
        lane = threadIdx.x;
        uint _e247 = lane;
        shared_values[_e247] = 20;
        local_value = 10;
        bool _e252 = choose_a;
        if (_e252) {
            int _e256 = local_value;
            local_value = (_e256 + 5);
        } else {
            uint _e253 = lane;
            int _e258 = shared_values[_e253];
            shared_values[_e253] = (_e258 + 5);
        }
        int _e265 = local_value;
        uint _e267 = lane;
        int _e269 = shared_values[_e267];
        uint _e261 = base;
        _group_0_binding_0_cs[(_e261 + 14u)] = ((_e265 * 100) + _e269);
        bool _e271 = choose_a;
        if (_e271) {
            uint _e272 = lane;
            shared_values[_e272] = 30;
        } else {
            uint _e275 = base;
            _group_0_binding_0_cs[(_e275 + 15u)] = 30;
        }
        uint _e283 = lane;
        int _e285 = shared_values[_e283];
        uint _e280 = base;
        _group_0_binding_0_cs[(_e280 + 16u)] = _e285;
        bool _e290 = choose_a;
        if (_e290) {
            local_5 = 1.5;
        } else {
            local_5 = float(3);
        }
        float _e295 = local_5;
        uint _e287 = base;
        _group_0_binding_0_cs[(_e287 + 17u)] = int(_e295);
        bool _e301 = choose_a;
        if (_e301) {
            local_6 = uint(-(1));
        } else {
            local_6 = 2u;
        }
        uint _e307 = local_6;
        uint _e298 = base;
        _group_0_binding_0_cs[(_e298 + 18u)] = int(_e307);
        bool _e313 = choose_a;
        if (_e313) {
            local_7 = true;
        } else {
            local_7 = false;
        }
        bool _e317 = local_7;
        uint _e310 = base;
        _group_0_binding_0_cs[(_e310 + 19u)] = int(_e317);
        bool _e323 = choose_a;
        if (_e323) {
            local_8 = int(true);
        } else {
            local_8 = 2;
        }
        int _e328 = local_8;
        uint _e320 = base;
        _group_0_binding_0_cs[(_e320 + 20u)] = _e328;
        bool _e333 = choose_a;
        if (_e333) {
            local_9 = 4u;
        } else {
            local_9 = 8u;
        }
        uint _e337 = local_9;
        uint _e330 = base;
        _group_0_binding_0_cs[(_e330 + 21u)] = int(_e337);
        a = 3;
        shift_value = 2;
        bool _e346 = choose_a;
        if (_e346) {
            int _e348 = shift_value;
            local_10 = _e348;
        } else {
            int _e349 = a;
            local_10 = _e349;
        }
        int _e350 = local_10;
        shift_value = 1;
        int _e352 = shift_value;
        local_11 = uint(_e352);
        uint _e355 = local_11;
        uint _e343 = base;
        _group_0_binding_0_cs[(_e343 + 22u)] = (_e350 << _e355);
        value = 3;
        other = 7;
        bool _e365 = choose_a;
        if (_e365) {
            int _e367 = value;
            local_12 = _e367;
        } else {
            int _e368 = other;
            local_12 = _e368;
        }
        int _e369 = local_12;
        int _e370 = value;
        value = (_e370 + 1);
        int _e373 = pair(_e369, _e370);
        uint _e362 = base;
        _group_0_binding_0_cs[(_e362 + 23u)] = _e373;
        count = 0;
        left = 0;
        right = 0;
        {
            bool loop_init = true;
            while(true) {
                if (!loop_init) {
                    bool _e383 = choose_a;
                    if (_e383) {
                        int _e384 = left;
                        left = (_e384 + 1);
                    } else {
                        int _e387 = right;
                        right = (_e387 + 1);
                    }
                }
                loop_init = false;
                int _e381 = count;
                if ((_e381 < 3)) {
                } else {
                    break;
                }
                int _e390 = count;
                count = (_e390 + 1);
                int _e394 = count;
                if ((_e394 == 2)) {
                    continue;
                }
            }
        }
        int _e401 = left;
        int _e404 = right;
        int _e407 = count;
        uint _e397 = base;
        _group_0_binding_0_cs[(_e397 + 24u)] = (((_e401 * 100) + (_e404 * 10)) + _e407);
    }
}

