#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

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
layout(std430) buffer type_1_block_0Compute { int _group_0_binding_0_cs[]; };

shared int shared_values[2][3];

shared float3_ shared_points[2][2];


void main() {
    if (gl_LocalInvocationID == uvec3(0u)) {
        shared_values = int[2][3](int[3](0, 0, 0), int[3](0, 0, 0));
        shared_points = float3_[2][2](float3_[2](float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0)), float3_[2](float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0)));
    }
    memoryBarrierShared();
    barrier();
    ivec2 original = ivec2(0);
    ivec2 points[2][2] = ivec2[2][2](ivec2[2](ivec2(0), ivec2(0)), ivec2[2](ivec2(0), ivec2(0)));
    int3_ original_1 = int3_(0, 0, 0);
    int3_ points_1[2][2] = int3_[2][2](int3_[2](int3_(0, 0, 0), int3_(0, 0, 0)), int3_[2](int3_(0, 0, 0), int3_(0, 0, 0)));
    ivec4 original_2 = ivec4(0);
    ivec4 points_2[2][2] = ivec4[2][2](ivec4[2](ivec4(0), ivec4(0)), ivec4[2](ivec4(0), ivec4(0)));
    uvec2 original_3 = uvec2(0u);
    uvec2 points_3[2][2] = uvec2[2][2](uvec2[2](uvec2(0u), uvec2(0u)), uvec2[2](uvec2(0u), uvec2(0u)));
    uint3_ original_4 = uint3_(0u, 0u, 0u);
    uint3_ points_4[2][2] = uint3_[2][2](uint3_[2](uint3_(0u, 0u, 0u), uint3_(0u, 0u, 0u)), uint3_[2](uint3_(0u, 0u, 0u), uint3_(0u, 0u, 0u)));
    uvec4 original_5 = uvec4(0u);
    uvec4 points_5[2][2] = uvec4[2][2](uvec4[2](uvec4(0u), uvec4(0u)), uvec4[2](uvec4(0u), uvec4(0u)));
    vec2 original_6 = vec2(0.0);
    vec2 points_6[2][2] = vec2[2][2](vec2[2](vec2(0.0), vec2(0.0)), vec2[2](vec2(0.0), vec2(0.0)));
    float3_ original_7 = float3_(0.0, 0.0, 0.0);
    float3_ points_7[2][2] = float3_[2][2](float3_[2](float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0)), float3_[2](float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0)));
    vec4 original_8 = vec4(0.0);
    vec4 points_8[2][2] = vec4[2][2](vec4[2](vec4(0.0), vec4(0.0)), vec4[2](vec4(0.0), vec4(0.0)));
    int changed[2][2] = int[2][2](int[2](0, 0), int[2](0, 0));
    uint local = 0u;
    int nested_empty[2][3] = int[2][3](int[3](0, 0, 0), int[3](0, 0, 0));
    uint local_1 = 0u;
    uint local_2 = 0u;
    int partial[2][3] = int[2][3](int[3](0, 0, 0), int[3](0, 0, 0));
    uint local_3 = 0u;
    uint local_4 = 0u;
    uint local_5 = 0u;
    int volume[2][2][3] = int[2][2][3](int[2][3](int[3](0, 0, 0), int[3](0, 0, 0)), int[2][3](int[3](0, 0, 0), int[3](0, 0, 0)));
    uint local_6 = 0u;
    uint local_7 = 0u;
    uint local_8 = 0u;
    uint local_9 = 0u;
    uint local_10 = 0u;
    uint local_11 = 0u;
    {
        original.x = 1;
        original.y = 2;
        ivec2 _e9 = original;
        points[0][0] = _e9;
        points[0][1] = ivec2(0);
        ivec2 _e14 = original;
        points[1][0] = _e14;
        points[1][1].x = 5;
        points[1][1].y = 0;
        int _e27 = points[0][0].y;
        _group_0_binding_0_cs[0] = _e27;
        int _e35 = points[1][1].x;
        _group_0_binding_0_cs[1] = _e35;
        int _e43 = points[1][1].y;
        _group_0_binding_0_cs[2] = _e43;
    }
    {
        original_1.x = 1;
        original_1.y = 2;
        original_1.z = 3;
        int3_ _e54 = original_1;
        points_1[0][0] = _e54;
        points_1[0][1] = int3_(0, 0, 0);
        int3_ _e59 = original_1;
        points_1[1][0] = _e59;
        points_1[1][1].x = 5;
        points_1[1][1].y = 0;
        points_1[1][1].z = 0;
        int _e74 = points_1[0][0].z;
        _group_0_binding_0_cs[3] = _e74;
        int _e82 = points_1[1][1].x;
        _group_0_binding_0_cs[4] = _e82;
        int _e90 = points_1[1][1].z;
        _group_0_binding_0_cs[5] = _e90;
    }
    {
        original_2.x = 1;
        original_2.y = 2;
        original_2.z = 3;
        original_2.w = 4;
        ivec4 _e103 = original_2;
        points_2[0][0] = _e103;
        points_2[0][1] = ivec4(0);
        ivec4 _e108 = original_2;
        points_2[1][0] = _e108;
        points_2[1][1].x = 5;
        points_2[1][1].y = 0;
        points_2[1][1].z = 0;
        points_2[1][1].w = 0;
        int _e125 = points_2[0][0].w;
        _group_0_binding_0_cs[6] = _e125;
        int _e133 = points_2[1][1].x;
        _group_0_binding_0_cs[7] = _e133;
        int _e141 = points_2[1][1].w;
        _group_0_binding_0_cs[8] = _e141;
    }
    {
        original_3.x = uint(1);
        original_3.y = uint(2);
        uvec2 _e152 = original_3;
        points_3[0][0] = _e152;
        points_3[0][1] = uvec2(0u);
        uvec2 _e157 = original_3;
        points_3[1][0] = _e157;
        points_3[1][1].x = uint(5);
        points_3[1][1].y = 0u;
        uint _e171 = points_3[0][0].y;
        _group_0_binding_0_cs[9] = int(_e171);
        uint _e180 = points_3[1][1].x;
        _group_0_binding_0_cs[10] = int(_e180);
        uint _e189 = points_3[1][1].y;
        _group_0_binding_0_cs[11] = int(_e189);
    }
    {
        original_4.x = uint(1);
        original_4.y = uint(2);
        original_4.z = uint(3);
        uint3_ _e204 = original_4;
        points_4[0][0] = _e204;
        points_4[0][1] = uint3_(0u, 0u, 0u);
        uint3_ _e209 = original_4;
        points_4[1][0] = _e209;
        points_4[1][1].x = uint(5);
        points_4[1][1].y = 0u;
        points_4[1][1].z = 0u;
        uint _e225 = points_4[0][0].z;
        _group_0_binding_0_cs[12] = int(_e225);
        uint _e234 = points_4[1][1].x;
        _group_0_binding_0_cs[13] = int(_e234);
        uint _e243 = points_4[1][1].z;
        _group_0_binding_0_cs[14] = int(_e243);
    }
    {
        original_5.x = uint(1);
        original_5.y = uint(2);
        original_5.z = uint(3);
        original_5.w = uint(4);
        uvec4 _e261 = original_5;
        points_5[0][0] = _e261;
        points_5[0][1] = uvec4(0u);
        uvec4 _e266 = original_5;
        points_5[1][0] = _e266;
        points_5[1][1].x = uint(5);
        points_5[1][1].y = 0u;
        points_5[1][1].z = 0u;
        points_5[1][1].w = 0u;
        uint _e284 = points_5[0][0].w;
        _group_0_binding_0_cs[15] = int(_e284);
        uint _e293 = points_5[1][1].x;
        _group_0_binding_0_cs[16] = int(_e293);
        uint _e302 = points_5[1][1].w;
        _group_0_binding_0_cs[17] = int(_e302);
    }
    {
        original_6.x = float(1);
        original_6.y = float(2);
        vec2 _e314 = original_6;
        points_6[0][0] = _e314;
        points_6[0][1] = vec2(0.0);
        vec2 _e319 = original_6;
        points_6[1][0] = _e319;
        points_6[1][1].x = float(5);
        points_6[1][1].y = 0.0;
        float _e333 = points_6[0][0].y;
        _group_0_binding_0_cs[18] = int(_e333);
        float _e342 = points_6[1][1].x;
        _group_0_binding_0_cs[19] = int(_e342);
        float _e351 = points_6[1][1].y;
        _group_0_binding_0_cs[20] = int(_e351);
    }
    {
        original_7.x = float(1);
        original_7.y = float(2);
        original_7.z = float(3);
        float3_ _e366 = original_7;
        points_7[0][0] = _e366;
        points_7[0][1] = float3_(0.0, 0.0, 0.0);
        float3_ _e371 = original_7;
        points_7[1][0] = _e371;
        points_7[1][1].x = float(5);
        points_7[1][1].y = 0.0;
        points_7[1][1].z = 0.0;
        float _e387 = points_7[0][0].z;
        _group_0_binding_0_cs[21] = int(_e387);
        float _e396 = points_7[1][1].x;
        _group_0_binding_0_cs[22] = int(_e396);
        float _e405 = points_7[1][1].z;
        _group_0_binding_0_cs[23] = int(_e405);
    }
    {
        original_8.x = float(1);
        original_8.y = float(2);
        original_8.z = float(3);
        original_8.w = float(4);
        vec4 _e423 = original_8;
        points_8[0][0] = _e423;
        points_8[0][1] = vec4(0.0);
        vec4 _e428 = original_8;
        points_8[1][0] = _e428;
        points_8[1][1].x = float(5);
        points_8[1][1].y = 0.0;
        points_8[1][1].z = 0.0;
        points_8[1][1].w = 0.0;
        float _e446 = points_8[0][0].w;
        _group_0_binding_0_cs[24] = int(_e446);
        float _e455 = points_8[1][1].x;
        _group_0_binding_0_cs[25] = int(_e455);
        float _e464 = points_8[1][1].w;
        _group_0_binding_0_cs[26] = int(_e464);
    }
    changed[0][0] = 1;
    local = 1u;
    bool loop_init = true;
    while(true) {
        if (!loop_init) {
            uint _e477 = local;
            local = (_e477 + 1u);
        }
        loop_init = false;
        uint _e473 = local;
        if ((_e473 >= 2u)) {
            break;
        }
        changed[0][_e473] = 0;
    }
    changed[0][1] = 9;
    changed[1][0] = 2;
    changed[1][1] = 3;
    local_1 = 0u;
    bool loop_init_1 = true;
    while(true) {
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
        bool loop_init_2 = true;
        while(true) {
            if (!loop_init_2) {
                uint _e504 = local_2;
                local_2 = (_e504 + 1u);
            }
            loop_init_2 = false;
            uint _e500 = local_2;
            if ((_e500 >= 3u)) {
                break;
            }
            nested_empty[_e494][_e500] = 0;
        }
    }
    partial[0][0] = 1;
    local_3 = 1u;
    bool loop_init_3 = true;
    while(true) {
        if (!loop_init_3) {
            uint _e521 = local_3;
            local_3 = (_e521 + 1u);
        }
        loop_init_3 = false;
        uint _e517 = local_3;
        if ((_e517 >= 3u)) {
            break;
        }
        partial[0][_e517] = 0;
    }
    local_4 = 1u;
    bool loop_init_4 = true;
    while(true) {
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
        bool loop_init_5 = true;
        while(true) {
            if (!loop_init_5) {
                uint _e537 = local_5;
                local_5 = (_e537 + 1u);
            }
            loop_init_5 = false;
            uint _e533 = local_5;
            if ((_e533 >= 3u)) {
                break;
            }
            partial[_e527][_e533] = 0;
        }
    }
    volume[0][0][0] = 1;
    local_6 = 1u;
    bool loop_init_6 = true;
    while(true) {
        if (!loop_init_6) {
            uint _e555 = local_6;
            local_6 = (_e555 + 1u);
        }
        loop_init_6 = false;
        uint _e551 = local_6;
        if ((_e551 >= 3u)) {
            break;
        }
        volume[0][0][_e551] = 0;
    }
    local_7 = 1u;
    bool loop_init_7 = true;
    while(true) {
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
        bool loop_init_8 = true;
        while(true) {
            if (!loop_init_8) {
                uint _e571 = local_8;
                local_8 = (_e571 + 1u);
            }
            loop_init_8 = false;
            uint _e567 = local_8;
            if ((_e567 >= 3u)) {
                break;
            }
            volume[0][_e561][_e567] = 0;
        }
    }
    local_9 = 1u;
    bool loop_init_9 = true;
    while(true) {
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
        bool loop_init_10 = true;
        while(true) {
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
            bool loop_init_11 = true;
            while(true) {
                if (!loop_init_11) {
                    uint _e596 = local_11;
                    local_11 = (_e596 + 1u);
                }
                loop_init_11 = false;
                uint _e592 = local_11;
                if ((_e592 >= 3u)) {
                    break;
                }
                volume[_e580][_e586][_e592] = 0;
            }
        }
    }
    int _e615 = changed[0][1];
    shared_values[1][2] = _e615;
    int _e624 = changed[1][0];
    int _e629 = changed[1][1];
    shared_points[1][0] = float3_(float(_e624), float(_e629), float(4));
    memoryBarrierBuffer();
    memoryBarrierShared();
    barrier();
    int _e645 = shared_values[1][2];
    int _e646 = nested_empty[1][2];
    int _e652 = partial[1][2];
    int _e660 = volume[1][1][2];
    _group_0_binding_0_cs[27] = (((_e645 + _e646) + _e652) + _e660);
    float _e669 = shared_points[1][0].x;
    _group_0_binding_0_cs[28] = int(_e669);
    float _e678 = shared_points[1][0].y;
    _group_0_binding_0_cs[29] = int(_e678);
}

