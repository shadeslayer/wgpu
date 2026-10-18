#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct _6
{
    float _m0;
    float _m1;
    float _m2;
};

layout(set = 0, binding = 0, std430) buffer _12_11
{
    uint _m0[];
} _11;

float _142(float _141)
{
    float _144 = 0.0;
    uint _146 = 0u;
    _144 = _141;
    uint _151 = floatBitsToUint(_144) & 2147483647u;
    _146 = (_151 > 2139095040u) ? 2143289344u : _151;
    return uintBitsToFloat(_146);
}

float _158(int _157)
{
    int _160 = 0;
    uint _163 = 0u;
    _160 = _157;
    uint _169 = floatBitsToUint(float(_160)) & 2147483647u;
    _163 = (_169 > 2139095040u) ? 2143289344u : _169;
    return uintBitsToFloat(_163);
}

float _176(uint _175)
{
    uint _178 = 0u;
    uint _180 = 0u;
    _178 = _175;
    uint _186 = floatBitsToUint(float(_178)) & 2147483647u;
    _180 = (_186 > 2139095040u) ? 2143289344u : _186;
    return uintBitsToFloat(_180);
}

float _193(bool _192)
{
    bool _195 = false;
    uint _198 = 0u;
    _195 = _192;
    uint _206 = floatBitsToUint(float(_195)) & 2147483647u;
    _198 = (_206 > 2139095040u) ? 2143289344u : _206;
    return uintBitsToFloat(_198);
}

int _213(int _212)
{
    int _216 = 0;
    _216 = _212;
    return _216 + 40;
}

uint _223(uint _222)
{
    uint _226 = 0u;
    _226 = _222;
    return _226 + 50u;
}

int _233(bool _232)
{
    bool _237 = false;
    int _239 = 0;
    _237 = _232;
    if (_237)
    {
        _239 = 7;
    }
    else
    {
        _239 = 9;
    }
    return _239;
}

int _249(bool _248)
{
    bool _252 = false;
    int _254 = 0;
    _252 = _248;
    if (_252)
    {
        _254 = 11;
    }
    else
    {
        _254 = 13;
    }
    return _254;
}

_6 _52(_6 _51)
{
    uint _57 = 0u;
    uint _61 = 0u;
    _6 _54 = _6(0.0, 0.0, 0.0);
    uint _59 = 0u;
    _54 = _51;
    uint _67 = floatBitsToUint(_54._m0) & 2147483647u;
    _57 = (_67 > 2139095040u) ? 2143289344u : _67;
    uint _75 = floatBitsToUint(_54._m1) & 2147483647u;
    _59 = (_75 > 2139095040u) ? 2143289344u : _75;
    uint _84 = floatBitsToUint(_54._m2) & 2147483647u;
    _61 = (_84 > 2139095040u) ? 2143289344u : _84;
    return _6(uintBitsToFloat(_57), uintBitsToFloat(_59), uintBitsToFloat(_61));
}

float _265(float _263, float _264)
{
    float _267 = 0.0;
    float _269 = 0.0;
    _267 = _263;
    _269 = _264;
    return _267 + _269;
}

vec2 _16(vec2 _15)
{
    vec2 _21 = vec2(0.0);
    uint _24 = 0u;
    uint _27 = 0u;
    _21 = _15;
    uint _35 = floatBitsToUint(_21.x) & 2147483647u;
    _24 = (_35 > 2139095040u) ? 2143289344u : _35;
    uint _44 = floatBitsToUint(_21.y) & 2147483647u;
    _27 = (_44 > 2139095040u) ? 2143289344u : _44;
    return vec2(uintBitsToFloat(_24), uintBitsToFloat(_27));
}

vec4 _92(vec4 _91)
{
    uint _103 = 0u;
    uint _97 = 0u;
    uint _101 = 0u;
    vec4 _94 = vec4(0.0);
    uint _99 = 0u;
    _94 = _91;
    uint _109 = floatBitsToUint(_94.x) & 2147483647u;
    _97 = (_109 > 2139095040u) ? 2143289344u : _109;
    uint _117 = floatBitsToUint(_94.y) & 2147483647u;
    _99 = (_117 > 2139095040u) ? 2143289344u : _117;
    uint _125 = floatBitsToUint(_94.z) & 2147483647u;
    _101 = (_125 > 2139095040u) ? 2143289344u : _125;
    uint _134 = floatBitsToUint(_94.w) & 2147483647u;
    _103 = (_134 > 2139095040u) ? 2143289344u : _134;
    return vec4(uintBitsToFloat(_97), uintBitsToFloat(_99), uintBitsToFloat(_101), uintBitsToFloat(_103));
}

