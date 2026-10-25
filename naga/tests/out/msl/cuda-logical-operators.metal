// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

struct type_1 {
    int inner[2];
};
struct Sample_cuda_record_0_ {
    int value;
    type_1 payload;
};
typedef uint type_4[1];

Sample_cuda_record_0_ operator_u0026_u0026_(
    Sample_cuda_record_0_ left_1,
    Sample_cuda_record_0_ right_1
) {
    Sample_cuda_record_0_ left_2 = {};
    Sample_cuda_record_0_ right_2 = {};
    left_2 = left_1;
    right_2 = right_1;
    int _e7 = left_2.value;
    int _e10 = right_2.value;
    left_2.value = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e7) * as_type<uint>(10))) + as_type<uint>(_e10));
    int _e18 = right_2.payload.inner[0];
    int _e19 = left_2.payload.inner[1];
    left_2.payload.inner[1] = as_type<int>(as_type<uint>(_e19) + as_type<uint>(_e18));
    Sample_cuda_record_0_ _e21 = left_2;
    return _e21;
}

Sample_cuda_record_0_ operator_u007c_u007c(
    Sample_cuda_record_0_ left_3,
    Sample_cuda_record_0_ right_3
) {
    Sample_cuda_record_0_ left_4 = {};
    Sample_cuda_record_0_ right_4 = {};
    left_4 = left_3;
    right_4 = right_3;
    int _e7 = left_4.value;
    int _e10 = right_4.value;
    left_4.value = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e7) * as_type<uint>(100))) + as_type<uint>(_e10));
    int _e18 = right_4.payload.inner[1];
    int _e19 = left_4.payload.inner[0];
    left_4.payload.inner[0] = as_type<int>(as_type<uint>(_e19) + as_type<uint>(_e18));
    Sample_cuda_record_0_ _e21 = left_4;
    return _e21;
}

int operator_u0026_u0026_1(
    Sample_cuda_record_0_ left_5,
    int right_5
) {
    Sample_cuda_record_0_ left_6 = {};
    int right_6 = {};
    left_6 = left_5;
    right_6 = right_5;
    int _e6 = left_6.value;
    int _e8 = right_6;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e6) * as_type<uint>(10))) + as_type<uint>(_e8));
}

int operator_u007c_u007c_1(
    int left_7,
    Sample_cuda_record_0_ right_7
) {
    int left_8 = {};
    Sample_cuda_record_0_ right_8 = {};
    left_8 = left_7;
    right_8 = right_7;
    int _e5 = left_8;
    int _e8 = right_8.value;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e5) * as_type<uint>(100))) + as_type<uint>(_e8));
}

metal::int2 operator_u0026_u0026_2(
    metal::int2 left_9,
    metal::int2 right_9
) {
    metal::int2 left_10 = {};
    metal::int2 right_10 = {};
    left_10 = left_9;
    right_10 = right_9;
    int _e6 = left_10.x;
    int _e9 = right_10.x;
    int _e13 = left_10.y;
    int _e16 = right_10.y;
    return metal::int2(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e6) * as_type<uint>(10))) + as_type<uint>(_e9)), as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e13) * as_type<uint>(10))) + as_type<uint>(_e16)));
}

metal::int2 operator_u007c_u007c_2(
    metal::int2 left_11,
    metal::int2 right_11
) {
    metal::int2 left_12 = {};
    metal::int2 right_12 = {};
    left_12 = left_11;
    right_12 = right_11;
    int _e6 = left_12.x;
    int _e9 = right_12.x;
    int _e13 = left_12.y;
    int _e16 = right_12.y;
    return metal::int2(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e6) * as_type<uint>(100))) + as_type<uint>(_e9)), as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e13) * as_type<uint>(100))) + as_type<uint>(_e16)));
}

