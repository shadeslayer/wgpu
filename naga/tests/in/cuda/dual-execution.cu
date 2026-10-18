typedef unsigned int Slot;
using Value = int;
using Fixed = const int;
using Weight = const float;
using Flag = bool;

__host__ __device__ Value step(Value value) {
    value += 3;
    return value;
}

__device__ int __host__ doubled(int value) {
    return step(value) * 2;
}

unsigned __host__ inline __device__ int advance(unsigned int value) {
    return value + 5u;
}

int __device__ unsigned __host__ inline retreat(unsigned int value) {
    return value - 2u;
}

__host__ Weight inline __device__ scale(Weight value, Fixed bias) {
    return value * 2.0f + bias;
}

inline bool __device__ __host__ same(Fixed left, const int right) {
    return left == right;
}

__device__ void const __host__ consume(Flag stop, int value) {
    if (stop) {
        return;
    }
    value = doubled(value);
}

__host__ __device__ const inline void forward(Flag stop, int value) {
    return consume(stop, step(value));
}

__device__ int device_only(int value) {
    return step(value) + 1;
}

inline __global__ const void dual_execution(int *out, const int *input, Slot n) {
    Slot i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        Slot base = i * 12u;
        int value = input[i];
        out[base] = step(value);
        out[base + 1u] = value;
        out[base + 2u] = doubled(value);
        out[base + 3u] = advance(Slot(value));
        out[base + 4u] = retreat(Slot(value));
        out[base + 5u] = scale(Weight(value), Fixed(4));
        out[base + 6u] = same(value, input[i]);
        consume(i % 2u == 0u, value++);
        out[base + 7u] = value;
        (forward(i % 2u != 0u, value++));
        out[base + 8u] = value;
        out[base + 9u] = same(value, input[i]);
        out[base + 10u] = device_only(step(value));
        out[base + 11u] = value;
    }
}
