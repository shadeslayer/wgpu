__global__ void float_separators(float *out, const float *input, unsigned int n) {
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        unsigned int base = i * 12u;
        out[base] = 1'024.5f;
        out[base + 1u] = .1'25f;
        out[base + 2u] = 1'2.f;
        out[base + 3u] = 2.5e+0'1f;
        out[base + 4u] = 5e-0'1f;
        out[base + 5u] = 6'25e-0'2F;
        out[base + 6u] = -1'024.5F;
        out[base + 7u] = 0'0.1'25f;
        out[base + 8u] = 3.14'15'927f;
        out[base + 9u] = 1.000'000'059'604'644'775'390'625f;
        out[base + 10u] = 1.000'000'178'813'934'326'171'875f;
        out[base + 11u] = input[i] * 0.2'5f + 1'0.0f;
    }
}
