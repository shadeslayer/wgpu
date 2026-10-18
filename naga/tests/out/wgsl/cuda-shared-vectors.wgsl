struct float3_ {
    x: f32,
    y: f32,
    z: f32,
}

@group(0) @binding(0)
var<storage, read_write> output: array<f32>;
var<workgroup> values: array<float3_, 8>;
var<workgroup> broadcast: float3_;

fn shift(value: float3_) -> float3_ {
    var value_1: float3_;

    value_1 = value;
    let _e4 = value_1.x;
    value_1.x = (_e4 + 1f);
    let _e9 = value_1.z;
    let _e10 = value_1.y;
    value_1.y = (_e9 - _e10);
    let _e12 = value_1;
    return _e12;
}

@compute @workgroup_size(8, 1, 1)
fn shared_vectors(@builtin(local_invocation_id) threadIdx: vec3<u32>, @builtin(workgroup_id) blockIdx: vec3<u32>) {
    var lane: u32;
    var group: u32;
    var base: f32;
    var next: u32;
    var copied: float3_;
    var offset: i32;
    var choose_shared: bool;
    var row: u32;

    lane = threadIdx.x;
    group = blockIdx.x;
    let _e11 = group;
    let _e13 = lane;
    base = f32(((_e11 * 16u) + _e13));
    let _e19 = base;
    let _e21 = base;
    let _e26 = base;
    let _e16 = lane;
    values[_e16] = float3_((_e19 + 0.5f), (-(_e21) - 1.25f), (_e26 + 2.5f));
    let _e30 = lane;
    if (_e30 == 0u) {
        let _e32 = group;
        let _e36 = group;
        let _e40 = group;
        broadcast = float3_((f32(_e32) + 0.25f), (f32(_e36) + 0.5f), (f32(_e40) + 0.75f));
    }
    storageBarrier();
    workgroupBarrier();
    let _e47 = lane;
    next = ((_e47 + 1u) % 8u);
    let _e52 = next;
    let _e54 = values[_e52];
    let _e55 = shift(_e54);
    copied = _e55;
    {
        offset = 1i;
        let _e59 = offset;
        let _e60 = copied.x;
        copied.x = (_e60 + f32(_e59));
    }
    storageBarrier();
    workgroupBarrier();
    let _e65 = lane;
    choose_shared = ((_e65 & 1u) != 0u);
    let _e75 = broadcast.x;
    let _e69 = choose_shared;
    if _e69 {
        let _e70 = lane;
        let _e76 = values[_e70].y;
        values[_e70].y = (_e76 + _e75);
    } else {
        let _e78 = copied.y;
        copied.y = (_e78 + _e75);
    }
    let _e84 = copied.x;
    let _e80 = lane;
    values[_e80].z = _e84;
    let _e87 = group;
    let _e89 = lane;
    row = (((_e87 * 8u) + _e89) * 9u);
    let _e96 = copied.x;
    let _e93 = row;
    output[_e93] = _e96;
    let _e102 = copied.y;
    let _e98 = row;
    output[(_e98 + 1u)] = _e102;
    let _e108 = copied.z;
    let _e104 = row;
    output[(_e104 + 2u)] = _e108;
    storageBarrier();
    workgroupBarrier();
    let _e113 = next;
    let _e116 = values[_e113].x;
    let _e110 = row;
    output[(_e110 + 3u)] = _e116;
    let _e121 = next;
    let _e124 = values[_e121].y;
    let _e118 = row;
    output[(_e118 + 4u)] = _e124;
    let _e129 = next;
    let _e132 = values[_e129].z;
    let _e126 = row;
    output[(_e126 + 5u)] = _e132;
    let _e138 = broadcast.x;
    let _e134 = row;
    output[(_e134 + 6u)] = _e138;
    let _e144 = broadcast.y;
    let _e140 = row;
    output[(_e140 + 7u)] = _e144;
    let _e150 = broadcast.z;
    let _e146 = row;
    output[(_e146 + 8u)] = _e150;
}
