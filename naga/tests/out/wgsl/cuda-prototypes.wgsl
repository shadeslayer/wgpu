struct Packet_cuda_record_0_ {
    key: i32,
    values: array<i32, 2>,
}

@group(0) @binding(0)
var<storage, read_write> out: array<u32>;
var<private> blockIdx_1: vec3<u32>;
var<private> threadIdx_1: vec3<u32>;

fn route(value: f32) -> i32 {
    var value_1: f32;

    value_1 = value;
    let _e3 = value_1;
    return (10i + i32(_e3));
}

fn earlier(value_2: i32) -> i32 {
    var value_3: i32;

    value_3 = value_2;
    let _e2 = value_3;
    let _e4 = route(f32(_e2));
    return _e4;
}

fn route_1(value_4: i32) -> i32 {
    var value_5: i32;

    value_5 = value_4;
    let _e3 = value_5;
    return (20i + _e3);
}

fn forward(value_6: i32) -> i32 {
    var value_7: i32;

    value_7 = value_6;
    let _e2 = value_7;
    let _e3 = route_1(_e2);
    let _e4 = value_7;
    let _e5 = earlier(_e4);
    return (_e3 + _e5);
}

fn packet(value_8: i32) -> Packet_cuda_record_0_ {
    var value_9: i32;
    var local: Packet_cuda_record_0_;

    value_9 = value_8;
    let _e3 = value_9;
    local.key = _e3;
    let _e7 = value_9;
    local.values[0] = (_e7 + 1i);
    let _e12 = value_9;
    local.values[1] = (_e12 + 2i);
    let _e14 = local;
    return _e14;
}

fn adjust(value_10: Packet_cuda_record_0_) -> Packet_cuda_record_0_ {
    var value_11: Packet_cuda_record_0_;

    value_11 = value_10;
    let _e4 = value_11.key;
    value_11.key = (_e4 + 4i);
    let _e10 = value_11.values[1i];
    value_11.values[1i] = (_e10 + 6i);
    let _e12 = value_11;
    return _e12;
}

fn operator_u002b(left: Packet_cuda_record_0_, right: Packet_cuda_record_0_) -> Packet_cuda_record_0_ {
    var left_1: Packet_cuda_record_0_;
    var right_1: Packet_cuda_record_0_;
    var local_1: Packet_cuda_record_0_;

    left_1 = left;
    right_1 = right;
    let _e7 = left_1.key;
    let _e8 = right_1.key;
    local_1.key = (_e7 + _e8);
    let _e18 = left_1.values[0i];
    let _e19 = right_1.values[0i];
    local_1.values[0] = (_e18 + _e19);
    let _e29 = left_1.values[1i];
    let _e30 = right_1.values[1i];
    local_1.values[1] = (_e29 + _e30);
    let _e32 = local_1;
    return _e32;
}

fn pair(first_1: i32, second_1: i32) -> i32 {
    var first_2: i32;
    var second_2: i32;

    first_2 = first_1;
    second_2 = second_1;
    let _e5 = first_2;
    let _e7 = second_2;
    return ((_e5 * 10i) + _e7);
}

fn consume(value_12: i32) {
    var value_13: i32;

    value_13 = value_12;
    let _e2 = value_13;
    if (_e2 != 0i) {
        return;
    } else {
        return;
    }
}

