struct type_3 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<storage> input_a: array<i32>;
@group(0) @binding(2)
var<storage> input_b: array<i32>;
@group(0) @binding(3)
var<uniform> n_1: type_3;
var<workgroup> shared_values: array<i32, 64>;

fn shifted(value_1: i32) -> i32 {
    var value_2: i32;

    value_2 = value_1;
    let _e3 = value_2;
    return (_e3 + 10i);
}

fn discard_(value_3: i32) {
    var value_4: i32;

    value_4 = value_3;
    let _e3 = value_4;
    value_4 = (_e3 + 1i);
    return;
}

fn pair(x: i32, y: i32) -> i32 {
    var x_1: i32;
    var y_1: i32;

    x_1 = x;
    y_1 = y;
    let _e5 = x_1;
    let _e7 = y_1;
    return ((_e5 * 10i) + _e7);
}

@compute @workgroup_size(64, 1, 1)
fn conditionals(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;
    var choose_a: bool;
    var local: i32;
    var a: i32;
    var b: i32;
    var local_1: i32;
    var index_a: i32;
    var index_b: i32;
    var local_2: i32;
    var local_3: i32;
    var c: i32;
    var local_4: i32;
    var condition: i32;
    var lane: u32;
    var local_value: i32;
    var local_5: f32;
    var local_6: u32;
    var local_7: bool;
    var local_8: i32;
    var local_9: u32;
    var shift_value: i32;
    var local_10: i32;
    var local_11: u32;
    var value: i32;
    var other: i32;
    var local_12: i32;
    var count: i32;
    var left: i32;
    var right: i32;

    let _e5 = n_1.value;
    n = _e5;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    let _e16 = i;
    let _e17 = n;
    if (_e16 < _e17) {
        let _e21 = i;
        base = (_e21 * 25u);
        let _e25 = i;
        choose_a = ((_e25 % 2u) == 0u);
        let _e31 = choose_a;
        if _e31 {
            let _e32 = i;
            let _e37 = input_a[_e32];
            local = _e37;
        } else {
            let _e34 = i;
            let _e38 = input_b[_e34];
            local = _e38;
        }
        let _e39 = local;
        let _e29 = base;
        out[_e29] = _e39;
        a = 3i;
        b = 5i;
        let _e48 = choose_a;
        if _e48 {
            let _e49 = a;
            a = (_e49 + 1i);
            let _e52 = shifted(_e49);
            local_1 = _e52;
        } else {
            let _e53 = b;
            b = (_e53 + 1i);
            let _e56 = shifted(_e53);
            local_1 = _e56;
        }
        let _e58 = local_1;
        let _e45 = base;
        out[(_e45 + 1u)] = _e58;
        let _e64 = a;
        let _e66 = b;
        let _e60 = base;
        out[(_e60 + 2u)] = ((_e64 * 10i) + _e66);
        let _e68 = choose_a;
        if _e68 {
            let _e69 = a;
            a = (_e69 + 1i);
            discard_(_e69);
        } else {
            let _e72 = b;
            b = (_e72 + 1i);
            discard_(_e72);
        }
        let _e80 = a;
        let _e82 = b;
        let _e76 = base;
        out[(_e76 + 3u)] = ((_e80 * 10i) + _e82);
        index_a = 0i;
        index_b = 0i;
        let _e88 = choose_a;
        if _e88 {
            let _e89 = index_a;
            index_a = (_e89 + 1i);
        } else {
            let _e92 = index_b;
            index_b = (_e92 + 1i);
        }
        let _e100 = index_a;
        let _e102 = index_b;
        let _e96 = base;
        out[(_e96 + 4u)] = ((_e100 * 10i) + _e102);
        a = 3i;
        b = 5i;
        let _e106 = choose_a;
        if _e106 {
            a = 7i;
        } else {
            b = 7i;
        }
        let _e113 = a;
        let _e115 = b;
        let _e109 = base;
        out[(_e109 + 5u)] = ((_e113 * 10i) + _e115);
        let _e117 = choose_a;
        if _e117 {
            let _e119 = a;
            a = (_e119 + 2i);
        } else {
            let _e121 = b;
            b = (_e121 + 2i);
        }
        let _e128 = a;
        let _e130 = b;
        let _e124 = base;
        out[(_e124 + 6u)] = ((_e128 * 10i) + _e130);
        let _e136 = choose_a;
        if _e136 {
            let _e137 = a;
            a = (_e137 + 1i);
            let _e144 = a;
            local_2 = _e144;
        } else {
            let _e140 = b;
            b = (_e140 + 1i);
            let _e145 = b;
            local_2 = _e145;
        }
        let _e146 = local_2;
        let _e133 = base;
        out[(_e133 + 7u)] = _e146;
        let _e151 = choose_a;
        if _e151 {
            let _e153 = a;
            a = (_e153 + 1i);
            local_3 = _e153;
        } else {
            let _e156 = b;
            b = (_e156 + 1i);
            local_3 = _e156;
        }
        let _e159 = local_3;
        let _e148 = base;
        out[(_e148 + 8u)] = _e159;
        let _e165 = a;
        let _e167 = b;
        let _e161 = base;
        out[(_e161 + 9u)] = ((_e165 * 100i) + _e167);
        let _e170 = choose_a;
        c = i32(_e170);
        b = 7i;
        let _e177 = c;
        if (_e177 != 0i) {
            c = 0i;
            let _e182 = c;
            c = (_e182 + 1i);
            local_4 = _e182;
        } else {
            b = 0i;
            let _e185 = b;
            b = (_e185 + 1i);
            local_4 = _e185;
        }
        let _e188 = local_4;
        let _e174 = base;
        out[(_e174 + 10u)] = _e188;
        let _e194 = c;
        let _e196 = b;
        let _e190 = base;
        out[(_e190 + 11u)] = ((_e194 * 10i) + _e196);
        condition = 0i;
        a = 2i;
        b = 4i;
        condition = 1i;
        let _e206 = condition;
        let _e202 = condition;
        if (_e202 != 0i) {
            a = _e206;
        } else {
            b = _e206;
        }
        let _e212 = a;
        let _e214 = b;
        let _e208 = base;
        out[(_e208 + 12u)] = ((_e212 * 10i) + _e214);
        a = 2i;
        b = 4i;
        c = 6i;
        let _e219 = choose_a;
        if _e219 {
            let _e221 = i;
            if ((_e221 % 4u) == 0u) {
                let _e226 = a;
                a = (_e226 + 3i);
            } else {
                let _e228 = b;
                b = (_e228 + 3i);
            }
        } else {
            let _e230 = c;
            c = (_e230 + 3i);
        }
        let _e237 = a;
        let _e240 = b;
        let _e243 = c;
        let _e233 = base;
        out[(_e233 + 13u)] = (((_e237 * 100i) + (_e240 * 10i)) + _e243);
        lane = threadIdx.x;
        let _e247 = lane;
        shared_values[_e247] = 20i;
        local_value = 10i;
        let _e252 = choose_a;
        if _e252 {
            let _e256 = local_value;
            local_value = (_e256 + 5i);
        } else {
            let _e253 = lane;
            let _e258 = shared_values[_e253];
            shared_values[_e253] = (_e258 + 5i);
        }
        let _e265 = local_value;
        let _e267 = lane;
        let _e269 = shared_values[_e267];
        let _e261 = base;
        out[(_e261 + 14u)] = ((_e265 * 100i) + _e269);
        let _e271 = choose_a;
        if _e271 {
            let _e272 = lane;
            shared_values[_e272] = 30i;
        } else {
            let _e275 = base;
            out[(_e275 + 15u)] = 30i;
        }
        let _e283 = lane;
        let _e285 = shared_values[_e283];
        let _e280 = base;
        out[(_e280 + 16u)] = _e285;
        let _e290 = choose_a;
        if _e290 {
            local_5 = 1.5f;
        } else {
            local_5 = f32(3i);
        }
        let _e295 = local_5;
        let _e287 = base;
        out[(_e287 + 17u)] = i32(_e295);
        let _e301 = choose_a;
        if _e301 {
            local_6 = u32(-(1i));
        } else {
            local_6 = 2u;
        }
        let _e307 = local_6;
        let _e298 = base;
        out[(_e298 + 18u)] = i32(_e307);
        let _e313 = choose_a;
        if _e313 {
            local_7 = true;
        } else {
            local_7 = false;
        }
        let _e317 = local_7;
        let _e310 = base;
        out[(_e310 + 19u)] = i32(_e317);
        let _e323 = choose_a;
        if _e323 {
            local_8 = i32(true);
        } else {
            local_8 = 2i;
        }
        let _e328 = local_8;
        let _e320 = base;
        out[(_e320 + 20u)] = _e328;
        let _e333 = choose_a;
        if _e333 {
            local_9 = 4u;
        } else {
            local_9 = 8u;
        }
        let _e337 = local_9;
        let _e330 = base;
        out[(_e330 + 21u)] = i32(_e337);
        a = 3i;
        shift_value = 2i;
        let _e346 = choose_a;
        if _e346 {
            let _e348 = shift_value;
            local_10 = _e348;
        } else {
            let _e349 = a;
            local_10 = _e349;
        }
        let _e350 = local_10;
        shift_value = 1i;
        let _e352 = shift_value;
        local_11 = u32(_e352);
        let _e355 = local_11;
        let _e343 = base;
        out[(_e343 + 22u)] = (_e350 << _e355);
        value = 3i;
        other = 7i;
        let _e365 = choose_a;
        if _e365 {
            let _e367 = value;
            local_12 = _e367;
        } else {
            let _e368 = other;
            local_12 = _e368;
        }
        let _e369 = local_12;
        let _e370 = value;
        value = (_e370 + 1i);
        let _e373 = pair(_e369, _e370);
        let _e362 = base;
        out[(_e362 + 23u)] = _e373;
        count = 0i;
        left = 0i;
        right = 0i;
        {
            loop {
                let _e381 = count;
                if (_e381 < 3i) {
                } else {
                    break;
                }
                let _e390 = count;
                count = (_e390 + 1i);
                let _e394 = count;
                if (_e394 == 2i) {
                    continue;
                }
                continuing {
                    let _e383 = choose_a;
                    if _e383 {
                        let _e384 = left;
                        left = (_e384 + 1i);
                    } else {
                        let _e387 = right;
                        right = (_e387 + 1i);
                    }
                }
            }
        }
        let _e401 = left;
        let _e404 = right;
        let _e407 = count;
        let _e397 = base;
        out[(_e397 + 24u)] = (((_e401 * 100i) + (_e404 * 10i)) + _e407);
    }
}
