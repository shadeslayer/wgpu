struct Record { int value; int history[2]; };
using Sample = int2;
using Fixed = const Sample;

__device__ Sample operator+(Fixed);
__device__ Sample operator-(Fixed);
__device__ Sample operator~(Fixed);
__device__ bool operator!(Fixed);
__device__ int operator*(Fixed);
__device__ Sample operator&(Fixed);
__device__ Sample operator+(Sample, Sample);
__device__ Sample operator-(Sample, Sample);
__device__ Sample operator*(Sample, Sample);
__device__ Sample operator/(Sample, Sample);
__device__ Sample operator%(Sample, Sample);
__device__ Sample operator&(Sample, Sample);
__device__ Sample operator|(Sample, Sample);
__device__ Sample operator^(Sample, Sample);
__device__ Sample operator<<(Sample, Sample);
__device__ Sample operator>>(Sample, Sample);
__device__ Record operator*(Record);
__device__ int operator&(Record);
__device__ Record operator-(Record);
__device__ Record operator*(Record, int);

__global__ void operator_functions(const Sample *input, unsigned *out) {
    unsigned base = threadIdx.x * 36u;
    const Sample left = input[threadIdx.x];
    Sample right = make_int2(3, 2);
    out[base + 0u] = (+left).x;
    out[base + 1u] = (-left).y;
    out[base + 2u] = (~left).x;
    out[base + 3u] = !left;
    out[base + 4u] = *left;
    out[base + 5u] = (&left).x;
    out[base + 6u] = (left + right).x;
    out[base + 7u] = (left - right).y;
    out[base + 8u] = (left * right).x;
    out[base + 9u] = (left / right).y;
    out[base + 10u] = (left % right).y;
    out[base + 11u] = (left & right).x;
    out[base + 12u] = (left | right).y;
    out[base + 13u] = (left ^ right).x;
    out[base + 14u] = (left << right).x;
    out[base + 15u] = (left >> right).y;
    out[base + 16u] = operator+(left).y;
    out[base + 17u] = operator-(left, right).x;
    out[base + 18u] = operator*(left);
    out[base + 19u] = operator&(left, right).y;
    out[base + 20u] = *(&left);
    out[base + 21u] = (&(&left)).x;
    out[base + 22u] = (left + right * right).x;
    out[base + 23u] = (left << right + make_int2(1, 1)).x;
    out[base + 24u] = (left | right & make_int2(1, 1)).y;
    out[base + 25u] = (left - right - right).x;
    Record original{9, {1, 2}};
    Record scaled = original * 3;
    out[base + 26u] = &Record{9, {1, 2}};
    out[base + 27u] = &*original;
    out[base + 28u] = scaled.value;
    out[base + 29u] = scaled.history[1];
    out[base + 30u] = original.value;
    out[base + 31u] = original.history[1];
    out[base + 32u] = (-original).value;
    out[base + 33u] = operator*(original, 2).value;
    out[base + 34u] = operator&(operator*(original));
    out[base + 35u] = operator!(make_int2(0, 0));
}

__device__ Sample operator+(Fixed value) { return make_int2(+value.x, +value.y); }
__device__ Sample operator-(Fixed value) { return make_int2(-value.x, -value.y); }
__device__ Sample operator~(Fixed value) { return make_int2(~value.x, ~value.y); }
__device__ bool operator!(Fixed value) { return value.x == 0 && value.y == 0; }
__device__ int operator*(Fixed value) { return value.x * 10 + value.y; }
__device__ Sample operator&(Fixed value) { return make_int2(value.y, value.x); }
__device__ Sample operator+(Sample left, Sample right) { return make_int2(left.x + right.x, left.y + right.y); }
__device__ Sample operator-(Sample left, Sample right) { return make_int2(left.x - right.x, left.y - right.y); }
__device__ Sample operator*(Sample left, Sample right) { return make_int2(left.x * right.x, left.y * right.y); }
__device__ Sample operator/(Sample left, Sample right) { return make_int2(left.x / right.x, left.y / right.y); }
__device__ Sample operator%(Sample left, Sample right) { return make_int2(left.x % right.x, left.y % right.y); }
__device__ Sample operator&(Sample left, Sample right) { return make_int2(left.x & right.x, left.y & right.y); }
__device__ Sample operator|(Sample left, Sample right) { return make_int2(left.x | right.x, left.y | right.y); }
__device__ Sample operator^(Sample left, Sample right) { return make_int2(left.x ^ right.x, left.y ^ right.y); }
__device__ Sample operator<<(Sample left, Sample right) { return make_int2(left.x << right.x, left.y << right.y); }
__device__ Sample operator>>(Sample left, Sample right) { return make_int2(left.x >> right.x, left.y >> right.y); }
__device__ Record operator*(Record value) { ++value.value; return value; }
__device__ int operator&(Record value) { return value.value; }
__device__ Record operator-(Record value) { value.value = -value.value; return value; }
__device__ Record operator*(Record value, int factor) { value.value *= factor; value.history[1] += factor; return value; }
