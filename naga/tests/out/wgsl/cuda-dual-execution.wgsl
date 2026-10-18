struct type_5 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<storage> input: array<i32>;
@group(0) @binding(2)
var<uniform> n_1: type_5;

fn step_(value_1: i32) -> i32 {
    var value_2: i32;

    value_2 = value_1;
    let _e3 = value_2;
    value_2 = (_e3 + 3i);
    let _e5 = value_2;
    return _e5;
}

fn doubled(value_3: i32) -> i32 {
    var value_4: i32;

    value_4 = value_3;
    let _e2 = value_4;
    let _e3 = step_(_e2);
    return (_e3 * 2i);
}

fn advance(value_5: u32) -> u32 {
    var value_6: u32;

    value_6 = value_5;
    let _e3 = value_6;
    return (_e3 + 5u);
}

fn retreat(value_7: u32) -> u32 {
    var value_8: u32;

    value_8 = value_7;
    let _e3 = value_8;
    return (_e3 - 2u);
}

fn scale(value_9: f32, bias: i32) -> f32 {
    var value_10: f32;
    var bias_1: i32;

    value_10 = value_9;
    bias_1 = bias;
    let _e5 = value_10;
    let _e7 = bias_1;
    return ((_e5 * 2f) + f32(_e7));
}

fn same(left: i32, right: i32) -> bool {
    var left_1: i32;
    var right_1: i32;

    left_1 = left;
    right_1 = right;
    let _e4 = left_1;
    let _e5 = right_1;
    return (_e4 == _e5);
}

fn consume(stop: bool, value_11: i32) {
    var stop_1: bool;
    var value_12: i32;

    stop_1 = stop;
    value_12 = value_11;
    let _e4 = stop_1;
    if _e4 {
        return;
    }
    let _e5 = value_12;
    let _e6 = doubled(_e5);
    value_12 = _e6;
    return;
}

fn forward(stop_2: bool, value_13: i32) {
    var stop_3: bool;
    var value_14: i32;

    stop_3 = stop_2;
    value_14 = value_13;
    let _e4 = stop_3;
    let _e5 = value_14;
    let _e6 = step_(_e5);
    consume(_e4, _e6);
    return;
}

fn device_only(value_15: i32) -> i32 {
    var value_16: i32;

    value_16 = value_15;
    let _e2 = value_16;
    let _e3 = step_(_e2);
    return (_e3 + 1i);
}

@compute @workgroup_size(64, 1, 1)
fn dual_execution(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;
    var value: i32;

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
        let _e27 = value;
        let _e28 = step_(_e27);
        let _e25 = base;
        out[_e25] = _e28;
        let _e33 = value;
        let _e30 = base;
        out[(_e30 + 1u)] = _e33;
        let _e38 = value;
        let _e39 = doubled(_e38);
        let _e35 = base;
        out[(_e35 + 2u)] = _e39;
        let _e44 = value;
        let _e46 = advance(u32(_e44));
        let _e41 = base;
        out[(_e41 + 3u)] = i32(_e46);
        let _e52 = value;
        let _e54 = retreat(u32(_e52));
        let _e49 = base;
        out[(_e49 + 4u)] = i32(_e54);
        let _e60 = value;
        let _e63 = scale(f32(_e60), 4i);
        let _e57 = base;
        out[(_e57 + 5u)] = i32(_e63);
        let _e69 = value;
        let _e70 = i;
        let _e72 = input[_e70];
        let _e73 = same(_e69, _e72);
        let _e66 = base;
        out[(_e66 + 6u)] = i32(_e73);
        let _e76 = i;
        let _e80 = value;
        value = (_e80 + 1i);
        consume(((_e76 % 2u) == 0u), _e80);
        let _e87 = value;
        let _e84 = base;
        out[(_e84 + 7u)] = _e87;
        let _e89 = i;
        let _e93 = value;
        value = (_e93 + 1i);
        forward(((_e89 % 2u) != 0u), _e93);
        let _e100 = value;
        let _e97 = base;
        out[(_e97 + 8u)] = _e100;
        let _e105 = value;
        let _e106 = i;
        let _e108 = input[_e106];
        let _e109 = same(_e105, _e108);
        let _e102 = base;
        out[(_e102 + 9u)] = i32(_e109);
        let _e115 = value;
        let _e116 = step_(_e115);
        let _e117 = device_only(_e116);
        let _e112 = base;
        out[(_e112 + 10u)] = _e117;
        let _e122 = value;
        let _e119 = base;
        out[(_e119 + 11u)] = _e122;
    }
}
