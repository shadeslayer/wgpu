using Sample = const float;
using Bits = unsigned;

__device__ Sample floor_value(Sample value) {
    return floorf(value);
}

__device__ float floor_value(int value) {
    return floorf(value);
}

__device__ float floor_value(unsigned value) {
    return floorf(value);
}

__device__ float floor_value(bool value) {
    return floorf(value);
}

__device__ int floorf(bool value) {
    return value ? 7 : 9;
}

__device__ float floorf(float first, float second) {
    return first + second;
}

__global__ void floors(Bits *out) {
    out[0] = __float_as_uint(floorf(0.0f));
    out[1] = __float_as_uint(floorf(-0.0f));
    out[2] = __float_as_uint(floorf(1.75f));
    out[3] = __float_as_uint(floorf(-1.25f));
    out[4] = __float_as_uint(floor_value(3));
    out[5] = __float_as_uint(floor_value(5u));
    out[6] = __float_as_uint(floor_value(true));
    out[7] = __float_as_uint(floor_value(false));
    out[8] = __float_as_uint(floorf(__uint_as_float(1u)));
    out[9] = __float_as_uint(floorf(__uint_as_float(0x80000001u)));
    out[10] = __float_as_uint(floorf(__uint_as_float(0x007fffffu)));
    out[11] = __float_as_uint(floorf(__uint_as_float(0x807fffffu)));
    out[12] = __float_as_uint(floorf(3.402823466e38f));
    out[13] = __float_as_uint(floorf(-3.402823466e38f));
    out[14] = __float_as_uint(floorf(__uint_as_float(0x7f7fffffu)));
    out[15] = __float_as_uint(floorf(__uint_as_float(0xff7fffffu)));
    out[16] = __float_as_uint(floorf(__uint_as_float(0x7f800000u)));
    out[17] = __float_as_uint(floorf(__uint_as_float(0xff800000u)));
    out[18] = __float_as_uint(floorf(__uint_as_float(0x7fc12345u)));
    out[19] = __float_as_uint(floorf(__uint_as_float(0x7f800001u)));
    float value = 2.75f;
    out[20] = __float_as_uint(floor_value(value++));
    out[21] = __float_as_uint(value);
    out[22] = unsigned(floorf(true));
    out[23] = unsigned(floorf(false));
    out[24] = __float_as_uint(floorf(2.0f, 3.0f));
    out[25] = __float_as_uint(floorf(__uint_as_float(0x4affffffu)));
    out[26] = __float_as_uint(floorf(__uint_as_float(0xcaffffffu)));
    out[27] = __float_as_uint(floor_value(0xffffffffu));
    out[28] = __float_as_uint(floor_value(-2147483647 - 1));
    out[29] = __float_as_uint(floorf(1e10f));
}
