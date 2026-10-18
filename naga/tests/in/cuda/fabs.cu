using Sample = const float;
using Bits = unsigned;

inline __host__ __device__ float2 fabs(float2 value)
{
    return make_float2(fabs(value.x), fabs(value.y));
}
inline __host__ __device__ float3 fabs(float3 value)
{
    return make_float3(fabs(value.x), fabs(value.y), fabs(value.z));
}
inline __host__ __device__ float4 fabs(float4 value)
{
    return make_float4(fabs(value.x), fabs(value.y), fabs(value.z), fabs(value.w));
}

__device__ Sample absolute(Sample value) {
    return fabs(value);
}

__device__ float converted_absolute(int value) {
    return fabsf(value);
}

__device__ float converted_absolute(unsigned value) {
    return fabsf(value);
}

__device__ float converted_absolute(bool value) {
    return fabsf(value);
}

__device__ int fabs(int value) {
    return value + 40;
}

__device__ unsigned fabs(unsigned value) {
    return value + 50u;
}

__device__ int fabs(bool value) {
    return value ? 7 : 9;
}

__device__ int fabsf(bool value) {
    return value ? 11 : 13;
}

__device__ float fabsf(float first, float second) {
    return first + second;
}

__global__ void absolutes(Bits *out) {
    out[0] = __float_as_uint(fabsf(0.0f));
    out[1] = __float_as_uint(fabs(-0.0f));
    out[2] = __float_as_uint(fabsf(-1.75f));
    out[3] = __float_as_uint(absolute(-1.25f));
    out[4] = __float_as_uint(converted_absolute(-3));
    out[5] = __float_as_uint(converted_absolute(0xffffffffu));
    out[6] = __float_as_uint(converted_absolute(true));
    out[7] = __float_as_uint(converted_absolute(false));
    out[8] = __float_as_uint(fabsf(__uint_as_float(1u)));
    out[9] = __float_as_uint(fabs(__uint_as_float(0x80000001u)));
    out[10] = __float_as_uint(fabsf(__uint_as_float(0x007fffffu)));
    out[11] = __float_as_uint(fabs(__uint_as_float(0x807fffffu)));
    out[12] = __float_as_uint(fabsf(3.402823466e38f));
    out[13] = __float_as_uint(fabs(-3.402823466e38f));
    out[14] = __float_as_uint(fabsf(__uint_as_float(0x7f800000u)));
    out[15] = __float_as_uint(fabs(__uint_as_float(0xff800000u)));
    out[16] = __float_as_uint(fabsf(__uint_as_float(0x7fc12345u)));
    out[17] = __float_as_uint(fabs(__uint_as_float(0x7f800001u)));
    float value = -2.75f;
    out[18] = __float_as_uint(fabsf(value++));
    out[19] = __float_as_uint(value);
    out[20] = unsigned(fabs(-3));
    out[21] = fabs(3u);
    out[22] = unsigned(fabs(true));
    out[23] = unsigned(fabs(false));
    out[24] = unsigned(fabsf(true));
    float3 triple = fabs(make_float3(-2.5f, -0.0f, __uint_as_float(0x80000001u)));
    out[25] = __float_as_uint(triple.x);
    out[26] = __float_as_uint(triple.y);
    out[27] = __float_as_uint(triple.z);
    out[28] = __float_as_uint(fabsf(-2.0f, 3.0f));
    float2 pair = fabs(make_float2(-3.0f, -4.0f));
    float4 quadruple = fabs(make_float4(-1.0f, -2.0f, -3.0f, -4.0f));
    out[29] = __float_as_uint(pair.x + pair.y + quadruple.x + quadruple.w);
}
