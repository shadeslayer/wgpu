@group(0) @binding(0)
var<storage> a: array<f32>;
@group(0) @binding(1)
var<storage> b: array<f32>;
@group(0) @binding(2)
var<storage, read_write> out: array<f32>;

@compute @workgroup_size(1, 1, 1)
fn add(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var i: u32;

    i = ((blockIdx.x * 1u) + threadIdx.x);
    let _e13 = i;
    let _e15 = i;
    let _e18 = b[_e15];
    let _e20 = a[_e13];
    let _e11 = i;
    out[_e11] = (_e20 + (_e18 * 2f));
}