fn gen__naga_sqrtf(input_bits: u32) -> u32 {
    var significand: u32;
    var adjusted_exponent: u32;
    var upper_word: u32;
    var lower_word: u32;
    var root: u32;
    var remainder: u32;
    var iteration: u32;

    let _e5 = (input_bits & 2147483647u);
    if ((_e5 == 0u) || (input_bits == 2139095040u)) {
        return input_bits;
    }
    if (((input_bits & 2147483648u) != 0u) || (_e5 > 2139095040u)) {
        return 2143289344u;
    }
    let _e19 = (input_bits & 8388607u);
    let _e20 = (_e5 >> 23u);
    significand = (_e19 | 8388608u);
    adjusted_exponent = (_e20 + 127u);
    if (_e20 == 0u) {
        let _e28 = (countLeadingZeros(_e19) - 8u);
        significand = (_e19 << _e28);
        adjusted_exponent = (128u - _e28);
    }
    let _e32 = significand;
    let _e33 = adjusted_exponent;
    let _e34 = (_e33 & 1u);
    upper_word = (_e32 >> (9u - _e34));
    lower_word = (_e32 << (23u + _e34));
    root = 0u;
    remainder = 0u;
    iteration = 0u;
    loop {
        let _e50 = iteration;
        if (_e50 >= 24u) {
            break;
        }
        let _e52 = upper_word;
        let _e53 = lower_word;
        let _e54 = root;
        let _e55 = remainder;
        let _e63 = ((_e55 << 2u) | (_e52 >> 14u));
        let _e65 = ((_e54 << 2u) | 1u);
        let _e66 = (_e63 >= _e65);
        let _e69 = (_e54 << 1u);
        upper_word = (((_e52 << 2u) | (_e53 >> 30u)) & 65535u);
        lower_word = (_e53 << 2u);
        remainder = select(_e63, (_e63 - _e65), _e66);
        root = select(_e69, (_e69 | 1u), _e66);
        continuing {
            let _e72 = iteration;
            iteration = (_e72 + 1u);
        }
    }
    let _e74 = root;
    let _e75 = remainder;
    return (((_e33 >> 1u) << 23u) | (select(_e74, (_e74 + 1u), (_e75 > _e74)) & 8388607u));
}

fn gen__naga_trig_reduce_multiply(left_2: array<u32, 2>, right_2: array<u32, 20>) -> array<u32, 22> {
    var left_digits: array<u32, 2> = array<u32, 2>();
    var right_digits: array<u32, 20> = array<u32, 20>();
    var product: array<u32, 22> = array<u32, 22>();
    var carry: u32 = u32();
    var left_index: u32 = u32();
    var right_index: u32 = u32();
    var result: array<u32, 22> = array<u32, 22>();
    var result_index: u32 = u32();

    left_digits = left_2;
    right_digits = right_2;
    left_index = 0u;
    loop {
        let _e13 = left_index;
        if (_e13 >= 2u) {
            break;
        }
        carry = 0u;
        let _e18 = left_digits[_e13];
        right_index = 0u;
        loop {
            let _e22 = right_index;
            if (_e22 >= 20u) {
                break;
            }
            let _e26 = right_digits[_e22];
            let _e27 = (_e13 + _e22);
            let _e29 = product[_e27];
            let _e30 = carry;
            let _e33 = (((_e18 * _e26) + _e29) + _e30);
            product[_e27] = (_e33 & 65535u);
            carry = (_e33 >> 16u);
            continuing {
                let _e39 = right_index;
                right_index = (_e39 + 1u);
            }
        }
        let _e44 = carry;
        product[(_e13 + 20u)] = _e44;
        continuing {
            let _e46 = left_index;
            left_index = (_e46 + 1u);
        }
    }
    result_index = 0u;
    loop {
        let _e54 = result_index;
        if (_e54 >= 22u) {
            break;
        }
        let _e60 = product[(_e54 + 0u)];
        result[_e54] = _e60;
        continuing {
            let _e62 = result_index;
            result_index = (_e62 + 1u);
        }
    }
    let _e65 = result;
    return _e65;
}

fn gen__naga_trig_extract_product(digits: array<u32, 22>, offset: u32) -> u32 {
    var source: array<u32, 22> = array<u32, 22>();
    var low: u32 = u32();
    var high: u32 = u32();

    source = digits;
    let _e5 = (offset / 16u);
    let _e7 = (offset % 16u);
    let _e13 = (_e5 + 0u);
    if (_e13 < 22u) {
        let _e17 = source[_e13];
        low = _e17;
    }
    let _e19 = (_e5 + 1u);
    if (_e19 < 22u) {
        let _e23 = source[_e19];
        high = _e23;
    }
    let _e24 = low;
    let _e25 = high;
    return (((_e24 >> _e7) | (_e25 << (16u - _e7))) & 65535u);
}

