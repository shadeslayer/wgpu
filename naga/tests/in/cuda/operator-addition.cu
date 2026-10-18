__device__ float2 operator+(float2 left, float2 right) {
    return make_float2(left.x + right.x, left.y + right.y);
}

inline __host__ __device__ float2 operator+(float2 left, float right) {
    return make_float2(left.x + right, left.y + right);
}

static __device__ float2 operator+(float left, float2 right) {
    return make_float2(left + right.x, left + right.y);
}

__device__ float3 operator+(float3 left, float3 right) {
    return make_float3(left.x + right.x, left.y + right.y, left.z + right.z);
}

inline __host__ __device__ float3 operator+(float3 left, float right) {
    return make_float3(left.x + right, left.y + right, left.z + right);
}

static __device__ float3 operator+(float left, float3 right) {
    return make_float3(left + right.x, left + right.y, left + right.z);
}

__device__ float4 operator+(float4 left, float4 right) {
    return make_float4(left.x + right.x, left.y + right.y, left.z + right.z, left.w + right.w);
}

inline __host__ __device__ float4 operator+(float4 left, float right) {
    return make_float4(left.x + right, left.y + right, left.z + right, left.w + right);
}

static __device__ float4 operator+(float left, float4 right) {
    return make_float4(left + right.x, left + right.y, left + right.z, left + right.w);
}

__device__ int2 operator+(int2 left, int2 right) {
    return make_int2(left.x + right.x, left.y + right.y);
}

inline __host__ __device__ int2 operator+(int2 left, int right) {
    return make_int2(left.x + right, left.y + right);
}

static __device__ int2 operator+(int left, int2 right) {
    return make_int2(left + right.x, left + right.y);
}

__device__ int3 operator+(int3 left, int3 right) {
    return make_int3(left.x + right.x, left.y + right.y, left.z + right.z);
}

inline __host__ __device__ int3 operator+(int3 left, int right) {
    return make_int3(left.x + right, left.y + right, left.z + right);
}

static __device__ int3 operator+(int left, int3 right) {
    return make_int3(left + right.x, left + right.y, left + right.z);
}

__device__ int4 operator+(int4 left, int4 right) {
    return make_int4(left.x + right.x, left.y + right.y, left.z + right.z, left.w + right.w);
}

inline __host__ __device__ int4 operator+(int4 left, int right) {
    return make_int4(left.x + right, left.y + right, left.z + right, left.w + right);
}

static __device__ int4 operator+(int left, int4 right) {
    return make_int4(left + right.x, left + right.y, left + right.z, left + right.w);
}

__device__ uint2 operator+(uint2 left, uint2 right) {
    return make_uint2(left.x + right.x, left.y + right.y);
}

inline __host__ __device__ uint2 operator+(uint2 left, unsigned right) {
    return make_uint2(left.x + right, left.y + right);
}

static __device__ uint2 operator+(unsigned left, uint2 right) {
    return make_uint2(left + right.x, left + right.y);
}

__device__ uint3 operator+(uint3 left, uint3 right) {
    return make_uint3(left.x + right.x, left.y + right.y, left.z + right.z);
}

inline __host__ __device__ uint3 operator+(uint3 left, unsigned right) {
    return make_uint3(left.x + right, left.y + right, left.z + right);
}

static __device__ uint3 operator+(unsigned left, uint3 right) {
    return make_uint3(left + right.x, left + right.y, left + right.z);
}

__device__ uint4 operator+(uint4 left, uint4 right) {
    return make_uint4(left.x + right.x, left.y + right.y, left.z + right.z, left.w + right.w);
}

inline __host__ __device__ uint4 operator+(uint4 left, unsigned right) {
    return make_uint4(left.x + right, left.y + right, left.z + right, left.w + right);
}

static __device__ uint4 operator+(unsigned left, uint4 right) {
    return make_uint4(left + right.x, left + right.y, left + right.z, left + right.w);
}

