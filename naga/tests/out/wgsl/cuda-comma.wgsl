struct type_3 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<storage> input: array<i32>;
@group(0) @binding(2)
var<uniform> n_1: type_3;

fn consume(value_1: i32) {
    var value_2: i32;

    value_2 = value_1;
    let _e3 = value_2;
    if (_e3 < 0i) {
        return;
    } else {
        return;
    }
}

fn pair_add(left_1: i32, right_1: i32) -> i32 {
    var left_2: i32;
    var right_2: i32;

    left_2 = left_1;
    right_2 = right_1;
    let _e4 = left_2;
    let _e5 = right_2;
    return (_e4 + _e5);
}

@compute @workgroup_size(64, 1, 1)
fn comma(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;
    var value: i32;
    var choose: bool;
    var x: i32;
    var sequenced: i32;
    var target_: i32;
    var rhs: i32;
    var middle: i32;
    var selected: i32;
    var local: i32;
    var left: i32;
    var right: i32;
    var identity: i32;
    var position: u32;
    var effect: i32;
    var total: i32;
    var iteration: i32;
    var current: i32;
    var ticks: i32;
    var passes: i32;
    var argument: i32;
    var first: i32;
    var second: i32;

    let _e4 = n_1.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    let _e14 = i;
    let _e15 = n;
    if (_e14 < _e15) {
        let _e19 = i;
        base = (_e19 * 12u);
        let _e22 = i;
        let _e24 = input[_e22];
        value = _e24;
        let _e27 = i;
        choose = ((_e27 % 2u) == 0u);
        let _e32 = value;
        x = _e32;
        let _e35 = x;
        x = (_e35 + 2i);
        let _e38 = x;
        x = (_e38 * 3i);
        let _e41 = x;
        sequenced = (_e41 - 1i);
        let _e45 = sequenced;
        let _e43 = base;
        out[_e43] = _e45;
        let _e50 = x;
        let _e47 = base;
        out[(_e47 + 1u)] = _e50;
        target_ = 0i;
        let _e53 = value;
        target_ = _e53;
        let _e55 = target_;
        target_ = (_e55 + 4i);
        let _e61 = target_;
        let _e58 = base;
        out[(_e58 + 2u)] = _e61;
        let _e64 = value;
        target_ = (_e64 + 1i);
        let _e67 = target_;
        rhs = (_e67 + 3i);
        let _e74 = rhs;
        let _e76 = target_;
        let _e70 = base;
        out[(_e70 + 3u)] = ((_e74 * 10i) + _e76);
        middle = 0i;
        let _e81 = choose;
        if _e81 {
            let _e83 = value;
            middle = (_e83 + 2i);
            let _e86 = middle;
            local = (_e86 + 3i);
        } else {
            let _e89 = value;
            local = (_e89 - 2i);
        }
        let _e92 = local;
        selected = _e92;
        let _e98 = selected;
        let _e100 = middle;
        let _e94 = base;
        out[(_e94 + 4u)] = ((_e98 * 10i) + _e100);
        let _e103 = value;
        left = _e103;
        let _e106 = value;
        right = (_e106 + 10i);
        let _e108 = value;
        consume(_e108);
        let _e109 = choose;
        if _e109 {
            let _e111 = left;
            left = (_e111 + 3i);
        } else {
            let _e113 = right;
            right = (_e113 + 3i);
        }
        let _e120 = left;
        let _e122 = right;
        let _e116 = base;
        out[(_e116 + 5u)] = ((_e120 * 100i) + _e122);
        let _e125 = value;
        identity = _e125;
        let _e126 = identity;
        identity = (_e126 + 1i);
        consume(_e126);
        let _e129 = identity;
        identity = (_e129 + 1i);
        let _e136 = identity;
        let _e133 = base;
        out[(_e133 + 6u)] = _e136;
        let _e139 = base;
        position = (_e139 + 4u);
        let _e148 = position;
        position = (_e148 + 2u);
        let _e151 = value;
        let _e141 = position;
        position = (_e141 + u32(1i));
        let _e145 = position;
        out[_e145] = ((_e151 * 2i) + 2i);
        let _e156 = value;
        effect = _e156;
        let _e157 = effect;
        effect = (_e157 + 1i);
        consume(_e157);
        let _e160 = effect;
        effect = (_e160 + 1i);
        consume(_e160);
        let _e163 = effect;
        effect = (_e163 + 1i);
        consume(_e163);
        let _e166 = effect;
        effect = (_e166 + 1i);
        consume(_e166);
        let _e173 = effect;
        let _e170 = base;
        out[(_e170 + 8u)] = _e173;
        total = 0i;
        {
            iteration = 0i;
            let _e179 = value;
            current = _e179;
            loop {
                let _e181 = iteration;
                if (_e181 < 4i) {
                } else {
                    break;
                }
                let _e190 = iteration;
                if (_e190 != 1i) {
                    let _e192 = current;
                    let _e193 = total;
                    total = (_e193 + _e192);
                } else {
                    continue;
                }
                continuing {
                    let _e183 = iteration;
                    iteration = (_e183 + 1i);
                    let _e187 = current;
                    current = (_e187 + 2i);
                }
            }
        }
        let _e199 = total;
        let _e196 = base;
        out[(_e196 + 9u)] = _e199;
        let _e201 = value;
        ticks = _e201;
        passes = 0i;
        {
            let _e205 = value;
            ticks = (_e205 + 1i);
            passes = 0i;
            loop {
                let _e209 = passes;
                passes = (_e209 + 1i);
                let _e212 = passes;
                if (_e212 < 4i) {
                } else {
                    break;
                }
                continuing {
                    let _e215 = ticks;
                    ticks = (_e215 + 2i);
                }
            }
        }
        let _e222 = ticks;
        let _e224 = passes;
        let _e218 = base;
        out[(_e218 + 10u)] = ((_e222 * 10i) + _e224);
        let _e227 = value;
        argument = _e227;
        let _e230 = argument;
        argument = (_e230 + 1i);
        let _e232 = argument;
        let _e234 = pair_add(_e232, 2i);
        first = _e234;
        let _e236 = first;
        let _e238 = pair_add(_e236, 3i);
        second = _e238;
        let _e244 = second;
        let _e246 = argument;
        let _e240 = base;
        out[(_e240 + 11u)] = ((_e244 * 10i) + _e246);
    }
}
