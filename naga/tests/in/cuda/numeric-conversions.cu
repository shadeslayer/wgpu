__global__ void convert(const float *input, float *output, unsigned int n) {
int i = blockIdx.x * blockDim.x + threadIdx.x;
if (i < n) {
float offset = static_cast<float>(i);
output[i] = input[i] + offset;
}
}