__global__ void additions(unsigned *out) {
    float2 float_2_left = make_float2(1.25f, -2.5f);
    float2 float_2_right = make_float2(0.5f, 1.5f);
    float2 float_2_vectors = float_2_left + float_2_right;
    float2 float_2_vector_scalar = operator+(float_2_left, 2.0f);
    float2 float_2_scalar_vector = 2.0f + float_2_right;
    out[0] = __float_as_uint(float_2_vectors.x);
    out[1] = __float_as_uint(float_2_vectors.y);
    out[2] = __float_as_uint(float_2_vector_scalar.x);
    out[3] = __float_as_uint(float_2_vector_scalar.y);
    out[4] = __float_as_uint(float_2_scalar_vector.x);
    out[5] = __float_as_uint(float_2_scalar_vector.y);
    float3 float_3_left = make_float3(1.25f, -2.5f, 3.75f);
    float3 float_3_right = make_float3(0.5f, 1.5f, -2.0f);
    float3 float_3_vectors = float_3_left + float_3_right;
    float3 float_3_vector_scalar = operator+(float_3_left, 2.0f);
    float3 float_3_scalar_vector = 2.0f + float_3_right;
    out[6] = __float_as_uint(float_3_vectors.x);
    out[7] = __float_as_uint(float_3_vectors.y);
    out[8] = __float_as_uint(float_3_vectors.z);
    out[9] = __float_as_uint(float_3_vector_scalar.x);
    out[10] = __float_as_uint(float_3_vector_scalar.y);
    out[11] = __float_as_uint(float_3_vector_scalar.z);
    out[12] = __float_as_uint(float_3_scalar_vector.x);
    out[13] = __float_as_uint(float_3_scalar_vector.y);
    out[14] = __float_as_uint(float_3_scalar_vector.z);
    float4 float_4_left = make_float4(1.25f, -2.5f, 3.75f, -4.0f);
    float4 float_4_right = make_float4(0.5f, 1.5f, -2.0f, 0.25f);
    float4 float_4_vectors = float_4_left + float_4_right;
    float4 float_4_vector_scalar = operator+(float_4_left, 2.0f);
    float4 float_4_scalar_vector = 2.0f + float_4_right;
    out[15] = __float_as_uint(float_4_vectors.x);
    out[16] = __float_as_uint(float_4_vectors.y);
    out[17] = __float_as_uint(float_4_vectors.z);
    out[18] = __float_as_uint(float_4_vectors.w);
    out[19] = __float_as_uint(float_4_vector_scalar.x);
    out[20] = __float_as_uint(float_4_vector_scalar.y);
    out[21] = __float_as_uint(float_4_vector_scalar.z);
    out[22] = __float_as_uint(float_4_vector_scalar.w);
    out[23] = __float_as_uint(float_4_scalar_vector.x);
    out[24] = __float_as_uint(float_4_scalar_vector.y);
    out[25] = __float_as_uint(float_4_scalar_vector.z);
    out[26] = __float_as_uint(float_4_scalar_vector.w);
    int2 int_2_left = make_int2(1, -2);
    int2 int_2_right = make_int2(5, 6);
    int2 int_2_vectors = int_2_left + int_2_right;
    int2 int_2_vector_scalar = operator+(int_2_left, 3);
    int2 int_2_scalar_vector = 3 + int_2_right;
    out[27] = unsigned(int_2_vectors.x);
    out[28] = unsigned(int_2_vectors.y);
    out[29] = unsigned(int_2_vector_scalar.x);
    out[30] = unsigned(int_2_vector_scalar.y);
    out[31] = unsigned(int_2_scalar_vector.x);
    out[32] = unsigned(int_2_scalar_vector.y);
    int3 int_3_left = make_int3(1, -2, 3);
    int3 int_3_right = make_int3(5, 6, -7);
    int3 int_3_vectors = int_3_left + int_3_right;
    int3 int_3_vector_scalar = operator+(int_3_left, 3);
    int3 int_3_scalar_vector = 3 + int_3_right;
    out[33] = unsigned(int_3_vectors.x);
    out[34] = unsigned(int_3_vectors.y);
    out[35] = unsigned(int_3_vectors.z);
    out[36] = unsigned(int_3_vector_scalar.x);
    out[37] = unsigned(int_3_vector_scalar.y);
    out[38] = unsigned(int_3_vector_scalar.z);
    out[39] = unsigned(int_3_scalar_vector.x);
    out[40] = unsigned(int_3_scalar_vector.y);
    out[41] = unsigned(int_3_scalar_vector.z);
    int4 int_4_left = make_int4(1, -2, 3, -4);
    int4 int_4_right = make_int4(5, 6, -7, 8);
    int4 int_4_vectors = int_4_left + int_4_right;
    int4 int_4_vector_scalar = operator+(int_4_left, 3);
    int4 int_4_scalar_vector = 3 + int_4_right;
    out[42] = unsigned(int_4_vectors.x);
    out[43] = unsigned(int_4_vectors.y);
    out[44] = unsigned(int_4_vectors.z);
    out[45] = unsigned(int_4_vectors.w);
    out[46] = unsigned(int_4_vector_scalar.x);
    out[47] = unsigned(int_4_vector_scalar.y);
    out[48] = unsigned(int_4_vector_scalar.z);
    out[49] = unsigned(int_4_vector_scalar.w);
    out[50] = unsigned(int_4_scalar_vector.x);
    out[51] = unsigned(int_4_scalar_vector.y);
    out[52] = unsigned(int_4_scalar_vector.z);
    out[53] = unsigned(int_4_scalar_vector.w);
    uint2 uint_2_left = make_uint2(0xfffffffeu, 0x80000001u);
    uint2 uint_2_right = make_uint2(5u, 2u);
    uint2 uint_2_vectors = uint_2_left + uint_2_right;
    uint2 uint_2_vector_scalar = operator+(uint_2_left, 4u);
    uint2 uint_2_scalar_vector = 4u + uint_2_right;
    out[54] = uint_2_vectors.x;
    out[55] = uint_2_vectors.y;
    out[56] = uint_2_vector_scalar.x;
    out[57] = uint_2_vector_scalar.y;
    out[58] = uint_2_scalar_vector.x;
    out[59] = uint_2_scalar_vector.y;
    uint3 uint_3_left = make_uint3(0xfffffffeu, 0x80000001u, 3u);
    uint3 uint_3_right = make_uint3(5u, 2u, 0xfffffffdu);
    uint3 uint_3_vectors = uint_3_left + uint_3_right;
    uint3 uint_3_vector_scalar = operator+(uint_3_left, 4u);
    uint3 uint_3_scalar_vector = 4u + uint_3_right;
    out[60] = uint_3_vectors.x;
    out[61] = uint_3_vectors.y;
    out[62] = uint_3_vectors.z;
    out[63] = uint_3_vector_scalar.x;
    out[64] = uint_3_vector_scalar.y;
    out[65] = uint_3_vector_scalar.z;
    out[66] = uint_3_scalar_vector.x;
    out[67] = uint_3_scalar_vector.y;
    out[68] = uint_3_scalar_vector.z;
    uint4 uint_4_left = make_uint4(0xfffffffeu, 0x80000001u, 3u, 0xffffffffu);
    uint4 uint_4_right = make_uint4(5u, 2u, 0xfffffffdu, 1u);
    uint4 uint_4_vectors = uint_4_left + uint_4_right;
    uint4 uint_4_vector_scalar = operator+(uint_4_left, 4u);
    uint4 uint_4_scalar_vector = 4u + uint_4_right;
    out[69] = uint_4_vectors.x;
    out[70] = uint_4_vectors.y;
    out[71] = uint_4_vectors.z;
    out[72] = uint_4_vectors.w;
    out[73] = uint_4_vector_scalar.x;
    out[74] = uint_4_vector_scalar.y;
    out[75] = uint_4_vector_scalar.z;
    out[76] = uint_4_vector_scalar.w;
    out[77] = uint_4_scalar_vector.x;
    out[78] = uint_4_scalar_vector.y;
    out[79] = uint_4_scalar_vector.z;
    out[80] = uint_4_scalar_vector.w;
}
