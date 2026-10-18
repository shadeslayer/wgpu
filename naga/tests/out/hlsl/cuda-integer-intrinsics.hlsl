RWByteAddressBuffer out_ : register(u0);

int count(uint value)
{
    uint value_1 = (uint)0;

    value_1 = value;
    uint _e2 = value_1;
    return int(countbits(_e2));
}

int scan(int value_2)
{
    int value_3 = (int)0;

    value_3 = value_2;
    int _e2 = value_3;
    return asint(asuint(asint(firstbitlow(_e2))) + asuint(int(1)));
}

int _clz(int first, int second)
{
    int first_1 = (int)0;
    int second_1 = (int)0;

    first_1 = first;
    second_1 = second;
    int _e4 = first_1;
    int _e5 = second_1;
    return asint(asuint(_e4) + asuint(_e5));
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

uint naga_f2u32(float value) {
    return uint(clamp(value, 0.0, 4294967000.0));
}

[numthreads(1, 1, 1)]
void integer_intrinsics()
{
    int _e4 = int(true);
    out_.Store(int(0)*4, asuint(uint((_e4 < 0 ? 0 : 31 - asint(firstbithigh(_e4))))));
    out_.Store(int(1)*4, asuint(uint((int(8) < 0 ? 0 : 31 - asint(firstbithigh(int(8)))))));
    int _e15 = int(2147483648u);
    out_.Store(int(2)*4, asuint(uint((_e15 < 0 ? 0 : 31 - asint(firstbithigh(_e15))))));
    int _e21 = naga_f2i32(8.75);
    out_.Store(int(3)*4, asuint(uint((_e21 < 0 ? 0 : 31 - asint(firstbithigh(_e21))))));
    out_.Store(int(4)*4, asuint(uint(asint(asuint(asint(firstbitlow(int(true)))) + asuint(int(1))))));
    out_.Store(int(5)*4, asuint(uint(asint(asuint(asint(firstbitlow(int(8)))) + asuint(int(1))))));
    out_.Store(int(6)*4, asuint(uint(asint(asuint(asint(firstbitlow(int(2147483648u)))) + asuint(int(1))))));
    out_.Store(int(7)*4, asuint(uint(asint(asuint(asint(firstbitlow(naga_f2i32(8.75)))) + asuint(int(1))))));
    out_.Store(int(8)*4, asuint(uint(int(countbits(uint(true))))));
    out_.Store(int(9)*4, asuint(uint(int(countbits(uint(naga_neg(int(1))))))));
    out_.Store(int(10)*4, asuint(uint(int(countbits(2147483649u)))));
    out_.Store(int(11)*4, asuint(uint(int(countbits(naga_f2u32(3.75))))));
    out_.Store(int(12)*4, asuint(reversebits(uint(true))));
    out_.Store(int(13)*4, asuint(reversebits(uint(naga_neg(int(1))))));
    out_.Store(int(14)*4, asuint(reversebits(19088743u)));
    out_.Store(int(15)*4, asuint(reversebits(naga_f2u32(2.75))));
    const int _e106 = count(2147483649u);
    out_.Store(int(16)*4, asuint(uint(_e106)));
    const int _e112 = scan(int(2147483648u));
    out_.Store(int(17)*4, asuint(uint(_e112)));
    out_.Store(int(18)*4, asuint(uint((int(0) < 0 ? 0 : 31 - asint(firstbithigh(int(0)))))));
    out_.Store(int(19)*4, asuint(uint(asint(asuint(asint(firstbitlow(int(0)))) + asuint(int(1))))));
    out_.Store(int(20)*4, asuint(uint(int(countbits(4294967295u)))));
    out_.Store(int(21)*4, asuint(reversebits(0u)));
    out_.Store(int(22)*4, asuint(reversebits(1u)));
    out_.Store(int(23)*4, asuint(reversebits(2147483648u)));
    int _e147 = naga_neg(int(2));
    out_.Store(int(24)*4, asuint(uint((_e147 < 0 ? 0 : 31 - asint(firstbithigh(_e147))))));
    out_.Store(int(25)*4, asuint(uint(asint(asuint(asint(firstbitlow(naga_neg(int(2))))) + asuint(int(1))))));
    out_.Store(int(26)*4, asuint(uint(int(countbits(2863311530u)))));
    out_.Store(int(27)*4, asuint(reversebits(2863311530u)));
    const int _e172 = _clz(int(3), int(4));
    out_.Store(int(28)*4, asuint(uint(_e172)));
    out_.Store(int(29)*4, asuint(uint((asint(asuint(int(countbits(3u))) - asuint(int(3))) < int(0)))));
}
