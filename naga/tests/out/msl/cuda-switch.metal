// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef int type_3[1];

int case_chain(
    int selector
) {
    int selector_1 = {};
    int result = {};
    selector_1 = selector;
    result = 0;
    int _e4 = selector_1;
    switch(_e4) {
        case -1: {
            int _e6 = result;
            result = as_type<int>(as_type<uint>(_e6) + as_type<uint>(1));
        }
        case 0: {
            int _e9 = result;
            result = as_type<int>(as_type<uint>(_e9) + as_type<uint>(10));
        }
        case 1: {
            int _e12 = result;
            result = as_type<int>(as_type<uint>(_e12) + as_type<uint>(100));
            break;
        }
        default: {
            int _e15 = result;
            result = as_type<int>(as_type<uint>(_e15) + as_type<uint>(1000));
        }
        case 2: {
            int _e18 = result;
            result = as_type<int>(as_type<uint>(_e18) + as_type<uint>(10000));
            break;
        }
    }
    int _e20 = result;
    return _e20;
}

int grouped_labels(
    uint selector_2
) {
    uint selector_3 = {};
    int result_1 = {};
    selector_3 = selector_2;
    result_1 = 0;
    uint _e4 = selector_3;
    switch(_e4) {
        case 0u:
        case 1u: {
            int _e6 = result_1;
            result_1 = as_type<int>(as_type<uint>(_e6) + as_type<uint>(3));
        }
        case 2u: {
            int _e9 = result_1;
            result_1 = as_type<int>(as_type<uint>(_e9) + as_type<uint>(5));
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

int nested_switches(
    int outer,
    int inner
) {
    int outer_1 = {};
    int inner_1 = {};
    int result_2 = {};
    outer_1 = outer;
    inner_1 = inner;
    result_2 = 0;
    int _e6 = outer_1;
    switch(_e6) {
        case 0: {
            int _e8 = result_2;
            result_2 = as_type<int>(as_type<uint>(_e8) + as_type<uint>(1));
            int _e10 = inner_1;
            switch(_e10) {
                case 0: {
                    int _e12 = result_2;
                    result_2 = as_type<int>(as_type<uint>(_e12) + as_type<uint>(10));
                }
                default: {
                    int _e15 = result_2;
                    result_2 = as_type<int>(as_type<uint>(_e15) + as_type<uint>(20));
                    break;
                }
            }
        }
        case 1: {
            int _e18 = result_2;
            result_2 = as_type<int>(as_type<uint>(_e18) + as_type<uint>(100));
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

int loop_jumps(
    int selector_4
) {
    int selector_5 = {};
    int result_3 = {};
    int iteration = {};
    selector_5 = selector_4;
    result_3 = 0;
    {
        iteration = 0;
        uint2 loop_bound = uint2(4294967295u);
        bool loop_init = true;
        while(true) {
            if (metal::all(loop_bound == uint2(0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e9 = iteration;
                iteration = as_type<int>(as_type<uint>(_e9) + as_type<uint>(1));
            }
            loop_init = false;
            int _e7 = iteration;
            if (_e7 < 4) {
            } else {
                break;
            }
            int _e12 = selector_5;
            switch(_e12) {
                case 0: {
                    int _e14 = iteration;
                    if (_e14 == 1) {
                        continue;
                    }
                    int _e17 = iteration;
                    int _e19 = result_3;
                    result_3 = as_type<int>(as_type<uint>(_e19) + as_type<uint>(as_type<int>(as_type<uint>(_e17) + as_type<uint>(1))));
                }
                case 1: {
                    int _e22 = iteration;
                    if (_e22 == 2) {
                        break;
                    }
                    int _e25 = result_3;
                    result_3 = as_type<int>(as_type<uint>(_e25) + as_type<uint>(10));
                    break;
                }
                default: {
                    int _e28 = result_3;
                    result_3 = as_type<int>(as_type<uint>(_e28) + as_type<uint>(100));
                    continue;
                }
            }
            int _e31 = result_3;
            result_3 = as_type<int>(as_type<uint>(_e31) + as_type<uint>(1000));
        }
    }
    int _e33 = result_3;
    return _e33;
}

int terminal_paths(
    int selector_6,
    int flag
) {
    int selector_7 = {};
    int flag_1 = {};
    int result_4 = {};
    selector_7 = selector_6;
    flag_1 = flag;
    result_4 = 0;
    int _e6 = selector_7;
    switch(_e6) {
        case 0: {
            int _e8 = flag_1;
            if (_e8 != 0) {
                return 3;
            }
            int _e12 = result_4;
            result_4 = as_type<int>(as_type<uint>(_e12) + as_type<uint>(1));
        }
        case 1: {
            int _e15 = result_4;
            return as_type<int>(as_type<uint>(_e15) + as_type<uint>(5));
        }
        default: {
            result_4 = 9;
        }
        case 2: {
            {
                int _e19 = result_4;
                return as_type<int>(as_type<uint>(_e19) + as_type<uint>(10));
            }
            break;
        }
    }
}

int update_value(
    int value,
    int token
) {
    int value_1 = {};
    int token_1 = {};
    value_1 = value;
    token_1 = token;
    int _e4 = value_1;
    int _e5 = token_1;
    return as_type<int>(as_type<uint>(_e4) + as_type<uint>(_e5));
}

metal::int2 call_results(
    int selector_8,
    int counter_1
) {
    int selector_9 = {};
    int counter_2 = {};
    int result_5 = {};
    int named_value = {};
    int named_value_1 = {};
    int named_value_2 = {};
    selector_9 = selector_8;
    counter_2 = counter_1;
    result_5 = 2;
    int _e6 = selector_9;
    switch(_e6) {
        case 0: {
            {
                int _e8 = result_5;
                int _e9 = counter_2;
                counter_2 = as_type<int>(as_type<uint>(_e9) + as_type<uint>(1));
                int _e12 = counter_2;
                int _e13 = update_value(_e8, _e12);
                named_value = _e13;
                int _e14 = named_value;
                int _e15 = named_value;
                result_5 = as_type<int>(as_type<uint>(_e14) + as_type<uint>(_e15));
            }
        }
        case 1: {
            {
                int _e18 = result_5;
                int _e19 = counter_2;
                counter_2 = as_type<int>(as_type<uint>(_e19) + as_type<uint>(1));
                int _e22 = counter_2;
                int _e23 = update_value(_e18, _e22);
                named_value_1 = _e23;
                int _e24 = named_value_1;
                int _e25 = named_value_1;
                result_5 = as_type<int>(as_type<uint>(_e24) * as_type<uint>(_e25));
            }
        }
        default: {
            {
                int _e28 = result_5;
                int _e29 = counter_2;
                counter_2 = as_type<int>(as_type<uint>(_e29) + as_type<uint>(1));
                int _e32 = counter_2;
                int _e33 = update_value(_e28, _e32);
                named_value_2 = _e33;
                int _e34 = named_value_2;
                int _e35 = named_value_2;
                result_5 = as_type<int>(as_type<uint>(_e34) + as_type<uint>(_e35));
                break;
            }
            break;
        }
    }
    int _e37 = result_5;
    int _e38 = counter_2;
    return metal::int2(_e37, _e38);
}
int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}


kernel void switches(
  device type_3& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int counter = {};
    metal::int2 first = {};
    metal::int2 second = {};
    metal::int2 third = {};
    int _e5 = case_chain(naga_neg(1));
    out[0] = _e5;
    int _e9 = case_chain(0);
    out[1] = _e9;
    int _e13 = case_chain(1);
    out[2] = _e13;
    int _e17 = case_chain(2);
    out[3] = _e17;
    int _e21 = case_chain(8);
    out[4] = _e21;
    int _e25 = grouped_labels(0u);
    out[5] = _e25;
    int _e29 = grouped_labels(1u);
    out[6] = _e29;
    int _e33 = grouped_labels(2u);
    out[7] = _e33;
    int _e37 = grouped_labels(8u);
    out[8] = _e37;
    int _e42 = nested_switches(0, 0);
    out[9] = _e42;
    int _e47 = nested_switches(0, 8);
    out[10] = _e47;
    int _e52 = nested_switches(1, 0);
    out[11] = _e52;
    int _e57 = nested_switches(8, 0);
    out[12] = _e57;
    int _e61 = loop_jumps(0);
    out[13] = _e61;
    int _e65 = loop_jumps(1);
    out[14] = _e65;
    int _e69 = loop_jumps(8);
    out[15] = _e69;
    int _e74 = terminal_paths(0, 1);
    out[16] = _e74;
    int _e79 = terminal_paths(0, 0);
    out[17] = _e79;
    int _e84 = terminal_paths(1, 1);
    out[18] = _e84;
    int _e89 = terminal_paths(8, 0);
    out[19] = _e89;
    int _e94 = terminal_paths(2, 0);
    out[20] = _e94;
    counter = 0;
    int _e97 = counter;
    counter = as_type<int>(as_type<uint>(_e97) + as_type<uint>(1));
    switch(_e97) {
        case 0: {
            int _e102 = counter;
            out[21] = _e102;
        }
        case 1: {
            int _e106 = counter;
            out[22] = as_type<int>(as_type<uint>(_e106) + as_type<uint>(1));
            break;
        }
        default: {
            out[21] = 99;
            break;
        }
    }
    int _e113 = counter;
    out[23] = _e113;
    metal::int2 _e117 = call_results(0, 0);
    first = _e117;
    int _e121 = first.y;
    metal::int2 _e122 = call_results(1, _e121);
    second = _e122;
    int _e126 = second.y;
    metal::int2 _e127 = call_results(8, _e126);
    third = _e127;
    int _e131 = first.x;
    out[24] = _e131;
    int _e135 = second.x;
    out[25] = _e135;
    int _e139 = third.x;
    out[26] = _e139;
    int _e143 = third.y;
    out[27] = _e143;
    int _e145 = third.y;
    counter = _e145;
    int _e149 = third.x;
    int _e150 = counter;
    counter = as_type<int>(as_type<uint>(_e150) + as_type<uint>(1));
    int _e153 = counter;
    int _e154 = update_value(_e149, _e153);
    out[28] = _e154;
    int _e157 = counter;
    out[29] = _e157;
}
