struct type_3 {
    value: u32,
}

@group(0) @binding(0)
var<storage> input: array<f32>;
@group(0) @binding(1)
var<storage, read_write> output: array<f32>;
@group(0) @binding(2)
var<uniform> n_1: type_3;

@compute @workgroup_size(1, 1, 1)
fn convert(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: i32;
    var offset: f32;

    let _e4 = n_1.value;
    n = _e4;
    i = i32(((blockIdx.x * 1u) + threadIdx.x));
    let _e15 = i;
    let _e17 = n;
    if (u32(_e15) < _e17) {
        let _e20 = i;
        offset = f32(_e20);
        let _e24 = i;
        let _e26 = input[_e24];
        let _e27 = offset;
        let _e22 = i;
        output[_e22] = (_e26 + _e27);
    }
}
