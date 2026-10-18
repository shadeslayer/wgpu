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

static const int Global = int(2);
static const float Scale = 0.5;

RWByteAddressBuffer out_ : register(u0);

int pair(int first, int second)
{
    int first_1 = (int)0;
    int second_1 = (int)0;

    first_1 = first;
    second_1 = second;
    int _e5 = first_1;
    int _e7 = second_1;
    return asint(asuint(asint(asuint(_e5) * asuint(int(10)))) + asuint(_e7));
}

int ZeroValueint() {
    return (int)0;
}

int list_sum(int seed_1)
{
    int seed_2 = (int)0;
    int values_1[3] = (int[3])0;
    uint local_4 = (uint)0;

    seed_2 = seed_1;
    int _e4 = seed_2;
    values_1[0] = _e4;
    int _e7 = seed_2;
    values_1[1] = asint(asuint(_e7) + asuint(int(1)));
    local_4 = 2u;
    uint2 loop_bound = uint2(4294967295u, 4294967295u);
    bool loop_init = true;
    while(true) {
        if (all(loop_bound == uint2(0u, 0u))) { break; }
        loop_bound -= uint2(loop_bound.y == 0u, 1u);
        if (!loop_init) {
            uint _e16 = local_4;
            local_4 = (_e16 + 1u);
        }
        loop_init = false;
        uint _e12 = local_4;
        if ((_e12 >= 3u)) {
            break;
        }
        values_1[min(uint(_e12), 2u)] = ZeroValueint();
    }
    int _e23 = values_1[int(0)];
    int _e24 = values_1[int(1)];
    int _e28 = values_1[int(2)];
    return asint(asuint(asint(asuint(_e23) + asuint(_e24))) + asuint(_e28));
}

int3_ make_list(int seed_3)
{
    int seed_4 = (int)0;
    int3_ value = (int3_)0;

    seed_4 = seed_3;
    int _e3 = seed_4;
    value.x = _e3;
    int _e6 = seed_4;
    value.y = asint(asuint(_e6) + asuint(int(1)));
    int _e10 = seed_4;
    value.z = asint(asuint(_e10) + asuint(int(2)));
    int3_ _e13 = value;
    const int3_ int3_1 = _e13;
    return int3_1;
}

