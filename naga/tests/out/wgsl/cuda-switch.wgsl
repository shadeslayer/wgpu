@group(0) @binding(0)
var<storage, read_write> out: array<i32>;

fn case_chain(selector: i32) -> i32 {
    var selector_1: i32;
    var result: i32;

    selector_1 = selector;
    result = 0i;
    let _e4 = selector_1;
    switch _e4 {
        case -1: {
            {
                let _e6 = result;
                result = (_e6 + 1i);
            }
            {
                let _e9 = result;
                result = (_e9 + 10i);
            }
            {
                let _e12 = result;
                result = (_e12 + 100i);
                break;
            }
        }
        case 0: {
            {
                let _e9 = result;
                result = (_e9 + 10i);
            }
            {
                let _e12 = result;
                result = (_e12 + 100i);
                break;
            }
        }
        case 1: {
            let _e12 = result;
            result = (_e12 + 100i);
            break;
        }
        default: {
            {
                let _e15 = result;
                result = (_e15 + 1000i);
            }
            {
                let _e18 = result;
                result = (_e18 + 10000i);
                break;
            }
        }
        case 2: {
            let _e18 = result;
            result = (_e18 + 10000i);
            break;
        }
    }
    let _e20 = result;
    return _e20;
}

fn grouped_labels(selector_2: u32) -> i32 {
    var selector_3: u32;
    var result_1: i32;

    selector_3 = selector_2;
    result_1 = 0i;
    let _e4 = selector_3;
    switch _e4 {
        case 0u, 1u: {
            {
                let _e6 = result_1;
                result_1 = (_e6 + 3i);
            }
            {
                let _e9 = result_1;
                result_1 = (_e9 + 5i);
                break;
            }
        }
        case 2u: {
            let _e9 = result_1;
            result_1 = (_e9 + 5i);
            break;
        }
        default: {
            result_1 = 7i;
            break;
        }
    }
    let _e12 = result_1;
    return _e12;
}

fn nested_switches(outer: i32, inner: i32) -> i32 {
    var outer_1: i32;
    var inner_1: i32;
    var result_2: i32;

    outer_1 = outer;
    inner_1 = inner;
    result_2 = 0i;
    let _e6 = outer_1;
    switch _e6 {
        case 0: {
            {
                let _e8 = result_2;
                result_2 = (_e8 + 1i);
                let _e10 = inner_1;
                switch _e10 {
                    case 0: {
                        {
                            let _e12 = result_2;
                            result_2 = (_e12 + 10i);
                        }
                        {
                            let _e15 = result_2;
                            result_2 = (_e15 + 20i);
                            break;
                        }
                    }
                    default: {
                        let _e15 = result_2;
                        result_2 = (_e15 + 20i);
                        break;
                    }
                }
            }
            {
                let _e18 = result_2;
                result_2 = (_e18 + 100i);
                break;
            }
        }
        case 1: {
            let _e18 = result_2;
            result_2 = (_e18 + 100i);
            break;
        }
        default: {
            result_2 = 900i;
            break;
        }
    }
    let _e21 = result_2;
    return _e21;
}

fn loop_jumps(selector_4: i32) -> i32 {
    var selector_5: i32;
    var result_3: i32;
    var iteration: i32;

    selector_5 = selector_4;
    result_3 = 0i;
    {
        iteration = 0i;
        loop {
            let _e7 = iteration;
            if (_e7 < 4i) {
            } else {
                break;
            }
            let _e12 = selector_5;
            switch _e12 {
                case 0: {
                    {
                        let _e14 = iteration;
                        if (_e14 == 1i) {
                            continue;
                        }
                        let _e17 = iteration;
                        let _e19 = result_3;
                        result_3 = (_e19 + (_e17 + 1i));
                    }
                    {
                        let _e22 = iteration;
                        if (_e22 == 2i) {
                            break;
                        }
                        let _e25 = result_3;
                        result_3 = (_e25 + 10i);
                        break;
                    }
                }
                case 1: {
                    let _e22 = iteration;
                    if (_e22 == 2i) {
                        break;
                    }
                    let _e25 = result_3;
                    result_3 = (_e25 + 10i);
                    break;
                }
                default: {
                    let _e28 = result_3;
                    result_3 = (_e28 + 100i);
                    continue;
                }
            }
            let _e31 = result_3;
            result_3 = (_e31 + 1000i);
            continuing {
                let _e9 = iteration;
                iteration = (_e9 + 1i);
            }
        }
    }
    let _e33 = result_3;
    return _e33;
}

