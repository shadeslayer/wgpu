using Sample = const float;
using Bits = unsigned;

__device__ Sample sine_value(Sample value) {
    return sinf(value);
}

__host__ __device__ Sample cosine_value(Sample value) {
    return cosf(value);
}

__device__ int sinf(bool value) {
    return value ? 17 : 19;
}

__device__ int cosf(int value) {
    return value + 23;
}

__device__ float sinf(float first, float second) {
    return first + second;
}

__device__ float cosf(float2 value) {
    return value.x + value.y;
}

__device__ float identity(float value) {
    return value;
}

__device__ float pair(float first, float second) {
    return first * 10.0f + second;
}

__global__ void trigonometry(Bits *out) {
    out[0] = __float_as_uint(sinf(0.0f));
    out[1] = __float_as_uint(sinf(-0.0f));
    out[2] = __float_as_uint(cosf(0.0f));
    out[3] = __float_as_uint(cosf(-0.0f));
    out[4] = __float_as_uint(sine_value(0));
    out[5] = __float_as_uint(cosine_value(0u));
    out[6] = unsigned(sinf(true));
    out[7] = unsigned(sinf(false));
    out[8] = unsigned(cosf(4));
    out[9] = __float_as_uint(cosf(0.0f));
    out[10] = __float_as_uint(sinf(2.0f, 3.0f));
    out[11] = __float_as_uint(cosf(make_float2(2.0f, 3.0f)));
    float value = 0.0f;
    out[12] = __float_as_uint(sinf(value++));
    out[13] = __float_as_uint(value);
    value = -1.0f;
    out[14] = __float_as_uint(cosf(++value));
    out[15] = __float_as_uint(value);
    out[16] = __float_as_uint(pair(sinf(identity(value++)), value++));
    out[17] = __float_as_uint(value);
    bool choose = false;
    int counter = 0;
    out[18] = unsigned(choose && sinf(float(counter++)));
    out[19] = unsigned(true || cosf(float(counter++)));
    out[20] = unsigned(choose ? sinf(float(++counter)) : cosf(float(counter)));
    out[21] = unsigned(counter);
    float values[]{sinf(0.0f), cosf(0.0f)};
    out[22] = __float_as_uint(values[0] + values[1]);
    constexpr int Count = (values[1], 2);
    float scratch[Count]{};
    out[23] = __float_as_uint(scratch[1]);
    int repeated = 0;
    for (int index = 0; index < 3; ++index) {
        if (index == 0) continue;
        repeated += int(cosf(0.0f));
    }
    out[24] = unsigned(repeated);
    value = 0.0f;
    out[25] = __float_as_uint((sinf(value++), cosf(--value)));
    out[26] = __float_as_uint(value);
    out[27] = unsigned(true ? cosf(0.0f) : sinf(99.0f));
    out[28] = __float_as_uint(cosine_value(false));
}
