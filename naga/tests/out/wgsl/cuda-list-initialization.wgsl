struct int3_ {
    x: i32,
    y: i32,
    z: i32,
}

struct float3_ {
    x: f32,
    y: f32,
    z: f32,
}

const Global: i32 = 2i;
const Scale: f32 = 0.5f;

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;

fn pair(first: i32, second: i32) -> i32 {
    var first_1: i32;
    var second_1: i32;

    first_1 = first;
    second_1 = second;
    let _e5 = first_1;
    let _e7 = second_1;
    return ((_e5 * 10i) + _e7);
}

fn list_sum(seed_1: i32) -> i32 {
    var seed_2: i32;
    var values_1: array<i32, 3>;
    var local_4: u32;

    seed_2 = seed_1;
    let _e4 = seed_2;
    values_1[0] = _e4;
    let _e7 = seed_2;
    values_1[1] = (_e7 + 1i);
    local_4 = 2u;
    loop {
        let _e12 = local_4;
        if (_e12 >= 3u) {
            break;
        }
        values_1[_e12] = i32();
        continuing {
            let _e16 = local_4;
            local_4 = (_e16 + 1u);
        }
    }
    let _e23 = values_1[0i];
    let _e24 = values_1[1i];
    let _e28 = values_1[2i];
    return ((_e23 + _e24) + _e28);
}

fn make_list(seed_3: i32) -> int3_ {
    var seed_4: i32;
    var value: int3_;

    seed_4 = seed_3;
    let _e3 = seed_4;
    value.x = _e3;
    let _e6 = seed_4;
    value.y = (_e6 + 1i);
    let _e10 = seed_4;
    value.z = (_e10 + 2i);
    let _e13 = value;
    return _e13;
}

