// language: metal1.2
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef uint type_4[1];
struct type_5 {
    int inner[2];
};

int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

int signed_conversion(
    float first_1,
    float second_1
) {
    float first_2 = {};
    float second_2 = {};
    first_2 = first_1;
    second_2 = second_1;
    float _e4 = first_2;
    float _e5 = second_2;
    return as_type<int>(as_type<uint>(metal::extract_bits(naga_f2i32(_e4), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(naga_f2i32(_e5), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))));
}

uint naga_f2u32(float value) {
    return static_cast<uint>(metal::clamp(value, 0.0, 4294967000.0));
}

uint unsigned_conversion(
    float first_3,
    float second_3
) {
    float first_4 = {};
    float second_4 = {};
    first_4 = first_3;
    second_4 = second_3;
    float _e4 = first_4;
    float _e5 = second_4;
    return metal::extract_bits(naga_f2u32(_e4), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))) * metal::extract_bits(naga_f2u32(_e5), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)));
}

int _mul24_(
    bool first_5,
    bool second_5
) {
    bool first_6 = {};
    bool second_6 = {};
    int local_2 = {};
    int local_3 = {};
    first_6 = first_5;
    second_6 = second_5;
    bool _e4 = first_6;
    if (_e4) {
        bool _e5 = second_6;
        if (_e5) {
            local_2 = 71;
        } else {
            local_2 = 73;
        }
        int _e9 = local_2;
        local_3 = _e9;
    } else {
        local_3 = 79;
    }
    int _e12 = local_3;
    return _e12;
}

int _mul24_1(
    float first_7,
    float second_7
) {
    float first_8 = {};
    float second_8 = {};
    first_8 = first_7;
    second_8 = second_7;
    float _e5 = first_8;
    float _e7 = second_8;
    return naga_f2i32((_e5 * 10.0) + _e7);
}

uint _umul24_(
    int first_9,
    int second_9
) {
    int first_10 = {};
    int second_10 = {};
    first_10 = first_9;
    second_10 = second_9;
    int _e4 = first_10;
    int _e5 = second_10;
    return static_cast<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e4) + as_type<uint>(_e5))) + as_type<uint>(100)));
}

int signed_pair(
    int first_11,
    int second_11
) {
    int first_12 = {};
    int second_12 = {};
    first_12 = first_11;
    second_12 = second_11;
    int _e4 = first_12;
    int _e5 = second_12;
    return as_type<int>(as_type<uint>(metal::extract_bits(_e4, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(_e5, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))));
}
int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}


