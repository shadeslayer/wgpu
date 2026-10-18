using Number = int;
constexpr int Global{2};
constexpr float Scale = {0.5f};

__device__ int pair(int first, int second) {
    return first * 10 + second;
}

__device__ int list_sum(int seed) {
    int values[3]{seed, seed + 1};
    return values[0] + values[1] + values[2];
}

__device__ int3 make_list(int seed) {
    int3 value{seed, seed + 1, seed + 2};
    return value;
}

__global__ void list_initialization(int *out) {
    int seed = out[0];
    int zero{}, copy_zero = {};
    bool enabled{true};
    unsigned count{7};
    float sample{1.5f};
    constexpr int Local = {3};
    const int Safe{true ? 3 : (1 / 0)};
    int safe_values[Safe]{1};
    out[0] = zero;
    out[1] = copy_zero;
    out[2] = int(enabled);
    out[3] = int(count);
    out[4] = int(sample * 2.0f);
    out[5] = Global + Local;
    out[6] = safe_values[2];
    int values[3] = {1, values[0] + 1, values[1] + 1};
    out[7] = values[0] + values[1] + values[2];
    int changed[3]{1, (changed[0] = 7, 2), changed[0]};
    out[8] = changed[0];
    out[9] = changed[2];
    int3 point{1, point.x + 1, point.y + 1};
    out[10] = point.z;
    int3 changed_point{1, (changed_point.x = 9, 2), changed_point.x};
    out[11] = changed_point.x;
    out[12] = changed_point.z;
    int3 nested[2]{{1, 2}, {3}};
    out[13] = nested[0].z + nested[1].x;
    int2 flat[2]{1, 2, 3};
    out[14] = flat[0].x + flat[0].y + flat[1].x + flat[1].y;
    int3 original{2, 3, 4};
    int3 copied{original};
    int3 copies[2]{original, 5, 6};
    original.x = 99;
    out[15] = copied.x;
    out[16] = copies[0].x;
    out[17] = copies[1].z;
    int counter = seed;
    int ordered[3]{counter++, counter++, counter};
    out[18] = ordered[0] * 100 + ordered[1] * 10 + ordered[2];
    out[19] = counter;
    int called{pair(counter++, 4)};
    out[20] = called;
    int zero_total{0};
    for (int iteration{0}; iteration < 3; ++iteration) {
        int reset[3]{iteration};
        zero_total += reset[2];
        reset[2] = 9;
    }
    out[21] = zero_total;
    int total{0};
    for (int iteration{0}, limit{3}; iteration < limit; ++iteration) {
        total += iteration;
    }
    out[22] = total;
    Number Number{4}, cached[2]{Number, Number + 1};
    out[23] = cached[1];
    const int frozen[3]{3};
    out[24] = frozen[0] + frozen[1] + frozen[2];
    float3 floating{1.5f, -0.0f};
    out[25] = __float_as_int(floating.y);
    out[26] = __float_as_int(floating.z);
    int3 from_call{make_list(6)};
    out[27] = from_call.x + from_call.z;
    out[28] = int(Scale * 10.0f);
    out[29] = list_sum(seed + 3);
}
