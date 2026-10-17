struct type_5 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<uniform> n_1: type_5;

fn affine(value_1: f32, scale: f32, offset: i32) -> f32 {
    var value_2: f32;
    var scale_1: f32;
    var offset_1: i32;

    value_2 = value_1;
    scale_1 = scale;
    offset_1 = offset;
    let _e6 = scale_1;
    let _e7 = value_2;
    value_2 = (_e7 * _e6);
    let _e9 = value_2;
    let _e10 = offset_1;
    return (_e9 + f32(_e10));
}

fn nonnegative(value_3: i32) -> bool {
    var value_4: i32;

    value_4 = value_3;
    let _e3 = value_4;
    return (_e3 >= 0i);
}

fn sum_steps(count: u32, include_even: bool) -> u32 {
    var count_1: u32;
    var include_even_1: bool;
    var total: u32;
    var i_1: u32;
    var local: bool;

    count_1 = count;
    include_even_1 = include_even;
    total = 0u;
    {
        i_1 = 0u;
        loop {
            let _e8 = i_1;
            let _e9 = count_1;
            if (_e8 < _e9) {
            } else {
                break;
            }
            let _e15 = include_even_1;
            local = _e15;
            if _e15 {
            } else {
                let _e18 = i_1;
                local = ((_e18 % 2u) != 0u);
            }
            let _e22 = local;
            if _e22 {
                let _e23 = i_1;
                let _e24 = total;
                total = (_e24 + _e23);
            }
            continuing {
                let _e11 = i_1;
                i_1 = (_e11 + u32(1i));
            }
        }
    }
    let _e26 = total;
    return _e26;
}

fn choose(value_5: i32) -> i32 {
    var value_6: i32;

    value_6 = value_5;
    let _e2 = value_6;
    let _e3 = nonnegative(_e2);
    if _e3 {
        let _e4 = value_6;
        let _e8 = affine(f32(_e4), 2f, 1i);
        return i32(_e8);
    }
    let _e10 = value_6;
    return -(_e10);
}

fn pair(a: i32, b: i32) -> i32 {
    var a_1: i32;
    var b_1: i32;

    a_1 = a;
    b_1 = b;
    let _e5 = a_1;
    let _e7 = b_1;
    return ((_e5 * 10i) + _e7);
}

@compute @workgroup_size(64, 1, 1)
fn device_functions(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;
    var value: i32;
    var argument: i32;

    let _e3 = n_1.value;
    n = _e3;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    let _e13 = i;
    let _e14 = n;
    if (_e13 < _e14) {
        let _e18 = i;
        base = (_e18 * 10u);
        let _e21 = i;
        value = (i32(_e21) - 35i);
        argument = 3i;
        let _e29 = value;
        let _e30 = choose(_e29);
        let _e27 = base;
        out[_e27] = _e30;
        let _e35 = value;
        let _e39 = affine(f32(_e35), 1.5f, 2i);
        let _e32 = base;
        out[(_e32 + 1u)] = i32(_e39);
        let _e48 = sum_steps(u32(5i), true);
        let _e42 = base;
        out[(_e42 + 2u)] = i32(_e48);
        let _e58 = sum_steps(5u, (0i != 0i));
        let _e51 = base;
        out[(_e51 + 3u)] = i32(_e58);
        let _e64 = value;
        let _e70 = affine(f32(_e64), f32(2u), i32(true));
        let _e61 = base;
        out[(_e61 + 4u)] = i32(_e70);
        let _e76 = value;
        let _e73 = base;
        out[(_e73 + 5u)] = _e76;
        let _e81 = value;
        let _e82 = choose(_e81);
        let _e84 = value;
        let _e86 = choose((_e84 + 1i));
        let _e78 = base;
        out[(_e78 + 6u)] = (_e82 + _e86);
        let _e92 = value;
        let _e93 = nonnegative(_e92);
        let _e89 = base;
        out[(_e89 + 7u)] = i32(_e93);
        let _e99 = argument;
        let _e100 = argument;
        argument = (_e100 + 1i);
        let _e103 = pair(_e99, _e100);
        let _e96 = base;
        out[(_e96 + 8u)] = _e103;
        let _e108 = argument;
        let _e105 = base;
        out[(_e105 + 9u)] = _e108;
    }
}
