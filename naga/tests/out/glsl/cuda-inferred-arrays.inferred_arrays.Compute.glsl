#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

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
layout(std430) buffer type_2_block_0Compute { int _group_0_binding_0_cs[]; };


int pair(int first, int second) {
    int first_1 = 0;
    int second_1 = 0;
    first_1 = first;
    second_1 = second;
    int _e5 = first_1;
    int _e7 = second_1;
    return ((_e5 * 10) + _e7);
}

int sum(int seed_1) {
    int seed_2 = 0;
    int values_1[2] = int[2](0, 0);
    seed_2 = seed_1;
    int _e4 = seed_2;
    values_1[0] = _e4;
    int _e7 = seed_2;
    values_1[1] = (_e7 + 1);
    int _e13 = values_1[0];
    int _e14 = values_1[1];
    return (_e13 + _e14);
}

int3_ triple(int seed_3) {
    int seed_4 = 0;
    int3_ value = int3_(0, 0, 0);
    seed_4 = seed_3;
    int _e3 = seed_4;
    value.x = _e3;
    int _e6 = seed_4;
    value.y = (_e6 + 1);
    int _e10 = seed_4;
    value.z = (_e10 + 2);
    int3_ _e13 = value;
    return _e13;
}

void main() {
    int seed = 0;
    int values[3] = int[3](0, 0, 0);
    int copied[2] = int[2](0, 0);
    int zeroes[2] = int[2](0, 0);
    bool flags[3] = bool[3](false, false, false);
    uint counts[2] = uint[2](0u, 0u);
    float samples[2] = float[2](0.0, 0.0);
    int self[3] = int[3](0, 0, 0);
    int changed[3] = int[3](0, 0, 0);
    int3_ points[2] = int3_[2](int3_(0, 0, 0), int3_(0, 0, 0));
    ivec2 flat_[2] = ivec2[2](ivec2(0), ivec2(0));
    int3_ original = int3_(0, 0, 0);
    int3_ copies[3] = int3_[3](int3_(0, 0, 0), int3_(0, 0, 0), int3_(0, 0, 0));
    int Number[2] = int[2](0, 0);
    int next[2] = int[2](0, 0);
    int Count = 0;
    int scratch[3] = int[3](0, 0, 0);
    float pending[2] = float[2](0.0, 0.0);
    float3_ pending_vectors[2] = float3_[2](float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0));
    int counter = 0;
    int ordered[3] = int[3](0, 0, 0);
    int called[2] = int[2](0, 0);
    int zero_total = 0;
    int iteration = 0;
    ivec2 reset[2] = ivec2[2](ivec2(0), ivec2(0));
    int total = 0;
    int iteration_1 = 0;
    int frozen[2] = int[2](0, 0);
    float signed_zeroes[2] = float[2](0.0, 0.0);
    int3_ from_calls[2] = int3_[2](int3_(0, 0, 0), int3_(0, 0, 0));
    int _e4 = _group_0_binding_0_cs[0];
    seed = _e4;
    int _e7 = seed;
    values[0] = _e7;
    int _e10 = seed;
    values[1] = (_e10 + 1);
    int _e14 = seed;
    values[2] = (_e14 + 2);
    copied[0] = 4;
    copied[1] = 5;
    zeroes[0] = 0;
    zeroes[1] = 0;
    flags[0] = true;
    flags[1] = false;
    flags[2] = true;
    counts[0] = 3u;
    counts[1] = 5u;
    samples[0] = 1.5;
    samples[1] = 2.5;
    int _e49 = values[0];
    int _e50 = values[1];
    int _e54 = values[2];
    _group_0_binding_0_cs[0] = ((_e49 + _e50) + _e54);
    int _e60 = copied[0];
    _group_0_binding_0_cs[1] = _e60;
    int _e67 = zeroes[0];
    int _e68 = zeroes[1];
    _group_0_binding_0_cs[2] = (_e67 + _e68);
    bool _e74 = flags[0];
    bool _e78 = flags[1];
    bool _e83 = flags[2];
    _group_0_binding_0_cs[3] = ((int(_e74) + int(_e78)) + int(_e83));
    uint _e92 = counts[0];
    uint _e93 = counts[1];
    _group_0_binding_0_cs[4] = int((_e92 + _e93));
    float _e102 = samples[0];
    float _e103 = samples[1];
    _group_0_binding_0_cs[5] = int((_e102 + _e103));
    self[0] = 1;
    int _e113 = self[0];
    self[1] = (_e113 + 1);
    int _e119 = self[1];
    self[2] = (_e119 + 1);
    int _e126 = self[0];
    int _e131 = self[1];
    int _e136 = self[2];
    _group_0_binding_0_cs[6] = (((_e126 * 100) + (_e131 * 10)) + _e136);
    changed[0] = 1;
    changed[0] = 7;
    changed[1] = 2;
    int _e149 = changed[0];
    changed[2] = _e149;
    int _e154 = changed[0];
    _group_0_binding_0_cs[7] = _e154;
    int _e159 = changed[2];
    _group_0_binding_0_cs[8] = _e159;
    points[0].x = 1;
    int _e168 = points[0].x;
    points[0].y = (_e168 + 1);
    int _e175 = points[0].y;
    points[0].z = (_e175 + 1);
    int3_ _e181 = points[0];
    points[1] = _e181;
    int _e187 = points[1].z;
    _group_0_binding_0_cs[9] = _e187;
    flat_[0].x = 1;
    flat_[0].y = 2;
    flat_[1].x = 3;
    flat_[1].y = 0;
    int _e204 = flat_[1].y;
    _group_0_binding_0_cs[10] = _e204;
    int _e210 = flat_[1].x;
    _group_0_binding_0_cs[11] = _e210;
    original.x = 4;
    original.y = 5;
    original.z = 6;
    int3_ _e220 = original;
    copies[0] = _e220;
    int3_ _e222 = original;
    copies[1] = _e222;
    copies[2].x = 7;
    copies[2].y = 8;
    copies[2].z = 0;
    original.x = 99;
    int _e237 = copies[0].x;
    _group_0_binding_0_cs[12] = _e237;
    int _e243 = copies[1].y;
    _group_0_binding_0_cs[13] = _e243;
    int _e249 = copies[2].z;
    _group_0_binding_0_cs[14] = _e249;
    Number[0] = 2;
    Number[1] = 3;
    int _e259 = Number[1];
    next[0] = _e259;
    int _e263 = Number[0];
    next[1] = _e263;
    int _e268 = next[0];
    _group_0_binding_0_cs[15] = _e268;
    Count = 3;
    scratch = int[3](0, 0, 0);
    int _e277 = scratch[2];
    _group_0_binding_0_cs[16] = _e277;
    pending[0] = 1.0;
    pending[1] = float(2);
    float _e290 = pending[0];
    float _e291 = pending[1];
    _group_0_binding_0_cs[17] = int((_e290 + _e291));
    pending_vectors[0].x = 1.0;
    pending_vectors[0].y = float(2);
    pending_vectors[0].z = 0.0;
    pending_vectors[1].x = 3.0;
    pending_vectors[1].y = 0.0;
    pending_vectors[1].z = 0.0;
    float _e318 = pending_vectors[0].x;
    float _e319 = pending_vectors[0].y;
    float _e324 = pending_vectors[1].x;
    _group_0_binding_0_cs[18] = int(((_e318 + _e319) + _e324));
    int _e328 = seed;
    counter = _e328;
    int _e331 = counter;
    counter = (_e331 + 1);
    ordered[0] = _e331;
    int _e335 = counter;
    counter = (_e335 + 1);
    ordered[1] = _e335;
    int _e339 = counter;
    ordered[2] = _e339;
    int _e345 = ordered[0];
    int _e350 = ordered[1];
    int _e355 = ordered[2];
    _group_0_binding_0_cs[19] = (((_e345 * 100) + (_e350 * 10)) + _e355);
    int _e359 = counter;
    _group_0_binding_0_cs[20] = _e359;
    int _e362 = counter;
    counter = (_e362 + 1);
    int _e366 = pair(_e362, 4);
    called[0] = _e366;
    int _e368 = counter;
    called[1] = _e368;
    int _e375 = called[0];
    int _e376 = called[1];
    _group_0_binding_0_cs[21] = (_e375 + _e376);
    int _e380 = counter;
    _group_0_binding_0_cs[22] = _e380;
    zero_total = 0;
    {
        iteration = 0;
        bool loop_init = true;
        while(true) {
            if (!loop_init) {
                int _e388 = iteration;
                iteration = (_e388 + 1);
            }
            loop_init = false;
            int _e386 = iteration;
            if ((_e386 < 3)) {
            } else {
                break;
            }
            int _e393 = iteration;
            reset[0].x = _e393;
            reset[0].y = 0;
            reset[1] = ivec2(0);
            int _e405 = reset[0].y;
            int _e406 = reset[1].x;
            int _e408 = zero_total;
            zero_total = (_e408 + (_e405 + _e406));
            reset[0].y = 9;
            reset[1].x = 8;
        }
    }
    int _e420 = zero_total;
    _group_0_binding_0_cs[23] = _e420;
    total = 0;
    {
        iteration_1 = 0;
        bool loop_init_1 = true;
        while(true) {
            if (!loop_init_1) {
                int _e428 = iteration_1;
                iteration_1 = (_e428 + 1);
            }
            loop_init_1 = false;
            int _e426 = iteration_1;
            if ((_e426 < 4)) {
            } else {
                break;
            }
            int _e432 = iteration_1;
            if ((_e432 == 1)) {
                continue;
            }
            int _e434 = iteration_1;
            int _e435 = total;
            total = (_e435 + _e434);
        }
    }
    int _e439 = total;
    _group_0_binding_0_cs[24] = _e439;
    int _e443 = sum(4);
    _group_0_binding_0_cs[25] = _e443;
    frozen[0] = 2;
    frozen[1] = 3;
    int _e455 = frozen[0];
    int _e456 = frozen[1];
    _group_0_binding_0_cs[26] = (_e455 + _e456);
    signed_zeroes[0] = -(0.0);
    signed_zeroes[1] = 0.0;
    float _e468 = signed_zeroes[0];
    _group_0_binding_0_cs[27] = floatBitsToInt(_e468);
    float _e474 = signed_zeroes[1];
    _group_0_binding_0_cs[28] = floatBitsToInt(_e474);
    int3_ _e479 = triple(6);
    from_calls[0] = _e479;
    int3_ _e482 = triple(9);
    from_calls[1] = _e482;
    int _e491 = from_calls[0].x;
    int _e492 = from_calls[1].z;
    _group_0_binding_0_cs[29] = (_e491 + _e492);
}

