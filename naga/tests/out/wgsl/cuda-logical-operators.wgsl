struct Sample_cuda_record_0_ {
    value: i32,
    payload: array<i32, 2>,
}

@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

fn operator_u0026_u0026_(left_1: Sample_cuda_record_0_, right_1: Sample_cuda_record_0_) -> Sample_cuda_record_0_ {
    var left_2: Sample_cuda_record_0_;
    var right_2: Sample_cuda_record_0_;

    left_2 = left_1;
    right_2 = right_1;
    let _e7 = left_2.value;
    let _e10 = right_2.value;
    left_2.value = ((_e7 * 10i) + _e10);
    let _e18 = right_2.payload[0i];
    let _e19 = left_2.payload[1i];
    left_2.payload[1i] = (_e19 + _e18);
    let _e21 = left_2;
    return _e21;
}

fn operator_u007c_u007c(left_3: Sample_cuda_record_0_, right_3: Sample_cuda_record_0_) -> Sample_cuda_record_0_ {
    var left_4: Sample_cuda_record_0_;
    var right_4: Sample_cuda_record_0_;

    left_4 = left_3;
    right_4 = right_3;
    let _e7 = left_4.value;
    let _e10 = right_4.value;
    left_4.value = ((_e7 * 100i) + _e10);
    let _e18 = right_4.payload[1i];
    let _e19 = left_4.payload[0i];
    left_4.payload[0i] = (_e19 + _e18);
    let _e21 = left_4;
    return _e21;
}

fn operator_u0026_u0026_1(left_5: Sample_cuda_record_0_, right_5: i32) -> i32 {
    var left_6: Sample_cuda_record_0_;
    var right_6: i32;

    left_6 = left_5;
    right_6 = right_5;
    let _e6 = left_6.value;
    let _e8 = right_6;
    return ((_e6 * 10i) + _e8);
}

fn operator_u007c_u007c_1(left_7: i32, right_7: Sample_cuda_record_0_) -> i32 {
    var left_8: i32;
    var right_8: Sample_cuda_record_0_;

    left_8 = left_7;
    right_8 = right_7;
    let _e5 = left_8;
    let _e8 = right_8.value;
    return ((_e5 * 100i) + _e8);
}

fn operator_u0026_u0026_2(left_9: vec2<i32>, right_9: vec2<i32>) -> vec2<i32> {
    var left_10: vec2<i32>;
    var right_10: vec2<i32>;

    left_10 = left_9;
    right_10 = right_9;
    let _e6 = left_10.x;
    let _e9 = right_10.x;
    let _e13 = left_10.y;
    let _e16 = right_10.y;
    return vec2<i32>(((_e6 * 10i) + _e9), ((_e13 * 10i) + _e16));
}

fn operator_u007c_u007c_2(left_11: vec2<i32>, right_11: vec2<i32>) -> vec2<i32> {
    var left_12: vec2<i32>;
    var right_12: vec2<i32>;

    left_12 = left_11;
    right_12 = right_11;
    let _e6 = left_12.x;
    let _e9 = right_12.x;
    let _e13 = left_12.y;
    let _e16 = right_12.y;
    return vec2<i32>(((_e6 * 100i) + _e9), ((_e13 * 100i) + _e16));
}

