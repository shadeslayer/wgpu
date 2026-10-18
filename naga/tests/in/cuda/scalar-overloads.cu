using Slot = unsigned int;

__device__ bool classify(bool value) {
    return !value;
}

__device__ int classify(int value) {
    return value + 10;
}

__device__ unsigned int classify(unsigned int value) {
    return value + 20u;
}

__device__ float classify(float value) {
    return value * 1.5f + 0.5f;
}

__device__ int rank(int left, float right) {
    return left * 10 + int(right);
}

__device__ int rank(float left, int right) {
    return 100 + int(left) * 10 + right;
}

__device__ int pair(float left, int right) {
    return int(left) * 10 + right;
}

__device__ int pair(unsigned int left, float right) {
    return 200 + int(left) + int(right);
}

__device__ int promote(unsigned int left, float right) {
    return 200 + int(left) + int(right);
}

__device__ int promote(float left, float right) {
    return 300 + int(left) + int(right);
}

__device__ void touch(bool stop, int value) {
    if (stop) {
        return;
    }
    value = classify(value);
}

__device__ int touch(int value) {
    return classify(value) + 3;
}

__device__ int stage(int value) {
    return value + 2;
}

__device__ int frozen(float value) {
    return stage(value);
}

__device__ int stage(float value) {
    return int(value) + 100;
}

__global__ void scalar_overloads(int *out, const int *input, Slot n) {
    Slot i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        Slot base = i * 13u;
        int value = input[i];
        bool choose = i % 2u == 0u;
        out[base] = classify(choose);
        out[base + 1u] = classify(value);
        out[base + 2u] = classify(Slot(value + 16));
        out[base + 3u] = classify(float(value));
        out[base + 4u] = rank(choose, float(value) + 0.5f);
        out[base + 5u] = rank(float(value) + 0.5f, choose);
        out[base + 6u] = promote(Slot(value + 16), value);
        int effect = value;
        touch(choose, effect++);
        out[base + 7u] = effect;
        out[base + 8u] = touch(effect);
        out[base + 9u] = frozen(float(value) + 0.75f);
        out[base + 10u] = stage(float(value) + 0.75f);
        int nested = touch(classify(effect++));
        out[base + 11u] = nested * 10 + effect;
        int argument = value + 8;
        int captured = pair(argument, argument++);
        out[base + 12u] = captured * 100 + argument;
    }
}
