// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef int type_1[1];
struct type_3 {
    metal::int2 inner[2];
};
struct type_4 {
    type_3 inner[2];
};
struct int3_ {
    int x;
    int y;
    int z;
};
struct type_5 {
    int3_ inner[2];
};
struct type_6 {
    type_5 inner[2];
};
struct type_8 {
    metal::int4 inner[2];
};
struct type_9 {
    type_8 inner[2];
};
struct type_12 {
    metal::uint2 inner[2];
};
struct type_13 {
    type_12 inner[2];
};
struct uint3_ {
    uint x;
    uint y;
    uint z;
};
struct type_14 {
    uint3_ inner[2];
};
struct type_15 {
    type_14 inner[2];
};
struct type_17 {
    metal::uint4 inner[2];
};
struct type_18 {
    type_17 inner[2];
};
struct type_21 {
    metal::float2 inner[2];
};
struct type_22 {
    type_21 inner[2];
};
struct float3_ {
    float x;
    float y;
    float z;
};
struct type_23 {
    float3_ inner[2];
};
struct type_24 {
    type_23 inner[2];
};
struct type_26 {
    metal::float4 inner[2];
};
struct type_27 {
    type_26 inner[2];
};
struct type_28 {
    int inner[2];
};
struct type_29 {
    type_28 inner[2];
};
struct type_30 {
    int inner[3];
};
struct type_31 {
    type_30 inner[2];
};
struct type_32 {
    type_31 inner[2];
};
int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}


