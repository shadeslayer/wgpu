struct Record { int x; int values[2]; };
__device__ void combine(int *first, const int *second) { first[0] = 5; first[1] += second[0]; first[0] += second[1]; }
__device__ int back(const int *value) { return value[-1] + value[-2] * 10; }
__device__ void add(int *value, int amount) { value[0] += amount; }
__device__ void pointer_step(int *value) { *(value + 0) += 1; 1[value] += 7; }
__device__ void reference_step(int &value) { pointer_step(+&value); }
__device__ void nested(int *value) { reference_step(*(value - 0)); }
__device__ void initialize(int *value) { value[0] = 23; }
__device__ void records(Record *value) { value[1].x += value[0].values[1]; value[0].values[0] += value[1].x; }
__device__ void vectors(int2 *value, const int2 *view) { value[true].x += view[false].y; value[0].y += value[1].x; }
__device__ void floating(float *value, const float *view) { value[0] += view[1]; value[1] *= 2.0f; }
__device__ void flags(bool *value) { value[0] = value[1]; value[1] = !value[0]; }
__device__ int rank(int *value) { return 10; }
__device__ int rank(const int *value) { return 20; }
__global__ void pointer_parameters(unsigned *out) {
    int values[6]{1, 2, 3, 4, 5, 6}; combine(&values[1], &values[1]);
    out[0] = unsigned(values[0]); out[1] = unsigned(values[1]); out[2] = unsigned(values[2]); out[3] = unsigned(values[3]);
    out[4] = unsigned(back(&values[3])); out[5] = unsigned(back(&values[6]));
    nested(&values[2]); out[6] = unsigned(values[2]); out[7] = unsigned(values[3]);
    int &saved = values[3]; pointer_step(&saved); out[8] = unsigned(values[3]); out[9] = unsigned(values[4]);
    int fresh; initialize(&fresh); out[10] = unsigned(fresh);
    int single = 3; add(&single, 2); out[11] = unsigned(single);
    Record entries[2]{{4, {5, 6}}, {7, {8, 9}}}; records(entries);
    out[12] = unsigned(entries[0].x); out[13] = unsigned(entries[0].values[0]); out[14] = unsigned(entries[0].values[1]);
    out[15] = unsigned(entries[1].x); out[16] = unsigned(entries[1].values[0]); out[17] = unsigned(entries[1].values[1]);
    int2 pairs[2]{{2, 3}, {5, 7}}; vectors(pairs, pairs);
    out[18] = unsigned(pairs[0].x); out[19] = unsigned(pairs[0].y); out[20] = unsigned(pairs[1].x); out[21] = unsigned(pairs[1].y);
    initialize(&entries[1].values[1]); out[22] = unsigned(entries[1].values[1]); add(&pairs[0].x, 3); out[23] = unsigned(pairs[0].x);
    float samples[3]{1.5f, 2.0f, 4.0f}; floating(&samples[1], samples);
    out[24] = __float_as_uint(samples[0]); out[25] = __float_as_uint(samples[1]); out[26] = __float_as_uint(samples[2]);
    bool enabled[2]{false, true}; flags(enabled); out[27] = unsigned(enabled[0]); out[28] = unsigned(enabled[1]);
    const int fixed[2]{3, 4}; out[29] = unsigned(rank(values)); out[30] = unsigned(rank(fixed)); out[31] = unsigned(back(2 + values));
}
