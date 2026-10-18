// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

struct type_2 {
    int inner[3];
};
struct int3_ {
    int x;
    int y;
    int z;
};
typedef int type_4[1];
struct type_6 {
    int3_ inner[2];
};
struct type_8 {
    metal::int2 inner[2];
};
struct type_9 {
    int inner[2];
};
struct float3_ {
    float x;
    float y;
    float z;
};
constant int Global = 2;
constant float Scale = 0.5;

int pair(
    int first,
    int second
) {
    int first_1 = {};
    int second_1 = {};
    first_1 = first;
    second_1 = second;
    int _e5 = first_1;
    int _e7 = second_1;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e5) * as_type<uint>(10))) + as_type<uint>(_e7));
}

int list_sum(
    int seed_1
) {
    int seed_2 = {};
    type_2 values_1 = {};
    uint local_4 = {};
    seed_2 = seed_1;
    int _e4 = seed_2;
    values_1.inner[0] = _e4;
    int _e7 = seed_2;
    values_1.inner[1] = as_type<int>(as_type<uint>(_e7) + as_type<uint>(1));
    local_4 = 2u;
    uint2 loop_bound = uint2(4294967295u);
    bool loop_init = true;
    while(true) {
        if (metal::all(loop_bound == uint2(0u))) { break; }
        loop_bound -= uint2(loop_bound.y == 0u, 1u);
        if (!loop_init) {
            uint _e16 = local_4;
            local_4 = _e16 + 1u;
        }
        loop_init = false;
        uint _e12 = local_4;
        if (_e12 >= 3u) {
            break;
        }
        values_1.inner[_e12] = int {};
    }
    int _e23 = values_1.inner[0];
    int _e24 = values_1.inner[1];
    int _e28 = values_1.inner[2];
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e23) + as_type<uint>(_e24))) + as_type<uint>(_e28));
}

int3_ make_list(
    int seed_3
) {
    int seed_4 = {};
    int3_ value = {};
    seed_4 = seed_3;
    int _e3 = seed_4;
    value.x = _e3;
    int _e6 = seed_4;
    value.y = as_type<int>(as_type<uint>(_e6) + as_type<uint>(1));
    int _e10 = seed_4;
    value.z = as_type<int>(as_type<uint>(_e10) + as_type<uint>(2));
    int3_ _e13 = value;
    return _e13;
}
int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}


