__global__ void scale(float factor, const float *input, float *output) {
factor = factor * 2.0f;
output[threadIdx.x] = input[threadIdx.x] * factor;
}