float ZeroValuefloat() {
    return (float)0;
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

[numthreads(1, 1, 1)]
void list_initialization()
{
    int seed = (int)0;
    int zero = (int)0;
    int copy_zero = (int)0;
    bool enabled = (bool)0;
    uint count = (uint)0;
    float sample_ = (float)0;
    int Local = (int)0;
    int Safe = (int)0;
    int local = (int)0;
    int safe_values[3] = (int[3])0;
    uint local_1 = (uint)0;
    int values[3] = (int[3])0;
    int changed[3] = (int[3])0;
    int3_ point_ = (int3_)0;
    int3_ changed_point = (int3_)0;
    int3_ nested[2] = (int3_[2])0;
    int2 flat[2] = (int2[2])0;
    int3_ original = (int3_)0;
    int3_ copied = (int3_)0;
    int3_ copies[2] = (int3_[2])0;
    int counter = (int)0;
    int ordered[3] = (int[3])0;
    int called = (int)0;
    int zero_total = (int)0;
    int iteration = (int)0;
    int reset[3] = (int[3])0;
    uint local_2 = (uint)0;
    int total = (int)0;
    int iteration_1 = (int)0;
    int limit = (int)0;
    int Number = (int)0;
    int cached[2] = (int[2])0;
    int frozen[3] = (int[3])0;
    uint local_3 = (uint)0;
    float3_ floating = (float3_)0;
    int3_ from_call = (int3_)0;

    int _e4 = asint(out_.Load(int(0)*4));
    seed = _e4;
    zero = int(0);
    copy_zero = int(0);
    enabled = true;
    count = uint(int(7));
    sample_ = 1.5;
    Local = int(3);
    Safe = int(3);
    safe_values[0] = int(1);
    local_1 = 1u;
    uint2 loop_bound_1 = uint2(4294967295u, 4294967295u);
    bool loop_init_1 = true;
    while(true) {
        if (all(loop_bound_1 == uint2(0u, 0u))) { break; }
        loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
        if (!loop_init_1) {
            uint _e30 = local_1;
            local_1 = (_e30 + 1u);
        }
        loop_init_1 = false;
        uint _e26 = local_1;
        if ((_e26 >= 3u)) {
            break;
        }
        safe_values[min(uint(_e26), 2u)] = ZeroValueint();
    }
    int _e35 = zero;
    out_.Store(int(0)*4, asuint(_e35));
    int _e38 = copy_zero;
    out_.Store(int(1)*4, asuint(_e38));
    bool _e41 = enabled;
    out_.Store(int(2)*4, asuint(int(_e41)));
    uint _e45 = count;
    out_.Store(int(3)*4, asuint(int(_e45)));
    float _e50 = sample_;
    out_.Store(int(4)*4, asuint(naga_f2i32((_e50 * 2.0))));
    int _e56 = Local;
    out_.Store(int(5)*4, asuint(asint(asuint(Global) + asuint(_e56))));
    int _e62 = safe_values[int(2)];
    out_.Store(int(6)*4, asuint(_e62));
    values[0] = int(1);
    int _e70 = values[int(0)];
    values[1] = asint(asuint(_e70) + asuint(int(1)));
    int _e76 = values[int(1)];
    values[2] = asint(asuint(_e76) + asuint(int(1)));
    int _e84 = values[int(0)];
    int _e85 = values[int(1)];
    int _e89 = values[int(2)];
    out_.Store(int(7)*4, asuint(asint(asuint(asint(asuint(_e84) + asuint(_e85))) + asuint(_e89))));
    changed[0] = int(1);
    changed[int(0)] = int(7);
    changed[1] = int(2);
    int _e102 = changed[int(0)];
    changed[2] = _e102;
    int _e107 = changed[int(0)];
    out_.Store(int(8)*4, asuint(_e107));
    int _e112 = changed[int(2)];
    out_.Store(int(9)*4, asuint(_e112));
    point_.x = int(1);
    int _e118 = point_.x;
    point_.y = asint(asuint(_e118) + asuint(int(1)));
    int _e123 = point_.y;
    point_.z = asint(asuint(_e123) + asuint(int(1)));
    int _e129 = point_.z;
    out_.Store(int(10)*4, asuint(_e129));
    changed_point.x = int(1);
    changed_point.x = int(9);
    changed_point.y = int(2);
    int _e138 = changed_point.x;
    changed_point.z = _e138;
    int _e143 = changed_point.x;
    out_.Store(int(11)*4, asuint(_e143));
    int _e147 = changed_point.z;
    out_.Store(int(12)*4, asuint(_e147));
    nested[0].x = int(1);
    nested[0].y = int(2);
    nested[0].z = ZeroValueint();
    nested[1].x = int(3);
    nested[1].y = ZeroValueint();
    nested[1].z = ZeroValueint();
    int _e171 = nested[int(0)].z;
    int _e172 = nested[int(1)].x;
    out_.Store(int(13)*4, asuint(asint(asuint(_e171) + asuint(_e172))));
    flat[0].x = int(1);
    flat[0].y = int(2);
    flat[1].x = int(3);
    flat[1].y = ZeroValueint();
    int _e193 = flat[int(0)].x;
    int _e194 = flat[int(0)].y;
    int _e199 = flat[int(1)].x;
    int _e204 = flat[int(1)].y;
    out_.Store(int(14)*4, asuint(asint(asuint(asint(asuint(asint(asuint(_e193) + asuint(_e194))) + asuint(_e199))) + asuint(_e204))));
    original.x = int(2);
    original.y = int(3);
    original.z = int(4);
    int3_ _e214 = original;
    copied = _e214;
    int3_ _e217 = original;
    copies[0] = _e217;
    copies[1].x = int(5);
    copies[1].y = int(6);
    copies[1].z = ZeroValueint();
    original.x = int(99);
    int _e230 = copied.x;
    out_.Store(int(15)*4, asuint(_e230));
    int _e236 = copies[int(0)].x;
    out_.Store(int(16)*4, asuint(_e236));
    int _e242 = copies[int(1)].z;
    out_.Store(int(17)*4, asuint(_e242));
    int _e244 = seed;
    counter = _e244;
    int _e247 = counter;
    counter = asint(asuint(_e247) + asuint(int(1)));
    ordered[0] = _e247;
    int _e251 = counter;
    counter = asint(asuint(_e251) + asuint(int(1)));
    ordered[1] = _e251;
    int _e255 = counter;
    ordered[2] = _e255;
    int _e261 = ordered[int(0)];
    int _e266 = ordered[int(1)];
    int _e271 = ordered[int(2)];
    out_.Store(int(18)*4, asuint(asint(asuint(asint(asuint(asint(asuint(_e261) * asuint(int(100)))) + asuint(asint(asuint(_e266) * asuint(int(10)))))) + asuint(_e271))));
    int _e275 = counter;
    out_.Store(int(19)*4, asuint(_e275));
    int _e277 = counter;
    counter = asint(asuint(_e277) + asuint(int(1)));
    const int _e281 = pair(_e277, int(4));
    called = _e281;
    int _e284 = called;
    out_.Store(int(20)*4, asuint(_e284));
    zero_total = int(0);
    {
        iteration = int(0);
        uint2 loop_bound_2 = uint2(4294967295u, 4294967295u);
        bool loop_init_2 = true;
        while(true) {
            if (all(loop_bound_2 == uint2(0u, 0u))) { break; }
            loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
            if (!loop_init_2) {
                int _e292 = iteration;
                iteration = asint(asuint(_e292) + asuint(int(1)));
            }
            loop_init_2 = false;
            int _e290 = iteration;
            if ((_e290 < int(3))) {
            } else {
                break;
            }
            int _e297 = iteration;
            reset[0] = _e297;
            local_2 = 1u;
            uint2 loop_bound_3 = uint2(4294967295u, 4294967295u);
            bool loop_init_3 = true;
            while(true) {
                if (all(loop_bound_3 == uint2(0u, 0u))) { break; }
                loop_bound_3 -= uint2(loop_bound_3.y == 0u, 1u);
                if (!loop_init_3) {
                    uint _e305 = local_2;
                    local_2 = (_e305 + 1u);
                }
                loop_init_3 = false;
                uint _e301 = local_2;
                if ((_e301 >= 3u)) {
                    break;
                }
                reset[min(uint(_e301), 2u)] = ZeroValueint();
            }
            int _e310 = reset[int(2)];
            int _e311 = zero_total;
            zero_total = asint(asuint(_e311) + asuint(_e310));
            reset[int(2)] = int(9);
        }
    }
    int _e318 = zero_total;
    out_.Store(int(21)*4, asuint(_e318));
    total = int(0);
    {
        iteration_1 = int(0);
        limit = int(3);
        uint2 loop_bound_4 = uint2(4294967295u, 4294967295u);
        bool loop_init_4 = true;
        while(true) {
            if (all(loop_bound_4 == uint2(0u, 0u))) { break; }
            loop_bound_4 -= uint2(loop_bound_4.y == 0u, 1u);
            if (!loop_init_4) {
                int _e328 = iteration_1;
                iteration_1 = asint(asuint(_e328) + asuint(int(1)));
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
            total = asint(asuint(_e332) + asuint(_e331));
        }
    }
    int _e336 = total;
    out_.Store(int(22)*4, asuint(_e336));
    Number = int(4);
    int _e341 = Number;
    cached[0] = _e341;
    int _e344 = Number;
    cached[1] = asint(asuint(_e344) + asuint(int(1)));
    int _e350 = cached[int(1)];
    out_.Store(int(23)*4, asuint(_e350));
    frozen[0] = int(3);
    local_3 = 1u;
    uint2 loop_bound_5 = uint2(4294967295u, 4294967295u);
    bool loop_init_5 = true;
    while(true) {
        if (all(loop_bound_5 == uint2(0u, 0u))) { break; }
        loop_bound_5 -= uint2(loop_bound_5.y == 0u, 1u);
        if (!loop_init_5) {
            uint _e361 = local_3;
            local_3 = (_e361 + 1u);
        }
        loop_init_5 = false;
        uint _e357 = local_3;
        if ((_e357 >= 3u)) {
            break;
        }
        frozen[min(uint(_e357), 2u)] = ZeroValueint();
    }
    int _e370 = frozen[int(0)];
    int _e371 = frozen[int(1)];
    int _e375 = frozen[int(2)];
    out_.Store(int(24)*4, asuint(asint(asuint(asint(asuint(_e370) + asuint(_e371))) + asuint(_e375))));
    floating.x = 1.5;
    floating.y = -(0.0);
    floating.z = ZeroValuefloat();
    float _e388 = floating.y;
    out_.Store(int(25)*4, asuint(asint(_e388)));
    float _e393 = floating.z;
    out_.Store(int(26)*4, asuint(asint(_e393)));
    const int3_ _e397 = make_list(int(6));
    from_call = _e397;
    int _e402 = from_call.x;
    int _e403 = from_call.z;
    out_.Store(int(27)*4, asuint(asint(asuint(_e402) + asuint(_e403))));
    out_.Store(int(28)*4, asuint(naga_f2i32((Scale * 10.0))));
    int _e414 = seed;
    const int _e416 = list_sum(asint(asuint(_e414) + asuint(int(3))));
    out_.Store(int(29)*4, asuint(_e416));
}
