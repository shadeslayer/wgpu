struct float3_ {
    float x;
    float y;
    float z;
};

RWByteAddressBuffer out_ : register(u0);

float2 fabs(float2 value_1)
{
    float2 value_2 = (float2)0;
    uint local_23 = (uint)0;
    uint local_24 = (uint)0;

    value_2 = value_1;
    float _e3 = value_2.x;
    uint _e6 = (asuint(_e3) & 2147483647u);
    local_23 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_23;
    float _e15 = value_2.y;
    uint _e18 = (asuint(_e15) & 2147483647u);
    local_24 = ((_e18 > 2139095040u) ? 2143289344u : _e18);
    uint _e24 = local_24;
    return float2(asfloat(_e12), asfloat(_e24));
}

float3_ Constructfloat3_(float arg0, float arg1, float arg2) {
    float3_ ret = (float3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

float3_ fabs_1(float3_ value_3)
{
    float3_ value_4 = (float3_)0;
    uint local_25 = (uint)0;
    uint local_26 = (uint)0;
    uint local_27 = (uint)0;

    value_4 = value_3;
    float _e3 = value_4.x;
    uint _e6 = (asuint(_e3) & 2147483647u);
    local_25 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_25;
    float _e15 = value_4.y;
    uint _e18 = (asuint(_e15) & 2147483647u);
    local_26 = ((_e18 > 2139095040u) ? 2143289344u : _e18);
    uint _e24 = local_26;
    float _e27 = value_4.z;
    uint _e30 = (asuint(_e27) & 2147483647u);
    local_27 = ((_e30 > 2139095040u) ? 2143289344u : _e30);
    uint _e36 = local_27;
    const float3_ float3_1 = Constructfloat3_(asfloat(_e12), asfloat(_e24), asfloat(_e36));
    return float3_1;
}

float4 fabs_2(float4 value_5)
{
    float4 value_6 = (float4)0;
    uint local_28 = (uint)0;
    uint local_29 = (uint)0;
    uint local_30 = (uint)0;
    uint local_31 = (uint)0;

    value_6 = value_5;
    float _e3 = value_6.x;
    uint _e6 = (asuint(_e3) & 2147483647u);
    local_28 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_28;
    float _e15 = value_6.y;
    uint _e18 = (asuint(_e15) & 2147483647u);
    local_29 = ((_e18 > 2139095040u) ? 2143289344u : _e18);
    uint _e24 = local_29;
    float _e27 = value_6.z;
    uint _e30 = (asuint(_e27) & 2147483647u);
    local_30 = ((_e30 > 2139095040u) ? 2143289344u : _e30);
    uint _e36 = local_30;
    float _e39 = value_6.w;
    uint _e42 = (asuint(_e39) & 2147483647u);
    local_31 = ((_e42 > 2139095040u) ? 2143289344u : _e42);
    uint _e48 = local_31;
    return float4(asfloat(_e12), asfloat(_e24), asfloat(_e36), asfloat(_e48));
}

float absolute(float value_7)
{
    float value_8 = (float)0;
    uint local_32 = (uint)0;

    value_8 = value_7;
    float _e2 = value_8;
    uint _e5 = (asuint(_e2) & 2147483647u);
    local_32 = ((_e5 > 2139095040u) ? 2143289344u : _e5);
    uint _e11 = local_32;
    return asfloat(_e11);
}

float converted_absolute(int value_9)
{
    int value_10 = (int)0;
    uint local_33 = (uint)0;

    value_10 = value_9;
    int _e2 = value_10;
    uint _e6 = (asuint(float(_e2)) & 2147483647u);
    local_33 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_33;
    return asfloat(_e12);
}

float converted_absolute_1(uint value_11)
{
    uint value_12 = (uint)0;
    uint local_34 = (uint)0;

    value_12 = value_11;
    uint _e2 = value_12;
    uint _e6 = (asuint(float(_e2)) & 2147483647u);
    local_34 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_34;
    return asfloat(_e12);
}

float converted_absolute_2(bool value_13)
{
    bool value_14 = (bool)0;
    uint local_35 = (uint)0;

    value_14 = value_13;
    bool _e2 = value_14;
    uint _e6 = (asuint(float(_e2)) & 2147483647u);
    local_35 = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local_35;
    return asfloat(_e12);
}

int fabs_3(int value_15)
{
    int value_16 = (int)0;

    value_16 = value_15;
    int _e3 = value_16;
    return asint(asuint(_e3) + asuint(int(40)));
}

uint fabs_4(uint value_17)
{
    uint value_18 = (uint)0;

    value_18 = value_17;
    uint _e3 = value_18;
    return (_e3 + 50u);
}

int fabs_5(bool value_19)
{
    bool value_20 = (bool)0;
    int local_36 = (int)0;

    value_20 = value_19;
    bool _e2 = value_20;
    if (_e2) {
        local_36 = int(7);
    } else {
        local_36 = int(9);
    }
    int _e6 = local_36;
    return _e6;
}

int fabsf(bool value_21)
{
    bool value_22 = (bool)0;
    int local_37 = (int)0;

    value_22 = value_21;
    bool _e2 = value_22;
    if (_e2) {
        local_37 = int(11);
    } else {
        local_37 = int(13);
    }
    int _e6 = local_37;
    return _e6;
}

float fabsf_1(float first, float second)
{
    float first_1 = (float)0;
    float second_1 = (float)0;

    first_1 = first;
    second_1 = second;
    float _e4 = first_1;
    float _e5 = second_1;
    return (_e4 + _e5);
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

[numthreads(1, 1, 1)]
void absolutes()
{
    uint local = (uint)0;
    uint local_1 = (uint)0;
    uint local_2 = (uint)0;
    uint local_3 = (uint)0;
    uint local_4 = (uint)0;
    uint local_5 = (uint)0;
    uint local_6 = (uint)0;
    uint local_7 = (uint)0;
    uint local_8 = (uint)0;
    uint local_9 = (uint)0;
    uint local_10 = (uint)0;
    uint local_11 = (uint)0;
    uint local_12 = (uint)0;
    uint local_13 = (uint)0;
    uint local_14 = (uint)0;
    uint local_15 = (uint)0;
    uint local_16 = (uint)0;
    uint local_17 = (uint)0;
    uint local_18 = (uint)0;
    uint local_19 = (uint)0;
    uint local_20 = (uint)0;
    float value = (float)0;
    uint local_21 = (uint)0;
    float3_ triple = (float3_)0;
    uint local_22 = (uint)0;
    float2 pair = (float2)0;
    float4 quadruple = (float4)0;

    uint _e6 = (asuint(0.0) & 2147483647u);
    local = ((_e6 > 2139095040u) ? 2143289344u : _e6);
    uint _e12 = local;
    out_.Store(int(0)*4, asuint(asuint(asfloat(_e12))));
    uint _e21 = (asuint(-(0.0)) & 2147483647u);
    local_1 = ((_e21 > 2139095040u) ? 2143289344u : _e21);
    uint _e27 = local_1;
    out_.Store(int(1)*4, asuint(asuint(asfloat(_e27))));
    uint _e36 = (asuint(-(1.75)) & 2147483647u);
    local_2 = ((_e36 > 2139095040u) ? 2143289344u : _e36);
    uint _e42 = local_2;
    out_.Store(int(2)*4, asuint(asuint(asfloat(_e42))));
    const float _e49 = absolute(-(1.25));
    out_.Store(int(3)*4, asuint(asuint(_e49)));
    const float _e55 = converted_absolute(naga_neg(int(3)));
    out_.Store(int(4)*4, asuint(asuint(_e55)));
    const float _e60 = converted_absolute_1(4294967295u);
    out_.Store(int(5)*4, asuint(asuint(_e60)));
    const float _e65 = converted_absolute_2(true);
    out_.Store(int(6)*4, asuint(asuint(_e65)));
    const float _e70 = converted_absolute_2(false);
    out_.Store(int(7)*4, asuint(asuint(_e70)));
    local_3 = 1u;
    uint _e76 = local_3;
    uint _e80 = (asuint(asfloat(_e76)) & 2147483647u);
    local_4 = ((_e80 > 2139095040u) ? 2143289344u : _e80);
    uint _e86 = local_4;
    out_.Store(int(8)*4, asuint(asuint(asfloat(_e86))));
    local_5 = 2147483649u;
    uint _e93 = local_5;
    uint _e97 = (asuint(asfloat(_e93)) & 2147483647u);
    local_6 = ((_e97 > 2139095040u) ? 2143289344u : _e97);
    uint _e103 = local_6;
    out_.Store(int(9)*4, asuint(asuint(asfloat(_e103))));
    local_7 = 8388607u;
    uint _e110 = local_7;
    uint _e114 = (asuint(asfloat(_e110)) & 2147483647u);
    local_8 = ((_e114 > 2139095040u) ? 2143289344u : _e114);
    uint _e120 = local_8;
    out_.Store(int(10)*4, asuint(asuint(asfloat(_e120))));
    local_9 = 2155872255u;
    uint _e127 = local_9;
    uint _e131 = (asuint(asfloat(_e127)) & 2147483647u);
    local_10 = ((_e131 > 2139095040u) ? 2143289344u : _e131);
    uint _e137 = local_10;
    out_.Store(int(11)*4, asuint(asuint(asfloat(_e137))));
    uint _e145 = (asuint(3.4028235e38) & 2147483647u);
    local_11 = ((_e145 > 2139095040u) ? 2143289344u : _e145);
    uint _e151 = local_11;
    out_.Store(int(12)*4, asuint(asuint(asfloat(_e151))));
    uint _e160 = (asuint(-(3.4028235e38)) & 2147483647u);
    local_12 = ((_e160 > 2139095040u) ? 2143289344u : _e160);
    uint _e166 = local_12;
    out_.Store(int(13)*4, asuint(asuint(asfloat(_e166))));
    local_13 = 2139095040u;
    uint _e173 = local_13;
    uint _e177 = (asuint(asfloat(_e173)) & 2147483647u);
    local_14 = ((_e177 > 2139095040u) ? 2143289344u : _e177);
    uint _e183 = local_14;
    out_.Store(int(14)*4, asuint(asuint(asfloat(_e183))));
    local_15 = 4286578688u;
    uint _e190 = local_15;
    uint _e194 = (asuint(asfloat(_e190)) & 2147483647u);
    local_16 = ((_e194 > 2139095040u) ? 2143289344u : _e194);
    uint _e200 = local_16;
    out_.Store(int(15)*4, asuint(asuint(asfloat(_e200))));
    local_17 = 2143363909u;
    uint _e207 = local_17;
    uint _e211 = (asuint(asfloat(_e207)) & 2147483647u);
    local_18 = ((_e211 > 2139095040u) ? 2143289344u : _e211);
    uint _e217 = local_18;
    out_.Store(int(16)*4, asuint(asuint(asfloat(_e217))));
    local_19 = 2139095041u;
    uint _e224 = local_19;
    uint _e228 = (asuint(asfloat(_e224)) & 2147483647u);
    local_20 = ((_e228 > 2139095040u) ? 2143289344u : _e228);
    uint _e234 = local_20;
    out_.Store(int(17)*4, asuint(asuint(asfloat(_e234))));
    value = -(2.75);
    float _e242 = value;
    value = (_e242 + float(int(1)));
    uint _e248 = (asuint(_e242) & 2147483647u);
    local_21 = ((_e248 > 2139095040u) ? 2143289344u : _e248);
    uint _e254 = local_21;
    out_.Store(int(18)*4, asuint(asuint(asfloat(_e254))));
    float _e259 = value;
    out_.Store(int(19)*4, asuint(asuint(_e259)));
    const int _e265 = fabs_3(naga_neg(int(3)));
    out_.Store(int(20)*4, asuint(uint(_e265)));
    const uint _e270 = fabs_4(3u);
    out_.Store(int(21)*4, asuint(_e270));
    const int _e274 = fabs_5(true);
    out_.Store(int(22)*4, asuint(uint(_e274)));
    const int _e279 = fabs_5(false);
    out_.Store(int(23)*4, asuint(uint(_e279)));
    const int _e284 = fabsf(true);
    out_.Store(int(24)*4, asuint(uint(_e284)));
    local_22 = 2147483649u;
    uint _e293 = local_22;
    const float3_ _e296 = fabs_1(Constructfloat3_(-(2.5), -(0.0), asfloat(_e293)));
    triple = _e296;
    float _e300 = triple.x;
    out_.Store(int(25)*4, asuint(asuint(_e300)));
    float _e305 = triple.y;
    out_.Store(int(26)*4, asuint(asuint(_e305)));
    float _e310 = triple.z;
    out_.Store(int(27)*4, asuint(asuint(_e310)));
    const float _e317 = fabsf_1(-(2.0), 3.0);
    out_.Store(int(28)*4, asuint(asuint(_e317)));
    const float2 _e325 = fabs(float2(-(3.0), -(4.0)));
    pair = _e325;
    const float4 _e336 = fabs_2(float4(-(1.0), -(2.0), -(3.0), -(4.0)));
    quadruple = _e336;
    float _e341 = pair.x;
    float _e342 = pair.y;
    float _e345 = quadruple.x;
    float _e348 = quadruple.w;
    out_.Store(int(29)*4, asuint(asuint((((_e341 + _e342) + _e345) + _e348))));
}
