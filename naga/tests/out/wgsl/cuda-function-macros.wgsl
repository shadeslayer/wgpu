struct Pair_cuda_record_0_ {
    first: u32,
    second: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

fn advance(value: u32) -> u32 {
    var value_1: u32;

    value_1 = value;
    let _e3 = value_1;
    return (_e3 + 3u);
}

@compute @workgroup_size(2, 1, 1)
fn function_macros(@builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var lane: u32;
    var start: u32;
    var pair: Pair_cuda_record_0_;
    var local: Pair_cuda_record_0_;
    var values: array<u32, 3>;
    var vector: vec2<u32>;
    var counter: u32;
    var local_1: u32;
    var declared: u32;

    lane = threadIdx.x;
    let _e5 = lane;
    if (_e5 >= 2u) {
        return;
    }
    let _e9 = lane;
    start = (_e9 * 24u);
    local.first = 11u;
    local.second = 13u;
    let _e17 = local;
    pair = _e17;
    values[0] = 2u;
    values[1] = 3u;
    values[2] = 5u;
    vector = vec2<u32>(17u, 19u);
    counter = 0u;
    let _e31 = start;
    out[_e31] = 7u;
    let _e35 = start;
    out[(_e35 + 1u)] = 11u;
    let _e40 = start;
    out[(_e40 + 2u)] = (2u + 3u);
    let _e47 = start;
    out[(_e47 + 3u)] = ((1u + 2u) * 3u);
    let _e56 = start;
    out[(_e56 + 4u)] = (5u + 6u);
    let _e63 = start;
    out[(_e63 + 5u)] = (7u + 8u);
    let _e74 = pair.first;
    let _e70 = start;
    out[(_e70 + 6u)] = _e74;
    let _e80 = pair.second;
    let _e76 = start;
    out[(_e76 + 7u)] = _e80;
    let _e82 = start;
    out[(_e82 + 8u)] = u32(3i);
    let _e91 = lane;
    let _e93 = values[_e91];
    let _e88 = start;
    out[(_e88 + 9u)] = _e93;
    let _e99 = vector.x;
    let _e95 = start;
    out[(_e95 + 10u)] = _e99;
    let _e105 = vector.y;
    let _e101 = start;
    out[(_e101 + 11u)] = _e105;
    let _e110 = counter;
    counter = (_e110 + u32(1i));
    let _e114 = counter;
    counter = (_e114 + u32(1i));
    let _e107 = start;
    out[(_e107 + 12u)] = _e114;
    let _e122 = counter;
    let _e119 = start;
    out[(_e119 + 13u)] = _e122;
    let _e124 = start;
    out[(_e124 + 14u)] = 19u;
    let _e132 = counter;
    let _e129 = start;
    out[(_e129 + 15u)] = _e132;
    values[0i] = 23u;
    let _e140 = values[0i];
    let _e134 = start;
    out[(_e134 + 16u)] = _e140;
    let _e147 = values[0i];
    let _e142 = start;
    out[(_e142 + 17u)] = _e147;
    let _e153 = lane;
    if (_e153 == 0u) {
        local_1 = 29u;
    } else {
        local_1 = 31u;
    }
    let _e158 = local_1;
    let _e149 = start;
    out[(_e149 + 18u)] = _e158;
    let _e163 = lane;
    let _e164 = advance(_e163);
    let _e160 = start;
    out[(_e160 + 19u)] = _e164;
    declared = 37u;
    let _e171 = declared;
    let _e168 = start;
    out[(_e168 + 20u)] = _e171;
    let _e173 = start;
    out[(_e173 + 21u)] = bitcast<u32>(-(0f));
    let _e180 = start;
    out[(_e180 + 22u)] = 4294967295u;
    let _e189 = lane;
    let _e185 = start;
    out[(_e185 + 23u)] = (41u + _e189);
}
