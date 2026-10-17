__device__ int next_value(int value) {
    return value + 1;
}

__device__ void consume(bool stop, int value, unsigned int count, float scale) {
    value += count;
    if (stop) {
        return;
    }
    value *= scale;
}

__device__ void forward(bool stop, int value) {
    return consume(stop, next_value(value), 2u, 0.5f);
}

__global__ void void_functions(int *out, unsigned int n) {
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        unsigned int base = i * 6u;
        int value = 2;
        consume(false, value++, 1u, 1.5f);
        out[base] = value;
        (consume(true, next_value(value++), 2u, 0.5f));
        out[base + 1u] = value;
        forward(i % 2u == 0u, value++);
        out[base + 2u] = value;
        int iterations = 0;
        for (consume(0, value++, 0, 1u); iterations < 3; consume(false, ++value, 0u, 1.0f)) {
            ++iterations;
            if (iterations == 1) {
                continue;
            }
        }
        out[base + 3u] = value;
        out[base + 4u] = iterations;
        out[base + 5u] = next_value(value);
    }
}
