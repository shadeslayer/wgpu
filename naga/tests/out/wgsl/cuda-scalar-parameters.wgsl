struct type_1 {
    value: f32,
}

@group(0) @binding(0)
var<uniform> factor_1: type_1;
@group(0) @binding(1)
var<storage> input: array<f32>;
@group(0) @binding(2)
var<storage, read_write> output: array<f32>;

@compute @workgroup_size(1, 1, 1)
fn scale(@builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var factor: f32;

    let _e2 = factor_1.value;
    factor = _e2;
    let _e7 = factor;
    factor = (_e7 * 2f);
    let _e14 = input[threadIdx.x];
    let _e15 = factor;
    output[threadIdx.x] = (_e14 * _e15);
}