fn gen__naga_trig_multiply(left_3: array<u32, 12>, right_3: array<u32, 12>) -> array<u32, 12> {
    var left_digits_1: array<u32, 12> = array<u32, 12>();
    var right_digits_1: array<u32, 12> = array<u32, 12>();
    var product_1: array<u32, 24> = array<u32, 24>();
    var carry_1: u32 = u32();
    var left_index_1: u32 = u32();
    var right_index_1: u32 = u32();
    var result_1: array<u32, 12> = array<u32, 12>();
    var result_index_1: u32 = u32();

    left_digits_1 = left_3;
    right_digits_1 = right_3;
    left_index_1 = 0u;
    loop {
        let _e13 = left_index_1;
        if (_e13 >= 12u) {
            break;
        }
        carry_1 = 0u;
        let _e18 = left_digits_1[_e13];
        right_index_1 = 0u;
        loop {
            let _e22 = right_index_1;
            if (_e22 >= 12u) {
                break;
            }
            let _e26 = right_digits_1[_e22];
            let _e27 = (_e13 + _e22);
            let _e29 = product_1[_e27];
            let _e30 = carry_1;
            let _e33 = (((_e18 * _e26) + _e29) + _e30);
            product_1[_e27] = (_e33 & 65535u);
            carry_1 = (_e33 >> 16u);
            continuing {
                let _e39 = right_index_1;
                right_index_1 = (_e39 + 1u);
            }
        }
        let _e44 = carry_1;
        product_1[(_e13 + 12u)] = _e44;
        continuing {
            let _e46 = left_index_1;
            left_index_1 = (_e46 + 1u);
        }
    }
    result_index_1 = 0u;
    loop {
        let _e54 = result_index_1;
        if (_e54 >= 12u) {
            break;
        }
        let _e60 = product_1[(_e54 + 11u)];
        result_1[_e54] = _e60;
        continuing {
            let _e62 = result_index_1;
            result_index_1 = (_e62 + 1u);
        }
    }
    let _e65 = result_1;
    return _e65;
}

fn gen__naga_trig_divide(value_14: array<u32, 12>, divisor: u32) -> array<u32, 12> {
    var source_1: array<u32, 12> = array<u32, 12>();
    var result_2: array<u32, 12> = array<u32, 12>();
    var remainder_1: u32 = u32();
    var step_: u32 = u32();

    source_1 = value_14;
    step_ = 0u;
    loop {
        let _e11 = step_;
        if (_e11 >= 12u) {
            break;
        }
        let _e15 = (11u - _e11);
        let _e16 = remainder_1;
        let _e20 = source_1[_e15];
        let _e21 = ((_e16 << 16u) | _e20);
        result_2[_e15] = (_e21 / divisor);
        remainder_1 = (_e21 % divisor);
        continuing {
            let _e25 = step_;
            step_ = (_e25 + 1u);
        }
    }
    let _e28 = result_2;
    return _e28;
}

fn gen__naga_trig_combine(left_4: array<u32, 12>, right_4: array<u32, 12>, subtract: u32) -> array<u32, 12> {
    var left_digits_2: array<u32, 12> = array<u32, 12>();
    var right_digits_2: array<u32, 12> = array<u32, 12>();
    var result_3: array<u32, 12> = array<u32, 12>();
    var carry_2: u32 = u32();
    var digit_index: u32 = u32();

    left_digits_2 = left_4;
    right_digits_2 = right_4;
    let _e12 = (subtract != 0u);
    digit_index = 0u;
    loop {
        let _e16 = digit_index;
        if (_e16 >= 12u) {
            break;
        }
        let _e20 = left_digits_2[_e16];
        let _e22 = right_digits_2[_e16];
        let _e23 = carry_2;
        let _e24 = (_e22 + _e23);
        let _e26 = (_e20 + _e24);
        result_3[_e16] = (select(_e26, (_e20 - _e24), _e12) & 65535u);
        carry_2 = select((_e26 >> 16u), select(0u, 1u, (_e20 < _e24)), _e12);
        continuing {
            let _e38 = digit_index;
            digit_index = (_e38 + 1u);
        }
    }
    let _e41 = result_3;
    return _e41;
}

