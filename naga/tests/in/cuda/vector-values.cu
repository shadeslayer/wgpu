using Triple = float3;
using Whole = int3;

__device__ int choose(int2 value) {
    return value.x * 10 + value.y;
}

__device__ int choose(uint3 value) {
    return int(value.z);
}

__device__ int choose(float4 value) {
    return int(value.w);
}

__device__ Whole bump(Whole value) {
    value.y += 1;
    return value;
}

__device__ Triple shift(Triple value) {
    value.x += 2.0f;
    value.z = value.x + value.y;
    return value;
}

__device__ int pair(Triple first, int second) {
    return int(first.x) * 10 + second;
}

__global__ void vector_values(int *out) {
    int seed = out[0];
    float2 f2 = make_float2(seed, 1.5f);
    Triple f3 = make_float3(1.0f, 2.0f, 3.0f);
    float4 f4 = make_float4(1, 2, 3, 4);
    int2 i2 = make_int2(seed, -2);
    Whole i3 = make_int3(1, 2, 3);
    int4 i4 = make_int4(1, 2, 3, 4);
    uint2 u2 = make_uint2(-1, 2);
    uint3 u3 = make_uint3(1, 2, 3);
    uint4 u4 = make_uint4(1, 2, 3, 4);
    out[0] = int(f2.x + f2.y);
    out[1] = int(f3.x + f3.y + f3.z);
    out[2] = int(f4.w);
    out[3] = i2.x + i2.y;
    out[4] = i3.z;
    out[5] = i4.x + i4.w;
    out[6] = u2.x == 0xffff'ffffu;
    out[7] = int(u3.x + u3.z);
    out[8] = int(u4.w);
    using Frozen = const float3;
    Frozen frozen = f3;
    Triple copied = frozen;
    copied.x = 9.0f;
    out[9] = int(frozen.x * 10.0f + copied.x);
    Whole changed = bump(i3);
    out[10] = changed.y * 10 + i3.y;
    out[11] = int(shift(f3).z);
    out[12] = choose(i2) + choose(u3) + choose(f4);
    Triple samples[2];
    samples[0] = f3;
    samples[1] = make_float3(5, 6, 7);
    int index = 0;
    out[13] = int(samples[index++].x += ++index);
    out[14] = index * 100 + int(samples[0].x) * 10 + int(samples[1].x);
    bool choose_first = false;
    out[15] = int((choose_first ? samples[0] : samples[1]).y++);
    out[16] = int(samples[1].y);
    out[17] = int(((choose_first ? samples[0] : samples[1]) = f3).z);
    out[18] = int((true ? make_float3(7, 8, 9) : f3).y);
    {
        using float3 = int;
        float3 local = seed + 2;
        out[19] = local;
    }
    int argument = 3;
    Whole captured = make_int3(argument, argument++, argument);
    out[20] = captured.x * 100 + captured.y * 10 + captured.z;
    out[21] = argument;
    out[22] = pair(f3, (f3.x = 9.0f, 4));
    out[23] = int(f3.x);
}
