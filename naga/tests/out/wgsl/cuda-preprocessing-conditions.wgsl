@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

fn later(value: u32) -> u32 {
    var value_1: u32;

    value_1 = value;
    let _e3 = value_1;
    return (_e3 + 73u);
}

@compute @workgroup_size(2, 1, 1)
fn preprocessing_conditions(@builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var lane: u32;
    var start: u32;
    var counter: u32;

    lane = threadIdx.x;
    let _e5 = lane;
    if (_e5 >= 2u) {
        return;
    }
    let _e9 = lane;
    start = (_e9 * 24u);
    let _e11 = start;
    out[_e11] = 7u;
    let _e15 = start;
    out[(_e15 + 1u)] = 11u;
    let _e20 = start;
    out[(_e20 + 2u)] = 13u;
    let _e25 = start;
    out[(_e25 + 3u)] = 17u;
    let _e30 = start;
    out[(_e30 + 4u)] = 19u;
    let _e35 = start;
    out[(_e35 + 5u)] = 23u;
    let _e40 = start;
    out[(_e40 + 6u)] = 29u;
    let _e45 = start;
    out[(_e45 + 7u)] = 31u;
    let _e50 = start;
    out[(_e50 + 8u)] = 37u;
    let _e55 = start;
    out[(_e55 + 9u)] = 41u;
    let _e60 = start;
    out[(_e60 + 10u)] = 43u;
    let _e65 = start;
    out[(_e65 + 11u)] = 47u;
    let _e70 = start;
    out[(_e70 + 12u)] = 53u;
    let _e75 = start;
    out[(_e75 + 13u)] = 59u;
    let _e80 = start;
    out[(_e80 + 14u)] = 61u;
    let _e85 = start;
    out[(_e85 + 15u)] = 67u;
    let _e90 = start;
    out[(_e90 + 16u)] = 71u;
    let _e98 = lane;
    let _e99 = later(_e98);
    let _e95 = start;
    out[(_e95 + 17u)] = _e99;
    counter = 0u;
    let _e106 = counter;
    counter = (_e106 + u32(1i));
    let _e103 = start;
    out[(_e103 + 18u)] = _e106;
    let _e114 = counter;
    let _e111 = start;
    out[(_e111 + 19u)] = _e114;
    let _e116 = start;
    out[(_e116 + 20u)] = u32(true);
    let _e125 = lane;
    let _e122 = start;
    out[(_e122 + 21u)] = _e125;
    let _e127 = start;
    out[(_e127 + 22u)] = bitcast<u32>(-(0f));
    let _e134 = start;
    out[(_e134 + 23u)] = 4294967295u;
}