@compute @workgroup_size(1, 1, 1)
fn list_initialization() {
    var seed: i32;
    var zero: i32;
    var copy_zero: i32;
    var enabled: bool;
    var count: u32;
    var sample: f32;
    var Local: i32;
    var Safe: i32;
    var local: i32;
    var safe_values: array<i32, 3>;
    var local_1: u32;
    var values: array<i32, 3>;
    var changed: array<i32, 3>;
    var point: int3_;
    var changed_point: int3_;
    var nested: array<int3_, 2>;
    var flat: array<vec2<i32>, 2>;
    var original: int3_;
    var copied: int3_;
    var copies: array<int3_, 2>;
    var counter: i32;
    var ordered: array<i32, 3>;
    var called: i32;
    var zero_total: i32;
    var iteration: i32;
    var reset: array<i32, 3>;
    var local_2: u32;
    var total: i32;
    var iteration_1: i32;
    var limit: i32;
    var Number: i32;
    var cached: array<i32, 2>;
    var frozen: array<i32, 3>;
    var local_3: u32;
    var floating: float3_;
    var from_call: int3_;

    let _e4 = out[0i];
    seed = _e4;
    zero = 0i;
    copy_zero = 0i;
    enabled = true;
    count = u32(7i);
    sample = 1.5f;
    Local = 3i;
    Safe = 3i;
    safe_values[0] = 1i;
    local_1 = 1u;
    loop {
        let _e26 = local_1;
        if (_e26 >= 3u) {
            break;
        }
        safe_values[_e26] = i32();
        continuing {
            let _e30 = local_1;
            local_1 = (_e30 + 1u);
        }
    }
    let _e35 = zero;
    out[0i] = _e35;
    let _e38 = copy_zero;
    out[1i] = _e38;
    let _e41 = enabled;
    out[2i] = i32(_e41);
    let _e45 = count;
    out[3i] = i32(_e45);
    let _e50 = sample;
    out[4i] = i32((_e50 * 2f));
    let _e56 = Local;
    out[5i] = (Global + _e56);
    let _e62 = safe_values[2i];
    out[6i] = _e62;
    values[0] = 1i;
    let _e70 = values[0i];
    values[1] = (_e70 + 1i);
    let _e76 = values[1i];
    values[2] = (_e76 + 1i);
    let _e84 = values[0i];
    let _e85 = values[1i];
    let _e89 = values[2i];
    out[7i] = ((_e84 + _e85) + _e89);
    changed[0] = 1i;
    changed[0i] = 7i;
    changed[1] = 2i;
    let _e102 = changed[0i];
    changed[2] = _e102;
    let _e107 = changed[0i];
    out[8i] = _e107;
    let _e112 = changed[2i];
    out[9i] = _e112;
    point.x = 1i;
    let _e118 = point.x;
    point.y = (_e118 + 1i);
    let _e123 = point.y;
    point.z = (_e123 + 1i);
    let _e129 = point.z;
    out[10i] = _e129;
    changed_point.x = 1i;
    changed_point.x = 9i;
    changed_point.y = 2i;
    let _e138 = changed_point.x;
    changed_point.z = _e138;
    let _e143 = changed_point.x;
    out[11i] = _e143;
    let _e147 = changed_point.z;
    out[12i] = _e147;
    nested[0].x = 1i;
    nested[0].y = 2i;
    nested[0].z = i32();
    nested[1].x = 3i;
    nested[1].y = i32();
    nested[1].z = i32();
    let _e171 = nested[0i].z;
    let _e172 = nested[1i].x;
    out[13i] = (_e171 + _e172);
    flat[0].x = 1i;
    flat[0].y = 2i;
    flat[1].x = 3i;
    flat[1].y = i32();
    let _e193 = flat[0i].x;
    let _e194 = flat[0i].y;
    let _e199 = flat[1i].x;
    let _e204 = flat[1i].y;
    out[14i] = (((_e193 + _e194) + _e199) + _e204);
    original.x = 2i;
    original.y = 3i;
    original.z = 4i;
    let _e214 = original;
    copied = _e214;
    let _e217 = original;
    copies[0] = _e217;
    copies[1].x = 5i;
    copies[1].y = 6i;
    copies[1].z = i32();
    original.x = 99i;
    let _e230 = copied.x;
    out[15i] = _e230;
    let _e236 = copies[0i].x;
    out[16i] = _e236;
    let _e242 = copies[1i].z;
    out[17i] = _e242;
    let _e244 = seed;
    counter = _e244;
    let _e247 = counter;
    counter = (_e247 + 1i);
    ordered[0] = _e247;
    let _e251 = counter;
    counter = (_e251 + 1i);
    ordered[1] = _e251;
    let _e255 = counter;
    ordered[2] = _e255;
    let _e261 = ordered[0i];
    let _e266 = ordered[1i];
    let _e271 = ordered[2i];
    out[18i] = (((_e261 * 100i) + (_e266 * 10i)) + _e271);
    let _e275 = counter;
    out[19i] = _e275;
    let _e277 = counter;
    counter = (_e277 + 1i);
    let _e281 = pair(_e277, 4i);
    called = _e281;
    let _e284 = called;
    out[20i] = _e284;
    zero_total = 0i;
    {
        iteration = 0i;
        loop {
            let _e290 = iteration;
            if (_e290 < 3i) {
            } else {
                break;
            }
            let _e297 = iteration;
            reset[0] = _e297;
            local_2 = 1u;
            loop {
                let _e301 = local_2;
                if (_e301 >= 3u) {
                    break;
                }
                reset[_e301] = i32();
                continuing {
                    let _e305 = local_2;
                    local_2 = (_e305 + 1u);
                }
            }
            let _e310 = reset[2i];
            let _e311 = zero_total;
            zero_total = (_e311 + _e310);
            reset[2i] = 9i;
            continuing {
                let _e292 = iteration;
                iteration = (_e292 + 1i);
            }
        }
    }
    let _e318 = zero_total;
    out[21i] = _e318;
    total = 0i;
    {
        iteration_1 = 0i;
        limit = 3i;
        loop {
            let _e325 = iteration_1;
            let _e326 = limit;
            if (_e325 < _e326) {
            } else {
                break;
            }
            let _e331 = iteration_1;
            let _e332 = total;
            total = (_e332 + _e331);
            continuing {
                let _e328 = iteration_1;
                iteration_1 = (_e328 + 1i);
            }
        }
    }
    let _e336 = total;
    out[22i] = _e336;
    Number = 4i;
    let _e341 = Number;
    cached[0] = _e341;
    let _e344 = Number;
    cached[1] = (_e344 + 1i);
    let _e350 = cached[1i];
    out[23i] = _e350;
    frozen[0] = 3i;
    local_3 = 1u;
    loop {
        let _e357 = local_3;
        if (_e357 >= 3u) {
            break;
        }
        frozen[_e357] = i32();
        continuing {
            let _e361 = local_3;
            local_3 = (_e361 + 1u);
        }
    }
    let _e370 = frozen[0i];
    let _e371 = frozen[1i];
    let _e375 = frozen[2i];
    out[24i] = ((_e370 + _e371) + _e375);
    floating.x = 1.5f;
    floating.y = -(0f);
    floating.z = f32();
    let _e388 = floating.y;
    out[25i] = bitcast<i32>(_e388);
    let _e393 = floating.z;
    out[26i] = bitcast<i32>(_e393);
    let _e397 = make_list(6i);
    from_call = _e397;
    let _e402 = from_call.x;
    let _e403 = from_call.z;
    out[27i] = (_e402 + _e403);
    out[28i] = i32((Scale * 10f));
    let _e414 = seed;
    let _e416 = list_sum((_e414 + 3i));
    out[29i] = _e416;
}
