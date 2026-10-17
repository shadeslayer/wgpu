@group(0) @binding(0)
var<storage, read_write> out: array<u32>;
var<workgroup> values: array<u32, 64>;
var<workgroup> signed_value: i32;
var<workgroup> float_value: f32;

@compute @workgroup_size(64, 1, 1)
fn shared_storage(@builtin(local_invocation_id) threadIdx: vec3<u32>, @builtin(workgroup_id) blockIdx: vec3<u32>) {
    var lane: u32;
    var base: u32;

    lane = threadIdx.x;
    base = (blockIdx.x * 128u);
    let _e17 = lane;
    let _e12 = lane;
    values[_e12] = (((blockIdx.x * 1000u) + _e17) + 10u);
    let _e25 = lane;
    let _e27 = values[_e25];
    let _e21 = base;
    let _e22 = lane;
    out[(_e21 + _e22)] = _e27;
    let _e29 = lane;
    if (_e29 == 0u) {
        signed_value = -(7i);
        float_value = 3.5f;
        let _e39 = signed_value;
        let _e35 = base;
        out[(_e35 + 64u)] = u32((_e39 + 10i));
        let _e47 = float_value;
        let _e43 = base;
        out[(_e43 + 65u)] = u32((_e47 * 2f));
    }
}
