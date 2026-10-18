struct type_5 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<storage> input: array<i32>;
@group(0) @binding(2)
var<uniform> n_1: type_5;

fn select_value(value_1: i32, choose_first: bool) -> i32 {
    var value_2: i32;
    var choose_first_1: bool;
    var result: i32;

    value_2 = value_1;
    choose_first_1 = choose_first;
    let _e5 = choose_first_1;
    if _e5 {
        let _e7 = value_2;
        result = (_e7 + 3i);
    } else {
        let _e10 = value_2;
        result = (_e10 - 2i);
    }
    let _e12 = result;
    return _e12;
}

fn scaled(value_3: f32) -> f32 {
    var value_4: f32;
    var result_1: f32;

    value_4 = value_3;
    let _e4 = value_4;
    result_1 = ((_e4 * 2f) + 4f);
    let _e8 = result_1;
    return _e8;
}

fn accumulate(limit: u32) -> u32 {
    var limit_1: u32;
    var sum: u32;
    var i_1: u32;
    var term: u32;

    limit_1 = limit;
    sum = 0u;
    {
        i_1 = 0u;
        loop {
            let _e6 = i_1;
            let _e7 = limit_1;
            if (_e6 < _e7) {
            } else {
                break;
            }
            let _e15 = i_1;
            if ((_e15 % 2u) == 0u) {
                let _e20 = i_1;
                term = (_e20 + 1u);
            } else {
                let _e23 = i_1;
                term = (_e23 * 2u);
            }
            let _e25 = term;
            let _e26 = sum;
            sum = (_e26 + _e25);
            continuing {
                let _e9 = i_1;
                i_1 = (_e9 + u32(1i));
            }
        }
    }
    let _e28 = sum;
    return _e28;
}

@compute @workgroup_size(64, 1, 1)
fn uninitialized(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;
    var value: i32;
    var chosen: i32;
    var weighted: f32;
    var negative: bool;
    var magnitude: u32;
    var total: i32;
    var iteration: i32;
    var copied: i32;
    var descending: i32;
    var iteration_1: i32;
    var staged: i32;
    var remaining: i32;
    var partial: i32;
    var next: i32;
    var shadow: i32;
    var shadow_1: i32;

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
        if ((_e27 % 2u) == 0u) {
            let _e32 = value;
            chosen = (_e32 + 7i);
        } else {
            let _e35 = value;
            chosen = (_e35 - 4i);
        }
        let _e39 = chosen;
        let _e37 = base;
        out[_e37] = _e39;
        let _e44 = value;
        let _e46 = i;
        let _e50 = select_value(_e44, ((_e46 % 3u) == 0u));
        let _e41 = base;
        out[(_e41 + 1u)] = _e50;
        let _e52 = value;
        weighted = ((f32(_e52) * 1.5f) + 2f);
        let _e62 = weighted;
        let _e59 = base;
        out[(_e59 + 2u)] = i32(_e62);
        let _e66 = value;
        negative = (_e66 < 0i);
        let _e72 = negative;
        let _e69 = base;
        out[(_e69 + 3u)] = i32(_e72);
        let _e76 = value;
        magnitude = u32((_e76 + 5i));
        let _e84 = magnitude;
        let _e80 = base;
        out[(_e80 + 4u)] = i32((_e84 * 3u));
        let _e92 = i;
        let _e94 = accumulate((_e92 % 5u));
        let _e88 = base;
        out[(_e88 + 5u)] = i32(_e94);
        total = 0i;
        {
            iteration = 0i;
            loop {
                let _e101 = iteration;
                if (_e101 < 3i) {
                } else {
                    break;
                }
                let _e107 = value;
                let _e108 = iteration;
                copied = (_e107 + _e108);
                let _e110 = copied;
                let _e111 = total;
                total = (_e111 + _e110);
                continuing {
                    let _e103 = iteration;
                    iteration = (_e103 + 1i);
                }
            }
        }
        let _e117 = total;
        let _e114 = base;
        out[(_e114 + 6u)] = _e117;
        descending = 0i;
        iteration_1 = 0i;
        {
            loop {
                let _e124 = iteration_1;
                if (_e124 < 3i) {
                } else {
                    break;
                }
                let _e129 = value;
                let _e130 = iteration_1;
                staged = (_e129 - _e130);
                let _e132 = staged;
                let _e133 = descending;
                descending = (_e133 + _e132);
                continuing {
                    let _e126 = iteration_1;
                    iteration_1 = (_e126 + 1i);
                }
            }
        }
        let _e139 = descending;
        let _e136 = base;
        out[(_e136 + 7u)] = _e139;
        remaining = 2i;
        let _e143 = value;
        partial = _e143;
        loop {
            let _e145 = remaining;
            if (_e145 > 0i) {
            } else {
                break;
            }
            {
                let _e148 = partial;
                let _e149 = remaining;
                next = (_e148 + _e149);
                let _e151 = next;
                partial = _e151;
                let _e152 = remaining;
                remaining = (_e152 - 1i);
            }
        }
        let _e159 = partial;
        let _e156 = base;
        out[(_e156 + 8u)] = _e159;
        let _e161 = value;
        shadow = _e161;
        {
            let _e164 = value;
            shadow_1 = (_e164 + 10i);
            let _e170 = shadow_1;
            let _e167 = base;
            out[(_e167 + 9u)] = _e170;
        }
        let _e175 = shadow;
        let _e172 = base;
        out[(_e172 + 10u)] = _e175;
        let _e180 = value;
        let _e182 = scaled(f32(_e180));
        let _e177 = base;
        out[(_e177 + 11u)] = i32(_e182);
    }
}
