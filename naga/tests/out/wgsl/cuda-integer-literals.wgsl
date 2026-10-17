struct type_2 {
    value: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<u32>;
@group(0) @binding(1)
var<storage> input: array<u32>;
@group(0) @binding(2)
var<uniform> n_1: type_2;
var<workgroup> shared_values: array<u32, 64>;

@compute @workgroup_size(64, 1, 1)
fn integer_literals(@builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: u32;
    var i: u32;
    var value: u32;
    var shift: u32;
    var base: u32;

    let _e4 = n_1.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    let _e15 = i;
    let _e16 = n;
    if (_e15 < _e16) {
        let _e19 = i;
        let _e21 = input[_e19];
        value = _e21;
        let _e24 = value;
        shift = (_e24 & 3u);
        let _e28 = i;
        base = (_e28 * 18u);
        let _e33 = value;
        shared_values[threadIdx.x] = (_e33 ^ u32(255i));
        let _e40 = shared_values[threadIdx.x];
        let _e36 = base;
        out[_e36] = _e40;
        let _e46 = value;
        let _e42 = base;
        out[(_e42 + 1u)] = (_e46 & u32(255i));
        let _e54 = value;
        let _e50 = base;
        out[(_e50 + 2u)] = (_e54 & 240u);
        let _e61 = value;
        let _e57 = base;
        out[(_e57 + 3u)] = (_e61 + 1000u);
        let _e68 = shift;
        let _e64 = base;
        out[(_e64 + 4u)] = (2147483648u >> _e68);
        let _e76 = shift;
        let _e71 = base;
        out[(_e71 + 5u)] = u32((i32(2147483648u) >> _e76));
        let _e84 = value;
        let _e80 = base;
        out[(_e80 + 6u)] = (4294967295u ^ _e84);
        let _e91 = value;
        let _e87 = base;
        out[(_e87 + 7u)] = u32((_e91 < 2147483648u));
        let _e98 = value;
        let _e95 = base;
        out[(_e95 + 8u)] = u32((i32(_e98) < 2147483647i));
        let _e108 = shift;
        let _e104 = base;
        out[(_e104 + 9u)] = (2147483648u >> _e108);
        let _e115 = value;
        let _e111 = base;
        out[(_e111 + 10u)] = (255u + _e115);
        let _e122 = value;
        let _e118 = base;
        out[(_e118 + 11u)] = (((_e122 + 10u) + 8u) + 10u);
        let _e133 = value;
        let _e129 = base;
        out[(_e129 + 12u)] = (_e133 + u32(15i));
        let _e141 = value;
        let _e137 = base;
        out[(_e137 + 13u)] = (4294967295u - _e141);
        let _e148 = value;
        let _e144 = base;
        out[(_e144 + 14u)] = (3735928559u ^ _e148);
        let _e156 = value;
        let _e151 = base;
        out[(_e151 + 15u)] = u32((~(2147483647i) < i32(_e156)));
        let _e165 = value;
        let _e161 = base;
        out[(_e161 + 16u)] = ((_e165 + u32(30i)) + u32(2i));
        let _e176 = value;
        let _e172 = base;
        out[(_e172 + 17u)] = ((_e176 + u32(30i)) - u32(2i));
    }
}
