__global__ void shared_storage(unsigned int *out) {
    __shared__ unsigned int values[64];
    __shared__ int signed_value;
    __shared__ float float_value;
    unsigned int lane = threadIdx.x;
    unsigned int base = blockIdx.x * 128u;
    values[lane] = blockIdx.x * 1000u + lane + 10u;
    out[base + lane] = values[lane];
    if (lane == 0u) {
        signed_value = -7;
        float_value = 3.5f;
        out[base + 64u] = signed_value + 10;
        out[base + 65u] = float_value * 2.0f;
    }
}
