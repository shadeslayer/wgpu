struct type_5 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
ByteAddressBuffer input : register(t1);
cbuffer n_1 : register(b2) { type_5 n_1; }

int increment(int value_1)
{
    int value_2 = (int)0;

    value_2 = value_1;
    int _e3 = value_2;
    value_2 = asint(asuint(_e3) + asuint(int(2)));
    int _e5 = value_2;
    return _e5;
}

int doubled(int value_3)
{
    int value_4 = (int)0;

    value_4 = value_3;
    int _e2 = value_4;
    const int _e3 = increment(_e2);
    return asint(asuint(_e3) * asuint(int(2)));
}

uint advance(uint value_5)
{
    uint value_6 = (uint)0;

    value_6 = value_5;
    uint _e3 = value_6;
    return (_e3 + 3u);
}

uint retreat(uint value_7)
{
    uint value_8 = (uint)0;

    value_8 = value_7;
    uint _e3 = value_8;
    return (_e3 - 1u);
}

float scale(float value_9, int bias)
{
    float value_10 = (float)0;
    int bias_1 = (int)0;

    value_10 = value_9;
    bias_1 = bias;
    float _e5 = value_10;
    int _e7 = bias_1;
    return ((_e5 * 1.5) + float(_e7));
}

bool same(int left, int right)
{
    int left_1 = (int)0;
    int right_1 = (int)0;

    left_1 = left;
    right_1 = right;
    int _e4 = left_1;
    int _e5 = right_1;
    return (_e4 == _e5);
}

void consume(bool stop, int value_11)
{
    bool stop_1 = (bool)0;
    int value_12 = (int)0;

    stop_1 = stop;
    value_12 = value_11;
    bool _e4 = stop_1;
    if (_e4) {
        return;
    }
    int _e5 = value_12;
    const int _e6 = doubled(_e5);
    value_12 = _e6;
    return;
}

void forward(bool stop_2, int value_13)
{
    bool stop_3 = (bool)0;
    int value_14 = (int)0;

    stop_3 = stop_2;
    value_14 = value_13;
    bool _e4 = stop_3;
    int _e5 = value_14;
    const int _e6 = increment(_e5);
    consume(_e4, _e6);
    return;
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

[numthreads(64, 1, 1)]
void inline_functions(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;
    int value = (int)0;

    uint _e4 = n_1.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e14 = i;
    uint _e15 = n;
    if ((_e14 < _e15)) {
        uint _e19 = i;
        base = (_e19 * 12u);
        uint _e22 = i;
        int _e24 = asint(input.Load(_e22*4));
        value = _e24;
        int _e27 = value;
        const int _e28 = increment(_e27);
        uint _e25 = base;
        out_.Store(_e25*4, asuint(_e28));
        int _e33 = value;
        uint _e30 = base;
        out_.Store((_e30 + 1u)*4, asuint(_e33));
        int _e38 = value;
        const int _e39 = doubled(_e38);
        uint _e35 = base;
        out_.Store((_e35 + 2u)*4, asuint(_e39));
        int _e44 = value;
        const uint _e46 = advance(uint(_e44));
        uint _e41 = base;
        out_.Store((_e41 + 3u)*4, asuint(int(_e46)));
        int _e52 = value;
        const uint _e54 = retreat(uint(_e52));
        uint _e49 = base;
        out_.Store((_e49 + 4u)*4, asuint(int(_e54)));
        int _e60 = value;
        const float _e63 = scale(float(_e60), int(2));
        uint _e57 = base;
        out_.Store((_e57 + 5u)*4, asuint(naga_f2i32(_e63)));
        int _e69 = value;
        uint _e70 = i;
        int _e72 = asint(input.Load(_e70*4));
        const bool _e73 = same(_e69, _e72);
        uint _e66 = base;
        out_.Store((_e66 + 6u)*4, asuint(int(_e73)));
        uint _e76 = i;
        int _e80 = value;
        value = asint(asuint(_e80) + asuint(int(1)));
        consume((naga_mod(_e76, 2u) == 0u), _e80);
        int _e87 = value;
        uint _e84 = base;
        out_.Store((_e84 + 7u)*4, asuint(_e87));
        uint _e89 = i;
        int _e93 = value;
        value = asint(asuint(_e93) + asuint(int(1)));
        forward((naga_mod(_e89, 2u) != 0u), _e93);
        int _e100 = value;
        uint _e97 = base;
        out_.Store((_e97 + 8u)*4, asuint(_e100));
        int _e105 = value;
        uint _e106 = i;
        int _e108 = asint(input.Load(_e106*4));
        const bool _e109 = same(_e105, _e108);
        uint _e102 = base;
        out_.Store((_e102 + 9u)*4, asuint(int(_e109)));
        int _e115 = value;
        const int _e116 = increment(_e115);
        const int _e117 = increment(_e116);
        uint _e112 = base;
        out_.Store((_e112 + 10u)*4, asuint(_e117));
        int _e122 = value;
        uint _e119 = base;
        out_.Store((_e119 + 11u)*4, asuint(_e122));
    }
}
