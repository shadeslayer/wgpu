struct Output {
    values: array<i32>,
}

@group(0) @binding(0)
var<storage, read_write> global: Output;

fn case_chain(selector: i32) -> i32 {
    var selector_1: i32;
    var result: i32 = 0i;

    selector_1 = selector;
    let _e4 = selector_1;
    switch _e4 {
        case -1: {
            {
                let _e5 = result;
                result = (_e5 + 1i);
            }
            {
                let _e8 = result;
                result = (_e8 + 10i);
            }
            {
                let _e11 = result;
                result = (_e11 + 100i);
            }
        }
        case 0: {
            {
                let _e8 = result;
                result = (_e8 + 10i);
            }
            {
                let _e11 = result;
                result = (_e11 + 100i);
            }
        }
        case 1: {
            let _e11 = result;
            result = (_e11 + 100i);
        }
        default: {
            {
                let _e14 = result;
                result = (_e14 + 1000i);
            }
            {
                let _e17 = result;
                result = (_e17 + 10000i);
            }
        }
        case 2: {
            let _e17 = result;
            result = (_e17 + 10000i);
        }
    }
    let _e20 = result;
    return _e20;
}

fn grouped_labels(selector_2: u32) -> i32 {
    var selector_3: u32;
    var result_1: i32 = 0i;

    selector_3 = selector_2;
    let _e4 = selector_3;
    switch _e4 {
        case 0u, 1u: {
            {
                let _e5 = result_1;
                result_1 = (_e5 + 3i);
            }
            {
                let _e8 = result_1;
                result_1 = (_e8 + 5i);
            }
        }
        case 2u: {
            let _e8 = result_1;
            result_1 = (_e8 + 5i);
        }
        default: {
            result_1 = 7i;
        }
    }
    let _e12 = result_1;
    return _e12;
}

fn nested_switches(outer: i32, inner: i32) -> i32 {
    var outer_1: i32;
    var inner_1: i32;
    var result_2: i32 = 0i;

    outer_1 = outer;
    inner_1 = inner;
    let _e6 = outer_1;
    switch _e6 {
        case 0: {
            {
                let _e7 = result_2;
                result_2 = (_e7 + 1i);
                let _e10 = inner_1;
                switch _e10 {
                    case 0: {
                        {
                            let _e11 = result_2;
                            result_2 = (_e11 + 10i);
                        }
                        {
                            let _e14 = result_2;
                            result_2 = (_e14 + 20i);
                        }
                    }
                    default: {
                        let _e14 = result_2;
                        result_2 = (_e14 + 20i);
                    }
                }
            }
            {
                let _e17 = result_2;
                result_2 = (_e17 + 100i);
            }
        }
        case 1: {
            let _e17 = result_2;
            result_2 = (_e17 + 100i);
        }
        default: {
            result_2 = 900i;
        }
    }
    let _e21 = result_2;
    return _e21;
}

fn loop_jumps(selector_4: i32) -> i32 {
    var selector_5: i32;
    var result_3: i32 = 0i;
    var iteration: i32 = 0i;

    selector_5 = selector_4;
    loop {
        let _e6 = iteration;
        if !((_e6 < 4i)) {
            break;
        }
        {
            let _e13 = selector_5;
            switch _e13 {
                case 0: {
                    {
                        let _e14 = iteration;
                        if (_e14 == 1i) {
                            {
                                continue;
                            }
                        }
                        let _e17 = result_3;
                        let _e18 = iteration;
                        result_3 = (_e17 + (_e18 + 1i));
                    }
                    {
                        let _e22 = iteration;
                        if (_e22 == 2i) {
                            {
                                break;
                            }
                        }
                        let _e25 = result_3;
                        result_3 = (_e25 + 10i);
                    }
                }
                case 1: {
                    let _e22 = iteration;
                    if (_e22 == 2i) {
                        {
                            break;
                        }
                    }
                    let _e25 = result_3;
                    result_3 = (_e25 + 10i);
                }
                default: {
                    let _e28 = result_3;
                    result_3 = (_e28 + 100i);
                    continue;
                }
            }
            let _e31 = result_3;
            result_3 = (_e31 + 1000i);
        }
        continuing {
            let _e10 = iteration;
            iteration = (_e10 + 1i);
        }
    }
    let _e34 = result_3;
    return _e34;
}

fn terminal_paths(selector_6: i32, flag: i32) -> i32 {
    var selector_7: i32;
    var flag_1: i32;
    var result_4: i32 = 0i;

    selector_7 = selector_6;
    flag_1 = flag;
    let _e6 = selector_7;
    switch _e6 {
        case 0: {
            {
                let _e7 = flag_1;
                if (_e7 != 0i) {
                    {
                        return 3i;
                    }
                }
                let _e11 = result_4;
                result_4 = (_e11 + 1i);
            }
            {
                let _e14 = result_4;
                return (_e14 + 5i);
            }
            {
                result_4 = 9i;
            }
            {
                {
                    let _e18 = result_4;
                    return (_e18 + 10i);
                }
            }
        }
        case 1: {
            {
                let _e14 = result_4;
                return (_e14 + 5i);
            }
            {
                result_4 = 9i;
            }
            {
                {
                    let _e18 = result_4;
                    return (_e18 + 10i);
                }
            }
        }
        default: {
            {
                result_4 = 9i;
            }
            {
                {
                    let _e18 = result_4;
                    return (_e18 + 10i);
                }
            }
        }
        case 2: {
            {
                let _e18 = result_4;
                return (_e18 + 10i);
            }
        }
    }
    return -1i;
}

