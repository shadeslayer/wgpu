RWByteAddressBuffer out_ : register(u0);

typedef int ret_ZeroValuearray3_int_[3];
ret_ZeroValuearray3_int_ ZeroValuearray3_int_() {
    return (int[3])0;
}

int local_sum(int value)
{
    int value_1 = (int)0;
    int values[3] = (int[3])0;

    value_1 = value;
    values = ZeroValuearray3_int_();
    int _e6 = value_1;
    values[int(0)] = _e6;
    int _e10 = value_1;
    values[int(1)] = asint(asuint(_e10) + asuint(int(1)));
    int _e18 = values[int(0)];
    int _e19 = values[int(1)];
    values[int(2)] = asint(asuint(_e18) + asuint(_e19));
    int _e23 = values[int(2)];
    return _e23;
}

typedef int ret_ZeroValuearray2_int_[2];
ret_ZeroValuearray2_int_ ZeroValuearray2_int_() {
    return (int[2])0;
}

int reset_sum(int count)
{
    int count_1 = (int)0;
    int result = (int)0;
    int i_1 = (int)0;
    int values_1[2] = (int[2])0;

    count_1 = count;
    result = int(0);
    {
        i_1 = int(0);
        uint2 loop_bound = uint2(4294967295u, 4294967295u);
        bool loop_init = true;
        while(true) {
            if (all(loop_bound == uint2(0u, 0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e9 = i_1;
                i_1 = asint(asuint(_e9) + asuint(int(1)));
            }
            loop_init = false;
            int _e6 = i_1;
            int _e7 = count_1;
            if ((_e6 < _e7)) {
            } else {
                break;
            }
            values_1 = ZeroValuearray2_int_();
            int _e16 = i_1;
            values_1[int(1)] = _e16;
            int _e21 = values_1[int(0)];
            int _e22 = values_1[int(1)];
            int _e24 = result;
            result = asint(asuint(_e24) + asuint(asint(asuint(_e21) + asuint(_e22))));
            values_1[int(0)] = int(99);
        }
    }
    int _e29 = result;
    return _e29;
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

int naga_mod(int lhs, int rhs) {
    int divisor = ((lhs == int(-2147483647 - 1) & rhs == -1) | (rhs == 0)) ? 1 : rhs;
    return lhs - (lhs / divisor) * divisor;
}

typedef uint ret_ZeroValuearray2_uint_[2];
ret_ZeroValuearray2_uint_ ZeroValuearray2_uint_() {
    return (uint[2])0;
}

typedef float ret_ZeroValuearray2_float_[2];
ret_ZeroValuearray2_float_ ZeroValuearray2_float_() {
    return (float[2])0;
}

typedef bool ret_ZeroValuearray3_bool_[3];
ret_ZeroValuearray3_bool_ ZeroValuearray3_bool_() {
    return (bool[3])0;
}

typedef bool ret_ZeroValuearray2_bool_[2];
ret_ZeroValuearray2_bool_ ZeroValuearray2_bool_() {
    return (bool[2])0;
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

[numthreads(1, 1, 1)]
void local_arrays()
{
    int seed = (int)0;
    int signed_values[3] = (int[3])0;
    uint words[2] = (uint[2])0;
    float samples[2] = (float[2])0;
    bool flags[3] = (bool[3])0;
    int zeros[2] = (int[2])0;
    bool zero_flags[2] = (bool[2])0;
    bool local = (bool)0;
    uint bank[2] = (uint[2])0;
    float bank_1[2] = (float[2])0;
    int signed_values_1[2] = (int[2])0;
    int mixed[2] = (int[2])0;
    int next = (int)0;
    int reset_total = (int)0;
    int i = (int)0;
    int reset[2] = (int[2])0;
    int total = (int)0;
    int scratch[2] = (int[2])0;
    int j = (int)0;
    float zero_samples[2] = (float[2])0;
    int left[2] = (int[2])0;
    int right[2] = (int[2])0;
    int temporary[2] = (int[2])0;
    int temporary_1[2] = (int[2])0;
    bool choose = (bool)0;
    int index = (int)0;
    int rhs = (int)0;
    int local_1 = (int)0;
    int local_2 = (int)0;

    int _e4 = asint(out_.Load(int(0)*4));
    seed = _e4;
    int _e8 = seed;
    signed_values[int(0)] = _e8;
    signed_values[int(1)] = naga_neg(int(2));
    signed_values[int(2)] = int(4);
    int _e22 = signed_values[int(0)];
    int _e23 = signed_values[int(1)];
    int _e27 = signed_values[int(2)];
    out_.Store(int(0)*4, asuint(asint(asuint(asint(asuint(_e22) + asuint(_e23))) + asuint(_e27))));
    words = ZeroValuearray2_uint_();
    words[int(0)] = 4294967295u;
    words[int(1)] = 1u;
    uint _e43 = words[int(0)];
    uint _e44 = words[int(1)];
    out_.Store(int(1)*4, asuint(int(((_e43 + _e44) == 0u))));
    samples = ZeroValuearray2_float_();
    samples[int(0)] = 1.5;
    samples[int(1)] = 2.25;
    float _e63 = samples[int(0)];
    float _e64 = samples[int(1)];
    out_.Store(int(2)*4, asuint(naga_f2i32(((_e63 + _e64) * 4.0))));
    flags = ZeroValuearray3_bool_();
    int _e74 = seed;
    flags[int(0)] = (_e74 != int(0));
    flags[int(2)] = true;
    bool _e83 = flags[int(0)];
    bool _e89 = flags[int(1)];
    bool _e96 = flags[int(2)];
    out_.Store(int(3)*4, asuint(asint(asuint(asint(asuint(asint(asuint(int(_e83)) * asuint(int(100)))) + asuint(asint(asuint(int(_e89)) * asuint(int(10)))))) + asuint(int(_e96)))));
    zeros = ZeroValuearray2_int_();
    int _e107 = zeros[int(0)];
    int _e108 = zeros[int(1)];
    out_.Store(int(4)*4, asuint(asint(asuint(_e107) + asuint(_e108))));
    int _e112 = seed;
    const int _e113 = local_sum(_e112);
    out_.Store(int(5)*4, asuint(_e113));
    const int _e117 = reset_sum(int(3));
    out_.Store(int(6)*4, asuint(_e117));
    zero_flags = ZeroValuearray2_bool_();
    bool _e124 = zero_flags[int(0)];
    local = _e124;
    if (_e124) {
    } else {
        bool _e128 = zero_flags[int(1)];
        local = _e128;
    }
    bool _e129 = local;
    out_.Store(int(7)*4, asuint(int(_e129)));
    bank = ZeroValuearray2_uint_();
    bank[int(0)] = 3u;
    bank[int(1)] = 4u;
    {
        bank_1 = ZeroValuearray2_float_();
        bank_1[int(1)] = 6.5;
        float _e150 = bank_1[int(0)];
        float _e151 = bank_1[int(1)];
        out_.Store(int(8)*4, asuint(naga_f2i32((_e150 + _e151))));
    }
    uint _e160 = bank[int(0)];
    uint _e161 = bank[int(1)];
    out_.Store(int(9)*4, asuint(int((_e160 + _e161))));
    {
        int _e167 = seed;
        signed_values_1[int(0)] = _e167;
        int _e170 = seed;
        signed_values_1[int(1)] = _e170;
        int _e177 = signed_values_1[int(0)];
        int _e178 = signed_values_1[int(1)];
        out_.Store(int(10)*4, asuint(asint(asuint(_e177) + asuint(_e178))));
    }
    int _e184 = signed_values[int(0)];
    out_.Store(int(11)*4, asuint(_e184));
    mixed = ZeroValuearray2_int_();
    int _e188 = seed;
    seed = asint(asuint(_e188) + asuint(int(1)));
    int _e191 = seed;
    next = _e191;
    int _e194 = next;
    mixed[int(0)] = _e194;
    int _e197 = seed;
    seed = asint(asuint(_e197) + asuint(int(1)));
    mixed[int(1)] = _e197;
    int _e205 = mixed[int(0)];
    int _e209 = mixed[int(1)];
    out_.Store(int(12)*4, asuint(asint(asuint(asint(asuint(_e205) * asuint(int(10)))) + asuint(_e209))));
    int _e213 = seed;
    out_.Store(int(13)*4, asuint(_e213));
    reset_total = int(0);
    {
        i = int(0);
        uint2 loop_bound_1 = uint2(4294967295u, 4294967295u);
        bool loop_init_1 = true;
        while(true) {
            if (all(loop_bound_1 == uint2(0u, 0u))) { break; }
            loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
            if (!loop_init_1) {
                int _e221 = i;
                i = asint(asuint(_e221) + asuint(int(1)));
            }
            loop_init_1 = false;
            int _e219 = i;
            if ((_e219 < int(3))) {
            } else {
                break;
            }
            reset = ZeroValuearray2_int_();
            int _e227 = i;
            if ((_e227 == int(1))) {
                reset[int(0)] = int(90);
                continue;
            }
            int _e234 = reset[int(0)];
            int _e235 = i;
            int _e237 = reset_total;
            reset_total = asint(asuint(_e237) + asuint(asint(asuint(_e234) + asuint(_e235))));
            reset[int(0)] = int(99);
        }
    }
    int _e244 = reset_total;
    out_.Store(int(14)*4, asuint(_e244));
    total = int(0);
    {
        scratch = ZeroValuearray2_int_();
        j = int(0);
        uint2 loop_bound_2 = uint2(4294967295u, 4294967295u);
        bool loop_init_2 = true;
        while(true) {
            if (all(loop_bound_2 == uint2(0u, 0u))) { break; }
            loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
            if (!loop_init_2) {
                int _e254 = j;
                j = asint(asuint(_e254) + asuint(int(1)));
            }
            loop_init_2 = false;
            int _e252 = j;
            if ((_e252 < int(3))) {
            } else {
                break;
            }
            int _e258 = j;
            int _e262 = scratch[min(uint(naga_mod(_e258, int(2))), 1u)];
            scratch[min(uint(naga_mod(_e258, int(2))), 1u)] = asint(asuint(_e262) + asuint(int(1)));
            int _e265 = j;
            if ((_e265 == int(1))) {
                continue;
            }
            int _e270 = scratch[int(0)];
            int _e274 = scratch[int(1)];
            int _e276 = total;
            total = asint(asuint(_e276) + asuint(asint(asuint(asint(asuint(_e270) * asuint(int(10)))) + asuint(_e274))));
        }
    }
    int _e280 = total;
    out_.Store(int(15)*4, asuint(_e280));
    zero_samples = ZeroValuearray2_float_();
    float _e289 = zero_samples[int(0)];
    float _e290 = zero_samples[int(1)];
    out_.Store(int(16)*4, asuint(naga_f2i32((_e289 + _e290))));
    left = ZeroValuearray2_int_();
    right = ZeroValuearray2_int_();
    left[int(0)] = int(5);
    left[int(1)] = int(7);
    right[int(0)] = int(11);
    right[int(1)] = int(13);
    int _e315 = left[int(0)];
    int _e316 = right[int(0)];
    out_.Store(int(17)*4, asuint(asint(asuint(_e315) + asuint(_e316))));
    {
        temporary = ZeroValuearray2_int_();
        int _e322 = seed;
        temporary[int(0)] = _e322;
        {
            temporary_1 = ZeroValuearray2_int_();
            temporary_1[int(1)] = int(3);
            int _e334 = temporary_1[int(0)];
            int _e335 = temporary_1[int(1)];
            out_.Store(int(18)*4, asuint(asint(asuint(_e334) + asuint(_e335))));
        }
        int _e341 = temporary[int(0)];
        out_.Store(int(19)*4, asuint(_e341));
    }
    int _e344 = seed;
    choose = ((_e344 & int(1)) == int(0));
    index = int(0);
    rhs = int(20);
    int _e363 = rhs;
    rhs = asint(asuint(_e363) + asuint(int(1)));
    int _e366 = rhs;
    bool _e354 = choose;
    if (_e354) {
        int _e355 = index;
        index = asint(asuint(_e355) + asuint(int(1)));
        int _e367 = left[min(uint(_e355), 1u)];
        left[min(uint(_e355), 1u)] = asint(asuint(_e367) + asuint(_e366));
        int _e372 = left[min(uint(_e355), 1u)];
        local_1 = _e372;
    } else {
        int _e359 = index;
        index = asint(asuint(_e359) + asuint(int(1)));
        int _e369 = right[min(uint(_e359), 1u)];
        right[min(uint(_e359), 1u)] = asint(asuint(_e369) + asuint(_e366));
        int _e373 = right[min(uint(_e359), 1u)];
        local_1 = _e373;
    }
    int _e374 = local_1;
    out_.Store(int(20)*4, asuint(_e374));
    int _e377 = index;
    out_.Store(int(21)*4, asuint(_e377));
    int _e383 = left[int(0)];
    int _e387 = right[int(0)];
    out_.Store(int(22)*4, asuint(asint(asuint(asint(asuint(_e383) * asuint(int(100)))) + asuint(_e387))));
    bool _e391 = choose;
    if (_e391) {
        right[int(1)] = int(9);
        int _e398 = right[int(1)];
        right[int(1)] = asint(asuint(_e398) + asuint(int(1)));
        local_2 = _e398;
    } else {
        left[int(1)] = int(9);
        int _e401 = left[int(1)];
        left[int(1)] = asint(asuint(_e401) + asuint(int(1)));
        local_2 = _e401;
    }
    int _e404 = local_2;
    out_.Store(int(23)*4, asuint(_e404));
    int _e410 = left[int(1)];
    int _e414 = right[int(1)];
    out_.Store(int(24)*4, asuint(asint(asuint(asint(asuint(_e410) * asuint(int(100)))) + asuint(_e414))));
}
