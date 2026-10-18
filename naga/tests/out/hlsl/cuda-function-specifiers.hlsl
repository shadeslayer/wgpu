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

int twice(int value_3)
{
    int value_4 = (int)0;

    value_4 = value_3;
    int _e2 = value_4;
    const int _e3 = increment(_e2);
    return asint(asuint(_e3) * asuint(int(2)));
}

int advance(int value_5)
{
    int value_6 = (int)0;

    value_6 = value_5;
    int _e2 = value_6;
    const int _e3 = twice(_e2);
    return asint(asuint(_e3) + asuint(int(1)));
}

int retreat(int value_7)
{
    int value_8 = (int)0;

    value_8 = value_7;
    int _e3 = value_8;
    return asint(asuint(_e3) - asuint(int(3)));
}

float scaled(float value_9)
{
    float value_10 = (float)0;

    value_10 = value_9;
    float _e3 = value_10;
    return ((_e3 * 1.5) + 0.5);
}

bool differs(int value_11, int other)
{
    int value_12 = (int)0;
    int other_1 = (int)0;

    value_12 = value_11;
    other_1 = other;
    int _e4 = value_12;
    int _e5 = other_1;
    return (_e4 != _e5);
}

void consume(bool stop, int value_13)
{
    bool stop_1 = (bool)0;
    int value_14 = (int)0;

    stop_1 = stop;
    value_14 = value_13;
    bool _e4 = stop_1;
    if (_e4) {
        return;
    }
    int _e5 = value_14;
    const int _e6 = advance(_e5);
    value_14 = _e6;
    return;
}

void forward(bool stop_2, int value_15)
{
    bool stop_3 = (bool)0;
    int value_16 = (int)0;

    stop_3 = stop_2;
    value_16 = value_15;
    bool _e4 = stop_3;
    int _e5 = value_16;
    const int _e6 = retreat(_e5);
    consume(_e4, _e6);
    return;
}

uint plus_one(uint value_17)
{
    uint value_18 = (uint)0;

    value_18 = value_17;
    uint _e3 = value_18;
    return (_e3 + 1u);
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

[numthreads(64, 1, 1)]
void function_specifiers(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;
    int value = (int)0;
    bool choose = (bool)0;
    int effect = (int)0;

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
        uint _e27 = i;
        choose = (naga_mod(_e27, 2u) == 0u);
        int _e33 = value;
        const int _e34 = increment(_e33);
        uint _e31 = base;
        out_.Store(_e31*4, asuint(_e34));
        int _e39 = value;
        const int _e40 = twice(_e39);
        uint _e36 = base;
        out_.Store((_e36 + 1u)*4, asuint(_e40));
        int _e45 = value;
        const int _e46 = advance(_e45);
        uint _e42 = base;
        out_.Store((_e42 + 2u)*4, asuint(_e46));
        int _e51 = value;
        const int _e52 = retreat(_e51);
        uint _e48 = base;
        out_.Store((_e48 + 3u)*4, asuint(_e52));
        int _e57 = value;
        const float _e59 = scaled(float(_e57));
        uint _e54 = base;
        out_.Store((_e54 + 4u)*4, asuint(naga_f2i32(_e59)));
        int _e65 = value;
        uint _e66 = i;
        int _e68 = asint(input.Load(_e66*4));
        const bool _e69 = differs(_e65, _e68);
        uint _e62 = base;
        out_.Store((_e62 + 5u)*4, asuint(int(_e69)));
        int _e72 = value;
        effect = _e72;
        bool _e73 = choose;
        int _e74 = effect;
        effect = asint(asuint(_e74) + asuint(int(1)));
        consume(_e73, _e74);
        int _e81 = effect;
        uint _e78 = base;
        out_.Store((_e78 + 6u)*4, asuint(_e81));
        bool _e82 = choose;
        int _e84 = effect;
        effect = asint(asuint(_e84) + asuint(int(1)));
        forward(!(_e82), _e84);
        int _e91 = effect;
        uint _e88 = base;
        out_.Store((_e88 + 7u)*4, asuint(_e91));
        int _e96 = effect;
        const int _e97 = advance(_e96);
        uint _e93 = base;
        out_.Store((_e93 + 8u)*4, asuint(_e97));
        int _e103 = effect;
        const uint _e106 = plus_one(uint(asint(asuint(_e103) + asuint(int(16)))));
        uint _e99 = base;
        out_.Store((_e99 + 9u)*4, asuint(int(_e106)));
        int _e112 = effect;
        uint _e113 = i;
        int _e115 = asint(input.Load(_e113*4));
        const bool _e116 = differs(_e112, _e115);
        uint _e109 = base;
        out_.Store((_e109 + 10u)*4, asuint(int(_e116)));
        int _e122 = effect;
        uint _e119 = base;
        out_.Store((_e119 + 11u)*4, asuint(_e122));
    }
}