fn gen__naga_trig_kernel(angle: array<u32, 12>, sine: u32) -> array<u32, 12> {
    var term: array<u32, 12> = array<u32, 12>();
    var total_1: array<u32, 12> = array<u32, 12>();
    var step_1: u32 = u32();

    let _e7 = (sine != 0u);
    if _e7 {
        term = angle;
    } else {
        term = array<u32, 12>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 1u);
    }
    let _e21 = term;
    total_1 = _e21;
    let _e22 = gen__naga_trig_multiply(angle, angle);
    step_1 = 0u;
    loop {
        let _e26 = step_1;
        if (_e26 >= 8u) {
            break;
        }
        let _e30 = (_e26 + 1u);
        let _e32 = (_e30 * 2u);
        let _e39 = term;
        let _e40 = gen__naga_trig_multiply(_e39, _e22);
        let _e41 = gen__naga_trig_divide(_e40, (_e32 * select((_e32 - 1u), (_e32 + 1u), _e7)));
        term = _e41;
        let _e42 = total_1;
        let _e45 = gen__naga_trig_combine(_e42, _e41, (_e30 & 1u));
        total_1 = _e45;
        continuing {
            let _e46 = step_1;
            step_1 = (_e46 + 1u);
        }
    }
    let _e49 = total_1;
    return _e49;
}

fn gen__naga_trig_extract_fixed(digits_1: array<u32, 12>, offset_1: u32) -> u32 {
    var source_2: array<u32, 12> = array<u32, 12>();
    var low_1: u32 = u32();
    var high_1: u32 = u32();

    source_2 = digits_1;
    let _e5 = (offset_1 / 16u);
    let _e7 = (offset_1 % 16u);
    let _e13 = (_e5 + 0u);
    if (_e13 < 12u) {
        let _e17 = source_2[_e13];
        low_1 = _e17;
    }
    let _e19 = (_e5 + 1u);
    if (_e19 < 12u) {
        let _e23 = source_2[_e19];
        high_1 = _e23;
    }
    let _e24 = low_1;
    let _e25 = high_1;
    return (((_e24 >> _e7) | (_e25 << (16u - _e7))) & 65535u);
}

fn gen__naga_trig_round(digits_2: array<u32, 12>) -> u32 {
    var source_3: array<u32, 12> = array<u32, 12>();
    var highest_bit: u32 = u32();
    var nonzero: u32 = u32();
    var digit_index_1: u32 = u32();
    var sticky: u32 = u32();
    var sticky_index: u32 = u32();

    source_3 = digits_2;
    digit_index_1 = 0u;
    loop {
        let _e10 = digit_index_1;
        if (_e10 >= 12u) {
            break;
        }
        let _e14 = source_3[_e10];
        let _e15 = nonzero;
        nonzero = (_e15 | _e14);
        if (_e14 != 0u) {
            highest_bit = ((_e10 * 16u) + (31u - countLeadingZeros(_e14)));
        }
        continuing {
            let _e25 = digit_index_1;
            digit_index_1 = (_e25 + 1u);
        }
    }
    let _e28 = nonzero;
    if (_e28 == 0u) {
        return 0u;
    }
    let _e32 = highest_bit;
    let _e38 = select(27u, (_e32 - 23u), (_e32 >= 50u));
    let _e39 = gen__naga_trig_extract_fixed(digits_2, _e38);
    let _e42 = gen__naga_trig_extract_fixed(digits_2, (_e38 + 16u));
    let _e45 = (_e39 | (_e42 << 16u));
    let _e47 = (_e38 - 1u);
    let _e48 = gen__naga_trig_extract_fixed(digits_2, _e47);
    let _e54 = (_e47 / 16u);
    sticky_index = 0u;
    loop {
        let _e66 = sticky_index;
        if (_e66 >= 12u) {
            break;
        }
        let _e76 = source_3[_e66];
        let _e78 = sticky;
        sticky = (_e78 | (_e76 & select(select(0u, ((1u << (_e47 % 16u)) - 1u), (_e66 == _e54)), 65535u, (_e66 < _e54))));
        continuing {
            let _e80 = sticky_index;
            sticky_index = (_e80 + 1u);
        }
    }
    let _e83 = sticky;
    let _e93 = (_e45 + select(0u, 1u, (((_e48 & 1u) != 0u) && ((_e83 | (_e45 & 1u)) != 0u))));
    if (_e32 < 50u) {
        return _e93;
    }
    let _e97 = (_e93 == 16777216u);
    let _e102 = (_e32 - 49u);
    return ((select(_e102, (_e102 + 1u), _e97) << 23u) | (select(_e93, (_e93 >> 1u), _e97) & 8388607u));
}

