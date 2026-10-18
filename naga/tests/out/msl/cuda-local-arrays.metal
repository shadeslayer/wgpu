// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

struct type_1 {
    int inner[3];
};
struct type_2 {
    int inner[2];
};
typedef int type_3[1];
struct type_5 {
    uint inner[2];
};
struct type_7 {
    float inner[2];
};
struct type_9 {
    bool inner[3];
};
struct type_10 {
    bool inner[2];
};

int local_sum(
    int value
) {
    int value_1 = {};
    type_1 values = {};
    value_1 = value;
    values = type_1 {};
    int _e6 = value_1;
    values.inner[0] = _e6;
    int _e10 = value_1;
    values.inner[1] = as_type<int>(as_type<uint>(_e10) + as_type<uint>(1));
    int _e18 = values.inner[0];
    int _e19 = values.inner[1];
    values.inner[2] = as_type<int>(as_type<uint>(_e18) + as_type<uint>(_e19));
    int _e23 = values.inner[2];
    return _e23;
}

int reset_sum(
    int count
) {
    int count_1 = {};
    int result = {};
    int i_1 = {};
    type_2 values_1 = {};
    count_1 = count;
    result = 0;
    {
        i_1 = 0;
        uint2 loop_bound = uint2(4294967295u);
        bool loop_init = true;
        while(true) {
            if (metal::all(loop_bound == uint2(0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e9 = i_1;
                i_1 = as_type<int>(as_type<uint>(_e9) + as_type<uint>(1));
            }
            loop_init = false;
            int _e6 = i_1;
            int _e7 = count_1;
            if (_e6 < _e7) {
            } else {
                break;
            }
            values_1 = type_2 {};
            int _e16 = i_1;
            values_1.inner[1] = _e16;
            int _e21 = values_1.inner[0];
            int _e22 = values_1.inner[1];
            int _e24 = result;
            result = as_type<int>(as_type<uint>(_e24) + as_type<uint>(as_type<int>(as_type<uint>(_e21) + as_type<uint>(_e22))));
            values_1.inner[0] = 99;
        }
    }
    int _e29 = result;
    return _e29;
}
int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}

int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

int naga_mod(int lhs, int rhs) {
    int divisor = metal::select(rhs, 1, (lhs == (-2147483647 - 1) & rhs == -1) | (rhs == 0));
    return lhs - (lhs / divisor) * divisor;
}


kernel void local_arrays(
  device type_3& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int seed = {};
    type_1 signed_values = {};
    type_5 words = {};
    type_7 samples = {};
    type_9 flags = {};
    type_2 zeros = {};
    type_10 zero_flags = {};
    bool local = {};
    type_5 bank = {};
    type_7 bank_1 = {};
    type_2 signed_values_1 = {};
    type_2 mixed = {};
    int next = {};
    int reset_total = {};
    int i = {};
    type_2 reset = {};
    int total = {};
    type_2 scratch = {};
    int j = {};
    type_7 zero_samples = {};
    type_2 left = {};
    type_2 right = {};
    type_2 temporary = {};
    type_2 temporary_1 = {};
    bool choose = {};
    int index = {};
    int rhs = {};
    int local_1 = {};
    int local_2 = {};
    int _e4 = out[0];
    seed = _e4;
    int _e8 = seed;
    signed_values.inner[0] = _e8;
    signed_values.inner[1] = naga_neg(2);
    signed_values.inner[2] = 4;
    int _e22 = signed_values.inner[0];
    int _e23 = signed_values.inner[1];
    int _e27 = signed_values.inner[2];
    out[0] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e22) + as_type<uint>(_e23))) + as_type<uint>(_e27));
    words = type_5 {};
    words.inner[0] = 4294967295u;
    words.inner[1] = 1u;
    uint _e43 = words.inner[0];
    uint _e44 = words.inner[1];
    out[1] = static_cast<int>((_e43 + _e44) == 0u);
    samples = type_7 {};
    samples.inner[0] = 1.5;
    samples.inner[1] = 2.25;
    float _e63 = samples.inner[0];
    float _e64 = samples.inner[1];
    out[2] = naga_f2i32((_e63 + _e64) * 4.0);
    flags = type_9 {};
    int _e74 = seed;
    flags.inner[0] = _e74 != 0;
    flags.inner[2] = true;
    bool _e83 = flags.inner[0];
    bool _e89 = flags.inner[1];
    bool _e96 = flags.inner[2];
    out[3] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(static_cast<int>(_e83)) * as_type<uint>(100))) + as_type<uint>(as_type<int>(as_type<uint>(static_cast<int>(_e89)) * as_type<uint>(10))))) + as_type<uint>(static_cast<int>(_e96)));
    zeros = type_2 {};
    int _e107 = zeros.inner[0];
    int _e108 = zeros.inner[1];
    out[4] = as_type<int>(as_type<uint>(_e107) + as_type<uint>(_e108));
    int _e112 = seed;
    int _e113 = local_sum(_e112);
    out[5] = _e113;
    int _e117 = reset_sum(3);
    out[6] = _e117;
    zero_flags = type_10 {};
    bool _e124 = zero_flags.inner[0];
    local = _e124;
    if (_e124) {
    } else {
        bool _e128 = zero_flags.inner[1];
        local = _e128;
    }
    bool _e129 = local;
    out[7] = static_cast<int>(_e129);
    bank = type_5 {};
    bank.inner[0] = 3u;
    bank.inner[1] = 4u;
    {
        bank_1 = type_7 {};
        bank_1.inner[1] = 6.5;
        float _e150 = bank_1.inner[0];
        float _e151 = bank_1.inner[1];
        out[8] = naga_f2i32(_e150 + _e151);
    }
    uint _e160 = bank.inner[0];
    uint _e161 = bank.inner[1];
    out[9] = static_cast<int>(_e160 + _e161);
    {
        int _e167 = seed;
        signed_values_1.inner[0] = _e167;
        int _e170 = seed;
        signed_values_1.inner[1] = _e170;
        int _e177 = signed_values_1.inner[0];
        int _e178 = signed_values_1.inner[1];
        out[10] = as_type<int>(as_type<uint>(_e177) + as_type<uint>(_e178));
    }
    int _e184 = signed_values.inner[0];
    out[11] = _e184;
    mixed = type_2 {};
    int _e188 = seed;
    seed = as_type<int>(as_type<uint>(_e188) + as_type<uint>(1));
    int _e191 = seed;
    next = _e191;
    int _e194 = next;
    mixed.inner[0] = _e194;
    int _e197 = seed;
    seed = as_type<int>(as_type<uint>(_e197) + as_type<uint>(1));
    mixed.inner[1] = _e197;
    int _e205 = mixed.inner[0];
    int _e209 = mixed.inner[1];
    out[12] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e205) * as_type<uint>(10))) + as_type<uint>(_e209));
    int _e213 = seed;
    out[13] = _e213;
    reset_total = 0;
    {
        i = 0;
        uint2 loop_bound_1 = uint2(4294967295u);
        bool loop_init_1 = true;
        while(true) {
            if (metal::all(loop_bound_1 == uint2(0u))) { break; }
            loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
            if (!loop_init_1) {
                int _e221 = i;
                i = as_type<int>(as_type<uint>(_e221) + as_type<uint>(1));
            }
            loop_init_1 = false;
            int _e219 = i;
            if (_e219 < 3) {
            } else {
                break;
            }
            reset = type_2 {};
            int _e227 = i;
            if (_e227 == 1) {
                reset.inner[0] = 90;
                continue;
            }
            int _e234 = reset.inner[0];
            int _e235 = i;
            int _e237 = reset_total;
            reset_total = as_type<int>(as_type<uint>(_e237) + as_type<uint>(as_type<int>(as_type<uint>(_e234) + as_type<uint>(_e235))));
            reset.inner[0] = 99;
        }
    }
    int _e244 = reset_total;
    out[14] = _e244;
    total = 0;
    {
        scratch = type_2 {};
        j = 0;
        uint2 loop_bound_2 = uint2(4294967295u);
        bool loop_init_2 = true;
        while(true) {
            if (metal::all(loop_bound_2 == uint2(0u))) { break; }
            loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
            if (!loop_init_2) {
                int _e254 = j;
                j = as_type<int>(as_type<uint>(_e254) + as_type<uint>(1));
            }
            loop_init_2 = false;
            int _e252 = j;
            if (_e252 < 3) {
            } else {
                break;
            }
            int _e258 = j;
            int _e262 = scratch.inner[naga_mod(_e258, 2)];
            scratch.inner[naga_mod(_e258, 2)] = as_type<int>(as_type<uint>(_e262) + as_type<uint>(1));
            int _e265 = j;
            if (_e265 == 1) {
                continue;
            }
            int _e270 = scratch.inner[0];
            int _e274 = scratch.inner[1];
            int _e276 = total;
            total = as_type<int>(as_type<uint>(_e276) + as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e270) * as_type<uint>(10))) + as_type<uint>(_e274))));
        }
    }
    int _e280 = total;
    out[15] = _e280;
    zero_samples = type_7 {};
    float _e289 = zero_samples.inner[0];
    float _e290 = zero_samples.inner[1];
    out[16] = naga_f2i32(_e289 + _e290);
    left = type_2 {};
    right = type_2 {};
    left.inner[0] = 5;
    left.inner[1] = 7;
    right.inner[0] = 11;
    right.inner[1] = 13;
    int _e315 = left.inner[0];
    int _e316 = right.inner[0];
    out[17] = as_type<int>(as_type<uint>(_e315) + as_type<uint>(_e316));
    {
        temporary = type_2 {};
        int _e322 = seed;
        temporary.inner[0] = _e322;
        {
            temporary_1 = type_2 {};
            temporary_1.inner[1] = 3;
            int _e334 = temporary_1.inner[0];
            int _e335 = temporary_1.inner[1];
            out[18] = as_type<int>(as_type<uint>(_e334) + as_type<uint>(_e335));
        }
        int _e341 = temporary.inner[0];
        out[19] = _e341;
    }
    int _e344 = seed;
    choose = (_e344 & 1) == 0;
    index = 0;
    rhs = 20;
    int _e363 = rhs;
    rhs = as_type<int>(as_type<uint>(_e363) + as_type<uint>(1));
    int _e366 = rhs;
    bool _e354 = choose;
    if (_e354) {
        int _e355 = index;
        index = as_type<int>(as_type<uint>(_e355) + as_type<uint>(1));
        int _e367 = left.inner[_e355];
        left.inner[_e355] = as_type<int>(as_type<uint>(_e367) + as_type<uint>(_e366));
        int _e372 = left.inner[_e355];
        local_1 = _e372;
    } else {
        int _e359 = index;
        index = as_type<int>(as_type<uint>(_e359) + as_type<uint>(1));
        int _e369 = right.inner[_e359];
        right.inner[_e359] = as_type<int>(as_type<uint>(_e369) + as_type<uint>(_e366));
        int _e373 = right.inner[_e359];
        local_1 = _e373;
    }
    int _e374 = local_1;
    out[20] = _e374;
    int _e377 = index;
    out[21] = _e377;
    int _e383 = left.inner[0];
    int _e387 = right.inner[0];
    out[22] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e383) * as_type<uint>(100))) + as_type<uint>(_e387));
    bool _e391 = choose;
    if (_e391) {
        right.inner[1] = 9;
        int _e398 = right.inner[1];
        right.inner[1] = as_type<int>(as_type<uint>(_e398) + as_type<uint>(1));
        local_2 = _e398;
    } else {
        left.inner[1] = 9;
        int _e401 = left.inner[1];
        left.inner[1] = as_type<int>(as_type<uint>(_e401) + as_type<uint>(1));
        local_2 = _e401;
    }
    int _e404 = local_2;
    out[23] = _e404;
    int _e410 = left.inner[1];
    int _e414 = right.inner[1];
    out[24] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e410) * as_type<uint>(100))) + as_type<uint>(_e414));
}
