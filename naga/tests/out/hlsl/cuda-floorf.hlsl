RWByteAddressBuffer out_ : register(u0);

float floor_value(float value_1)
{
    float value_2 = (float)0;
    uint local_31 = (uint)0;

    value_2 = value_1;
    float _e2 = value_2;
    uint _e3 = asuint(_e2);
    uint _e8 = (_e3 & 2147483647u);
    bool _e10 = ((_e3 & 2147483648u) != 0u);
    uint _e17 = (1u << ((150u - (_e8 >> 23u)) & 31u));
    uint _e18 = (_e17 - 1u);
    uint _e22 = (_e3 & ~(_e18));
    local_31 = ((_e8 > 2139095040u) ? 2143289344u : ((_e8 >= 1258291200u) ? _e3 : ((_e8 < 1065353216u) ? ((_e8 == 0u) ? _e3 : (_e10 ? 3212836864u : 0u)) : ((_e10 && ((_e3 & _e18) != 0u)) ? (_e22 + _e17) : _e22))));
    uint _e41 = local_31;
    return asfloat(_e41);
}

float floor_value_1(int value_3)
{
    int value_4 = (int)0;
    uint local_32 = (uint)0;

    value_4 = value_3;
    int _e2 = value_4;
    uint _e4 = asuint(float(_e2));
    uint _e9 = (_e4 & 2147483647u);
    bool _e11 = ((_e4 & 2147483648u) != 0u);
    uint _e18 = (1u << ((150u - (_e9 >> 23u)) & 31u));
    uint _e19 = (_e18 - 1u);
    uint _e23 = (_e4 & ~(_e19));
    local_32 = ((_e9 > 2139095040u) ? 2143289344u : ((_e9 >= 1258291200u) ? _e4 : ((_e9 < 1065353216u) ? ((_e9 == 0u) ? _e4 : (_e11 ? 3212836864u : 0u)) : ((_e11 && ((_e4 & _e19) != 0u)) ? (_e23 + _e18) : _e23))));
    uint _e42 = local_32;
    return asfloat(_e42);
}

float floor_value_2(uint value_5)
{
    uint value_6 = (uint)0;
    uint local_33 = (uint)0;

    value_6 = value_5;
    uint _e2 = value_6;
    uint _e4 = asuint(float(_e2));
    uint _e9 = (_e4 & 2147483647u);
    bool _e11 = ((_e4 & 2147483648u) != 0u);
    uint _e18 = (1u << ((150u - (_e9 >> 23u)) & 31u));
    uint _e19 = (_e18 - 1u);
    uint _e23 = (_e4 & ~(_e19));
    local_33 = ((_e9 > 2139095040u) ? 2143289344u : ((_e9 >= 1258291200u) ? _e4 : ((_e9 < 1065353216u) ? ((_e9 == 0u) ? _e4 : (_e11 ? 3212836864u : 0u)) : ((_e11 && ((_e4 & _e19) != 0u)) ? (_e23 + _e18) : _e23))));
    uint _e42 = local_33;
    return asfloat(_e42);
}

float floor_value_3(bool value_7)
{
    bool value_8 = (bool)0;
    uint local_34 = (uint)0;

    value_8 = value_7;
    bool _e2 = value_8;
    uint _e4 = asuint(float(_e2));
    uint _e9 = (_e4 & 2147483647u);
    bool _e11 = ((_e4 & 2147483648u) != 0u);
    uint _e18 = (1u << ((150u - (_e9 >> 23u)) & 31u));
    uint _e19 = (_e18 - 1u);
    uint _e23 = (_e4 & ~(_e19));
    local_34 = ((_e9 > 2139095040u) ? 2143289344u : ((_e9 >= 1258291200u) ? _e4 : ((_e9 < 1065353216u) ? ((_e9 == 0u) ? _e4 : (_e11 ? 3212836864u : 0u)) : ((_e11 && ((_e4 & _e19) != 0u)) ? (_e23 + _e18) : _e23))));
    uint _e42 = local_34;
    return asfloat(_e42);
}

