@group(0) @binding(0)
var<storage, read_write> out: array<u32>;
var<workgroup> values: array<u32, 64>;
var<workgroup> broadcast: u32;

@compute @workgroup_size(64, 1, 1)
fn barriers(@builtin(local_invocation_id) threadIdx: vec3<u32>, @builtin(workgroup_id) blockIdx: vec3<u32>) {
    var lane: u32;
    var base: u32;

    lane = threadIdx.x;
    base = (blockIdx.x * 256u);
    let _e16 = lane;
    let _e11 = lane;
    values[_e11] = (((blockIdx.x * 2000u) + _e16) + 7u);
    let _e27 = lane;
    let _e20 = base;
    let _e21 = lane;
    out[(_e20 + _e21)] = ((blockIdx.x * 1000u) + _e27);
    let _e30 = lane;
    if (_e30 == 0u) {
        broadcast = ((blockIdx.x * 100u) + 11u);
    }
    storageBarrier();
    workgroupBarrier();
    let _e44 = lane;
    let _e47 = values[(63u - _e44)];
    let _e38 = base;
    let _e40 = lane;
    out[((_e38 + 64u) + _e40)] = _e47;
    let _e55 = lane;
    let _e59 = base;
    let _e62 = out[(_e59 + ((_e55 + 1u) % 64u))];
    let _e49 = base;
    let _e51 = lane;
    out[((_e49 + 128u) + _e51)] = _e62;
    let _e69 = broadcast;
    let _e64 = base;
    let _e66 = lane;
    out[((_e64 + 192u) + _e66)] = _e69;
}
