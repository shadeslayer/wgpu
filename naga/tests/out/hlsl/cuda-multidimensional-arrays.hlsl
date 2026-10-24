struct int3_ {
    int x;
    int y;
    int z;
};

struct uint3_ {
    uint x;
    uint y;
    uint z;
};

struct float3_ {
    float x;
    float y;
    float z;
};

RWByteAddressBuffer out_ : register(u0);
groupshared int shared_values[2][3];
groupshared float3_ shared_points[2][2];

float3_ Constructfloat3_(float arg0, float arg1, float arg2) {
    float3_ ret = (float3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

int2 ZeroValueint2() {
    return (int2)0;
}

int ZeroValueint() {
    return (int)0;
}

int3_ ZeroValueint3_() {
    return (int3_)0;
}

int4 ZeroValueint4() {
    return (int4)0;
}

uint2 ZeroValueuint2() {
    return (uint2)0;
}

uint ZeroValueuint() {
    return (uint)0;
}

uint3_ ZeroValueuint3_() {
    return (uint3_)0;
}

uint4 ZeroValueuint4() {
    return (uint4)0;
}

float2 ZeroValuefloat2() {
    return (float2)0;
}

float ZeroValuefloat() {
    return (float)0;
}

float3_ ZeroValuefloat3_() {
    return (float3_)0;
}

float4 ZeroValuefloat4() {
    return (float4)0;
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

[numthreads(1, 1, 1)]
void multidimensional_arrays(uint local_invocation_index : SV_GroupIndex)
{
    if (local_invocation_index == 0) {
        shared_values = (int[2][3])0;
        shared_points = (float3_[2][2])0;
    }
    GroupMemoryBarrierWithGroupSync();
    int2 original = (int2)0;
    int2 points[2][2] = (int2[2][2])0;
    int3_ original_1 = (int3_)0;
    int3_ points_1[2][2] = (int3_[2][2])0;
    int4 original_2 = (int4)0;
    int4 points_2[2][2] = (int4[2][2])0;
    uint2 original_3 = (uint2)0;
    uint2 points_3[2][2] = (uint2[2][2])0;
    uint3_ original_4 = (uint3_)0;
    uint3_ points_4[2][2] = (uint3_[2][2])0;
    uint4 original_5 = (uint4)0;
    uint4 points_5[2][2] = (uint4[2][2])0;
    float2 original_6 = (float2)0;
    float2 points_6[2][2] = (float2[2][2])0;
    float3_ original_7 = (float3_)0;
    float3_ points_7[2][2] = (float3_[2][2])0;
    float4 original_8 = (float4)0;
    float4 points_8[2][2] = (float4[2][2])0;
    int changed[2][2] = (int[2][2])0;
    uint local = (uint)0;
    int nested_empty[2][3] = (int[2][3])0;
    uint local_1 = (uint)0;
    uint local_2 = (uint)0;
    int partial[2][3] = (int[2][3])0;
    uint local_3 = (uint)0;
    uint local_4 = (uint)0;
    uint local_5 = (uint)0;
    int volume[2][2][3] = (int[2][2][3])0;
    uint local_6 = (uint)0;
    uint local_7 = (uint)0;
    uint local_8 = (uint)0;
    uint local_9 = (uint)0;
    uint local_10 = (uint)0;
    uint local_11 = (uint)0;

    {
        original.x = int(1);
        original.y = int(2);
        int2 _e9 = original;
        points[0][0] = _e9;
        points[0][1] = ZeroValueint2();
        int2 _e14 = original;
        points[1][0] = _e14;
        points[1][1].x = int(5);
        points[1][1].y = ZeroValueint();
        int _e27 = points[int(0)][int(0)].y;
        out_.Store(int(0)*4, asuint(_e27));
        int _e35 = points[int(1)][int(1)].x;
        out_.Store(int(1)*4, asuint(_e35));
        int _e43 = points[int(1)][int(1)].y;
        out_.Store(int(2)*4, asuint(_e43));
    }
    {
        original_1.x = int(1);
        original_1.y = int(2);
        original_1.z = int(3);
        int3_ _e54 = original_1;
        points_1[0][0] = _e54;
        points_1[0][1] = ZeroValueint3_();
        int3_ _e59 = original_1;
        points_1[1][0] = _e59;
        points_1[1][1].x = int(5);
        points_1[1][1].y = ZeroValueint();
        points_1[1][1].z = ZeroValueint();
        int _e74 = points_1[int(0)][int(0)].z;
        out_.Store(int(3)*4, asuint(_e74));
        int _e82 = points_1[int(1)][int(1)].x;
        out_.Store(int(4)*4, asuint(_e82));
        int _e90 = points_1[int(1)][int(1)].z;
        out_.Store(int(5)*4, asuint(_e90));
    }
    {
        original_2.x = int(1);
        original_2.y = int(2);
        original_2.z = int(3);
        original_2.w = int(4);
        int4 _e103 = original_2;
        points_2[0][0] = _e103;
        points_2[0][1] = ZeroValueint4();
        int4 _e108 = original_2;
        points_2[1][0] = _e108;
        points_2[1][1].x = int(5);
        points_2[1][1].y = ZeroValueint();
        points_2[1][1].z = ZeroValueint();
        points_2[1][1].w = ZeroValueint();
        int _e125 = points_2[int(0)][int(0)].w;
        out_.Store(int(6)*4, asuint(_e125));
        int _e133 = points_2[int(1)][int(1)].x;
        out_.Store(int(7)*4, asuint(_e133));
        int _e141 = points_2[int(1)][int(1)].w;
        out_.Store(int(8)*4, asuint(_e141));
    }
    {
        original_3.x = uint(int(1));
        original_3.y = uint(int(2));
        uint2 _e152 = original_3;
        points_3[0][0] = _e152;
        points_3[0][1] = ZeroValueuint2();
        uint2 _e157 = original_3;
        points_3[1][0] = _e157;
        points_3[1][1].x = uint(int(5));
        points_3[1][1].y = ZeroValueuint();
        uint _e171 = points_3[int(0)][int(0)].y;
        out_.Store(int(9)*4, asuint(int(_e171)));
        uint _e180 = points_3[int(1)][int(1)].x;
        out_.Store(int(10)*4, asuint(int(_e180)));
        uint _e189 = points_3[int(1)][int(1)].y;
        out_.Store(int(11)*4, asuint(int(_e189)));
    }
    {
        original_4.x = uint(int(1));
        original_4.y = uint(int(2));
        original_4.z = uint(int(3));
        uint3_ _e204 = original_4;
        points_4[0][0] = _e204;
        points_4[0][1] = ZeroValueuint3_();
        uint3_ _e209 = original_4;
        points_4[1][0] = _e209;
        points_4[1][1].x = uint(int(5));
        points_4[1][1].y = ZeroValueuint();
        points_4[1][1].z = ZeroValueuint();
        uint _e225 = points_4[int(0)][int(0)].z;
        out_.Store(int(12)*4, asuint(int(_e225)));
        uint _e234 = points_4[int(1)][int(1)].x;
        out_.Store(int(13)*4, asuint(int(_e234)));
        uint _e243 = points_4[int(1)][int(1)].z;
        out_.Store(int(14)*4, asuint(int(_e243)));
    }
    {
        original_5.x = uint(int(1));
        original_5.y = uint(int(2));
        original_5.z = uint(int(3));
        original_5.w = uint(int(4));
        uint4 _e261 = original_5;
        points_5[0][0] = _e261;
        points_5[0][1] = ZeroValueuint4();
        uint4 _e266 = original_5;
        points_5[1][0] = _e266;
        points_5[1][1].x = uint(int(5));
        points_5[1][1].y = ZeroValueuint();
        points_5[1][1].z = ZeroValueuint();
        points_5[1][1].w = ZeroValueuint();
        uint _e284 = points_5[int(0)][int(0)].w;
        out_.Store(int(15)*4, asuint(int(_e284)));
        uint _e293 = points_5[int(1)][int(1)].x;
        out_.Store(int(16)*4, asuint(int(_e293)));
        uint _e302 = points_5[int(1)][int(1)].w;
        out_.Store(int(17)*4, asuint(int(_e302)));
    }
    {
        original_6.x = float(int(1));
        original_6.y = float(int(2));
        float2 _e314 = original_6;
        points_6[0][0] = _e314;
        points_6[0][1] = ZeroValuefloat2();
        float2 _e319 = original_6;
        points_6[1][0] = _e319;
        points_6[1][1].x = float(int(5));
        points_6[1][1].y = ZeroValuefloat();
        float _e333 = points_6[int(0)][int(0)].y;
        out_.Store(int(18)*4, asuint(naga_f2i32(_e333)));
        float _e342 = points_6[int(1)][int(1)].x;
        out_.Store(int(19)*4, asuint(naga_f2i32(_e342)));
        float _e351 = points_6[int(1)][int(1)].y;
        out_.Store(int(20)*4, asuint(naga_f2i32(_e351)));
    }
    {
        original_7.x = float(int(1));
        original_7.y = float(int(2));
        original_7.z = float(int(3));
        float3_ _e366 = original_7;
        points_7[0][0] = _e366;
        points_7[0][1] = ZeroValuefloat3_();
        float3_ _e371 = original_7;
        points_7[1][0] = _e371;
        points_7[1][1].x = float(int(5));
        points_7[1][1].y = ZeroValuefloat();
        points_7[1][1].z = ZeroValuefloat();
        float _e387 = points_7[int(0)][int(0)].z;
        out_.Store(int(21)*4, asuint(naga_f2i32(_e387)));
        float _e396 = points_7[int(1)][int(1)].x;
        out_.Store(int(22)*4, asuint(naga_f2i32(_e396)));
        float _e405 = points_7[int(1)][int(1)].z;
        out_.Store(int(23)*4, asuint(naga_f2i32(_e405)));
    }
    {
        original_8.x = float(int(1));
        original_8.y = float(int(2));
        original_8.z = float(int(3));
        original_8.w = float(int(4));
        float4 _e423 = original_8;
        points_8[0][0] = _e423;
        points_8[0][1] = ZeroValuefloat4();
        float4 _e428 = original_8;
        points_8[1][0] = _e428;
        points_8[1][1].x = float(int(5));
        points_8[1][1].y = ZeroValuefloat();
        points_8[1][1].z = ZeroValuefloat();
        points_8[1][1].w = ZeroValuefloat();
        float _e446 = points_8[int(0)][int(0)].w;
        out_.Store(int(24)*4, asuint(naga_f2i32(_e446)));
        float _e455 = points_8[int(1)][int(1)].x;
        out_.Store(int(25)*4, asuint(naga_f2i32(_e455)));
        float _e464 = points_8[int(1)][int(1)].w;
        out_.Store(int(26)*4, asuint(naga_f2i32(_e464)));
    }
    changed[0][0] = int(1);
    local = 1u;
    uint2 loop_bound = uint2(4294967295u, 4294967295u);
    bool loop_init = true;
    while(true) {
        if (all(loop_bound == uint2(0u, 0u))) { break; }
        loop_bound -= uint2(loop_bound.y == 0u, 1u);
        if (!loop_init) {
            uint _e477 = local;
            local = (_e477 + 1u);
        }
        loop_init = false;
        uint _e473 = local;
        if ((_e473 >= 2u)) {
            break;
        }
        changed[0][min(uint(_e473), 1u)] = ZeroValueint();
    }
    changed[int(0)][int(1)] = int(9);
    changed[1][0] = int(2);
    changed[1][1] = int(3);
    local_1 = 0u;
    uint2 loop_bound_1 = uint2(4294967295u, 4294967295u);
    bool loop_init_1 = true;
    while(true) {
        if (all(loop_bound_1 == uint2(0u, 0u))) { break; }
        loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
        if (!loop_init_1) {
            uint _e507 = local_1;
            local_1 = (_e507 + 1u);
        }
        loop_init_1 = false;
        uint _e494 = local_1;
        if ((_e494 >= 2u)) {
            break;
        }
        local_2 = 0u;
        uint2 loop_bound_2 = uint2(4294967295u, 4294967295u);
        bool loop_init_2 = true;
        while(true) {
            if (all(loop_bound_2 == uint2(0u, 0u))) { break; }
            loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
            if (!loop_init_2) {
                uint _e504 = local_2;
                local_2 = (_e504 + 1u);
            }
            loop_init_2 = false;
            uint _e500 = local_2;
            if ((_e500 >= 3u)) {
                break;
            }
            nested_empty[min(uint(_e494), 1u)][min(uint(_e500), 2u)] = ZeroValueint();
        }
    }
    partial[0][0] = int(1);
    local_3 = 1u;
    uint2 loop_bound_3 = uint2(4294967295u, 4294967295u);
    bool loop_init_3 = true;
    while(true) {
        if (all(loop_bound_3 == uint2(0u, 0u))) { break; }
        loop_bound_3 -= uint2(loop_bound_3.y == 0u, 1u);
        if (!loop_init_3) {
            uint _e521 = local_3;
            local_3 = (_e521 + 1u);
        }
        loop_init_3 = false;
        uint _e517 = local_3;
        if ((_e517 >= 3u)) {
            break;
        }
        partial[0][min(uint(_e517), 2u)] = ZeroValueint();
    }
    local_4 = 1u;
    uint2 loop_bound_4 = uint2(4294967295u, 4294967295u);
    bool loop_init_4 = true;
    while(true) {
        if (all(loop_bound_4 == uint2(0u, 0u))) { break; }
        loop_bound_4 -= uint2(loop_bound_4.y == 0u, 1u);
        if (!loop_init_4) {
            uint _e540 = local_4;
            local_4 = (_e540 + 1u);
        }
        loop_init_4 = false;
        uint _e527 = local_4;
        if ((_e527 >= 2u)) {
            break;
        }
        local_5 = 0u;
        uint2 loop_bound_5 = uint2(4294967295u, 4294967295u);
        bool loop_init_5 = true;
        while(true) {
            if (all(loop_bound_5 == uint2(0u, 0u))) { break; }
            loop_bound_5 -= uint2(loop_bound_5.y == 0u, 1u);
            if (!loop_init_5) {
                uint _e537 = local_5;
                local_5 = (_e537 + 1u);
            }
            loop_init_5 = false;
            uint _e533 = local_5;
            if ((_e533 >= 3u)) {
                break;
            }
            partial[min(uint(_e527), 1u)][min(uint(_e533), 2u)] = ZeroValueint();
        }
    }
    volume[0][0][0] = int(1);
    local_6 = 1u;
    uint2 loop_bound_6 = uint2(4294967295u, 4294967295u);
    bool loop_init_6 = true;
    while(true) {
        if (all(loop_bound_6 == uint2(0u, 0u))) { break; }
        loop_bound_6 -= uint2(loop_bound_6.y == 0u, 1u);
        if (!loop_init_6) {
            uint _e555 = local_6;
            local_6 = (_e555 + 1u);
        }
        loop_init_6 = false;
        uint _e551 = local_6;
        if ((_e551 >= 3u)) {
            break;
        }
        volume[0][0][min(uint(_e551), 2u)] = ZeroValueint();
    }
    local_7 = 1u;
    uint2 loop_bound_7 = uint2(4294967295u, 4294967295u);
    bool loop_init_7 = true;
    while(true) {
        if (all(loop_bound_7 == uint2(0u, 0u))) { break; }
        loop_bound_7 -= uint2(loop_bound_7.y == 0u, 1u);
        if (!loop_init_7) {
            uint _e574 = local_7;
            local_7 = (_e574 + 1u);
        }
        loop_init_7 = false;
        uint _e561 = local_7;
        if ((_e561 >= 2u)) {
            break;
        }
        local_8 = 0u;
        uint2 loop_bound_8 = uint2(4294967295u, 4294967295u);
        bool loop_init_8 = true;
        while(true) {
            if (all(loop_bound_8 == uint2(0u, 0u))) { break; }
            loop_bound_8 -= uint2(loop_bound_8.y == 0u, 1u);
            if (!loop_init_8) {
                uint _e571 = local_8;
                local_8 = (_e571 + 1u);
            }
            loop_init_8 = false;
            uint _e567 = local_8;
            if ((_e567 >= 3u)) {
                break;
            }
            volume[0][min(uint(_e561), 1u)][min(uint(_e567), 2u)] = ZeroValueint();
        }
    }
    local_9 = 1u;
    uint2 loop_bound_9 = uint2(4294967295u, 4294967295u);
    bool loop_init_9 = true;
    while(true) {
        if (all(loop_bound_9 == uint2(0u, 0u))) { break; }
        loop_bound_9 -= uint2(loop_bound_9.y == 0u, 1u);
        if (!loop_init_9) {
            uint _e602 = local_9;
            local_9 = (_e602 + 1u);
        }
        loop_init_9 = false;
        uint _e580 = local_9;
        if ((_e580 >= 2u)) {
            break;
        }
        local_10 = 0u;
        uint2 loop_bound_10 = uint2(4294967295u, 4294967295u);
        bool loop_init_10 = true;
        while(true) {
            if (all(loop_bound_10 == uint2(0u, 0u))) { break; }
            loop_bound_10 -= uint2(loop_bound_10.y == 0u, 1u);
            if (!loop_init_10) {
                uint _e599 = local_10;
                local_10 = (_e599 + 1u);
            }
            loop_init_10 = false;
            uint _e586 = local_10;
            if ((_e586 >= 2u)) {
                break;
            }
            local_11 = 0u;
            uint2 loop_bound_11 = uint2(4294967295u, 4294967295u);
            bool loop_init_11 = true;
            while(true) {
                if (all(loop_bound_11 == uint2(0u, 0u))) { break; }
                loop_bound_11 -= uint2(loop_bound_11.y == 0u, 1u);
                if (!loop_init_11) {
                    uint _e596 = local_11;
                    local_11 = (_e596 + 1u);
                }
                loop_init_11 = false;
                uint _e592 = local_11;
                if ((_e592 >= 3u)) {
                    break;
                }
                volume[min(uint(_e580), 1u)][min(uint(_e586), 1u)][min(uint(_e592), 2u)] = ZeroValueint();
            }
        }
    }
    int _e615 = changed[int(0)][int(1)];
    shared_values[int(1)][int(2)] = _e615;
    int _e624 = changed[int(1)][int(0)];
    int _e629 = changed[int(1)][int(1)];
    shared_points[int(1)][int(0)] = Constructfloat3_(float(_e624), float(_e629), float(int(4)));
    DeviceMemoryBarrierWithGroupSync();
    GroupMemoryBarrierWithGroupSync();
    int _e645 = shared_values[int(1)][int(2)];
    int _e646 = nested_empty[int(1)][int(2)];
    int _e652 = partial[int(1)][int(2)];
    int _e660 = volume[int(1)][int(1)][int(2)];
    out_.Store(int(27)*4, asuint(asint(asuint(asint(asuint(asint(asuint(_e645) + asuint(_e646))) + asuint(_e652))) + asuint(_e660))));
    float _e669 = shared_points[int(1)][int(0)].x;
    out_.Store(int(28)*4, asuint(naga_f2i32(_e669)));
    float _e678 = shared_points[int(1)][int(0)].y;
    out_.Store(int(29)*4, asuint(naga_f2i32(_e678)));
}
