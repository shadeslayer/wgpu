__device__ unsigned thread_component(unsigned axis) {
    if (axis == 0u) return threadIdx.x;
    if (axis == 1u) return threadIdx.y;
    return threadIdx.z;
}

__device__ unsigned group_component(unsigned axis) {
    if (axis == 0u) return blockIdx.x;
    if (axis == 1u) return blockIdx.y;
    return blockIdx.z;
}

__host__ __device__ unsigned shape_component(unsigned axis) {
    if (axis == 0u) return blockDim.x;
    if (axis == 1u) return blockDim.y;
    return blockDim.z;
}

__device__ unsigned grid_component(unsigned axis) {
    if (axis == 0u) return gridDim.x;
    if (axis == 1u) return gridDim.y;
    return gridDim.z;
}

__device__ unsigned component(unsigned axis) { return thread_component(axis); }
__device__ float component(float offset) { return float(blockIdx.x) + offset; }
__device__ unsigned global_x() {
    return group_component(0u) * shape_component(0u) + component(0u);
}
__device__ unsigned kind(unsigned value) { return 11u; }
__device__ unsigned kind(int value) { return 22u; }
__device__ void observe() { if (gridDim.z) return; }

__global__ void coordinates(const int *input, float scale, const float3 *points, unsigned limit, unsigned *out) {
    if (global_x() >= limit) return;
    observe();
    out[0] = thread_component(0u);
    out[1] = thread_component(1u);
    out[2] = thread_component(2u);
    out[3] = group_component(0u);
    out[4] = group_component(1u);
    out[5] = group_component(2u);
    out[6] = shape_component(0u);
    out[7] = shape_component(1u);
    out[8] = shape_component(2u);
    out[9] = grid_component(0u);
    out[10] = grid_component(1u);
    out[11] = grid_component(2u);
    out[12] = global_x();
    out[13] = component(0u);
    out[14] = __float_as_uint(component(2.5f));
    out[15] = threadIdx.x;
    out[16] = blockIdx.y;
    out[17] = gridDim.z;
    out[18] = blockDim.y;
    out[19] = unsigned(input[0]);
    out[20] = __float_as_uint(scale);
    out[21] = __float_as_uint(points[0].x);
    out[22] = __float_as_uint(points[0].y);
    out[23] = __float_as_uint(points[0].z);
    out[24] = limit;
    out[25] = thread_component(0u) + group_component(1u) + shape_component(2u) + grid_component(0u);
    out[26] = global_x() + global_x();
    out[27] = __float_as_uint(float(input[0]) * scale + points[0].x);
    out[28] = kind(threadIdx.x);
    out[29] = grid_component(2u);
}
