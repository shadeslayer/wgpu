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
const int Global = 2;
const float Scale = 0.5;

layout(std430) buffer type_4_block_0Compute { int _group_0_binding_0_cs[]; };


int pair(int first, int second) {
    int first_1 = 0;
    int second_1 = 0;
    first_1 = first;
    second_1 = second;
    int _e5 = first_1;
    int _e7 = second_1;
    return ((_e5 * 10) + _e7);
}

int list_sum(int seed_1) {
    int seed_2 = 0;
    int values_1[3] = int[3](0, 0, 0);
    uint local_4 = 0u;
    seed_2 = seed_1;
    int _e4 = seed_2;
    values_1[0] = _e4;
    int _e7 = seed_2;
    values_1[1] = (_e7 + 1);
    local_4 = 2u;
    bool loop_init = true;
    while(true) {
        if (!loop_init) {
            uint _e16 = local_4;
            local_4 = (_e16 + 1u);
        }
        loop_init = false;
        uint _e12 = local_4;
        if ((_e12 >= 3u)) {
            break;
        }
        values_1[_e12] = 0;
    }
    int _e23 = values_1[0];
    int _e24 = values_1[1];
    int _e28 = values_1[2];
    return ((_e23 + _e24) + _e28);
}

int3_ make_list(int seed_3) {
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
    int zero = 0;
    int copy_zero = 0;
    bool enabled = false;
    uint count = 0u;
    float sample_ = 0.0;
    int Local = 0;
    int Safe = 0;
    int local = 0;
    int safe_values[3] = int[3](0, 0, 0);
    uint local_1 = 0u;
    int values[3] = int[3](0, 0, 0);
    int changed[3] = int[3](0, 0, 0);
    int3_ point = int3_(0, 0, 0);
    int3_ changed_point = int3_(0, 0, 0);
    int3_ nested[2] = int3_[2](int3_(0, 0, 0), int3_(0, 0, 0));
    ivec2 flat_[2] = ivec2[2](ivec2(0), ivec2(0));
    int3_ original = int3_(0, 0, 0);
    int3_ copied = int3_(0, 0, 0);
    int3_ copies[2] = int3_[2](int3_(0, 0, 0), int3_(0, 0, 0));
    int counter = 0;
    int ordered[3] = int[3](0, 0, 0);
    int called = 0;
    int zero_total = 0;
    int iteration = 0;
    int reset[3] = int[3](0, 0, 0);
    uint local_2 = 0u;
    int total = 0;
    int iteration_1 = 0;
    int limit = 0;
    int Number = 0;
    int cached[2] = int[2](0, 0);
    int frozen[3] = int[3](0, 0, 0);
    uint local_3 = 0u;
    float3_ floating = float3_(0.0, 0.0, 0.0);
    int3_ from_call = int3_(0, 0, 0);
    int _e4 = _group_0_binding_0_cs[0];
    seed = _e4;
    zero = 0;
    copy_zero = 0;
    enabled = true;
    count = uint(7);
    sample_ = 1.5;
    Local = 3;
    Safe = 3;
    safe_values[0] = 1;
    local_1 = 1u;
    bool loop_init_1 = true;
    while(true) {
        if (!loop_init_1) {
            uint _e30 = local_1;
            local_1 = (_e30 + 1u);
        }
        loop_init_1 = false;
        uint _e26 = local_1;
        if ((_e26 >= 3u)) {
            break;
        }
        safe_values[_e26] = 0;
    }
    int _e35 = zero;
    _group_0_binding_0_cs[0] = _e35;
    int _e38 = copy_zero;
    _group_0_binding_0_cs[1] = _e38;
    bool _e41 = enabled;
    _group_0_binding_0_cs[2] = int(_e41);
    uint _e45 = count;
    _group_0_binding_0_cs[3] = int(_e45);
    float _e50 = sample_;
    _group_0_binding_0_cs[4] = int((_e50 * 2.0));
    int _e56 = Local;
    _group_0_binding_0_cs[5] = (Global + _e56);
    int _e62 = safe_values[2];
    _group_0_binding_0_cs[6] = _e62;
    values[0] = 1;
    int _e70 = values[0];
    values[1] = (_e70 + 1);
    int _e76 = values[1];
    values[2] = (_e76 + 1);
    int _e84 = values[0];
    int _e85 = values[1];
    int _e89 = values[2];
    _group_0_binding_0_cs[7] = ((_e84 + _e85) + _e89);
    changed[0] = 1;
    changed[0] = 7;
    changed[1] = 2;
    int _e102 = changed[0];
    changed[2] = _e102;
    int _e107 = changed[0];
    _group_0_binding_0_cs[8] = _e107;
    int _e112 = changed[2];
    _group_0_binding_0_cs[9] = _e112;
    point.x = 1;
    int _e118 = point.x;
    point.y = (_e118 + 1);
    int _e123 = point.y;
    point.z = (_e123 + 1);
    int _e129 = point.z;
    _group_0_binding_0_cs[10] = _e129;
    changed_point.x = 1;
    changed_point.x = 9;
    changed_point.y = 2;
    int _e138 = changed_point.x;
    changed_point.z = _e138;
    int _e143 = changed_point.x;
    _group_0_binding_0_cs[11] = _e143;
    int _e147 = changed_point.z;
    _group_0_binding_0_cs[12] = _e147;
    nested[0].x = 1;
    nested[0].y = 2;
    nested[0].z = 0;
    nested[1].x = 3;
    nested[1].y = 0;
    nested[1].z = 0;
    int _e171 = nested[0].z;
    int _e172 = nested[1].x;
    _group_0_binding_0_cs[13] = (_e171 + _e172);
    flat_[0].x = 1;
    flat_[0].y = 2;
    flat_[1].x = 3;
    flat_[1].y = 0;
    int _e193 = flat_[0].x;
    int _e194 = flat_[0].y;
    int _e199 = flat_[1].x;
    int _e204 = flat_[1].y;
    _group_0_binding_0_cs[14] = (((_e193 + _e194) + _e199) + _e204);
    original.x = 2;
    original.y = 3;
    original.z = 4;
    int3_ _e214 = original;
    copied = _e214;
    int3_ _e217 = original;
    copies[0] = _e217;
    copies[1].x = 5;
    copies[1].y = 6;
    copies[1].z = 0;
    original.x = 99;
    int _e230 = copied.x;
    _group_0_binding_0_cs[15] = _e230;
    int _e236 = copies[0].x;
    _group_0_binding_0_cs[16] = _e236;
    int _e242 = copies[1].z;
    _group_0_binding_0_cs[17] = _e242;
    int _e244 = seed;
    counter = _e244;
    int _e247 = counter;
    counter = (_e247 + 1);
    ordered[0] = _e247;
    int _e251 = counter;
    counter = (_e251 + 1);
    ordered[1] = _e251;
    int _e255 = counter;
    ordered[2] = _e255;
    int _e261 = ordered[0];
    int _e266 = ordered[1];
    int _e271 = ordered[2];
    _group_0_binding_0_cs[18] = (((_e261 * 100) + (_e266 * 10)) + _e271);
    int _e275 = counter;
    _group_0_binding_0_cs[19] = _e275;
    int _e277 = counter;
    counter = (_e277 + 1);
    int _e281 = pair(_e277, 4);
    called = _e281;
    int _e284 = called;
    _group_0_binding_0_cs[20] = _e284;
    zero_total = 0;
    {
        iteration = 0;
        bool loop_init_2 = true;
        while(true) {
            if (!loop_init_2) {
                int _e292 = iteration;
                iteration = (_e292 + 1);
            }
            loop_init_2 = false;
            int _e290 = iteration;
            if ((_e290 < 3)) {
            } else {
                break;
            }
            int _e297 = iteration;
            reset[0] = _e297;
            local_2 = 1u;
            bool loop_init_3 = true;
            while(true) {
                if (!loop_init_3) {
                    uint _e305 = local_2;
                    local_2 = (_e305 + 1u);
                }
                loop_init_3 = false;
                uint _e301 = local_2;
                if ((_e301 >= 3u)) {
                    break;
                }
                reset[_e301] = 0;
            }
            int _e310 = reset[2];
            int _e311 = zero_total;
            zero_total = (_e311 + _e310);
            reset[2] = 9;
        }
    }
    int _e318 = zero_total;
    _group_0_binding_0_cs[21] = _e318;
    total = 0;
    {
        iteration_1 = 0;
        limit = 3;
        bool loop_init_4 = true;
        while(true) {
            if (!loop_init_4) {
                int _e328 = iteration_1;
                iteration_1 = (_e328 + 1);
            }
            loop_init_4 = false;
            int _e325 = iteration_1;
            int _e326 = limit;
            if ((_e325 < _e326)) {
            } else {
                break;
            }
            int _e331 = iteration_1;
            int _e332 = total;
            total = (_e332 + _e331);
        }
    }
    int _e336 = total;
    _group_0_binding_0_cs[22] = _e336;
    Number = 4;
    int _e341 = Number;
    cached[0] = _e341;
    int _e344 = Number;
    cached[1] = (_e344 + 1);
    int _e350 = cached[1];
    _group_0_binding_0_cs[23] = _e350;
    frozen[0] = 3;
    local_3 = 1u;
    bool loop_init_5 = true;
    while(true) {
        if (!loop_init_5) {
            uint _e361 = local_3;
            local_3 = (_e361 + 1u);
        }
        loop_init_5 = false;
        uint _e357 = local_3;
        if ((_e357 >= 3u)) {
            break;
        }
        frozen[_e357] = 0;
    }
    int _e370 = frozen[0];
    int _e371 = frozen[1];
    int _e375 = frozen[2];
    _group_0_binding_0_cs[24] = ((_e370 + _e371) + _e375);
    floating.x = 1.5;
    floating.y = -(0.0);
    floating.z = 0.0;
    float _e388 = floating.y;
    _group_0_binding_0_cs[25] = floatBitsToInt(_e388);
    float _e393 = floating.z;
    _group_0_binding_0_cs[26] = floatBitsToInt(_e393);
    int3_ _e397 = make_list(6);
    from_call = _e397;
    int _e402 = from_call.x;
    int _e403 = from_call.z;
    _group_0_binding_0_cs[27] = (_e402 + _e403);
    _group_0_binding_0_cs[28] = int((Scale * 10.0));
    int _e414 = seed;
    int _e416 = list_sum((_e414 + 3));
    _group_0_binding_0_cs[29] = _e416;
}

