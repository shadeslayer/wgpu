struct type_2 {
    value: i32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<uniform> value_1: type_2;

@compute @workgroup_size(1, 1, 1)
fn updates(@builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var value: i32;
    var previous: i32;

    let _e3 = value_1.value;
    value = _e3;
    let _e6 = value;
    value = (_e6 + 1i);
    previous = _e6;
    let _e12 = previous;
    let _e13 = out[threadIdx.x];
    out[threadIdx.x] = (_e13 + _e12);
    let _e17 = value;
    value = (_e17 + 1i);
    let _e20 = value;
    out[threadIdx.x] = _e20;
}
