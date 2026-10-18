#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct float3_ {
    float x;
    float y;
    float z;
};
struct int3_ {
    int x;
    int y;
    int z;
};
struct uint3_ {
    uint x;
    uint y;
    uint z;
};
layout(std430) buffer type_9_block_0Compute { uint _group_0_binding_0_cs[]; };


vec2 operator_u002b(vec2 left, vec2 right) {
    vec2 left_1 = vec2(0.0);
    vec2 right_1 = vec2(0.0);
    left_1 = left;
    right_1 = right;
    float _e6 = left_1.x;
    float _e7 = right_1.x;
    float _e11 = left_1.y;
    float _e12 = right_1.y;
    return vec2((_e6 + _e7), (_e11 + _e12));
}

vec2 operator_u002b_1(vec2 left_2, float right_2) {
    vec2 left_3 = vec2(0.0);
    float right_3 = 0.0;
    left_3 = left_2;
    right_3 = right_2;
    float _e5 = left_3.x;
    float _e6 = right_3;
    float _e9 = left_3.y;
    float _e10 = right_3;
    return vec2((_e5 + _e6), (_e9 + _e10));
}

vec2 operator_u002b_2(float left_4, vec2 right_4) {
    float left_5 = 0.0;
    vec2 right_5 = vec2(0.0);
    left_5 = left_4;
    right_5 = right_4;
    float _e5 = left_5;
    float _e6 = right_5.x;
    float _e9 = left_5;
    float _e10 = right_5.y;
    return vec2((_e5 + _e6), (_e9 + _e10));
}

float3_ operator_u002b_3(float3_ left_6, float3_ right_6) {
    float3_ left_7 = float3_(0.0, 0.0, 0.0);
    float3_ right_7 = float3_(0.0, 0.0, 0.0);
    left_7 = left_6;
    right_7 = right_6;
    float _e6 = left_7.x;
    float _e7 = right_7.x;
    float _e11 = left_7.y;
    float _e12 = right_7.y;
    float _e16 = left_7.z;
    float _e17 = right_7.z;
    return float3_((_e6 + _e7), (_e11 + _e12), (_e16 + _e17));
}

