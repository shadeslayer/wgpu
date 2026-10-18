struct type_5 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<storage> input: array<i32>;
@group(0) @binding(2)
var<uniform> n_1: type_5;

fn classify(value_1: bool) -> bool {
    var value_2: bool;

    value_2 = value_1;
    let _e2 = value_2;
    return !(_e2);
}

fn classify_1(value_3: i32) -> i32 {
    var value_4: i32;

    value_4 = value_3;
    let _e3 = value_4;
    return (_e3 + 10i);
}

fn classify_2(value_5: u32) -> u32 {
    var value_6: u32;

    value_6 = value_5;
    let _e3 = value_6;
    return (_e3 + 20u);
}

fn classify_3(value_7: f32) -> f32 {
    var value_8: f32;

    value_8 = value_7;
    let _e3 = value_8;
    return ((_e3 * 1.5f) + 0.5f);
}

fn rank(left: i32, right: f32) -> i32 {
    var left_1: i32;
    var right_1: f32;

    left_1 = left;
    right_1 = right;
    let _e5 = left_1;
    let _e7 = right_1;
    return ((_e5 * 10i) + i32(_e7));
}

fn rank_1(left_2: f32, right_2: i32) -> i32 {
    var left_3: f32;
    var right_3: i32;

    left_3 = left_2;
    right_3 = right_2;
    let _e5 = left_3;
    let _e10 = right_3;
    return ((100i + (i32(_e5) * 10i)) + _e10);
}

fn pair(left_4: f32, right_4: i32) -> i32 {
    var left_5: f32;
    var right_5: i32;

    left_5 = left_4;
    right_5 = right_4;
    let _e4 = left_5;
    let _e8 = right_5;
    return ((i32(_e4) * 10i) + _e8);
}

fn promote(left_6: u32, right_6: f32) -> i32 {
    var left_7: u32;
    var right_7: f32;

    left_7 = left_6;
    right_7 = right_6;
    let _e5 = left_7;
    let _e8 = right_7;
    return ((200i + i32(_e5)) + i32(_e8));
}

fn touch(stop: bool, value_9: i32) {
    var stop_1: bool;
    var value_10: i32;

    stop_1 = stop;
    value_10 = value_9;
    let _e4 = stop_1;
    if _e4 {
        return;
    }
    let _e5 = value_10;
    let _e6 = classify_1(_e5);
    value_10 = _e6;
    return;
}

fn touch_1(value_11: i32) -> i32 {
    var value_12: i32;

    value_12 = value_11;
    let _e2 = value_12;
    let _e3 = classify_1(_e2);
    return (_e3 + 3i);
}

fn stage(value_13: i32) -> i32 {
    var value_14: i32;

    value_14 = value_13;
    let _e3 = value_14;
    return (_e3 + 2i);
}

fn frozen(value_15: f32) -> i32 {
    var value_16: f32;

    value_16 = value_15;
    let _e2 = value_16;
    let _e4 = stage(i32(_e2));
    return _e4;
}

fn stage_1(value_17: f32) -> i32 {
    var value_18: f32;

    value_18 = value_17;
    let _e2 = value_18;
    return (i32(_e2) + 100i);
}

@compute @workgroup_size(64, 1, 1)
fn scalar_overloads(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;
    var value: i32;
    var choose: bool;
    var effect: i32;
    var nested: i32;
    var argument: i32;
    var captured: i32;

    let _e4 = n_1.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    let _e14 = i;
    let _e15 = n;
    if (_e14 < _e15) {
        let _e19 = i;
        base = (_e19 * 13u);
        let _e22 = i;
        let _e24 = input[_e22];
        value = _e24;
        let _e27 = i;
        choose = ((_e27 % 2u) == 0u);
        let _e33 = choose;
        let _e34 = classify(_e33);
        let _e31 = base;
        out[_e31] = i32(_e34);
        let _e40 = value;
        let _e41 = classify_1(_e40);
        let _e37 = base;
        out[(_e37 + 1u)] = _e41;
        let _e47 = value;
        let _e50 = classify_2(u32((_e47 + 16i)));
        let _e43 = base;
        out[(_e43 + 2u)] = i32(_e50);
        let _e56 = value;
        let _e58 = classify_3(f32(_e56));
        let _e53 = base;
        out[(_e53 + 3u)] = i32(_e58);
        let _e64 = choose;
        let _e65 = value;
        let _e70 = rank(i32(_e64), (f32(_e65) + 0.5f));
        let _e61 = base;
        out[(_e61 + 4u)] = _e70;
        let _e75 = value;
        let _e79 = choose;
        let _e81 = rank_1((f32(_e75) + 0.5f), i32(_e79));
        let _e72 = base;
        out[(_e72 + 5u)] = _e81;
        let _e87 = value;
        let _e90 = value;
        let _e92 = promote(u32((_e87 + 16i)), f32(_e90));
        let _e83 = base;
        out[(_e83 + 6u)] = _e92;
        let _e94 = value;
        effect = _e94;
        let _e95 = choose;
        let _e96 = effect;
        effect = (_e96 + 1i);
        touch(_e95, _e96);
        let _e103 = effect;
        let _e100 = base;
        out[(_e100 + 7u)] = _e103;
        let _e108 = effect;
        let _e109 = touch_1(_e108);
        let _e105 = base;
        out[(_e105 + 8u)] = _e109;
        let _e114 = value;
        let _e118 = frozen((f32(_e114) + 0.75f));
        let _e111 = base;
        out[(_e111 + 9u)] = _e118;
        let _e123 = value;
        let _e127 = stage_1((f32(_e123) + 0.75f));
        let _e120 = base;
        out[(_e120 + 10u)] = _e127;
        let _e129 = effect;
        effect = (_e129 + 1i);
        let _e132 = classify_1(_e129);
        let _e133 = touch_1(_e132);
        nested = _e133;
        let _e139 = nested;
        let _e141 = effect;
        let _e135 = base;
        out[(_e135 + 11u)] = ((_e139 * 10i) + _e141);
        let _e145 = value;
        argument = (_e145 + 8i);
        let _e148 = argument;
        let _e149 = argument;
        argument = (_e149 + 1i);
        let _e153 = pair(f32(_e148), _e149);
        captured = _e153;
        let _e159 = captured;
        let _e161 = argument;
        let _e155 = base;
        out[(_e155 + 12u)] = ((_e159 * 100i) + _e161);
    }
}
