__global__ void updates(int *out, int value) {
int previous = value++;
out[threadIdx.x] += previous;
out[threadIdx.x] = ++value;
}