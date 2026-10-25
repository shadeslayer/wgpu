#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0, std430) buffer _11_10
{
    uint _m0[];
} _10;

int _16(float _14, float _15)
{
    float _20 = 0.0;
    float _23 = 0.0;
    _20 = _14;
    _23 = _15;
    uint _36 = min(0u, 32u);
    uint _41 = min(0u, 32u);
    return int(uint(bitfieldExtract(int(clamp(_20, -2147483648.0, 2147483520.0)), int(_36), int(min(24u, (32u - _36))))) * uint(bitfieldExtract(int(clamp(_23, -2147483648.0, 2147483520.0)), int(_41), int(min(24u, (32u - _41))))));
}

uint _50(float _48, float _49)
{
    float _52 = 0.0;
    float _54 = 0.0;
    _52 = _48;
    _54 = _49;
    uint _66 = min(0u, 32u);
    uint _70 = min(0u, 32u);
    return bitfieldExtract(uint(clamp(_52, 0.0, 4294967040.0)), int(_66), int(min(24u, (32u - _66)))) * bitfieldExtract(uint(clamp(_54, 0.0, 4294967040.0)), int(_70), int(min(24u, (32u - _70))));
}

int _77(bool _75, bool _76)
{
    bool _85 = false;
    int _90 = 0;
    bool _82 = false;
    int _87 = 0;
    _82 = _75;
    _85 = _76;
    if (_82)
    {
        if (_85)
        {
            _87 = 71;
        }
        else
        {
            _87 = 73;
        }
        _90 = _87;
    }
    else
    {
        _90 = 79;
    }
    return _90;
}

int _106(float _104, float _105)
{
    float _108 = 0.0;
    float _110 = 0.0;
    _108 = _104;
    _110 = _105;
    return int(clamp((_108 * 10.0) + _110, -2147483648.0, 2147483520.0));
}

uint _122(int _120, int _121)
{
    int _125 = 0;
    int _127 = 0;
    _125 = _120;
    _127 = _121;
    return uint((_125 + _127) + 100);
}

int _138(int _136, int _137)
{
    int _140 = 0;
    int _142 = 0;
    _140 = _136;
    _142 = _137;
    uint _148 = min(0u, 32u);
    uint _153 = min(0u, 32u);
    return int(uint(bitfieldExtract(_140, int(_148), int(min(24u, (32u - _148))))) * uint(bitfieldExtract(_142, int(_153), int(min(24u, (32u - _153))))));
}