void main()
{
    vec2 _373 = vec2(0.0);
    uint _367 = 0u;
    uint _361 = 0u;
    uint _355 = 0u;
    uint _349 = 0u;
    uint _343 = 0u;
    uint _337 = 0u;
    uint _331 = 0u;
    uint _325 = 0u;
    uint _371 = 0u;
    float _365 = 0.0;
    uint _359 = 0u;
    uint _353 = 0u;
    uint _347 = 0u;
    uint _341 = 0u;
    uint _335 = 0u;
    uint _329 = 0u;
    uint _323 = 0u;
    vec4 _375 = vec4(0.0);
    _6 _369 = _6(0.0, 0.0, 0.0);
    uint _363 = 0u;
    uint _357 = 0u;
    uint _351 = 0u;
    uint _345 = 0u;
    uint _339 = 0u;
    uint _333 = 0u;
    uint _327 = 0u;
    uint _379 = floatBitsToUint(0.0) & 2147483647u;
    _323 = (_379 > 2139095040u) ? 2143289344u : _379;
    _11._m0[0u] = floatBitsToUint(uintBitsToFloat(_323));
    float _387 = -0.0;
    uint _389 = floatBitsToUint(_387) & 2147483647u;
    _325 = (_389 > 2139095040u) ? 2143289344u : _389;
    _11._m0[1u] = floatBitsToUint(uintBitsToFloat(_325));
    float _396 = -1.75;
    uint _398 = floatBitsToUint(_396) & 2147483647u;
    _327 = (_398 > 2139095040u) ? 2143289344u : _398;
    _11._m0[2u] = floatBitsToUint(uintBitsToFloat(_327));
    float _405 = -1.25;
    _11._m0[3u] = floatBitsToUint(_142(_405));
    int _409 = -3;
    _11._m0[4u] = floatBitsToUint(_158(_409));
    _11._m0[5u] = floatBitsToUint(_176(4294967295u));
    _11._m0[6u] = floatBitsToUint(_193(true));
    _11._m0[7u] = floatBitsToUint(_193(false));
    _329 = 1u;
    uint _429 = floatBitsToUint(uintBitsToFloat(_329)) & 2147483647u;
    _331 = (_429 > 2139095040u) ? 2143289344u : _429;
    _11._m0[8u] = floatBitsToUint(uintBitsToFloat(_331));
    _333 = 2147483649u;
    uint _440 = floatBitsToUint(uintBitsToFloat(_333)) & 2147483647u;
    _335 = (_440 > 2139095040u) ? 2143289344u : _440;
    _11._m0[9u] = floatBitsToUint(uintBitsToFloat(_335));
    _337 = 8388607u;
    uint _451 = floatBitsToUint(uintBitsToFloat(_337)) & 2147483647u;
    _339 = (_451 > 2139095040u) ? 2143289344u : _451;
    _11._m0[10u] = floatBitsToUint(uintBitsToFloat(_339));
    _341 = 2155872255u;
    uint _462 = floatBitsToUint(uintBitsToFloat(_341)) & 2147483647u;
    _343 = (_462 > 2139095040u) ? 2143289344u : _462;
    _11._m0[11u] = floatBitsToUint(uintBitsToFloat(_343));
    uint _471 = floatBitsToUint(3.4028234663852885981170418348452e+38) & 2147483647u;
    _345 = (_471 > 2139095040u) ? 2143289344u : _471;
    _11._m0[12u] = floatBitsToUint(uintBitsToFloat(_345));
    float _479 = -3.4028234663852885981170418348452e+38;
    uint _481 = floatBitsToUint(_479) & 2147483647u;
    _347 = (_481 > 2139095040u) ? 2143289344u : _481;
    _11._m0[13u] = floatBitsToUint(uintBitsToFloat(_347));
    _349 = 2139095040u;
    uint _492 = floatBitsToUint(uintBitsToFloat(_349)) & 2147483647u;
    _351 = (_492 > 2139095040u) ? 2143289344u : _492;
    _11._m0[14u] = floatBitsToUint(uintBitsToFloat(_351));
    _353 = 4286578688u;
    uint _503 = floatBitsToUint(uintBitsToFloat(_353)) & 2147483647u;
    _355 = (_503 > 2139095040u) ? 2143289344u : _503;
    _11._m0[15u] = floatBitsToUint(uintBitsToFloat(_355));
    _357 = 2143363909u;
    uint _514 = floatBitsToUint(uintBitsToFloat(_357)) & 2147483647u;
    _359 = (_514 > 2139095040u) ? 2143289344u : _514;
    _11._m0[16u] = floatBitsToUint(uintBitsToFloat(_359));
    _361 = 2139095041u;
    uint _525 = floatBitsToUint(uintBitsToFloat(_361)) & 2147483647u;
    _363 = (_525 > 2139095040u) ? 2143289344u : _525;
    _11._m0[17u] = floatBitsToUint(uintBitsToFloat(_363));
    float _533 = -2.75;
    _365 = _533;
    float _534 = _365;
    float _535 = float(1);
    _365 = _534 + _535;
    uint _538 = floatBitsToUint(_534) & 2147483647u;
    _367 = (_538 > 2139095040u) ? 2143289344u : _538;
    _11._m0[18u] = floatBitsToUint(uintBitsToFloat(_367));
    _11._m0[19u] = floatBitsToUint(_365);
    int _550 = -3;
    _11._m0[20u] = uint(_213(_550));
    _11._m0[21u] = _223(3u);
    _11._m0[22u] = uint(_233(true));
    _11._m0[23u] = uint(_233(false));
    _11._m0[24u] = uint(_249(true));
    float _570 = -2.5;
    float _571 = -0.0;
    _371 = 2147483649u;
    _369 = _52(_6(_570, _571, uintBitsToFloat(_371)));
    _11._m0[25u] = floatBitsToUint(_369._m0);
    _11._m0[26u] = floatBitsToUint(_369._m1);
    _11._m0[27u] = floatBitsToUint(_369._m2);
    float _591 = -2.0;
    _11._m0[28u] = floatBitsToUint(_265(_591, 3.0));
    float _596 = -3.0;
    float _597 = -4.0;
    _373 = _16(vec2(_596, _597));
    float _600 = -1.0;
    float _601 = -2.0;
    float _602 = -3.0;
    float _603 = -4.0;
    _375 = _92(vec4(_600, _601, _602, _603));
    _11._m0[29u] = floatBitsToUint(((_373.x + _373.y) + _375.x) + _375.w);
}

