struct Packet {
    int key;
    int values[2];
};

__device__ int route(float);
__device__ int earlier(int value) {
    return route(value);
}
__device__ int route(int);
__device__ int forward(int);
__device__ Packet packet(int);
__device__ Packet adjust(Packet);
__device__ Packet operator+(Packet, Packet);
__device__ int pair(int, int);
__device__ void consume(int);
__device__ float generated(float);
__device__ int control(int);
__device__ unsigned late_index();
__device__ float sqrtf(float);
__device__ float sinf(float);
__device__ float cosf(float);

__global__ void prototypes(unsigned *out) {
    unsigned lane = late_index();
    unsigned base = lane * 16u;
    int counter = 1;
    Packet first = packet(counter++);
    Packet second = adjust(first);
    Packet total = first + second;
    out[base] = earlier(3);
    out[base + 1] = route(3);
    out[base + 2] = forward(4);
    out[base + 3] = first.key;
    out[base + 4] = first.values[1];
    out[base + 5] = second.key;
    out[base + 6] = second.values[1];
    out[base + 7] = total.key;
    out[base + 8] = total.values[0];
    out[base + 9] = total.values[1];
    int first_argument = counter++;
    out[base + 10] = pair(first_argument, counter++);
    consume(counter++);
    out[base + 11] = counter;
    out[base + 12] = __float_as_uint(generated(0.0f));
    out[base + 13] = control(3);
    out[base + 14] = lane;
    out[base + 15] = (counter++, forward(counter));
}

__device__ int forward(int value) {
    return route(value) + earlier(value);
}

__device__ Packet adjust(Packet value) {
    value.key += 4;
    value.values[1] += 6;
    return value;
}

__device__ Packet operator+(Packet left, Packet right) {
    return Packet{
        left.key + right.key,
        {left.values[0] + right.values[0], left.values[1] + right.values[1]}
    };
}

__device__ int pair(int first, int second) {
    return first * 10 + second;
}

__device__ void consume(int value) {
    if (value) return;
}

__device__ float generated(float value) {
    return sqrtf(4.0f) + sinf(value) + cosf(value);
}

__device__ int control(int count) {
    int result = 0;
    for (int index = 0; index < count; ++index) {
        if (index == 1) continue;
        switch (index) {
            case 0: result += route(index); break;
            default: result += forward(index) - earlier(index); break;
        }
    }
    return result;
}

__device__ Packet packet(int value) {
    return Packet{value, {value + 1, value + 2}};
}

__device__ int route(float value) {
    return 10 + int(value);
}

__device__ int route(int value) {
    return 20 + value;
}

__device__ unsigned late_index() {
    return blockIdx.x * blockDim.x + threadIdx.x;
}