kernel void list_initialization(
  device type_4& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int seed = {};
    int zero = {};
    int copy_zero = {};
    bool enabled = {};
    uint count = {};
    float sample = {};
    int Local = {};
    int Safe = {};
    int local = {};
    type_2 safe_values = {};
    uint local_1 = {};
    type_2 values = {};
    type_2 changed = {};
    int3_ point = {};
    int3_ changed_point = {};
    type_6 nested = {};
    type_8 flat = {};
    int3_ original = {};
    int3_ copied = {};
    type_6 copies = {};
    int counter = {};
    type_2 ordered = {};
    int called = {};
    int zero_total = {};
    int iteration = {};
    type_2 reset = {};
    uint local_2 = {};
    int total = {};
    int iteration_1 = {};
    int limit = {};
    int Number = {};
    type_9 cached = {};
    type_2 frozen = {};
    uint local_3 = {};
    float3_ floating = {};
    int3_ from_call = {};
    int _e4 = out[0];
    seed = _e4;
    zero = 0;
    copy_zero = 0;
    enabled = true;
    count = static_cast<uint>(7);
    sample = 1.5;
    Local = 3;
    Safe = 3;
    safe_values.inner[0] = 1;
    local_1 = 1u;
    uint2 loop_bound_1 = uint2(4294967295u);
    bool loop_init_1 = true;
    while(true) {
        if (metal::all(loop_bound_1 == uint2(0u))) { break; }
        loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
        if (!loop_init_1) {
            uint _e30 = local_1;
            local_1 = _e30 + 1u;
        }
        loop_init_1 = false;
        uint _e26 = local_1;
        if (_e26 >= 3u) {
            break;
        }
        safe_values.inner[_e26] = int {};
    }
    int _e35 = zero;
    out[0] = _e35;
    int _e38 = copy_zero;
    out[1] = _e38;
    bool _e41 = enabled;
    out[2] = static_cast<int>(_e41);
    uint _e45 = count;
    out[3] = static_cast<int>(_e45);
    float _e50 = sample;
    out[4] = naga_f2i32(_e50 * 2.0);
    int _e56 = Local;
    out[5] = as_type<int>(as_type<uint>(Global) + as_type<uint>(_e56));
    int _e62 = safe_values.inner[2];
    out[6] = _e62;
    values.inner[0] = 1;
    int _e70 = values.inner[0];
    values.inner[1] = as_type<int>(as_type<uint>(_e70) + as_type<uint>(1));
    int _e76 = values.inner[1];
    values.inner[2] = as_type<int>(as_type<uint>(_e76) + as_type<uint>(1));
    int _e84 = values.inner[0];
    int _e85 = values.inner[1];
    int _e89 = values.inner[2];
    out[7] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e84) + as_type<uint>(_e85))) + as_type<uint>(_e89));
    changed.inner[0] = 1;
    changed.inner[0] = 7;
    changed.inner[1] = 2;
    int _e102 = changed.inner[0];
    changed.inner[2] = _e102;
    int _e107 = changed.inner[0];
    out[8] = _e107;
    int _e112 = changed.inner[2];
    out[9] = _e112;
    point.x = 1;
    int _e118 = point.x;
    point.y = as_type<int>(as_type<uint>(_e118) + as_type<uint>(1));
    int _e123 = point.y;
    point.z = as_type<int>(as_type<uint>(_e123) + as_type<uint>(1));
    int _e129 = point.z;
    out[10] = _e129;
    changed_point.x = 1;
    changed_point.x = 9;
    changed_point.y = 2;
    int _e138 = changed_point.x;
    changed_point.z = _e138;
    int _e143 = changed_point.x;
    out[11] = _e143;
    int _e147 = changed_point.z;
    out[12] = _e147;
    nested.inner[0].x = 1;
    nested.inner[0].y = 2;
    nested.inner[0].z = int {};
    nested.inner[1].x = 3;
    nested.inner[1].y = int {};
    nested.inner[1].z = int {};
    int _e171 = nested.inner[0].z;
    int _e172 = nested.inner[1].x;
    out[13] = as_type<int>(as_type<uint>(_e171) + as_type<uint>(_e172));
    flat.inner[0].x = 1;
    flat.inner[0].y = 2;
    flat.inner[1].x = 3;
    flat.inner[1].y = int {};
    int _e193 = flat.inner[0].x;
    int _e194 = flat.inner[0].y;
    int _e199 = flat.inner[1].x;
    int _e204 = flat.inner[1].y;
    out[14] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e193) + as_type<uint>(_e194))) + as_type<uint>(_e199))) + as_type<uint>(_e204));
    original.x = 2;
    original.y = 3;
    original.z = 4;
    int3_ _e214 = original;
    copied = _e214;
    int3_ _e217 = original;
    copies.inner[0] = _e217;
    copies.inner[1].x = 5;
    copies.inner[1].y = 6;
    copies.inner[1].z = int {};
    original.x = 99;
    int _e230 = copied.x;
    out[15] = _e230;
    int _e236 = copies.inner[0].x;
    out[16] = _e236;
    int _e242 = copies.inner[1].z;
    out[17] = _e242;
    int _e244 = seed;
    counter = _e244;
    int _e247 = counter;
    counter = as_type<int>(as_type<uint>(_e247) + as_type<uint>(1));
    ordered.inner[0] = _e247;
    int _e251 = counter;
    counter = as_type<int>(as_type<uint>(_e251) + as_type<uint>(1));
    ordered.inner[1] = _e251;
    int _e255 = counter;
    ordered.inner[2] = _e255;
    int _e261 = ordered.inner[0];
    int _e266 = ordered.inner[1];
    int _e271 = ordered.inner[2];
    out[18] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e261) * as_type<uint>(100))) + as_type<uint>(as_type<int>(as_type<uint>(_e266) * as_type<uint>(10))))) + as_type<uint>(_e271));
    int _e275 = counter;
    out[19] = _e275;
    int _e277 = counter;
    counter = as_type<int>(as_type<uint>(_e277) + as_type<uint>(1));
    int _e281 = pair(_e277, 4);
    called = _e281;
    int _e284 = called;
    out[20] = _e284;
    zero_total = 0;
    {
        iteration = 0;
        uint2 loop_bound_2 = uint2(4294967295u);
        bool loop_init_2 = true;
        while(true) {
            if (metal::all(loop_bound_2 == uint2(0u))) { break; }
            loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
            if (!loop_init_2) {
                int _e292 = iteration;
                iteration = as_type<int>(as_type<uint>(_e292) + as_type<uint>(1));
            }
            loop_init_2 = false;
            int _e290 = iteration;
            if (_e290 < 3) {
            } else {
                break;
            }
            int _e297 = iteration;
            reset.inner[0] = _e297;
            local_2 = 1u;
            uint2 loop_bound_3 = uint2(4294967295u);
            bool loop_init_3 = true;
            while(true) {
                if (metal::all(loop_bound_3 == uint2(0u))) { break; }
                loop_bound_3 -= uint2(loop_bound_3.y == 0u, 1u);
                if (!loop_init_3) {
                    uint _e305 = local_2;
                    local_2 = _e305 + 1u;
                }
                loop_init_3 = false;
                uint _e301 = local_2;
                if (_e301 >= 3u) {
                    break;
                }
                reset.inner[_e301] = int {};
            }
            int _e310 = reset.inner[2];
            int _e311 = zero_total;
            zero_total = as_type<int>(as_type<uint>(_e311) + as_type<uint>(_e310));
            reset.inner[2] = 9;
        }
    }
    int _e318 = zero_total;
    out[21] = _e318;
    total = 0;
    {
        iteration_1 = 0;
        limit = 3;
        uint2 loop_bound_4 = uint2(4294967295u);
        bool loop_init_4 = true;
        while(true) {
            if (metal::all(loop_bound_4 == uint2(0u))) { break; }
            loop_bound_4 -= uint2(loop_bound_4.y == 0u, 1u);
            if (!loop_init_4) {
                int _e328 = iteration_1;
                iteration_1 = as_type<int>(as_type<uint>(_e328) + as_type<uint>(1));
            }
            loop_init_4 = false;
            int _e325 = iteration_1;
            int _e326 = limit;
            if (_e325 < _e326) {
            } else {
                break;
            }
            int _e331 = iteration_1;
            int _e332 = total;
            total = as_type<int>(as_type<uint>(_e332) + as_type<uint>(_e331));
        }
    }
    int _e336 = total;
    out[22] = _e336;
    Number = 4;
    int _e341 = Number;
    cached.inner[0] = _e341;
    int _e344 = Number;
    cached.inner[1] = as_type<int>(as_type<uint>(_e344) + as_type<uint>(1));
    int _e350 = cached.inner[1];
    out[23] = _e350;
    frozen.inner[0] = 3;
    local_3 = 1u;
    uint2 loop_bound_5 = uint2(4294967295u);
    bool loop_init_5 = true;
    while(true) {
        if (metal::all(loop_bound_5 == uint2(0u))) { break; }
        loop_bound_5 -= uint2(loop_bound_5.y == 0u, 1u);
        if (!loop_init_5) {
            uint _e361 = local_3;
            local_3 = _e361 + 1u;
        }
        loop_init_5 = false;
        uint _e357 = local_3;
        if (_e357 >= 3u) {
            break;
        }
        frozen.inner[_e357] = int {};
    }
    int _e370 = frozen.inner[0];
    int _e371 = frozen.inner[1];
    int _e375 = frozen.inner[2];
    out[24] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e370) + as_type<uint>(_e371))) + as_type<uint>(_e375));
    floating.x = 1.5;
    floating.y = -(0.0);
    floating.z = float {};
    float _e388 = floating.y;
    out[25] = as_type<int>(_e388);
    float _e393 = floating.z;
    out[26] = as_type<int>(_e393);
    int3_ _e397 = make_list(6);
    from_call = _e397;
    int _e402 = from_call.x;
    int _e403 = from_call.z;
    out[27] = as_type<int>(as_type<uint>(_e402) + as_type<uint>(_e403));
    out[28] = naga_f2i32(Scale * 10.0);
    int _e414 = seed;
    int _e416 = list_sum(as_type<int>(as_type<uint>(_e414) + as_type<uint>(3)));
    out[29] = _e416;
}
