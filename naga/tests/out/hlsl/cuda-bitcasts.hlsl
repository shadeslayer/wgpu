RWByteAddressBuffer out_ : register(u0);

float from_bits(uint value_1)
{
    uint value_2 = (uint)0;
    uint local_13 = (uint)0;

    value_2 = value_1;
    uint _e2 = value_2;
    local_13 = _e2;
    uint _e4 = local_13;
    return asfloat(_e4);
}

int _float_as_int(int first, int second)
{
    int first_1 = (int)0;
    int second_1 = (int)0;

    first_1 = first;
    second_1 = second;
    int _e4 = first_1;
    int _e5 = second_1;
    return asint(asuint(_e4) + asuint(_e5));
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

uint naga_f2u32(float value) {
    return uint(clamp(value, 0.0, 4294967000.0));
}

[numthreads(1, 1, 1)]
void bitcasts()
{
    int local = (int)0;
    int local_1 = (int)0;
    int local_2 = (int)0;
    int local_3 = (int)0;
    uint local_4 = (uint)0;
    uint local_5 = (uint)0;
    uint local_6 = (uint)0;
    uint local_7 = (uint)0;
    float value = (float)0;
    int signed_bits = (int)0;
    int local_8 = (int)0;
    uint unsigned_bits = (uint)0;
    uint local_9 = (uint)0;
    float copied = (float)0;
    int local_10 = (int)0;
    uint local_11 = (uint)0;
    uint local_12 = (uint)0;

    out_.Store(int(0)*4, asuint(uint(asint(float(true)))));
    out_.Store(int(1)*4, asuint(uint(asint(float(int(3))))));
    out_.Store(int(2)*4, asuint(uint(asint(float(5u)))));
    out_.Store(int(3)*4, asuint(uint(asint(2.75))));
    out_.Store(int(4)*4, asuint(asuint(float(true))));
    out_.Store(int(5)*4, asuint(asuint(float(int(3)))));
    out_.Store(int(6)*4, asuint(asuint(float(5u))));
    out_.Store(int(7)*4, asuint(asuint(2.75)));
    local = int(true);
    int _e48 = local;
    out_.Store(int(8)*4, asuint(asuint(asfloat(_e48))));
    local_1 = int(3);
    int _e55 = local_1;
    out_.Store(int(9)*4, asuint(asuint(asfloat(_e55))));
    local_2 = int(5u);
    int _e63 = local_2;
    out_.Store(int(10)*4, asuint(asuint(asfloat(_e63))));
    local_3 = naga_f2i32(2.75);
    int _e71 = local_3;
    out_.Store(int(11)*4, asuint(asuint(asfloat(_e71))));
    local_4 = uint(true);
    uint _e79 = local_4;
    out_.Store(int(12)*4, asuint(asuint(asfloat(_e79))));
    local_5 = uint(int(3));
    uint _e87 = local_5;
    out_.Store(int(13)*4, asuint(asuint(asfloat(_e87))));
    local_6 = 5u;
    uint _e94 = local_6;
    out_.Store(int(14)*4, asuint(asuint(asfloat(_e94))));
    local_7 = naga_f2u32(2.75);
    uint _e102 = local_7;
    out_.Store(int(15)*4, asuint(asuint(asfloat(_e102))));
    value = 1.5;
    float _e109 = value;
    value = (_e109 + float(int(1)));
    out_.Store(int(16)*4, asuint(uint(asint(_e109))));
    float _e117 = value;
    out_.Store(int(17)*4, asuint(asuint(_e117)));
    float _e121 = value;
    value = (_e121 + float(int(1)));
    out_.Store(int(18)*4, asuint(asuint(_e121)));
    float _e128 = value;
    out_.Store(int(19)*4, asuint(asuint(_e128)));
    signed_bits = int(1065353216);
    int _e134 = signed_bits;
    signed_bits = asint(asuint(_e134) + asuint(int(1)));
    local_8 = _e134;
    int _e138 = local_8;
    out_.Store(int(20)*4, asuint(asuint(asfloat(_e138))));
    int _e143 = signed_bits;
    out_.Store(int(21)*4, asuint(uint(_e143)));
    unsigned_bits = 1073741824u;
    uint _e149 = unsigned_bits;
    unsigned_bits = (_e149 + uint(int(1)));
    local_9 = _e149;
    uint _e154 = local_9;
    out_.Store(int(22)*4, asuint(asuint(asfloat(_e154))));
    uint _e159 = unsigned_bits;
    out_.Store(int(23)*4, asuint(_e159));
    const int _e164 = _float_as_int(int(3), int(4));
    out_.Store(int(24)*4, asuint(uint(_e164)));
    const float _e168 = from_bits(3212836864u);
    copied = _e168;
    float _e171 = copied;
    out_.Store(int(25)*4, asuint(asuint(_e171)));
    local_10 = int(2139095040);
    int _e177 = local_10;
    out_.Store(int(26)*4, asuint(asuint(asfloat(_e177))));
    local_11 = 2143363909u;
    uint _e184 = local_11;
    out_.Store(int(27)*4, asuint(asuint(asfloat(_e184))));
    local_12 = naga_f2u32(1065353200.0);
    uint _e192 = local_12;
    out_.Store(int(28)*4, asuint(asuint(asfloat(_e192))));
    out_.Store(int(29)*4, asuint(uint(asint(float(1u)))));
}
