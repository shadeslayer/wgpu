struct float3_ {
    float x;
    float y;
    float z;
};

static const int Base_1 = int(2);
static const uint High = 2147483648u;
static const float Scale = 0.5;
static const int Second = int(3);
static const int After = int(4);

ByteAddressBuffer input : register(t0);
RWByteAddressBuffer out_ : register(u1);

typedef int ret_ZeroValuearray3_int_[3];
ret_ZeroValuearray3_int_ ZeroValuearray3_int_() {
    return (int[3])0;
}

int accumulate(int runtime_1)
{
    int runtime_2 = (int)0;
    int Size_1 = (int)0;
    int local_8 = (int)0;
    int local_9[3] = (int[3])0;
    int total = (int)0;
    int i_1 = (int)0;

    runtime_2 = runtime_1;
    Size_1 = int(3);
    local_9 = ZeroValuearray3_int_();
    total = int(0);
    {
        i_1 = int(0);
        uint2 loop_bound = uint2(4294967295u, 4294967295u);
        bool loop_init = true;
        while(true) {
            if (all(loop_bound == uint2(0u, 0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e13 = i_1;
                i_1 = asint(asuint(_e13) + asuint(int(1)));
            }
            loop_init = false;
            int _e10 = i_1;
            int _e11 = Size_1;
            if ((_e10 < _e11)) {
            } else {
                break;
            }
            int _e18 = runtime_2;
            int _e19 = i_1;
            int _e16 = i_1;
            local_9[min(uint(_e16), 2u)] = asint(asuint(_e18) + asuint(_e19));
            int _e21 = i_1;
            int _e23 = local_9[min(uint(_e21), 2u)];
            int _e24 = total;
            total = asint(asuint(_e24) + asuint(_e23));
        }
    }
    int _e26 = total;
    return _e26;
}

typedef int ret_ZeroValuearray5_int_[5];
ret_ZeroValuearray5_int_ ZeroValuearray5_int_() {
    return (int[5])0;
}

typedef float3_ ret_ZeroValuearray2_float3__[2];
ret_ZeroValuearray2_float3__ ZeroValuearray2_float3__() {
    return (float3_[2])0;
}

typedef int ret_ZeroValuearray2_int_[2];
ret_ZeroValuearray2_int_ ZeroValuearray2_int_() {
    return (int[2])0;
}

uint naga_f2u32(float value) {
    return uint(clamp(value, 0.0, 4294967000.0));
}

[numthreads(1, 1, 1)]
void constant_expressions()
{
    int runtime = (int)0;
    int Chosen = (int)0;
    int local = (int)0;
    int Size = (int)0;
    int values[5] = (int[5])0;
    int SignedHigh = (int)0;
    uint local_1 = (uint)0;
    int Negative = (int)0;
    uint local_2 = (uint)0;
    uint All = (uint)0;
    uint Wrapped = (uint)0;
    uint local_3 = (uint)0;
    uint TruncatedZero = (uint)0;
    int Minimum = (int)0;
    uint Large = (uint)0;
    bool Ready = (bool)0;
    int Comma = (int)0;
    int Lazy = (int)0;
    int local_4 = (int)0;
    bool Skipped = (bool)0;
    bool local_5 = (bool)0;
    bool Yes = (bool)0;
    bool local_6 = (bool)0;
    int VectorCount = (int)0;
    float3_ vectors[2] = (float3_[2])0;
    int Base[2] = (int[2])0;
    int i = (int)0;
    int Count = (int)0;
    int scratch[3] = (int[3])0;
    int Safe = (int)0;
    int local_7 = (int)0;

    int _e5 = asint(input.Load(int(0)*4));
    runtime = _e5;
    Chosen = int(3);
    Size = int(4);
    values = ZeroValuearray5_int_();
    int _e14 = runtime;
    values[int(0)] = _e14;
    int _e19 = asint(input.Load(int(1)*4));
    int _e15 = Size;
    values[min(uint(_e15), 4u)] = _e19;
    out_.Store(int(0)*4, asuint(uint(Base_1)));
    out_.Store(int(1)*4, asuint(High));
    out_.Store(int(2)*4, asuint(asuint(Scale)));
    int _e33 = Chosen;
    out_.Store(int(3)*4, asuint(uint(_e33)));
    int _e37 = runtime;
    const int _e38 = accumulate(_e37);
    out_.Store(int(4)*4, asuint(uint(_e38)));
    int _e42 = Size;
    int _e44 = values[min(uint(_e42), 4u)];
    out_.Store(int(5)*4, asuint(uint(_e44)));
    int _e50 = values[int(1)];
    out_.Store(int(6)*4, asuint(uint(_e50)));
    int _e54 = runtime;
    out_.Store(int(7)*4, asuint(naga_f2u32((float(_e54) * Scale))));
    SignedHigh = int(-2147483647 - 1);
    Negative = int(-2);
    All = 4294967295u;
    Wrapped = 0u;
    int _e69 = SignedHigh;
    out_.Store(int(8)*4, asuint(uint(_e69)));
    int _e73 = Negative;
    out_.Store(int(9)*4, asuint(uint(_e73)));
    uint _e77 = All;
    out_.Store(int(10)*4, asuint(_e77));
    uint _e80 = Wrapped;
    out_.Store(int(11)*4, asuint(_e80));
    TruncatedZero = 0u;
    Minimum = int(-2147483647 - 1);
    Large = 4294967040u;
    Ready = true;
    uint _e91 = TruncatedZero;
    out_.Store(int(12)*4, asuint(_e91));
    int _e94 = Minimum;
    out_.Store(int(13)*4, asuint(uint(_e94)));
    uint _e98 = Large;
    out_.Store(int(14)*4, asuint(_e98));
    bool _e101 = Ready;
    out_.Store(int(15)*4, asuint(uint(_e101)));
    Comma = int(4);
    Lazy = int(6);
    int _e109 = Comma;
    out_.Store(int(16)*4, asuint(uint(_e109)));
    int _e113 = Lazy;
    out_.Store(int(17)*4, asuint(uint(_e113)));
    int _e117 = runtime;
    out_.Store(int(18)*4, asuint(uint(_e117)));
    Skipped = false;
    Yes = true;
    bool _e125 = Skipped;
    out_.Store(int(19)*4, asuint(uint(_e125)));
    bool _e129 = Yes;
    out_.Store(int(20)*4, asuint(uint(_e129)));
    int _e133 = runtime;
    out_.Store(int(21)*4, asuint(uint(_e133)));
    VectorCount = int(2);
    vectors = ZeroValuearray2_float3__();
    int _e144 = asint(input.Load(int(1)*4));
    vectors[int(1)].y = float(_e144);
    float _e151 = vectors[int(1)].y;
    out_.Store(int(22)*4, asuint(naga_f2u32(_e151)));
    {
        Base = ZeroValuearray2_int_();
        int _e159 = Base[int(1)];
        out_.Store(int(23)*4, asuint(uint(_e159)));
    }
    {
        i = int(0);
        uint2 loop_bound_1 = uint2(4294967295u, 4294967295u);
        bool loop_init_1 = true;
        while(true) {
            if (all(loop_bound_1 == uint2(0u, 0u))) { break; }
            loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
            if (!loop_init_1) {
                int _e166 = i;
                i = asint(asuint(_e166) + asuint(int(1)));
            }
            loop_init_1 = false;
            int _e164 = i;
            if ((_e164 < int(3))) {
            } else {
                break;
            }
            Count = int(3);
            scratch = ZeroValuearray3_int_();
            int _e179 = scratch[int(2)];
            int _e174 = i;
            out_.Store(asint(asuint(int(24)) + asuint(_e174))*4, asuint(uint(_e179)));
            int _e183 = runtime;
            int _e184 = i;
            scratch[int(2)] = asint(asuint(_e183) + asuint(_e184));
        }
    }
    int _e190 = asint(input.Load(int(1)*4));
    const int _e191 = accumulate(_e190);
    out_.Store(int(27)*4, asuint(uint(_e191)));
    out_.Store(int(28)*4, asuint(uint(Second)));
    Safe = int(3);
    int _e203 = runtime;
    int _e204 = asint(input.Load(int(1)*4));
    int _e206 = Safe;
    out_.Store(int(29)*4, asuint(uint(asint(asuint(asint(asuint(asint(asuint(_e203) + asuint(_e204))) + asuint(_e206))) - asuint(int(3))))));
}