struct logical_operatorsInput {
};
kernel void logical_operators(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_4& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint base = {};
    Sample_cuda_record_0_ left = {};
    Sample_cuda_record_0_ right = {};
    Sample_cuda_record_0_ both = {};
    Sample_cuda_record_0_ either = {};
    metal::int2 first = {};
    metal::int2 second = {};
    Sample_cuda_record_0_ local = {};
    Sample_cuda_record_0_ local_1 = {};
    int counter = {};
    Sample_cuda_record_0_ evaluated = {};
    Sample_cuda_record_0_ local_2 = {};
    Sample_cuda_record_0_ local_3 = {};
    Sample_cuda_record_0_ local_4 = {};
    bool local_5 = {};
    bool local_6 = {};
    bool local_7 = {};
    bool local_8 = {};
    base = threadIdx.x * 24u;
    left.value = 2;
    left.payload.inner[0] = 3;
    left.payload.inner[1] = 4;
    right.value = 5;
    right.payload.inner[0] = 6;
    right.payload.inner[1] = 7;
    Sample_cuda_record_0_ _e23 = left;
    Sample_cuda_record_0_ _e24 = right;
    Sample_cuda_record_0_ _e25 = operator_u0026_u0026_(_e23, _e24);
    both = _e25;
    Sample_cuda_record_0_ _e27 = left;
    Sample_cuda_record_0_ _e28 = right;
    Sample_cuda_record_0_ _e29 = operator_u007c_u007c(_e27, _e28);
    either = _e29;
    int _e35 = both.value;
    uint _e31 = base;
    out[_e31 + 0u] = static_cast<uint>(_e35);
    int _e44 = both.payload.inner[1];
    uint _e38 = base;
    out[_e38 + 1u] = static_cast<uint>(_e44);
    int _e51 = either.value;
    uint _e47 = base;
    out[_e47 + 2u] = static_cast<uint>(_e51);
    int _e60 = either.payload.inner[0];
    uint _e54 = base;
    out[_e54 + 3u] = static_cast<uint>(_e60);
    int _e67 = left.value;
    uint _e63 = base;
    out[_e63 + 4u] = static_cast<uint>(_e67);
    int _e76 = left.payload.inner[1];
    uint _e70 = base;
    out[_e70 + 5u] = static_cast<uint>(_e76);
    Sample_cuda_record_0_ _e82 = left;
    int _e84 = operator_u0026_u0026_1(_e82, 9);
    uint _e79 = base;
    out[_e79 + 6u] = static_cast<uint>(_e84);
    Sample_cuda_record_0_ _e91 = right;
    int _e92 = operator_u007c_u007c_1(0, _e91);
    uint _e87 = base;
    out[_e87 + 7u] = static_cast<uint>(_e92);
    Sample_cuda_record_0_ _e98 = left;
    Sample_cuda_record_0_ _e99 = right;
    Sample_cuda_record_0_ _e100 = operator_u0026_u0026_(_e98, _e99);
    uint _e95 = base;
    out[_e95 + 8u] = static_cast<uint>(_e100.value);
    Sample_cuda_record_0_ _e107 = left;
    Sample_cuda_record_0_ _e108 = right;
    Sample_cuda_record_0_ _e109 = operator_u007c_u007c(_e107, _e108);
    uint _e104 = base;
    out[_e104 + 9u] = static_cast<uint>(_e109.payload.inner[0]);
    first = metal::int2(2, 3);
    second = metal::int2(4, 5);
    metal::int2 _e126 = first;
    metal::int2 _e127 = second;
    metal::int2 _e128 = operator_u0026_u0026_2(_e126, _e127);
    uint _e123 = base;
    out[_e123 + 10u] = static_cast<uint>(_e128.x);
    metal::int2 _e135 = first;
    metal::int2 _e136 = second;
    metal::int2 _e137 = operator_u007c_u007c_2(_e135, _e136);
    uint _e132 = base;
    out[_e132 + 11u] = static_cast<uint>(_e137.y);
    metal::int2 _e144 = first;
    metal::int2 _e145 = second;
    metal::int2 _e146 = operator_u0026_u0026_2(_e144, _e145);
    metal::int2 _e147 = first;
    metal::int2 _e148 = operator_u0026_u0026_2(_e146, _e147);
    uint _e141 = base;
    out[_e141 + 12u] = static_cast<uint>(_e148.x);
    metal::int2 _e155 = first;
    metal::int2 _e156 = second;
    metal::int2 _e157 = first;
    metal::int2 _e158 = operator_u0026_u0026_2(_e156, _e157);
    metal::int2 _e159 = operator_u007c_u007c_2(_e155, _e158);
    uint _e152 = base;
    out[_e152 + 13u] = static_cast<uint>(_e159.y);
    metal::int2 _e166 = first;
    metal::int2 _e167 = second;
    metal::int2 _e168 = operator_u007c_u007c_2(_e166, _e167);
    metal::int2 _e169 = first;
    metal::int2 _e170 = operator_u0026_u0026_2(_e168, _e169);
    uint _e163 = base;
    out[_e163 + 14u] = static_cast<uint>(_e170.x);
    if (true) {
        Sample_cuda_record_0_ _e179 = left;
        local = _e179;
    } else {
        Sample_cuda_record_0_ _e180 = right;
        local = _e180;
    }
    Sample_cuda_record_0_ _e181 = local;
    int _e183 = operator_u0026_u0026_1(_e181, 1);
    uint _e174 = base;
    out[_e174 + 15u] = static_cast<uint>(_e183);
    if (true) {
        Sample_cuda_record_0_ _e190 = left;
        Sample_cuda_record_0_ _e191 = right;
        Sample_cuda_record_0_ _e192 = operator_u0026_u0026_(_e190, _e191);
        local_1 = _e192;
    } else {
        Sample_cuda_record_0_ _e193 = right;
        Sample_cuda_record_0_ _e194 = left;
        Sample_cuda_record_0_ _e195 = operator_u007c_u007c(_e193, _e194);
        local_1 = _e195;
    }
    Sample_cuda_record_0_ _e197 = local_1;
    uint _e186 = base;
    out[_e186 + 16u] = static_cast<uint>(_e197.value);
    counter = 0;
    if (false) {
        int _e205 = counter;
        counter = as_type<int>(as_type<uint>(_e205) + as_type<uint>(1));
        local_2.value = _e205;
        local_2.payload.inner[0] = 1;
        local_2.payload.inner[1] = 2;
        Sample_cuda_record_0_ _e214 = local_2;
        Sample_cuda_record_0_ _e215 = right;
        Sample_cuda_record_0_ _e216 = operator_u0026_u0026_(_e214, _e215);
        local_4 = _e216;
    } else {
        Sample_cuda_record_0_ _e217 = left;
        int _e219 = counter;
        counter = as_type<int>(as_type<uint>(_e219) + as_type<uint>(1));
        local_3.value = _e219;
        local_3.payload.inner[0] = 1;
        local_3.payload.inner[1] = 2;
        Sample_cuda_record_0_ _e228 = local_3;
        Sample_cuda_record_0_ _e229 = operator_u007c_u007c(_e217, _e228);
        local_4 = _e229;
    }
    Sample_cuda_record_0_ _e231 = local_4;
    evaluated = _e231;
    int _e237 = evaluated.value;
    uint _e233 = base;
    out[_e233 + 17u] = static_cast<uint>(_e237);
    int _e243 = counter;
    uint _e240 = base;
    out[_e240 + 18u] = static_cast<uint>(_e243);
    local_5 = false;
    if (false) {
        Sample_cuda_record_0_ _e250 = left;
        Sample_cuda_record_0_ _e251 = right;
        Sample_cuda_record_0_ _e252 = operator_u0026_u0026_(_e250, _e251);
        local_5 = _e252.value != 0;
    }
    bool _e257 = local_5;
    uint _e246 = base;
    out[_e246 + 19u] = static_cast<uint>(_e257);
    local_6 = true;
    if (true) {
    } else {
        Sample_cuda_record_0_ _e264 = left;
        Sample_cuda_record_0_ _e265 = right;
        Sample_cuda_record_0_ _e266 = operator_u007c_u007c(_e264, _e265);
        local_6 = _e266.value != 0;
    }
    bool _e271 = local_6;
    uint _e260 = base;
    out[_e260 + 20u] = static_cast<uint>(_e271);
    bool _e280 = 2 != 0;
    local_7 = _e280;
    if (_e280) {
        local_7 = 3 != 0;
    }
    bool _e284 = local_7;
    uint _e274 = base;
    out[_e274 + 21u] = static_cast<uint>(_e284);
    bool _e293 = 0 != 0;
    local_8 = _e293;
    if (_e293) {
    } else {
        local_8 = 4 != 0;
    }
    bool _e297 = local_8;
    uint _e287 = base;
    out[_e287 + 22u] = static_cast<uint>(_e297);
    uint _e300 = base;
    out[_e300 + 23u] = threadIdx.x;
}
