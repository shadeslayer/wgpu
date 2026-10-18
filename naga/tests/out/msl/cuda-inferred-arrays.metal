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
struct int3_ {
    int x;
    int y;
    int z;
};
typedef int type_2[1];
struct type_3 {
    int inner[3];
};
struct type_5 {
    bool inner[3];
};
struct type_7 {
    uint inner[2];
};
struct type_9 {
    float inner[2];
};
struct type_10 {
    int3_ inner[2];
};
struct type_12 {
    metal::int2 inner[2];
};
struct type_13 {
    int3_ inner[3];
};
struct float3_ {
    float x;
    float y;
    float z;
};
struct type_14 {
    float3_ inner[2];
};

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

int sum(
    int seed_1
) {
    int seed_2 = {};
    type_1 values_1 = {};
    seed_2 = seed_1;
    int _e4 = seed_2;
    values_1.inner[0] = _e4;
    int _e7 = seed_2;
    values_1.inner[1] = as_type<int>(as_type<uint>(_e7) + as_type<uint>(1));
    int _e13 = values_1.inner[0];
    int _e14 = values_1.inner[1];
    return as_type<int>(as_type<uint>(_e13) + as_type<uint>(_e14));
}

int3_ triple(
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


kernel void inferred_arrays(
  device type_2& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int seed = {};
    type_3 values = {};
    type_1 copied = {};
    type_1 zeroes = {};
    type_5 flags = {};
    type_7 counts = {};
    type_9 samples = {};
    type_3 self = {};
    type_3 changed = {};
    type_10 points = {};
    type_12 flat = {};
    int3_ original = {};
    type_13 copies = {};
    type_1 Number = {};
    type_1 next = {};
    int Count = {};
    type_3 scratch = {};
    type_9 pending = {};
    type_14 pending_vectors = {};
    int counter = {};
    type_3 ordered = {};
    type_1 called = {};
    int zero_total = {};
    int iteration = {};
    type_12 reset = {};
    int total = {};
    int iteration_1 = {};
    type_1 frozen = {};
    type_9 signed_zeroes = {};
    type_10 from_calls = {};
    int _e4 = out[0];
    seed = _e4;
    int _e7 = seed;
    values.inner[0] = _e7;
    int _e10 = seed;
    values.inner[1] = as_type<int>(as_type<uint>(_e10) + as_type<uint>(1));
    int _e14 = seed;
    values.inner[2] = as_type<int>(as_type<uint>(_e14) + as_type<uint>(2));
    copied.inner[0] = 4;
    copied.inner[1] = 5;
    zeroes.inner[0] = 0;
    zeroes.inner[1] = 0;
    flags.inner[0] = true;
    flags.inner[1] = false;
    flags.inner[2] = true;
    counts.inner[0] = 3u;
    counts.inner[1] = 5u;
    samples.inner[0] = 1.5;
    samples.inner[1] = 2.5;
    int _e49 = values.inner[0];
    int _e50 = values.inner[1];
    int _e54 = values.inner[2];
    out[0] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e49) + as_type<uint>(_e50))) + as_type<uint>(_e54));
    int _e60 = copied.inner[0];
    out[1] = _e60;
    int _e67 = zeroes.inner[0];
    int _e68 = zeroes.inner[1];
    out[2] = as_type<int>(as_type<uint>(_e67) + as_type<uint>(_e68));
    bool _e74 = flags.inner[0];
    bool _e78 = flags.inner[1];
    bool _e83 = flags.inner[2];
    out[3] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(static_cast<int>(_e74)) + as_type<uint>(static_cast<int>(_e78)))) + as_type<uint>(static_cast<int>(_e83)));
    uint _e92 = counts.inner[0];
    uint _e93 = counts.inner[1];
    out[4] = static_cast<int>(_e92 + _e93);
    float _e102 = samples.inner[0];
    float _e103 = samples.inner[1];
    out[5] = naga_f2i32(_e102 + _e103);
    self.inner[0] = 1;
    int _e113 = self.inner[0];
    self.inner[1] = as_type<int>(as_type<uint>(_e113) + as_type<uint>(1));
    int _e119 = self.inner[1];
    self.inner[2] = as_type<int>(as_type<uint>(_e119) + as_type<uint>(1));
    int _e126 = self.inner[0];
    int _e131 = self.inner[1];
    int _e136 = self.inner[2];
    out[6] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e126) * as_type<uint>(100))) + as_type<uint>(as_type<int>(as_type<uint>(_e131) * as_type<uint>(10))))) + as_type<uint>(_e136));
    changed.inner[0] = 1;
    changed.inner[0] = 7;
    changed.inner[1] = 2;
    int _e149 = changed.inner[0];
    changed.inner[2] = _e149;
    int _e154 = changed.inner[0];
    out[7] = _e154;
    int _e159 = changed.inner[2];
    out[8] = _e159;
    points.inner[0].x = 1;
    int _e168 = points.inner[0].x;
    points.inner[0].y = as_type<int>(as_type<uint>(_e168) + as_type<uint>(1));
    int _e175 = points.inner[0].y;
    points.inner[0].z = as_type<int>(as_type<uint>(_e175) + as_type<uint>(1));
    int3_ _e181 = points.inner[0];
    points.inner[1] = _e181;
    int _e187 = points.inner[1].z;
    out[9] = _e187;
    flat.inner[0].x = 1;
    flat.inner[0].y = 2;
    flat.inner[1].x = 3;
    flat.inner[1].y = int {};
    int _e204 = flat.inner[1].y;
    out[10] = _e204;
    int _e210 = flat.inner[1].x;
    out[11] = _e210;
    original.x = 4;
    original.y = 5;
    original.z = 6;
    int3_ _e220 = original;
    copies.inner[0] = _e220;
    int3_ _e222 = original;
    copies.inner[1] = _e222;
    copies.inner[2].x = 7;
    copies.inner[2].y = 8;
    copies.inner[2].z = int {};
    original.x = 99;
    int _e237 = copies.inner[0].x;
    out[12] = _e237;
    int _e243 = copies.inner[1].y;
    out[13] = _e243;
    int _e249 = copies.inner[2].z;
    out[14] = _e249;
    Number.inner[0] = 2;
    Number.inner[1] = 3;
    int _e259 = Number.inner[1];
    next.inner[0] = _e259;
    int _e263 = Number.inner[0];
    next.inner[1] = _e263;
    int _e268 = next.inner[0];
    out[15] = _e268;
    Count = 3;
    scratch = type_3 {};
    int _e277 = scratch.inner[2];
    out[16] = _e277;
    pending.inner[0] = 1.0;
    pending.inner[1] = static_cast<float>(2);
    float _e290 = pending.inner[0];
    float _e291 = pending.inner[1];
    out[17] = naga_f2i32(_e290 + _e291);
    pending_vectors.inner[0].x = 1.0;
    pending_vectors.inner[0].y = static_cast<float>(2);
    pending_vectors.inner[0].z = float {};
    pending_vectors.inner[1].x = 3.0;
    pending_vectors.inner[1].y = float {};
    pending_vectors.inner[1].z = float {};
    float _e318 = pending_vectors.inner[0].x;
    float _e319 = pending_vectors.inner[0].y;
    float _e324 = pending_vectors.inner[1].x;
    out[18] = naga_f2i32((_e318 + _e319) + _e324);
    int _e328 = seed;
    counter = _e328;
    int _e331 = counter;
    counter = as_type<int>(as_type<uint>(_e331) + as_type<uint>(1));
    ordered.inner[0] = _e331;
    int _e335 = counter;
    counter = as_type<int>(as_type<uint>(_e335) + as_type<uint>(1));
    ordered.inner[1] = _e335;
    int _e339 = counter;
    ordered.inner[2] = _e339;
    int _e345 = ordered.inner[0];
    int _e350 = ordered.inner[1];
    int _e355 = ordered.inner[2];
    out[19] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e345) * as_type<uint>(100))) + as_type<uint>(as_type<int>(as_type<uint>(_e350) * as_type<uint>(10))))) + as_type<uint>(_e355));
    int _e359 = counter;
    out[20] = _e359;
    int _e362 = counter;
    counter = as_type<int>(as_type<uint>(_e362) + as_type<uint>(1));
    int _e366 = pair(_e362, 4);
    called.inner[0] = _e366;
    int _e368 = counter;
    called.inner[1] = _e368;
    int _e375 = called.inner[0];
    int _e376 = called.inner[1];
    out[21] = as_type<int>(as_type<uint>(_e375) + as_type<uint>(_e376));
    int _e380 = counter;
    out[22] = _e380;
    zero_total = 0;
    {
        iteration = 0;
        uint2 loop_bound = uint2(4294967295u);
        bool loop_init = true;
        while(true) {
            if (metal::all(loop_bound == uint2(0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e388 = iteration;
                iteration = as_type<int>(as_type<uint>(_e388) + as_type<uint>(1));
            }
            loop_init = false;
            int _e386 = iteration;
            if (_e386 < 3) {
            } else {
                break;
            }
            int _e393 = iteration;
            reset.inner[0].x = _e393;
            reset.inner[0].y = int {};
            reset.inner[1] = metal::int2 {};
            int _e405 = reset.inner[0].y;
            int _e406 = reset.inner[1].x;
            int _e408 = zero_total;
            zero_total = as_type<int>(as_type<uint>(_e408) + as_type<uint>(as_type<int>(as_type<uint>(_e405) + as_type<uint>(_e406))));
            reset.inner[0].y = 9;
            reset.inner[1].x = 8;
        }
    }
    int _e420 = zero_total;
    out[23] = _e420;
    total = 0;
    {
        iteration_1 = 0;
        uint2 loop_bound_1 = uint2(4294967295u);
        bool loop_init_1 = true;
        while(true) {
            if (metal::all(loop_bound_1 == uint2(0u))) { break; }
            loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
            if (!loop_init_1) {
                int _e428 = iteration_1;
                iteration_1 = as_type<int>(as_type<uint>(_e428) + as_type<uint>(1));
            }
            loop_init_1 = false;
            int _e426 = iteration_1;
            if (_e426 < 4) {
            } else {
                break;
            }
            int _e432 = iteration_1;
            if (_e432 == 1) {
                continue;
            }
            int _e434 = iteration_1;
            int _e435 = total;
            total = as_type<int>(as_type<uint>(_e435) + as_type<uint>(_e434));
        }
    }
    int _e439 = total;
    out[24] = _e439;
    int _e443 = sum(4);
    out[25] = _e443;
    frozen.inner[0] = 2;
    frozen.inner[1] = 3;
    int _e455 = frozen.inner[0];
    int _e456 = frozen.inner[1];
    out[26] = as_type<int>(as_type<uint>(_e455) + as_type<uint>(_e456));
    signed_zeroes.inner[0] = -(0.0);
    signed_zeroes.inner[1] = 0.0;
    float _e468 = signed_zeroes.inner[0];
    out[27] = as_type<int>(_e468);
    float _e474 = signed_zeroes.inner[1];
    out[28] = as_type<int>(_e474);
    int3_ _e479 = triple(6);
    from_calls.inner[0] = _e479;
    int3_ _e482 = triple(9);
    from_calls.inner[1] = _e482;
    int _e491 = from_calls.inner[0].x;
    int _e492 = from_calls.inner[1].z;
    out[29] = as_type<int>(as_type<uint>(_e491) + as_type<uint>(_e492));
}
