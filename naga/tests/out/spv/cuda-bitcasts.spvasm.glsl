#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0, std430) buffer _8_7
{
    uint _m0[];
} _7;

int _26(int _24, int _25)
{
    int _28 = 0;
    int _31 = 0;
    _28 = _24;
    _31 = _25;
    return _28 + _31;
}

float _12(uint _11)
{
    uint _14 = 0u;
    uint _17 = 0u;
    _14 = _11;
    _17 = _14;
    return uintBitsToFloat(_17);
}

void main()
{
    uint _118 = 0u;
    float _112 = 0.0;
    int _106 = 0;
    uint _99 = 0u;
    uint _93 = 0u;
    int _87 = 0;
    uint _116 = 0u;
    uint _110 = 0u;
    int _104 = 0;
    uint _97 = 0u;
    int _91 = 0;
    int _85 = 0;
    int _114 = 0;
    uint _108 = 0u;
    float _101 = 0.0;
    uint _95 = 0u;
    int _89 = 0;
    _7._m0[0u] = uint(floatBitsToInt(float(true)));
    float _128 = float(3);
    _7._m0[1u] = uint(floatBitsToInt(_128));
    float _132 = float(5u);
    _7._m0[2u] = uint(floatBitsToInt(_132));
    _7._m0[3u] = uint(floatBitsToInt(2.75));
    _7._m0[4u] = floatBitsToUint(float(true));
    float _145 = float(3);
    _7._m0[5u] = floatBitsToUint(_145);
    float _148 = float(5u);
    _7._m0[6u] = floatBitsToUint(_148);
    _7._m0[7u] = floatBitsToUint(2.75);
    _85 = int(true);
    _7._m0[8u] = floatBitsToUint(intBitsToFloat(_85));
    _87 = 3;
    _7._m0[9u] = floatBitsToUint(intBitsToFloat(_87));
    _89 = int(5u);
    _7._m0[10u] = floatBitsToUint(intBitsToFloat(_89));
    float _174 = clamp(2.75, -2147483648.0, 2147483520.0);
    _91 = int(_174);
    _7._m0[11u] = floatBitsToUint(intBitsToFloat(_91));
    _93 = uint(true);
    _7._m0[12u] = floatBitsToUint(uintBitsToFloat(_93));
    _95 = uint(3);
    _7._m0[13u] = floatBitsToUint(uintBitsToFloat(_95));
    _97 = 5u;
    _7._m0[14u] = floatBitsToUint(uintBitsToFloat(_97));
    float _199 = clamp(2.75, 0.0, 4294967040.0);
    _99 = uint(_199);
    _7._m0[15u] = floatBitsToUint(uintBitsToFloat(_99));
    _101 = 1.5;
    float _206 = _101;
    float _207 = float(1);
    _101 = _206 + _207;
    _7._m0[16u] = uint(floatBitsToInt(_206));
    _7._m0[17u] = floatBitsToUint(_101);
    float _217 = _101;
    float _218 = float(1);
    _101 = _217 + _218;
    _7._m0[18u] = floatBitsToUint(_217);
    _7._m0[19u] = floatBitsToUint(_101);
    _104 = 1065353216;
    int _227 = _104;
    _104 = _227 + 1;
    _106 = _227;
    _7._m0[20u] = floatBitsToUint(intBitsToFloat(_106));
    _7._m0[21u] = uint(_104);
    _108 = 1073741824u;
    uint _238 = _108;
    _108 = _238 + uint(1);
    _110 = _238;
    _7._m0[22u] = floatBitsToUint(uintBitsToFloat(_110));
    _7._m0[23u] = _108;
    _7._m0[24u] = uint(_26(3, 4));
    _112 = _12(3212836864u);
    _7._m0[25u] = floatBitsToUint(_112);
    _114 = 2139095040;
    _7._m0[26u] = floatBitsToUint(intBitsToFloat(_114));
    _116 = 2143363909u;
    _7._m0[27u] = floatBitsToUint(uintBitsToFloat(_116));
    float _268 = clamp(1065353216.0, 0.0, 4294967040.0);
    _118 = uint(_268);
    _7._m0[28u] = floatBitsToUint(uintBitsToFloat(_118));
    float _275 = float(1u);
    _7._m0[29u] = uint(floatBitsToInt(_275));
}

