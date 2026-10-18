using Slot = unsigned int;
using Number = int;
using Fixed = const int;

__device__ Number cached_type(Number value) {
    Number Number = value, next = Number + 1, result;
    result = next * 2;
    return result;
}

__device__ int pair_add(int left, int right) {
    return left + right;
}

__global__ void local_declarators(int *out, const int *input, Slot n) {
    Slot i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        Slot base = i * 12u;
        int value = input[i];
        int first = value++, second = value++, third = value;
        out[base] = first * 100 + second * 10 + third;
        out[base + 1u] = value;
        int omitted, initialized = value + 3, final;
        omitted = initialized + 2;
        final = omitted + initialized;
        out[base + 2u] = final;
        const int fixed = value + 1, fixed_next = fixed + 2;
        out[base + 3u] = fixed_next;
        Fixed alias_first = value + 4, alias_next = alias_first + 1;
        out[base + 4u] = alias_next;
        bool positive = value > 0, same = positive && value == input[i] + 2;
        out[base + 5u] = same;
        float low = float(value) * 0.5f, high = low + 1.5f;
        out[base + 6u] = int(high * 2.0f);
        Slot start = i % 5u, finish = start + 3u;
        out[base + 7u] = finish;
        out[base + 8u] = cached_type(value);
        int outside = value;
        {
            int outside = value + 5, derived = outside + 2;
            out[base + 9u] = derived;
        }
        int total = 0;
        for (int iteration = 0, seed = value; iteration < 4; ++iteration) {
            if (iteration == 1) {
                continue;
            }
            int term = seed + iteration, copied = term;
            total += copied;
        }
        out[base + 10u] = total;
        int from_call = pair_add(value, 2), from_prior = pair_add(from_call, 3);
        out[base + 11u] = from_prior + outside;
    }
}
