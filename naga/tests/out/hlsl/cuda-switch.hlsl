RWByteAddressBuffer out_ : register(u0);

int case_chain(int selector)
{
    int selector_1 = (int)0;
    int result = (int)0;

    selector_1 = selector;
    result = int(0);
    int _e4 = selector_1;
    switch(_e4) {
        case -1: {
            {
                int _e6 = result;
                result = asint(asuint(_e6) + asuint(int(1)));
            }
            {
                int _e9 = result;
                result = asint(asuint(_e9) + asuint(int(10)));
            }
            {
                int _e12 = result;
                result = asint(asuint(_e12) + asuint(int(100)));
                break;
            }
        }
        case 0: {
            {
                int _e9 = result;
                result = asint(asuint(_e9) + asuint(int(10)));
            }
            {
                int _e12 = result;
                result = asint(asuint(_e12) + asuint(int(100)));
                break;
            }
        }
        case 1: {
            int _e12 = result;
            result = asint(asuint(_e12) + asuint(int(100)));
            break;
        }
        default: {
            {
                int _e15 = result;
                result = asint(asuint(_e15) + asuint(int(1000)));
            }
            {
                int _e18 = result;
                result = asint(asuint(_e18) + asuint(int(10000)));
                break;
            }
        }
        case 2: {
            int _e18 = result;
            result = asint(asuint(_e18) + asuint(int(10000)));
            break;
        }
    }
    int _e20 = result;
    return _e20;
}

int grouped_labels(uint selector_2)
{
    uint selector_3 = (uint)0;
    int result_1 = (int)0;

    selector_3 = selector_2;
    result_1 = int(0);
    uint _e4 = selector_3;
    switch(_e4) {
        case 0u:
        case 1u: {
            {
                int _e6 = result_1;
                result_1 = asint(asuint(_e6) + asuint(int(3)));
            }
            {
                int _e9 = result_1;
                result_1 = asint(asuint(_e9) + asuint(int(5)));
                break;
            }
        }
        case 2u: {
            int _e9 = result_1;
            result_1 = asint(asuint(_e9) + asuint(int(5)));
            break;
        }
        default: {
            result_1 = int(7);
            break;
        }
    }
    int _e12 = result_1;
    return _e12;
}

int nested_switches(int outer, int inner)
{
    int outer_1 = (int)0;
    int inner_1 = (int)0;
    int result_2 = (int)0;

    outer_1 = outer;
    inner_1 = inner;
    result_2 = int(0);
    int _e6 = outer_1;
    switch(_e6) {
        case 0: {
            {
                int _e8 = result_2;
                result_2 = asint(asuint(_e8) + asuint(int(1)));
                int _e10 = inner_1;
                switch(_e10) {
                    case 0: {
                        {
                            int _e12 = result_2;
                            result_2 = asint(asuint(_e12) + asuint(int(10)));
                        }
                        {
                            int _e15 = result_2;
                            result_2 = asint(asuint(_e15) + asuint(int(20)));
                            break;
                        }
                    }
                    default: {
                        int _e15 = result_2;
                        result_2 = asint(asuint(_e15) + asuint(int(20)));
                        break;
                    }
                }
            }
            {
                int _e18 = result_2;
                result_2 = asint(asuint(_e18) + asuint(int(100)));
                break;
            }
        }
        case 1: {
            int _e18 = result_2;
            result_2 = asint(asuint(_e18) + asuint(int(100)));
            break;
        }
        default: {
            result_2 = int(900);
            break;
        }
    }
    int _e21 = result_2;
    return _e21;
}

