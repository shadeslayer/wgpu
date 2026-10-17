__device__ float affine(float value, float scale, int offset) {
    value *= scale;
    return value + offset;
}

__device__ bool nonnegative(int value) {
    return value >= 0;
}

__device__ unsigned int sum_steps(unsigned int count, bool include_even) {
    unsigned int total = 0u;
    for (unsigned int i = 0u; i < count; ++i) {
        if (include_even || i % 2u != 0u) {
            total += i;
        }
    }
    return total;
}

__device__ int choose(int value) {
    if (nonnegative(value)) {
        return affine(value, 2.0f, 1);
    }
    return -value;
}

__device__ int pair(int a, int b) {
    return a * 10 + b;
}

__global__ void device_functions(int *out, unsigned int n) {
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        unsigned int base = i * 10u;
        int value = int(i) - 35;
        int argument = 3;
        out[base] = choose(value);
        out[base + 1u] = affine(value, 1.5f, 2);
        out[base + 2u] = sum_steps(5, true);
        out[base + 3u] = sum_steps(5u, 0);
        out[base + 4u] = affine(value, 2u, true);
        out[base + 5u] = value;
        out[base + 6u] = choose(value) + choose(value + 1);
        out[base + 7u] = nonnegative(value);
        out[base + 8u] = pair(argument, argument++);
        out[base + 9u] = argument;
    }
}
