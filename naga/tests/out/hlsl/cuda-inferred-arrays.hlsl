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

int sum(int seed_1)
{
    int seed_2 = (int)0;
    int values_1[2] = (int[2])0;

    seed_2 = seed_1;
    int _e4 = seed_2;
    values_1[0] = _e4;
    int _e7 = seed_2;
    values_1[1] = asint(asuint(_e7) + asuint(int(1)));
    int _e13 = values_1[int(0)];
    int _e14 = values_1[int(1)];
    return asint(asuint(_e13) + asuint(_e14));
}

int3_ triple(int seed_3)
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

int ZeroValueint() {
    return (int)0;
}

typedef int ret_ZeroValuearray3_int_[3];
ret_ZeroValuearray3_int_ ZeroValuearray3_int_() {
    return (int[3])0;
}

float ZeroValuefloat() {
    return (float)0;
}

int2 ZeroValueint2() {
    return (int2)0;
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

[numthreads(1, 1, 1)]
void inferred_arrays()
{
    int seed = (int)0;
    int values[3] = (int[3])0;
    int copied[2] = (int[2])0;
    int zeroes[2] = (int[2])0;
    bool flags[3] = (bool[3])0;
    uint counts[2] = (uint[2])0;
    float samples[2] = (float[2])0;
    int self[3] = (int[3])0;
    int changed[3] = (int[3])0;
    int3_ points[2] = (int3_[2])0;
    int2 flat[2] = (int2[2])0;
    int3_ original = (int3_)0;
    int3_ copies[3] = (int3_[3])0;
    int Number[2] = (int[2])0;
    int next[2] = (int[2])0;
    int Count = (int)0;
    int scratch[3] = (int[3])0;
    float pending[2] = (float[2])0;
    float3_ pending_vectors[2] = (float3_[2])0;
    int counter = (int)0;
    int ordered[3] = (int[3])0;
    int called[2] = (int[2])0;
    int zero_total = (int)0;
    int iteration = (int)0;
    int2 reset[2] = (int2[2])0;
    int total = (int)0;
    int iteration_1 = (int)0;
    int frozen[2] = (int[2])0;
    float signed_zeroes[2] = (float[2])0;
    int3_ from_calls[2] = (int3_[2])0;

    int _e4 = asint(out_.Load(int(0)*4));
    seed = _e4;
    int _e7 = seed;
    values[0] = _e7;
    int _e10 = seed;
    values[1] = asint(asuint(_e10) + asuint(int(1)));
    int _e14 = seed;
    values[2] = asint(asuint(_e14) + asuint(int(2)));
    copied[0] = int(4);
    copied[1] = int(5);
    zeroes[0] = int(0);
    zeroes[1] = int(0);
    flags[0] = true;
    flags[1] = false;
    flags[2] = true;
    counts[0] = 3u;
    counts[1] = 5u;
    samples[0] = 1.5;
    samples[1] = 2.5;
    int _e49 = values[int(0)];
    int _e50 = values[int(1)];
    int _e54 = values[int(2)];
    out_.Store(int(0)*4, asuint(asint(asuint(asint(asuint(_e49) + asuint(_e50))) + asuint(_e54))));
    int _e60 = copied[int(0)];
    out_.Store(int(1)*4, asuint(_e60));
    int _e67 = zeroes[int(0)];
    int _e68 = zeroes[int(1)];
    out_.Store(int(2)*4, asuint(asint(asuint(_e67) + asuint(_e68))));
    bool _e74 = flags[int(0)];
    bool _e78 = flags[int(1)];
    bool _e83 = flags[int(2)];
    out_.Store(int(3)*4, asuint(asint(asuint(asint(asuint(int(_e74)) + asuint(int(_e78)))) + asuint(int(_e83)))));
    uint _e92 = counts[int(0)];
    uint _e93 = counts[int(1)];
    out_.Store(int(4)*4, asuint(int((_e92 + _e93))));
    float _e102 = samples[int(0)];
    float _e103 = samples[int(1)];
    out_.Store(int(5)*4, asuint(naga_f2i32((_e102 + _e103))));
    self[0] = int(1);
    int _e113 = self[int(0)];
    self[1] = asint(asuint(_e113) + asuint(int(1)));
    int _e119 = self[int(1)];
    self[2] = asint(asuint(_e119) + asuint(int(1)));
    int _e126 = self[int(0)];
    int _e131 = self[int(1)];
    int _e136 = self[int(2)];
    out_.Store(int(6)*4, asuint(asint(asuint(asint(asuint(asint(asuint(_e126) * asuint(int(100)))) + asuint(asint(asuint(_e131) * asuint(int(10)))))) + asuint(_e136))));
    changed[0] = int(1);
    changed[int(0)] = int(7);
    changed[1] = int(2);
    int _e149 = changed[int(0)];
    changed[2] = _e149;
    int _e154 = changed[int(0)];
    out_.Store(int(7)*4, asuint(_e154));
    int _e159 = changed[int(2)];
    out_.Store(int(8)*4, asuint(_e159));
    points[0].x = int(1);
    int _e168 = points[int(0)].x;
    points[0].y = asint(asuint(_e168) + asuint(int(1)));
    int _e175 = points[int(0)].y;
    points[0].z = asint(asuint(_e175) + asuint(int(1)));
    int3_ _e181 = points[int(0)];
    points[1] = _e181;
    int _e187 = points[int(1)].z;
    out_.Store(int(9)*4, asuint(_e187));
    flat[0].x = int(1);
    flat[0].y = int(2);
    flat[1].x = int(3);
    flat[1].y = ZeroValueint();
    int _e204 = flat[int(1)].y;
    out_.Store(int(10)*4, asuint(_e204));
    int _e210 = flat[int(1)].x;
    out_.Store(int(11)*4, asuint(_e210));
    original.x = int(4);
    original.y = int(5);
    original.z = int(6);
    int3_ _e220 = original;
    copies[0] = _e220;
    int3_ _e222 = original;
    copies[1] = _e222;
    copies[2].x = int(7);
    copies[2].y = int(8);
    copies[2].z = ZeroValueint();
    original.x = int(99);
    int _e237 = copies[int(0)].x;
    out_.Store(int(12)*4, asuint(_e237));
    int _e243 = copies[int(1)].y;
    out_.Store(int(13)*4, asuint(_e243));
    int _e249 = copies[int(2)].z;
    out_.Store(int(14)*4, asuint(_e249));
    Number[0] = int(2);
    Number[1] = int(3);
    int _e259 = Number[int(1)];
    next[0] = _e259;
    int _e263 = Number[int(0)];
    next[1] = _e263;
    int _e268 = next[int(0)];
    out_.Store(int(15)*4, asuint(_e268));
    Count = int(3);
    scratch = ZeroValuearray3_int_();
    int _e277 = scratch[int(2)];
    out_.Store(int(16)*4, asuint(_e277));
    pending[0] = 1.0;
    pending[1] = float(int(2));
    float _e290 = pending[int(0)];
    float _e291 = pending[int(1)];
    out_.Store(int(17)*4, asuint(naga_f2i32((_e290 + _e291))));
    pending_vectors[0].x = 1.0;
    pending_vectors[0].y = float(int(2));
    pending_vectors[0].z = ZeroValuefloat();
    pending_vectors[1].x = 3.0;
    pending_vectors[1].y = ZeroValuefloat();
    pending_vectors[1].z = ZeroValuefloat();
    float _e318 = pending_vectors[int(0)].x;
    float _e319 = pending_vectors[int(0)].y;
    float _e324 = pending_vectors[int(1)].x;
    out_.Store(int(18)*4, asuint(naga_f2i32(((_e318 + _e319) + _e324))));
    int _e328 = seed;
    counter = _e328;
    int _e331 = counter;
    counter = asint(asuint(_e331) + asuint(int(1)));
    ordered[0] = _e331;
    int _e335 = counter;
    counter = asint(asuint(_e335) + asuint(int(1)));
    ordered[1] = _e335;
    int _e339 = counter;
    ordered[2] = _e339;
    int _e345 = ordered[int(0)];
    int _e350 = ordered[int(1)];
    int _e355 = ordered[int(2)];
    out_.Store(int(19)*4, asuint(asint(asuint(asint(asuint(asint(asuint(_e345) * asuint(int(100)))) + asuint(asint(asuint(_e350) * asuint(int(10)))))) + asuint(_e355))));
    int _e359 = counter;
    out_.Store(int(20)*4, asuint(_e359));
    int _e362 = counter;
    counter = asint(asuint(_e362) + asuint(int(1)));
    const int _e366 = pair(_e362, int(4));
    called[0] = _e366;
    int _e368 = counter;
    called[1] = _e368;
    int _e375 = called[int(0)];
    int _e376 = called[int(1)];
    out_.Store(int(21)*4, asuint(asint(asuint(_e375) + asuint(_e376))));
    int _e380 = counter;
    out_.Store(int(22)*4, asuint(_e380));
    zero_total = int(0);
    {
        iteration = int(0);
        uint2 loop_bound = uint2(4294967295u, 4294967295u);
        bool loop_init = true;
        while(true) {
            if (all(loop_bound == uint2(0u, 0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e388 = iteration;
                iteration = asint(asuint(_e388) + asuint(int(1)));
            }
            loop_init = false;
            int _e386 = iteration;
            if ((_e386 < int(3))) {
            } else {
                break;
            }
            int _e393 = iteration;
            reset[0].x = _e393;
            reset[0].y = ZeroValueint();
            reset[1] = ZeroValueint2();
            int _e405 = reset[int(0)].y;
            int _e406 = reset[int(1)].x;
            int _e408 = zero_total;
            zero_total = asint(asuint(_e408) + asuint(asint(asuint(_e405) + asuint(_e406))));
            reset[int(0)].y = int(9);
            reset[int(1)].x = int(8);
        }
    }
    int _e420 = zero_total;
    out_.Store(int(23)*4, asuint(_e420));
    total = int(0);
    {
        iteration_1 = int(0);
        uint2 loop_bound_1 = uint2(4294967295u, 4294967295u);
        bool loop_init_1 = true;
        while(true) {
            if (all(loop_bound_1 == uint2(0u, 0u))) { break; }
            loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
            if (!loop_init_1) {
                int _e428 = iteration_1;
                iteration_1 = asint(asuint(_e428) + asuint(int(1)));
            }
            loop_init_1 = false;
            int _e426 = iteration_1;
            if ((_e426 < int(4))) {
            } else {
                break;
            }
            int _e432 = iteration_1;
            if ((_e432 == int(1))) {
                continue;
            }
            int _e434 = iteration_1;
            int _e435 = total;
            total = asint(asuint(_e435) + asuint(_e434));
        }
    }
    int _e439 = total;
    out_.Store(int(24)*4, asuint(_e439));
    const int _e443 = sum(int(4));
    out_.Store(int(25)*4, asuint(_e443));
    frozen[0] = int(2);
    frozen[1] = int(3);
    int _e455 = frozen[int(0)];
    int _e456 = frozen[int(1)];
    out_.Store(int(26)*4, asuint(asint(asuint(_e455) + asuint(_e456))));
    signed_zeroes[0] = -(0.0);
    signed_zeroes[1] = 0.0;
    float _e468 = signed_zeroes[int(0)];
    out_.Store(int(27)*4, asuint(asint(_e468)));
    float _e474 = signed_zeroes[int(1)];
    out_.Store(int(28)*4, asuint(asint(_e474)));
    const int3_ _e479 = triple(int(6));
    from_calls[0] = _e479;
    const int3_ _e482 = triple(int(9));
    from_calls[1] = _e482;
    int _e491 = from_calls[int(0)].x;
    int _e492 = from_calls[int(1)].z;
    out_.Store(int(29)*4, asuint(asint(asuint(_e491) + asuint(_e492))));
}
