struct uint3_ {
    x: u32,
    y: u32,
    z: u32,
}

struct int3_ {
    x: i32,
    y: i32,
    z: i32,
}

struct float3_ {
    x: f32,
    y: f32,
    z: f32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;

fn choose(value: vec2<i32>) -> i32 {
    var value_1: vec2<i32>;

    value_1 = value;
    let _e4 = value_1.x;
    let _e7 = value_1.y;
    return ((_e4 * 10i) + _e7);
}

fn choose_1(value_2: uint3_) -> i32 {
    var value_3: uint3_;

    value_3 = value_2;
    let _e3 = value_3.z;
    return i32(_e3);
}

fn choose_2(value_4: vec4<f32>) -> i32 {
    var value_5: vec4<f32>;

    value_5 = value_4;
    let _e3 = value_5.w;
    return i32(_e3);
}

fn bump(value_6: int3_) -> int3_ {
    var value_7: int3_;

    value_7 = value_6;
    let _e4 = value_7.y;
    value_7.y = (_e4 + 1i);
    let _e6 = value_7;
    return _e6;
}

fn shift(value_8: float3_) -> float3_ {
    var value_9: float3_;

    value_9 = value_8;
    let _e4 = value_9.x;
    value_9.x = (_e4 + 2f);
    let _e9 = value_9.x;
    let _e10 = value_9.y;
    value_9.z = (_e9 + _e10);
    let _e12 = value_9;
    return _e12;
}

fn pair(first: float3_, second: i32) -> i32 {
    var first_1: float3_;
    var second_1: i32;

    first_1 = first;
    second_1 = second;
    let _e5 = first_1.x;
    let _e9 = second_1;
    return ((i32(_e5) * 10i) + _e9);
}

@compute @workgroup_size(1, 1, 1)
fn vector_values() {
    var seed: i32;
    var f2_: vec2<f32>;
    var f3_: float3_;
    var f4_: vec4<f32>;
    var i2_: vec2<i32>;
    var i3_: int3_;
    var i4_: vec4<i32>;
    var u2_: vec2<u32>;
    var u3_: uint3_;
    var u4_: vec4<u32>;
    var frozen: float3_;
    var copied: float3_;
    var changed: int3_;
    var samples: array<float3_, 2>;
    var index: i32;
    var choose_first: bool;
    var local: f32;
    var local_1: f32;
    var local_2: float3_;
    var local_3: i32;
    var argument: i32;
    var captured: int3_;

    let _e4 = out[0i];
    seed = _e4;
    let _e6 = seed;
    f2_ = vec2<f32>(f32(_e6), 1.5f);
    f3_ = float3_(1f, 2f, 3f);
    f4_ = vec4<f32>(f32(1i), f32(2i), f32(3i), f32(4i));
    let _e26 = seed;
    i2_ = vec2<i32>(_e26, -(2i));
    i3_ = int3_(1i, 2i, 3i);
    i4_ = vec4<i32>(1i, 2i, 3i, 4i);
    u2_ = vec2<u32>(u32(-(1i)), u32(2i));
    u3_ = uint3_(u32(1i), u32(2i), u32(3i));
    u4_ = vec4<u32>(u32(1i), u32(2i), u32(3i), u32(4i));
    let _e70 = f2_.x;
    let _e71 = f2_.y;
    out[0i] = i32((_e70 + _e71));
    let _e78 = f3_.x;
    let _e79 = f3_.y;
    let _e82 = f3_.z;
    out[1i] = i32(((_e78 + _e79) + _e82));
    let _e88 = f4_.w;
    out[2i] = i32(_e88);
    let _e94 = i2_.x;
    let _e95 = i2_.y;
    out[3i] = (_e94 + _e95);
    let _e100 = i3_.z;
    out[4i] = _e100;
    let _e105 = i4_.x;
    let _e106 = i4_.w;
    out[5i] = (_e105 + _e106);
    let _e112 = u2_.x;
    out[6i] = i32((_e112 == 4294967295u));
    let _e119 = u3_.x;
    let _e120 = u3_.z;
    out[7i] = i32((_e119 + _e120));
    let _e126 = u4_.w;
    out[8i] = i32(_e126);
    let _e129 = f3_;
    frozen = _e129;
    let _e131 = frozen;
    copied = _e131;
    copied.x = 9f;
    let _e138 = frozen.x;
    let _e141 = copied.x;
    out[9i] = i32(((_e138 * 10f) + _e141));
    let _e145 = i3_;
    let _e146 = bump(_e145);
    changed = _e146;
    let _e151 = changed.y;
    let _e154 = i3_.y;
    out[10i] = ((_e151 * 10i) + _e154);
    let _e158 = f3_;
    let _e159 = shift(_e158);
    out[11i] = i32(_e159.z);
    let _e164 = i2_;
    let _e165 = choose(_e164);
    let _e166 = u3_;
    let _e167 = choose_1(_e166);
    let _e169 = f4_;
    let _e170 = choose_2(_e169);
    out[12i] = ((_e165 + _e167) + _e170);
    let _e175 = f3_;
    samples[0i] = _e175;
    samples[1i] = float3_(f32(5i), f32(6i), f32(7i));
    index = 0i;
    let _e194 = index;
    index = (_e194 + 1i);
    let _e197 = index;
    let _e189 = index;
    index = (_e189 + 1i);
    let _e198 = samples[_e189].x;
    samples[_e189].x = (_e198 + f32(_e197));
    let _e201 = samples[_e189].x;
    out[13i] = i32(_e201);
    let _e206 = index;
    let _e211 = samples[0i].x;
    let _e219 = samples[1i].x;
    out[14i] = (((_e206 * 100i) + (i32(_e211) * 10i)) + i32(_e219));
    choose_first = false;
    let _e226 = choose_first;
    if _e226 {
        let _e234 = samples[0i].y;
        samples[0i].y = (_e234 + f32(1i));
        local = _e234;
    } else {
        let _e238 = samples[1i].y;
        samples[1i].y = (_e238 + f32(1i));
        local = _e238;
    }
    let _e242 = local;
    out[15i] = i32(_e242);
    let _e249 = samples[1i].y;
    out[16i] = i32(_e249);
    let _e258 = f3_;
    let _e253 = choose_first;
    if _e253 {
        samples[0i] = _e258;
        let _e262 = samples[0i].z;
        local_1 = _e262;
    } else {
        samples[1i] = _e258;
        let _e263 = samples[1i].z;
        local_1 = _e263;
    }
    let _e264 = local_1;
    out[17i] = i32(_e264);
    if true {
        local_2 = float3_(f32(7i), f32(8i), f32(9i));
    } else {
        let _e277 = f3_;
        local_2 = _e277;
    }
    let _e278 = local_2;
    out[18i] = i32(_e278.y);
    {
        let _e283 = seed;
        local_3 = (_e283 + 2i);
        let _e287 = local_3;
        out[19i] = _e287;
    }
    argument = 3i;
    let _e291 = argument;
    let _e292 = argument;
    argument = (_e292 + 1i);
    let _e295 = argument;
    captured = int3_(_e291, _e292, _e295);
    let _e301 = captured.x;
    let _e305 = captured.y;
    let _e309 = captured.z;
    out[20i] = (((_e301 * 100i) + (_e305 * 10i)) + _e309);
    let _e313 = argument;
    out[21i] = _e313;
    let _e316 = f3_;
    f3_.x = 9f;
    let _e320 = pair(_e316, 4i);
    out[22i] = _e320;
    let _e324 = f3_.x;
    out[23i] = i32(_e324);
}
