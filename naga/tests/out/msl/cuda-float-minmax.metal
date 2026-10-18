// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef float type_1[1];

float min(
    float first_1,
    float second_1,
    float third
) {
    float first_2 = {};
    float second_2 = {};
    float third_1 = {};
    first_2 = first_1;
    second_2 = second_1;
    third_1 = third;
    float _e6 = first_2;
    float _e7 = second_2;
    uint _e12 = as_type<uint>(_e6);
    bool _e18 = (_e12 & 2147483647u) > 2139095040u;
    uint _e19 = as_type<uint>(_e7);
    bool _e25 = (_e19 & 2147483647u) > 2139095040u;
    float _e33 = third_1;
    uint _e38 = as_type<uint>(as_type<float>((_e18 && _e25) ? 2143289344u : (_e25 ? _e12 : (_e18 ? _e19 : ((((_e12 >= 2147483648u) ? ~(_e12) : (_e12 ^ 2147483648u)) < ((_e19 >= 2147483648u) ? ~(_e19) : (_e19 ^ 2147483648u))) ? _e12 : _e19)))));
    bool _e44 = (_e38 & 2147483647u) > 2139095040u;
    uint _e45 = as_type<uint>(_e33);
    bool _e51 = (_e45 & 2147483647u) > 2139095040u;
    return as_type<float>((_e44 && _e51) ? 2143289344u : (_e51 ? _e38 : (_e44 ? _e45 : ((((_e38 >= 2147483648u) ? ~(_e38) : (_e38 ^ 2147483648u)) < ((_e45 >= 2147483648u) ? ~(_e45) : (_e45 ^ 2147483648u))) ? _e38 : _e45))));
}

int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}

float bounded(
    float value_1
) {
    float value_2 = {};
    value_2 = value_1;
    float _e5 = value_2;
    uint _e12 = as_type<uint>(_e5);
    bool _e18 = (_e12 & 2147483647u) > 2139095040u;
    uint _e19 = as_type<uint>(static_cast<float>(4));
    bool _e25 = (_e19 & 2147483647u) > 2139095040u;
    uint _e37 = as_type<uint>(static_cast<float>(naga_neg(2)));
    bool _e43 = (_e37 & 2147483647u) > 2139095040u;
    uint _e44 = as_type<uint>(as_type<float>((_e18 && _e25) ? 2143289344u : (_e25 ? _e12 : (_e18 ? _e19 : ((((_e12 >= 2147483648u) ? ~(_e12) : (_e12 ^ 2147483648u)) < ((_e19 >= 2147483648u) ? ~(_e19) : (_e19 ^ 2147483648u))) ? _e12 : _e19)))));
    bool _e50 = (_e44 & 2147483647u) > 2139095040u;
    return as_type<float>((_e43 && _e50) ? 2143289344u : (_e50 ? _e37 : (_e43 ? _e44 : ((((_e37 >= 2147483648u) ? ~(_e37) : (_e37 ^ 2147483648u)) > ((_e44 >= 2147483648u) ? ~(_e44) : (_e44 ^ 2147483648u))) ? _e37 : _e44))));
}

