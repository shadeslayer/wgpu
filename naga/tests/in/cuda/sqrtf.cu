using Sample = const float;
using Bits = unsigned;

__device__ Sample root(Sample value) {
    return sqrtf(value);
}

__device__ float root(int value) {
    return sqrtf(value);
}

__device__ float root(unsigned value) {
    return sqrtf(value);
}

__device__ float root(bool value) {
    return sqrtf(value);
}

__device__ int sqrtf(bool value) {
    return value ? 7 : 9;
}

__device__ float sqrtf(float first, float second) {
    return first + second;
}

__global__ void roots(Bits *out) {
    out[0] = __float_as_uint(sqrtf(0.0f));
    out[1] = __float_as_uint(sqrtf(-0.0f));
    out[2] = __float_as_uint(sqrtf(1.0f));
    out[3] = __float_as_uint(root(4));
    out[4] = __float_as_uint(root(9u));
    out[5] = __float_as_uint(sqrtf(6.25f));
    out[6] = __float_as_uint(sqrtf(2.0f));
    out[7] = __float_as_uint(sqrtf(__uint_as_float(1u)));
    out[8] = __float_as_uint(sqrtf(__uint_as_float(0x007fffffu)));
    out[9] = __float_as_uint(sqrtf(__uint_as_float(0x00800000u)));
    out[10] = __float_as_uint(sqrtf(__uint_as_float(0x7f7fffffu)));
    out[11] = __float_as_uint(sqrtf(__uint_as_float(0x7f800000u)));
    out[12] = __float_as_uint(sqrtf(-1.0f));
    out[13] = __float_as_uint(sqrtf(__uint_as_float(0x7f800001u)));
    float value = 4.0f;
    out[14] = __float_as_uint(root(value++));
    out[15] = __float_as_uint(value);
    out[16] = unsigned(sqrtf(true));
    out[17] = unsigned(sqrtf(false));
    out[18] = __float_as_uint(sqrtf(2.0f, 3.0f));
    out[19] = __float_as_uint(sqrtf(__uint_as_float(0x3f800001u)));
    out[20] = __float_as_uint(sqrtf(__uint_as_float(0x3f800002u)));
    out[21] = __float_as_uint(sqrtf(__uint_as_float(0x3f7fffffu)));
    out[22] = __float_as_uint(root(0xffffffffu));
    out[23] = __float_as_uint(root(-4));
    out[24] = __float_as_uint(root(16.0f));
    out[25] = __float_as_uint(sqrtf(0.25f));
    out[26] = __float_as_uint(sqrtf(__uint_as_float(0x80000001u)));
    out[27] = __float_as_uint(root(false));
    out[28] = __float_as_uint(root(true));
    out[29] = __float_as_uint(root(0.5f));
}
