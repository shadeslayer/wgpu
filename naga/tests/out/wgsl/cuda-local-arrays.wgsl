@group(0) @binding(0)
var<storage, read_write> out: array<i32>;

fn local_sum(value: i32) -> i32 {
    var value_1: i32;
    var values: array<i32, 3>;

    value_1 = value;
    values = array<i32, 3>();
    let _e6 = value_1;
    values[0i] = _e6;
    let _e10 = value_1;
    values[1i] = (_e10 + 1i);
    let _e18 = values[0i];
    let _e19 = values[1i];
    values[2i] = (_e18 + _e19);
    let _e23 = values[2i];
    return _e23;
}

fn reset_sum(count: i32) -> i32 {
    var count_1: i32;
    var result: i32;
    var i_1: i32;
    var values_1: array<i32, 2>;

    count_1 = count;
    result = 0i;
    {
        i_1 = 0i;
        loop {
            let _e6 = i_1;
            let _e7 = count_1;
            if (_e6 < _e7) {
            } else {
                break;
            }
            values_1 = array<i32, 2>();
            let _e16 = i_1;
            values_1[1i] = _e16;
            let _e21 = values_1[0i];
            let _e22 = values_1[1i];
            let _e24 = result;
            result = (_e24 + (_e21 + _e22));
            values_1[0i] = 99i;
            continuing {
                let _e9 = i_1;
                i_1 = (_e9 + 1i);
            }
        }
    }
    let _e29 = result;
    return _e29;
}

