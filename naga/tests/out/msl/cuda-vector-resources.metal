// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
};

struct float3_ {
    float x;
    float y;
    float z;
};
typedef float3_ type_2[1];

float3_ shift(
    float3_ value
) {
    float3_ value_1 = {};
    value_1 = value;
    float _e4 = value_1.x;
    value_1.x = _e4 + 1.0;
    float _e9 = value_1.y;
    float _e10 = value_1.z;
    value_1.z = _e9 + _e10;
    float3_ _e12 = value_1;
    return _e12;
}

float pair(
    float3_ value_2,
    int digit
) {
    float3_ value_3 = {};
    int digit_1 = {};
    value_3 = value_2;
    digit_1 = digit;
    float _e6 = value_3.x;
    int _e8 = digit_1;
    return (_e6 * 10.0) + static_cast<float>(_e8);
}

kernel void vector_resources(
  device type_2 const& input [[user(fake0)]]
, device type_2& output [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int i = {};
    float3_ copied = {};
    int index = {};
    bool choose_first = {};
    float local = {};
    float local_1 = {};
    float3_ local_2 = {};
    bool pick_local = {};
    float3_ snapshot = {};
    {
        i = 0;
        uint2 loop_bound = uint2(4294967295u);
        bool loop_init = true;
        while(true) {
            if (metal::all(loop_bound == uint2(0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e7 = i;
                i = as_type<int>(as_type<uint>(_e7) + as_type<uint>(1));
            }
            loop_init = false;
            int _e5 = i;
            if (_e5 < 3) {
            } else {
                break;
            }
            int _e11 = i;
            float3_ _e13 = input[_e11];
            copied = _e13;
            float3_ _e16 = copied;
            float3_ _e17 = shift(_e16);
            int _e14 = i;
            output[_e14] = _e17;
        }
    }
    float3_ _e22 = input[0];
    output[3] = _e22;
    float3_ _e28 = input[1];
    float3_ _e29 = shift(_e28);
    output[3].y = _e29.x;
    index = 0;
    index = 1;
    float _e42 = input[2].x;
    int _e33 = index;
    index = as_type<int>(as_type<uint>(_e33) + as_type<uint>(1));
    output[_e33].x = _e42;
    int _e45 = index;
    float _e49 = input[0].x;
    float _e53 = input[1].x;
    output[4] = float3_ {static_cast<float>(_e45), _e49, _e53};
    choose_first = true;
    bool _e58 = choose_first;
    if (_e58) {
        float _e66 = output[0].y;
        output[0].y = _e66 + 2.0;
    } else {
        float _e68 = output[2].y;
        output[2].y = _e68 + 2.0;
    }
    bool _e73 = choose_first;
    if (_e73) {
        float _e81 = output[0].z;
        output[0].z = _e81 + static_cast<float>(1);
        local = _e81;
    } else {
        float _e85 = output[2].z;
        output[2].z = _e85 + static_cast<float>(1);
        local = _e85;
    }
    float _e89 = local;
    output[4].x = _e89;
    int _e93 = index;
    output[4].z = static_cast<float>(_e93);
    float3_ _e102 = input[1];
    bool _e95 = choose_first;
    if (_e95) {
        output[0] = _e102;
        float _e106 = output[0].x;
        output[0].x = _e106 + static_cast<float>(1);
        local_1 = _e106;
    } else {
        output[2] = _e102;
        float _e110 = output[2].x;
        output[2].x = _e110 + static_cast<float>(1);
        local_1 = _e110;
    }
    float3_ _e118 = output[0];
    output[5] = _e118;
    local_2 = float3_ {static_cast<float>(20), static_cast<float>(30), static_cast<float>(40)};
    pick_local = false;
    pick_local = true;
    bool _e129 = pick_local;
    if (_e129) {
        local_2.z = 7.0;
    } else {
        output[1].z = 7.0;
    }
    float3_ _e138 = local_2;
    output[6] = _e138;
    float _e145 = output[1].z;
    output[6].x = _e145;
    float _e151 = output[0].x;
    float _e155 = output[1].x;
    float _e159 = output[2].x;
    output[7] = float3_ {_e151, _e155, _e159};
    float3_ _e164 = input[0];
    snapshot = _e164;
    float3_ _e167 = snapshot;
    snapshot.x = 9.0;
    float _e171 = pair(_e167, 4);
    float _e173 = snapshot.x;
    float _e177 = input[2].z;
    output[8] = float3_ {_e171, _e173, _e177};
}
