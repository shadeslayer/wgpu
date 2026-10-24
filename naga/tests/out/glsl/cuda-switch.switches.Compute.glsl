#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_3_block_0Compute { int _group_0_binding_0_cs[]; };


int case_chain(int selector) {
    int selector_1 = 0;
    int result = 0;
    selector_1 = selector;
    result = 0;
    int _e4 = selector_1;
    switch(_e4) {
        case -1: {
            int _e6 = result;
            result = (_e6 + 1);
        }
        case 0: {
            int _e9 = result;
            result = (_e9 + 10);
        }
        case 1: {
            int _e12 = result;
            result = (_e12 + 100);
            break;
        }
        default: {
            int _e15 = result;
            result = (_e15 + 1000);
        }
        case 2: {
            int _e18 = result;
            result = (_e18 + 10000);
            break;
        }
    }
    int _e20 = result;
    return _e20;
}

int grouped_labels(uint selector_2) {
    uint selector_3 = 0u;
    int result_1 = 0;
    selector_3 = selector_2;
    result_1 = 0;
    uint _e4 = selector_3;
    switch(_e4) {
        case 0u:
        case 1u: {
            int _e6 = result_1;
            result_1 = (_e6 + 3);
        }
        case 2u: {
            int _e9 = result_1;
            result_1 = (_e9 + 5);
            break;
        }
        default: {
            result_1 = 7;
            break;
        }
    }
    int _e12 = result_1;
    return _e12;
}

int nested_switches(int outer, int inner) {
    int outer_1 = 0;
    int inner_1 = 0;
    int result_2 = 0;
    outer_1 = outer;
    inner_1 = inner;
    result_2 = 0;
    int _e6 = outer_1;
    switch(_e6) {
        case 0: {
            int _e8 = result_2;
            result_2 = (_e8 + 1);
            int _e10 = inner_1;
            switch(_e10) {
                case 0: {
                    int _e12 = result_2;
                    result_2 = (_e12 + 10);
                }
                default: {
                    int _e15 = result_2;
                    result_2 = (_e15 + 20);
                    break;
                }
            }
        }
        case 1: {
            int _e18 = result_2;
            result_2 = (_e18 + 100);
            break;
        }
        default: {
            result_2 = 900;
            break;
        }
    }
    int _e21 = result_2;
    return _e21;
}

int loop_jumps(int selector_4) {
    int selector_5 = 0;
    int result_3 = 0;
    int iteration = 0;
    selector_5 = selector_4;
    result_3 = 0;
    {
        iteration = 0;
        bool loop_init = true;
        while(true) {
            if (!loop_init) {
                int _e9 = iteration;
                iteration = (_e9 + 1);
            }
            loop_init = false;
            int _e7 = iteration;
            if ((_e7 < 4)) {
            } else {
                break;
            }
            int _e12 = selector_5;
            switch(_e12) {
                case 0: {
                    int _e14 = iteration;
                    if ((_e14 == 1)) {
                        continue;
                    }
                    int _e17 = iteration;
                    int _e19 = result_3;
                    result_3 = (_e19 + (_e17 + 1));
                }
                case 1: {
                    int _e22 = iteration;
                    if ((_e22 == 2)) {
                        break;
                    }
                    int _e25 = result_3;
                    result_3 = (_e25 + 10);
                    break;
                }
                default: {
                    int _e28 = result_3;
                    result_3 = (_e28 + 100);
                    continue;
                }
            }
            int _e31 = result_3;
            result_3 = (_e31 + 1000);
        }
    }
    int _e33 = result_3;
    return _e33;
}

int terminal_paths(int selector_6, int flag) {
    int selector_7 = 0;
    int flag_1 = 0;
    int result_4 = 0;
    selector_7 = selector_6;
    flag_1 = flag;
    result_4 = 0;
    int _e6 = selector_7;
    switch(_e6) {
        case 0: {
            int _e8 = flag_1;
            if ((_e8 != 0)) {
                return 3;
            }
            int _e12 = result_4;
            result_4 = (_e12 + 1);
        }
        case 1: {
            int _e15 = result_4;
            return (_e15 + 5);
        }
        default: {
            result_4 = 9;
        }
        case 2: {
            {
                int _e19 = result_4;
                return (_e19 + 10);
            }
            break;
        }
    }
}

int update_value(int value, int token) {
    int value_1 = 0;
    int token_1 = 0;
    value_1 = value;
    token_1 = token;
    int _e4 = value_1;
    int _e5 = token_1;
    return (_e4 + _e5);
}

