// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef uint type_3[1];

float from_bits(
    uint value_1
) {
    uint value_2 = {};
    uint local_13 = {};
    value_2 = value_1;
    uint _e2 = value_2;
    local_13 = _e2;
    uint _e4 = local_13;
    return as_type<float>(_e4);
}

int _float_as_int(
    int first,
    int second
) {
    int first_1 = {};
    int second_1 = {};
    first_1 = first;
    second_1 = second;
    int _e4 = first_1;
    int _e5 = second_1;
    return as_type<int>(as_type<uint>(_e4) + as_type<uint>(_e5));
}
int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

uint naga_f2u32(float value) {
    return static_cast<uint>(metal::clamp(value, 0.0, 4294967000.0));
}


kernel void bitcasts(
  device type_3& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int local = {};
    int local_1 = {};
    int local_2 = {};
    int local_3 = {};
    uint local_4 = {};
    uint local_5 = {};
    uint local_6 = {};
    uint local_7 = {};
    float value = {};
    int signed_bits = {};
    int local_8 = {};
    uint unsigned_bits = {};
    uint local_9 = {};
    float copied = {};
    int local_10 = {};
    uint local_11 = {};
    uint local_12 = {};
    out[0] = static_cast<uint>(as_type<int>(static_cast<float>(true)));
    out[1] = static_cast<uint>(as_type<int>(static_cast<float>(3)));
    out[2] = static_cast<uint>(as_type<int>(static_cast<float>(5u)));
    out[3] = static_cast<uint>(as_type<int>(2.75));
    out[4] = as_type<uint>(static_cast<float>(true));
    out[5] = as_type<uint>(static_cast<float>(3));
    out[6] = as_type<uint>(static_cast<float>(5u));
    out[7] = as_type<uint>(2.75);
    local = static_cast<int>(true);
    int _e48 = local;
    out[8] = as_type<uint>(as_type<float>(_e48));
    local_1 = 3;
    int _e55 = local_1;
    out[9] = as_type<uint>(as_type<float>(_e55));
    local_2 = static_cast<int>(5u);
    int _e63 = local_2;
    out[10] = as_type<uint>(as_type<float>(_e63));
    local_3 = naga_f2i32(2.75);
    int _e71 = local_3;
    out[11] = as_type<uint>(as_type<float>(_e71));
    local_4 = static_cast<uint>(true);
    uint _e79 = local_4;
    out[12] = as_type<uint>(as_type<float>(_e79));
    local_5 = static_cast<uint>(3);
    uint _e87 = local_5;
    out[13] = as_type<uint>(as_type<float>(_e87));
    local_6 = 5u;
    uint _e94 = local_6;
    out[14] = as_type<uint>(as_type<float>(_e94));
    local_7 = naga_f2u32(2.75);
    uint _e102 = local_7;
    out[15] = as_type<uint>(as_type<float>(_e102));
    value = 1.5;
    float _e109 = value;
    value = _e109 + static_cast<float>(1);
    out[16] = static_cast<uint>(as_type<int>(_e109));
    float _e117 = value;
    out[17] = as_type<uint>(_e117);
    float _e121 = value;
    value = _e121 + static_cast<float>(1);
    out[18] = as_type<uint>(_e121);
    float _e128 = value;
    out[19] = as_type<uint>(_e128);
    signed_bits = 1065353216;
    int _e134 = signed_bits;
    signed_bits = as_type<int>(as_type<uint>(_e134) + as_type<uint>(1));
    local_8 = _e134;
    int _e138 = local_8;
    out[20] = as_type<uint>(as_type<float>(_e138));
    int _e143 = signed_bits;
    out[21] = static_cast<uint>(_e143);
    unsigned_bits = 1073741824u;
    uint _e149 = unsigned_bits;
    unsigned_bits = _e149 + static_cast<uint>(1);
    local_9 = _e149;
    uint _e154 = local_9;
    out[22] = as_type<uint>(as_type<float>(_e154));
    uint _e159 = unsigned_bits;
    out[23] = _e159;
    int _e164 = _float_as_int(3, 4);
    out[24] = static_cast<uint>(_e164);
    float _e168 = from_bits(3212836864u);
    copied = _e168;
    float _e171 = copied;
    out[25] = as_type<uint>(_e171);
    local_10 = 2139095040;
    int _e177 = local_10;
    out[26] = as_type<uint>(as_type<float>(_e177));
    local_11 = 2143363909u;
    uint _e184 = local_11;
    out[27] = as_type<uint>(as_type<float>(_e184));
    local_12 = naga_f2u32(1065353200.0);
    uint _e192 = local_12;
    out[28] = as_type<uint>(as_type<float>(_e192));
    out[29] = static_cast<uint>(as_type<int>(static_cast<float>(1u)));
}
