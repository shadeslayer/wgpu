struct type_5 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<u32>;
@group(0) @binding(1)
var<storage> input: array<i32>;
@group(0) @binding(2)
var<uniform> n_1: type_5;
var<workgroup> shared_values: array<u32, 64>;

fn transform(value: f32, multiplier: f32, offset: i32) -> i32 {
    var value_1: f32;
    var multiplier_1: f32;
    var offset_1: i32;

    value_1 = value;
    multiplier_1 = multiplier;
    offset_1 = offset;
    let _e6 = value_1;
    let _e7 = multiplier_1;
    let _e9 = offset_1;
    return i32(((_e6 * _e7) + f32(_e9)));
}

@compute @workgroup_size(64, 1, 1)
fn scalar_aliases(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;
    var lane: u32;
    var signed_count: i32;
    var fraction: f32;
    var after_block: i32;
    var coefficient: f32;
    var selected: bool;
    var total: i32;
    var current: i32;
    var after_loop: f32;

    let _e4 = n_1.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    let _e15 = i;
    let _e16 = n;
    if (_e15 < _e16) {
        let _e20 = i;
        base = (_e20 * 10u);
        lane = threadIdx.x;
        let _e27 = i;
        let _e24 = lane;
        shared_values[_e24] = (_e27 + 10u);
        let _e31 = lane;
        let _e33 = shared_values[_e31];
        let _e29 = base;
        out[_e29] = _e33;
        let _e38 = i;
        let _e40 = input[_e38];
        let _e44 = transform(f32(_e40), 2f, 1i);
        let _e35 = base;
        out[(_e35 + 1u)] = u32(_e44);
        signed_count = -(3i);
        let _e53 = signed_count;
        let _e50 = base;
        out[(_e50 + 2u)] = u32(_e53);
        {
            fraction = 3.75f;
            let _e61 = fraction;
            let _e58 = base;
            out[(_e58 + 3u)] = u32(_e61);
        }
        after_block = -(5i);
        let _e70 = after_block;
        let _e67 = base;
        out[(_e67 + 4u)] = u32(_e70);
        coefficient = 1.25f;
        let _e79 = coefficient;
        let _e75 = base;
        out[(_e75 + 5u)] = u32((_e79 * 4f));
        let _e84 = i;
        selected = ((_e84 % 2u) == 0u);
        let _e92 = selected;
        let _e89 = base;
        out[(_e89 + 6u)] = u32(_e92);
        total = 0i;
        {
            loop {
                let _e97 = total;
                if (_e97 < 3i) {
                } else {
                    break;
                }
                let _e100 = total;
                current = _e100;
                let _e102 = current;
                total = (_e102 + 1i);
            }
        }
        let _e108 = total;
        let _e105 = base;
        out[(_e105 + 7u)] = u32(_e108);
        after_loop = 1.5f;
        let _e117 = after_loop;
        let _e113 = base;
        out[(_e113 + 8u)] = u32((_e117 * 2f));
        let _e124 = n;
        let _e121 = base;
        out[(_e121 + 9u)] = u32(f32(_e124));
    }
}
