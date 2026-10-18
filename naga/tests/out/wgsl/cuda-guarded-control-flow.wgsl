struct type_3 {
    value: u32,
}

@group(0) @binding(0)
var<storage> a: array<f32>;
@group(0) @binding(1)
var<storage> b: array<f32>;
@group(0) @binding(2)
var<storage, read_write> out: array<f32>;
@group(0) @binding(3)
var<uniform> n_1: type_3;

@compute @workgroup_size(1, 1, 1)
fn add(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;

    let _e5 = n_1.value;
    n = _e5;
    i = ((blockIdx.x * 1u) + threadIdx.x);
    let _e15 = i;
    let _e16 = n;
    if (_e15 < _e16) {
        let _e20 = i;
        let _e22 = i;
        let _e24 = a[_e20];
        let _e25 = b[_e22];
        let _e18 = i;
        out[_e18] = (_e24 + _e25);
    }
}
