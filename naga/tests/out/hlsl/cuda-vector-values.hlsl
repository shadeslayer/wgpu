struct uint3_ {
    uint x;
    uint y;
    uint z;
};

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

int choose(int2 value)
{
    int2 value_1 = (int2)0;

    value_1 = value;
    int _e4 = value_1.x;
    int _e7 = value_1.y;
    return asint(asuint(asint(asuint(_e4) * asuint(int(10)))) + asuint(_e7));
}

int choose_1(uint3_ value_2)
{
    uint3_ value_3 = (uint3_)0;

    value_3 = value_2;
    uint _e3 = value_3.z;
    return int(_e3);
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

int choose_2(float4 value_4)
{
    float4 value_5 = (float4)0;

    value_5 = value_4;
    float _e3 = value_5.w;
    return naga_f2i32(_e3);
}

int3_ bump(int3_ value_6)
{
    int3_ value_7 = (int3_)0;

    value_7 = value_6;
    int _e4 = value_7.y;
    value_7.y = asint(asuint(_e4) + asuint(int(1)));
    int3_ _e6 = value_7;
    const int3_ int3_1 = _e6;
    return int3_1;
}

float3_ shift(float3_ value_8)
{
    float3_ value_9 = (float3_)0;

    value_9 = value_8;
    float _e4 = value_9.x;
    value_9.x = (_e4 + 2.0);
    float _e9 = value_9.x;
    float _e10 = value_9.y;
    value_9.z = (_e9 + _e10);
    float3_ _e12 = value_9;
    const float3_ float3_1 = _e12;
    return float3_1;
}

int pair(float3_ first, int second)
{
    float3_ first_1 = (float3_)0;
    int second_1 = (int)0;

    first_1 = first;
    second_1 = second;
    float _e5 = first_1.x;
    int _e9 = second_1;
    return asint(asuint(asint(asuint(naga_f2i32(_e5)) * asuint(int(10)))) + asuint(_e9));
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

float3_ Constructfloat3_(float arg0, float arg1, float arg2) {
    float3_ ret = (float3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

int3_ Constructint3_(int arg0, int arg1, int arg2) {
    int3_ ret = (int3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

uint3_ Constructuint3_(uint arg0, uint arg1, uint arg2) {
    uint3_ ret = (uint3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

[numthreads(1, 1, 1)]
void vector_values()
{
    int seed = (int)0;
    float2 f2_ = (float2)0;
    float3_ f3_ = (float3_)0;
    float4 f4_ = (float4)0;
    int2 i2_ = (int2)0;
    int3_ i3_ = (int3_)0;
    int4 i4_ = (int4)0;
    uint2 u2_ = (uint2)0;
    uint3_ u3_ = (uint3_)0;
    uint4 u4_ = (uint4)0;
    float3_ frozen = (float3_)0;
    float3_ copied = (float3_)0;
    int3_ changed = (int3_)0;
    float3_ samples[2] = (float3_[2])0;
    int index = (int)0;
    bool choose_first = (bool)0;
    float local = (float)0;
    float local_1 = (float)0;
    float3_ local_2 = (float3_)0;
    int local_3 = (int)0;
    int argument = (int)0;
    int3_ captured = (int3_)0;

    int _e4 = asint(out_.Load(int(0)*4));
    seed = _e4;
    int _e6 = seed;
    f2_ = float2(float(_e6), 1.5);
    f3_ = Constructfloat3_(1.0, 2.0, 3.0);
    f4_ = float4(float(int(1)), float(int(2)), float(int(3)), float(int(4)));
    int _e26 = seed;
    i2_ = int2(_e26, naga_neg(int(2)));
    i3_ = Constructint3_(int(1), int(2), int(3));
    i4_ = int4(int(1), int(2), int(3), int(4));
    u2_ = uint2(uint(naga_neg(int(1))), uint(int(2)));
    u3_ = Constructuint3_(uint(int(1)), uint(int(2)), uint(int(3)));
    u4_ = uint4(uint(int(1)), uint(int(2)), uint(int(3)), uint(int(4)));
    float _e70 = f2_.x;
    float _e71 = f2_.y;
    out_.Store(int(0)*4, asuint(naga_f2i32((_e70 + _e71))));
    float _e78 = f3_.x;
    float _e79 = f3_.y;
    float _e82 = f3_.z;
    out_.Store(int(1)*4, asuint(naga_f2i32(((_e78 + _e79) + _e82))));
    float _e88 = f4_.w;
    out_.Store(int(2)*4, asuint(naga_f2i32(_e88)));
    int _e94 = i2_.x;
    int _e95 = i2_.y;
    out_.Store(int(3)*4, asuint(asint(asuint(_e94) + asuint(_e95))));
    int _e100 = i3_.z;
    out_.Store(int(4)*4, asuint(_e100));
    int _e105 = i4_.x;
    int _e106 = i4_.w;
    out_.Store(int(5)*4, asuint(asint(asuint(_e105) + asuint(_e106))));
    uint _e112 = u2_.x;
    out_.Store(int(6)*4, asuint(int((_e112 == 4294967295u))));
    uint _e119 = u3_.x;
    uint _e120 = u3_.z;
    out_.Store(int(7)*4, asuint(int((_e119 + _e120))));
    uint _e126 = u4_.w;
    out_.Store(int(8)*4, asuint(int(_e126)));
    float3_ _e129 = f3_;
    frozen = _e129;
    float3_ _e131 = frozen;
    copied = _e131;
    copied.x = 9.0;
    float _e138 = frozen.x;
    float _e141 = copied.x;
    out_.Store(int(9)*4, asuint(naga_f2i32(((_e138 * 10.0) + _e141))));
    int3_ _e145 = i3_;
    const int3_ _e146 = bump(_e145);
    changed = _e146;
    int _e151 = changed.y;
    int _e154 = i3_.y;
    out_.Store(int(10)*4, asuint(asint(asuint(asint(asuint(_e151) * asuint(int(10)))) + asuint(_e154))));
    float3_ _e158 = f3_;
    const float3_ _e159 = shift(_e158);
    out_.Store(int(11)*4, asuint(naga_f2i32(_e159.z)));
    int2 _e164 = i2_;
    const int _e165 = choose(_e164);
    uint3_ _e166 = u3_;
    const int _e167 = choose_1(_e166);
    float4 _e169 = f4_;
    const int _e170 = choose_2(_e169);
    out_.Store(int(12)*4, asuint(asint(asuint(asint(asuint(_e165) + asuint(_e167))) + asuint(_e170))));
    float3_ _e175 = f3_;
    samples[int(0)] = _e175;
    samples[int(1)] = Constructfloat3_(float(int(5)), float(int(6)), float(int(7)));
    index = int(0);
    int _e194 = index;
    index = asint(asuint(_e194) + asuint(int(1)));
    int _e197 = index;
    int _e189 = index;
    index = asint(asuint(_e189) + asuint(int(1)));
    float _e198 = samples[min(uint(_e189), 1u)].x;
    samples[min(uint(_e189), 1u)].x = (_e198 + float(_e197));
    float _e201 = samples[min(uint(_e189), 1u)].x;
    out_.Store(int(13)*4, asuint(naga_f2i32(_e201)));
    int _e206 = index;
    float _e211 = samples[int(0)].x;
    float _e219 = samples[int(1)].x;
    out_.Store(int(14)*4, asuint(asint(asuint(asint(asuint(asint(asuint(_e206) * asuint(int(100)))) + asuint(asint(asuint(naga_f2i32(_e211)) * asuint(int(10)))))) + asuint(naga_f2i32(_e219)))));
    choose_first = false;
    bool _e226 = choose_first;
    if (_e226) {
        float _e234 = samples[int(0)].y;
        samples[int(0)].y = (_e234 + float(int(1)));
        local = _e234;
    } else {
        float _e238 = samples[int(1)].y;
        samples[int(1)].y = (_e238 + float(int(1)));
        local = _e238;
    }
    float _e242 = local;
    out_.Store(int(15)*4, asuint(naga_f2i32(_e242)));
    float _e249 = samples[int(1)].y;
    out_.Store(int(16)*4, asuint(naga_f2i32(_e249)));
    float3_ _e258 = f3_;
    bool _e253 = choose_first;
    if (_e253) {
        samples[int(0)] = _e258;
        float _e262 = samples[int(0)].z;
        local_1 = _e262;
    } else {
        samples[int(1)] = _e258;
        float _e263 = samples[int(1)].z;
        local_1 = _e263;
    }
    float _e264 = local_1;
    out_.Store(int(17)*4, asuint(naga_f2i32(_e264)));
    if (true) {
        local_2 = Constructfloat3_(float(int(7)), float(int(8)), float(int(9)));
    } else {
        float3_ _e277 = f3_;
        local_2 = _e277;
    }
    float3_ _e278 = local_2;
    out_.Store(int(18)*4, asuint(naga_f2i32(_e278.y)));
    {
        int _e283 = seed;
        local_3 = asint(asuint(_e283) + asuint(int(2)));
        int _e287 = local_3;
        out_.Store(int(19)*4, asuint(_e287));
    }
    argument = int(3);
    int _e291 = argument;
    int _e292 = argument;
    argument = asint(asuint(_e292) + asuint(int(1)));
    int _e295 = argument;
    captured = Constructint3_(_e291, _e292, _e295);
    int _e301 = captured.x;
    int _e305 = captured.y;
    int _e309 = captured.z;
    out_.Store(int(20)*4, asuint(asint(asuint(asint(asuint(asint(asuint(_e301) * asuint(int(100)))) + asuint(asint(asuint(_e305) * asuint(int(10)))))) + asuint(_e309))));
    int _e313 = argument;
    out_.Store(int(21)*4, asuint(_e313));
    float3_ _e316 = f3_;
    f3_.x = 9.0;
    const int _e320 = pair(_e316, int(4));
    out_.Store(int(22)*4, asuint(_e320));
    float _e324 = f3_.x;
    out_.Store(int(23)*4, asuint(naga_f2i32(_e324)));
}