int floorf(bool value_9)
{
    bool value_10 = (bool)0;
    int local_35 = (int)0;

    value_10 = value_9;
    bool _e2 = value_10;
    if (_e2) {
        local_35 = int(7);
    } else {
        local_35 = int(9);
    }
    int _e6 = local_35;
    return _e6;
}

float floorf_1(float first, float second)
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
void floors()
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
    uint local_21 = (uint)0;
    uint local_22 = (uint)0;
    uint local_23 = (uint)0;
    uint local_24 = (uint)0;
    uint local_25 = (uint)0;
    float value = (float)0;
    uint local_26 = (uint)0;
    uint local_27 = (uint)0;
    uint local_28 = (uint)0;
    uint local_29 = (uint)0;
    uint local_30 = (uint)0;

    uint _e4 = asuint(0.0);
    uint _e9 = (_e4 & 2147483647u);
    bool _e11 = ((_e4 & 2147483648u) != 0u);
    uint _e18 = (1u << ((150u - (_e9 >> 23u)) & 31u));
    uint _e19 = (_e18 - 1u);
    uint _e23 = (_e4 & ~(_e19));
    local = ((_e9 > 2139095040u) ? 2143289344u : ((_e9 >= 1258291200u) ? _e4 : ((_e9 < 1065353216u) ? ((_e9 == 0u) ? _e4 : (_e11 ? 3212836864u : 0u)) : ((_e11 && ((_e4 & _e19) != 0u)) ? (_e23 + _e18) : _e23))));
    uint _e42 = local;
    out_.Store(int(0)*4, asuint(asuint(asfloat(_e42))));
    uint _e49 = asuint(-(0.0));
    uint _e54 = (_e49 & 2147483647u);
    bool _e56 = ((_e49 & 2147483648u) != 0u);
    uint _e63 = (1u << ((150u - (_e54 >> 23u)) & 31u));
    uint _e64 = (_e63 - 1u);
    uint _e68 = (_e49 & ~(_e64));
    local_1 = ((_e54 > 2139095040u) ? 2143289344u : ((_e54 >= 1258291200u) ? _e49 : ((_e54 < 1065353216u) ? ((_e54 == 0u) ? _e49 : (_e56 ? 3212836864u : 0u)) : ((_e56 && ((_e49 & _e64) != 0u)) ? (_e68 + _e63) : _e68))));
    uint _e87 = local_1;
    out_.Store(int(1)*4, asuint(asuint(asfloat(_e87))));
    uint _e93 = asuint(1.75);
    uint _e98 = (_e93 & 2147483647u);
    bool _e100 = ((_e93 & 2147483648u) != 0u);
    uint _e107 = (1u << ((150u - (_e98 >> 23u)) & 31u));
    uint _e108 = (_e107 - 1u);
    uint _e112 = (_e93 & ~(_e108));
    local_2 = ((_e98 > 2139095040u) ? 2143289344u : ((_e98 >= 1258291200u) ? _e93 : ((_e98 < 1065353216u) ? ((_e98 == 0u) ? _e93 : (_e100 ? 3212836864u : 0u)) : ((_e100 && ((_e93 & _e108) != 0u)) ? (_e112 + _e107) : _e112))));
    uint _e131 = local_2;
    out_.Store(int(2)*4, asuint(asuint(asfloat(_e131))));
    uint _e138 = asuint(-(1.25));
    uint _e143 = (_e138 & 2147483647u);
    bool _e145 = ((_e138 & 2147483648u) != 0u);
    uint _e152 = (1u << ((150u - (_e143 >> 23u)) & 31u));
    uint _e153 = (_e152 - 1u);
    uint _e157 = (_e138 & ~(_e153));
    local_3 = ((_e143 > 2139095040u) ? 2143289344u : ((_e143 >= 1258291200u) ? _e138 : ((_e143 < 1065353216u) ? ((_e143 == 0u) ? _e138 : (_e145 ? 3212836864u : 0u)) : ((_e145 && ((_e138 & _e153) != 0u)) ? (_e157 + _e152) : _e157))));
    uint _e176 = local_3;
    out_.Store(int(3)*4, asuint(asuint(asfloat(_e176))));
    const float _e182 = floor_value_1(int(3));
    out_.Store(int(4)*4, asuint(asuint(_e182)));
    const float _e187 = floor_value_2(5u);
    out_.Store(int(5)*4, asuint(asuint(_e187)));
    const float _e192 = floor_value_3(true);
    out_.Store(int(6)*4, asuint(asuint(_e192)));
    const float _e197 = floor_value_3(false);
    out_.Store(int(7)*4, asuint(asuint(_e197)));
    local_4 = 1u;
    uint _e203 = local_4;
    uint _e205 = asuint(asfloat(_e203));
    uint _e210 = (_e205 & 2147483647u);
    bool _e212 = ((_e205 & 2147483648u) != 0u);
    uint _e219 = (1u << ((150u - (_e210 >> 23u)) & 31u));
    uint _e220 = (_e219 - 1u);
    uint _e224 = (_e205 & ~(_e220));
    local_5 = ((_e210 > 2139095040u) ? 2143289344u : ((_e210 >= 1258291200u) ? _e205 : ((_e210 < 1065353216u) ? ((_e210 == 0u) ? _e205 : (_e212 ? 3212836864u : 0u)) : ((_e212 && ((_e205 & _e220) != 0u)) ? (_e224 + _e219) : _e224))));
    uint _e243 = local_5;
    out_.Store(int(8)*4, asuint(asuint(asfloat(_e243))));
    local_6 = 2147483649u;
    uint _e250 = local_6;
    uint _e252 = asuint(asfloat(_e250));
    uint _e257 = (_e252 & 2147483647u);
    bool _e259 = ((_e252 & 2147483648u) != 0u);
    uint _e266 = (1u << ((150u - (_e257 >> 23u)) & 31u));
    uint _e267 = (_e266 - 1u);
    uint _e271 = (_e252 & ~(_e267));
    local_7 = ((_e257 > 2139095040u) ? 2143289344u : ((_e257 >= 1258291200u) ? _e252 : ((_e257 < 1065353216u) ? ((_e257 == 0u) ? _e252 : (_e259 ? 3212836864u : 0u)) : ((_e259 && ((_e252 & _e267) != 0u)) ? (_e271 + _e266) : _e271))));
    uint _e290 = local_7;
    out_.Store(int(9)*4, asuint(asuint(asfloat(_e290))));
    local_8 = 8388607u;
    uint _e297 = local_8;
    uint _e299 = asuint(asfloat(_e297));
    uint _e304 = (_e299 & 2147483647u);
    bool _e306 = ((_e299 & 2147483648u) != 0u);
    uint _e313 = (1u << ((150u - (_e304 >> 23u)) & 31u));
    uint _e314 = (_e313 - 1u);
    uint _e318 = (_e299 & ~(_e314));
    local_9 = ((_e304 > 2139095040u) ? 2143289344u : ((_e304 >= 1258291200u) ? _e299 : ((_e304 < 1065353216u) ? ((_e304 == 0u) ? _e299 : (_e306 ? 3212836864u : 0u)) : ((_e306 && ((_e299 & _e314) != 0u)) ? (_e318 + _e313) : _e318))));
    uint _e337 = local_9;
    out_.Store(int(10)*4, asuint(asuint(asfloat(_e337))));
    local_10 = 2155872255u;
    uint _e344 = local_10;
    uint _e346 = asuint(asfloat(_e344));
    uint _e351 = (_e346 & 2147483647u);
    bool _e353 = ((_e346 & 2147483648u) != 0u);
    uint _e360 = (1u << ((150u - (_e351 >> 23u)) & 31u));
    uint _e361 = (_e360 - 1u);
    uint _e365 = (_e346 & ~(_e361));
    local_11 = ((_e351 > 2139095040u) ? 2143289344u : ((_e351 >= 1258291200u) ? _e346 : ((_e351 < 1065353216u) ? ((_e351 == 0u) ? _e346 : (_e353 ? 3212836864u : 0u)) : ((_e353 && ((_e346 & _e361) != 0u)) ? (_e365 + _e360) : _e365))));
    uint _e384 = local_11;
    out_.Store(int(11)*4, asuint(asuint(asfloat(_e384))));
    uint _e390 = asuint(3.4028235e38);
    uint _e395 = (_e390 & 2147483647u);
    bool _e397 = ((_e390 & 2147483648u) != 0u);
    uint _e404 = (1u << ((150u - (_e395 >> 23u)) & 31u));
    uint _e405 = (_e404 - 1u);
    uint _e409 = (_e390 & ~(_e405));
    local_12 = ((_e395 > 2139095040u) ? 2143289344u : ((_e395 >= 1258291200u) ? _e390 : ((_e395 < 1065353216u) ? ((_e395 == 0u) ? _e390 : (_e397 ? 3212836864u : 0u)) : ((_e397 && ((_e390 & _e405) != 0u)) ? (_e409 + _e404) : _e409))));
    uint _e428 = local_12;
    out_.Store(int(12)*4, asuint(asuint(asfloat(_e428))));
    uint _e435 = asuint(-(3.4028235e38));
    uint _e440 = (_e435 & 2147483647u);
    bool _e442 = ((_e435 & 2147483648u) != 0u);
    uint _e449 = (1u << ((150u - (_e440 >> 23u)) & 31u));
    uint _e450 = (_e449 - 1u);
    uint _e454 = (_e435 & ~(_e450));
    local_13 = ((_e440 > 2139095040u) ? 2143289344u : ((_e440 >= 1258291200u) ? _e435 : ((_e440 < 1065353216u) ? ((_e440 == 0u) ? _e435 : (_e442 ? 3212836864u : 0u)) : ((_e442 && ((_e435 & _e450) != 0u)) ? (_e454 + _e449) : _e454))));
    uint _e473 = local_13;
    out_.Store(int(13)*4, asuint(asuint(asfloat(_e473))));
    local_14 = 2139095039u;
    uint _e480 = local_14;
    uint _e482 = asuint(asfloat(_e480));
    uint _e487 = (_e482 & 2147483647u);
    bool _e489 = ((_e482 & 2147483648u) != 0u);
    uint _e496 = (1u << ((150u - (_e487 >> 23u)) & 31u));
    uint _e497 = (_e496 - 1u);
    uint _e501 = (_e482 & ~(_e497));
    local_15 = ((_e487 > 2139095040u) ? 2143289344u : ((_e487 >= 1258291200u) ? _e482 : ((_e487 < 1065353216u) ? ((_e487 == 0u) ? _e482 : (_e489 ? 3212836864u : 0u)) : ((_e489 && ((_e482 & _e497) != 0u)) ? (_e501 + _e496) : _e501))));
    uint _e520 = local_15;
    out_.Store(int(14)*4, asuint(asuint(asfloat(_e520))));
    local_16 = 4286578687u;
    uint _e527 = local_16;
    uint _e529 = asuint(asfloat(_e527));
    uint _e534 = (_e529 & 2147483647u);
    bool _e536 = ((_e529 & 2147483648u) != 0u);
    uint _e543 = (1u << ((150u - (_e534 >> 23u)) & 31u));
    uint _e544 = (_e543 - 1u);
    uint _e548 = (_e529 & ~(_e544));
    local_17 = ((_e534 > 2139095040u) ? 2143289344u : ((_e534 >= 1258291200u) ? _e529 : ((_e534 < 1065353216u) ? ((_e534 == 0u) ? _e529 : (_e536 ? 3212836864u : 0u)) : ((_e536 && ((_e529 & _e544) != 0u)) ? (_e548 + _e543) : _e548))));
    uint _e567 = local_17;
    out_.Store(int(15)*4, asuint(asuint(asfloat(_e567))));
    local_18 = 2139095040u;
    uint _e574 = local_18;
    uint _e576 = asuint(asfloat(_e574));
    uint _e581 = (_e576 & 2147483647u);
    bool _e583 = ((_e576 & 2147483648u) != 0u);
    uint _e590 = (1u << ((150u - (_e581 >> 23u)) & 31u));
    uint _e591 = (_e590 - 1u);
    uint _e595 = (_e576 & ~(_e591));
    local_19 = ((_e581 > 2139095040u) ? 2143289344u : ((_e581 >= 1258291200u) ? _e576 : ((_e581 < 1065353216u) ? ((_e581 == 0u) ? _e576 : (_e583 ? 3212836864u : 0u)) : ((_e583 && ((_e576 & _e591) != 0u)) ? (_e595 + _e590) : _e595))));
    uint _e614 = local_19;
    out_.Store(int(16)*4, asuint(asuint(asfloat(_e614))));
    local_20 = 4286578688u;
    uint _e621 = local_20;
    uint _e623 = asuint(asfloat(_e621));
    uint _e628 = (_e623 & 2147483647u);
    bool _e630 = ((_e623 & 2147483648u) != 0u);
    uint _e637 = (1u << ((150u - (_e628 >> 23u)) & 31u));
    uint _e638 = (_e637 - 1u);
    uint _e642 = (_e623 & ~(_e638));
    local_21 = ((_e628 > 2139095040u) ? 2143289344u : ((_e628 >= 1258291200u) ? _e623 : ((_e628 < 1065353216u) ? ((_e628 == 0u) ? _e623 : (_e630 ? 3212836864u : 0u)) : ((_e630 && ((_e623 & _e638) != 0u)) ? (_e642 + _e637) : _e642))));
    uint _e661 = local_21;
    out_.Store(int(17)*4, asuint(asuint(asfloat(_e661))));
    local_22 = 2143363909u;
    uint _e668 = local_22;
    uint _e670 = asuint(asfloat(_e668));
    uint _e675 = (_e670 & 2147483647u);
    bool _e677 = ((_e670 & 2147483648u) != 0u);
    uint _e684 = (1u << ((150u - (_e675 >> 23u)) & 31u));
    uint _e685 = (_e684 - 1u);
    uint _e689 = (_e670 & ~(_e685));
    local_23 = ((_e675 > 2139095040u) ? 2143289344u : ((_e675 >= 1258291200u) ? _e670 : ((_e675 < 1065353216u) ? ((_e675 == 0u) ? _e670 : (_e677 ? 3212836864u : 0u)) : ((_e677 && ((_e670 & _e685) != 0u)) ? (_e689 + _e684) : _e689))));
    uint _e708 = local_23;
    out_.Store(int(18)*4, asuint(asuint(asfloat(_e708))));
    local_24 = 2139095041u;
    uint _e715 = local_24;
    uint _e717 = asuint(asfloat(_e715));
    uint _e722 = (_e717 & 2147483647u);
    bool _e724 = ((_e717 & 2147483648u) != 0u);
    uint _e731 = (1u << ((150u - (_e722 >> 23u)) & 31u));
    uint _e732 = (_e731 - 1u);
    uint _e736 = (_e717 & ~(_e732));
    local_25 = ((_e722 > 2139095040u) ? 2143289344u : ((_e722 >= 1258291200u) ? _e717 : ((_e722 < 1065353216u) ? ((_e722 == 0u) ? _e717 : (_e724 ? 3212836864u : 0u)) : ((_e724 && ((_e717 & _e732) != 0u)) ? (_e736 + _e731) : _e736))));
    uint _e755 = local_25;
    out_.Store(int(19)*4, asuint(asuint(asfloat(_e755))));
    value = 2.75;
    float _e762 = value;
    value = (_e762 + float(int(1)));
    const float _e766 = floor_value(_e762);
    out_.Store(int(20)*4, asuint(asuint(_e766)));
    float _e770 = value;
    out_.Store(int(21)*4, asuint(asuint(_e770)));
    const int _e775 = floorf(true);
    out_.Store(int(22)*4, asuint(uint(_e775)));
    const int _e780 = floorf(false);
    out_.Store(int(23)*4, asuint(uint(_e780)));
    const float _e786 = floorf_1(2.0, 3.0);
    out_.Store(int(24)*4, asuint(asuint(_e786)));
    local_26 = 1258291199u;
    uint _e792 = local_26;
    uint _e794 = asuint(asfloat(_e792));
    uint _e799 = (_e794 & 2147483647u);
    bool _e801 = ((_e794 & 2147483648u) != 0u);
    uint _e808 = (1u << ((150u - (_e799 >> 23u)) & 31u));
    uint _e809 = (_e808 - 1u);
    uint _e813 = (_e794 & ~(_e809));
    local_27 = ((_e799 > 2139095040u) ? 2143289344u : ((_e799 >= 1258291200u) ? _e794 : ((_e799 < 1065353216u) ? ((_e799 == 0u) ? _e794 : (_e801 ? 3212836864u : 0u)) : ((_e801 && ((_e794 & _e809) != 0u)) ? (_e813 + _e808) : _e813))));
    uint _e832 = local_27;
    out_.Store(int(25)*4, asuint(asuint(asfloat(_e832))));
    local_28 = 3405774847u;
    uint _e839 = local_28;
    uint _e841 = asuint(asfloat(_e839));
    uint _e846 = (_e841 & 2147483647u);
    bool _e848 = ((_e841 & 2147483648u) != 0u);
    uint _e855 = (1u << ((150u - (_e846 >> 23u)) & 31u));
    uint _e856 = (_e855 - 1u);
    uint _e860 = (_e841 & ~(_e856));
    local_29 = ((_e846 > 2139095040u) ? 2143289344u : ((_e846 >= 1258291200u) ? _e841 : ((_e846 < 1065353216u) ? ((_e846 == 0u) ? _e841 : (_e848 ? 3212836864u : 0u)) : ((_e848 && ((_e841 & _e856) != 0u)) ? (_e860 + _e855) : _e860))));
    uint _e879 = local_29;
    out_.Store(int(26)*4, asuint(asuint(asfloat(_e879))));
    const float _e885 = floor_value_2(4294967295u);
    out_.Store(int(27)*4, asuint(asuint(_e885)));
    const float _e893 = floor_value_1(asint(asuint(naga_neg(int(2147483647))) - asuint(int(1))));
    out_.Store(int(28)*4, asuint(asuint(_e893)));
    uint _e898 = asuint(10000000000.0);
    uint _e903 = (_e898 & 2147483647u);
    bool _e905 = ((_e898 & 2147483648u) != 0u);
    uint _e912 = (1u << ((150u - (_e903 >> 23u)) & 31u));
    uint _e913 = (_e912 - 1u);
    uint _e917 = (_e898 & ~(_e913));
    local_30 = ((_e903 > 2139095040u) ? 2143289344u : ((_e903 >= 1258291200u) ? _e898 : ((_e903 < 1065353216u) ? ((_e903 == 0u) ? _e898 : (_e905 ? 3212836864u : 0u)) : ((_e905 && ((_e898 & _e913) != 0u)) ? (_e917 + _e912) : _e917))));
    uint _e936 = local_30;
    out_.Store(int(29)*4, asuint(asuint(asfloat(_e936))));
}