kernel void float_minmax(
  device type_1& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    float seed = {};
    float first = {};
    float second = {};
    float value = {};
    float _e4 = out[0];
    seed = _e4;
    float _e7 = seed;
    uint _e14 = as_type<uint>(_e7);
    bool _e20 = (_e14 & 2147483647u) > 2139095040u;
    uint _e21 = as_type<uint>(-(7.5));
    bool _e27 = (_e21 & 2147483647u) > 2139095040u;
    out[0] = as_type<float>((_e20 && _e27) ? 2143289344u : (_e27 ? _e14 : (_e20 ? _e21 : ((((_e14 >= 2147483648u) ? ~(_e14) : (_e14 ^ 2147483648u)) < ((_e21 >= 2147483648u) ? ~(_e21) : (_e21 ^ 2147483648u))) ? _e14 : _e21))));
    float _e37 = seed;
    uint _e44 = as_type<uint>(_e37);
    bool _e50 = (_e44 & 2147483647u) > 2139095040u;
    uint _e51 = as_type<uint>(-(7.5));
    bool _e57 = (_e51 & 2147483647u) > 2139095040u;
    out[1] = as_type<float>((_e50 && _e57) ? 2143289344u : (_e57 ? _e44 : (_e50 ? _e51 : ((((_e44 >= 2147483648u) ? ~(_e44) : (_e44 ^ 2147483648u)) > ((_e51 >= 2147483648u) ? ~(_e51) : (_e51 ^ 2147483648u))) ? _e44 : _e51))));
    float _e67 = seed;
    uint _e73 = as_type<uint>(_e67);
    bool _e79 = (_e73 & 2147483647u) > 2139095040u;
    uint _e80 = as_type<uint>(1.5);
    bool _e86 = (_e80 & 2147483647u) > 2139095040u;
    out[2] = as_type<float>((_e79 && _e86) ? 2143289344u : (_e86 ? _e73 : (_e79 ? _e80 : ((((_e73 >= 2147483648u) ? ~(_e73) : (_e73 ^ 2147483648u)) < ((_e80 >= 2147483648u) ? ~(_e80) : (_e80 ^ 2147483648u))) ? _e73 : _e80))));
    float _e98 = seed;
    uint _e103 = as_type<uint>(-(2.25));
    bool _e109 = (_e103 & 2147483647u) > 2139095040u;
    uint _e110 = as_type<uint>(_e98);
    bool _e116 = (_e110 & 2147483647u) > 2139095040u;
    out[3] = as_type<float>((_e109 && _e116) ? 2143289344u : (_e116 ? _e103 : (_e109 ? _e110 : ((((_e103 >= 2147483648u) ? ~(_e103) : (_e103 ^ 2147483648u)) > ((_e110 >= 2147483648u) ? ~(_e110) : (_e110 ^ 2147483648u))) ? _e103 : _e110))));
    uint _e135 = as_type<uint>(static_cast<float>(naga_neg(3)));
    bool _e141 = (_e135 & 2147483647u) > 2139095040u;
    uint _e142 = as_type<uint>(static_cast<float>(5u));
    bool _e148 = (_e142 & 2147483647u) > 2139095040u;
    out[4] = as_type<float>((_e141 && _e148) ? 2143289344u : (_e148 ? _e135 : (_e141 ? _e142 : ((((_e135 >= 2147483648u) ? ~(_e135) : (_e135 ^ 2147483648u)) < ((_e142 >= 2147483648u) ? ~(_e142) : (_e142 ^ 2147483648u))) ? _e135 : _e142))));
    uint _e167 = as_type<uint>(static_cast<float>(5u));
    bool _e173 = (_e167 & 2147483647u) > 2139095040u;
    uint _e174 = as_type<uint>(static_cast<float>(naga_neg(3)));
    bool _e180 = (_e174 & 2147483647u) > 2139095040u;
    out[5] = as_type<float>((_e173 && _e180) ? 2143289344u : (_e180 ? _e167 : (_e173 ? _e174 : ((((_e167 >= 2147483648u) ? ~(_e167) : (_e167 ^ 2147483648u)) > ((_e174 >= 2147483648u) ? ~(_e174) : (_e174 ^ 2147483648u))) ? _e167 : _e174))));
    uint _e198 = as_type<uint>(static_cast<float>(true));
    bool _e204 = (_e198 & 2147483647u) > 2139095040u;
    uint _e205 = as_type<uint>(static_cast<float>(false));
    bool _e211 = (_e205 & 2147483647u) > 2139095040u;
    out[6] = as_type<float>((_e204 && _e211) ? 2143289344u : (_e211 ? _e198 : (_e204 ? _e205 : ((((_e198 >= 2147483648u) ? ~(_e198) : (_e198 ^ 2147483648u)) < ((_e205 >= 2147483648u) ? ~(_e205) : (_e205 ^ 2147483648u))) ? _e198 : _e205))));
    uint _e229 = as_type<uint>(static_cast<float>(false));
    bool _e235 = (_e229 & 2147483647u) > 2139095040u;
    uint _e236 = as_type<uint>(static_cast<float>(true));
    bool _e242 = (_e236 & 2147483647u) > 2139095040u;
    out[7] = as_type<float>((_e235 && _e242) ? 2143289344u : (_e242 ? _e229 : (_e235 ? _e236 : ((((_e229 >= 2147483648u) ? ~(_e229) : (_e229 ^ 2147483648u)) > ((_e236 >= 2147483648u) ? ~(_e236) : (_e236 ^ 2147483648u))) ? _e229 : _e236))));
    float _e252 = seed;
    uint _e259 = as_type<uint>(_e252);
    bool _e265 = (_e259 & 2147483647u) > 2139095040u;
    uint _e266 = as_type<uint>(-(4.0));
    bool _e272 = (_e266 & 2147483647u) > 2139095040u;
    uint _e285 = as_type<uint>(as_type<float>((_e265 && _e272) ? 2143289344u : (_e272 ? _e259 : (_e265 ? _e266 : ((((_e259 >= 2147483648u) ? ~(_e259) : (_e259 ^ 2147483648u)) > ((_e266 >= 2147483648u) ? ~(_e266) : (_e266 ^ 2147483648u))) ? _e259 : _e266)))));
    bool _e291 = (_e285 & 2147483647u) > 2139095040u;
    uint _e292 = as_type<uint>(2.5);
    bool _e298 = (_e292 & 2147483647u) > 2139095040u;
    out[8] = as_type<float>((_e291 && _e298) ? 2143289344u : (_e298 ? _e285 : (_e291 ? _e292 : ((((_e285 >= 2147483648u) ? ~(_e285) : (_e285 ^ 2147483648u)) < ((_e292 >= 2147483648u) ? ~(_e292) : (_e292 ^ 2147483648u))) ? _e285 : _e292))));
    float _e308 = seed;
    float _e312 = min(_e308, 1.25, -(3.5));
    out[9] = _e312;
    float _e315 = seed;
    float _e316 = bounded(_e315);
    out[10] = _e316;
    first = 1.25;
    second = 2.5;
    float _e323 = first;
    first = _e323 + static_cast<float>(1);
    float _e327 = second;
    second = _e327 + static_cast<float>(1);
    uint _e335 = as_type<uint>(_e323);
    bool _e341 = (_e335 & 2147483647u) > 2139095040u;
    uint _e342 = as_type<uint>(_e327);
    bool _e348 = (_e342 & 2147483647u) > 2139095040u;
    out[11] = as_type<float>((_e341 && _e348) ? 2143289344u : (_e348 ? _e335 : (_e341 ? _e342 : ((((_e335 >= 2147483648u) ? ~(_e335) : (_e335 ^ 2147483648u)) > ((_e342 >= 2147483648u) ? ~(_e342) : (_e342 ^ 2147483648u))) ? _e335 : _e342))));
    float _e358 = first;
    out[12] = _e358;
    float _e361 = second;
    out[13] = _e361;
    float _e363 = seed;
    value = _e363;
    float _e366 = value;
    float _e367 = value;
    value = _e367 + static_cast<float>(1);
    uint _e375 = as_type<uint>(_e366);
    bool _e381 = (_e375 & 2147483647u) > 2139095040u;
    uint _e382 = as_type<uint>(_e367);
    bool _e388 = (_e382 & 2147483647u) > 2139095040u;
    out[14] = as_type<float>((_e381 && _e388) ? 2143289344u : (_e388 ? _e375 : (_e381 ? _e382 : ((((_e375 >= 2147483648u) ? ~(_e375) : (_e375 ^ 2147483648u)) > ((_e382 >= 2147483648u) ? ~(_e382) : (_e382 ^ 2147483648u))) ? _e375 : _e382))));
    float _e398 = value;
    out[15] = _e398;
    uint _e408 = as_type<uint>(static_cast<float>(16777217));
    bool _e414 = (_e408 & 2147483647u) > 2139095040u;
    uint _e415 = as_type<uint>(16777216.0);
    bool _e421 = (_e415 & 2147483647u) > 2139095040u;
    out[16] = as_type<float>((_e414 && _e421) ? 2143289344u : (_e421 ? _e408 : (_e414 ? _e415 : ((((_e408 >= 2147483648u) ? ~(_e408) : (_e408 ^ 2147483648u)) > ((_e415 >= 2147483648u) ? ~(_e415) : (_e415 ^ 2147483648u))) ? _e408 : _e415))));
    uint _e438 = as_type<uint>(static_cast<float>(4294967295u));
    bool _e444 = (_e438 & 2147483647u) > 2139095040u;
    uint _e445 = as_type<uint>(4294967300.0);
    bool _e451 = (_e445 & 2147483647u) > 2139095040u;
    out[17] = as_type<float>((_e444 && _e451) ? 2143289344u : (_e451 ? _e438 : (_e444 ? _e445 : ((((_e438 >= 2147483648u) ? ~(_e438) : (_e438 ^ 2147483648u)) < ((_e445 >= 2147483648u) ? ~(_e445) : (_e445 ^ 2147483648u))) ? _e438 : _e445))));
    float _e463 = seed;
    uint _e468 = as_type<uint>(static_cast<float>(true));
    bool _e474 = (_e468 & 2147483647u) > 2139095040u;
    uint _e475 = as_type<uint>(_e463);
    bool _e481 = (_e475 & 2147483647u) > 2139095040u;
    out[18] = as_type<float>((_e474 && _e481) ? 2143289344u : (_e481 ? _e468 : (_e474 ? _e475 : ((((_e468 >= 2147483648u) ? ~(_e468) : (_e468 ^ 2147483648u)) > ((_e475 >= 2147483648u) ? ~(_e475) : (_e475 ^ 2147483648u))) ? _e468 : _e475))));
    float _e491 = seed;
    float _e492 = seed;
    uint _e497 = as_type<uint>(_e491);
    bool _e503 = (_e497 & 2147483647u) > 2139095040u;
    uint _e504 = as_type<uint>(_e492);
    bool _e510 = (_e504 & 2147483647u) > 2139095040u;
    out[19] = as_type<float>((_e503 && _e510) ? 2143289344u : (_e510 ? _e497 : (_e503 ? _e504 : ((((_e497 >= 2147483648u) ? ~(_e497) : (_e497 ^ 2147483648u)) < ((_e504 >= 2147483648u) ? ~(_e504) : (_e504 ^ 2147483648u))) ? _e497 : _e504))));
}
