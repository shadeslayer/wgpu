using Slot = unsigned int;
using Fixed = const int;

static __device__ int increment(int value) {
    value += 2;
    return value;
}

__device__ static inline int twice(int value) {
    return increment(value) * 2;
}

__forceinline__ __device__ int advance(int value) {
    return twice(value) + 1;
}

__device__ __noinline__ static int retreat(int value) {
    return value - 3;
}

static __host__ __device__ __forceinline__ float scaled(const float value) {
    return value * 1.5f + 0.5f;
}

__noinline__ __host__ __device__ bool differs(Fixed value, const int other) {
    return value != other;
}

static __device__ __forceinline__ void consume(bool stop, int value) {
    if (stop) {
        return;
    }
    value = advance(value);
}

__device__ static __noinline__ void forward(bool stop, int value) {
    return consume(stop, retreat(value));
}

unsigned __noinline__ __host__ __device__ int plus_one(unsigned int value) {
    return value + 1u;
}

static __global__ void function_specifiers(int *out, const int *input, Slot n) {
    Slot i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        Slot base = i * 12u;
        int value = input[i];
        bool choose = i % 2u == 0u;
        out[base] = increment(value);
        out[base + 1u] = twice(value);
        out[base + 2u] = advance(value);
        out[base + 3u] = retreat(value);
        out[base + 4u] = scaled(float(value));
        out[base + 5u] = differs(value, input[i]);
        int effect = value;
        consume(choose, effect++);
        out[base + 6u] = effect;
        forward(!choose, effect++);
        out[base + 7u] = effect;
        out[base + 8u] = advance(effect);
        out[base + 9u] = plus_one(Slot(effect + 16));
        out[base + 10u] = differs(effect, input[i]);
        out[base + 11u] = effect;
    }
}
