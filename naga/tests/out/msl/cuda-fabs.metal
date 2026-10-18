// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

struct float3_ {
    float x;
    float y;
    float z;
};
typedef uint type_6[1];

metal::float2 fabs(
    metal::float2 value_1
) {
    metal::float2 value_2 = {};
    uint local_23 = {};
    uint local_24 = {};
    value_2 = value_1;
    float _e3 = value_2.x;
    uint _e6 = as_type<uint>(_e3) & 2147483647u;
    local_23 = (_e6 > 2139095040u) ? 2143289344u : _e6;
    uint _e12 = local_23;
    float _e15 = value_2.y;
    uint _e18 = as_type<uint>(_e15) & 2147483647u;
    local_24 = (_e18 > 2139095040u) ? 2143289344u : _e18;
    uint _e24 = local_24;
    return metal::float2(as_type<float>(_e12), as_type<float>(_e24));
}

float3_ fabs_1(
    float3_ value_3
) {
    float3_ value_4 = {};
    uint local_25 = {};
    uint local_26 = {};
    uint local_27 = {};
    value_4 = value_3;
    float _e3 = value_4.x;
    uint _e6 = as_type<uint>(_e3) & 2147483647u;
    local_25 = (_e6 > 2139095040u) ? 2143289344u : _e6;
    uint _e12 = local_25;
    float _e15 = value_4.y;
    uint _e18 = as_type<uint>(_e15) & 2147483647u;
    local_26 = (_e18 > 2139095040u) ? 2143289344u : _e18;
    uint _e24 = local_26;
    float _e27 = value_4.z;
    uint _e30 = as_type<uint>(_e27) & 2147483647u;
    local_27 = (_e30 > 2139095040u) ? 2143289344u : _e30;
    uint _e36 = local_27;
    return float3_ {as_type<float>(_e12), as_type<float>(_e24), as_type<float>(_e36)};
}

metal::float4 fabs_2(
    metal::float4 value_5
) {
    metal::float4 value_6 = {};
    uint local_28 = {};
    uint local_29 = {};
    uint local_30 = {};
    uint local_31 = {};
    value_6 = value_5;
    float _e3 = value_6.x;
    uint _e6 = as_type<uint>(_e3) & 2147483647u;
    local_28 = (_e6 > 2139095040u) ? 2143289344u : _e6;
    uint _e12 = local_28;
    float _e15 = value_6.y;
    uint _e18 = as_type<uint>(_e15) & 2147483647u;
    local_29 = (_e18 > 2139095040u) ? 2143289344u : _e18;
    uint _e24 = local_29;
    float _e27 = value_6.z;
    uint _e30 = as_type<uint>(_e27) & 2147483647u;
    local_30 = (_e30 > 2139095040u) ? 2143289344u : _e30;
    uint _e36 = local_30;
    float _e39 = value_6.w;
    uint _e42 = as_type<uint>(_e39) & 2147483647u;
    local_31 = (_e42 > 2139095040u) ? 2143289344u : _e42;
    uint _e48 = local_31;
    return metal::float4(as_type<float>(_e12), as_type<float>(_e24), as_type<float>(_e36), as_type<float>(_e48));
}

float absolute(
    float value_7
) {
    float value_8 = {};
    uint local_32 = {};
    value_8 = value_7;
    float _e2 = value_8;
    uint _e5 = as_type<uint>(_e2) & 2147483647u;
    local_32 = (_e5 > 2139095040u) ? 2143289344u : _e5;
    uint _e11 = local_32;
    return as_type<float>(_e11);
}

float converted_absolute(
    int value_9
) {
    int value_10 = {};
    uint local_33 = {};
    value_10 = value_9;
    int _e2 = value_10;
    uint _e6 = as_type<uint>(static_cast<float>(_e2)) & 2147483647u;
    local_33 = (_e6 > 2139095040u) ? 2143289344u : _e6;
    uint _e12 = local_33;
    return as_type<float>(_e12);
}

float converted_absolute_1(
    uint value_11
) {
    uint value_12 = {};
    uint local_34 = {};
    value_12 = value_11;
    uint _e2 = value_12;
    uint _e6 = as_type<uint>(static_cast<float>(_e2)) & 2147483647u;
    local_34 = (_e6 > 2139095040u) ? 2143289344u : _e6;
    uint _e12 = local_34;
    return as_type<float>(_e12);
}

