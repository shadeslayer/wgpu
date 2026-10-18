using Number = int;
using Frozen = const int;

__device__ int pair(int first, int second) {
    return first * 10 + second;
}

__device__ int sum(int seed) {
    int values[]{seed, seed + 1};
    return values[0] + values[1];
}

__device__ int3 triple(int seed) {
    int3 value{seed, seed + 1, seed + 2};
    return value;
}

__global__ void inferred_arrays(int *out) {
    int seed = out[0];
    int values[]{seed, seed + 1, seed + 2};
    int copied[] = {4, 5};
    int zeroes[]{{}, {}};
    bool flags[]{true, false, true};
    unsigned counts[]{3u, 5u};
    float samples[]{1.5f, 2.5f};
    out[0] = values[0] + values[1] + values[2];
    out[1] = copied[0];
    out[2] = zeroes[0] + zeroes[1];
    out[3] = int(flags[0]) + int(flags[1]) + int(flags[2]);
    out[4] = int(counts[0] + counts[1]);
    out[5] = int(samples[0] + samples[1]);
    int self[]{1, self[0] + 1, self[1] + 1};
    out[6] = self[0] * 100 + self[1] * 10 + self[2];
    int changed[]{1, (changed[0] = 7, 2), changed[0]};
    out[7] = changed[0];
    out[8] = changed[2];
    int3 points[]{{1, points[0].x + 1, points[0].y + 1}, {points[0]}};
    out[9] = points[1].z;
    int2 flat[]{1, 2, 3};
    out[10] = flat[1].y;
    out[11] = flat[1].x;
    int3 original{4, 5, 6};
    int3 copies[]{original, {original}, 7, 8};
    original.x = 99;
    out[12] = copies[0].x;
    out[13] = copies[1].y;
    out[14] = copies[2].z;
    Number Number[]{2, 3}, next[]{Number[1], Number[0]};
    out[15] = next[0];
    constexpr int Count = (self[2], 3);
    int scratch[Count]{};
    out[16] = scratch[2];
    float pending[]{1.0f, (pending[0], 2)};
    out[17] = int(pending[0] + pending[1]);
    float3 pending_vectors[]{{1.0f, (pending_vectors[0].x, 2)}, {3.0f}};
    out[18] = int(pending_vectors[0].x + pending_vectors[0].y + pending_vectors[1].x);
    int counter = seed;
    int ordered[]{counter++, counter++, counter};
    out[19] = ordered[0] * 100 + ordered[1] * 10 + ordered[2];
    out[20] = counter;
    int called[]{pair(counter++, 4), counter};
    out[21] = called[0] + called[1];
    out[22] = counter;
    int zero_total = 0;
    for (int iteration = 0; iteration < 3; ++iteration) {
        int2 reset[]{{iteration}, {}};
        zero_total += reset[0].y + reset[1].x;
        reset[0].y = 9;
        reset[1].x = 8;
    }
    out[23] = zero_total;
    int total = 0;
    for (int iteration = 0; iteration < 4; ++iteration) {
        if (iteration == 1) continue;
        total += iteration;
    }
    out[24] = total;
    out[25] = sum(4);
    Frozen frozen[]{2, 3};
    out[26] = frozen[0] + frozen[1];
    float signed_zeroes[]{-0.0f, {}};
    out[27] = __float_as_int(signed_zeroes[0]);
    out[28] = __float_as_int(signed_zeroes[1]);
    int3 from_calls[]{triple(6), triple(9)};
    out[29] = from_calls[0].x + from_calls[1].z;
}
