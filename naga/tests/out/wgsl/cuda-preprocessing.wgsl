struct Packet_cuda_record_0_ {
    first: u32,
    second: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

fn later(value: u32) -> u32 {
    var value_1: u32;

    value_1 = value;
    let _e3 = value_1;
    return (_e3 + 5u);
}

@compute @workgroup_size(2, 1, 1)
fn preprocessing(@builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var lane: u32;
    var start: u32;
    var local: u32;
    var original: Packet_cuda_record_0_;
    var copied: Packet_cuda_record_0_;
    var SELF: u32;
    var LEFT: u32;
    var RIGHT: u32;
    var counter: u32;
    var ordered: Packet_cuda_record_0_;
    var values: array<u32, 3>;
    var index: u32;

    lane = threadIdx.x;
    let _e5 = lane;
    if (_e5 >= 2u) {
        return;
    }
    let _e9 = lane;
    start = (_e9 * 24u);
    let _e12 = start;
    out[(_e12 + 0u)] = 7u;
    let _e17 = start;
    out[(_e17 + 1u)] = 11u;
    let _e22 = start;
    out[(_e22 + 2u)] = 22u;
    let _e27 = start;
    out[(_e27 + 3u)] = (2u + (3u * 4u));
    let _e36 = start;
    out[(_e36 + 4u)] = (2u + 3u);
    local = 2u;
    let _e49 = local;
    let _e43 = start;
    out[(_e43 + 5u)] = (3u << _e49);
    let _e55 = lane;
    let _e56 = later(_e55);
    let _e52 = start;
    out[(_e52 + 6u)] = _e56;
    let _e59 = lane;
    original.first = (_e59 + 1u);
    let _e64 = lane;
    original.second = (_e64 + 2u);
    let _e67 = original;
    copied = _e67;
    let _e73 = copied.first;
    let _e69 = start;
    out[(_e69 + 7u)] = _e73;
    let _e79 = copied.second;
    let _e75 = start;
    out[(_e75 + 8u)] = _e79;
    SELF = 17u;
    LEFT = 19u;
    RIGHT = 23u;
    let _e90 = SELF;
    let _e87 = start;
    out[(_e87 + 9u)] = _e90;
    let _e95 = LEFT;
    let _e92 = start;
    out[(_e92 + 10u)] = _e95;
    let _e100 = RIGHT;
    let _e97 = start;
    out[(_e97 + 11u)] = _e100;
    counter = 0u;
    let _e104 = counter;
    counter = (_e104 + u32(1i));
    let _e108 = counter;
    ordered.first = _e108;
    let _e111 = counter;
    counter = (_e111 + u32(1i));
    let _e115 = counter;
    ordered.second = _e115;
    let _e121 = ordered.first;
    let _e117 = start;
    out[(_e117 + 12u)] = _e121;
    let _e127 = ordered.second;
    let _e123 = start;
    out[(_e123 + 13u)] = _e127;
    let _e132 = counter;
    let _e129 = start;
    out[(_e129 + 14u)] = _e132;
    let _e134 = start;
    out[(_e134 + 15u)] = 31u;
    values[0] = 2u;
    values[1] = 4u;
    values[2] = 6u;
    let _e149 = lane;
    let _e151 = values[_e149];
    let _e146 = start;
    out[(_e146 + 16u)] = _e151;
    index = 0u;
    let _e158 = index;
    index = (_e158 + u32(1i));
    let _e163 = values[_e158];
    let _e155 = start;
    out[(_e155 + 17u)] = _e163;
    let _e168 = index;
    let _e165 = start;
    out[(_e165 + 18u)] = _e168;
    let _e170 = start;
    out[(_e170 + 19u)] = u32(true);
    let _e176 = start;
    out[(_e176 + 20u)] = 61u;
    let _e184 = lane;
    let _e181 = start;
    out[(_e181 + 21u)] = _e184;
    let _e186 = start;
    out[(_e186 + 22u)] = bitcast<u32>(-(0f));
    let _e193 = start;
    out[(_e193 + 23u)] = 4294967295u;
}
