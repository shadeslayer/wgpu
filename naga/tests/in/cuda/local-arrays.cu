using Element = unsigned int;

__device__ int local_sum(int value) {
    int values[3] = {};
    values[0] = value;
    values[1] = value + 1;
    values[2] = values[0] + values[1];
    return values[2];
}

__device__ int reset_sum(int count) {
    int result = 0;
    for (int i = 0; i < count; ++i) {
        int values[2]{};
        values[1] = i;
        result += values[0] + values[1];
        values[0] = 99;
    }
    return result;
}

__global__ void local_arrays(int *out) {
    int seed = out[0];
    int signed_values[3];
    signed_values[0] = seed;
    signed_values[1] = -2;
    signed_values[2] = 4;
    out[0] = signed_values[0] + signed_values[1] + signed_values[2];
    unsigned int words[0x2] = {};
    words[0] = 0xffff'ffffu;
    words[1] = 1u;
    out[1] = words[0] + words[1] == 0u;
    float samples[0b10]{};
    samples[0] = 1.5f;
    samples[1] = 2.25f;
    out[2] = int((samples[0] + samples[1]) * 4.0f);
    bool flags[03]{};
    flags[0] = seed != 0;
    flags[2] = true;
    out[3] = int(flags[0]) * 100 + int(flags[1]) * 10 + int(flags[2]);
    const int zeros[2]{};
    out[4] = zeros[0] + zeros[1];
    out[5] = local_sum(seed);
    out[6] = reset_sum(3);
    const bool zero_flags[2] = {};
    out[7] = zero_flags[0] || zero_flags[1];
    Element bank[2] = {};
    bank[0] = 3u;
    bank[1] = 4u;
    {
        using Element = float;
        Element bank[2]{};
        bank[1] = 6.5f;
        out[8] = int(bank[0] + bank[1]);
    }
    out[9] = int(bank[0] + bank[1]);
    {
        int signed_values[2];
        signed_values[0] = seed;
        signed_values[1] = seed;
        out[10] = signed_values[0] + signed_values[1];
    }
    out[11] = signed_values[0];
    int mixed[2] = {}, next = ++seed;
    mixed[0] = next;
    mixed[1] = seed++;
    out[12] = mixed[0] * 10 + mixed[1];
    out[13] = seed;
    int reset_total = 0;
    for (int i = 0; i < 3; ++i) {
        int reset[2] = {};
        if (i == 1) {
            reset[0] = 90;
            continue;
        }
        reset_total += reset[0] + i;
        reset[0] = 99;
    }
    out[14] = reset_total;
    int total = 0;
    for (int scratch[2] = {}, j = 0; j < 3; ++j) {
        scratch[j % 2] += 1;
        if (j == 1) continue;
        total += scratch[0] * 10 + scratch[1];
    }
    out[15] = total;
    const float zero_samples[2] = {};
    out[16] = int(zero_samples[0] + zero_samples[1]);
    int left[2]{}, right[2] = {};
    left[0] = 5;
    left[1] = 7;
    right[0] = 11;
    right[1] = 13;
    out[17] = left[0] + right[0];
    {
        int temporary[2]{};
        temporary[0] = seed;
        {
            int temporary[2]{};
            temporary[1] = 3;
            out[18] = temporary[0] + temporary[1];
        }
        out[19] = temporary[0];
    }
    bool choose = (seed & 1) == 0;
    int index = 0;
    int rhs = 20;
    out[20] = ((choose ? left[index++] : right[index++]) += ++rhs);
    out[21] = index;
    out[22] = left[0] * 100 + right[0];
    out[23] = ((choose ? right[1] : left[1]) = 9)++;
    out[24] = left[1] * 100 + right[1];
}
