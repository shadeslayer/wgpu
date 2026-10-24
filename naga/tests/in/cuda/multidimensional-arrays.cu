__global__ void multidimensional_arrays(int *out) {
    {
        int2 original{1, 2};
        const int2 points[2][2]{{original, {}}, {original, 5}};
        out[0] = int(points[0][0].y);
        out[1] = int(points[1][1].x);
        out[2] = int(points[1][1].y);
    }
    {
        int3 original{1, 2, 3};
        const int3 points[2][2]{{original, {}}, {original, 5}};
        out[3] = int(points[0][0].z);
        out[4] = int(points[1][1].x);
        out[5] = int(points[1][1].z);
    }
    {
        int4 original{1, 2, 3, 4};
        const int4 points[2][2]{{original, {}}, {original, 5}};
        out[6] = int(points[0][0].w);
        out[7] = int(points[1][1].x);
        out[8] = int(points[1][1].w);
    }
    {
        uint2 original{1, 2};
        const uint2 points[2][2]{{original, {}}, {original, 5}};
        out[9] = int(points[0][0].y);
        out[10] = int(points[1][1].x);
        out[11] = int(points[1][1].y);
    }
    {
        uint3 original{1, 2, 3};
        const uint3 points[2][2]{{original, {}}, {original, 5}};
        out[12] = int(points[0][0].z);
        out[13] = int(points[1][1].x);
        out[14] = int(points[1][1].z);
    }
    {
        uint4 original{1, 2, 3, 4};
        const uint4 points[2][2]{{original, {}}, {original, 5}};
        out[15] = int(points[0][0].w);
        out[16] = int(points[1][1].x);
        out[17] = int(points[1][1].w);
    }
    {
        float2 original{1, 2};
        const float2 points[2][2]{{original, {}}, {original, 5}};
        out[18] = int(points[0][0].y);
        out[19] = int(points[1][1].x);
        out[20] = int(points[1][1].y);
    }
    {
        float3 original{1, 2, 3};
        const float3 points[2][2]{{original, {}}, {original, 5}};
        out[21] = int(points[0][0].z);
        out[22] = int(points[1][1].x);
        out[23] = int(points[1][1].z);
    }
    {
        float4 original{1, 2, 3, 4};
        const float4 points[2][2]{{original, {}}, {original, 5}};
        out[24] = int(points[0][0].w);
        out[25] = int(points[1][1].x);
        out[26] = int(points[1][1].w);
    }
    int changed[2][2]{{1}, {(changed[0][1] = 9, 2), 3}};
    int nested_empty[2][3]{};
    int partial[2][3]{{1}};
    int volume[2][2][3]{{{1}}};
    __shared__ int shared_values[2][3];
    __shared__ float3 shared_points[2][2];
    shared_values[1][2] = changed[0][1];
    shared_points[1][0] = make_float3(changed[1][0], changed[1][1], 4);
    __syncthreads();
    out[27] = shared_values[1][2] + nested_empty[1][2] + partial[1][2] + volume[1][1][2];
    out[28] = int(shared_points[1][0].x);
    out[29] = int(shared_points[1][0].y);
}