float3_ operator_u002b_4(float3_ left_8, float right_8) {
    float3_ left_9 = float3_(0.0, 0.0, 0.0);
    float right_9 = 0.0;
    left_9 = left_8;
    right_9 = right_8;
    float _e5 = left_9.x;
    float _e6 = right_9;
    float _e9 = left_9.y;
    float _e10 = right_9;
    float _e13 = left_9.z;
    float _e14 = right_9;
    return float3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

float3_ operator_u002b_5(float left_10, float3_ right_10) {
    float left_11 = 0.0;
    float3_ right_11 = float3_(0.0, 0.0, 0.0);
    left_11 = left_10;
    right_11 = right_10;
    float _e5 = left_11;
    float _e6 = right_11.x;
    float _e9 = left_11;
    float _e10 = right_11.y;
    float _e13 = left_11;
    float _e14 = right_11.z;
    return float3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

vec4 operator_u002b_6(vec4 left_12, vec4 right_12) {
    vec4 left_13 = vec4(0.0);
    vec4 right_13 = vec4(0.0);
    left_13 = left_12;
    right_13 = right_12;
    float _e6 = left_13.x;
    float _e7 = right_13.x;
    float _e11 = left_13.y;
    float _e12 = right_13.y;
    float _e16 = left_13.z;
    float _e17 = right_13.z;
    float _e21 = left_13.w;
    float _e22 = right_13.w;
    return vec4((_e6 + _e7), (_e11 + _e12), (_e16 + _e17), (_e21 + _e22));
}

vec4 operator_u002b_7(vec4 left_14, float right_14) {
    vec4 left_15 = vec4(0.0);
    float right_15 = 0.0;
    left_15 = left_14;
    right_15 = right_14;
    float _e5 = left_15.x;
    float _e6 = right_15;
    float _e9 = left_15.y;
    float _e10 = right_15;
    float _e13 = left_15.z;
    float _e14 = right_15;
    float _e17 = left_15.w;
    float _e18 = right_15;
    return vec4((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

vec4 operator_u002b_8(float left_16, vec4 right_16) {
    float left_17 = 0.0;
    vec4 right_17 = vec4(0.0);
    left_17 = left_16;
    right_17 = right_16;
    float _e5 = left_17;
    float _e6 = right_17.x;
    float _e9 = left_17;
    float _e10 = right_17.y;
    float _e13 = left_17;
    float _e14 = right_17.z;
    float _e17 = left_17;
    float _e18 = right_17.w;
    return vec4((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

ivec2 operator_u002b_9(ivec2 left_18, ivec2 right_18) {
    ivec2 left_19 = ivec2(0);
    ivec2 right_19 = ivec2(0);
    left_19 = left_18;
    right_19 = right_18;
    int _e6 = left_19.x;
    int _e7 = right_19.x;
    int _e11 = left_19.y;
    int _e12 = right_19.y;
    return ivec2((_e6 + _e7), (_e11 + _e12));
}

ivec2 operator_u002b_10(ivec2 left_20, int right_20) {
    ivec2 left_21 = ivec2(0);
    int right_21 = 0;
    left_21 = left_20;
    right_21 = right_20;
    int _e5 = left_21.x;
    int _e6 = right_21;
    int _e9 = left_21.y;
    int _e10 = right_21;
    return ivec2((_e5 + _e6), (_e9 + _e10));
}

ivec2 operator_u002b_11(int left_22, ivec2 right_22) {
    int left_23 = 0;
    ivec2 right_23 = ivec2(0);
    left_23 = left_22;
    right_23 = right_22;
    int _e5 = left_23;
    int _e6 = right_23.x;
    int _e9 = left_23;
    int _e10 = right_23.y;
    return ivec2((_e5 + _e6), (_e9 + _e10));
}

int3_ operator_u002b_12(int3_ left_24, int3_ right_24) {
    int3_ left_25 = int3_(0, 0, 0);
    int3_ right_25 = int3_(0, 0, 0);
    left_25 = left_24;
    right_25 = right_24;
    int _e6 = left_25.x;
    int _e7 = right_25.x;
    int _e11 = left_25.y;
    int _e12 = right_25.y;
    int _e16 = left_25.z;
    int _e17 = right_25.z;
    return int3_((_e6 + _e7), (_e11 + _e12), (_e16 + _e17));
}

int3_ operator_u002b_13(int3_ left_26, int right_26) {
    int3_ left_27 = int3_(0, 0, 0);
    int right_27 = 0;
    left_27 = left_26;
    right_27 = right_26;
    int _e5 = left_27.x;
    int _e6 = right_27;
    int _e9 = left_27.y;
    int _e10 = right_27;
    int _e13 = left_27.z;
    int _e14 = right_27;
    return int3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

int3_ operator_u002b_14(int left_28, int3_ right_28) {
    int left_29 = 0;
    int3_ right_29 = int3_(0, 0, 0);
    left_29 = left_28;
    right_29 = right_28;
    int _e5 = left_29;
    int _e6 = right_29.x;
    int _e9 = left_29;
    int _e10 = right_29.y;
    int _e13 = left_29;
    int _e14 = right_29.z;
    return int3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

ivec4 operator_u002b_15(ivec4 left_30, ivec4 right_30) {
    ivec4 left_31 = ivec4(0);
    ivec4 right_31 = ivec4(0);
    left_31 = left_30;
    right_31 = right_30;
    int _e6 = left_31.x;
    int _e7 = right_31.x;
    int _e11 = left_31.y;
    int _e12 = right_31.y;
    int _e16 = left_31.z;
    int _e17 = right_31.z;
    int _e21 = left_31.w;
    int _e22 = right_31.w;
    return ivec4((_e6 + _e7), (_e11 + _e12), (_e16 + _e17), (_e21 + _e22));
}

ivec4 operator_u002b_16(ivec4 left_32, int right_32) {
    ivec4 left_33 = ivec4(0);
    int right_33 = 0;
    left_33 = left_32;
    right_33 = right_32;
    int _e5 = left_33.x;
    int _e6 = right_33;
    int _e9 = left_33.y;
    int _e10 = right_33;
    int _e13 = left_33.z;
    int _e14 = right_33;
    int _e17 = left_33.w;
    int _e18 = right_33;
    return ivec4((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

ivec4 operator_u002b_17(int left_34, ivec4 right_34) {
    int left_35 = 0;
    ivec4 right_35 = ivec4(0);
    left_35 = left_34;
    right_35 = right_34;
    int _e5 = left_35;
    int _e6 = right_35.x;
    int _e9 = left_35;
    int _e10 = right_35.y;
    int _e13 = left_35;
    int _e14 = right_35.z;
    int _e17 = left_35;
    int _e18 = right_35.w;
    return ivec4((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

uvec2 operator_u002b_18(uvec2 left_36, uvec2 right_36) {
    uvec2 left_37 = uvec2(0u);
    uvec2 right_37 = uvec2(0u);
    left_37 = left_36;
    right_37 = right_36;
    uint _e6 = left_37.x;
    uint _e7 = right_37.x;
    uint _e11 = left_37.y;
    uint _e12 = right_37.y;
    return uvec2((_e6 + _e7), (_e11 + _e12));
}

uvec2 operator_u002b_19(uvec2 left_38, uint right_38) {
    uvec2 left_39 = uvec2(0u);
    uint right_39 = 0u;
    left_39 = left_38;
    right_39 = right_38;
    uint _e5 = left_39.x;
    uint _e6 = right_39;
    uint _e9 = left_39.y;
    uint _e10 = right_39;
    return uvec2((_e5 + _e6), (_e9 + _e10));
}

uvec2 operator_u002b_20(uint left_40, uvec2 right_40) {
    uint left_41 = 0u;
    uvec2 right_41 = uvec2(0u);
    left_41 = left_40;
    right_41 = right_40;
    uint _e5 = left_41;
    uint _e6 = right_41.x;
    uint _e9 = left_41;
    uint _e10 = right_41.y;
    return uvec2((_e5 + _e6), (_e9 + _e10));
}

uint3_ operator_u002b_21(uint3_ left_42, uint3_ right_42) {
    uint3_ left_43 = uint3_(0u, 0u, 0u);
    uint3_ right_43 = uint3_(0u, 0u, 0u);
    left_43 = left_42;
    right_43 = right_42;
    uint _e6 = left_43.x;
    uint _e7 = right_43.x;
    uint _e11 = left_43.y;
    uint _e12 = right_43.y;
    uint _e16 = left_43.z;
    uint _e17 = right_43.z;
    return uint3_((_e6 + _e7), (_e11 + _e12), (_e16 + _e17));
}

uint3_ operator_u002b_22(uint3_ left_44, uint right_44) {
    uint3_ left_45 = uint3_(0u, 0u, 0u);
    uint right_45 = 0u;
    left_45 = left_44;
    right_45 = right_44;
    uint _e5 = left_45.x;
    uint _e6 = right_45;
    uint _e9 = left_45.y;
    uint _e10 = right_45;
    uint _e13 = left_45.z;
    uint _e14 = right_45;
    return uint3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

uint3_ operator_u002b_23(uint left_46, uint3_ right_46) {
    uint left_47 = 0u;
    uint3_ right_47 = uint3_(0u, 0u, 0u);
    left_47 = left_46;
    right_47 = right_46;
    uint _e5 = left_47;
    uint _e6 = right_47.x;
    uint _e9 = left_47;
    uint _e10 = right_47.y;
    uint _e13 = left_47;
    uint _e14 = right_47.z;
    return uint3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
}

uvec4 operator_u002b_24(uvec4 left_48, uvec4 right_48) {
    uvec4 left_49 = uvec4(0u);
    uvec4 right_49 = uvec4(0u);
    left_49 = left_48;
    right_49 = right_48;
    uint _e6 = left_49.x;
    uint _e7 = right_49.x;
    uint _e11 = left_49.y;
    uint _e12 = right_49.y;
    uint _e16 = left_49.z;
    uint _e17 = right_49.z;
    uint _e21 = left_49.w;
    uint _e22 = right_49.w;
    return uvec4((_e6 + _e7), (_e11 + _e12), (_e16 + _e17), (_e21 + _e22));
}

uvec4 operator_u002b_25(uvec4 left_50, uint right_50) {
    uvec4 left_51 = uvec4(0u);
    uint right_51 = 0u;
    left_51 = left_50;
    right_51 = right_50;
    uint _e5 = left_51.x;
    uint _e6 = right_51;
    uint _e9 = left_51.y;
    uint _e10 = right_51;
    uint _e13 = left_51.z;
    uint _e14 = right_51;
    uint _e17 = left_51.w;
    uint _e18 = right_51;
    return uvec4((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

uvec4 operator_u002b_26(uint left_52, uvec4 right_52) {
    uint left_53 = 0u;
    uvec4 right_53 = uvec4(0u);
    left_53 = left_52;
    right_53 = right_52;
    uint _e5 = left_53;
    uint _e6 = right_53.x;
    uint _e9 = left_53;
    uint _e10 = right_53.y;
    uint _e13 = left_53;
    uint _e14 = right_53.z;
    uint _e17 = left_53;
    uint _e18 = right_53.w;
    return uvec4((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

void main() {
    vec2 float_2_left = vec2(0.0);
    vec2 float_2_right = vec2(0.0);
    vec2 float_2_vectors = vec2(0.0);
    vec2 float_2_vector_scalar = vec2(0.0);
    vec2 float_2_scalar_vector = vec2(0.0);
    float3_ float_3_left = float3_(0.0, 0.0, 0.0);
    float3_ float_3_right = float3_(0.0, 0.0, 0.0);
    float3_ float_3_vectors = float3_(0.0, 0.0, 0.0);
    float3_ float_3_vector_scalar = float3_(0.0, 0.0, 0.0);
    float3_ float_3_scalar_vector = float3_(0.0, 0.0, 0.0);
    vec4 float_4_left = vec4(0.0);
    vec4 float_4_right = vec4(0.0);
    vec4 float_4_vectors = vec4(0.0);
    vec4 float_4_vector_scalar = vec4(0.0);
    vec4 float_4_scalar_vector = vec4(0.0);
    ivec2 int_2_left = ivec2(0);
    ivec2 int_2_right = ivec2(0);
    ivec2 int_2_vectors = ivec2(0);
    ivec2 int_2_vector_scalar = ivec2(0);
    ivec2 int_2_scalar_vector = ivec2(0);
    int3_ int_3_left = int3_(0, 0, 0);
    int3_ int_3_right = int3_(0, 0, 0);
    int3_ int_3_vectors = int3_(0, 0, 0);
    int3_ int_3_vector_scalar = int3_(0, 0, 0);
    int3_ int_3_scalar_vector = int3_(0, 0, 0);
    ivec4 int_4_left = ivec4(0);
    ivec4 int_4_right = ivec4(0);
    ivec4 int_4_vectors = ivec4(0);
    ivec4 int_4_vector_scalar = ivec4(0);
    ivec4 int_4_scalar_vector = ivec4(0);
    uvec2 uint_2_left = uvec2(0u);
    uvec2 uint_2_right = uvec2(0u);
    uvec2 uint_2_vectors = uvec2(0u);
    uvec2 uint_2_vector_scalar = uvec2(0u);
    uvec2 uint_2_scalar_vector = uvec2(0u);
    uint3_ uint_3_left = uint3_(0u, 0u, 0u);
    uint3_ uint_3_right = uint3_(0u, 0u, 0u);
    uint3_ uint_3_vectors = uint3_(0u, 0u, 0u);
    uint3_ uint_3_vector_scalar = uint3_(0u, 0u, 0u);
    uint3_ uint_3_scalar_vector = uint3_(0u, 0u, 0u);
    uvec4 uint_4_left = uvec4(0u);
    uvec4 uint_4_right = uvec4(0u);
    uvec4 uint_4_vectors = uvec4(0u);
    uvec4 uint_4_vector_scalar = uvec4(0u);
    uvec4 uint_4_scalar_vector = uvec4(0u);
    float_2_left = vec2(1.25, -(2.5));
    float_2_right = vec2(0.5, 1.5);
    vec2 _e11 = float_2_left;
    vec2 _e12 = float_2_right;
    vec2 _e13 = operator_u002b(_e11, _e12);
    float_2_vectors = _e13;
    vec2 _e15 = float_2_left;
    vec2 _e17 = operator_u002b_1(_e15, 2.0);
    float_2_vector_scalar = _e17;
    vec2 _e20 = float_2_right;
    vec2 _e21 = operator_u002b_2(2.0, _e20);
    float_2_scalar_vector = _e21;
    float _e25 = float_2_vectors.x;
    _group_0_binding_0_cs[0] = floatBitsToUint(_e25);
    float _e30 = float_2_vectors.y;
    _group_0_binding_0_cs[1] = floatBitsToUint(_e30);
    float _e35 = float_2_vector_scalar.x;
    _group_0_binding_0_cs[2] = floatBitsToUint(_e35);
    float _e40 = float_2_vector_scalar.y;
    _group_0_binding_0_cs[3] = floatBitsToUint(_e40);
    float _e45 = float_2_scalar_vector.x;
    _group_0_binding_0_cs[4] = floatBitsToUint(_e45);
    float _e50 = float_2_scalar_vector.y;
    _group_0_binding_0_cs[5] = floatBitsToUint(_e50);
    float_3_left = float3_(1.25, -(2.5), 3.75);
    float_3_right = float3_(0.5, 1.5, -(2.0));
    float3_ _e65 = float_3_left;
    float3_ _e66 = float_3_right;
    float3_ _e67 = operator_u002b_3(_e65, _e66);
    float_3_vectors = _e67;
    float3_ _e69 = float_3_left;
    float3_ _e71 = operator_u002b_4(_e69, 2.0);
    float_3_vector_scalar = _e71;
    float3_ _e74 = float_3_right;
    float3_ _e75 = operator_u002b_5(2.0, _e74);
    float_3_scalar_vector = _e75;
    float _e79 = float_3_vectors.x;
    _group_0_binding_0_cs[6] = floatBitsToUint(_e79);
    float _e84 = float_3_vectors.y;
    _group_0_binding_0_cs[7] = floatBitsToUint(_e84);
    float _e89 = float_3_vectors.z;
    _group_0_binding_0_cs[8] = floatBitsToUint(_e89);
    float _e94 = float_3_vector_scalar.x;
    _group_0_binding_0_cs[9] = floatBitsToUint(_e94);
    float _e99 = float_3_vector_scalar.y;
    _group_0_binding_0_cs[10] = floatBitsToUint(_e99);
    float _e104 = float_3_vector_scalar.z;
    _group_0_binding_0_cs[11] = floatBitsToUint(_e104);
    float _e109 = float_3_scalar_vector.x;
    _group_0_binding_0_cs[12] = floatBitsToUint(_e109);
    float _e114 = float_3_scalar_vector.y;
    _group_0_binding_0_cs[13] = floatBitsToUint(_e114);
    float _e119 = float_3_scalar_vector.z;
    _group_0_binding_0_cs[14] = floatBitsToUint(_e119);
    float_4_left = vec4(1.25, -(2.5), 3.75, -(4.0));
    float_4_right = vec4(0.5, 1.5, -(2.0), 0.25);
    vec4 _e137 = float_4_left;
    vec4 _e138 = float_4_right;
    vec4 _e139 = operator_u002b_6(_e137, _e138);
    float_4_vectors = _e139;
    vec4 _e141 = float_4_left;
    vec4 _e143 = operator_u002b_7(_e141, 2.0);
    float_4_vector_scalar = _e143;
    vec4 _e146 = float_4_right;
    vec4 _e147 = operator_u002b_8(2.0, _e146);
    float_4_scalar_vector = _e147;
    float _e151 = float_4_vectors.x;
    _group_0_binding_0_cs[15] = floatBitsToUint(_e151);
    float _e156 = float_4_vectors.y;
    _group_0_binding_0_cs[16] = floatBitsToUint(_e156);
    float _e161 = float_4_vectors.z;
    _group_0_binding_0_cs[17] = floatBitsToUint(_e161);
    float _e166 = float_4_vectors.w;
    _group_0_binding_0_cs[18] = floatBitsToUint(_e166);
    float _e171 = float_4_vector_scalar.x;
    _group_0_binding_0_cs[19] = floatBitsToUint(_e171);
    float _e176 = float_4_vector_scalar.y;
    _group_0_binding_0_cs[20] = floatBitsToUint(_e176);
    float _e181 = float_4_vector_scalar.z;
    _group_0_binding_0_cs[21] = floatBitsToUint(_e181);
    float _e186 = float_4_vector_scalar.w;
    _group_0_binding_0_cs[22] = floatBitsToUint(_e186);
    float _e191 = float_4_scalar_vector.x;
    _group_0_binding_0_cs[23] = floatBitsToUint(_e191);
    float _e196 = float_4_scalar_vector.y;
    _group_0_binding_0_cs[24] = floatBitsToUint(_e196);
    float _e201 = float_4_scalar_vector.z;
    _group_0_binding_0_cs[25] = floatBitsToUint(_e201);
    float _e206 = float_4_scalar_vector.w;
    _group_0_binding_0_cs[26] = floatBitsToUint(_e206);
    int_2_left = ivec2(1, -(2));
    int_2_right = ivec2(5, 6);
    ivec2 _e218 = int_2_left;
    ivec2 _e219 = int_2_right;
    ivec2 _e220 = operator_u002b_9(_e218, _e219);
    int_2_vectors = _e220;
    ivec2 _e222 = int_2_left;
    ivec2 _e224 = operator_u002b_10(_e222, 3);
    int_2_vector_scalar = _e224;
    ivec2 _e227 = int_2_right;
    ivec2 _e228 = operator_u002b_11(3, _e227);
    int_2_scalar_vector = _e228;
    int _e232 = int_2_vectors.x;
    _group_0_binding_0_cs[27] = uint(_e232);
    int _e237 = int_2_vectors.y;
    _group_0_binding_0_cs[28] = uint(_e237);
    int _e242 = int_2_vector_scalar.x;
    _group_0_binding_0_cs[29] = uint(_e242);
    int _e247 = int_2_vector_scalar.y;
    _group_0_binding_0_cs[30] = uint(_e247);
    int _e252 = int_2_scalar_vector.x;
    _group_0_binding_0_cs[31] = uint(_e252);
    int _e257 = int_2_scalar_vector.y;
    _group_0_binding_0_cs[32] = uint(_e257);
    int_3_left = int3_(1, -(2), 3);
    int_3_right = int3_(5, 6, -(7));
    int3_ _e272 = int_3_left;
    int3_ _e273 = int_3_right;
    int3_ _e274 = operator_u002b_12(_e272, _e273);
    int_3_vectors = _e274;
    int3_ _e276 = int_3_left;
    int3_ _e278 = operator_u002b_13(_e276, 3);
    int_3_vector_scalar = _e278;
    int3_ _e281 = int_3_right;
    int3_ _e282 = operator_u002b_14(3, _e281);
    int_3_scalar_vector = _e282;
    int _e286 = int_3_vectors.x;
    _group_0_binding_0_cs[33] = uint(_e286);
    int _e291 = int_3_vectors.y;
    _group_0_binding_0_cs[34] = uint(_e291);
    int _e296 = int_3_vectors.z;
    _group_0_binding_0_cs[35] = uint(_e296);
    int _e301 = int_3_vector_scalar.x;
    _group_0_binding_0_cs[36] = uint(_e301);
    int _e306 = int_3_vector_scalar.y;
    _group_0_binding_0_cs[37] = uint(_e306);
    int _e311 = int_3_vector_scalar.z;
    _group_0_binding_0_cs[38] = uint(_e311);
    int _e316 = int_3_scalar_vector.x;
    _group_0_binding_0_cs[39] = uint(_e316);
    int _e321 = int_3_scalar_vector.y;
    _group_0_binding_0_cs[40] = uint(_e321);
    int _e326 = int_3_scalar_vector.z;
    _group_0_binding_0_cs[41] = uint(_e326);
    int_4_left = ivec4(1, -(2), 3, -(4));
    int_4_right = ivec4(5, 6, -(7), 8);
    ivec4 _e344 = int_4_left;
    ivec4 _e345 = int_4_right;
    ivec4 _e346 = operator_u002b_15(_e344, _e345);
    int_4_vectors = _e346;
    ivec4 _e348 = int_4_left;
    ivec4 _e350 = operator_u002b_16(_e348, 3);
    int_4_vector_scalar = _e350;
    ivec4 _e353 = int_4_right;
    ivec4 _e354 = operator_u002b_17(3, _e353);
    int_4_scalar_vector = _e354;
    int _e358 = int_4_vectors.x;
    _group_0_binding_0_cs[42] = uint(_e358);
    int _e363 = int_4_vectors.y;
    _group_0_binding_0_cs[43] = uint(_e363);
    int _e368 = int_4_vectors.z;
    _group_0_binding_0_cs[44] = uint(_e368);
    int _e373 = int_4_vectors.w;
    _group_0_binding_0_cs[45] = uint(_e373);
    int _e378 = int_4_vector_scalar.x;
    _group_0_binding_0_cs[46] = uint(_e378);
    int _e383 = int_4_vector_scalar.y;
    _group_0_binding_0_cs[47] = uint(_e383);
    int _e388 = int_4_vector_scalar.z;
    _group_0_binding_0_cs[48] = uint(_e388);
    int _e393 = int_4_vector_scalar.w;
    _group_0_binding_0_cs[49] = uint(_e393);
    int _e398 = int_4_scalar_vector.x;
    _group_0_binding_0_cs[50] = uint(_e398);
    int _e403 = int_4_scalar_vector.y;
    _group_0_binding_0_cs[51] = uint(_e403);
    int _e408 = int_4_scalar_vector.z;
    _group_0_binding_0_cs[52] = uint(_e408);
    int _e413 = int_4_scalar_vector.w;
    _group_0_binding_0_cs[53] = uint(_e413);
    uint_2_left = uvec2(4294967294u, 2147483649u);
    uint_2_right = uvec2(5u, 2u);
    uvec2 _e424 = uint_2_left;
    uvec2 _e425 = uint_2_right;
    uvec2 _e426 = operator_u002b_18(_e424, _e425);
    uint_2_vectors = _e426;
    uvec2 _e428 = uint_2_left;
    uvec2 _e430 = operator_u002b_19(_e428, 4u);
    uint_2_vector_scalar = _e430;
    uvec2 _e433 = uint_2_right;
    uvec2 _e434 = operator_u002b_20(4u, _e433);
    uint_2_scalar_vector = _e434;
    uint _e438 = uint_2_vectors.x;
    _group_0_binding_0_cs[54] = _e438;
    uint _e442 = uint_2_vectors.y;
    _group_0_binding_0_cs[55] = _e442;
    uint _e446 = uint_2_vector_scalar.x;
    _group_0_binding_0_cs[56] = _e446;
    uint _e450 = uint_2_vector_scalar.y;
    _group_0_binding_0_cs[57] = _e450;
    uint _e454 = uint_2_scalar_vector.x;
    _group_0_binding_0_cs[58] = _e454;
    uint _e458 = uint_2_scalar_vector.y;
    _group_0_binding_0_cs[59] = _e458;
    uint_3_left = uint3_(4294967294u, 2147483649u, 3u);
    uint_3_right = uint3_(5u, 2u, 4294967293u);
    uint3_ _e470 = uint_3_left;
    uint3_ _e471 = uint_3_right;
    uint3_ _e472 = operator_u002b_21(_e470, _e471);
    uint_3_vectors = _e472;
    uint3_ _e474 = uint_3_left;
    uint3_ _e476 = operator_u002b_22(_e474, 4u);
    uint_3_vector_scalar = _e476;
    uint3_ _e479 = uint_3_right;
    uint3_ _e480 = operator_u002b_23(4u, _e479);
    uint_3_scalar_vector = _e480;
    uint _e484 = uint_3_vectors.x;
    _group_0_binding_0_cs[60] = _e484;
    uint _e488 = uint_3_vectors.y;
    _group_0_binding_0_cs[61] = _e488;
    uint _e492 = uint_3_vectors.z;
    _group_0_binding_0_cs[62] = _e492;
    uint _e496 = uint_3_vector_scalar.x;
    _group_0_binding_0_cs[63] = _e496;
    uint _e500 = uint_3_vector_scalar.y;
    _group_0_binding_0_cs[64] = _e500;
    uint _e504 = uint_3_vector_scalar.z;
    _group_0_binding_0_cs[65] = _e504;
    uint _e508 = uint_3_scalar_vector.x;
    _group_0_binding_0_cs[66] = _e508;
    uint _e512 = uint_3_scalar_vector.y;
    _group_0_binding_0_cs[67] = _e512;
    uint _e516 = uint_3_scalar_vector.z;
    _group_0_binding_0_cs[68] = _e516;
    uint_4_left = uvec4(4294967294u, 2147483649u, 3u, 4294967295u);
    uint_4_right = uvec4(5u, 2u, 4294967293u, 1u);
    uvec4 _e530 = uint_4_left;
    uvec4 _e531 = uint_4_right;
    uvec4 _e532 = operator_u002b_24(_e530, _e531);
    uint_4_vectors = _e532;
    uvec4 _e534 = uint_4_left;
    uvec4 _e536 = operator_u002b_25(_e534, 4u);
    uint_4_vector_scalar = _e536;
    uvec4 _e539 = uint_4_right;
    uvec4 _e540 = operator_u002b_26(4u, _e539);
    uint_4_scalar_vector = _e540;
    uint _e544 = uint_4_vectors.x;
    _group_0_binding_0_cs[69] = _e544;
    uint _e548 = uint_4_vectors.y;
    _group_0_binding_0_cs[70] = _e548;
    uint _e552 = uint_4_vectors.z;
    _group_0_binding_0_cs[71] = _e552;
    uint _e556 = uint_4_vectors.w;
    _group_0_binding_0_cs[72] = _e556;
    uint _e560 = uint_4_vector_scalar.x;
    _group_0_binding_0_cs[73] = _e560;
    uint _e564 = uint_4_vector_scalar.y;
    _group_0_binding_0_cs[74] = _e564;
    uint _e568 = uint_4_vector_scalar.z;
    _group_0_binding_0_cs[75] = _e568;
    uint _e572 = uint_4_vector_scalar.w;
    _group_0_binding_0_cs[76] = _e572;
    uint _e576 = uint_4_scalar_vector.x;
    _group_0_binding_0_cs[77] = _e576;
    uint _e580 = uint_4_scalar_vector.y;
    _group_0_binding_0_cs[78] = _e580;
    uint _e584 = uint_4_scalar_vector.z;
    _group_0_binding_0_cs[79] = _e584;
    uint _e588 = uint_4_scalar_vector.w;
    _group_0_binding_0_cs[80] = _e588;
}