void main()
{
    int _233[2] = int[](0, 0);
    int _226[2] = int[](0, 0);
    int _219 = 0;
    int _235 = 0;
    int _229 = 0;
    uint _221 = 0u;
    int _237 = 0;
    int _231 = 0;
    int _224 = 0;
    uvec2 _597 = uvec2(4294967295u);
    uint _243 = min(0u, 32u);
    uint _248 = min(0u, 32u);
    _10._m0[0u] = uint(int(uint(bitfieldExtract(8388608, int(_243), int(min(24u, (32u - _243))))) * uint(bitfieldExtract(1, int(_248), int(min(24u, (32u - _248)))))));
    uint _258 = min(0u, 32u);
    uint _262 = min(0u, 32u);
    _10._m0[1u] = bitfieldExtract(16777215u, int(_258), int(min(24u, (32u - _258)))) * bitfieldExtract(16777215u, int(_262), int(min(24u, (32u - _262))));
    uint _269 = min(0u, 32u);
    uint _274 = min(0u, 32u);
    _10._m0[2u] = uint(int(uint(bitfieldExtract(8388607, int(_269), int(min(24u, (32u - _269))))) * uint(bitfieldExtract(8388607, int(_274), int(min(24u, (32u - _274)))))));
    uint _283 = min(0u, 32u);
    uint _287 = min(0u, 32u);
    _10._m0[3u] = bitfieldExtract(16777216u, int(_283), int(min(24u, (32u - _283)))) * bitfieldExtract(13u, int(_287), int(min(24u, (32u - _287))));
    float _292 = -2.75;
    _10._m0[4u] = uint(_16(3.75, _292));
    float _296 = -3.75;
    _10._m0[5u] = uint(_16(_296, 2.75));
    _10._m0[6u] = _50(3.75, 2.75);
    _10._m0[7u] = _50(16777215.0, 1.0);
    _10._m0[8u] = uint(_77(true, true));
    _10._m0[9u] = uint(_77(false, true));
    uint _316 = min(0u, 32u);
    uint _321 = min(0u, 32u);
    _10._m0[10u] = uint(int(uint(bitfieldExtract(3, int(_316), int(min(24u, (32u - _316))))) * uint(bitfieldExtract(int(true), int(_321), int(min(24u, (32u - _321)))))));
    uint _332 = min(0u, 32u);
    uint _337 = min(0u, 32u);
    _10._m0[11u] = uint(int(uint(bitfieldExtract(int(true), int(_332), int(min(24u, (32u - _332))))) * uint(bitfieldExtract(3, int(_337), int(min(24u, (32u - _337)))))));
    _10._m0[12u] = uint(_106(3.75, 2.0));
    _10._m0[13u] = _122(3, 4);
    uint _353 = min(0u, 32u);
    uint _357 = min(0u, 32u);
    _10._m0[14u] = bitfieldExtract(3u, int(_353), int(min(24u, (32u - _353)))) * bitfieldExtract(4u, int(_357), int(min(24u, (32u - _357))));
    _10._m0[15u] = _122(int(true), int(true));
    uint _370 = min(0u, 32u);
    uint _374 = min(0u, 32u);
    _10._m0[16u] = bitfieldExtract(3u, int(_370), int(min(24u, (32u - _370)))) * bitfieldExtract(uint(4), int(_374), int(min(24u, (32u - _374))));
    int _380 = -7;
    _10._m0[17u] = uint(_138(_380, 3));
    uint _386 = min(0u, 32u);
    uint _391 = min(0u, 32u);
    uint _398 = min(0u, 32u);
    uint _403 = min(0u, 32u);
    _10._m0[18u] = uint(int(uint(bitfieldExtract(int(uint(bitfieldExtract(3, int(_386), int(min(24u, (32u - _386))))) * uint(bitfieldExtract(4, int(_391), int(min(24u, (32u - _391)))))), int(_398), int(min(24u, (32u - _398))))) * uint(bitfieldExtract(5, int(_403), int(min(24u, (32u - _403)))))));
    uint _413 = min(0u, 32u);
    uint _417 = min(0u, 32u);
    uint _422 = min(0u, 32u);
    uint _426 = min(0u, 32u);
    _10._m0[19u] = bitfieldExtract(bitfieldExtract(16777215u, int(_413), int(min(24u, (32u - _413)))) * bitfieldExtract(2u, int(_417), int(min(24u, (32u - _417)))), int(_422), int(min(24u, (32u - _422)))) * bitfieldExtract(2u, int(_426), int(min(24u, (32u - _426))));
    _219 = -3;
    _221 = 5u;
    uint _436 = min(0u, 32u);
    uint _441 = min(0u, 32u);
    _10._m0[20u] = uint(int(uint(bitfieldExtract(_219, int(_436), int(min(24u, (32u - _436))))) * uint(bitfieldExtract(int(_221), int(_441), int(min(24u, (32u - _441)))))));
    uint _454 = min(0u, 32u);
    uint _458 = min(0u, 32u);
    _10._m0[21u] = bitfieldExtract(uint(_219), int(_454), int(min(24u, (32u - _454)))) * bitfieldExtract(_221, int(_458), int(min(24u, (32u - _458))));
    uint _465 = min(0u, 32u);
    uint _470 = min(0u, 32u);
    _10._m0[22u] = uint(int(uint(bitfieldExtract(25165824, int(_465), int(min(24u, (32u - _465))))) * uint(bitfieldExtract(16777217, int(_470), int(min(24u, (32u - _470)))))));
    uint _480 = min(0u, 32u);
    uint _484 = min(0u, 32u);
    _10._m0[23u] = bitfieldExtract(4286578688u, int(_480), int(min(24u, (32u - _480)))) * bitfieldExtract(4278190081u, int(_484), int(min(24u, (32u - _484))));
    if (true)
    {
        int _493 = -3;
        uint _495 = min(0u, 32u);
        uint _500 = min(0u, 32u);
        _224 = int(uint(bitfieldExtract(_493, int(_495), int(min(24u, (32u - _495))))) * uint(bitfieldExtract(7, int(_500), int(min(24u, (32u - _500))))));
    }
    else
    {
        uint _507 = min(0u, 32u);
        uint _512 = min(0u, 32u);
        _224 = int(uint(bitfieldExtract(0, int(_507), int(min(24u, (32u - _507))))) * uint(bitfieldExtract(0, int(_512), int(min(24u, (32u - _512))))));
    }
    _10._m0[24u] = uint(_224);
    uint _522 = min(0u, 32u);
    uint _527 = min(0u, 32u);
    _226[0u] = int(uint(bitfieldExtract(2, int(_522), int(min(24u, (32u - _522))))) * uint(bitfieldExtract(3, int(_527), int(min(24u, (32u - _527))))));
    uint _535 = min(0u, 32u);
    uint _540 = min(0u, 32u);
    _226[1u] = int(uint(bitfieldExtract(3, int(_535), int(min(24u, (32u - _535))))) * uint(bitfieldExtract(4, int(_540), int(min(24u, (32u - _540))))));
    _10._m0[25u] = uint(_226[0u] + _226[1u]);
    _229 = 2;
    _233 = int[](0, 0);
    _10._m0[26u] = uint(_229 + _233[0u]);
    _235 = 2;
    int _562 = _235;
    _235 = _562 + 1;
    int _564 = _235;
    _235 = _564 + 1;
    uint _567 = min(0u, 32u);
    uint _572 = min(0u, 32u);
    _10._m0[27u] = uint(int(uint(bitfieldExtract(_562, int(_567), int(min(24u, (32u - _567))))) * uint(bitfieldExtract(_564, int(_572), int(min(24u, (32u - _572)))))));
    _10._m0[28u] = uint(_235);
    _237 = 0;
    int _239 = 0;
    for (;;)
    {
        if (all(equal(uvec2(0u), _597)))
        {
            break;
        }
        _597 -= uvec2(uint(_597.y == 0u), 1u);
        if (!(_239 < 3))
        {
            break;
        }
        uint _615 = min(0u, 32u);
        uint _620 = min(0u, 32u);
        _237 += int(uint(bitfieldExtract(_239 + 1, int(_615), int(min(24u, (32u - _615))))) * uint(bitfieldExtract(2, int(_620), int(min(24u, (32u - _620))))));
        _239++;
        continue;
    }
    _10._m0[29u] = uint(_237);
    float _636 = clamp(9.75, -2147483648.0, 2147483520.0);
    _10._m0[30u] = uint(int(_636));
    uint _642 = min(0u, 32u);
    uint _647 = min(0u, 32u);
    _10._m0[31u] = uint(int(uint(bitfieldExtract(16777215, int(_642), int(min(24u, (32u - _642))))) * uint(bitfieldExtract(3, int(_647), int(min(24u, (32u - _647)))))));
}