@compute @workgroup_size(2, 1, 1)
fn logical_operators(@builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var base: u32;
    var left: Sample_cuda_record_0_;
    var right: Sample_cuda_record_0_;
    var both: Sample_cuda_record_0_;
    var either: Sample_cuda_record_0_;
    var first: vec2<i32>;
    var second: vec2<i32>;
    var local: Sample_cuda_record_0_;
    var local_1: Sample_cuda_record_0_;
    var counter: i32;
    var evaluated: Sample_cuda_record_0_;
    var local_2: Sample_cuda_record_0_;
    var local_3: Sample_cuda_record_0_;
    var local_4: Sample_cuda_record_0_;
    var local_5: bool;
    var local_6: bool;
    var local_7: bool;
    var local_8: bool;

    base = (threadIdx.x * 24u);
    left.value = 2i;
    left.payload[0] = 3i;
    left.payload[1] = 4i;
    right.value = 5i;
    right.payload[0] = 6i;
    right.payload[1] = 7i;
    let _e23 = left;
    let _e24 = right;
    let _e25 = operator_u0026_u0026_(_e23, _e24);
    both = _e25;
    let _e27 = left;
    let _e28 = right;
    let _e29 = operator_u007c_u007c(_e27, _e28);
    either = _e29;
    let _e35 = both.value;
    let _e31 = base;
    out[(_e31 + 0u)] = u32(_e35);
    let _e44 = both.payload[1i];
    let _e38 = base;
    out[(_e38 + 1u)] = u32(_e44);
    let _e51 = either.value;
    let _e47 = base;
    out[(_e47 + 2u)] = u32(_e51);
    let _e60 = either.payload[0i];
    let _e54 = base;
    out[(_e54 + 3u)] = u32(_e60);
    let _e67 = left.value;
    let _e63 = base;
    out[(_e63 + 4u)] = u32(_e67);
    let _e76 = left.payload[1i];
    let _e70 = base;
    out[(_e70 + 5u)] = u32(_e76);
    let _e82 = left;
    let _e84 = operator_u0026_u0026_1(_e82, 9i);
    let _e79 = base;
    out[(_e79 + 6u)] = u32(_e84);
    let _e91 = right;
    let _e92 = operator_u007c_u007c_1(0i, _e91);
    let _e87 = base;
    out[(_e87 + 7u)] = u32(_e92);
    let _e98 = left;
    let _e99 = right;
    let _e100 = operator_u0026_u0026_(_e98, _e99);
    let _e95 = base;
    out[(_e95 + 8u)] = u32(_e100.value);
    let _e107 = left;
    let _e108 = right;
    let _e109 = operator_u007c_u007c(_e107, _e108);
    let _e104 = base;
    out[(_e104 + 9u)] = u32(_e109.payload[0i]);
    first = vec2<i32>(2i, 3i);
    second = vec2<i32>(4i, 5i);
    let _e126 = first;
    let _e127 = second;
    let _e128 = operator_u0026_u0026_2(_e126, _e127);
    let _e123 = base;
    out[(_e123 + 10u)] = u32(_e128.x);
    let _e135 = first;
    let _e136 = second;
    let _e137 = operator_u007c_u007c_2(_e135, _e136);
    let _e132 = base;
    out[(_e132 + 11u)] = u32(_e137.y);
    let _e144 = first;
    let _e145 = second;
    let _e146 = operator_u0026_u0026_2(_e144, _e145);
    let _e147 = first;
    let _e148 = operator_u0026_u0026_2(_e146, _e147);
    let _e141 = base;
    out[(_e141 + 12u)] = u32(_e148.x);
    let _e155 = first;
    let _e156 = second;
    let _e157 = first;
    let _e158 = operator_u0026_u0026_2(_e156, _e157);
    let _e159 = operator_u007c_u007c_2(_e155, _e158);
    let _e152 = base;
    out[(_e152 + 13u)] = u32(_e159.y);
    let _e166 = first;
    let _e167 = second;
    let _e168 = operator_u007c_u007c_2(_e166, _e167);
    let _e169 = first;
    let _e170 = operator_u0026_u0026_2(_e168, _e169);
    let _e163 = base;
    out[(_e163 + 14u)] = u32(_e170.x);
    if true {
        let _e179 = left;
        local = _e179;
    } else {
        let _e180 = right;
        local = _e180;
    }
    let _e181 = local;
    let _e183 = operator_u0026_u0026_1(_e181, 1i);
    let _e174 = base;
    out[(_e174 + 15u)] = u32(_e183);
    if true {
        let _e190 = left;
        let _e191 = right;
        let _e192 = operator_u0026_u0026_(_e190, _e191);
        local_1 = _e192;
    } else {
        let _e193 = right;
        let _e194 = left;
        let _e195 = operator_u007c_u007c(_e193, _e194);
        local_1 = _e195;
    }
    let _e197 = local_1;
    let _e186 = base;
    out[(_e186 + 16u)] = u32(_e197.value);
    counter = 0i;
    if false {
        let _e205 = counter;
        counter = (_e205 + 1i);
        local_2.value = _e205;
        local_2.payload[0] = 1i;
        local_2.payload[1] = 2i;
        let _e214 = local_2;
        let _e215 = right;
        let _e216 = operator_u0026_u0026_(_e214, _e215);
        local_4 = _e216;
    } else {
        let _e217 = left;
        let _e219 = counter;
        counter = (_e219 + 1i);
        local_3.value = _e219;
        local_3.payload[0] = 1i;
        local_3.payload[1] = 2i;
        let _e228 = local_3;
        let _e229 = operator_u007c_u007c(_e217, _e228);
        local_4 = _e229;
    }
    let _e231 = local_4;
    evaluated = _e231;
    let _e237 = evaluated.value;
    let _e233 = base;
    out[(_e233 + 17u)] = u32(_e237);
    let _e243 = counter;
    let _e240 = base;
    out[(_e240 + 18u)] = u32(_e243);
    local_5 = false;
    if false {
        let _e250 = left;
        let _e251 = right;
        let _e252 = operator_u0026_u0026_(_e250, _e251);
        local_5 = (_e252.value != 0i);
    }
    let _e257 = local_5;
    let _e246 = base;
    out[(_e246 + 19u)] = u32(_e257);
    local_6 = true;
    if true {
    } else {
        let _e264 = left;
        let _e265 = right;
        let _e266 = operator_u007c_u007c(_e264, _e265);
        local_6 = (_e266.value != 0i);
    }
    let _e271 = local_6;
    let _e260 = base;
    out[(_e260 + 20u)] = u32(_e271);
    let _e280 = (2i != 0i);
    local_7 = _e280;
    if _e280 {
        local_7 = (3i != 0i);
    }
    let _e284 = local_7;
    let _e274 = base;
    out[(_e274 + 21u)] = u32(_e284);
    let _e293 = (0i != 0i);
    local_8 = _e293;
    if _e293 {
    } else {
        local_8 = (4i != 0i);
    }
    let _e297 = local_8;
    let _e287 = base;
    out[(_e287 + 22u)] = u32(_e297);
    let _e300 = base;
    out[(_e300 + 23u)] = threadIdx.x;
}
