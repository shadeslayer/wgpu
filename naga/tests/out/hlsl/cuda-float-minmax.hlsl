RWByteAddressBuffer out_ : register(u0);

float min_(float first_1, float second_1, float third)
{
    float first_2 = (float)0;
    float second_2 = (float)0;
    float third_1 = (float)0;

    first_2 = first_1;
    second_2 = second_1;
    third_1 = third;
    float _e6 = first_2;
    float _e7 = second_2;
    uint _e12 = asuint(_e6);
    bool _e18 = ((_e12 & 2147483647u) > 2139095040u);
    uint _e19 = asuint(_e7);
    bool _e25 = ((_e19 & 2147483647u) > 2139095040u);
    float _e33 = third_1;
    uint _e38 = asuint(asfloat(((_e18 && _e25) ? 2143289344u : (_e25 ? _e12 : (_e18 ? _e19 : ((((_e12 >= 2147483648u) ? ~(_e12) : (_e12 ^ 2147483648u)) < ((_e19 >= 2147483648u) ? ~(_e19) : (_e19 ^ 2147483648u))) ? _e12 : _e19))))));
    bool _e44 = ((_e38 & 2147483647u) > 2139095040u);
    uint _e45 = asuint(_e33);
    bool _e51 = ((_e45 & 2147483647u) > 2139095040u);
    return asfloat(((_e44 && _e51) ? 2143289344u : (_e51 ? _e38 : (_e44 ? _e45 : ((((_e38 >= 2147483648u) ? ~(_e38) : (_e38 ^ 2147483648u)) < ((_e45 >= 2147483648u) ? ~(_e45) : (_e45 ^ 2147483648u))) ? _e38 : _e45)))));
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

float bounded(float value_1)
{
    float value_2 = (float)0;

    value_2 = value_1;
    float _e5 = value_2;
    uint _e12 = asuint(_e5);
    bool _e18 = ((_e12 & 2147483647u) > 2139095040u);
    uint _e19 = asuint(float(int(4)));
    bool _e25 = ((_e19 & 2147483647u) > 2139095040u);
    uint _e37 = asuint(float(naga_neg(int(2))));
    bool _e43 = ((_e37 & 2147483647u) > 2139095040u);
    uint _e44 = asuint(asfloat(((_e18 && _e25) ? 2143289344u : (_e25 ? _e12 : (_e18 ? _e19 : ((((_e12 >= 2147483648u) ? ~(_e12) : (_e12 ^ 2147483648u)) < ((_e19 >= 2147483648u) ? ~(_e19) : (_e19 ^ 2147483648u))) ? _e12 : _e19))))));
    bool _e50 = ((_e44 & 2147483647u) > 2139095040u);
    return asfloat(((_e43 && _e50) ? 2143289344u : (_e50 ? _e37 : (_e43 ? _e44 : ((((_e37 >= 2147483648u) ? ~(_e37) : (_e37 ^ 2147483648u)) > ((_e44 >= 2147483648u) ? ~(_e44) : (_e44 ^ 2147483648u))) ? _e37 : _e44)))));
}

[numthreads(1, 1, 1)]
void float_minmax()
{
    float seed = (float)0;
    float first = (float)0;
    float second = (float)0;
    float value = (float)0;

    float _e4 = asfloat(out_.Load(int(0)*4));
    seed = _e4;
    float _e7 = seed;
    uint _e14 = asuint(_e7);
    bool _e20 = ((_e14 & 2147483647u) > 2139095040u);
    uint _e21 = asuint(-(7.5));
    bool _e27 = ((_e21 & 2147483647u) > 2139095040u);
    out_.Store(int(0)*4, asuint(asfloat(((_e20 && _e27) ? 2143289344u : (_e27 ? _e14 : (_e20 ? _e21 : ((((_e14 >= 2147483648u) ? ~(_e14) : (_e14 ^ 2147483648u)) < ((_e21 >= 2147483648u) ? ~(_e21) : (_e21 ^ 2147483648u))) ? _e14 : _e21)))))));
    float _e37 = seed;
    uint _e44 = asuint(_e37);
    bool _e50 = ((_e44 & 2147483647u) > 2139095040u);
    uint _e51 = asuint(-(7.5));
    bool _e57 = ((_e51 & 2147483647u) > 2139095040u);
    out_.Store(int(1)*4, asuint(asfloat(((_e50 && _e57) ? 2143289344u : (_e57 ? _e44 : (_e50 ? _e51 : ((((_e44 >= 2147483648u) ? ~(_e44) : (_e44 ^ 2147483648u)) > ((_e51 >= 2147483648u) ? ~(_e51) : (_e51 ^ 2147483648u))) ? _e44 : _e51)))))));
    float _e67 = seed;
    uint _e73 = asuint(_e67);
    bool _e79 = ((_e73 & 2147483647u) > 2139095040u);
    uint _e80 = asuint(1.5);
    bool _e86 = ((_e80 & 2147483647u) > 2139095040u);
    out_.Store(int(2)*4, asuint(asfloat(((_e79 && _e86) ? 2143289344u : (_e86 ? _e73 : (_e79 ? _e80 : ((((_e73 >= 2147483648u) ? ~(_e73) : (_e73 ^ 2147483648u)) < ((_e80 >= 2147483648u) ? ~(_e80) : (_e80 ^ 2147483648u))) ? _e73 : _e80)))))));
    float _e98 = seed;
    uint _e103 = asuint(-(2.25));
    bool _e109 = ((_e103 & 2147483647u) > 2139095040u);
    uint _e110 = asuint(_e98);
    bool _e116 = ((_e110 & 2147483647u) > 2139095040u);
    out_.Store(int(3)*4, asuint(asfloat(((_e109 && _e116) ? 2143289344u : (_e116 ? _e103 : (_e109 ? _e110 : ((((_e103 >= 2147483648u) ? ~(_e103) : (_e103 ^ 2147483648u)) > ((_e110 >= 2147483648u) ? ~(_e110) : (_e110 ^ 2147483648u))) ? _e103 : _e110)))))));
    uint _e135 = asuint(float(naga_neg(int(3))));
    bool _e141 = ((_e135 & 2147483647u) > 2139095040u);
    uint _e142 = asuint(float(5u));
    bool _e148 = ((_e142 & 2147483647u) > 2139095040u);
    out_.Store(int(4)*4, asuint(asfloat(((_e141 && _e148) ? 2143289344u : (_e148 ? _e135 : (_e141 ? _e142 : ((((_e135 >= 2147483648u) ? ~(_e135) : (_e135 ^ 2147483648u)) < ((_e142 >= 2147483648u) ? ~(_e142) : (_e142 ^ 2147483648u))) ? _e135 : _e142)))))));
    uint _e167 = asuint(float(5u));
    bool _e173 = ((_e167 & 2147483647u) > 2139095040u);
    uint _e174 = asuint(float(naga_neg(int(3))));
    bool _e180 = ((_e174 & 2147483647u) > 2139095040u);
    out_.Store(int(5)*4, asuint(asfloat(((_e173 && _e180) ? 2143289344u : (_e180 ? _e167 : (_e173 ? _e174 : ((((_e167 >= 2147483648u) ? ~(_e167) : (_e167 ^ 2147483648u)) > ((_e174 >= 2147483648u) ? ~(_e174) : (_e174 ^ 2147483648u))) ? _e167 : _e174)))))));
    uint _e198 = asuint(float(true));
    bool _e204 = ((_e198 & 2147483647u) > 2139095040u);
    uint _e205 = asuint(float(false));
    bool _e211 = ((_e205 & 2147483647u) > 2139095040u);
    out_.Store(int(6)*4, asuint(asfloat(((_e204 && _e211) ? 2143289344u : (_e211 ? _e198 : (_e204 ? _e205 : ((((_e198 >= 2147483648u) ? ~(_e198) : (_e198 ^ 2147483648u)) < ((_e205 >= 2147483648u) ? ~(_e205) : (_e205 ^ 2147483648u))) ? _e198 : _e205)))))));
    uint _e229 = asuint(float(false));
    bool _e235 = ((_e229 & 2147483647u) > 2139095040u);
    uint _e236 = asuint(float(true));
    bool _e242 = ((_e236 & 2147483647u) > 2139095040u);
    out_.Store(int(7)*4, asuint(asfloat(((_e235 && _e242) ? 2143289344u : (_e242 ? _e229 : (_e235 ? _e236 : ((((_e229 >= 2147483648u) ? ~(_e229) : (_e229 ^ 2147483648u)) > ((_e236 >= 2147483648u) ? ~(_e236) : (_e236 ^ 2147483648u))) ? _e229 : _e236)))))));
    float _e252 = seed;
    uint _e259 = asuint(_e252);
    bool _e265 = ((_e259 & 2147483647u) > 2139095040u);
    uint _e266 = asuint(-(4.0));
    bool _e272 = ((_e266 & 2147483647u) > 2139095040u);
    uint _e285 = asuint(asfloat(((_e265 && _e272) ? 2143289344u : (_e272 ? _e259 : (_e265 ? _e266 : ((((_e259 >= 2147483648u) ? ~(_e259) : (_e259 ^ 2147483648u)) > ((_e266 >= 2147483648u) ? ~(_e266) : (_e266 ^ 2147483648u))) ? _e259 : _e266))))));
    bool _e291 = ((_e285 & 2147483647u) > 2139095040u);
    uint _e292 = asuint(2.5);
    bool _e298 = ((_e292 & 2147483647u) > 2139095040u);
    out_.Store(int(8)*4, asuint(asfloat(((_e291 && _e298) ? 2143289344u : (_e298 ? _e285 : (_e291 ? _e292 : ((((_e285 >= 2147483648u) ? ~(_e285) : (_e285 ^ 2147483648u)) < ((_e292 >= 2147483648u) ? ~(_e292) : (_e292 ^ 2147483648u))) ? _e285 : _e292)))))));
    float _e308 = seed;
    const float _e312 = min_(_e308, 1.25, -(3.5));
    out_.Store(int(9)*4, asuint(_e312));
    float _e315 = seed;
    const float _e316 = bounded(_e315);
    out_.Store(int(10)*4, asuint(_e316));
    first = 1.25;
    second = 2.5;
    float _e323 = first;
    first = (_e323 + float(int(1)));
    float _e327 = second;
    second = (_e327 + float(int(1)));
    uint _e335 = asuint(_e323);
    bool _e341 = ((_e335 & 2147483647u) > 2139095040u);
    uint _e342 = asuint(_e327);
    bool _e348 = ((_e342 & 2147483647u) > 2139095040u);
    out_.Store(int(11)*4, asuint(asfloat(((_e341 && _e348) ? 2143289344u : (_e348 ? _e335 : (_e341 ? _e342 : ((((_e335 >= 2147483648u) ? ~(_e335) : (_e335 ^ 2147483648u)) > ((_e342 >= 2147483648u) ? ~(_e342) : (_e342 ^ 2147483648u))) ? _e335 : _e342)))))));
    float _e358 = first;
    out_.Store(int(12)*4, asuint(_e358));
    float _e361 = second;
    out_.Store(int(13)*4, asuint(_e361));
    float _e363 = seed;
    value = _e363;
    float _e366 = value;
    float _e367 = value;
    value = (_e367 + float(int(1)));
    uint _e375 = asuint(_e366);
    bool _e381 = ((_e375 & 2147483647u) > 2139095040u);
    uint _e382 = asuint(_e367);
    bool _e388 = ((_e382 & 2147483647u) > 2139095040u);
    out_.Store(int(14)*4, asuint(asfloat(((_e381 && _e388) ? 2143289344u : (_e388 ? _e375 : (_e381 ? _e382 : ((((_e375 >= 2147483648u) ? ~(_e375) : (_e375 ^ 2147483648u)) > ((_e382 >= 2147483648u) ? ~(_e382) : (_e382 ^ 2147483648u))) ? _e375 : _e382)))))));
    float _e398 = value;
    out_.Store(int(15)*4, asuint(_e398));
    uint _e408 = asuint(float(int(16777217)));
    bool _e414 = ((_e408 & 2147483647u) > 2139095040u);
    uint _e415 = asuint(16777216.0);
    bool _e421 = ((_e415 & 2147483647u) > 2139095040u);
    out_.Store(int(16)*4, asuint(asfloat(((_e414 && _e421) ? 2143289344u : (_e421 ? _e408 : (_e414 ? _e415 : ((((_e408 >= 2147483648u) ? ~(_e408) : (_e408 ^ 2147483648u)) > ((_e415 >= 2147483648u) ? ~(_e415) : (_e415 ^ 2147483648u))) ? _e408 : _e415)))))));
    uint _e438 = asuint(float(4294967295u));
    bool _e444 = ((_e438 & 2147483647u) > 2139095040u);
    uint _e445 = asuint(4294967300.0);
    bool _e451 = ((_e445 & 2147483647u) > 2139095040u);
    out_.Store(int(17)*4, asuint(asfloat(((_e444 && _e451) ? 2143289344u : (_e451 ? _e438 : (_e444 ? _e445 : ((((_e438 >= 2147483648u) ? ~(_e438) : (_e438 ^ 2147483648u)) < ((_e445 >= 2147483648u) ? ~(_e445) : (_e445 ^ 2147483648u))) ? _e438 : _e445)))))));
    float _e463 = seed;
    uint _e468 = asuint(float(true));
    bool _e474 = ((_e468 & 2147483647u) > 2139095040u);
    uint _e475 = asuint(_e463);
    bool _e481 = ((_e475 & 2147483647u) > 2139095040u);
    out_.Store(int(18)*4, asuint(asfloat(((_e474 && _e481) ? 2143289344u : (_e481 ? _e468 : (_e474 ? _e475 : ((((_e468 >= 2147483648u) ? ~(_e468) : (_e468 ^ 2147483648u)) > ((_e475 >= 2147483648u) ? ~(_e475) : (_e475 ^ 2147483648u))) ? _e468 : _e475)))))));
    float _e491 = seed;
    float _e492 = seed;
    uint _e497 = asuint(_e491);
    bool _e503 = ((_e497 & 2147483647u) > 2139095040u);
    uint _e504 = asuint(_e492);
    bool _e510 = ((_e504 & 2147483647u) > 2139095040u);
    out_.Store(int(19)*4, asuint(asfloat(((_e503 && _e510) ? 2143289344u : (_e510 ? _e497 : (_e503 ? _e504 : ((((_e497 >= 2147483648u) ? ~(_e497) : (_e497 ^ 2147483648u)) < ((_e504 >= 2147483648u) ? ~(_e504) : (_e504 ^ 2147483648u))) ? _e497 : _e504)))))));
}
