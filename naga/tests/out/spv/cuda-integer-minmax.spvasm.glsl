#version 460
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0, std430) buffer _6_5
{
    int _m0[];
} _5;

int _12(int _9, int _10, int _11)
{
    int _15 = 0;
    int _18 = 0;
    int _20 = 0;
    _15 = _9;
    _18 = _10;
    _20 = _11;
    return ((_15 + _18) + _20) + 100;
}

int _31(int _30)
{
    int _34 = 0;
    _34 = _30;
    int _38 = -2;
    return min(max(_34, _38), 2);
}

void main()
{
    int _83 = 0;
    int _87 = 0;
    int _81 = 0;
    int _85 = 0;
    _81 = _5._m0[0u];
    int _94 = -7;
    _5._m0[0u] = min(_81, _94);
    int _98 = -7;
    _5._m0[1u] = max(_81, _98);
    uint _101 = min(3u, 9u);
    _5._m0[2u] = int(_101);
    uint _104 = max(3u, 9u);
    _5._m0[3u] = int(_104);
    int _107 = -1;
    _5._m0[4u] = int(min(uint(_107), 2u) == 2u);
    int _113 = -1;
    _5._m0[5u] = int(max(uint(_113), 2u) == 4294967295u);
    int _120 = -1;
    _5._m0[6u] = int(min(2u, uint(_120)) == 2u);
    int _127 = -1;
    _5._m0[7u] = int(max(2u, uint(_127)) == 4294967295u);
    _5._m0[8u] = min(int(true), int(false));
    _5._m0[9u] = max(int(false), int(true));
    int _144 = -5;
    _5._m0[10u] = min(max(_81, _144), 2);
    int _149 = -2;
    _5._m0[11u] = int(min(uint(_149), 4u) == 4u);
    _5._m0[12u] = _12(_81, 4, 5);
    _5._m0[13u] = _31(_81);
    _83 = 3;
    _85 = 7;
    int _164 = _83;
    _83 = _164 + 1;
    int _166 = _85;
    _85 = _166 + 1;
    _5._m0[14u] = max(_164, _166);
    _5._m0[15u] = _83;
    _5._m0[16u] = _85;
    _87 = _81;
    int _178 = _87;
    int _179 = _87;
    _87 = _179 + 1;
    _5._m0[17u] = max(_178, _179);
    _5._m0[18u] = _87;
    int _187 = -2147483647;
    int _190 = -2147483647;
    _5._m0[19u] = int(min((_187 - 1), 2147483647) == (_190 - 1));
    uint _196 = max(4294967295u, 0u);
    _5._m0[20u] = int(_196 == 4294967295u);
    _5._m0[21u] = int(min(uint(false), 3u) == 0u);
    _5._m0[22u] = int(max(uint(true), 0u) == 1u);
    _5._m0[23u] = min(_81, _81);
}

