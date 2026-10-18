using Count = int;
constexpr Count Base = 2;
constexpr unsigned High = 1u << 31;
constexpr float Scale = 0.5f;

__device__ int accumulate(int runtime) {
    constexpr int Size = true ? Base + 1 : runtime;
    int local[Size]{};
    int total = 0;
    for (int i = 0; i < Size; ++i) {
        local[i] = runtime + i;
        total += local[i];
    }
    return total;
}

constexpr int Second = Base + 1;

__global__ void constant_expressions(const int *input, unsigned *out) {
    int runtime = input[0];
    constexpr int Chosen = true ? 3 : runtime;
    const int Size = Base + 2;
    int values[Size + 1]{};
    values[0] = runtime;
    values[Size] = input[1];
    out[0] = unsigned(Base);
    out[1] = High;
    out[2] = __float_as_uint(Scale);
    out[3] = unsigned(Chosen);
    out[4] = unsigned(accumulate(runtime));
    out[5] = unsigned(values[Size]);
    out[6] = unsigned(values[1]);
    out[7] = unsigned(float(runtime) * Scale);
    constexpr int SignedHigh = 1 << 31;
    constexpr int Negative = -1 << 1;
    constexpr unsigned All = 0u - 1u, Wrapped = 0x80000000u << 1;
    out[8] = unsigned(SignedHigh);
    out[9] = unsigned(Negative);
    out[10] = All;
    out[11] = Wrapped;
    constexpr unsigned TruncatedZero = -0.5f;
    constexpr int Minimum = -2147483648.0f;
    constexpr unsigned Large = 4294967040.0f;
    constexpr bool Ready = 2.75f;
    out[12] = TruncatedZero;
    out[13] = unsigned(Minimum);
    out[14] = Large;
    out[15] = unsigned(Ready);
    constexpr int Comma = (runtime, 4);
    constexpr int Lazy = true ? 6 : (runtime = 99);
    out[16] = unsigned(Comma);
    out[17] = unsigned(Lazy);
    out[18] = unsigned(runtime);
    constexpr bool Skipped = false && accumulate(runtime);
    constexpr bool Yes = true || runtime++;
    out[19] = unsigned(Skipped);
    out[20] = unsigned(Yes);
    out[21] = unsigned(runtime);
    constexpr int VectorCount = 2;
    float3 vectors[VectorCount]{};
    vectors[1].y = float(input[1]);
    out[22] = unsigned(vectors[1].y);
    {
        int Base[Base]{};
        out[23] = unsigned(Base[1]);
    }
    for (int i = 0; i < 3; ++i) {
        constexpr int Count = 2 + 1;
        int scratch[Count]{};
        out[24 + i] = unsigned(scratch[2]);
        scratch[2] = runtime + i;
    }
    out[27] = unsigned(accumulate(input[1]));
    out[28] = unsigned(Second);
    const int Safe = true ? 3 : (1 / 0);
    out[29] = unsigned(runtime + input[1] + Safe - 3);
}

constexpr int After = Second + 1;