fn gen__naga_trigonometry(input_bits_1: u32, cosine: u32) -> u32 {
    var quadrant: u32 = u32();
    var fraction: array<u32, 12> = array<u32, 12>();
    var fraction_index: u32 = u32();

    let _e3 = (input_bits_1 & 2147483647u);
    if (_e3 >= 2139095040u) {
        return 2143289344u;
    }
    let _e8 = (cosine != 0u);
    if (_e3 <= 956301312u) {
        return select(input_bits_1, 1065353216u, _e8);
    }
    let _e16 = (470u - (_e3 >> 23u));
    let _e20 = ((_e3 & 8388607u) | 8388608u);
    let _e47 = gen__naga_trig_reduce_multiply(array<u32, 2>((_e20 & 65535u), (_e20 >> 16u)), array<u32, 20>(53984u, 16973u, 28218u, 46884u, 50529u, 57019u, 25515u, 65105u, 36929u, 15427u, 38297u, 56162u, 56768u, 62772u, 22481u, 64551u, 5417u, 20036u, 33646u, 41721u));
    let _e50 = gen__naga_trig_extract_product(_e47, _e16);
    quadrant = (_e50 & 3u);
    fraction_index = 0u;
    loop {
        let _e60 = fraction_index;
        if (_e60 >= 11u) {
            break;
        }
        let _e66 = gen__naga_trig_extract_product(_e47, ((_e16 - 176u) + (_e60 * 16u)));
        fraction[_e60] = _e66;
        continuing {
            let _e68 = fraction_index;
            fraction_index = (_e68 + 1u);
        }
    }
    let _e73 = fraction[10u];
    let _e75 = (_e73 >= 32768u);
    if _e75 {
        let _e89 = quadrant;
        quadrant = ((_e89 + 1u) & 3u);
        let _e94 = fraction;
        let _e96 = gen__naga_trig_combine(array<u32, 12>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 1u), _e94, 1u);
        fraction = _e96;
    }
    let _e97 = fraction;
    let _e111 = gen__naga_trig_multiply(_e97, array<u32, 12>(5327u, 39953u, 20996u, 14754u, 440u, 50455u, 35212u, 33897u, 17105u, 46404u, 37407u, 1u));
    let _e112 = quadrant;
    let _e115 = ((_e112 & 1u) == cosine);
    let _e119 = gen__naga_trig_kernel(_e111, select(0u, 1u, _e115));
    let _e120 = gen__naga_trig_round(_e119);
    return (_e120 | ((select(0u, 2147483648u, (_e75 && _e115)) ^ ((select(_e112, (_e112 + 1u), _e8) & 2u) << 30u)) ^ select((input_bits_1 & 2147483648u), 0u, _e8)));
}

fn generated(value_15: f32) -> f32 {
    var value_16: f32;

    value_16 = value_15;
    let _e4 = gen__naga_sqrtf(bitcast<u32>(4f));
    let _e6 = value_16;
    let _e9 = gen__naga_trigonometry(bitcast<u32>(_e6), 0u);
    let _e12 = value_16;
    let _e15 = gen__naga_trigonometry(bitcast<u32>(_e12), 1u);
    return ((bitcast<f32>(_e4) + bitcast<f32>(_e9)) + bitcast<f32>(_e15));
}

