// language: metal1.2
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

struct type_1 {
    int inner[2];
};
struct Record_cuda_record_0_ {
    int value;
    type_1 samples;
};
typedef uint type_3[1];

int adjust(
    int value_1
) {
    int value_2 = {};
    int local_20 = {};
    value_2 = value_1;
    int _e3 = value_2;
    value_2 = as_type<int>(as_type<uint>(_e3) + as_type<uint>(2));
    int _e6 = value_2;
    local_20 = _e6;
    int _e7 = local_20;
    return _e7;
}
uint naga_f2u32(float value) {
    return static_cast<uint>(metal::clamp(value, 0.0, 4294967000.0));
}

int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}


kernel void reference_bindings(
  device type_3& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int value = {};
    int local = {};
    int local_1 = {};
    int local_2 = {};
    int left = {};
    int right = {};
    int local_3 = {};
    int source = {};
    float local_4 = {};
    float local_5 = {};
    int local_6 = {};
    int uninitialized = {};
    uint local_7 = {};
    int original = {};
    int replacement = {};
    int local_8 = {};
    int local_9 = {};
    float local_10 = {};
    int local_11 = {};
    int local_12 = {};
    float local_13 = {};
    int local_14 = {};
    int updated = {};
    int local_15 = {};
    int local_16 = {};
    int comma_left = {};
    int comma_right = {};
    int counter = {};
    bool enabled = {};
    uint bits = {};
    uint local_17 = {};
    float floating = {};
    Record_cuda_record_0_ record = {};
    int local_18 = {};
    int local_19 = {};
    value = 1;
    value = 5;
    int _e5 = value;
    value = as_type<int>(as_type<uint>(_e5) + as_type<uint>(2));
    int _e8 = value;
    local = _e8;
    int _e9 = local;
    int _e10 = value;
    value = as_type<int>(as_type<uint>(_e10) + as_type<uint>(_e9));
    int _e15 = value;
    local_1 = _e15;
    int _e16 = local_1;
    out[0] = static_cast<uint>(_e16);
    int _e21 = value;
    local_2 = _e21;
    int _e22 = local_2;
    out[1] = static_cast<uint>(_e22);
    int _e26 = value;
    out[2] = static_cast<uint>(_e26);
    left = 1;
    right = 2;
    left = 5;
    int _e34 = right;
    right = as_type<int>(as_type<uint>(_e34) + as_type<uint>(2));
    int _e37 = right;
    local_3 = _e37;
    int _e38 = local_3;
    int _e39 = left;
    left = as_type<int>(as_type<uint>(_e39) + as_type<uint>(_e38));
    int _e43 = left;
    out[3] = static_cast<uint>(_e43);
    int _e47 = right;
    out[4] = static_cast<uint>(_e47);
    source = 3;
    int _e51 = source;
    local_4 = static_cast<float>(_e51);
    source = 9;
    float _e58 = local_4;
    local_5 = _e58;
    float _e59 = local_5;
    out[5] = naga_f2u32(_e59);
    int _e64 = source;
    local_6 = _e64;
    int _e65 = local_6;
    out[6] = static_cast<uint>(_e65);
    int _e69 = source;
    out[7] = static_cast<uint>(_e69);
    uninitialized = 7;
    local_7 = static_cast<uint>(8);
    int _e76 = uninitialized;
    uint _e78 = local_7;
    out[_e78] = static_cast<uint>(_e76);
    original = 2;
    replacement = 8;
    int _e84 = replacement;
    original = _e84;
    replacement = 10;
    int _e88 = original;
    out[9] = static_cast<uint>(_e88);
    int _e93 = original;
    local_8 = _e93;
    int _e94 = local_8;
    out[10] = static_cast<uint>(_e94);
    int _e98 = replacement;
    out[11] = static_cast<uint>(_e98);
    local_9 = int {};
    local_10 = 1.5;
    local_11 = naga_f2i32(3.75);
    int _e110 = local_9;
    local_12 = _e110;
    int _e111 = local_12;
    out[12] = static_cast<uint>(_e111);
    float _e116 = local_10;
    local_13 = _e116;
    float _e117 = local_13;
    out[13] = as_type<uint>(_e117);
    int _e122 = local_11;
    local_14 = _e122;
    int _e123 = local_14;
    out[14] = static_cast<uint>(_e123);
    updated = 1;
    int _e127 = updated;
    updated = as_type<int>(as_type<uint>(_e127) + as_type<uint>(1));
    int _e131 = updated;
    updated = as_type<int>(as_type<uint>(_e131) + as_type<uint>(3));
    int _e136 = updated;
    local_15 = _e136;
    int _e137 = local_15;
    out[15] = static_cast<uint>(_e137);
    updated = 9;
    int _e141 = updated;
    updated = as_type<int>(as_type<uint>(_e141) + as_type<uint>(2));
    int _e146 = updated;
    local_16 = _e146;
    int _e147 = local_16;
    out[16] = static_cast<uint>(_e147);
    comma_left = 3;
    comma_right = 4;
    counter = 0;
    int _e155 = counter;
    counter = as_type<int>(as_type<uint>(_e155) + as_type<uint>(1));
    int _e159 = comma_right;
    comma_right = as_type<int>(as_type<uint>(_e159) + as_type<uint>(5));
    int _e163 = comma_left;
    out[17] = static_cast<uint>(_e163);
    int _e167 = comma_right;
    out[18] = static_cast<uint>(_e167);
    int _e171 = counter;
    out[19] = static_cast<uint>(_e171);
    enabled = false;
    enabled = true;
    bool _e178 = enabled;
    out[20] = static_cast<uint>(_e178);
    bits = 4294901760u;
    uint _e183 = bits;
    bits = _e183 + 3u;
    uint _e188 = bits;
    local_17 = _e188;
    uint _e189 = local_17;
    out[21] = _e189;
    floating = 2.0;
    float _e193 = floating;
    floating = _e193 * 0.5;
    float _e197 = floating;
    out[22] = as_type<uint>(_e197);
    record.value = 10;
    record.samples.inner[0] = 2;
    record.samples.inner[1] = 3;
    int _e209 = record.value;
    local_18 = _e209;
    int _e210 = local_18;
    int _e211 = adjust(_e210);
    record.value = _e211;
    int _e217 = record.value;
    local_19 = _e217;
    int _e218 = local_19;
    out[23] = static_cast<uint>(_e218);
}
