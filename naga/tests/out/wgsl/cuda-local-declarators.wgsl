struct type_3 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<storage> input: array<i32>;
@group(0) @binding(2)
var<uniform> n_1: type_3;

fn cached_type(value_1: i32) -> i32 {
    var value_2: i32;
    var Number: i32;
    var next: i32;
    var result: i32;

    value_2 = value_1;
    let _e3 = value_2;
    Number = _e3;
    let _e6 = Number;
    next = (_e6 + 1i);
    let _e10 = next;
    result = (_e10 * 2i);
    let _e12 = result;
    return _e12;
}

fn pair_add(left: i32, right: i32) -> i32 {
    var left_1: i32;
    var right_1: i32;

    left_1 = left;
    right_1 = right;
    let _e4 = left_1;
    let _e5 = right_1;
    return (_e4 + _e5);
}

@compute @workgroup_size(64, 1, 1)
fn local_declarators(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;
    var value: i32;
    var first: i32;
    var second: i32;
    var third: i32;
    var omitted: i32;
    var initialized: i32;
    var final_: i32;
    var fixed: i32;
    var fixed_next: i32;
    var alias_first: i32;
    var alias_next: i32;
    var positive: bool;
    var same: bool;
    var local: bool;
    var low: f32;
    var high: f32;
    var start: u32;
    var finish: u32;
    var outside: i32;
    var outside_1: i32;
    var derived: i32;
    var total: i32;
    var iteration: i32;
    var seed: i32;
    var term: i32;
    var copied: i32;
    var from_call: i32;
    var from_prior: i32;

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
        let _e26 = value;
        value = (_e26 + 1i);
        first = _e26;
        let _e30 = value;
        value = (_e30 + 1i);
        second = _e30;
        let _e34 = value;
        third = _e34;
        let _e38 = first;
        let _e41 = second;
        let _e44 = third;
        let _e35 = base;
        out[_e35] = (((_e38 * 100i) + (_e41 * 10i)) + _e44);
        let _e50 = value;
        let _e47 = base;
        out[(_e47 + 1u)] = _e50;
        let _e54 = value;
        initialized = (_e54 + 3i);
        let _e58 = initialized;
        omitted = (_e58 + 2i);
        let _e60 = omitted;
        let _e61 = initialized;
        final_ = (_e60 + _e61);
        let _e67 = final_;
        let _e64 = base;
        out[(_e64 + 2u)] = _e67;
        let _e70 = value;
        fixed = (_e70 + 1i);
        let _e74 = fixed;
        fixed_next = (_e74 + 2i);
        let _e80 = fixed_next;
        let _e77 = base;
        out[(_e77 + 3u)] = _e80;
        let _e83 = value;
        alias_first = (_e83 + 4i);
        let _e87 = alias_first;
        alias_next = (_e87 + 1i);
        let _e93 = alias_next;
        let _e90 = base;
        out[(_e90 + 4u)] = _e93;
        let _e96 = value;
        positive = (_e96 > 0i);
        let _e99 = positive;
        local = _e99;
        if _e99 {
            let _e101 = i;
            let _e104 = input[_e101];
            let _e106 = value;
            local = (_e106 == (_e104 + 2i));
        }
        let _e108 = local;
        same = _e108;
        let _e113 = same;
        let _e110 = base;
        out[(_e110 + 5u)] = i32(_e113);
        let _e116 = value;
        low = (f32(_e116) * 0.5f);
        let _e122 = low;
        high = (_e122 + 1.5f);
        let _e129 = high;
        let _e125 = base;
        out[(_e125 + 6u)] = i32((_e129 * 2f));
        let _e134 = i;
        start = (_e134 % 5u);
        let _e138 = start;
        finish = (_e138 + 3u);
        let _e144 = finish;
        let _e141 = base;
        out[(_e141 + 7u)] = i32(_e144);
        let _e150 = value;
        let _e151 = cached_type(_e150);
        let _e147 = base;
        out[(_e147 + 8u)] = _e151;
        let _e153 = value;
        outside = _e153;
        {
            let _e156 = value;
            outside_1 = (_e156 + 5i);
            let _e160 = outside_1;
            derived = (_e160 + 2i);
            let _e166 = derived;
            let _e163 = base;
            out[(_e163 + 9u)] = _e166;
        }
        total = 0i;
        {
            iteration = 0i;
            let _e172 = value;
            seed = _e172;
            loop {
                let _e174 = iteration;
                if (_e174 < 4i) {
                } else {
                    break;
                }
                let _e180 = iteration;
                if (_e180 == 1i) {
                    continue;
                }
                let _e183 = seed;
                let _e184 = iteration;
                term = (_e183 + _e184);
                let _e187 = term;
                copied = _e187;
                let _e188 = copied;
                let _e189 = total;
                total = (_e189 + _e188);
                continuing {
                    let _e176 = iteration;
                    iteration = (_e176 + 1i);
                }
            }
        }
        let _e195 = total;
        let _e192 = base;
        out[(_e192 + 10u)] = _e195;
        let _e197 = value;
        let _e199 = pair_add(_e197, 2i);
        from_call = _e199;
        let _e201 = from_call;
        let _e203 = pair_add(_e201, 3i);
        from_prior = _e203;
        let _e208 = from_prior;
        let _e209 = outside;
        let _e205 = base;
        out[(_e205 + 11u)] = (_e208 + _e209);
    }
}
