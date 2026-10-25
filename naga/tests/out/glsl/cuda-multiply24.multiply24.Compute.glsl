#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_4_block_0Compute { uint _group_0_binding_0_cs[]; };


int signed_conversion(float first_1, float second_1) {
    float first_2 = 0.0;
    float second_2 = 0.0;
    first_2 = first_1;
    second_2 = second_1;
    float _e4 = first_2;
    float _e5 = second_2;
    return int((uint(bitfieldExtract(int(_e4), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(int(_e5), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))))));
}

uint unsigned_conversion(float first_3, float second_3) {
    float first_4 = 0.0;
    float second_4 = 0.0;
    first_4 = first_3;
    second_4 = second_3;
    float _e4 = first_4;
    float _e5 = second_4;
    return (bitfieldExtract(uint(_e4), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))) * bitfieldExtract(uint(_e5), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))));
}

int _mul24_(bool first_5, bool second_5) {
    bool first_6 = false;
    bool second_6 = false;
    int local_2 = 0;
    int local_3 = 0;
    first_6 = first_5;
    second_6 = second_5;
    bool _e4 = first_6;
    if (_e4) {
        bool _e5 = second_6;
        if (_e5) {
            local_2 = 71;
        } else {
            local_2 = 73;
        }
        int _e9 = local_2;
        local_3 = _e9;
    } else {
        local_3 = 79;
    }
    int _e12 = local_3;
    return _e12;
}

int _mul24_1(float first_7, float second_7) {
    float first_8 = 0.0;
    float second_8 = 0.0;
    first_8 = first_7;
    second_8 = second_7;
    float _e5 = first_8;
    float _e7 = second_8;
    return int(((_e5 * 10.0) + _e7));
}

uint _umul24_(int first_9, int second_9) {
    int first_10 = 0;
    int second_10 = 0;
    first_10 = first_9;
    second_10 = second_9;
    int _e4 = first_10;
    int _e5 = second_10;
    return uint(((_e4 + _e5) + 100));
}

int signed_pair(int first_11, int second_11) {
    int first_12 = 0;
    int second_12 = 0;
    first_12 = first_11;
    second_12 = second_11;
    int _e4 = first_12;
    int _e5 = second_12;
    return int((uint(bitfieldExtract(_e4, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(_e5, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))))));
}