float converted_absolute_2(
    bool value_13
) {
    bool value_14 = {};
    uint local_35 = {};
    value_14 = value_13;
    bool _e2 = value_14;
    uint _e6 = as_type<uint>(static_cast<float>(_e2)) & 2147483647u;
    local_35 = (_e6 > 2139095040u) ? 2143289344u : _e6;
    uint _e12 = local_35;
    return as_type<float>(_e12);
}

int fabs_3(
    int value_15
) {
    int value_16 = {};
    value_16 = value_15;
    int _e3 = value_16;
    return as_type<int>(as_type<uint>(_e3) + as_type<uint>(40));
}

uint fabs_4(
    uint value_17
) {
    uint value_18 = {};
    value_18 = value_17;
    uint _e3 = value_18;
    return _e3 + 50u;
}

int fabs_5(
    bool value_19
) {
    bool value_20 = {};
    int local_36 = {};
    value_20 = value_19;
    bool _e2 = value_20;
    if (_e2) {
        local_36 = 7;
    } else {
        local_36 = 9;
    }
    int _e6 = local_36;
    return _e6;
}

int fabsf(
    bool value_21
) {
    bool value_22 = {};
    int local_37 = {};
    value_22 = value_21;
    bool _e2 = value_22;
    if (_e2) {
        local_37 = 11;
    } else {
        local_37 = 13;
    }
    int _e6 = local_37;
    return _e6;
}

float fabsf_1(
    float first,
    float second
) {
    float first_1 = {};
    float second_1 = {};
    first_1 = first;
    second_1 = second;
    float _e4 = first_1;
    float _e5 = second_1;
    return _e4 + _e5;
}
int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}


