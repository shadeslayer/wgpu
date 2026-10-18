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

fn sum(seed_1: i32) -> i32 {
    var seed_2: i32;
    var values_1: array<i32, 2>;

    seed_2 = seed_1;
    let _e4 = seed_2;
    values_1[0] = _e4;
    let _e7 = seed_2;
    values_1[1] = (_e7 + 1i);
    let _e13 = values_1[0i];
    let _e14 = values_1[1i];
    return (_e13 + _e14);
}

fn triple(seed_3: i32) -> int3_ {
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
fn inferred_arrays() {
    var seed: i32;
    var values: array<i32, 3>;
    var copied: array<i32, 2>;
    var zeroes: array<i32, 2>;
    var flags: array<bool, 3>;
    var counts: array<u32, 2>;
    var samples: array<f32, 2>;
    var self_: array<i32, 3>;
    var changed: array<i32, 3>;
    var points: array<int3_, 2>;
    var flat: array<vec2<i32>, 2>;
    var original: int3_;
    var copies: array<int3_, 3>;
    var Number: array<i32, 2>;
    var next: array<i32, 2>;
    var Count: i32;
    var scratch: array<i32, 3>;
    var pending: array<f32, 2>;
    var pending_vectors: array<float3_, 2>;
    var counter: i32;
    var ordered: array<i32, 3>;
    var called: array<i32, 2>;
    var zero_total: i32;
    var iteration: i32;
    var reset: array<vec2<i32>, 2>;
    var total: i32;
    var iteration_1: i32;
    var frozen: array<i32, 2>;
    var signed_zeroes: array<f32, 2>;
    var from_calls: array<int3_, 2>;

    let _e4 = out[0i];
    seed = _e4;
    let _e7 = seed;
    values[0] = _e7;
    let _e10 = seed;
    values[1] = (_e10 + 1i);
    let _e14 = seed;
    values[2] = (_e14 + 2i);
    copied[0] = 4i;
    copied[1] = 5i;
    zeroes[0] = 0i;
    zeroes[1] = 0i;
    flags[0] = true;
    flags[1] = false;
    flags[2] = true;
    counts[0] = 3u;
    counts[1] = 5u;
    samples[0] = 1.5f;
    samples[1] = 2.5f;
    let _e49 = values[0i];
    let _e50 = values[1i];
    let _e54 = values[2i];
    out[0i] = ((_e49 + _e50) + _e54);
    let _e60 = copied[0i];
    out[1i] = _e60;
    let _e67 = zeroes[0i];
    let _e68 = zeroes[1i];
    out[2i] = (_e67 + _e68);
    let _e74 = flags[0i];
    let _e78 = flags[1i];
    let _e83 = flags[2i];
    out[3i] = ((i32(_e74) + i32(_e78)) + i32(_e83));
    let _e92 = counts[0i];
    let _e93 = counts[1i];
    out[4i] = i32((_e92 + _e93));
    let _e102 = samples[0i];
    let _e103 = samples[1i];
    out[5i] = i32((_e102 + _e103));
    self_[0] = 1i;
    let _e113 = self_[0i];
    self_[1] = (_e113 + 1i);
    let _e119 = self_[1i];
    self_[2] = (_e119 + 1i);
    let _e126 = self_[0i];
    let _e131 = self_[1i];
    let _e136 = self_[2i];
    out[6i] = (((_e126 * 100i) + (_e131 * 10i)) + _e136);
    changed[0] = 1i;
    changed[0i] = 7i;
    changed[1] = 2i;
    let _e149 = changed[0i];
    changed[2] = _e149;
    let _e154 = changed[0i];
    out[7i] = _e154;
    let _e159 = changed[2i];
    out[8i] = _e159;
    points[0].x = 1i;
    let _e168 = points[0i].x;
    points[0].y = (_e168 + 1i);
    let _e175 = points[0i].y;
    points[0].z = (_e175 + 1i);
    let _e181 = points[0i];
    points[1] = _e181;
    let _e187 = points[1i].z;
    out[9i] = _e187;
    flat[0].x = 1i;
    flat[0].y = 2i;
    flat[1].x = 3i;
    flat[1].y = i32();
    let _e204 = flat[1i].y;
    out[10i] = _e204;
    let _e210 = flat[1i].x;
    out[11i] = _e210;
    original.x = 4i;
    original.y = 5i;
    original.z = 6i;
    let _e220 = original;
    copies[0] = _e220;
    let _e222 = original;
    copies[1] = _e222;
    copies[2].x = 7i;
    copies[2].y = 8i;
    copies[2].z = i32();
    original.x = 99i;
    let _e237 = copies[0i].x;
    out[12i] = _e237;
    let _e243 = copies[1i].y;
    out[13i] = _e243;
    let _e249 = copies[2i].z;
    out[14i] = _e249;
    Number[0] = 2i;
    Number[1] = 3i;
    let _e259 = Number[1i];
    next[0] = _e259;
    let _e263 = Number[0i];
    next[1] = _e263;
    let _e268 = next[0i];
    out[15i] = _e268;
    Count = 3i;
    scratch = array<i32, 3>();
    let _e277 = scratch[2i];
    out[16i] = _e277;
    pending[0] = 1f;
    pending[1] = f32(2i);
    let _e290 = pending[0i];
    let _e291 = pending[1i];
    out[17i] = i32((_e290 + _e291));
    pending_vectors[0].x = 1f;
    pending_vectors[0].y = f32(2i);
    pending_vectors[0].z = f32();
    pending_vectors[1].x = 3f;
    pending_vectors[1].y = f32();
    pending_vectors[1].z = f32();
    let _e318 = pending_vectors[0i].x;
    let _e319 = pending_vectors[0i].y;
    let _e324 = pending_vectors[1i].x;
    out[18i] = i32(((_e318 + _e319) + _e324));
    let _e328 = seed;
    counter = _e328;
    let _e331 = counter;
    counter = (_e331 + 1i);
    ordered[0] = _e331;
    let _e335 = counter;
    counter = (_e335 + 1i);
    ordered[1] = _e335;
    let _e339 = counter;
    ordered[2] = _e339;
    let _e345 = ordered[0i];
    let _e350 = ordered[1i];
    let _e355 = ordered[2i];
    out[19i] = (((_e345 * 100i) + (_e350 * 10i)) + _e355);
    let _e359 = counter;
    out[20i] = _e359;
    let _e362 = counter;
    counter = (_e362 + 1i);
    let _e366 = pair(_e362, 4i);
    called[0] = _e366;
    let _e368 = counter;
    called[1] = _e368;
    let _e375 = called[0i];
    let _e376 = called[1i];
    out[21i] = (_e375 + _e376);
    let _e380 = counter;
    out[22i] = _e380;
    zero_total = 0i;
    {
        iteration = 0i;
        loop {
            let _e386 = iteration;
            if (_e386 < 3i) {
            } else {
                break;
            }
            let _e393 = iteration;
            reset[0].x = _e393;
            reset[0].y = i32();
            reset[1] = vec2<i32>();
            let _e405 = reset[0i].y;
            let _e406 = reset[1i].x;
            let _e408 = zero_total;
            zero_total = (_e408 + (_e405 + _e406));
            reset[0i].y = 9i;
            reset[1i].x = 8i;
            continuing {
                let _e388 = iteration;
                iteration = (_e388 + 1i);
            }
        }
    }
    let _e420 = zero_total;
    out[23i] = _e420;
    total = 0i;
    {
        iteration_1 = 0i;
        loop {
            let _e426 = iteration_1;
            if (_e426 < 4i) {
            } else {
                break;
            }
            let _e432 = iteration_1;
            if (_e432 == 1i) {
                continue;
            }
            let _e434 = iteration_1;
            let _e435 = total;
            total = (_e435 + _e434);
            continuing {
                let _e428 = iteration_1;
                iteration_1 = (_e428 + 1i);
            }
        }
    }
    let _e439 = total;
    out[24i] = _e439;
    let _e443 = sum(4i);
    out[25i] = _e443;
    frozen[0] = 2i;
    frozen[1] = 3i;
    let _e455 = frozen[0i];
    let _e456 = frozen[1i];
    out[26i] = (_e455 + _e456);
    signed_zeroes[0] = -(0f);
    signed_zeroes[1] = 0f;
    let _e468 = signed_zeroes[0i];
    out[27i] = bitcast<i32>(_e468);
    let _e474 = signed_zeroes[1i];
    out[28i] = bitcast<i32>(_e474);
    let _e479 = triple(6i);
    from_calls[0] = _e479;
    let _e482 = triple(9i);
    from_calls[1] = _e482;
    let _e491 = from_calls[0i].x;
    let _e492 = from_calls[1i].z;
    out[29i] = (_e491 + _e492);
}