fn control(count: i32) -> i32 {
    var count_1: i32;
    var result_4: i32;
    var index: i32;

    count_1 = count;
    result_4 = 0i;
    {
        index = 0i;
        loop {
            let _e6 = index;
            let _e7 = count_1;
            if (_e6 < _e7) {
            } else {
                break;
            }
            let _e13 = index;
            if (_e13 == 1i) {
                continue;
            }
            let _e15 = index;
            switch _e15 {
                case 0: {
                    let _e16 = index;
                    let _e17 = route_1(_e16);
                    let _e18 = result_4;
                    result_4 = (_e18 + _e17);
                    break;
                }
                default: {
                    let _e20 = index;
                    let _e21 = forward(_e20);
                    let _e22 = index;
                    let _e23 = earlier(_e22);
                    let _e25 = result_4;
                    result_4 = (_e25 + (_e21 - _e23));
                    break;
                }
            }
            continuing {
                let _e9 = index;
                index = (_e9 + 1i);
            }
        }
    }
    let _e27 = result_4;
    return _e27;
}

fn late_index() -> u32 {
    let _e1 = blockIdx_1;
    let _e6 = threadIdx_1;
    return ((_e1.x * 2u) + _e6.x);
}

@compute @workgroup_size(2, 1, 1)
fn prototypes(@builtin(local_invocation_id) threadIdx: vec3<u32>, @builtin(workgroup_id) blockIdx: vec3<u32>) {
    var lane: u32;
    var base: u32;
    var counter: i32;
    var first: Packet_cuda_record_0_;
    var second: Packet_cuda_record_0_;
    var total: Packet_cuda_record_0_;
    var first_argument: i32;

    threadIdx_1 = threadIdx;
    blockIdx_1 = blockIdx;
    let _e2 = late_index();
    lane = _e2;
    let _e5 = lane;
    base = (_e5 * 16u);
    counter = 1i;
    let _e10 = counter;
    counter = (_e10 + 1i);
    let _e13 = packet(_e10);
    first = _e13;
    let _e15 = first;
    let _e16 = adjust(_e15);
    second = _e16;
    let _e18 = first;
    let _e19 = second;
    let _e20 = operator_u002b(_e18, _e19);
    total = _e20;
    let _e24 = earlier(3i);
    let _e21 = base;
    out[_e21] = u32(_e24);
    let _e32 = route_1(3i);
    let _e27 = base;
    out[(_e27 + u32(1i))] = u32(_e32);
    let _e40 = forward(4i);
    let _e35 = base;
    out[(_e35 + u32(2i))] = u32(_e40);
    let _e48 = first.key;
    let _e43 = base;
    out[(_e43 + u32(3i))] = u32(_e48);
    let _e58 = first.values[1i];
    let _e51 = base;
    out[(_e51 + u32(4i))] = u32(_e58);
    let _e66 = second.key;
    let _e61 = base;
    out[(_e61 + u32(5i))] = u32(_e66);
    let _e76 = second.values[1i];
    let _e69 = base;
    out[(_e69 + u32(6i))] = u32(_e76);
    let _e84 = total.key;
    let _e79 = base;
    out[(_e79 + u32(7i))] = u32(_e84);
    let _e94 = total.values[0i];
    let _e87 = base;
    out[(_e87 + u32(8i))] = u32(_e94);
    let _e104 = total.values[1i];
    let _e97 = base;
    out[(_e97 + u32(9i))] = u32(_e104);
    let _e107 = counter;
    counter = (_e107 + 1i);
    first_argument = _e107;
    let _e115 = first_argument;
    let _e116 = counter;
    counter = (_e116 + 1i);
    let _e119 = pair(_e115, _e116);
    let _e111 = base;
    out[(_e111 + u32(10i))] = u32(_e119);
    let _e121 = counter;
    counter = (_e121 + 1i);
    consume(_e121);
    let _e129 = counter;
    let _e125 = base;
    out[(_e125 + u32(11i))] = u32(_e129);
    let _e137 = generated(0f);
    let _e132 = base;
    out[(_e132 + u32(12i))] = bitcast<u32>(_e137);
    let _e145 = control(3i);
    let _e140 = base;
    out[(_e140 + u32(13i))] = u32(_e145);
    let _e152 = lane;
    let _e148 = base;
    out[(_e148 + u32(14i))] = _e152;
    let _e158 = counter;
    counter = (_e158 + 1i);
    let _e161 = counter;
    let _e162 = forward(_e161);
    let _e154 = base;
    out[(_e154 + u32(15i))] = u32(_e162);
}