ivec2 call_results(int selector_8, int counter_1) {
    int selector_9 = 0;
    int counter_2 = 0;
    int result_5 = 0;
    int named_value = 0;
    int named_value_1 = 0;
    int named_value_2 = 0;
    selector_9 = selector_8;
    counter_2 = counter_1;
    result_5 = 2;
    int _e6 = selector_9;
    switch(_e6) {
        case 0: {
            {
                int _e8 = result_5;
                int _e9 = counter_2;
                counter_2 = (_e9 + 1);
                int _e12 = counter_2;
                int _e13 = update_value(_e8, _e12);
                named_value = _e13;
                int _e14 = named_value;
                int _e15 = named_value;
                result_5 = (_e14 + _e15);
            }
        }
        case 1: {
            {
                int _e18 = result_5;
                int _e19 = counter_2;
                counter_2 = (_e19 + 1);
                int _e22 = counter_2;
                int _e23 = update_value(_e18, _e22);
                named_value_1 = _e23;
                int _e24 = named_value_1;
                int _e25 = named_value_1;
                result_5 = (_e24 * _e25);
            }
        }
        default: {
            {
                int _e28 = result_5;
                int _e29 = counter_2;
                counter_2 = (_e29 + 1);
                int _e32 = counter_2;
                int _e33 = update_value(_e28, _e32);
                named_value_2 = _e33;
                int _e34 = named_value_2;
                int _e35 = named_value_2;
                result_5 = (_e34 + _e35);
                break;
            }
            break;
        }
    }
    int _e37 = result_5;
    int _e38 = counter_2;
    return ivec2(_e37, _e38);
}

void main() {
    int counter = 0;
    ivec2 first = ivec2(0);
    ivec2 second = ivec2(0);
    ivec2 third = ivec2(0);
    int _e5 = case_chain(-(1));
    _group_0_binding_0_cs[0] = _e5;
    int _e9 = case_chain(0);
    _group_0_binding_0_cs[1] = _e9;
    int _e13 = case_chain(1);
    _group_0_binding_0_cs[2] = _e13;
    int _e17 = case_chain(2);
    _group_0_binding_0_cs[3] = _e17;
    int _e21 = case_chain(8);
    _group_0_binding_0_cs[4] = _e21;
    int _e25 = grouped_labels(0u);
    _group_0_binding_0_cs[5] = _e25;
    int _e29 = grouped_labels(1u);
    _group_0_binding_0_cs[6] = _e29;
    int _e33 = grouped_labels(2u);
    _group_0_binding_0_cs[7] = _e33;
    int _e37 = grouped_labels(8u);
    _group_0_binding_0_cs[8] = _e37;
    int _e42 = nested_switches(0, 0);
    _group_0_binding_0_cs[9] = _e42;
    int _e47 = nested_switches(0, 8);
    _group_0_binding_0_cs[10] = _e47;
    int _e52 = nested_switches(1, 0);
    _group_0_binding_0_cs[11] = _e52;
    int _e57 = nested_switches(8, 0);
    _group_0_binding_0_cs[12] = _e57;
    int _e61 = loop_jumps(0);
    _group_0_binding_0_cs[13] = _e61;
    int _e65 = loop_jumps(1);
    _group_0_binding_0_cs[14] = _e65;
    int _e69 = loop_jumps(8);
    _group_0_binding_0_cs[15] = _e69;
    int _e74 = terminal_paths(0, 1);
    _group_0_binding_0_cs[16] = _e74;
    int _e79 = terminal_paths(0, 0);
    _group_0_binding_0_cs[17] = _e79;
    int _e84 = terminal_paths(1, 1);
    _group_0_binding_0_cs[18] = _e84;
    int _e89 = terminal_paths(8, 0);
    _group_0_binding_0_cs[19] = _e89;
    int _e94 = terminal_paths(2, 0);
    _group_0_binding_0_cs[20] = _e94;
    counter = 0;
    int _e97 = counter;
    counter = (_e97 + 1);
    switch(_e97) {
        case 0: {
            int _e102 = counter;
            _group_0_binding_0_cs[21] = _e102;
        }
        case 1: {
            int _e106 = counter;
            _group_0_binding_0_cs[22] = (_e106 + 1);
            break;
        }
        default: {
            _group_0_binding_0_cs[21] = 99;
            break;
        }
    }
    int _e113 = counter;
    _group_0_binding_0_cs[23] = _e113;
    ivec2 _e117 = call_results(0, 0);
    first = _e117;
    int _e121 = first.y;
    ivec2 _e122 = call_results(1, _e121);
    second = _e122;
    int _e126 = second.y;
    ivec2 _e127 = call_results(8, _e126);
    third = _e127;
    int _e131 = first.x;
    _group_0_binding_0_cs[24] = _e131;
    int _e135 = second.x;
    _group_0_binding_0_cs[25] = _e135;
    int _e139 = third.x;
    _group_0_binding_0_cs[26] = _e139;
    int _e143 = third.y;
    _group_0_binding_0_cs[27] = _e143;
    int _e145 = third.y;
    counter = _e145;
    int _e149 = third.x;
    int _e150 = counter;
    counter = (_e150 + 1);
    int _e153 = counter;
    int _e154 = update_value(_e149, _e153);
    _group_0_binding_0_cs[28] = _e154;
    int _e157 = counter;
    _group_0_binding_0_cs[29] = _e157;
}

