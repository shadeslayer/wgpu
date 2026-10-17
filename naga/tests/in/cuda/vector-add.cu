extern "C" __global__ void add(const float *a, const float *b, float *out) {
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    out[i] = a[i] + b[i] * 2.0f;
}
