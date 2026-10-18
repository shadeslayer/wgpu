#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_1_block_0Compute { int _group_0_binding_0_cs[]; };


int min_(int first_1, int second_1, int third) {
    int first_2 = 0;
    int second_2 = 0;
    int third_1 = 0;
    first_2 = first_1;
    second_2 = second_1;
    third_1 = third;
    int _e6 = first_2;
    int _e7 = second_2;
    int _e9 = third_1;
    return (((_e6 + _e7) + _e9) + 100);
}

int bounded(int value_1) {
    int value_2 = 0;
    value_2 = value_1;
    int _e2 = value_2;
    return min(max(_e2, -(2)), 2);
}

void main() {
    int seed = 0;
    int first = 0;
    int second = 0;
    int value = 0;
    int _e4 = _group_0_binding_0_cs[0];
    seed = _e4;
    int _e7 = seed;
    _group_0_binding_0_cs[0] = min(_e7, -(7));
    int _e13 = seed;
    _group_0_binding_0_cs[1] = max(_e13, -(7));
    _group_0_binding_0_cs[2] = int(min(3u, 9u));
    _group_0_binding_0_cs[3] = int(max(3u, 9u));
    _group_0_binding_0_cs[4] = int((min(uint(-(1)), 2u) == 2u));
    _group_0_binding_0_cs[5] = int((max(uint(-(1)), 2u) == 4294967295u));
    _group_0_binding_0_cs[6] = int((min(2u, uint(-(1))) == 2u));
    _group_0_binding_0_cs[7] = int((max(2u, uint(-(1))) == 4294967295u));
    _group_0_binding_0_cs[8] = min(int(true), int(false));
    _group_0_binding_0_cs[9] = max(int(false), int(true));
    int _e85 = seed;
    _group_0_binding_0_cs[10] = min(max(_e85, -(5)), 2);
    _group_0_binding_0_cs[11] = int((min(uint(-(2)), 4u) == 4u));
    int _e103 = seed;
    int _e106 = min_(_e103, 4, 5);
    _group_0_binding_0_cs[12] = _e106;
    int _e109 = seed;
    int _e110 = bounded(_e109);
    _group_0_binding_0_cs[13] = _e110;
    first = 3;
    second = 7;
    int _e117 = first;
    first = (_e117 + 1);
    int _e120 = second;
    second = (_e120 + 1);
    _group_0_binding_0_cs[14] = max(_e117, _e120);
    int _e126 = first;
    _group_0_binding_0_cs[15] = _e126;
    int _e129 = second;
    _group_0_binding_0_cs[16] = _e129;
    int _e131 = seed;
    value = _e131;
    int _e134 = value;
    int _e135 = value;
    value = (_e135 + 1);
    _group_0_binding_0_cs[17] = max(_e134, _e135);
    int _e141 = value;
    _group_0_binding_0_cs[18] = _e141;
    _group_0_binding_0_cs[19] = int((min((-(2147483647) - 1), 2147483647) == (-(2147483647) - 1)));
    _group_0_binding_0_cs[20] = int((max(4294967295u, 0u) == 4294967295u));
    _group_0_binding_0_cs[21] = int((min(uint(false), 3u) == 0u));
    _group_0_binding_0_cs[22] = int((max(uint(true), 0u) == 1u));
    int _e184 = seed;
    int _e185 = seed;
    _group_0_binding_0_cs[23] = min(_e184, _e185);
}