kernel void absolutes(
  device type_6& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint local = {};
    uint local_1 = {};
    uint local_2 = {};
    uint local_3 = {};
    uint local_4 = {};
    uint local_5 = {};
    uint local_6 = {};
    uint local_7 = {};
    uint local_8 = {};
    uint local_9 = {};
    uint local_10 = {};
    uint local_11 = {};
    uint local_12 = {};
    uint local_13 = {};
    uint local_14 = {};
    uint local_15 = {};
    uint local_16 = {};
    uint local_17 = {};
    uint local_18 = {};
    uint local_19 = {};
    uint local_20 = {};
    float value = {};
    uint local_21 = {};
    float3_ triple = {};
    uint local_22 = {};
    metal::float2 pair = {};
    metal::float4 quadruple = {};
    uint _e6 = as_type<uint>(0.0) & 2147483647u;
    local = (_e6 > 2139095040u) ? 2143289344u : _e6;
    uint _e12 = local;
    out[0] = as_type<uint>(as_type<float>(_e12));
    uint _e21 = as_type<uint>(-(0.0)) & 2147483647u;
    local_1 = (_e21 > 2139095040u) ? 2143289344u : _e21;
    uint _e27 = local_1;
    out[1] = as_type<uint>(as_type<float>(_e27));
    uint _e36 = as_type<uint>(-(1.75)) & 2147483647u;
    local_2 = (_e36 > 2139095040u) ? 2143289344u : _e36;
    uint _e42 = local_2;
    out[2] = as_type<uint>(as_type<float>(_e42));
    float _e49 = absolute(-(1.25));
    out[3] = as_type<uint>(_e49);
    float _e55 = converted_absolute(naga_neg(3));
    out[4] = as_type<uint>(_e55);
    float _e60 = converted_absolute_1(4294967295u);
    out[5] = as_type<uint>(_e60);
    float _e65 = converted_absolute_2(true);
    out[6] = as_type<uint>(_e65);
    float _e70 = converted_absolute_2(false);
    out[7] = as_type<uint>(_e70);
    local_3 = 1u;
    uint _e76 = local_3;
    uint _e80 = as_type<uint>(as_type<float>(_e76)) & 2147483647u;
    local_4 = (_e80 > 2139095040u) ? 2143289344u : _e80;
    uint _e86 = local_4;
    out[8] = as_type<uint>(as_type<float>(_e86));
    local_5 = 2147483649u;
    uint _e93 = local_5;
    uint _e97 = as_type<uint>(as_type<float>(_e93)) & 2147483647u;
    local_6 = (_e97 > 2139095040u) ? 2143289344u : _e97;
    uint _e103 = local_6;
    out[9] = as_type<uint>(as_type<float>(_e103));
    local_7 = 8388607u;
    uint _e110 = local_7;
    uint _e114 = as_type<uint>(as_type<float>(_e110)) & 2147483647u;
    local_8 = (_e114 > 2139095040u) ? 2143289344u : _e114;
    uint _e120 = local_8;
    out[10] = as_type<uint>(as_type<float>(_e120));
    local_9 = 2155872255u;
    uint _e127 = local_9;
    uint _e131 = as_type<uint>(as_type<float>(_e127)) & 2147483647u;
    local_10 = (_e131 > 2139095040u) ? 2143289344u : _e131;
    uint _e137 = local_10;
    out[11] = as_type<uint>(as_type<float>(_e137));
    uint _e145 = as_type<uint>(340282350000000000000000000000000000000.0) & 2147483647u;
    local_11 = (_e145 > 2139095040u) ? 2143289344u : _e145;
    uint _e151 = local_11;
    out[12] = as_type<uint>(as_type<float>(_e151));
    uint _e160 = as_type<uint>(-(340282350000000000000000000000000000000.0)) & 2147483647u;
    local_12 = (_e160 > 2139095040u) ? 2143289344u : _e160;
    uint _e166 = local_12;
    out[13] = as_type<uint>(as_type<float>(_e166));
    local_13 = 2139095040u;
    uint _e173 = local_13;
    uint _e177 = as_type<uint>(as_type<float>(_e173)) & 2147483647u;
    local_14 = (_e177 > 2139095040u) ? 2143289344u : _e177;
    uint _e183 = local_14;
    out[14] = as_type<uint>(as_type<float>(_e183));
    local_15 = 4286578688u;
    uint _e190 = local_15;
    uint _e194 = as_type<uint>(as_type<float>(_e190)) & 2147483647u;
    local_16 = (_e194 > 2139095040u) ? 2143289344u : _e194;
    uint _e200 = local_16;
    out[15] = as_type<uint>(as_type<float>(_e200));
    local_17 = 2143363909u;
    uint _e207 = local_17;
    uint _e211 = as_type<uint>(as_type<float>(_e207)) & 2147483647u;
    local_18 = (_e211 > 2139095040u) ? 2143289344u : _e211;
    uint _e217 = local_18;
    out[16] = as_type<uint>(as_type<float>(_e217));
    local_19 = 2139095041u;
    uint _e224 = local_19;
    uint _e228 = as_type<uint>(as_type<float>(_e224)) & 2147483647u;
    local_20 = (_e228 > 2139095040u) ? 2143289344u : _e228;
    uint _e234 = local_20;
    out[17] = as_type<uint>(as_type<float>(_e234));
    value = -(2.75);
    float _e242 = value;
    value = _e242 + static_cast<float>(1);
    uint _e248 = as_type<uint>(_e242) & 2147483647u;
    local_21 = (_e248 > 2139095040u) ? 2143289344u : _e248;
    uint _e254 = local_21;
    out[18] = as_type<uint>(as_type<float>(_e254));
    float _e259 = value;
    out[19] = as_type<uint>(_e259);
    int _e265 = fabs_3(naga_neg(3));
    out[20] = static_cast<uint>(_e265);
    uint _e270 = fabs_4(3u);
    out[21] = _e270;
    int _e274 = fabs_5(true);
    out[22] = static_cast<uint>(_e274);
    int _e279 = fabs_5(false);
    out[23] = static_cast<uint>(_e279);
    int _e284 = fabsf(true);
    out[24] = static_cast<uint>(_e284);
    local_22 = 2147483649u;
    uint _e293 = local_22;
    float3_ _e296 = fabs_1(float3_ {-(2.5), -(0.0), as_type<float>(_e293)});
    triple = _e296;
    float _e300 = triple.x;
    out[25] = as_type<uint>(_e300);
    float _e305 = triple.y;
    out[26] = as_type<uint>(_e305);
    float _e310 = triple.z;
    out[27] = as_type<uint>(_e310);
    float _e317 = fabsf_1(-(2.0), 3.0);
    out[28] = as_type<uint>(_e317);
    metal::float2 _e325 = fabs(metal::float2(-(3.0), -(4.0)));
    pair = _e325;
    metal::float4 _e336 = fabs_2(metal::float4(-(1.0), -(2.0), -(3.0), -(4.0)));
    quadruple = _e336;
    float _e341 = pair.x;
    float _e342 = pair.y;
    float _e345 = quadruple.x;
    float _e348 = quadruple.w;
    out[29] = as_type<uint>(((_e341 + _e342) + _e345) + _e348);
}