fn terminal_paths(selector_6: i32, flag: i32) -> i32 {
    var selector_7: i32;
    var flag_1: i32;
    var result_4: i32;

    selector_7 = selector_6;
    flag_1 = flag;
    result_4 = 0i;
    let _e6 = selector_7;
    switch _e6 {
        case 0: {
            {
                let _e8 = flag_1;
                if (_e8 != 0i) {
                    return 3i;
                }
                let _e12 = result_4;
                result_4 = (_e12 + 1i);
            }
            {
                let _e15 = result_4;
                return (_e15 + 5i);
            }
        }
        case 1: {
            let _e15 = result_4;
            return (_e15 + 5i);
        }
        default: {
            {
                result_4 = 9i;
            }
            {
                {
                    let _e19 = result_4;
                    return (_e19 + 10i);
                }
            }
        }
        case 2: {
            {
                let _e19 = result_4;
                return (_e19 + 10i);
            }
        }
    }
}

fn update_value(value: i32, token: i32) -> i32 {
    var value_1: i32;
    var token_1: i32;

    value_1 = value;
    token_1 = token;
    let _e4 = value_1;
    let _e5 = token_1;
    return (_e4 + _e5);
}

fn call_results(selector_8: i32, counter_1: i32) -> vec2<i32> {
    var selector_9: i32;
    var counter_2: i32;
    var result_5: i32;
    var named_value: i32;
    var named_value_1: i32;
    var named_value_2: i32;

    selector_9 = selector_8;
    counter_2 = counter_1;
    result_5 = 2i;
    let _e6 = selector_9;
    switch _e6 {
        case 0: {
            {
                {
                    let _e8 = result_5;
                    let _e9 = counter_2;
                    counter_2 = (_e9 + 1i);
                    let _e12 = counter_2;
                    let _e13 = update_value(_e8, _e12);
                    named_value = _e13;
                    let _e14 = named_value;
                    let _e15 = named_value;
                    result_5 = (_e14 + _e15);
                }
            }
            {
                {
                    let _e18 = result_5;
                    let _e19 = counter_2;
                    counter_2 = (_e19 + 1i);
                    let _e22 = counter_2;
                    let _e23 = update_value(_e18, _e22);
                    named_value_1 = _e23;
                    let _e24 = named_value_1;
                    let _e25 = named_value_1;
                    result_5 = (_e24 * _e25);
                }
            }
            {
                {
                    let _e28 = result_5;
                    let _e29 = counter_2;
                    counter_2 = (_e29 + 1i);
                    let _e32 = counter_2;
                    let _e33 = update_value(_e28, _e32);
                    named_value_2 = _e33;
                    let _e34 = named_value_2;
                    let _e35 = named_value_2;
                    result_5 = (_e34 + _e35);
                    break;
                }
            }
        }
        case 1: {
            {
                {
                    let _e18 = result_5;
                    let _e19 = counter_2;
                    counter_2 = (_e19 + 1i);
                    let _e22 = counter_2;
                    let _e23 = update_value(_e18, _e22);
                    named_value_1 = _e23;
                    let _e24 = named_value_1;
                    let _e25 = named_value_1;
                    result_5 = (_e24 * _e25);
                }
            }
            {
                {
                    let _e28 = result_5;
                    let _e29 = counter_2;
                    counter_2 = (_e29 + 1i);
                    let _e32 = counter_2;
                    let _e33 = update_value(_e28, _e32);
                    named_value_2 = _e33;
                    let _e34 = named_value_2;
                    let _e35 = named_value_2;
                    result_5 = (_e34 + _e35);
                    break;
                }
            }
        }
        default: {
            {
                let _e28 = result_5;
                let _e29 = counter_2;
                counter_2 = (_e29 + 1i);
                let _e32 = counter_2;
                let _e33 = update_value(_e28, _e32);
                named_value_2 = _e33;
                let _e34 = named_value_2;
                let _e35 = named_value_2;
                result_5 = (_e34 + _e35);
                break;
            }
        }
    }
    let _e37 = result_5;
    let _e38 = counter_2;
    return vec2<i32>(_e37, _e38);
}