@compute @workgroup_size(1, 1, 1)
fn local_arrays() {
    var seed: i32;
    var signed_values: array<i32, 3>;
    var words: array<u32, 2>;
    var samples: array<f32, 2>;
    var flags: array<bool, 3>;
    var zeros: array<i32, 2>;
    var zero_flags: array<bool, 2>;
    var local: bool;
    var bank: array<u32, 2>;
    var bank_1: array<f32, 2>;
    var signed_values_1: array<i32, 2>;
    var mixed: array<i32, 2>;
    var next: i32;
    var reset_total: i32;
    var i: i32;
    var reset: array<i32, 2>;
    var total: i32;
    var scratch: array<i32, 2>;
    var j: i32;
    var zero_samples: array<f32, 2>;
    var left: array<i32, 2>;
    var right: array<i32, 2>;
    var temporary: array<i32, 2>;
    var temporary_1: array<i32, 2>;
    var choose: bool;
    var index: i32;
    var rhs: i32;
    var local_1: i32;
    var local_2: i32;

    let _e4 = out[0i];
    seed = _e4;
    let _e8 = seed;
    signed_values[0i] = _e8;
    signed_values[1i] = -(2i);
    signed_values[2i] = 4i;
    let _e22 = signed_values[0i];
    let _e23 = signed_values[1i];
    let _e27 = signed_values[2i];
    out[0i] = ((_e22 + _e23) + _e27);
    words = array<u32, 2>();
    words[0i] = 4294967295u;
    words[1i] = 1u;
    let _e43 = words[0i];
    let _e44 = words[1i];
    out[1i] = i32(((_e43 + _e44) == 0u));
    samples = array<f32, 2>();
    samples[0i] = 1.5f;
    samples[1i] = 2.25f;
    let _e63 = samples[0i];
    let _e64 = samples[1i];
    out[2i] = i32(((_e63 + _e64) * 4f));
    flags = array<bool, 3>();
    let _e74 = seed;
    flags[0i] = (_e74 != 0i);
    flags[2i] = true;
    let _e83 = flags[0i];
    let _e89 = flags[1i];
    let _e96 = flags[2i];
    out[3i] = (((i32(_e83) * 100i) + (i32(_e89) * 10i)) + i32(_e96));
    zeros = array<i32, 2>();
    let _e107 = zeros[0i];
    let _e108 = zeros[1i];
    out[4i] = (_e107 + _e108);
    let _e112 = seed;
    let _e113 = local_sum(_e112);
    out[5i] = _e113;
    let _e117 = reset_sum(3i);
    out[6i] = _e117;
    zero_flags = array<bool, 2>();
    let _e124 = zero_flags[0i];
    local = _e124;
    if _e124 {
    } else {
        let _e128 = zero_flags[1i];
        local = _e128;
    }
    let _e129 = local;
    out[7i] = i32(_e129);
    bank = array<u32, 2>();
    bank[0i] = 3u;
    bank[1i] = 4u;
    {
        bank_1 = array<f32, 2>();
        bank_1[1i] = 6.5f;
        let _e150 = bank_1[0i];
        let _e151 = bank_1[1i];
        out[8i] = i32((_e150 + _e151));
    }
    let _e160 = bank[0i];
    let _e161 = bank[1i];
    out[9i] = i32((_e160 + _e161));
    {
        let _e167 = seed;
        signed_values_1[0i] = _e167;
        let _e170 = seed;
        signed_values_1[1i] = _e170;
        let _e177 = signed_values_1[0i];
        let _e178 = signed_values_1[1i];
        out[10i] = (_e177 + _e178);
    }
    let _e184 = signed_values[0i];
    out[11i] = _e184;
    mixed = array<i32, 2>();
    let _e188 = seed;
    seed = (_e188 + 1i);
    let _e191 = seed;
    next = _e191;
    let _e194 = next;
    mixed[0i] = _e194;
    let _e197 = seed;
    seed = (_e197 + 1i);
    mixed[1i] = _e197;
    let _e205 = mixed[0i];
    let _e209 = mixed[1i];
    out[12i] = ((_e205 * 10i) + _e209);
    let _e213 = seed;
    out[13i] = _e213;
    reset_total = 0i;
    {
        i = 0i;
        loop {
            let _e219 = i;
            if (_e219 < 3i) {
            } else {
                break;
            }
            reset = array<i32, 2>();
            let _e227 = i;
            if (_e227 == 1i) {
                reset[0i] = 90i;
                continue;
            }
            let _e234 = reset[0i];
            let _e235 = i;
            let _e237 = reset_total;
            reset_total = (_e237 + (_e234 + _e235));
            reset[0i] = 99i;
            continuing {
                let _e221 = i;
                i = (_e221 + 1i);
            }
        }
    }
    let _e244 = reset_total;
    out[14i] = _e244;
    total = 0i;
    {
        scratch = array<i32, 2>();
        j = 0i;
        loop {
            let _e252 = j;
            if (_e252 < 3i) {
            } else {
                break;
            }
            let _e258 = j;
            let _e262 = scratch[(_e258 % 2i)];
            scratch[(_e258 % 2i)] = (_e262 + 1i);
            let _e265 = j;
            if (_e265 == 1i) {
                continue;
            }
            let _e270 = scratch[0i];
            let _e274 = scratch[1i];
            let _e276 = total;
            total = (_e276 + ((_e270 * 10i) + _e274));
            continuing {
                let _e254 = j;
                j = (_e254 + 1i);
            }
        }
    }
    let _e280 = total;
    out[15i] = _e280;
    zero_samples = array<f32, 2>();
    let _e289 = zero_samples[0i];
    let _e290 = zero_samples[1i];
    out[16i] = i32((_e289 + _e290));
    left = array<i32, 2>();
    right = array<i32, 2>();
    left[0i] = 5i;
    left[1i] = 7i;
    right[0i] = 11i;
    right[1i] = 13i;
    let _e315 = left[0i];
    let _e316 = right[0i];
    out[17i] = (_e315 + _e316);
    {
        temporary = array<i32, 2>();
        let _e322 = seed;
        temporary[0i] = _e322;
        {
            temporary_1 = array<i32, 2>();
            temporary_1[1i] = 3i;
            let _e334 = temporary_1[0i];
            let _e335 = temporary_1[1i];
            out[18i] = (_e334 + _e335);
        }
        let _e341 = temporary[0i];
        out[19i] = _e341;
    }
    let _e344 = seed;
    choose = ((_e344 & 1i) == 0i);
    index = 0i;
    rhs = 20i;
    let _e363 = rhs;
    rhs = (_e363 + 1i);
    let _e366 = rhs;
    let _e354 = choose;
    if _e354 {
        let _e355 = index;
        index = (_e355 + 1i);
        let _e367 = left[_e355];
        left[_e355] = (_e367 + _e366);
        let _e372 = left[_e355];
        local_1 = _e372;
    } else {
        let _e359 = index;
        index = (_e359 + 1i);
        let _e369 = right[_e359];
        right[_e359] = (_e369 + _e366);
        let _e373 = right[_e359];
        local_1 = _e373;
    }
    let _e374 = local_1;
    out[20i] = _e374;
    let _e377 = index;
    out[21i] = _e377;
    let _e383 = left[0i];
    let _e387 = right[0i];
    out[22i] = ((_e383 * 100i) + _e387);
    let _e391 = choose;
    if _e391 {
        right[1i] = 9i;
        let _e398 = right[1i];
        right[1i] = (_e398 + 1i);
        local_2 = _e398;
    } else {
        left[1i] = 9i;
        let _e401 = left[1i];
        left[1i] = (_e401 + 1i);
        local_2 = _e401;
    }
    let _e404 = local_2;
    out[23i] = _e404;
    let _e410 = left[1i];
    let _e414 = right[1i];
    out[24i] = ((_e410 * 100i) + _e414);
}