kernel void multidimensional_arrays(
  uint __local_invocation_index [[thread_index_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, threadgroup type_31& shared_values
, threadgroup type_24& shared_points
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    if (__local_invocation_index == 0u) {
        shared_values = {};
        shared_points = {};
    }
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    metal::int2 original = {};
    type_4 points = {};
    int3_ original_1 = {};
    type_6 points_1 = {};
    metal::int4 original_2 = {};
    type_9 points_2 = {};
    metal::uint2 original_3 = {};
    type_13 points_3 = {};
    uint3_ original_4 = {};
    type_15 points_4 = {};
    metal::uint4 original_5 = {};
    type_18 points_5 = {};
    metal::float2 original_6 = {};
    type_22 points_6 = {};
    float3_ original_7 = {};
    type_24 points_7 = {};
    metal::float4 original_8 = {};
    type_27 points_8 = {};
    type_29 changed = {};
    uint local = {};
    type_31 nested_empty = {};
    uint local_1 = {};
    uint local_2 = {};
    type_31 partial = {};
    uint local_3 = {};
    uint local_4 = {};
    uint local_5 = {};
    type_32 volume = {};
    uint local_6 = {};
    uint local_7 = {};
    uint local_8 = {};
    uint local_9 = {};
    uint local_10 = {};
    uint local_11 = {};
    {
        original.x = 1;
        original.y = 2;
        metal::int2 _e9 = original;
        points.inner[0].inner[0] = _e9;
        points.inner[0].inner[1] = metal::int2 {};
        metal::int2 _e14 = original;
        points.inner[1].inner[0] = _e14;
        points.inner[1].inner[1].x = 5;
        points.inner[1].inner[1].y = int {};
        int _e27 = points.inner[0].inner[0].y;
        out[0] = _e27;
        int _e35 = points.inner[1].inner[1].x;
        out[1] = _e35;
        int _e43 = points.inner[1].inner[1].y;
        out[2] = _e43;
    }
    {
        original_1.x = 1;
        original_1.y = 2;
        original_1.z = 3;
        int3_ _e54 = original_1;
        points_1.inner[0].inner[0] = _e54;
        points_1.inner[0].inner[1] = int3_ {};
        int3_ _e59 = original_1;
        points_1.inner[1].inner[0] = _e59;
        points_1.inner[1].inner[1].x = 5;
        points_1.inner[1].inner[1].y = int {};
        points_1.inner[1].inner[1].z = int {};
        int _e74 = points_1.inner[0].inner[0].z;
        out[3] = _e74;
        int _e82 = points_1.inner[1].inner[1].x;
        out[4] = _e82;
        int _e90 = points_1.inner[1].inner[1].z;
        out[5] = _e90;
    }
    {
        original_2.x = 1;
        original_2.y = 2;
        original_2.z = 3;
        original_2.w = 4;
        metal::int4 _e103 = original_2;
        points_2.inner[0].inner[0] = _e103;
        points_2.inner[0].inner[1] = metal::int4 {};
        metal::int4 _e108 = original_2;
        points_2.inner[1].inner[0] = _e108;
        points_2.inner[1].inner[1].x = 5;
        points_2.inner[1].inner[1].y = int {};
        points_2.inner[1].inner[1].z = int {};
        points_2.inner[1].inner[1].w = int {};
        int _e125 = points_2.inner[0].inner[0].w;
        out[6] = _e125;
        int _e133 = points_2.inner[1].inner[1].x;
        out[7] = _e133;
        int _e141 = points_2.inner[1].inner[1].w;
        out[8] = _e141;
    }
    {
        original_3.x = static_cast<uint>(1);
        original_3.y = static_cast<uint>(2);
        metal::uint2 _e152 = original_3;
        points_3.inner[0].inner[0] = _e152;
        points_3.inner[0].inner[1] = metal::uint2 {};
        metal::uint2 _e157 = original_3;
        points_3.inner[1].inner[0] = _e157;
        points_3.inner[1].inner[1].x = static_cast<uint>(5);
        points_3.inner[1].inner[1].y = uint {};
        uint _e171 = points_3.inner[0].inner[0].y;
        out[9] = static_cast<int>(_e171);
        uint _e180 = points_3.inner[1].inner[1].x;
        out[10] = static_cast<int>(_e180);
        uint _e189 = points_3.inner[1].inner[1].y;
        out[11] = static_cast<int>(_e189);
    }
    {
        original_4.x = static_cast<uint>(1);
        original_4.y = static_cast<uint>(2);
        original_4.z = static_cast<uint>(3);
        uint3_ _e204 = original_4;
        points_4.inner[0].inner[0] = _e204;
        points_4.inner[0].inner[1] = uint3_ {};
        uint3_ _e209 = original_4;
        points_4.inner[1].inner[0] = _e209;
        points_4.inner[1].inner[1].x = static_cast<uint>(5);
        points_4.inner[1].inner[1].y = uint {};
        points_4.inner[1].inner[1].z = uint {};
        uint _e225 = points_4.inner[0].inner[0].z;
        out[12] = static_cast<int>(_e225);
        uint _e234 = points_4.inner[1].inner[1].x;
        out[13] = static_cast<int>(_e234);
        uint _e243 = points_4.inner[1].inner[1].z;
        out[14] = static_cast<int>(_e243);
    }
    {
        original_5.x = static_cast<uint>(1);
        original_5.y = static_cast<uint>(2);
        original_5.z = static_cast<uint>(3);
        original_5.w = static_cast<uint>(4);
        metal::uint4 _e261 = original_5;
        points_5.inner[0].inner[0] = _e261;
        points_5.inner[0].inner[1] = metal::uint4 {};
        metal::uint4 _e266 = original_5;
        points_5.inner[1].inner[0] = _e266;
        points_5.inner[1].inner[1].x = static_cast<uint>(5);
        points_5.inner[1].inner[1].y = uint {};
        points_5.inner[1].inner[1].z = uint {};
        points_5.inner[1].inner[1].w = uint {};
        uint _e284 = points_5.inner[0].inner[0].w;
        out[15] = static_cast<int>(_e284);
        uint _e293 = points_5.inner[1].inner[1].x;
        out[16] = static_cast<int>(_e293);
        uint _e302 = points_5.inner[1].inner[1].w;
        out[17] = static_cast<int>(_e302);
    }
    {
        original_6.x = static_cast<float>(1);
        original_6.y = static_cast<float>(2);
        metal::float2 _e314 = original_6;
        points_6.inner[0].inner[0] = _e314;
        points_6.inner[0].inner[1] = metal::float2 {};
        metal::float2 _e319 = original_6;
        points_6.inner[1].inner[0] = _e319;
        points_6.inner[1].inner[1].x = static_cast<float>(5);
        points_6.inner[1].inner[1].y = float {};
        float _e333 = points_6.inner[0].inner[0].y;
        out[18] = naga_f2i32(_e333);
        float _e342 = points_6.inner[1].inner[1].x;
        out[19] = naga_f2i32(_e342);
        float _e351 = points_6.inner[1].inner[1].y;
        out[20] = naga_f2i32(_e351);
    }
    {
        original_7.x = static_cast<float>(1);
        original_7.y = static_cast<float>(2);
        original_7.z = static_cast<float>(3);
        float3_ _e366 = original_7;
        points_7.inner[0].inner[0] = _e366;
        points_7.inner[0].inner[1] = float3_ {};
        float3_ _e371 = original_7;
        points_7.inner[1].inner[0] = _e371;
        points_7.inner[1].inner[1].x = static_cast<float>(5);
        points_7.inner[1].inner[1].y = float {};
        points_7.inner[1].inner[1].z = float {};
        float _e387 = points_7.inner[0].inner[0].z;
        out[21] = naga_f2i32(_e387);
        float _e396 = points_7.inner[1].inner[1].x;
        out[22] = naga_f2i32(_e396);
        float _e405 = points_7.inner[1].inner[1].z;
        out[23] = naga_f2i32(_e405);
    }
    {
        original_8.x = static_cast<float>(1);
        original_8.y = static_cast<float>(2);
        original_8.z = static_cast<float>(3);
        original_8.w = static_cast<float>(4);
        metal::float4 _e423 = original_8;
        points_8.inner[0].inner[0] = _e423;
        points_8.inner[0].inner[1] = metal::float4 {};
        metal::float4 _e428 = original_8;
        points_8.inner[1].inner[0] = _e428;
        points_8.inner[1].inner[1].x = static_cast<float>(5);
        points_8.inner[1].inner[1].y = float {};
        points_8.inner[1].inner[1].z = float {};
        points_8.inner[1].inner[1].w = float {};
        float _e446 = points_8.inner[0].inner[0].w;
        out[24] = naga_f2i32(_e446);
        float _e455 = points_8.inner[1].inner[1].x;
        out[25] = naga_f2i32(_e455);
        float _e464 = points_8.inner[1].inner[1].w;
        out[26] = naga_f2i32(_e464);
    }
    changed.inner[0].inner[0] = 1;
    local = 1u;
    uint2 loop_bound = uint2(4294967295u);
    bool loop_init = true;
    while(true) {
        if (metal::all(loop_bound == uint2(0u))) { break; }
        loop_bound -= uint2(loop_bound.y == 0u, 1u);
        if (!loop_init) {
            uint _e477 = local;
            local = _e477 + 1u;
        }
        loop_init = false;
        uint _e473 = local;
        if (_e473 >= 2u) {
            break;
        }
        changed.inner[0].inner[_e473] = int {};
    }
    changed.inner[0].inner[1] = 9;
    changed.inner[1].inner[0] = 2;
    changed.inner[1].inner[1] = 3;
    local_1 = 0u;
    uint2 loop_bound_1 = uint2(4294967295u);
    bool loop_init_1 = true;
    while(true) {
        if (metal::all(loop_bound_1 == uint2(0u))) { break; }
        loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
        if (!loop_init_1) {
            uint _e507 = local_1;
            local_1 = _e507 + 1u;
        }
        loop_init_1 = false;
        uint _e494 = local_1;
        if (_e494 >= 2u) {
            break;
        }
        local_2 = 0u;
        uint2 loop_bound_2 = uint2(4294967295u);
        bool loop_init_2 = true;
        while(true) {
            if (metal::all(loop_bound_2 == uint2(0u))) { break; }
            loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
            if (!loop_init_2) {
                uint _e504 = local_2;
                local_2 = _e504 + 1u;
            }
            loop_init_2 = false;
            uint _e500 = local_2;
            if (_e500 >= 3u) {
                break;
            }
            nested_empty.inner[_e494].inner[_e500] = int {};
        }
    }
    partial.inner[0].inner[0] = 1;
    local_3 = 1u;
    uint2 loop_bound_3 = uint2(4294967295u);
    bool loop_init_3 = true;
    while(true) {
        if (metal::all(loop_bound_3 == uint2(0u))) { break; }
        loop_bound_3 -= uint2(loop_bound_3.y == 0u, 1u);
        if (!loop_init_3) {
            uint _e521 = local_3;
            local_3 = _e521 + 1u;
        }
        loop_init_3 = false;
        uint _e517 = local_3;
        if (_e517 >= 3u) {
            break;
        }
        partial.inner[0].inner[_e517] = int {};
    }
    local_4 = 1u;
    uint2 loop_bound_4 = uint2(4294967295u);
    bool loop_init_4 = true;
    while(true) {
        if (metal::all(loop_bound_4 == uint2(0u))) { break; }
        loop_bound_4 -= uint2(loop_bound_4.y == 0u, 1u);
        if (!loop_init_4) {
            uint _e540 = local_4;
            local_4 = _e540 + 1u;
        }
        loop_init_4 = false;
        uint _e527 = local_4;
        if (_e527 >= 2u) {
            break;
        }
        local_5 = 0u;
        uint2 loop_bound_5 = uint2(4294967295u);
        bool loop_init_5 = true;
        while(true) {
            if (metal::all(loop_bound_5 == uint2(0u))) { break; }
            loop_bound_5 -= uint2(loop_bound_5.y == 0u, 1u);
            if (!loop_init_5) {
                uint _e537 = local_5;
                local_5 = _e537 + 1u;
            }
            loop_init_5 = false;
            uint _e533 = local_5;
            if (_e533 >= 3u) {
                break;
            }
            partial.inner[_e527].inner[_e533] = int {};
        }
    }
    volume.inner[0].inner[0].inner[0] = 1;
    local_6 = 1u;
    uint2 loop_bound_6 = uint2(4294967295u);
    bool loop_init_6 = true;
    while(true) {
        if (metal::all(loop_bound_6 == uint2(0u))) { break; }
        loop_bound_6 -= uint2(loop_bound_6.y == 0u, 1u);
        if (!loop_init_6) {
            uint _e555 = local_6;
            local_6 = _e555 + 1u;
        }
        loop_init_6 = false;
        uint _e551 = local_6;
        if (_e551 >= 3u) {
            break;
        }
        volume.inner[0].inner[0].inner[_e551] = int {};
    }
    local_7 = 1u;
    uint2 loop_bound_7 = uint2(4294967295u);
    bool loop_init_7 = true;
    while(true) {
        if (metal::all(loop_bound_7 == uint2(0u))) { break; }
        loop_bound_7 -= uint2(loop_bound_7.y == 0u, 1u);
        if (!loop_init_7) {
            uint _e574 = local_7;
            local_7 = _e574 + 1u;
        }
        loop_init_7 = false;
        uint _e561 = local_7;
        if (_e561 >= 2u) {
            break;
        }
        local_8 = 0u;
        uint2 loop_bound_8 = uint2(4294967295u);
        bool loop_init_8 = true;
        while(true) {
            if (metal::all(loop_bound_8 == uint2(0u))) { break; }
            loop_bound_8 -= uint2(loop_bound_8.y == 0u, 1u);
            if (!loop_init_8) {
                uint _e571 = local_8;
                local_8 = _e571 + 1u;
            }
            loop_init_8 = false;
            uint _e567 = local_8;
            if (_e567 >= 3u) {
                break;
            }
            volume.inner[0].inner[_e561].inner[_e567] = int {};
        }
    }
    local_9 = 1u;
    uint2 loop_bound_9 = uint2(4294967295u);
    bool loop_init_9 = true;
    while(true) {
        if (metal::all(loop_bound_9 == uint2(0u))) { break; }
        loop_bound_9 -= uint2(loop_bound_9.y == 0u, 1u);
        if (!loop_init_9) {
            uint _e602 = local_9;
            local_9 = _e602 + 1u;
        }
        loop_init_9 = false;
        uint _e580 = local_9;
        if (_e580 >= 2u) {
            break;
        }
        local_10 = 0u;
        uint2 loop_bound_10 = uint2(4294967295u);
        bool loop_init_10 = true;
        while(true) {
            if (metal::all(loop_bound_10 == uint2(0u))) { break; }
            loop_bound_10 -= uint2(loop_bound_10.y == 0u, 1u);
            if (!loop_init_10) {
                uint _e599 = local_10;
                local_10 = _e599 + 1u;
            }
            loop_init_10 = false;
            uint _e586 = local_10;
            if (_e586 >= 2u) {
                break;
            }
            local_11 = 0u;
            uint2 loop_bound_11 = uint2(4294967295u);
            bool loop_init_11 = true;
            while(true) {
                if (metal::all(loop_bound_11 == uint2(0u))) { break; }
                loop_bound_11 -= uint2(loop_bound_11.y == 0u, 1u);
                if (!loop_init_11) {
                    uint _e596 = local_11;
                    local_11 = _e596 + 1u;
                }
                loop_init_11 = false;
                uint _e592 = local_11;
                if (_e592 >= 3u) {
                    break;
                }
                volume.inner[_e580].inner[_e586].inner[_e592] = int {};
            }
        }
    }
    int _e615 = changed.inner[0].inner[1];
    shared_values.inner[1].inner[2] = _e615;
    int _e624 = changed.inner[1].inner[0];
    int _e629 = changed.inner[1].inner[1];
    shared_points.inner[1].inner[0] = float3_ {static_cast<float>(_e624), static_cast<float>(_e629), static_cast<float>(4)};
    metal::threadgroup_barrier(metal::mem_flags::mem_device);
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    int _e645 = shared_values.inner[1].inner[2];
    int _e646 = nested_empty.inner[1].inner[2];
    int _e652 = partial.inner[1].inner[2];
    int _e660 = volume.inner[1].inner[1].inner[2];
    out[27] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e645) + as_type<uint>(_e646))) + as_type<uint>(_e652))) + as_type<uint>(_e660));
    float _e669 = shared_points.inner[1].inner[0].x;
    out[28] = naga_f2i32(_e669);
    float _e678 = shared_points.inner[1].inner[0].y;
    out[29] = naga_f2i32(_e678);
}
