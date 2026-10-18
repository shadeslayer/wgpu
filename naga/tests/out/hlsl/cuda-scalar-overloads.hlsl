struct type_5 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
ByteAddressBuffer input : register(t1);
cbuffer n_1 : register(b2) { type_5 n_1; }

bool classify(bool value_1)
{
    bool value_2 = (bool)0;

    value_2 = value_1;
    bool _e2 = value_2;
    return !(_e2);
}

int classify_1(int value_3)
{
    int value_4 = (int)0;

    value_4 = value_3;
    int _e3 = value_4;
    return asint(asuint(_e3) + asuint(int(10)));
}

uint classify_2(uint value_5)
{
    uint value_6 = (uint)0;

    value_6 = value_5;
    uint _e3 = value_6;
    return (_e3 + 20u);
}

float classify_3(float value_7)
{
    float value_8 = (float)0;

    value_8 = value_7;
    float _e3 = value_8;
    return ((_e3 * 1.5) + 0.5);
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

int rank(int left, float right)
{
    int left_1 = (int)0;
    float right_1 = (float)0;

    left_1 = left;
    right_1 = right;
    int _e5 = left_1;
    float _e7 = right_1;
    return asint(asuint(asint(asuint(_e5) * asuint(int(10)))) + asuint(naga_f2i32(_e7)));
}

int rank_1(float left_2, int right_2)
{
    float left_3 = (float)0;
    int right_3 = (int)0;

    left_3 = left_2;
    right_3 = right_2;
    float _e5 = left_3;
    int _e10 = right_3;
    return asint(asuint(asint(asuint(int(100)) + asuint(asint(asuint(naga_f2i32(_e5)) * asuint(int(10)))))) + asuint(_e10));
}

int pair(float left_4, int right_4)
{
    float left_5 = (float)0;
    int right_5 = (int)0;

    left_5 = left_4;
    right_5 = right_4;
    float _e4 = left_5;
    int _e8 = right_5;
    return asint(asuint(asint(asuint(naga_f2i32(_e4)) * asuint(int(10)))) + asuint(_e8));
}

int promote(uint left_6, float right_6)
{
    uint left_7 = (uint)0;
    float right_7 = (float)0;

    left_7 = left_6;
    right_7 = right_6;
    uint _e5 = left_7;
    float _e8 = right_7;
    return asint(asuint(asint(asuint(int(200)) + asuint(int(_e5)))) + asuint(naga_f2i32(_e8)));
}

void touch(bool stop, int value_9)
{
    bool stop_1 = (bool)0;
    int value_10 = (int)0;

    stop_1 = stop;
    value_10 = value_9;
    bool _e4 = stop_1;
    if (_e4) {
        return;
    }
    int _e5 = value_10;
    const int _e6 = classify_1(_e5);
    value_10 = _e6;
    return;
}

int touch_1(int value_11)
{
    int value_12 = (int)0;

    value_12 = value_11;
    int _e2 = value_12;
    const int _e3 = classify_1(_e2);
    return asint(asuint(_e3) + asuint(int(3)));
}

int stage(int value_13)
{
    int value_14 = (int)0;

    value_14 = value_13;
    int _e3 = value_14;
    return asint(asuint(_e3) + asuint(int(2)));
}

int frozen(float value_15)
{
    float value_16 = (float)0;

    value_16 = value_15;
    float _e2 = value_16;
    const int _e4 = stage(naga_f2i32(_e2));
    return _e4;
}

int stage_1(float value_17)
{
    float value_18 = (float)0;

    value_18 = value_17;
    float _e2 = value_18;
    return asint(asuint(naga_f2i32(_e2)) + asuint(int(100)));
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

[numthreads(64, 1, 1)]
void scalar_overloads(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;
    int value = (int)0;
    bool choose = (bool)0;
    int effect = (int)0;
    int nested = (int)0;
    int argument = (int)0;
    int captured = (int)0;

    uint _e4 = n_1.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e14 = i;
    uint _e15 = n;
    if ((_e14 < _e15)) {
        uint _e19 = i;
        base = (_e19 * 13u);
        uint _e22 = i;
        int _e24 = asint(input.Load(_e22*4));
        value = _e24;
        uint _e27 = i;
        choose = (naga_mod(_e27, 2u) == 0u);
        bool _e33 = choose;
        const bool _e34 = classify(_e33);
        uint _e31 = base;
        out_.Store(_e31*4, asuint(int(_e34)));
        int _e40 = value;
        const int _e41 = classify_1(_e40);
        uint _e37 = base;
        out_.Store((_e37 + 1u)*4, asuint(_e41));
        int _e47 = value;
        const uint _e50 = classify_2(uint(asint(asuint(_e47) + asuint(int(16)))));
        uint _e43 = base;
        out_.Store((_e43 + 2u)*4, asuint(int(_e50)));
        int _e56 = value;
        const float _e58 = classify_3(float(_e56));
        uint _e53 = base;
        out_.Store((_e53 + 3u)*4, asuint(naga_f2i32(_e58)));
        bool _e64 = choose;
        int _e65 = value;
        const int _e70 = rank(int(_e64), (float(_e65) + 0.5));
        uint _e61 = base;
        out_.Store((_e61 + 4u)*4, asuint(_e70));
        int _e75 = value;
        bool _e79 = choose;
        const int _e81 = rank_1((float(_e75) + 0.5), int(_e79));
        uint _e72 = base;
        out_.Store((_e72 + 5u)*4, asuint(_e81));
        int _e87 = value;
        int _e90 = value;
        const int _e92 = promote(uint(asint(asuint(_e87) + asuint(int(16)))), float(_e90));
        uint _e83 = base;
        out_.Store((_e83 + 6u)*4, asuint(_e92));
        int _e94 = value;
        effect = _e94;
        bool _e95 = choose;
        int _e96 = effect;
        effect = asint(asuint(_e96) + asuint(int(1)));
        touch(_e95, _e96);
        int _e103 = effect;
        uint _e100 = base;
        out_.Store((_e100 + 7u)*4, asuint(_e103));
        int _e108 = effect;
        const int _e109 = touch_1(_e108);
        uint _e105 = base;
        out_.Store((_e105 + 8u)*4, asuint(_e109));
        int _e114 = value;
        const int _e118 = frozen((float(_e114) + 0.75));
        uint _e111 = base;
        out_.Store((_e111 + 9u)*4, asuint(_e118));
        int _e123 = value;
        const int _e127 = stage_1((float(_e123) + 0.75));
        uint _e120 = base;
        out_.Store((_e120 + 10u)*4, asuint(_e127));
        int _e129 = effect;
        effect = asint(asuint(_e129) + asuint(int(1)));
        const int _e132 = classify_1(_e129);
        const int _e133 = touch_1(_e132);
        nested = _e133;
        int _e139 = nested;
        int _e141 = effect;
        uint _e135 = base;
        out_.Store((_e135 + 11u)*4, asuint(asint(asuint(asint(asuint(_e139) * asuint(int(10)))) + asuint(_e141))));
        int _e145 = value;
        argument = asint(asuint(_e145) + asuint(int(8)));
        int _e148 = argument;
        int _e149 = argument;
        argument = asint(asuint(_e149) + asuint(int(1)));
        const int _e153 = pair(float(_e148), _e149);
        captured = _e153;
        int _e159 = captured;
        int _e161 = argument;
        uint _e155 = base;
        out_.Store((_e155 + 12u)*4, asuint(asint(asuint(asint(asuint(_e159) * asuint(int(100)))) + asuint(_e161))));
    }
}