int loop_jumps(int selector_4)
{
    int selector_5 = (int)0;
    int result_3 = (int)0;
    int iteration = (int)0;

    selector_5 = selector_4;
    result_3 = int(0);
    {
        iteration = int(0);
        uint2 loop_bound = uint2(4294967295u, 4294967295u);
        bool loop_init = true;
        while(true) {
            if (all(loop_bound == uint2(0u, 0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e9 = iteration;
                iteration = asint(asuint(_e9) + asuint(int(1)));
            }
            loop_init = false;
            int _e7 = iteration;
            if ((_e7 < int(4))) {
            } else {
                break;
            }
            int _e12 = selector_5;
            bool should_continue = false;
            switch(_e12) {
                case 0: {
                    {
                        int _e14 = iteration;
                        if ((_e14 == int(1))) {
                            should_continue = true;
                            break;
                        }
                        int _e17 = iteration;
                        int _e19 = result_3;
                        result_3 = asint(asuint(_e19) + asuint(asint(asuint(_e17) + asuint(int(1)))));
                    }
                    {
                        int _e22 = iteration;
                        if ((_e22 == int(2))) {
                            break;
                        }
                        int _e25 = result_3;
                        result_3 = asint(asuint(_e25) + asuint(int(10)));
                        break;
                    }
                }
                case 1: {
                    int _e22 = iteration;
                    if ((_e22 == int(2))) {
                        break;
                    }
                    int _e25 = result_3;
                    result_3 = asint(asuint(_e25) + asuint(int(10)));
                    break;
                }
                default: {
                    int _e28 = result_3;
                    result_3 = asint(asuint(_e28) + asuint(int(100)));
                    should_continue = true;
                    break;
                }
            }
            if (should_continue) {
                continue;
            }
            int _e31 = result_3;
            result_3 = asint(asuint(_e31) + asuint(int(1000)));
        }
    }
    int _e33 = result_3;
    return _e33;
}

int terminal_paths(int selector_6, int flag)
{
    int selector_7 = (int)0;
    int flag_1 = (int)0;
    int result_4 = (int)0;

    selector_7 = selector_6;
    flag_1 = flag;
    result_4 = int(0);
    int _e6 = selector_7;
    switch(_e6) {
        case 0: {
            {
                int _e8 = flag_1;
                if ((_e8 != int(0))) {
                    return int(3);
                }
                int _e12 = result_4;
                result_4 = asint(asuint(_e12) + asuint(int(1)));
            }
            {
                int _e15 = result_4;
                return asint(asuint(_e15) + asuint(int(5)));
            }
        }
        case 1: {
            int _e15 = result_4;
            return asint(asuint(_e15) + asuint(int(5)));
        }
        default: {
            {
                result_4 = int(9);
            }
            {
                {
                    int _e19 = result_4;
                    return asint(asuint(_e19) + asuint(int(10)));
                }
            }
            break;
        }
        case 2: {
            {
                int _e19 = result_4;
                return asint(asuint(_e19) + asuint(int(10)));
            }
            break;
        }
    }
}

int update_value(int value, int token)
{
    int value_1 = (int)0;
    int token_1 = (int)0;

    value_1 = value;
    token_1 = token;
    int _e4 = value_1;
    int _e5 = token_1;
    return asint(asuint(_e4) + asuint(_e5));
}

int2 call_results(int selector_8, int counter_1)
{
    int selector_9 = (int)0;
    int counter_2 = (int)0;
    int result_5 = (int)0;
    int named_value = (int)0;
    int named_value_1 = (int)0;
    int named_value_2 = (int)0;

    selector_9 = selector_8;
    counter_2 = counter_1;
    result_5 = int(2);
    int _e6 = selector_9;
    switch(_e6) {
        case 0: {
            {
                {
                    int _e8 = result_5;
                    int _e9 = counter_2;
                    counter_2 = asint(asuint(_e9) + asuint(int(1)));
                    int _e12 = counter_2;
                    const int _e13 = update_value(_e8, _e12);
                    named_value = _e13;
                    int _e14 = named_value;
                    int _e15 = named_value;
                    result_5 = asint(asuint(_e14) + asuint(_e15));
                }
            }
            {
                {
                    int _e18 = result_5;
                    int _e19 = counter_2;
                    counter_2 = asint(asuint(_e19) + asuint(int(1)));
                    int _e22 = counter_2;
                    const int _e23 = update_value(_e18, _e22);
                    named_value_1 = _e23;
                    int _e24 = named_value_1;
                    int _e25 = named_value_1;
                    result_5 = asint(asuint(_e24) * asuint(_e25));
                }
            }
            {
                {
                    int _e28 = result_5;
                    int _e29 = counter_2;
                    counter_2 = asint(asuint(_e29) + asuint(int(1)));
                    int _e32 = counter_2;
                    const int _e33 = update_value(_e28, _e32);
                    named_value_2 = _e33;
                    int _e34 = named_value_2;
                    int _e35 = named_value_2;
                    result_5 = asint(asuint(_e34) + asuint(_e35));
                    break;
                }
            }
            break;
        }
        case 1: {
            {
                {
                    int _e18 = result_5;
                    int _e19 = counter_2;
                    counter_2 = asint(asuint(_e19) + asuint(int(1)));
                    int _e22 = counter_2;
                    const int _e23 = update_value(_e18, _e22);
                    named_value_1 = _e23;
                    int _e24 = named_value_1;
                    int _e25 = named_value_1;
                    result_5 = asint(asuint(_e24) * asuint(_e25));
                }
            }
            {
                {
                    int _e28 = result_5;
                    int _e29 = counter_2;
                    counter_2 = asint(asuint(_e29) + asuint(int(1)));
                    int _e32 = counter_2;
                    const int _e33 = update_value(_e28, _e32);
                    named_value_2 = _e33;
                    int _e34 = named_value_2;
                    int _e35 = named_value_2;
                    result_5 = asint(asuint(_e34) + asuint(_e35));
                    break;
                }
            }
            break;
        }
        default: {
            {
                int _e28 = result_5;
                int _e29 = counter_2;
                counter_2 = asint(asuint(_e29) + asuint(int(1)));
                int _e32 = counter_2;
                const int _e33 = update_value(_e28, _e32);
                named_value_2 = _e33;
                int _e34 = named_value_2;
                int _e35 = named_value_2;
                result_5 = asint(asuint(_e34) + asuint(_e35));
                break;
            }
            break;
        }
    }
    int _e37 = result_5;
    int _e38 = counter_2;
    return int2(_e37, _e38);
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

[numthreads(1, 1, 1)]
void switches()
{
    int counter = (int)0;
    int2 first = (int2)0;
    int2 second = (int2)0;
    int2 third = (int2)0;

    const int _e5 = case_chain(naga_neg(int(1)));
    out_.Store(int(0)*4, asuint(_e5));
    const int _e9 = case_chain(int(0));
    out_.Store(int(1)*4, asuint(_e9));
    const int _e13 = case_chain(int(1));
    out_.Store(int(2)*4, asuint(_e13));
    const int _e17 = case_chain(int(2));
    out_.Store(int(3)*4, asuint(_e17));
    const int _e21 = case_chain(int(8));
    out_.Store(int(4)*4, asuint(_e21));
    const int _e25 = grouped_labels(0u);
    out_.Store(int(5)*4, asuint(_e25));
    const int _e29 = grouped_labels(1u);
    out_.Store(int(6)*4, asuint(_e29));
    const int _e33 = grouped_labels(2u);
    out_.Store(int(7)*4, asuint(_e33));
    const int _e37 = grouped_labels(8u);
    out_.Store(int(8)*4, asuint(_e37));
    const int _e42 = nested_switches(int(0), int(0));
    out_.Store(int(9)*4, asuint(_e42));
    const int _e47 = nested_switches(int(0), int(8));
    out_.Store(int(10)*4, asuint(_e47));
    const int _e52 = nested_switches(int(1), int(0));
    out_.Store(int(11)*4, asuint(_e52));
    const int _e57 = nested_switches(int(8), int(0));
    out_.Store(int(12)*4, asuint(_e57));
    const int _e61 = loop_jumps(int(0));
    out_.Store(int(13)*4, asuint(_e61));
    const int _e65 = loop_jumps(int(1));
    out_.Store(int(14)*4, asuint(_e65));
    const int _e69 = loop_jumps(int(8));
    out_.Store(int(15)*4, asuint(_e69));
    const int _e74 = terminal_paths(int(0), int(1));
    out_.Store(int(16)*4, asuint(_e74));
    const int _e79 = terminal_paths(int(0), int(0));
    out_.Store(int(17)*4, asuint(_e79));
    const int _e84 = terminal_paths(int(1), int(1));
    out_.Store(int(18)*4, asuint(_e84));
    const int _e89 = terminal_paths(int(8), int(0));
    out_.Store(int(19)*4, asuint(_e89));
    const int _e94 = terminal_paths(int(2), int(0));
    out_.Store(int(20)*4, asuint(_e94));
    counter = int(0);
    int _e97 = counter;
    counter = asint(asuint(_e97) + asuint(int(1)));
    switch(_e97) {
        case 0: {
            {
                int _e102 = counter;
                out_.Store(int(21)*4, asuint(_e102));
            }
            {
                int _e106 = counter;
                out_.Store(int(22)*4, asuint(asint(asuint(_e106) + asuint(int(1)))));
                break;
            }
        }
        case 1: {
            int _e106 = counter;
            out_.Store(int(22)*4, asuint(asint(asuint(_e106) + asuint(int(1)))));
            break;
        }
        default: {
            out_.Store(int(21)*4, asuint(int(99)));
            break;
        }
    }
    int _e113 = counter;
    out_.Store(int(23)*4, asuint(_e113));
    const int2 _e117 = call_results(int(0), int(0));
    first = _e117;
    int _e121 = first.y;
    const int2 _e122 = call_results(int(1), _e121);
    second = _e122;
    int _e126 = second.y;
    const int2 _e127 = call_results(int(8), _e126);
    third = _e127;
    int _e131 = first.x;
    out_.Store(int(24)*4, asuint(_e131));
    int _e135 = second.x;
    out_.Store(int(25)*4, asuint(_e135));
    int _e139 = third.x;
    out_.Store(int(26)*4, asuint(_e139));
    int _e143 = third.y;
    out_.Store(int(27)*4, asuint(_e143));
    int _e145 = third.y;
    counter = _e145;
    int _e149 = third.x;
    int _e150 = counter;
    counter = asint(asuint(_e150) + asuint(int(1)));
    int _e153 = counter;
    const int _e154 = update_value(_e149, _e153);
    out_.Store(int(28)*4, asuint(_e154));
    int _e157 = counter;
    out_.Store(int(29)*4, asuint(_e157));
}
