struct type_5 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<uniform> n_1: type_5;

fn next_value(value_1: i32) -> i32 {
    var value_2: i32;

    value_2 = value_1;
    let _e3 = value_2;
    return (_e3 + 1i);
}

fn consume(stop: bool, value_3: i32, count: u32, scale: f32) {
    var stop_1: bool;
    var value_4: i32;
    var count_1: u32;
    var scale_1: f32;

    stop_1 = stop;
    value_4 = value_3;
    count_1 = count;
    scale_1 = scale;
    let _e8 = count_1;
    let _e9 = value_4;
    value_4 = i32((u32(_e9) + _e8));
    let _e13 = stop_1;
    if _e13 {
        return;
    }
    let _e14 = scale_1;
    let _e15 = value_4;
    value_4 = i32((f32(_e15) * _e14));
    return;
}

fn forward(stop_2: bool, value_5: i32) {
    var stop_3: bool;
    var value_6: i32;

    stop_3 = stop_2;
    value_6 = value_5;
    let _e4 = stop_3;
    let _e5 = value_6;
    let _e6 = next_value(_e5);
    consume(_e4, _e6, 2u, 0.5f);
    return;
}

@compute @workgroup_size(64, 1, 1)
fn void_functions(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;
    var value: i32;
    var iterations: i32;

    let _e3 = n_1.value;
    n = _e3;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    let _e13 = i;
    let _e14 = n;
    if (_e13 < _e14) {
        let _e18 = i;
        base = (_e18 * 6u);
        value = 2i;
        let _e23 = value;
        value = (_e23 + 1i);
        consume(false, _e23, 1u, 1.5f);
        let _e30 = value;
        let _e28 = base;
        out[_e28] = _e30;
        let _e32 = value;
        value = (_e32 + 1i);
        let _e35 = next_value(_e32);
        consume(true, _e35, 2u, 0.5f);
        let _e42 = value;
        let _e39 = base;
        out[(_e39 + 1u)] = _e42;
        let _e44 = i;
        let _e48 = value;
        value = (_e48 + 1i);
        forward(((_e44 % 2u) == 0u), _e48);
        let _e55 = value;
        let _e52 = base;
        out[(_e52 + 2u)] = _e55;
        iterations = 0i;
        {
            let _e59 = value;
            value = (_e59 + 1i);
            consume((0i != 0i), _e59, u32(0i), f32(1u));
            loop {
                let _e69 = iterations;
                if (_e69 < 3i) {
                } else {
                    break;
                }
                let _e78 = iterations;
                iterations = (_e78 + 1i);
                let _e82 = iterations;
                if (_e82 == 1i) {
                    continue;
                }
                continuing {
                    let _e72 = value;
                    value = (_e72 + 1i);
                    let _e75 = value;
                    consume(false, _e75, 0u, 1f);
                }
            }
        }
        let _e88 = value;
        let _e85 = base;
        out[(_e85 + 3u)] = _e88;
        let _e93 = iterations;
        let _e90 = base;
        out[(_e90 + 4u)] = _e93;
        let _e98 = value;
        let _e99 = next_value(_e98);
        let _e95 = base;
        out[(_e95 + 5u)] = _e99;
    }
}
