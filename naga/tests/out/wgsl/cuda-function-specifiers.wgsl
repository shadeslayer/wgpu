struct type_5 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<storage> input: array<i32>;
@group(0) @binding(2)
var<uniform> n_1: type_5;

fn increment(value_1: i32) -> i32 {
    var value_2: i32;

    value_2 = value_1;
    let _e3 = value_2;
    value_2 = (_e3 + 2i);
    let _e5 = value_2;
    return _e5;
}

fn twice(value_3: i32) -> i32 {
    var value_4: i32;

    value_4 = value_3;
    let _e2 = value_4;
    let _e3 = increment(_e2);
    return (_e3 * 2i);
}

fn advance(value_5: i32) -> i32 {
    var value_6: i32;

    value_6 = value_5;
    let _e2 = value_6;
    let _e3 = twice(_e2);
    return (_e3 + 1i);
}

fn retreat(value_7: i32) -> i32 {
    var value_8: i32;

    value_8 = value_7;
    let _e3 = value_8;
    return (_e3 - 3i);
}

fn scaled(value_9: f32) -> f32 {
    var value_10: f32;

    value_10 = value_9;
    let _e3 = value_10;
    return ((_e3 * 1.5f) + 0.5f);
}

fn differs(value_11: i32, other: i32) -> bool {
    var value_12: i32;
    var other_1: i32;

    value_12 = value_11;
    other_1 = other;
    let _e4 = value_12;
    let _e5 = other_1;
    return (_e4 != _e5);
}

fn consume(stop: bool, value_13: i32) {
    var stop_1: bool;
    var value_14: i32;

    stop_1 = stop;
    value_14 = value_13;
    let _e4 = stop_1;
    if _e4 {
        return;
    }
    let _e5 = value_14;
    let _e6 = advance(_e5);
    value_14 = _e6;
    return;
}

fn forward(stop_2: bool, value_15: i32) {
    var stop_3: bool;
    var value_16: i32;

    stop_3 = stop_2;
    value_16 = value_15;
    let _e4 = stop_3;
    let _e5 = value_16;
    let _e6 = retreat(_e5);
    consume(_e4, _e6);
    return;
}

fn plus_one(value_17: u32) -> u32 {
    var value_18: u32;

    value_18 = value_17;
    let _e3 = value_18;
    return (_e3 + 1u);
}

@compute @workgroup_size(64, 1, 1)
fn function_specifiers(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;
    var value: i32;
    var choose: bool;
    var effect: i32;

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
        let _e33 = value;
        let _e34 = increment(_e33);
        let _e31 = base;
        out[_e31] = _e34;
        let _e39 = value;
        let _e40 = twice(_e39);
        let _e36 = base;
        out[(_e36 + 1u)] = _e40;
        let _e45 = value;
        let _e46 = advance(_e45);
        let _e42 = base;
        out[(_e42 + 2u)] = _e46;
        let _e51 = value;
        let _e52 = retreat(_e51);
        let _e48 = base;
        out[(_e48 + 3u)] = _e52;
        let _e57 = value;
        let _e59 = scaled(f32(_e57));
        let _e54 = base;
        out[(_e54 + 4u)] = i32(_e59);
        let _e65 = value;
        let _e66 = i;
        let _e68 = input[_e66];
        let _e69 = differs(_e65, _e68);
        let _e62 = base;
        out[(_e62 + 5u)] = i32(_e69);
        let _e72 = value;
        effect = _e72;
        let _e73 = choose;
        let _e74 = effect;
        effect = (_e74 + 1i);
        consume(_e73, _e74);
        let _e81 = effect;
        let _e78 = base;
        out[(_e78 + 6u)] = _e81;
        let _e82 = choose;
        let _e84 = effect;
        effect = (_e84 + 1i);
        forward(!(_e82), _e84);
        let _e91 = effect;
        let _e88 = base;
        out[(_e88 + 7u)] = _e91;
        let _e96 = effect;
        let _e97 = advance(_e96);
        let _e93 = base;
        out[(_e93 + 8u)] = _e97;
        let _e103 = effect;
        let _e106 = plus_one(u32((_e103 + 16i)));
        let _e99 = base;
        out[(_e99 + 9u)] = i32(_e106);
        let _e112 = effect;
        let _e113 = i;
        let _e115 = input[_e113];
        let _e116 = differs(_e112, _e115);
        let _e109 = base;
        out[(_e109 + 10u)] = i32(_e116);
        let _e122 = effect;
        let _e119 = base;
        out[(_e119 + 11u)] = _e122;
    }
}
