#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0, std430) buffer _7_6
{
    uint _m0[];
} _6;

int _11(uint _10)
{
    uint _13 = 0u;
    _13 = _10;
    return int(uint(bitCount(_13)));
}

int _22(int _21)
{
    int _25 = 0;
    _25 = _21;
    return findLSB(_25) + 1;
}

int _35(int _33, int _34)
{
    int _37 = 0;
    int _39 = 0;
    _37 = _33;
    _39 = _34;
    return _37 + _39;
}

void main()
{
    _6._m0[0u] = uint(31 - findMSB(uint(int(true))));
    int _102 = findMSB(uint(8));
    _6._m0[1u] = uint(31 - _102);
    _6._m0[2u] = uint(31 - findMSB(uint(int(2147483648u))));
    float _113 = clamp(8.75, -2147483648.0, 2147483520.0);
    _6._m0[3u] = uint(31 - findMSB(uint(int(_113))));
    _6._m0[4u] = uint(findLSB(int(true)) + 1);
    int _125 = findLSB(8);
    _6._m0[5u] = uint(_125 + 1);
    _6._m0[6u] = uint(findLSB(int(2147483648u)) + 1);
    float _136 = clamp(8.75, -2147483648.0, 2147483520.0);
    _6._m0[7u] = uint(findLSB(int(_136)) + 1);
    _6._m0[8u] = uint(int(uint(bitCount(uint(true)))));
    int _149 = -1;
    _6._m0[9u] = uint(int(uint(bitCount(uint(_149)))));
    uint _156 = uint(bitCount(2147483649u));
    _6._m0[10u] = uint(int(_156));
    float _163 = clamp(3.75, 0.0, 4294967040.0);
    _6._m0[11u] = uint(int(uint(bitCount(uint(_163)))));
    _6._m0[12u] = bitfieldReverse(uint(true));
    int _174 = -1;
    _6._m0[13u] = bitfieldReverse(uint(_174));
    uint _179 = bitfieldReverse(19088743u);
    _6._m0[14u] = _179;
    float _182 = clamp(2.75, 0.0, 4294967040.0);
    _6._m0[15u] = bitfieldReverse(uint(_182));
    _6._m0[16u] = uint(_11(2147483649u));
    _6._m0[17u] = uint(_22(int(2147483648u)));
    int _197 = findMSB(uint(0));
    _6._m0[18u] = uint(31 - _197);
    int _201 = findLSB(0);
    _6._m0[19u] = uint(_201 + 1);
    uint _206 = uint(bitCount(4294967295u));
    _6._m0[20u] = uint(int(_206));
    uint _211 = bitfieldReverse(0u);
    _6._m0[21u] = _211;
    uint _214 = bitfieldReverse(1u);
    _6._m0[22u] = _214;
    uint _217 = bitfieldReverse(2147483648u);
    _6._m0[23u] = _217;
    int _220 = -2;
    _6._m0[24u] = uint(31 - findMSB(uint(_220)));
    int _226 = -2;
    _6._m0[25u] = uint(findLSB(_226) + 1);
    uint _232 = uint(bitCount(2863311530u));
    _6._m0[26u] = uint(int(_232));
    uint _237 = bitfieldReverse(2863311530u);
    _6._m0[27u] = _237;
    _6._m0[28u] = uint(_35(3, 4));
    uint _244 = uint(bitCount(3u));
    _6._m0[29u] = uint((int(_244) - 3) < 0);
}

