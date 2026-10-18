using Slot = unsigned int;
using Value = int;
using Weight = float;
using Flag = bool;

__device__ Value select_value(Value value, Flag choose_first) {
    Value result;
    if (choose_first) {
        result = value + 3;
    } else {
        result = value - 2;
    }
    return result;
}

__host__ __device__ Weight scaled(Weight value) {
    Weight result;
    result = value * 2.0f + 4.0f;
    return result;
}

__device__ Slot accumulate(Slot limit) {
    Slot sum;
    sum = 0u;
    for (Slot i = 0u; i < limit; ++i) {
        Slot term;
        if (i % 2u == 0u) {
            term = i + 1u;
        } else {
            term = i * 2u;
        }
        sum += term;
    }
    return sum;
}

__global__ void uninitialized(int *out, const int *input, Slot n) {
    Slot i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        Slot base = i * 12u;
        int value = input[i];
        int chosen;
        if (i % 2u == 0u) {
            chosen = value + 7;
        } else {
            chosen = value - 4;
        }
        out[base] = chosen;
        out[base + 1u] = select_value(value, i % 3u == 0u);
        float weighted;
        weighted = float(value) * 1.5f + 2.0f;
        out[base + 2u] = int(weighted);
        bool negative;
        negative = value < 0;
        out[base + 3u] = negative;
        Slot magnitude;
        magnitude = Slot(value + 5);
        out[base + 4u] = magnitude * 3u;
        out[base + 5u] = accumulate(i % 5u);
        int total = 0;
        for (int iteration = 0; iteration < 3; ++iteration) {
            int copied;
            copied = value + iteration;
            total += copied;
        }
        out[base + 6u] = total;
        int descending = 0;
        int iteration = 0;
        for (int staged; iteration < 3; ++iteration) {
            staged = value - iteration;
            descending += staged;
        }
        out[base + 7u] = descending;
        int remaining;
        remaining = 2;
        int partial;
        partial = value;
        while (remaining > 0) {
            int next;
            next = partial + remaining;
            partial = next;
            --remaining;
        }
        out[base + 8u] = partial;
        int shadow;
        shadow = value;
        {
            int shadow;
            shadow = value + 10;
            out[base + 9u] = shadow;
        }
        out[base + 10u] = shadow;
        out[base + 11u] = scaled(float(value));
    }
}
