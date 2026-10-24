struct int3_ {
    x: i32,
    y: i32,
    z: i32,
}

struct uint3_ {
    x: u32,
    y: u32,
    z: u32,
}

struct float3_ {
    x: f32,
    y: f32,
    z: f32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
var<workgroup> shared_values: array<array<i32, 3>, 2>;
var<workgroup> shared_points: array<array<float3_, 2>, 2>;

@compute @workgroup_size(1, 1, 1)
fn multidimensional_arrays() {
    var original: vec2<i32>;
    var points: array<array<vec2<i32>, 2>, 2>;
    var original_1: int3_;
    var points_1: array<array<int3_, 2>, 2>;
    var original_2: vec4<i32>;
    var points_2: array<array<vec4<i32>, 2>, 2>;
    var original_3: vec2<u32>;
    var points_3: array<array<vec2<u32>, 2>, 2>;
    var original_4: uint3_;
    var points_4: array<array<uint3_, 2>, 2>;
    var original_5: vec4<u32>;
    var points_5: array<array<vec4<u32>, 2>, 2>;
    var original_6: vec2<f32>;
    var points_6: array<array<vec2<f32>, 2>, 2>;
    var original_7: float3_;
    var points_7: array<array<float3_, 2>, 2>;
    var original_8: vec4<f32>;
    var points_8: array<array<vec4<f32>, 2>, 2>;
    var changed: array<array<i32, 2>, 2>;
    var local: u32;
    var nested_empty: array<array<i32, 3>, 2>;
    var local_1: u32;
    var local_2: u32;
    var partial: array<array<i32, 3>, 2>;
    var local_3: u32;
    var local_4: u32;
    var local_5: u32;
    var volume: array<array<array<i32, 3>, 2>, 2>;
    var local_6: u32;
    var local_7: u32;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var local_11: u32;

    {
        original.x = 1i;
        original.y = 2i;
        let _e9 = original;
        points[0][0] = _e9;
        points[0][1] = vec2<i32>();
        let _e14 = original;
        points[1][0] = _e14;
        points[1][1].x = 5i;
        points[1][1].y = i32();
        let _e27 = points[0i][0i].y;
        out[0i] = _e27;
        let _e35 = points[1i][1i].x;
        out[1i] = _e35;
        let _e43 = points[1i][1i].y;
        out[2i] = _e43;
    }
    {
        original_1.x = 1i;
        original_1.y = 2i;
        original_1.z = 3i;
        let _e54 = original_1;
        points_1[0][0] = _e54;
        points_1[0][1] = int3_();
        let _e59 = original_1;
        points_1[1][0] = _e59;
        points_1[1][1].x = 5i;
        points_1[1][1].y = i32();
        points_1[1][1].z = i32();
        let _e74 = points_1[0i][0i].z;
        out[3i] = _e74;
        let _e82 = points_1[1i][1i].x;
        out[4i] = _e82;
        let _e90 = points_1[1i][1i].z;
        out[5i] = _e90;
    }
    {
        original_2.x = 1i;
        original_2.y = 2i;
        original_2.z = 3i;
        original_2.w = 4i;
        let _e103 = original_2;
        points_2[0][0] = _e103;
        points_2[0][1] = vec4<i32>();
        let _e108 = original_2;
        points_2[1][0] = _e108;
        points_2[1][1].x = 5i;
        points_2[1][1].y = i32();
        points_2[1][1].z = i32();
        points_2[1][1].w = i32();
        let _e125 = points_2[0i][0i].w;
        out[6i] = _e125;
        let _e133 = points_2[1i][1i].x;
        out[7i] = _e133;
        let _e141 = points_2[1i][1i].w;
        out[8i] = _e141;
    }
    {
        original_3.x = u32(1i);
        original_3.y = u32(2i);
        let _e152 = original_3;
        points_3[0][0] = _e152;
        points_3[0][1] = vec2<u32>();
        let _e157 = original_3;
        points_3[1][0] = _e157;
        points_3[1][1].x = u32(5i);
        points_3[1][1].y = u32();
        let _e171 = points_3[0i][0i].y;
        out[9i] = i32(_e171);
        let _e180 = points_3[1i][1i].x;
        out[10i] = i32(_e180);
        let _e189 = points_3[1i][1i].y;
        out[11i] = i32(_e189);
    }
    {
        original_4.x = u32(1i);
        original_4.y = u32(2i);
        original_4.z = u32(3i);
        let _e204 = original_4;
        points_4[0][0] = _e204;
        points_4[0][1] = uint3_();
        let _e209 = original_4;
        points_4[1][0] = _e209;
        points_4[1][1].x = u32(5i);
        points_4[1][1].y = u32();
        points_4[1][1].z = u32();
        let _e225 = points_4[0i][0i].z;
        out[12i] = i32(_e225);
        let _e234 = points_4[1i][1i].x;
        out[13i] = i32(_e234);
        let _e243 = points_4[1i][1i].z;
        out[14i] = i32(_e243);
    }
    {
        original_5.x = u32(1i);
        original_5.y = u32(2i);
        original_5.z = u32(3i);
        original_5.w = u32(4i);
        let _e261 = original_5;
        points_5[0][0] = _e261;
        points_5[0][1] = vec4<u32>();
        let _e266 = original_5;
        points_5[1][0] = _e266;
        points_5[1][1].x = u32(5i);
        points_5[1][1].y = u32();
        points_5[1][1].z = u32();
        points_5[1][1].w = u32();
        let _e284 = points_5[0i][0i].w;
        out[15i] = i32(_e284);
        let _e293 = points_5[1i][1i].x;
        out[16i] = i32(_e293);
        let _e302 = points_5[1i][1i].w;
        out[17i] = i32(_e302);
    }
    {
        original_6.x = f32(1i);
        original_6.y = f32(2i);
        let _e314 = original_6;
        points_6[0][0] = _e314;
        points_6[0][1] = vec2<f32>();
        let _e319 = original_6;
        points_6[1][0] = _e319;
        points_6[1][1].x = f32(5i);
        points_6[1][1].y = f32();
        let _e333 = points_6[0i][0i].y;
        out[18i] = i32(_e333);
        let _e342 = points_6[1i][1i].x;
        out[19i] = i32(_e342);
        let _e351 = points_6[1i][1i].y;
        out[20i] = i32(_e351);
    }
    {
        original_7.x = f32(1i);
        original_7.y = f32(2i);
        original_7.z = f32(3i);
        let _e366 = original_7;
        points_7[0][0] = _e366;
        points_7[0][1] = float3_();
        let _e371 = original_7;
        points_7[1][0] = _e371;
        points_7[1][1].x = f32(5i);
        points_7[1][1].y = f32();
        points_7[1][1].z = f32();
        let _e387 = points_7[0i][0i].z;
        out[21i] = i32(_e387);
        let _e396 = points_7[1i][1i].x;
        out[22i] = i32(_e396);
        let _e405 = points_7[1i][1i].z;
        out[23i] = i32(_e405);
    }
    {
        original_8.x = f32(1i);
        original_8.y = f32(2i);
        original_8.z = f32(3i);
        original_8.w = f32(4i);
        let _e423 = original_8;
        points_8[0][0] = _e423;
        points_8[0][1] = vec4<f32>();
        let _e428 = original_8;
        points_8[1][0] = _e428;
        points_8[1][1].x = f32(5i);
        points_8[1][1].y = f32();
        points_8[1][1].z = f32();
        points_8[1][1].w = f32();
        let _e446 = points_8[0i][0i].w;
        out[24i] = i32(_e446);
        let _e455 = points_8[1i][1i].x;
        out[25i] = i32(_e455);
        let _e464 = points_8[1i][1i].w;
        out[26i] = i32(_e464);
    }
    changed[0][0] = 1i;
    local = 1u;
    loop {
        let _e473 = local;
        if (_e473 >= 2u) {
            break;
        }
        changed[0][_e473] = i32();
        continuing {
            let _e477 = local;
            local = (_e477 + 1u);
        }
    }
    changed[0i][1i] = 9i;
    changed[1][0] = 2i;
    changed[1][1] = 3i;
    local_1 = 0u;
    loop {
        let _e494 = local_1;
        if (_e494 >= 2u) {
            break;
        }
        local_2 = 0u;
        loop {
            let _e500 = local_2;
            if (_e500 >= 3u) {
                break;
            }
            nested_empty[_e494][_e500] = i32();
            continuing {
                let _e504 = local_2;
                local_2 = (_e504 + 1u);
            }
        }
        continuing {
            let _e507 = local_1;
            local_1 = (_e507 + 1u);
        }
    }
    partial[0][0] = 1i;
    local_3 = 1u;
    loop {
        let _e517 = local_3;
        if (_e517 >= 3u) {
            break;
        }
        partial[0][_e517] = i32();
        continuing {
            let _e521 = local_3;
            local_3 = (_e521 + 1u);
        }
    }
    local_4 = 1u;
    loop {
        let _e527 = local_4;
        if (_e527 >= 2u) {
            break;
        }
        local_5 = 0u;
        loop {
            let _e533 = local_5;
            if (_e533 >= 3u) {
                break;
            }
            partial[_e527][_e533] = i32();
            continuing {
                let _e537 = local_5;
                local_5 = (_e537 + 1u);
            }
        }
        continuing {
            let _e540 = local_4;
            local_4 = (_e540 + 1u);
        }
    }
    volume[0][0][0] = 1i;
    local_6 = 1u;
    loop {
        let _e551 = local_6;
        if (_e551 >= 3u) {
            break;
        }
        volume[0][0][_e551] = i32();
        continuing {
            let _e555 = local_6;
            local_6 = (_e555 + 1u);
        }
    }
    local_7 = 1u;
    loop {
        let _e561 = local_7;
        if (_e561 >= 2u) {
            break;
        }
        local_8 = 0u;
        loop {
            let _e567 = local_8;
            if (_e567 >= 3u) {
                break;
            }
            volume[0][_e561][_e567] = i32();
            continuing {
                let _e571 = local_8;
                local_8 = (_e571 + 1u);
            }
        }
        continuing {
            let _e574 = local_7;
            local_7 = (_e574 + 1u);
        }
    }
    local_9 = 1u;
    loop {
        let _e580 = local_9;
        if (_e580 >= 2u) {
            break;
        }
        local_10 = 0u;
        loop {
            let _e586 = local_10;
            if (_e586 >= 2u) {
                break;
            }
            local_11 = 0u;
            loop {
                let _e592 = local_11;
                if (_e592 >= 3u) {
                    break;
                }
                volume[_e580][_e586][_e592] = i32();
                continuing {
                    let _e596 = local_11;
                    local_11 = (_e596 + 1u);
                }
            }
            continuing {
                let _e599 = local_10;
                local_10 = (_e599 + 1u);
            }
        }
        continuing {
            let _e602 = local_9;
            local_9 = (_e602 + 1u);
        }
    }
    let _e615 = changed[0i][1i];
    shared_values[1i][2i] = _e615;
    let _e624 = changed[1i][0i];
    let _e629 = changed[1i][1i];
    shared_points[1i][0i] = float3_(f32(_e624), f32(_e629), f32(4i));
    storageBarrier();
    workgroupBarrier();
    let _e645 = shared_values[1i][2i];
    let _e646 = nested_empty[1i][2i];
    let _e652 = partial[1i][2i];
    let _e660 = volume[1i][1i][2i];
    out[27i] = (((_e645 + _e646) + _e652) + _e660);
    let _e669 = shared_points[1i][0i].x;
    out[28i] = i32(_e669);
    let _e678 = shared_points[1i][0i].y;
    out[29i] = i32(_e678);
}
