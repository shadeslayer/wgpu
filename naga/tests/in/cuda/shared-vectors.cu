using Triple = float3;

__device__ Triple shift(Triple value) {
    value.x += 1.0f;
    value.y = value.z - value.y;
    return value;
}

__global__ void shared_vectors(float *output) {
    __shared__ Triple values[8];
    __shared__ Triple broadcast;
    unsigned lane = threadIdx.x;
    unsigned group = blockIdx.x;
    float base = float(group * 16u + lane);
    values[lane] = make_float3(base + 0.5f, -base - 1.25f, base + 2.5f);
    if (lane == 0u) {
        broadcast = make_float3(float(group) + 0.25f, float(group) + 0.5f, float(group) + 0.75f);
    }
    __syncthreads();
    unsigned next = (lane + 1u) % 8u;
    Triple copied = shift(values[next]);
    {
        using Triple = int;
        Triple offset = 1;
        copied.x += offset;
    }
    __syncthreads();
    bool choose_shared = (lane & 1u) != 0u;
    (choose_shared ? values[lane] : copied).y += broadcast.x;
    values[lane].z = copied.x;
    unsigned row = (group * 8u + lane) * 9u;
    output[row] = copied.x;
    output[row + 1u] = copied.y;
    output[row + 2u] = copied.z;
    __syncthreads();
    output[row + 3u] = values[next].x;
    output[row + 4u] = values[next].y;
    output[row + 5u] = values[next].z;
    output[row + 6u] = broadcast.x;
    output[row + 7u] = broadcast.y;
    output[row + 8u] = broadcast.z;
}
