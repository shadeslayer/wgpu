using Real = float;
using Whole = int;
using Natural = unsigned int;

__device__ Real min(Real first, Real second, Real third) {
    return fminf(fminf(first, second), third);
}

__device__ Real bounded(Real value) {
    return max(Real(-2), min(value, Real(4)));
}

__global__ void float_minmax(float *out) {
    Real seed = out[0];
    out[0] = min(seed, -7.5f);
    out[1] = max(seed, -7.5f);
    out[2] = fminf(seed, 1.5f);
    out[3] = fmaxf(-2.25f, seed);
    out[4] = fminf(Whole(-3), Natural(5u));
    out[5] = fmaxf(Natural(5u), Whole(-3));
    out[6] = fminf(true, false);
    out[7] = fmaxf(false, true);
    out[8] = min(max(seed, -4.0f), 2.5f);
    out[9] = min(seed, 1.25f, -3.5f);
    out[10] = bounded(seed);
    Real first = 1.25f;
    Real second = 2.5f;
    out[11] = fmaxf(first++, second++);
    out[12] = first;
    out[13] = second;
    Real value = seed;
    out[14] = fmaxf(value, value++);
    out[15] = value;
    out[16] = fmaxf(Whole(16'777'217), 16'777'216.0f);
    out[17] = fminf(Natural(0xffff'ffffu), 4'294'967'296.0f);
    out[18] = max(Real(true), seed);
    out[19] = min(seed, seed);
}
