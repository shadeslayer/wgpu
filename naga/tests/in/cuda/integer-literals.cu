__global__ void integer_literals(unsigned int *out, const unsigned int *input, unsigned int n) {
    __shared__ unsigned int shared_values[0b100'0000];
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        unsigned int value = input[i];
        unsigned int shift = value & 3U;
        unsigned int base = i * 18u;
        shared_values[threadIdx.x] = value ^ 0xFf;
        out[base] = shared_values[threadIdx.x];
        out[base + 1u] = value & 0377;
        out[base + 2u] = value & 0b1111'0000u;
        out[base + 3u] = value + 1'000U;
        out[base + 4u] = 0x8000'0000 >> shift;
        out[base + 5u] = (int)0x80000000 >> shift;
        out[base + 6u] = 0xFFFF'FFFF ^ value;
        out[base + 7u] = value < 020000000000;
        out[base + 8u] = (int)value < 2'147'483'647;
        out[base + 9u] = 0b1000'0000'0000'0000'0000'0000'0000'0000 >> shift;
        out[base + 10u] = 0XFFU + value;
        out[base + 11u] = value + 0B1010u + 010U + 10u;
        out[base + 12u] = value + 0'1'7;
        out[base + 13u] = 037777777777U - value;
        out[base + 14u] = 0xdeadBEEF ^ value;
        out[base + 15u] = ~0x7fffFFFF < (int)value;
        out[base + 16u] = value + 0x1'e+2;
        out[base + 17u] = value + 0x1'E-2;
    }
}
