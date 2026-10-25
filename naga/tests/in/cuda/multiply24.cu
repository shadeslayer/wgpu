using Word = unsigned;
typedef const int Signed;

__device__ int __mul24(const int, int);
__device__ Word __umul24(Word, const Word);

__device__ int signed_conversion(float first, float second) {
    return __mul24(first, second);
}

__device__ Word unsigned_conversion(float first, float second) {
    return __umul24(first, second);
}

__device__ int __mul24(bool first, bool second) {
    return first ? (second ? 71 : 73) : 79;
}

__device__ int __mul24(float first, float second) {
    return int(first * 10.0f + second);
}

__device__ Word __umul24(int first, int second) {
    return Word(first + second + 100);
}

__device__ int signed_pair(int first, int second);

__global__ void multiply24(Word *out) {
    out[0] = Word(__mul24(0x00800000, 1));
    out[1] = __umul24(0x00ffffffu, 0x00ffffffu);
    out[2] = Word(__mul24(0x007fffff, 0x007fffff));
    out[3] = __umul24(0x01000000u, 13u);
    out[4] = Word(signed_conversion(3.75f, -2.75f));
    out[5] = Word(signed_conversion(-3.75f, 2.75f));
    out[6] = unsigned_conversion(3.75f, 2.75f);
    out[7] = unsigned_conversion(16777215.0f, 1.0f);
    out[8] = Word(__mul24(true, true));
    out[9] = Word(__mul24(false, true));
    out[10] = Word(__mul24(3, int(true)));
    out[11] = Word(__mul24(int(true), 3));
    out[12] = Word(__mul24(3.75f, 2.0f));
    out[13] = __umul24(3, 4);
    out[14] = __umul24(3u, 4u);
    out[15] = __umul24(true, true);
    out[16] = __umul24(3u, Word(4));
    out[17] = Word(signed_pair(-7, 3));
    out[18] = Word(__mul24(__mul24(3, 4), 5));
    out[19] = __umul24(__umul24(0x00ffffffu, 2u), 2u);
    Signed first = -3;
    Word second = 5u;
    out[20] = Word(__mul24(first, int(second)));
    out[21] = __umul24(Word(first), second);
    out[22] = Word(__mul24(0x01800000, 0x01000001));
    out[23] = __umul24(0xff800000u, 0xff000001u);
    out[24] = Word(true ? __mul24(-3, 7) : __mul24(0, 0));
    int values[] = {__mul24(2, 3), __mul24(3, 4)};
    out[25] = Word(values[0] + values[1]);
    constexpr int Count = true ? 2 : __mul24(1, 1);
    int scratch[Count] = {};
    out[26] = Word(Count + scratch[0]);
    int counter = 2;
    out[27] = Word(__mul24(counter++, counter++));
    out[28] = Word(counter);
    int total = 0;
    for (int index = 0; index < 3; ++index) {
        total += __mul24(index + 1, 2);
    }
    out[29] = Word(total);
    {
        using __mul24 = int;
        out[30] = Word(__mul24(9.75f));
    }
    out[31] = Word(__mul24(0x00ffffff, 3));
}

__device__ int signed_pair(int first, int second) {
    return __mul24(first, second);
}
