typedef unsigned int Slot;
using Weight = float;
typedef const int Fixed;
using FixedWeight = const Weight;
using Flag = bool;

inline __device__ int increment(int value) {
    value += 2;
    return value;
}

__device__ int inline doubled(int value) {
    return increment(value) * 2;
}

unsigned inline __device__ int advance(unsigned int value) {
    return value + 3u;
}

int __device__ unsigned inline retreat(unsigned int value) {
    return value - 1u;
}

__device__ inline FixedWeight scale(const Weight value, Fixed bias) {
    return value * 1.5f + bias;
}

inline bool __device__ same(Fixed left, const int right) {
    return left == right;
}

__device__ const inline void consume(Flag stop, int value) {
    if (stop) {
        return;
    }
    value = doubled(value);
}

__device__ void const inline forward(Flag stop, int value) {
    return consume(stop, increment(value));
}

const inline __global__ void inline_functions(int *out, const int *input, Slot n) {
    Slot i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        Slot base = i * 12u;
        int value = input[i];
        out[base] = increment(value);
        out[base + 1u] = value;
        out[base + 2u] = doubled(value);
        out[base + 3u] = advance(Slot(value));
        out[base + 4u] = retreat(Slot(value));
        out[base + 5u] = scale(Weight(value), Fixed(2));
        out[base + 6u] = same(value, input[i]);
        consume(i % 2u == 0u, value++);
        out[base + 7u] = value;
        (forward(i % 2u != 0u, value++));
        out[base + 8u] = value;
        out[base + 9u] = same(value, input[i]);
        out[base + 10u] = increment(increment(value));
        out[base + 11u] = value;
    }
}