kernel void multiply24_(
  device type_4& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int first = {};
    uint second = {};
    int local = {};
    type_5 values = {};
    int Count = {};
    int local_1 = {};
    type_5 scratch = {};
    int counter = {};
    int total = {};
    int index = {};
    out[0] = static_cast<uint>(as_type<int>(as_type<uint>(metal::extract_bits(8388608, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(1, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))))));
    out[1] = metal::extract_bits(16777215u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))) * metal::extract_bits(16777215u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)));
    out[2] = static_cast<uint>(as_type<int>(as_type<uint>(metal::extract_bits(8388607, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(8388607, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))))));
    out[3] = metal::extract_bits(16777216u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))) * metal::extract_bits(13u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)));
    int _e50 = signed_conversion(3.75, -(2.75));
    out[4] = static_cast<uint>(_e50);
    int _e57 = signed_conversion(-(3.75), 2.75);
    out[5] = static_cast<uint>(_e57);
    uint _e63 = unsigned_conversion(3.75, 2.75);
    out[6] = _e63;
    uint _e68 = unsigned_conversion(16777215.0, 1.0);
    out[7] = _e68;
    int _e73 = _mul24_(true, true);
    out[8] = static_cast<uint>(_e73);
    int _e79 = _mul24_(false, true);
    out[9] = static_cast<uint>(_e79);
    out[10] = static_cast<uint>(as_type<int>(as_type<uint>(metal::extract_bits(3, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(static_cast<int>(true), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))))));
    out[11] = static_cast<uint>(as_type<int>(as_type<uint>(metal::extract_bits(static_cast<int>(true), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(3, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))))));
    int _e113 = _mul24_1(3.75, 2.0);
    out[12] = static_cast<uint>(_e113);
    uint _e119 = _umul24_(3, 4);
    out[13] = _e119;
    out[14] = metal::extract_bits(3u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))) * metal::extract_bits(4u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)));
    uint _e135 = _umul24_(static_cast<int>(true), static_cast<int>(true));
    out[15] = _e135;
    out[16] = metal::extract_bits(3u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))) * metal::extract_bits(static_cast<uint>(4), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)));
    int _e151 = signed_pair(naga_neg(7), 3);
    out[17] = static_cast<uint>(_e151);
    out[18] = static_cast<uint>(as_type<int>(as_type<uint>(metal::extract_bits(as_type<int>(as_type<uint>(metal::extract_bits(3, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(4, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))))), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(5, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))))));
    out[19] = metal::extract_bits(metal::extract_bits(16777215u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))) * metal::extract_bits(2u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))) * metal::extract_bits(2u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)));
    first = -3;
    second = 5u;
    int _e196 = first;
    uint _e197 = second;
    out[20] = static_cast<uint>(as_type<int>(as_type<uint>(metal::extract_bits(_e196, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(static_cast<int>(_e197), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))))));
    int _e210 = first;
    uint _e212 = second;
    out[21] = metal::extract_bits(static_cast<uint>(_e210), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))) * metal::extract_bits(_e212, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)));
    out[22] = static_cast<uint>(as_type<int>(as_type<uint>(metal::extract_bits(25165824, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(16777217, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))))));
    out[23] = metal::extract_bits(4286578688u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))) * metal::extract_bits(4278190081u, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)));
    if (true) {
        local = as_type<int>(as_type<uint>(metal::extract_bits(naga_neg(3), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(7, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))));
    } else {
        local = as_type<int>(as_type<uint>(metal::extract_bits(0, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(0, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))));
    }
    int _e265 = local;
    out[24] = static_cast<uint>(_e265);
    values.inner[0] = as_type<int>(as_type<uint>(metal::extract_bits(2, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(3, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))));
    values.inner[1] = as_type<int>(as_type<uint>(metal::extract_bits(3, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(4, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))));
    int _e296 = values.inner[0];
    int _e297 = values.inner[1];
    out[25] = static_cast<uint>(as_type<int>(as_type<uint>(_e296) + as_type<uint>(_e297)));
    Count = 2;
    scratch = type_5 {};
    int _e308 = Count;
    int _e309 = scratch.inner[0];
    out[26] = static_cast<uint>(as_type<int>(as_type<uint>(_e308) + as_type<uint>(_e309)));
    counter = 2;
    int _e316 = counter;
    counter = as_type<int>(as_type<uint>(_e316) + as_type<uint>(1));
    int _e319 = counter;
    counter = as_type<int>(as_type<uint>(_e319) + as_type<uint>(1));
    out[27] = static_cast<uint>(as_type<int>(as_type<uint>(metal::extract_bits(_e316, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(_e319, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))))));
    int _e333 = counter;
    out[28] = static_cast<uint>(_e333);
    total = 0;
    {
        index = 0;
        uint2 loop_bound = uint2(4294967295u);
        bool loop_init = true;
        while(true) {
            if (metal::all(loop_bound == uint2(0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e342 = index;
                index = as_type<int>(as_type<uint>(_e342) + as_type<uint>(1));
            }
            loop_init = false;
            int _e340 = index;
            if (_e340 < 3) {
            } else {
                break;
            }
            int _e346 = index;
            int _e357 = total;
            total = as_type<int>(as_type<uint>(_e357) + as_type<uint>(as_type<int>(as_type<uint>(metal::extract_bits(as_type<int>(as_type<uint>(_e346) + as_type<uint>(1)), metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(2, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))))));
        }
    }
    int _e361 = total;
    out[29] = static_cast<uint>(_e361);
    {
        out[30] = static_cast<uint>(naga_f2i32(9.75));
    }
    out[31] = static_cast<uint>(as_type<int>(as_type<uint>(metal::extract_bits(16777215, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u)))) * as_type<uint>(metal::extract_bits(3, metal::min(0u, 32u), metal::min(24u, 32u - metal::min(0u, 32u))))));
}
