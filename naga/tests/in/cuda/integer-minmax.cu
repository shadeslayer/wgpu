using Whole = int;
using Natural = unsigned int;

__device__ Whole min(Whole first, Whole second, Whole third) {
    return first + second + third + 100;
}

__device__ Whole bounded(Whole value) {
    return min(max(value, -2), 2);
}

__global__ void integer_minmax(int *out) {
    int seed = out[0];
    out[0] = min(seed, -7);
    out[1] = max(seed, -7);
    out[2] = min(3u, 9u);
    out[3] = max(3u, 9u);
    out[4] = min(-1, 2u) == 2u;
    out[5] = max(-1, 2u) == 0xffff'ffffu;
    out[6] = min(2u, -1) == 2u;
    out[7] = max(2u, -1) == 0xffff'ffffu;
    out[8] = min(true, false);
    out[9] = max(false, true);
    out[10] = min(max(seed, -5), 2);
    out[11] = min(Whole(-2), Natural(4u)) == 4u;
    out[12] = min(seed, 4, 5);
    out[13] = bounded(seed);
    int first = 3;
    int second = 7;
    out[14] = max(first++, second++);
    out[15] = first;
    out[16] = second;
    int value = seed;
    out[17] = max(value, value++);
    out[18] = value;
    out[19] = min(-2147483647 - 1, 2147483647) == -2147483647 - 1;
    out[20] = max(0xffff'ffffu, 0u) == 0xffff'ffffu;
    out[21] = min(false, 3u) == 0u;
    out[22] = max(true, 0u) == 1u;
    out[23] = min(seed, seed);
}
