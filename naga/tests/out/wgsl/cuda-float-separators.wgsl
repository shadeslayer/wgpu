struct type_3 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<f32>;
@group(0) @binding(1)
var<storage> input: array<f32>;
@group(0) @binding(2)
var<uniform> n_1: type_3;

@compute @workgroup_size(64, 1, 1)
fn float_separators(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var base: u32;

    let _e4 = n_1.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    let _e14 = i;
    let _e15 = n;
    if (_e14 < _e15) {
        let _e19 = i;
        base = (_e19 * 12u);
        let _e21 = base;
        out[_e21] = 1024.5f;
        let _e25 = base;
        out[(_e25 + 1u)] = 0.125f;
        let _e30 = base;
        out[(_e30 + 2u)] = 12f;
        let _e35 = base;
        out[(_e35 + 3u)] = 25f;
        let _e40 = base;
        out[(_e40 + 4u)] = 0.5f;
        let _e45 = base;
        out[(_e45 + 5u)] = 6.25f;
        let _e50 = base;
        out[(_e50 + 6u)] = -(1024.5f);
        let _e56 = base;
        out[(_e56 + 7u)] = 0.125f;
        let _e61 = base;
        out[(_e61 + 8u)] = 3.1415927f;
        let _e66 = base;
        out[(_e66 + 9u)] = 1f;
        let _e71 = base;
        out[(_e71 + 10u)] = 1.0000002f;
        let _e79 = i;
        let _e82 = input[_e79];
        let _e76 = base;
        out[(_e76 + 11u)] = ((_e82 * 0.25f) + 10f);
    }
}
