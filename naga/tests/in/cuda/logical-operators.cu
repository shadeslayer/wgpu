struct Sample { int value; int payload[2]; };
using Value = Sample;
using Fixed = const Value;

__device__ Value operator&&(Fixed, Value);
__device__ Value operator||(Value, Value);
__device__ int operator&&(Value, int);
__device__ int operator||(int, Value);
__device__ int2 operator&&(int2, int2);
__device__ int2 operator||(int2, int2);

__global__ void logical_operators(unsigned *out) {
    unsigned base = threadIdx.x * 24u;
    const Sample left{2, {3, 4}};
    Sample right{5, {6, 7}};
    Sample both = left && right;
    Sample either = left || right;
    out[base + 0u] = both.value;
    out[base + 1u] = both.payload[1];
    out[base + 2u] = either.value;
    out[base + 3u] = either.payload[0];
    out[base + 4u] = left.value;
    out[base + 5u] = left.payload[1];
    out[base + 6u] = left && 9;
    out[base + 7u] = 0 || right;
    out[base + 8u] = operator&&(left, right).value;
    out[base + 9u] = operator||(left, right).payload[0];
    int2 first = make_int2(2, 3), second = make_int2(4, 5);
    out[base + 10u] = (first && second).x;
    out[base + 11u] = (first || second).y;
    out[base + 12u] = (first && second && first).x;
    out[base + 13u] = (first || second && first).y;
    out[base + 14u] = ((first || second) && first).x;
    out[base + 15u] = (true ? left : right) && 1;
    out[base + 16u] = (true ? left && right : right || left).value;
    int counter = 0;
    Sample evaluated = false ? (Sample{counter++, {1, 2}} && right) : (left || Sample{counter++, {1, 2}});
    out[base + 17u] = evaluated.value;
    out[base + 18u] = counter;
    out[base + 19u] = false && (left && right).value;
    out[base + 20u] = true || (left || right).value;
    out[base + 21u] = 2 && 3;
    out[base + 22u] = 0 || 4;
    out[base + 23u] = threadIdx.x;
}

__device__ Sample operator&&(Sample left, Sample right) {
    left.value = left.value * 10 + right.value;
    left.payload[1] += right.payload[0];
    return left;
}
__device__ Sample operator||(Sample left, Sample right) {
    left.value = left.value * 100 + right.value;
    left.payload[0] += right.payload[1];
    return left;
}
__device__ int operator&&(Sample left, int right) { return left.value * 10 + right; }
__device__ int operator||(int left, Sample right) { return left * 100 + right.value; }
__device__ int2 operator&&(int2 left, int2 right) { return make_int2(left.x * 10 + right.x, left.y * 10 + right.y); }
__device__ int2 operator||(int2 left, int2 right) { return make_int2(left.x * 100 + right.x, left.y * 100 + right.y); }
