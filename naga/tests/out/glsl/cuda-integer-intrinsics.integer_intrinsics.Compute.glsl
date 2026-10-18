#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_2_block_0Compute { uint _group_0_binding_0_cs[]; };


int count(uint value) {
    uint value_1 = 0u;
    value_1 = value;
    uint _e2 = value_1;
    return int(uint(bitCount(_e2)));
}

int scan(int value_2) {
    int value_3 = 0;
    value_3 = value_2;
    int _e2 = value_3;
    return (findLSB(_e2) + 1);
}

int _clz(int first, int second) {
    int first_1 = 0;
    int second_1 = 0;
    first_1 = first;
    second_1 = second;
    int _e4 = first_1;
    int _e5 = second_1;
    return (_e4 + _e5);
}

void main() {
    int _e4 = int(true);
    _group_0_binding_0_cs[0] = uint((_e4 < 0 ? 0 : 31 - findMSB(_e4)));
    _group_0_binding_0_cs[1] = uint((8 < 0 ? 0 : 31 - findMSB(8)));
    int _e15 = int(2147483648u);
    _group_0_binding_0_cs[2] = uint((_e15 < 0 ? 0 : 31 - findMSB(_e15)));
    int _e21 = int(8.75);
    _group_0_binding_0_cs[3] = uint((_e21 < 0 ? 0 : 31 - findMSB(_e21)));
    _group_0_binding_0_cs[4] = uint((findLSB(int(true)) + 1));
    _group_0_binding_0_cs[5] = uint((findLSB(8) + 1));
    _group_0_binding_0_cs[6] = uint((findLSB(int(2147483648u)) + 1));
    _group_0_binding_0_cs[7] = uint((findLSB(int(8.75)) + 1));
    _group_0_binding_0_cs[8] = uint(int(uint(bitCount(uint(true)))));
    _group_0_binding_0_cs[9] = uint(int(uint(bitCount(uint(-(1))))));
    _group_0_binding_0_cs[10] = uint(int(uint(bitCount(2147483649u))));
    _group_0_binding_0_cs[11] = uint(int(uint(bitCount(uint(3.75)))));
    _group_0_binding_0_cs[12] = bitfieldReverse(uint(true));
    _group_0_binding_0_cs[13] = bitfieldReverse(uint(-(1)));
    _group_0_binding_0_cs[14] = bitfieldReverse(19088743u);
    _group_0_binding_0_cs[15] = bitfieldReverse(uint(2.75));
    int _e106 = count(2147483649u);
    _group_0_binding_0_cs[16] = uint(_e106);
    int _e112 = scan(int(2147483648u));
    _group_0_binding_0_cs[17] = uint(_e112);
    _group_0_binding_0_cs[18] = uint((0 < 0 ? 0 : 31 - findMSB(0)));
    _group_0_binding_0_cs[19] = uint((findLSB(0) + 1));
    _group_0_binding_0_cs[20] = uint(int(uint(bitCount(4294967295u))));
    _group_0_binding_0_cs[21] = bitfieldReverse(0u);
    _group_0_binding_0_cs[22] = bitfieldReverse(1u);
    _group_0_binding_0_cs[23] = bitfieldReverse(2147483648u);
    int _e147 = -(2);
    _group_0_binding_0_cs[24] = uint((_e147 < 0 ? 0 : 31 - findMSB(_e147)));
    _group_0_binding_0_cs[25] = uint((findLSB(-(2)) + 1));
    _group_0_binding_0_cs[26] = uint(int(uint(bitCount(2863311530u))));
    _group_0_binding_0_cs[27] = bitfieldReverse(2863311530u);
    int _e172 = _clz(3, 4);
    _group_0_binding_0_cs[28] = uint(_e172);
    _group_0_binding_0_cs[29] = uint(((int(uint(bitCount(3u))) - 3) < 0));
}

