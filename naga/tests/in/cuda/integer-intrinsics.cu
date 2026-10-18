using Bits = unsigned;
using Signed = const int;

__device__ int count(Bits value) {
    return __popc(value);
}

__host__ __device__ int scan(Signed value) {
    return __ffs(value);
}

__device__ int __clz(int first, int second) {
    return first + second;
}

__global__ void integer_intrinsics(Bits *out) {
    out[0] = unsigned(__clz(true));
    out[1] = unsigned(__clz(8));
    out[2] = unsigned(__clz(0x80000000u));
    out[3] = unsigned(__clz(8.75f));
    out[4] = unsigned(__ffs(true));
    out[5] = unsigned(__ffs(8));
    out[6] = unsigned(__ffs(0x80000000u));
    out[7] = unsigned(__ffs(8.75f));
    out[8] = unsigned(__popc(true));
    out[9] = unsigned(__popc(-1));
    out[10] = unsigned(__popc(0x80000001u));
    out[11] = unsigned(__popc(3.75f));
    out[12] = __brev(true);
    out[13] = __brev(-1);
    out[14] = __brev(0x01234567u);
    out[15] = __brev(2.75f);
    out[16] = unsigned(count(0x80000001u));
    out[17] = unsigned(scan(int(0x80000000u)));
    out[18] = unsigned(__clz(0));
    out[19] = unsigned(__ffs(0));
    out[20] = unsigned(__popc(0xffffffffu));
    out[21] = __brev(0u);
    out[22] = __brev(1u);
    out[23] = __brev(0x80000000u);
    out[24] = unsigned(__clz(-2));
    out[25] = unsigned(__ffs(-2));
    out[26] = unsigned(__popc(0xaaaaaaaau));
    out[27] = __brev(0xaaaaaaaau);
    out[28] = unsigned(__clz(3, 4));
    out[29] = unsigned((__popc(3u) - 3) < 0);
}
