struct type_5 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
cbuffer n_1 : register(b1) { type_5 n_1; }

int next_value(int value_1)
{
    int value_2 = (int)0;

    value_2 = value_1;
    int _e3 = value_2;
    return asint(asuint(_e3) + asuint(int(1)));
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

void consume(bool stop, int value_3, uint count, float scale)
{
    bool stop_1 = (bool)0;
    int value_4 = (int)0;
    uint count_1 = (uint)0;
    float scale_1 = (float)0;

    stop_1 = stop;
    value_4 = value_3;
    count_1 = count;
    scale_1 = scale;
    uint _e8 = count_1;
    int _e9 = value_4;
    value_4 = int((uint(_e9) + _e8));
    bool _e13 = stop_1;
    if (_e13) {
        return;
    }
    float _e14 = scale_1;
    int _e15 = value_4;
    value_4 = naga_f2i32((float(_e15) * _e14));
    return;
}

void forward(bool stop_2, int value_5)
{
    bool stop_3 = (bool)0;
    int value_6 = (int)0;

    stop_3 = stop_2;
    value_6 = value_5;
    bool _e4 = stop_3;
    int _e5 = value_6;
    const int _e6 = next_value(_e5);
    consume(_e4, _e6, 2u, 0.5);
    return;
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

[numthreads(64, 1, 1)]
void void_functions(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;
    int value = (int)0;
    int iterations = (int)0;

    uint _e3 = n_1.value;
    n = _e3;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e13 = i;
    uint _e14 = n;
    if ((_e13 < _e14)) {
        uint _e18 = i;
        base = (_e18 * 6u);
        value = int(2);
        int _e23 = value;
        value = asint(asuint(_e23) + asuint(int(1)));
        consume(false, _e23, 1u, 1.5);
        int _e30 = value;
        uint _e28 = base;
        out_.Store(_e28*4, asuint(_e30));
        int _e32 = value;
        value = asint(asuint(_e32) + asuint(int(1)));
        const int _e35 = next_value(_e32);
        consume(true, _e35, 2u, 0.5);
        int _e42 = value;
        uint _e39 = base;
        out_.Store((_e39 + 1u)*4, asuint(_e42));
        uint _e44 = i;
        int _e48 = value;
        value = asint(asuint(_e48) + asuint(int(1)));
        forward((naga_mod(_e44, 2u) == 0u), _e48);
        int _e55 = value;
        uint _e52 = base;
        out_.Store((_e52 + 2u)*4, asuint(_e55));
        iterations = int(0);
        {
            int _e61 = value;
            value = asint(asuint(_e61) + asuint(int(1)));
            consume((int(0) != int(0)), _e61, uint(int(0)), float(1u));
            uint2 loop_bound = uint2(4294967295u, 4294967295u);
            bool loop_init = true;
            while(true) {
                if (all(loop_bound == uint2(0u, 0u))) { break; }
                loop_bound -= uint2(loop_bound.y == 0u, 1u);
                if (!loop_init) {
                    int _e72 = value;
                    value = asint(asuint(_e72) + asuint(int(1)));
                    int _e75 = value;
                    consume(false, _e75, 0u, 1.0);
                }
                loop_init = false;
                int _e69 = iterations;
                if ((_e69 < int(3))) {
                } else {
                    break;
                }
                int _e78 = iterations;
                iterations = asint(asuint(_e78) + asuint(int(1)));
                int _e82 = iterations;
                if ((_e82 == int(1))) {
                    continue;
                }
            }
        }
        int _e88 = value;
        uint _e85 = base;
        out_.Store((_e85 + 3u)*4, asuint(_e88));
        int _e93 = iterations;
        uint _e90 = base;
        out_.Store((_e90 + 4u)*4, asuint(_e93));
        int _e98 = value;
        const int _e99 = next_value(_e98);
        uint _e95 = base;
        out_.Store((_e95 + 5u)*4, asuint(_e99));
    }
}