void main() {
    int first = 0;
    uint second = 0u;
    int local = 0;
    int values[2] = int[2](0, 0);
    int Count = 0;
    int local_1 = 0;
    int scratch[2] = int[2](0, 0);
    int counter = 0;
    int total = 0;
    int index = 0;
    _group_0_binding_0_cs[0] = uint(int((uint(bitfieldExtract(8388608, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(1, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))));
    _group_0_binding_0_cs[1] = (bitfieldExtract(16777215u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))) * bitfieldExtract(16777215u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))));
    _group_0_binding_0_cs[2] = uint(int((uint(bitfieldExtract(8388607, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(8388607, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))));
    _group_0_binding_0_cs[3] = (bitfieldExtract(16777216u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))) * bitfieldExtract(13u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))));
    int _e50 = signed_conversion(3.75, -(2.75));
    _group_0_binding_0_cs[4] = uint(_e50);
    int _e57 = signed_conversion(-(3.75), 2.75);
    _group_0_binding_0_cs[5] = uint(_e57);
    uint _e63 = unsigned_conversion(3.75, 2.75);
    _group_0_binding_0_cs[6] = _e63;
    uint _e68 = unsigned_conversion(16777215.0, 1.0);
    _group_0_binding_0_cs[7] = _e68;
    int _e73 = _mul24_(true, true);
    _group_0_binding_0_cs[8] = uint(_e73);
    int _e79 = _mul24_(false, true);
    _group_0_binding_0_cs[9] = uint(_e79);
    _group_0_binding_0_cs[10] = uint(int((uint(bitfieldExtract(3, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(int(true), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))));
    _group_0_binding_0_cs[11] = uint(int((uint(bitfieldExtract(int(true), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(3, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))));
    int _e113 = _mul24_1(3.75, 2.0);
    _group_0_binding_0_cs[12] = uint(_e113);
    uint _e119 = _umul24_(3, 4);
    _group_0_binding_0_cs[13] = _e119;
    _group_0_binding_0_cs[14] = (bitfieldExtract(3u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))) * bitfieldExtract(4u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))));
    uint _e135 = _umul24_(int(true), int(true));
    _group_0_binding_0_cs[15] = _e135;
    _group_0_binding_0_cs[16] = (bitfieldExtract(3u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))) * bitfieldExtract(uint(4), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))));
    int _e151 = signed_pair(-(7), 3);
    _group_0_binding_0_cs[17] = uint(_e151);
    _group_0_binding_0_cs[18] = uint(int((uint(bitfieldExtract(int((uint(bitfieldExtract(3, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(4, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(5, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))));
    _group_0_binding_0_cs[19] = (bitfieldExtract((bitfieldExtract(16777215u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))) * bitfieldExtract(2u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))) * bitfieldExtract(2u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))));
    first = -3;
    second = 5u;
    int _e196 = first;
    uint _e197 = second;
    _group_0_binding_0_cs[20] = uint(int((uint(bitfieldExtract(_e196, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(int(_e197), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))));
    int _e210 = first;
    uint _e212 = second;
    _group_0_binding_0_cs[21] = (bitfieldExtract(uint(_e210), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))) * bitfieldExtract(_e212, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))));
    _group_0_binding_0_cs[22] = uint(int((uint(bitfieldExtract(25165824, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(16777217, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))));
    _group_0_binding_0_cs[23] = (bitfieldExtract(4286578688u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))) * bitfieldExtract(4278190081u, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))));
    if (true) {
        local = int((uint(bitfieldExtract(-(3), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(7, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))))));
    } else {
        local = int((uint(bitfieldExtract(0, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(0, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))))));
    }
    int _e265 = local;
    _group_0_binding_0_cs[24] = uint(_e265);
    values[0] = int((uint(bitfieldExtract(2, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(3, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))))));
    values[1] = int((uint(bitfieldExtract(3, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(4, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u)))))));
    int _e296 = values[0];
    int _e297 = values[1];
    _group_0_binding_0_cs[25] = uint((_e296 + _e297));
    Count = 2;
    scratch = int[2](0, 0);
    int _e308 = Count;
    int _e309 = scratch[0];
    _group_0_binding_0_cs[26] = uint((_e308 + _e309));
    counter = 2;
    int _e316 = counter;
    counter = (_e316 + 1);
    int _e319 = counter;
    counter = (_e319 + 1);
    _group_0_binding_0_cs[27] = uint(int((uint(bitfieldExtract(_e316, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(_e319, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))));
    int _e333 = counter;
    _group_0_binding_0_cs[28] = uint(_e333);
    total = 0;
    {
        index = 0;
        bool loop_init = true;
        while(true) {
            if (!loop_init) {
                int _e342 = index;
                index = (_e342 + 1);
            }
            loop_init = false;
            int _e340 = index;
            if ((_e340 < 3)) {
            } else {
                break;
            }
            int _e346 = index;
            int _e357 = total;
            total = (_e357 + int((uint(bitfieldExtract((_e346 + 1), int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(2, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))));
        }
    }
    int _e361 = total;
    _group_0_binding_0_cs[29] = uint(_e361);
    {
        _group_0_binding_0_cs[30] = uint(int(9.75));
    }
    _group_0_binding_0_cs[31] = uint(int((uint(bitfieldExtract(16777215, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))) * uint(bitfieldExtract(3, int(min(0u, 32u)), int(min(24u, 32u - min(0u, 32u))))))));
}

