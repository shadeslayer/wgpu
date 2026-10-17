__device__ int shifted(int value) {
    return value + 10;
}

__device__ void discard(int value) {
    value += 1;
}

__device__ int pair(int x, int y) {
    return x * 10 + y;
}

__global__ void conditionals(int *out, const int *input_a, const int *input_b, unsigned int n) {
    __shared__ int shared_values[64];
    unsigned int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        unsigned int base = i * 25u;
        bool choose_a = i % 2u == 0u;
        out[base] = choose_a ? input_a[i] : input_b[i];

        int a = 3;
        int b = 5;
        out[base + 1u] = choose_a ? shifted(a++) : shifted(b++);
        out[base + 2u] = a * 10 + b;
        choose_a ? discard(a++) : discard(b++);
        out[base + 3u] = a * 10 + b;

        int index_a = 0;
        int index_b = 0;
        choose_a ? input_a[index_a++] : input_b[index_b++];
        out[base + 4u] = index_a * 10 + index_b;

        a = 3;
        b = 5;
        (choose_a ? a : b) = 7;
        out[base + 5u] = a * 10 + b;
        (choose_a ? a : b) += 2;
        out[base + 6u] = a * 10 + b;
        out[base + 7u] = ++(choose_a ? a : b);
        out[base + 8u] = (choose_a ? a : b)++;
        out[base + 9u] = a * 100 + b;

        int c = int(choose_a);
        b = 7;
        out[base + 10u] = ((c ? c : b) = 0)++;
        out[base + 11u] = c * 10 + b;

        int condition = 0;
        a = 2;
        b = 4;
        (condition ? a : b) = (condition = 1);
        out[base + 12u] = a * 10 + b;

        a = 2;
        b = 4;
        c = 6;
        (choose_a ? (i % 4u == 0u ? a : b) : c) += 3;
        out[base + 13u] = a * 100 + b * 10 + c;

        unsigned int lane = threadIdx.x;
        shared_values[lane] = 20;
        int local_value = 10;
        (choose_a ? local_value : shared_values[lane]) += 5;
        out[base + 14u] = local_value * 100 + shared_values[lane];
        (choose_a ? shared_values[lane] : out[base + 15u]) = 30;
        out[base + 16u] = shared_values[lane];

        out[base + 17u] = choose_a ? 1.5f : 3;
        out[base + 18u] = choose_a ? -1 : 2u;
        out[base + 19u] = choose_a ? true : false;
        out[base + 20u] = choose_a ? true : 2;
        out[base + 21u] = choose_a ? 4u : 8u;

        a = 3;
        int shift_value = 2;
        out[base + 22u] = (choose_a ? shift_value : a) << (shift_value = 1);

        int value = 3;
        int other = 7;
        out[base + 23u] = pair(choose_a ? value : other, value++);

        int count = 0;
        int left = 0;
        int right = 0;
        for (; count < 3; choose_a ? ++left : ++right) {
            ++count;
            if (count == 2) {
                continue;
            }
        }
        out[base + 24u] = left * 100 + right * 10 + count;
    }
}