fn update_value(value: i32) -> i32 {
    var value_1: i32;

    value_1 = value;
    let _e7 = global.values[29];
    global.values[29i] = (_e7 + 1i);
    let _e10 = value_1;
    let _e12 = global.values[29];
    return (_e10 + _e12);
}

fn call_results(selector_8: i32) -> i32 {
    var selector_9: i32;
    var result_5: i32 = 2i;
    var named_value: i32;
    var named_value_1: i32;
    var named_value_2: i32;

    selector_9 = selector_8;
    let _e4 = selector_9;
    switch _e4 {
        case 0: {
            {
                {
                    let _e5 = result_5;
                    let _e6 = update_value(_e5);
                    named_value = _e6;
                    let _e8 = named_value;
                    let _e9 = named_value;
                    result_5 = (_e8 + _e9);
                }
            }
            {
                {
                    let _e11 = result_5;
                    let _e12 = update_value(_e11);
                    named_value_1 = _e12;
                    let _e14 = named_value_1;
                    let _e15 = named_value_1;
                    result_5 = (_e14 * _e15);
                }
            }
            {
                {
                    let _e17 = result_5;
                    let _e18 = update_value(_e17);
                    named_value_2 = _e18;
                    let _e20 = named_value_2;
                    let _e21 = named_value_2;
                    result_5 = (_e20 + _e21);
                    break;
                }
            }
        }
        case 1: {
            {
                {
                    let _e11 = result_5;
                    let _e12 = update_value(_e11);
                    named_value_1 = _e12;
                    let _e14 = named_value_1;
                    let _e15 = named_value_1;
                    result_5 = (_e14 * _e15);
                }
            }
            {
                {
                    let _e17 = result_5;
                    let _e18 = update_value(_e17);
                    named_value_2 = _e18;
                    let _e20 = named_value_2;
                    let _e21 = named_value_2;
                    result_5 = (_e20 + _e21);
                    break;
                }
            }
        }
        default: {
            {
                let _e17 = result_5;
                let _e18 = update_value(_e17);
                named_value_2 = _e18;
                let _e20 = named_value_2;
                let _e21 = named_value_2;
                result_5 = (_e20 + _e21);
                break;
            }
        }
    }
    let _e23 = result_5;
    return _e23;
}

fn main_1() {
    var counter: i32 = 0i;

    let _e5 = case_chain(-1i);
    global.values[0i] = _e5;
    let _e9 = case_chain(0i);
    global.values[1i] = _e9;
    let _e13 = case_chain(1i);
    global.values[2i] = _e13;
    let _e17 = case_chain(2i);
    global.values[3i] = _e17;
    let _e21 = case_chain(8i);
    global.values[4i] = _e21;
    let _e25 = grouped_labels(0u);
    global.values[5i] = _e25;
    let _e29 = grouped_labels(1u);
    global.values[6i] = _e29;
    let _e33 = grouped_labels(2u);
    global.values[7i] = _e33;
    let _e37 = grouped_labels(8u);
    global.values[8i] = _e37;
    let _e42 = nested_switches(0i, 0i);
    global.values[9i] = _e42;
    let _e47 = nested_switches(0i, 8i);
    global.values[10i] = _e47;
    let _e52 = nested_switches(1i, 0i);
    global.values[11i] = _e52;
    let _e57 = nested_switches(8i, 0i);
    global.values[12i] = _e57;
    let _e61 = loop_jumps(0i);
    global.values[13i] = _e61;
    let _e65 = loop_jumps(1i);
    global.values[14i] = _e65;
    let _e69 = loop_jumps(8i);
    global.values[15i] = _e69;
    let _e74 = terminal_paths(0i, 1i);
    global.values[16i] = _e74;
    let _e79 = terminal_paths(0i, 0i);
    global.values[17i] = _e79;
    let _e84 = terminal_paths(1i, 1i);
    global.values[18i] = _e84;
    let _e89 = terminal_paths(8i, 0i);
    global.values[19i] = _e89;
    let _e94 = terminal_paths(2i, 0i);
    global.values[20i] = _e94;
    let _e97 = counter;
    counter = (_e97 + 1i);
    switch _e97 {
        case 0: {
            {
                let _e102 = counter;
                global.values[21i] = _e102;
            }
            {
                let _e105 = counter;
                global.values[22i] = (_e105 + 1i);
            }
        }
        case 1: {
            let _e105 = counter;
            global.values[22i] = (_e105 + 1i);
        }
        default: {
            global.values[21i] = 99i;
        }
    }
    let _e113 = counter;
    global.values[23i] = _e113;
    global.values[29i] = 0i;
    let _e120 = call_results(0i);
    global.values[24i] = _e120;
    let _e124 = call_results(1i);
    global.values[25i] = _e124;
    let _e128 = call_results(8i);
    global.values[26i] = _e128;
    let _e132 = global.values[29];
    global.values[27i] = _e132;
    let _e136 = global.values[26];
    let _e137 = update_value(_e136);
    global.values[28i] = _e137;
    return;
}

@compute @workgroup_size(1, 1, 1)
fn main() {
    main_1();
    return;
}
