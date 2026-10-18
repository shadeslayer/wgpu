struct float3_ {
    x: f32,
    y: f32,
    z: f32,
}

struct int3_ {
    x: i32,
    y: i32,
    z: i32,
}

struct uint3_ {
    x: u32,
    y: u32,
    z: u32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<u32>;

fn operator_u002b(left: vec2<f32>, right: vec2<f32>) -> vec2<f32> {
    var left_1: vec2<f32>;
    var right_1: vec2<f32>;

    left_1 = left;
    right_1 = right;
    let _e6 = left_1.x;
    let _e7 = right_1.x;
    let _e11 = left_1.y;
    let _e12 = right_1.y;
    return vec2<f32>((_e6 + _e7), (_e11 + _e12));
}

fn operator_u002b_1(left_2: vec2<f32>, right_2: f32) -> vec2<f32> {
    var left_3: vec2<f32>;
    var right_3: f32;

    left_3 = left_2;
    right_3 = right_2;
    let _e5 = left_3.x;
    let _e6 = right_3;
    let _e9 = left_3.y;
    let _e10 = right_3;
    return vec2<f32>((_e5 + _e6), (_e9 + _e10));
}

fn operator_u002b_2(left_4: f32, right_4: vec2<f32>) -> vec2<f32> {
    var left_5: f32;
    var right_5: vec2<f32>;

    left_5 = left_4;
    right_5 = right_4;
    let _e5 = left_5;
    let _e6 = right_5.x;
    let _e9 = left_5;
    let _e10 = right_5.y;
    return vec2<f32>((_e5 + _e6), (_e9 + _e10));
}

fn operator_u002b_3(left_6: float3_, right_6: float3_) -> float3_ {
    var left_7: float3_;
    var right_7: float3_;

    left_7 = left_6;
    right_7 = right_6;
    let _e6 = left_7.x;
    let _e7 = right_7.x;
    let _e11 = left_7.y;
    let _e12 = right_7.y;
    let _e16 = left_7.z;
    let _e17 = right_7.z;
    return float3_((_e6 + _e7), (_e11 + _e12), (_e16 + _e17));
}

fn operator_u002b_4(left_8: float3_, right_8: f32) -> float3_ {
    var left_9: float3_;
    var right_9: f32;

    left_9 = left_8;
    right_9 = right_8;
    let _e5 = left_9.x;
    let _e6 = right_9;
    let _e9 = left_9.y;
    let _e10 = right_9;
    let _e13 = left_9.z;
    let _e14 = right_9;
    return float3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

fn operator_u002b_5(left_10: f32, right_10: float3_) -> float3_ {
    var left_11: f32;
    var right_11: float3_;

    left_11 = left_10;
    right_11 = right_10;
    let _e5 = left_11;
    let _e6 = right_11.x;
    let _e9 = left_11;
    let _e10 = right_11.y;
    let _e13 = left_11;
    let _e14 = right_11.z;
    return float3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

fn operator_u002b_6(left_12: vec4<f32>, right_12: vec4<f32>) -> vec4<f32> {
    var left_13: vec4<f32>;
    var right_13: vec4<f32>;

    left_13 = left_12;
    right_13 = right_12;
    let _e6 = left_13.x;
    let _e7 = right_13.x;
    let _e11 = left_13.y;
    let _e12 = right_13.y;
    let _e16 = left_13.z;
    let _e17 = right_13.z;
    let _e21 = left_13.w;
    let _e22 = right_13.w;
    return vec4<f32>((_e6 + _e7), (_e11 + _e12), (_e16 + _e17), (_e21 + _e22));
}

fn operator_u002b_7(left_14: vec4<f32>, right_14: f32) -> vec4<f32> {
    var left_15: vec4<f32>;
    var right_15: f32;

    left_15 = left_14;
    right_15 = right_14;
    let _e5 = left_15.x;
    let _e6 = right_15;
    let _e9 = left_15.y;
    let _e10 = right_15;
    let _e13 = left_15.z;
    let _e14 = right_15;
    let _e17 = left_15.w;
    let _e18 = right_15;
    return vec4<f32>((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

fn operator_u002b_8(left_16: f32, right_16: vec4<f32>) -> vec4<f32> {
    var left_17: f32;
    var right_17: vec4<f32>;

    left_17 = left_16;
    right_17 = right_16;
    let _e5 = left_17;
    let _e6 = right_17.x;
    let _e9 = left_17;
    let _e10 = right_17.y;
    let _e13 = left_17;
    let _e14 = right_17.z;
    let _e17 = left_17;
    let _e18 = right_17.w;
    return vec4<f32>((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

fn operator_u002b_9(left_18: vec2<i32>, right_18: vec2<i32>) -> vec2<i32> {
    var left_19: vec2<i32>;
    var right_19: vec2<i32>;

    left_19 = left_18;
    right_19 = right_18;
    let _e6 = left_19.x;
    let _e7 = right_19.x;
    let _e11 = left_19.y;
    let _e12 = right_19.y;
    return vec2<i32>((_e6 + _e7), (_e11 + _e12));
}

fn operator_u002b_10(left_20: vec2<i32>, right_20: i32) -> vec2<i32> {
    var left_21: vec2<i32>;
    var right_21: i32;

    left_21 = left_20;
    right_21 = right_20;
    let _e5 = left_21.x;
    let _e6 = right_21;
    let _e9 = left_21.y;
    let _e10 = right_21;
    return vec2<i32>((_e5 + _e6), (_e9 + _e10));
}

fn operator_u002b_11(left_22: i32, right_22: vec2<i32>) -> vec2<i32> {
    var left_23: i32;
    var right_23: vec2<i32>;

    left_23 = left_22;
    right_23 = right_22;
    let _e5 = left_23;
    let _e6 = right_23.x;
    let _e9 = left_23;
    let _e10 = right_23.y;
    return vec2<i32>((_e5 + _e6), (_e9 + _e10));
}

fn operator_u002b_12(left_24: int3_, right_24: int3_) -> int3_ {
    var left_25: int3_;
    var right_25: int3_;

    left_25 = left_24;
    right_25 = right_24;
    let _e6 = left_25.x;
    let _e7 = right_25.x;
    let _e11 = left_25.y;
    let _e12 = right_25.y;
    let _e16 = left_25.z;
    let _e17 = right_25.z;
    return int3_((_e6 + _e7), (_e11 + _e12), (_e16 + _e17));
}

fn operator_u002b_13(left_26: int3_, right_26: i32) -> int3_ {
    var left_27: int3_;
    var right_27: i32;

    left_27 = left_26;
    right_27 = right_26;
    let _e5 = left_27.x;
    let _e6 = right_27;
    let _e9 = left_27.y;
    let _e10 = right_27;
    let _e13 = left_27.z;
    let _e14 = right_27;
    return int3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

fn operator_u002b_14(left_28: i32, right_28: int3_) -> int3_ {
    var left_29: i32;
    var right_29: int3_;

    left_29 = left_28;
    right_29 = right_28;
    let _e5 = left_29;
    let _e6 = right_29.x;
    let _e9 = left_29;
    let _e10 = right_29.y;
    let _e13 = left_29;
    let _e14 = right_29.z;
    return int3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

fn operator_u002b_15(left_30: vec4<i32>, right_30: vec4<i32>) -> vec4<i32> {
    var left_31: vec4<i32>;
    var right_31: vec4<i32>;

    left_31 = left_30;
    right_31 = right_30;
    let _e6 = left_31.x;
    let _e7 = right_31.x;
    let _e11 = left_31.y;
    let _e12 = right_31.y;
    let _e16 = left_31.z;
    let _e17 = right_31.z;
    let _e21 = left_31.w;
    let _e22 = right_31.w;
    return vec4<i32>((_e6 + _e7), (_e11 + _e12), (_e16 + _e17), (_e21 + _e22));
}

fn operator_u002b_16(left_32: vec4<i32>, right_32: i32) -> vec4<i32> {
    var left_33: vec4<i32>;
    var right_33: i32;

    left_33 = left_32;
    right_33 = right_32;
    let _e5 = left_33.x;
    let _e6 = right_33;
    let _e9 = left_33.y;
    let _e10 = right_33;
    let _e13 = left_33.z;
    let _e14 = right_33;
    let _e17 = left_33.w;
    let _e18 = right_33;
    return vec4<i32>((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

fn operator_u002b_17(left_34: i32, right_34: vec4<i32>) -> vec4<i32> {
    var left_35: i32;
    var right_35: vec4<i32>;

    left_35 = left_34;
    right_35 = right_34;
    let _e5 = left_35;
    let _e6 = right_35.x;
    let _e9 = left_35;
    let _e10 = right_35.y;
    let _e13 = left_35;
    let _e14 = right_35.z;
    let _e17 = left_35;
    let _e18 = right_35.w;
    return vec4<i32>((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

fn operator_u002b_18(left_36: vec2<u32>, right_36: vec2<u32>) -> vec2<u32> {
    var left_37: vec2<u32>;
    var right_37: vec2<u32>;

    left_37 = left_36;
    right_37 = right_36;
    let _e6 = left_37.x;
    let _e7 = right_37.x;
    let _e11 = left_37.y;
    let _e12 = right_37.y;
    return vec2<u32>((_e6 + _e7), (_e11 + _e12));
}

fn operator_u002b_19(left_38: vec2<u32>, right_38: u32) -> vec2<u32> {
    var left_39: vec2<u32>;
    var right_39: u32;

    left_39 = left_38;
    right_39 = right_38;
    let _e5 = left_39.x;
    let _e6 = right_39;
    let _e9 = left_39.y;
    let _e10 = right_39;
    return vec2<u32>((_e5 + _e6), (_e9 + _e10));
}

fn operator_u002b_20(left_40: u32, right_40: vec2<u32>) -> vec2<u32> {
    var left_41: u32;
    var right_41: vec2<u32>;

    left_41 = left_40;
    right_41 = right_40;
    let _e5 = left_41;
    let _e6 = right_41.x;
    let _e9 = left_41;
    let _e10 = right_41.y;
    return vec2<u32>((_e5 + _e6), (_e9 + _e10));
}

fn operator_u002b_21(left_42: uint3_, right_42: uint3_) -> uint3_ {
    var left_43: uint3_;
    var right_43: uint3_;

    left_43 = left_42;
    right_43 = right_42;
    let _e6 = left_43.x;
    let _e7 = right_43.x;
    let _e11 = left_43.y;
    let _e12 = right_43.y;
    let _e16 = left_43.z;
    let _e17 = right_43.z;
    return uint3_((_e6 + _e7), (_e11 + _e12), (_e16 + _e17));
}

fn operator_u002b_22(left_44: uint3_, right_44: u32) -> uint3_ {
    var left_45: uint3_;
    var right_45: u32;

    left_45 = left_44;
    right_45 = right_44;
    let _e5 = left_45.x;
    let _e6 = right_45;
    let _e9 = left_45.y;
    let _e10 = right_45;
    let _e13 = left_45.z;
    let _e14 = right_45;
    return uint3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

fn operator_u002b_23(left_46: u32, right_46: uint3_) -> uint3_ {
    var left_47: u32;
    var right_47: uint3_;

    left_47 = left_46;
    right_47 = right_46;
    let _e5 = left_47;
    let _e6 = right_47.x;
    let _e9 = left_47;
    let _e10 = right_47.y;
    let _e13 = left_47;
    let _e14 = right_47.z;
    return uint3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

fn operator_u002b_24(left_48: vec4<u32>, right_48: vec4<u32>) -> vec4<u32> {
    var left_49: vec4<u32>;
    var right_49: vec4<u32>;

    left_49 = left_48;
    right_49 = right_48;
    let _e6 = left_49.x;
    let _e7 = right_49.x;
    let _e11 = left_49.y;
    let _e12 = right_49.y;
    let _e16 = left_49.z;
    let _e17 = right_49.z;
    let _e21 = left_49.w;
    let _e22 = right_49.w;
    return vec4<u32>((_e6 + _e7), (_e11 + _e12), (_e16 + _e17), (_e21 + _e22));
}

fn operator_u002b_25(left_50: vec4<u32>, right_50: u32) -> vec4<u32> {
    var left_51: vec4<u32>;
    var right_51: u32;

    left_51 = left_50;
    right_51 = right_50;
    let _e5 = left_51.x;
    let _e6 = right_51;
    let _e9 = left_51.y;
    let _e10 = right_51;
    let _e13 = left_51.z;
    let _e14 = right_51;
    let _e17 = left_51.w;
    let _e18 = right_51;
    return vec4<u32>((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

fn operator_u002b_26(left_52: u32, right_52: vec4<u32>) -> vec4<u32> {
    var left_53: u32;
    var right_53: vec4<u32>;

    left_53 = left_52;
    right_53 = right_52;
    let _e5 = left_53;
    let _e6 = right_53.x;
    let _e9 = left_53;
    let _e10 = right_53.y;
    let _e13 = left_53;
    let _e14 = right_53.z;
    let _e17 = left_53;
    let _e18 = right_53.w;
    return vec4<u32>((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

@compute @workgroup_size(1, 1, 1)
fn additions() {
    var float_2_left: vec2<f32>;
    var float_2_right: vec2<f32>;
    var float_2_vectors: vec2<f32>;
    var float_2_vector_scalar: vec2<f32>;
    var float_2_scalar_vector: vec2<f32>;
    var float_3_left: float3_;
    var float_3_right: float3_;
    var float_3_vectors: float3_;
    var float_3_vector_scalar: float3_;
    var float_3_scalar_vector: float3_;
    var float_4_left: vec4<f32>;
    var float_4_right: vec4<f32>;
    var float_4_vectors: vec4<f32>;
    var float_4_vector_scalar: vec4<f32>;
    var float_4_scalar_vector: vec4<f32>;
    var int_2_left: vec2<i32>;
    var int_2_right: vec2<i32>;
    var int_2_vectors: vec2<i32>;
    var int_2_vector_scalar: vec2<i32>;
    var int_2_scalar_vector: vec2<i32>;
    var int_3_left: int3_;
    var int_3_right: int3_;
    var int_3_vectors: int3_;
    var int_3_vector_scalar: int3_;
    var int_3_scalar_vector: int3_;
    var int_4_left: vec4<i32>;
    var int_4_right: vec4<i32>;
    var int_4_vectors: vec4<i32>;
    var int_4_vector_scalar: vec4<i32>;
    var int_4_scalar_vector: vec4<i32>;
    var uint_2_left: vec2<u32>;
    var uint_2_right: vec2<u32>;
    var uint_2_vectors: vec2<u32>;
    var uint_2_vector_scalar: vec2<u32>;
    var uint_2_scalar_vector: vec2<u32>;
    var uint_3_left: uint3_;
    var uint_3_right: uint3_;
    var uint_3_vectors: uint3_;
    var uint_3_vector_scalar: uint3_;
    var uint_3_scalar_vector: uint3_;
    var uint_4_left: vec4<u32>;
    var uint_4_right: vec4<u32>;
    var uint_4_vectors: vec4<u32>;
    var uint_4_vector_scalar: vec4<u32>;
    var uint_4_scalar_vector: vec4<u32>;

    float_2_left = vec2<f32>(1.25f, -(2.5f));
    float_2_right = vec2<f32>(0.5f, 1.5f);
    let _e11 = float_2_left;
    let _e12 = float_2_right;
    let _e13 = operator_u002b(_e11, _e12);
    float_2_vectors = _e13;
    let _e15 = float_2_left;
    let _e17 = operator_u002b_1(_e15, 2f);
    float_2_vector_scalar = _e17;
    let _e20 = float_2_right;
    let _e21 = operator_u002b_2(2f, _e20);
    float_2_scalar_vector = _e21;
    let _e25 = float_2_vectors.x;
    out[0i] = bitcast<u32>(_e25);
    let _e30 = float_2_vectors.y;
    out[1i] = bitcast<u32>(_e30);
    let _e35 = float_2_vector_scalar.x;
    out[2i] = bitcast<u32>(_e35);
    let _e40 = float_2_vector_scalar.y;
    out[3i] = bitcast<u32>(_e40);
    let _e45 = float_2_scalar_vector.x;
    out[4i] = bitcast<u32>(_e45);
    let _e50 = float_2_scalar_vector.y;
    out[5i] = bitcast<u32>(_e50);
    float_3_left = float3_(1.25f, -(2.5f), 3.75f);
    float_3_right = float3_(0.5f, 1.5f, -(2f));
    let _e65 = float_3_left;
    let _e66 = float_3_right;
    let _e67 = operator_u002b_3(_e65, _e66);
    float_3_vectors = _e67;
    let _e69 = float_3_left;
    let _e71 = operator_u002b_4(_e69, 2f);
    float_3_vector_scalar = _e71;
    let _e74 = float_3_right;
    let _e75 = operator_u002b_5(2f, _e74);
    float_3_scalar_vector = _e75;
    let _e79 = float_3_vectors.x;
    out[6i] = bitcast<u32>(_e79);
    let _e84 = float_3_vectors.y;
    out[7i] = bitcast<u32>(_e84);
    let _e89 = float_3_vectors.z;
    out[8i] = bitcast<u32>(_e89);
    let _e94 = float_3_vector_scalar.x;
    out[9i] = bitcast<u32>(_e94);
    let _e99 = float_3_vector_scalar.y;
    out[10i] = bitcast<u32>(_e99);
    let _e104 = float_3_vector_scalar.z;
    out[11i] = bitcast<u32>(_e104);
    let _e109 = float_3_scalar_vector.x;
    out[12i] = bitcast<u32>(_e109);
    let _e114 = float_3_scalar_vector.y;
    out[13i] = bitcast<u32>(_e114);
    let _e119 = float_3_scalar_vector.z;
    out[14i] = bitcast<u32>(_e119);
    float_4_left = vec4<f32>(1.25f, -(2.5f), 3.75f, -(4f));
    float_4_right = vec4<f32>(0.5f, 1.5f, -(2f), 0.25f);
    let _e137 = float_4_left;
    let _e138 = float_4_right;
    let _e139 = operator_u002b_6(_e137, _e138);
    float_4_vectors = _e139;
    let _e141 = float_4_left;
    let _e143 = operator_u002b_7(_e141, 2f);
    float_4_vector_scalar = _e143;
    let _e146 = float_4_right;
    let _e147 = operator_u002b_8(2f, _e146);
    float_4_scalar_vector = _e147;
    let _e151 = float_4_vectors.x;
    out[15i] = bitcast<u32>(_e151);
    let _e156 = float_4_vectors.y;
    out[16i] = bitcast<u32>(_e156);
    let _e161 = float_4_vectors.z;
    out[17i] = bitcast<u32>(_e161);
    let _e166 = float_4_vectors.w;
    out[18i] = bitcast<u32>(_e166);
    let _e171 = float_4_vector_scalar.x;
    out[19i] = bitcast<u32>(_e171);
    let _e176 = float_4_vector_scalar.y;
    out[20i] = bitcast<u32>(_e176);
    let _e181 = float_4_vector_scalar.z;
    out[21i] = bitcast<u32>(_e181);
    let _e186 = float_4_vector_scalar.w;
    out[22i] = bitcast<u32>(_e186);
    let _e191 = float_4_scalar_vector.x;
    out[23i] = bitcast<u32>(_e191);
    let _e196 = float_4_scalar_vector.y;
    out[24i] = bitcast<u32>(_e196);
    let _e201 = float_4_scalar_vector.z;
    out[25i] = bitcast<u32>(_e201);
    let _e206 = float_4_scalar_vector.w;
    out[26i] = bitcast<u32>(_e206);
    int_2_left = vec2<i32>(1i, -(2i));
    int_2_right = vec2<i32>(5i, 6i);
    let _e218 = int_2_left;
    let _e219 = int_2_right;
    let _e220 = operator_u002b_9(_e218, _e219);
    int_2_vectors = _e220;
    let _e222 = int_2_left;
    let _e224 = operator_u002b_10(_e222, 3i);
    int_2_vector_scalar = _e224;
    let _e227 = int_2_right;
    let _e228 = operator_u002b_11(3i, _e227);
    int_2_scalar_vector = _e228;
    let _e232 = int_2_vectors.x;
    out[27i] = u32(_e232);
    let _e237 = int_2_vectors.y;
    out[28i] = u32(_e237);
    let _e242 = int_2_vector_scalar.x;
    out[29i] = u32(_e242);
    let _e247 = int_2_vector_scalar.y;
    out[30i] = u32(_e247);
    let _e252 = int_2_scalar_vector.x;
    out[31i] = u32(_e252);
    let _e257 = int_2_scalar_vector.y;
    out[32i] = u32(_e257);
    int_3_left = int3_(1i, -(2i), 3i);
    int_3_right = int3_(5i, 6i, -(7i));
    let _e272 = int_3_left;
    let _e273 = int_3_right;
    let _e274 = operator_u002b_12(_e272, _e273);
    int_3_vectors = _e274;
    let _e276 = int_3_left;
    let _e278 = operator_u002b_13(_e276, 3i);
    int_3_vector_scalar = _e278;
    let _e281 = int_3_right;
    let _e282 = operator_u002b_14(3i, _e281);
    int_3_scalar_vector = _e282;
    let _e286 = int_3_vectors.x;
    out[33i] = u32(_e286);
    let _e291 = int_3_vectors.y;
    out[34i] = u32(_e291);
    let _e296 = int_3_vectors.z;
    out[35i] = u32(_e296);
    let _e301 = int_3_vector_scalar.x;
    out[36i] = u32(_e301);
    let _e306 = int_3_vector_scalar.y;
    out[37i] = u32(_e306);
    let _e311 = int_3_vector_scalar.z;
    out[38i] = u32(_e311);
    let _e316 = int_3_scalar_vector.x;
    out[39i] = u32(_e316);
    let _e321 = int_3_scalar_vector.y;
    out[40i] = u32(_e321);
    let _e326 = int_3_scalar_vector.z;
    out[41i] = u32(_e326);
    int_4_left = vec4<i32>(1i, -(2i), 3i, -(4i));
    int_4_right = vec4<i32>(5i, 6i, -(7i), 8i);
    let _e344 = int_4_left;
    let _e345 = int_4_right;
    let _e346 = operator_u002b_15(_e344, _e345);
    int_4_vectors = _e346;
    let _e348 = int_4_left;
    let _e350 = operator_u002b_16(_e348, 3i);
    int_4_vector_scalar = _e350;
    let _e353 = int_4_right;
    let _e354 = operator_u002b_17(3i, _e353);
    int_4_scalar_vector = _e354;
    let _e358 = int_4_vectors.x;
    out[42i] = u32(_e358);
    let _e363 = int_4_vectors.y;
    out[43i] = u32(_e363);
    let _e368 = int_4_vectors.z;
    out[44i] = u32(_e368);
    let _e373 = int_4_vectors.w;
    out[45i] = u32(_e373);
    let _e378 = int_4_vector_scalar.x;
    out[46i] = u32(_e378);
    let _e383 = int_4_vector_scalar.y;
    out[47i] = u32(_e383);
    let _e388 = int_4_vector_scalar.z;
    out[48i] = u32(_e388);
    let _e393 = int_4_vector_scalar.w;
    out[49i] = u32(_e393);
    let _e398 = int_4_scalar_vector.x;
    out[50i] = u32(_e398);
    let _e403 = int_4_scalar_vector.y;
    out[51i] = u32(_e403);
    let _e408 = int_4_scalar_vector.z;
    out[52i] = u32(_e408);
    let _e413 = int_4_scalar_vector.w;
    out[53i] = u32(_e413);
    uint_2_left = vec2<u32>(4294967294u, 2147483649u);
    uint_2_right = vec2<u32>(5u, 2u);
    let _e424 = uint_2_left;
    let _e425 = uint_2_right;
    let _e426 = operator_u002b_18(_e424, _e425);
    uint_2_vectors = _e426;
    let _e428 = uint_2_left;
    let _e430 = operator_u002b_19(_e428, 4u);
    uint_2_vector_scalar = _e430;
    let _e433 = uint_2_right;
    let _e434 = operator_u002b_20(4u, _e433);
    uint_2_scalar_vector = _e434;
    let _e438 = uint_2_vectors.x;
    out[54i] = _e438;
    let _e442 = uint_2_vectors.y;
    out[55i] = _e442;
    let _e446 = uint_2_vector_scalar.x;
    out[56i] = _e446;
    let _e450 = uint_2_vector_scalar.y;
    out[57i] = _e450;
    let _e454 = uint_2_scalar_vector.x;
    out[58i] = _e454;
    let _e458 = uint_2_scalar_vector.y;
    out[59i] = _e458;
    uint_3_left = uint3_(4294967294u, 2147483649u, 3u);
    uint_3_right = uint3_(5u, 2u, 4294967293u);
    let _e470 = uint_3_left;
    let _e471 = uint_3_right;
    let _e472 = operator_u002b_21(_e470, _e471);
    uint_3_vectors = _e472;
    let _e474 = uint_3_left;
    let _e476 = operator_u002b_22(_e474, 4u);
    uint_3_vector_scalar = _e476;
    let _e479 = uint_3_right;
    let _e480 = operator_u002b_23(4u, _e479);
    uint_3_scalar_vector = _e480;
    let _e484 = uint_3_vectors.x;
    out[60i] = _e484;
    let _e488 = uint_3_vectors.y;
    out[61i] = _e488;
    let _e492 = uint_3_vectors.z;
    out[62i] = _e492;
    let _e496 = uint_3_vector_scalar.x;
    out[63i] = _e496;
    let _e500 = uint_3_vector_scalar.y;
    out[64i] = _e500;
    let _e504 = uint_3_vector_scalar.z;
    out[65i] = _e504;
    let _e508 = uint_3_scalar_vector.x;
    out[66i] = _e508;
    let _e512 = uint_3_scalar_vector.y;
    out[67i] = _e512;
    let _e516 = uint_3_scalar_vector.z;
    out[68i] = _e516;
    uint_4_left = vec4<u32>(4294967294u, 2147483649u, 3u, 4294967295u);
    uint_4_right = vec4<u32>(5u, 2u, 4294967293u, 1u);
    let _e530 = uint_4_left;
    let _e531 = uint_4_right;
    let _e532 = operator_u002b_24(_e530, _e531);
    uint_4_vectors = _e532;
    let _e534 = uint_4_left;
    let _e536 = operator_u002b_25(_e534, 4u);
    uint_4_vector_scalar = _e536;
    let _e539 = uint_4_right;
    let _e540 = operator_u002b_26(4u, _e539);
    uint_4_scalar_vector = _e540;
    let _e544 = uint_4_vectors.x;
    out[69i] = _e544;
    let _e548 = uint_4_vectors.y;
    out[70i] = _e548;
    let _e552 = uint_4_vectors.z;
    out[71i] = _e552;
    let _e556 = uint_4_vectors.w;
    out[72i] = _e556;
    let _e560 = uint_4_vector_scalar.x;
    out[73i] = _e560;
    let _e564 = uint_4_vector_scalar.y;
    out[74i] = _e564;
    let _e568 = uint_4_vector_scalar.z;
    out[75i] = _e568;
    let _e572 = uint_4_vector_scalar.w;
    out[76i] = _e572;
    let _e576 = uint_4_scalar_vector.x;
    out[77i] = _e576;
    let _e580 = uint_4_scalar_vector.y;
    out[78i] = _e580;
    let _e584 = uint_4_scalar_vector.z;
    out[79i] = _e584;
    let _e588 = uint_4_scalar_vector.w;
    out[80i] = _e588;
}
