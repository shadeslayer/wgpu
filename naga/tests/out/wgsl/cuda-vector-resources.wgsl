struct float3_ {
    x: f32,
    y: f32,
    z: f32,
}

@group(0) @binding(0)
var<storage> input: array<float3_>;
@group(0) @binding(1)
var<storage, read_write> output: array<float3_>;

fn shift(value: float3_) -> float3_ {
    var value_1: float3_;

    value_1 = value;
    let _e4 = value_1.x;
    value_1.x = (_e4 + 1f);
    let _e9 = value_1.y;
    let _e10 = value_1.z;
    value_1.z = (_e9 + _e10);
    let _e12 = value_1;
    return _e12;
}

fn pair(value_2: float3_, digit: i32) -> f32 {
    var value_3: float3_;
    var digit_1: i32;

    value_3 = value_2;
    digit_1 = digit;
    let _e6 = value_3.x;
    let _e8 = digit_1;
    return ((_e6 * 10f) + f32(_e8));
}

@compute @workgroup_size(1, 1, 1)
fn vector_resources() {
    var i: i32;
    var copied: float3_;
    var index: i32;
    var choose_first: bool;
    var local: f32;
    var local_1: f32;
    var local_2: float3_;
    var pick_local: bool;
    var snapshot: float3_;

    {
        i = 0i;
        loop {
            let _e5 = i;
            if (_e5 < 3i) {
            } else {
                break;
            }
            let _e11 = i;
            let _e13 = input[_e11];
            copied = _e13;
            let _e16 = copied;
            let _e17 = shift(_e16);
            let _e14 = i;
            output[_e14] = _e17;
            continuing {
                let _e7 = i;
                i = (_e7 + 1i);
            }
        }
    }
    let _e22 = input[0i];
    output[3i] = _e22;
    let _e28 = input[1i];
    let _e29 = shift(_e28);
    output[3i].y = _e29.x;
    index = 0i;
    index = 1i;
    let _e42 = input[2i].x;
    let _e33 = index;
    index = (_e33 + 1i);
    output[_e33].x = _e42;
    let _e45 = index;
    let _e49 = input[0i].x;
    let _e53 = input[1i].x;
    output[4i] = float3_(f32(_e45), _e49, _e53);
    choose_first = true;
    let _e58 = choose_first;
    if _e58 {
        let _e66 = output[0i].y;
        output[0i].y = (_e66 + 2f);
    } else {
        let _e68 = output[2i].y;
        output[2i].y = (_e68 + 2f);
    }
    let _e73 = choose_first;
    if _e73 {
        let _e81 = output[0i].z;
        output[0i].z = (_e81 + f32(1i));
        local = _e81;
    } else {
        let _e85 = output[2i].z;
        output[2i].z = (_e85 + f32(1i));
        local = _e85;
    }
    let _e89 = local;
    output[4i].x = _e89;
    let _e93 = index;
    output[4i].z = f32(_e93);
    let _e102 = input[1i];
    let _e95 = choose_first;
    if _e95 {
        output[0i] = _e102;
        let _e106 = output[0i].x;
        output[0i].x = (_e106 + f32(1i));
        local_1 = _e106;
    } else {
        output[2i] = _e102;
        let _e110 = output[2i].x;
        output[2i].x = (_e110 + f32(1i));
        local_1 = _e110;
    }
    let _e118 = output[0i];
    output[5i] = _e118;
    local_2 = float3_(f32(20i), f32(30i), f32(40i));
    pick_local = false;
    pick_local = true;
    let _e129 = pick_local;
    if _e129 {
        local_2.z = 7f;
    } else {
        output[1i].z = 7f;
    }
    let _e138 = local_2;
    output[6i] = _e138;
    let _e145 = output[1i].z;
    output[6i].x = _e145;
    let _e151 = output[0i].x;
    let _e155 = output[1i].x;
    let _e159 = output[2i].x;
    output[7i] = float3_(_e151, _e155, _e159);
    let _e164 = input[0i];
    snapshot = _e164;
    let _e167 = snapshot;
    snapshot.x = 9f;
    let _e171 = pair(_e167, 4i);
    let _e173 = snapshot.x;
    let _e177 = input[2i].z;
    output[8i] = float3_(_e171, _e173, _e177);
}
