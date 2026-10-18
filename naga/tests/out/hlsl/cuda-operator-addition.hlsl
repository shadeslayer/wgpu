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

RWByteAddressBuffer out_ : register(u0);

float2 operator_u002b(float2 left, float2 right)
{
    float2 left_1 = (float2)0;
    float2 right_1 = (float2)0;

    left_1 = left;
    right_1 = right;
    float _e6 = left_1.x;
    float _e7 = right_1.x;
    float _e11 = left_1.y;
    float _e12 = right_1.y;
    return float2((_e6 + _e7), (_e11 + _e12));
}

float2 operator_u002b_1(float2 left_2, float right_2)
{
    float2 left_3 = (float2)0;
    float right_3 = (float)0;

    left_3 = left_2;
    right_3 = right_2;
    float _e5 = left_3.x;
    float _e6 = right_3;
    float _e9 = left_3.y;
    float _e10 = right_3;
    return float2((_e5 + _e6), (_e9 + _e10));
}

float2 operator_u002b_2(float left_4, float2 right_4)
{
    float left_5 = (float)0;
    float2 right_5 = (float2)0;

    left_5 = left_4;
    right_5 = right_4;
    float _e5 = left_5;
    float _e6 = right_5.x;
    float _e9 = left_5;
    float _e10 = right_5.y;
    return float2((_e5 + _e6), (_e9 + _e10));
}