@compute @workgroup_size(1, 1, 1)
fn switches() {
    var counter: i32;
    var first: vec2<i32>;
    var second: vec2<i32>;
    var third: vec2<i32>;

    let _e5 = case_chain(-(1i));
    out[0i] = _e5;
    let _e9 = case_chain(0i);
    out[1i] = _e9;
    let _e13 = case_chain(1i);
    out[2i] = _e13;
    let _e17 = case_chain(2i);
    out[3i] = _e17;
    let _e21 = case_chain(8i);
    out[4i] = _e21;
    let _e25 = grouped_labels(0u);
    out[5i] = _e25;
    let _e29 = grouped_labels(1u);
    out[6i] = _e29;
    let _e33 = grouped_labels(2u);
    out[7i] = _e33;
    let _e37 = grouped_labels(8u);
    out[8i] = _e37;
    let _e42 = nested_switches(0i, 0i);
    out[9i] = _e42;
    let _e47 = nested_switches(0i, 8i);
    out[10i] = _e47;
    let _e52 = nested_switches(1i, 0i);
    out[11i] = _e52;
    let _e57 = nested_switches(8i, 0i);
    out[12i] = _e57;
    let _e61 = loop_jumps(0i);
    out[13i] = _e61;
    let _e65 = loop_jumps(1i);
    out[14i] = _e65;
    let _e69 = loop_jumps(8i);
    out[15i] = _e69;
    let _e74 = terminal_paths(0i, 1i);
    out[16i] = _e74;
    let _e79 = terminal_paths(0i, 0i);
    out[17i] = _e79;
    let _e84 = terminal_paths(1i, 1i);
    out[18i] = _e84;
    let _e89 = terminal_paths(8i, 0i);
    out[19i] = _e89;
    let _e94 = terminal_paths(2i, 0i);
    out[20i] = _e94;
    counter = 0i;
    let _e97 = counter;
    counter = (_e97 + 1i);
    switch _e97 {
        case 0: {
            {
                let _e102 = counter;
                out[21i] = _e102;
            }
            {
                let _e106 = counter;
                out[22i] = (_e106 + 1i);
                break;
            }
        }
        case 1: {
            let _e106 = counter;
            out[22i] = (_e106 + 1i);
            break;
        }
        default: {
            out[21i] = 99i;
            break;
        }
    }
    let _e113 = counter;
    out[23i] = _e113;
    let _e117 = call_results(0i, 0i);
    first = _e117;
    let _e121 = first.y;
    let _e122 = call_results(1i, _e121);
    second = _e122;
    let _e126 = second.y;
    let _e127 = call_results(8i, _e126);
    third = _e127;
    let _e131 = first.x;
    out[24i] = _e131;
    let _e135 = second.x;
    out[25i] = _e135;
    let _e139 = third.x;
    out[26i] = _e139;
    let _e143 = third.y;
    out[27i] = _e143;
    let _e145 = third.y;
    counter = _e145;
    let _e149 = third.x;
    let _e150 = counter;
    counter = (_e150 + 1i);
    let _e153 = counter;
    let _e154 = update_value(_e149, _e153);
    out[28i] = _e154;
    let _e157 = counter;
    out[29i] = _e157;
}
