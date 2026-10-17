__global__ void barriers(unsigned int *out) {
    __shared__ unsigned int values[64];
    __shared__ unsigned int broadcast;
    unsigned int lane = threadIdx.x;
    unsigned int base = blockIdx.x * 256u;
    values[lane] = blockIdx.x * 2000u + lane + 7u;
    out[base + lane] = blockIdx.x * 1000u + lane;
    if (lane == 0u) {
        broadcast = blockIdx.x * 100u + 11u;
    }
    __syncthreads();
    out[base + 64u + lane] = values[63u - lane];
    out[base + 128u + lane] = out[base + ((lane + 1u) % 64u)];
    out[base + 192u + lane] = broadcast;
}