float3_ Constructfloat3_(float arg0, float arg1, float arg2) {
    float3_ ret = (float3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

float3_ operator_u002b_3(float3_ left_6, float3_ right_6)
{
    float3_ left_7 = (float3_)0;
    float3_ right_7 = (float3_)0;

    left_7 = left_6;
    right_7 = right_6;
    float _e6 = left_7.x;
    float _e7 = right_7.x;
    float _e11 = left_7.y;
    float _e12 = right_7.y;
    float _e16 = left_7.z;
    float _e17 = right_7.z;
    const float3_ float3_1 = Constructfloat3_((_e6 + _e7), (_e11 + _e12), (_e16 + _e17));
    return float3_1;
}

float3_ operator_u002b_4(float3_ left_8, float right_8)
{
    float3_ left_9 = (float3_)0;
    float right_9 = (float)0;

    left_9 = left_8;
    right_9 = right_8;
    float _e5 = left_9.x;
    float _e6 = right_9;
    float _e9 = left_9.y;
    float _e10 = right_9;
    float _e13 = left_9.z;
    float _e14 = right_9;
    const float3_ float3_2 = Constructfloat3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
    return float3_2;
}

float3_ operator_u002b_5(float left_10, float3_ right_10)
{
    float left_11 = (float)0;
    float3_ right_11 = (float3_)0;

    left_11 = left_10;
    right_11 = right_10;
    float _e5 = left_11;
    float _e6 = right_11.x;
    float _e9 = left_11;
    float _e10 = right_11.y;
    float _e13 = left_11;
    float _e14 = right_11.z;
    const float3_ float3_3 = Constructfloat3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
    return float3_3;
}

float4 operator_u002b_6(float4 left_12, float4 right_12)
{
    float4 left_13 = (float4)0;
    float4 right_13 = (float4)0;

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
    return float4((_e6 + _e7), (_e11 + _e12), (_e16 + _e17), (_e21 + _e22));
}

float4 operator_u002b_7(float4 left_14, float right_14)
{
    float4 left_15 = (float4)0;
    float right_15 = (float)0;

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
    return float4((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

float4 operator_u002b_8(float left_16, float4 right_16)
{
    float left_17 = (float)0;
    float4 right_17 = (float4)0;

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
    return float4((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

int2 operator_u002b_9(int2 left_18, int2 right_18)
{
    int2 left_19 = (int2)0;
    int2 right_19 = (int2)0;

    left_19 = left_18;
    right_19 = right_18;
    int _e6 = left_19.x;
    int _e7 = right_19.x;
    int _e11 = left_19.y;
    int _e12 = right_19.y;
    return int2(asint(asuint(_e6) + asuint(_e7)), asint(asuint(_e11) + asuint(_e12)));
}

int2 operator_u002b_10(int2 left_20, int right_20)
{
    int2 left_21 = (int2)0;
    int right_21 = (int)0;

    left_21 = left_20;
    right_21 = right_20;
    int _e5 = left_21.x;
    int _e6 = right_21;
    int _e9 = left_21.y;
    int _e10 = right_21;
    return int2(asint(asuint(_e5) + asuint(_e6)), asint(asuint(_e9) + asuint(_e10)));
}

int2 operator_u002b_11(int left_22, int2 right_22)
{
    int left_23 = (int)0;
    int2 right_23 = (int2)0;

    left_23 = left_22;
    right_23 = right_22;
    int _e5 = left_23;
    int _e6 = right_23.x;
    int _e9 = left_23;
    int _e10 = right_23.y;
    return int2(asint(asuint(_e5) + asuint(_e6)), asint(asuint(_e9) + asuint(_e10)));
}

int3_ Constructint3_(int arg0, int arg1, int arg2) {
    int3_ ret = (int3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

int3_ operator_u002b_12(int3_ left_24, int3_ right_24)
{
    int3_ left_25 = (int3_)0;
    int3_ right_25 = (int3_)0;

    left_25 = left_24;
    right_25 = right_24;
    int _e6 = left_25.x;
    int _e7 = right_25.x;
    int _e11 = left_25.y;
    int _e12 = right_25.y;
    int _e16 = left_25.z;
    int _e17 = right_25.z;
    const int3_ int3_1 = Constructint3_(asint(asuint(_e6) + asuint(_e7)), asint(asuint(_e11) + asuint(_e12)), asint(asuint(_e16) + asuint(_e17)));
    return int3_1;
}

int3_ operator_u002b_13(int3_ left_26, int right_26)
{
    int3_ left_27 = (int3_)0;
    int right_27 = (int)0;

    left_27 = left_26;
    right_27 = right_26;
    int _e5 = left_27.x;
    int _e6 = right_27;
    int _e9 = left_27.y;
    int _e10 = right_27;
    int _e13 = left_27.z;
    int _e14 = right_27;
    const int3_ int3_2 = Constructint3_(asint(asuint(_e5) + asuint(_e6)), asint(asuint(_e9) + asuint(_e10)), asint(asuint(_e13) + asuint(_e14)));
    return int3_2;
}

int3_ operator_u002b_14(int left_28, int3_ right_28)
{
    int left_29 = (int)0;
    int3_ right_29 = (int3_)0;

    left_29 = left_28;
    right_29 = right_28;
    int _e5 = left_29;
    int _e6 = right_29.x;
    int _e9 = left_29;
    int _e10 = right_29.y;
    int _e13 = left_29;
    int _e14 = right_29.z;
    const int3_ int3_3 = Constructint3_(asint(asuint(_e5) + asuint(_e6)), asint(asuint(_e9) + asuint(_e10)), asint(asuint(_e13) + asuint(_e14)));
    return int3_3;
}

int4 operator_u002b_15(int4 left_30, int4 right_30)
{
    int4 left_31 = (int4)0;
    int4 right_31 = (int4)0;

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
    return int4(asint(asuint(_e6) + asuint(_e7)), asint(asuint(_e11) + asuint(_e12)), asint(asuint(_e16) + asuint(_e17)), asint(asuint(_e21) + asuint(_e22)));
}

int4 operator_u002b_16(int4 left_32, int right_32)
{
    int4 left_33 = (int4)0;
    int right_33 = (int)0;

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
    return int4(asint(asuint(_e5) + asuint(_e6)), asint(asuint(_e9) + asuint(_e10)), asint(asuint(_e13) + asuint(_e14)), asint(asuint(_e17) + asuint(_e18)));
}

int4 operator_u002b_17(int left_34, int4 right_34)
{
    int left_35 = (int)0;
    int4 right_35 = (int4)0;

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
    return int4(asint(asuint(_e5) + asuint(_e6)), asint(asuint(_e9) + asuint(_e10)), asint(asuint(_e13) + asuint(_e14)), asint(asuint(_e17) + asuint(_e18)));
}

uint2 operator_u002b_18(uint2 left_36, uint2 right_36)
{
    uint2 left_37 = (uint2)0;
    uint2 right_37 = (uint2)0;

    left_37 = left_36;
    right_37 = right_36;
    uint _e6 = left_37.x;
    uint _e7 = right_37.x;
    uint _e11 = left_37.y;
    uint _e12 = right_37.y;
    return uint2((_e6 + _e7), (_e11 + _e12));
}

uint2 operator_u002b_19(uint2 left_38, uint right_38)
{
    uint2 left_39 = (uint2)0;
    uint right_39 = (uint)0;

    left_39 = left_38;
    right_39 = right_38;
    uint _e5 = left_39.x;
    uint _e6 = right_39;
    uint _e9 = left_39.y;
    uint _e10 = right_39;
    return uint2((_e5 + _e6), (_e9 + _e10));
}

uint2 operator_u002b_20(uint left_40, uint2 right_40)
{
    uint left_41 = (uint)0;
    uint2 right_41 = (uint2)0;

    left_41 = left_40;
    right_41 = right_40;
    uint _e5 = left_41;
    uint _e6 = right_41.x;
    uint _e9 = left_41;
    uint _e10 = right_41.y;
    return uint2((_e5 + _e6), (_e9 + _e10));
}

uint3_ Constructuint3_(uint arg0, uint arg1, uint arg2) {
    uint3_ ret = (uint3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

uint3_ operator_u002b_21(uint3_ left_42, uint3_ right_42)
{
    uint3_ left_43 = (uint3_)0;
    uint3_ right_43 = (uint3_)0;

    left_43 = left_42;
    right_43 = right_42;
    uint _e6 = left_43.x;
    uint _e7 = right_43.x;
    uint _e11 = left_43.y;
    uint _e12 = right_43.y;
    uint _e16 = left_43.z;
    uint _e17 = right_43.z;
    const uint3_ uint3_1 = Constructuint3_((_e6 + _e7), (_e11 + _e12), (_e16 + _e17));
    return uint3_1;
}

uint3_ operator_u002b_22(uint3_ left_44, uint right_44)
{
    uint3_ left_45 = (uint3_)0;
    uint right_45 = (uint)0;

    left_45 = left_44;
    right_45 = right_44;
    uint _e5 = left_45.x;
    uint _e6 = right_45;
    uint _e9 = left_45.y;
    uint _e10 = right_45;
    uint _e13 = left_45.z;
    uint _e14 = right_45;
    const uint3_ uint3_2 = Constructuint3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
    return uint3_2;
}

uint3_ operator_u002b_23(uint left_46, uint3_ right_46)
{
    uint left_47 = (uint)0;
    uint3_ right_47 = (uint3_)0;

    left_47 = left_46;
    right_47 = right_46;
    uint _e5 = left_47;
    uint _e6 = right_47.x;
    uint _e9 = left_47;
    uint _e10 = right_47.y;
    uint _e13 = left_47;
    uint _e14 = right_47.z;
    const uint3_ uint3_3 = Constructuint3_((_e5 + _e6), (_e9 + _e10), (_e13 + _e14));
    return uint3_3;
}

uint4 operator_u002b_24(uint4 left_48, uint4 right_48)
{
    uint4 left_49 = (uint4)0;
    uint4 right_49 = (uint4)0;

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
    return uint4((_e6 + _e7), (_e11 + _e12), (_e16 + _e17), (_e21 + _e22));
}

uint4 operator_u002b_25(uint4 left_50, uint right_50)
{
    uint4 left_51 = (uint4)0;
    uint right_51 = (uint)0;

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
    return uint4((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

uint4 operator_u002b_26(uint left_52, uint4 right_52)
{
    uint left_53 = (uint)0;
    uint4 right_53 = (uint4)0;

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
    return uint4((_e5 + _e6), (_e9 + _e10), (_e13 + _e14), (_e17 + _e18));
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

[numthreads(1, 1, 1)]
void additions()
{
    float2 float_2_left = (float2)0;
    float2 float_2_right = (float2)0;
    float2 float_2_vectors = (float2)0;
    float2 float_2_vector_scalar = (float2)0;
    float2 float_2_scalar_vector = (float2)0;
    float3_ float_3_left = (float3_)0;
    float3_ float_3_right = (float3_)0;
    float3_ float_3_vectors = (float3_)0;
    float3_ float_3_vector_scalar = (float3_)0;
    float3_ float_3_scalar_vector = (float3_)0;
    float4 float_4_left = (float4)0;
    float4 float_4_right = (float4)0;
    float4 float_4_vectors = (float4)0;
    float4 float_4_vector_scalar = (float4)0;
    float4 float_4_scalar_vector = (float4)0;
    int2 int_2_left = (int2)0;
    int2 int_2_right = (int2)0;
    int2 int_2_vectors = (int2)0;
    int2 int_2_vector_scalar = (int2)0;
    int2 int_2_scalar_vector = (int2)0;
    int3_ int_3_left = (int3_)0;
    int3_ int_3_right = (int3_)0;
    int3_ int_3_vectors = (int3_)0;
    int3_ int_3_vector_scalar = (int3_)0;
    int3_ int_3_scalar_vector = (int3_)0;
    int4 int_4_left = (int4)0;
    int4 int_4_right = (int4)0;
    int4 int_4_vectors = (int4)0;
    int4 int_4_vector_scalar = (int4)0;
    int4 int_4_scalar_vector = (int4)0;
    uint2 uint_2_left = (uint2)0;
    uint2 uint_2_right = (uint2)0;
    uint2 uint_2_vectors = (uint2)0;
    uint2 uint_2_vector_scalar = (uint2)0;
    uint2 uint_2_scalar_vector = (uint2)0;
    uint3_ uint_3_left = (uint3_)0;
    uint3_ uint_3_right = (uint3_)0;
    uint3_ uint_3_vectors = (uint3_)0;
    uint3_ uint_3_vector_scalar = (uint3_)0;
    uint3_ uint_3_scalar_vector = (uint3_)0;
    uint4 uint_4_left = (uint4)0;
    uint4 uint_4_right = (uint4)0;
    uint4 uint_4_vectors = (uint4)0;
    uint4 uint_4_vector_scalar = (uint4)0;
    uint4 uint_4_scalar_vector = (uint4)0;

    float_2_left = float2(1.25, -(2.5));
    float_2_right = float2(0.5, 1.5);
    float2 _e11 = float_2_left;
    float2 _e12 = float_2_right;
    const float2 _e13 = operator_u002b(_e11, _e12);
    float_2_vectors = _e13;
    float2 _e15 = float_2_left;
    const float2 _e17 = operator_u002b_1(_e15, 2.0);
    float_2_vector_scalar = _e17;
    float2 _e20 = float_2_right;
    const float2 _e21 = operator_u002b_2(2.0, _e20);
    float_2_scalar_vector = _e21;
    float _e25 = float_2_vectors.x;
    out_.Store(int(0)*4, asuint(asuint(_e25)));
    float _e30 = float_2_vectors.y;
    out_.Store(int(1)*4, asuint(asuint(_e30)));
    float _e35 = float_2_vector_scalar.x;
    out_.Store(int(2)*4, asuint(asuint(_e35)));
    float _e40 = float_2_vector_scalar.y;
    out_.Store(int(3)*4, asuint(asuint(_e40)));
    float _e45 = float_2_scalar_vector.x;
    out_.Store(int(4)*4, asuint(asuint(_e45)));
    float _e50 = float_2_scalar_vector.y;
    out_.Store(int(5)*4, asuint(asuint(_e50)));
    float_3_left = Constructfloat3_(1.25, -(2.5), 3.75);
    float_3_right = Constructfloat3_(0.5, 1.5, -(2.0));
    float3_ _e65 = float_3_left;
    float3_ _e66 = float_3_right;
    const float3_ _e67 = operator_u002b_3(_e65, _e66);
    float_3_vectors = _e67;
    float3_ _e69 = float_3_left;
    const float3_ _e71 = operator_u002b_4(_e69, 2.0);
    float_3_vector_scalar = _e71;
    float3_ _e74 = float_3_right;
    const float3_ _e75 = operator_u002b_5(2.0, _e74);
    float_3_scalar_vector = _e75;
    float _e79 = float_3_vectors.x;
    out_.Store(int(6)*4, asuint(asuint(_e79)));
    float _e84 = float_3_vectors.y;
    out_.Store(int(7)*4, asuint(asuint(_e84)));
    float _e89 = float_3_vectors.z;
    out_.Store(int(8)*4, asuint(asuint(_e89)));
    float _e94 = float_3_vector_scalar.x;
    out_.Store(int(9)*4, asuint(asuint(_e94)));
    float _e99 = float_3_vector_scalar.y;
    out_.Store(int(10)*4, asuint(asuint(_e99)));
    float _e104 = float_3_vector_scalar.z;
    out_.Store(int(11)*4, asuint(asuint(_e104)));
    float _e109 = float_3_scalar_vector.x;
    out_.Store(int(12)*4, asuint(asuint(_e109)));
    float _e114 = float_3_scalar_vector.y;
    out_.Store(int(13)*4, asuint(asuint(_e114)));
    float _e119 = float_3_scalar_vector.z;
    out_.Store(int(14)*4, asuint(asuint(_e119)));
    float_4_left = float4(1.25, -(2.5), 3.75, -(4.0));
    float_4_right = float4(0.5, 1.5, -(2.0), 0.25);
    float4 _e137 = float_4_left;
    float4 _e138 = float_4_right;
    const float4 _e139 = operator_u002b_6(_e137, _e138);
    float_4_vectors = _e139;
    float4 _e141 = float_4_left;
    const float4 _e143 = operator_u002b_7(_e141, 2.0);
    float_4_vector_scalar = _e143;
    float4 _e146 = float_4_right;
    const float4 _e147 = operator_u002b_8(2.0, _e146);
    float_4_scalar_vector = _e147;
    float _e151 = float_4_vectors.x;
    out_.Store(int(15)*4, asuint(asuint(_e151)));
    float _e156 = float_4_vectors.y;
    out_.Store(int(16)*4, asuint(asuint(_e156)));
    float _e161 = float_4_vectors.z;
    out_.Store(int(17)*4, asuint(asuint(_e161)));
    float _e166 = float_4_vectors.w;
    out_.Store(int(18)*4, asuint(asuint(_e166)));
    float _e171 = float_4_vector_scalar.x;
    out_.Store(int(19)*4, asuint(asuint(_e171)));
    float _e176 = float_4_vector_scalar.y;
    out_.Store(int(20)*4, asuint(asuint(_e176)));
    float _e181 = float_4_vector_scalar.z;
    out_.Store(int(21)*4, asuint(asuint(_e181)));
    float _e186 = float_4_vector_scalar.w;
    out_.Store(int(22)*4, asuint(asuint(_e186)));
    float _e191 = float_4_scalar_vector.x;
    out_.Store(int(23)*4, asuint(asuint(_e191)));
    float _e196 = float_4_scalar_vector.y;
    out_.Store(int(24)*4, asuint(asuint(_e196)));
    float _e201 = float_4_scalar_vector.z;
    out_.Store(int(25)*4, asuint(asuint(_e201)));
    float _e206 = float_4_scalar_vector.w;
    out_.Store(int(26)*4, asuint(asuint(_e206)));
    int_2_left = int2(int(1), naga_neg(int(2)));
    int_2_right = int2(int(5), int(6));
    int2 _e218 = int_2_left;
    int2 _e219 = int_2_right;
    const int2 _e220 = operator_u002b_9(_e218, _e219);
    int_2_vectors = _e220;
    int2 _e222 = int_2_left;
    const int2 _e224 = operator_u002b_10(_e222, int(3));
    int_2_vector_scalar = _e224;
    int2 _e227 = int_2_right;
    const int2 _e228 = operator_u002b_11(int(3), _e227);
    int_2_scalar_vector = _e228;
    int _e232 = int_2_vectors.x;
    out_.Store(int(27)*4, asuint(uint(_e232)));
    int _e237 = int_2_vectors.y;
    out_.Store(int(28)*4, asuint(uint(_e237)));
    int _e242 = int_2_vector_scalar.x;
    out_.Store(int(29)*4, asuint(uint(_e242)));
    int _e247 = int_2_vector_scalar.y;
    out_.Store(int(30)*4, asuint(uint(_e247)));
    int _e252 = int_2_scalar_vector.x;
    out_.Store(int(31)*4, asuint(uint(_e252)));
    int _e257 = int_2_scalar_vector.y;
    out_.Store(int(32)*4, asuint(uint(_e257)));
    int_3_left = Constructint3_(int(1), naga_neg(int(2)), int(3));
    int_3_right = Constructint3_(int(5), int(6), naga_neg(int(7)));
    int3_ _e272 = int_3_left;
    int3_ _e273 = int_3_right;
    const int3_ _e274 = operator_u002b_12(_e272, _e273);
    int_3_vectors = _e274;
    int3_ _e276 = int_3_left;
    const int3_ _e278 = operator_u002b_13(_e276, int(3));
    int_3_vector_scalar = _e278;
    int3_ _e281 = int_3_right;
    const int3_ _e282 = operator_u002b_14(int(3), _e281);
    int_3_scalar_vector = _e282;
    int _e286 = int_3_vectors.x;
    out_.Store(int(33)*4, asuint(uint(_e286)));
    int _e291 = int_3_vectors.y;
    out_.Store(int(34)*4, asuint(uint(_e291)));
    int _e296 = int_3_vectors.z;
    out_.Store(int(35)*4, asuint(uint(_e296)));
    int _e301 = int_3_vector_scalar.x;
    out_.Store(int(36)*4, asuint(uint(_e301)));
    int _e306 = int_3_vector_scalar.y;
    out_.Store(int(37)*4, asuint(uint(_e306)));
    int _e311 = int_3_vector_scalar.z;
    out_.Store(int(38)*4, asuint(uint(_e311)));
    int _e316 = int_3_scalar_vector.x;
    out_.Store(int(39)*4, asuint(uint(_e316)));
    int _e321 = int_3_scalar_vector.y;
    out_.Store(int(40)*4, asuint(uint(_e321)));
    int _e326 = int_3_scalar_vector.z;
    out_.Store(int(41)*4, asuint(uint(_e326)));
    int_4_left = int4(int(1), naga_neg(int(2)), int(3), naga_neg(int(4)));
    int_4_right = int4(int(5), int(6), naga_neg(int(7)), int(8));
    int4 _e344 = int_4_left;
    int4 _e345 = int_4_right;
    const int4 _e346 = operator_u002b_15(_e344, _e345);
    int_4_vectors = _e346;
    int4 _e348 = int_4_left;
    const int4 _e350 = operator_u002b_16(_e348, int(3));
    int_4_vector_scalar = _e350;
    int4 _e353 = int_4_right;
    const int4 _e354 = operator_u002b_17(int(3), _e353);
    int_4_scalar_vector = _e354;
    int _e358 = int_4_vectors.x;
    out_.Store(int(42)*4, asuint(uint(_e358)));
    int _e363 = int_4_vectors.y;
    out_.Store(int(43)*4, asuint(uint(_e363)));
    int _e368 = int_4_vectors.z;
    out_.Store(int(44)*4, asuint(uint(_e368)));
    int _e373 = int_4_vectors.w;
    out_.Store(int(45)*4, asuint(uint(_e373)));
    int _e378 = int_4_vector_scalar.x;
    out_.Store(int(46)*4, asuint(uint(_e378)));
    int _e383 = int_4_vector_scalar.y;
    out_.Store(int(47)*4, asuint(uint(_e383)));
    int _e388 = int_4_vector_scalar.z;
    out_.Store(int(48)*4, asuint(uint(_e388)));
    int _e393 = int_4_vector_scalar.w;
    out_.Store(int(49)*4, asuint(uint(_e393)));
    int _e398 = int_4_scalar_vector.x;
    out_.Store(int(50)*4, asuint(uint(_e398)));
    int _e403 = int_4_scalar_vector.y;
    out_.Store(int(51)*4, asuint(uint(_e403)));
    int _e408 = int_4_scalar_vector.z;
    out_.Store(int(52)*4, asuint(uint(_e408)));
    int _e413 = int_4_scalar_vector.w;
    out_.Store(int(53)*4, asuint(uint(_e413)));
    uint_2_left = uint2(4294967294u, 2147483649u);
    uint_2_right = uint2(5u, 2u);
    uint2 _e424 = uint_2_left;
    uint2 _e425 = uint_2_right;
    const uint2 _e426 = operator_u002b_18(_e424, _e425);
    uint_2_vectors = _e426;
    uint2 _e428 = uint_2_left;
    const uint2 _e430 = operator_u002b_19(_e428, 4u);
    uint_2_vector_scalar = _e430;
    uint2 _e433 = uint_2_right;
    const uint2 _e434 = operator_u002b_20(4u, _e433);
    uint_2_scalar_vector = _e434;
    uint _e438 = uint_2_vectors.x;
    out_.Store(int(54)*4, asuint(_e438));
    uint _e442 = uint_2_vectors.y;
    out_.Store(int(55)*4, asuint(_e442));
    uint _e446 = uint_2_vector_scalar.x;
    out_.Store(int(56)*4, asuint(_e446));
    uint _e450 = uint_2_vector_scalar.y;
    out_.Store(int(57)*4, asuint(_e450));
    uint _e454 = uint_2_scalar_vector.x;
    out_.Store(int(58)*4, asuint(_e454));
    uint _e458 = uint_2_scalar_vector.y;
    out_.Store(int(59)*4, asuint(_e458));
    uint_3_left = Constructuint3_(4294967294u, 2147483649u, 3u);
    uint_3_right = Constructuint3_(5u, 2u, 4294967293u);
    uint3_ _e470 = uint_3_left;
    uint3_ _e471 = uint_3_right;
    const uint3_ _e472 = operator_u002b_21(_e470, _e471);
    uint_3_vectors = _e472;
    uint3_ _e474 = uint_3_left;
    const uint3_ _e476 = operator_u002b_22(_e474, 4u);
    uint_3_vector_scalar = _e476;
    uint3_ _e479 = uint_3_right;
    const uint3_ _e480 = operator_u002b_23(4u, _e479);
    uint_3_scalar_vector = _e480;
    uint _e484 = uint_3_vectors.x;
    out_.Store(int(60)*4, asuint(_e484));
    uint _e488 = uint_3_vectors.y;
    out_.Store(int(61)*4, asuint(_e488));
    uint _e492 = uint_3_vectors.z;
    out_.Store(int(62)*4, asuint(_e492));
    uint _e496 = uint_3_vector_scalar.x;
    out_.Store(int(63)*4, asuint(_e496));
    uint _e500 = uint_3_vector_scalar.y;
    out_.Store(int(64)*4, asuint(_e500));
    uint _e504 = uint_3_vector_scalar.z;
    out_.Store(int(65)*4, asuint(_e504));
    uint _e508 = uint_3_scalar_vector.x;
    out_.Store(int(66)*4, asuint(_e508));
    uint _e512 = uint_3_scalar_vector.y;
    out_.Store(int(67)*4, asuint(_e512));
    uint _e516 = uint_3_scalar_vector.z;
    out_.Store(int(68)*4, asuint(_e516));
    uint_4_left = uint4(4294967294u, 2147483649u, 3u, 4294967295u);
    uint_4_right = uint4(5u, 2u, 4294967293u, 1u);
    uint4 _e530 = uint_4_left;
    uint4 _e531 = uint_4_right;
    const uint4 _e532 = operator_u002b_24(_e530, _e531);
    uint_4_vectors = _e532;
    uint4 _e534 = uint_4_left;
    const uint4 _e536 = operator_u002b_25(_e534, 4u);
    uint_4_vector_scalar = _e536;
    uint4 _e539 = uint_4_right;
    const uint4 _e540 = operator_u002b_26(4u, _e539);
    uint_4_scalar_vector = _e540;
    uint _e544 = uint_4_vectors.x;
    out_.Store(int(69)*4, asuint(_e544));
    uint _e548 = uint_4_vectors.y;
    out_.Store(int(70)*4, asuint(_e548));
    uint _e552 = uint_4_vectors.z;
    out_.Store(int(71)*4, asuint(_e552));
    uint _e556 = uint_4_vectors.w;
    out_.Store(int(72)*4, asuint(_e556));
    uint _e560 = uint_4_vector_scalar.x;
    out_.Store(int(73)*4, asuint(_e560));
    uint _e564 = uint_4_vector_scalar.y;
    out_.Store(int(74)*4, asuint(_e564));
    uint _e568 = uint_4_vector_scalar.z;
    out_.Store(int(75)*4, asuint(_e568));
    uint _e572 = uint_4_vector_scalar.w;
    out_.Store(int(76)*4, asuint(_e572));
    uint _e576 = uint_4_scalar_vector.x;
    out_.Store(int(77)*4, asuint(_e576));
    uint _e580 = uint_4_scalar_vector.y;
    out_.Store(int(78)*4, asuint(_e580));
    uint _e584 = uint_4_scalar_vector.z;
    out_.Store(int(79)*4, asuint(_e584));
    uint _e588 = uint_4_scalar_vector.w;
    out_.Store(int(80)*4, asuint(_e588));
}
