__global__ void integer_ops(unsigned int *out, int value, unsigned int amount) {
    int remainder = value % -3;
    unsigned int bits = unsigned(remainder) & 15u;
    bits |= 8u;
    bits ^= 1u;
    bits <<= amount;
    bits >>= 1u;
    bits %= 97u;
    bits &= 255u;
    out[threadIdx.x] = ~bits;
}