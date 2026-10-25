struct float3_ {
    x: f32,
    y: f32,
    z: f32,
}

struct Sample_cuda_record_0_ {
    position: float3_,
    tag: u32,
}

struct Packet_cuda_record_1_ {
    serial: u32,
    uv: vec2<f32>,
    samples: array<Sample_cuda_record_0_, 2>,
    weights: array<array<f32, 2>, 2>,
    colour: vec4<f32>,
}

struct Packet_cuda_record_2_ {
    w: i32,
    x: i32,
}

@group(0) @binding(0)
var<storage> input: array<Packet_cuda_record_1_>;
@group(0) @binding(1)
var<storage, read_write> output: array<Packet_cuda_record_1_>;

fn adjust(value: Packet_cuda_record_1_, bias: i32) -> Packet_cuda_record_1_ {
    var value_1: Packet_cuda_record_1_;
    var bias_1: i32;

    value_1 = value;
    bias_1 = bias;
    let _e5 = bias_1;
    let _e7 = value_1.serial;
    value_1.serial = (_e7 + u32(_e5));
    let _e19 = value_1.weights[0i][1i];
    let _e20 = value_1.samples[1i].position.x;
    value_1.samples[1i].position.x = (_e20 + _e19);
    let _e31 = value_1.samples[0i].tag;
    value_1.weights[1i][0i] = f32(_e31);
    let _e33 = value_1;
    return _e33;
}

fn operator_u002b(left: Packet_cuda_record_1_, right: Packet_cuda_record_1_) -> Packet_cuda_record_1_ {
    var left_1: Packet_cuda_record_1_;
    var right_1: Packet_cuda_record_1_;

    left_1 = left;
    right_1 = right;
    let _e6 = right_1.serial;
    let _e7 = left_1.serial;
    left_1.serial = (_e7 + _e6);
    let _e13 = right_1.uv.y;
    let _e14 = left_1.uv.x;
    left_1.uv.x = (_e14 + _e13);
    let _e16 = left_1;
    return _e16;
}

@compute @workgroup_size(2, 1, 1)
fn structures(@builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var lane: u32;
    var original: Packet_cuda_record_1_;
    var changed: Packet_cuda_record_1_;
    var local: Packet_cuda_record_2_;
    var combined: Packet_cuda_record_1_;
    var slot: u32;
    var sample: i32;
    var row: i32;
    var column: i32;

    lane = threadIdx.x;
    let _e6 = lane;
    if (_e6 >= 2u) {
        return;
    }
    let _e9 = lane;
    let _e11 = input[_e9];
    original = _e11;
    let _e13 = original;
    let _e15 = lane;
    let _e18 = adjust(_e13, (3i + i32(_e15)));
    changed = _e18;
    {
        local.w = 7i;
        local.x = 11i;
        let _e30 = local.w;
        let _e31 = local.x;
        let _e34 = changed.samples[0i].tag;
        changed.samples[0i].tag = (_e34 + u32((_e30 + _e31)));
    }
    let _e37 = changed;
    let _e38 = original;
    let _e39 = operator_u002b(_e37, _e38);
    combined = _e39;
    let _e46 = original.serial;
    let _e48 = combined.weights[0i][0i];
    combined.weights[0i][0i] = (_e48 + f32(_e46));
    let _e53 = combined.serial;
    combined.colour.w = f32(_e53);
    let _e57 = lane;
    slot = (_e57 + 1u);
    let _e63 = combined.serial;
    let _e59 = slot;
    output[_e59].serial = _e63;
    let _e68 = combined.uv;
    let _e64 = slot;
    output[_e64].uv = _e68;
    {
        sample = 0i;
        loop {
            let _e72 = sample;
            if (_e72 < 2i) {
            } else {
                break;
            }
            let _e83 = sample;
            let _e85 = combined.samples[_e83];
            let _e77 = slot;
            let _e80 = sample;
            output[_e77].samples[_e80] = _e85;
            continuing {
                let _e74 = sample;
                sample = (_e74 + 1i);
            }
        }
    }
    {
        row = 0i;
        loop {
            let _e89 = row;
            if (_e89 < 2i) {
            } else {
                break;
            }
            {
                column = 0i;
                loop {
                    let _e97 = column;
                    if (_e97 < 2i) {
                    } else {
                        break;
                    }
                    let _e110 = row;
                    let _e112 = column;
                    let _e114 = combined.weights[_e110][_e112];
                    let _e102 = slot;
                    let _e105 = row;
                    let _e107 = column;
                    output[_e102].weights[_e105][_e107] = _e114;
                    continuing {
                        let _e99 = column;
                        column = (_e99 + 1i);
                    }
                }
            }
            continuing {
                let _e91 = row;
                row = (_e91 + 1i);
            }
        }
    }
    let _e119 = combined.colour;
    let _e115 = slot;
    output[_e115].colour = _e119;
}
