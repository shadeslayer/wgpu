RWByteAddressBuffer out_ : register(u0);

int min_(int first_1, int second_1, int third)
{
    int first_2 = (int)0;
    int second_2 = (int)0;
    int third_1 = (int)0;

    first_2 = first_1;
    second_2 = second_1;
    third_1 = third;
    int _e6 = first_2;
    int _e7 = second_2;
    int _e9 = third_1;
    return asint(asuint(asint(asuint(asint(asuint(_e6) + asuint(_e7))) + asuint(_e9))) + asuint(int(100)));
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

int bounded(int value_1)
{
    int value_2 = (int)0;

    value_2 = value_1;
    int _e2 = value_2;
    return min(max(_e2, naga_neg(int(2))), int(2));
}

[numthreads(1, 1, 1)]
void integer_minmax()
{
    int seed = (int)0;
    int first = (int)0;
    int second = (int)0;
    int value = (int)0;

    int _e4 = asint(out_.Load(int(0)*4));
    seed = _e4;
    int _e7 = seed;
    out_.Store(int(0)*4, asuint(min(_e7, naga_neg(int(7)))));
    int _e13 = seed;
    out_.Store(int(1)*4, asuint(max(_e13, naga_neg(int(7)))));
    out_.Store(int(2)*4, asuint(int(min(3u, 9u))));
    out_.Store(int(3)*4, asuint(int(max(3u, 9u))));
    out_.Store(int(4)*4, asuint(int((min(uint(naga_neg(int(1))), 2u) == 2u))));
    out_.Store(int(5)*4, asuint(int((max(uint(naga_neg(int(1))), 2u) == 4294967295u))));
    out_.Store(int(6)*4, asuint(int((min(2u, uint(naga_neg(int(1)))) == 2u))));
    out_.Store(int(7)*4, asuint(int((max(2u, uint(naga_neg(int(1)))) == 4294967295u))));
    out_.Store(int(8)*4, asuint(min(int(true), int(false))));
    out_.Store(int(9)*4, asuint(max(int(false), int(true))));
    int _e85 = seed;
    out_.Store(int(10)*4, asuint(min(max(_e85, naga_neg(int(5))), int(2))));
    out_.Store(int(11)*4, asuint(int((min(uint(naga_neg(int(2))), 4u) == 4u))));
    int _e103 = seed;
    const int _e106 = min_(_e103, int(4), int(5));
    out_.Store(int(12)*4, asuint(_e106));
    int _e109 = seed;
    const int _e110 = bounded(_e109);
    out_.Store(int(13)*4, asuint(_e110));
    first = int(3);
    second = int(7);
    int _e117 = first;
    first = asint(asuint(_e117) + asuint(int(1)));
    int _e120 = second;
    second = asint(asuint(_e120) + asuint(int(1)));
    out_.Store(int(14)*4, asuint(max(_e117, _e120)));
    int _e126 = first;
    out_.Store(int(15)*4, asuint(_e126));
    int _e129 = second;
    out_.Store(int(16)*4, asuint(_e129));
    int _e131 = seed;
    value = _e131;
    int _e134 = value;
    int _e135 = value;
    value = asint(asuint(_e135) + asuint(int(1)));
    out_.Store(int(17)*4, asuint(max(_e134, _e135)));
    int _e141 = value;
    out_.Store(int(18)*4, asuint(_e141));
    out_.Store(int(19)*4, asuint(int((min(asint(asuint(naga_neg(int(2147483647))) - asuint(int(1))), int(2147483647)) == asint(asuint(naga_neg(int(2147483647))) - asuint(int(1)))))));
    out_.Store(int(20)*4, asuint(int((max(4294967295u, 0u) == 4294967295u))));
    out_.Store(int(21)*4, asuint(int((min(uint(false), 3u) == 0u))));
    out_.Store(int(22)*4, asuint(int((max(uint(true), 0u) == 1u))));
    int _e184 = seed;
    int _e185 = seed;
    out_.Store(int(23)*4, asuint(min(_e184, _e185)));
}
