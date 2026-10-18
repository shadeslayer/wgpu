struct type_3 {
    value: i32,
}

struct type_4 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<u32>;
@group(0) @binding(1)
var<uniform> value_1: type_3;
@group(0) @binding(2)
var<uniform> amount_1: type_4;

@compute @workgroup_size(1, 1, 1)
fn integer_ops(@builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var value: i32;
    var amount: u32;
    var remainder: i32;
    var bits: u32;
    var local: u32;

    let _e3 = value_1.value;
    value = _e3;
    let _e7 = amount_1.value;
    amount = _e7;
    let _e12 = value;
    remainder = (_e12 % -(3i));
    let _e15 = remainder;
    bits = (u32(_e15) & 15u);
    let _e20 = bits;
    bits = (_e20 | 8u);
    let _e23 = bits;
    bits = (_e23 ^ 1u);
    let _e25 = amount;
    let _e26 = bits;
    local = _e25;
    let _e28 = local;
    bits = (_e26 << _e28);
    let _e31 = bits;
    bits = (_e31 >> 1u);
    let _e34 = bits;
    bits = (_e34 % 97u);
    let _e37 = bits;
    bits = (_e37 & 255u);
    let _e42 = bits;
    out[threadIdx.x] = ~(_e42);
}
