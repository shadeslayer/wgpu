struct type_5 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
cbuffer n_1 : register(b1) { type_5 n_1; }

float affine(float value_1, float scale, int offset)
{
    float value_2 = (float)0;
    float scale_1 = (float)0;
    int offset_1 = (int)0;

    value_2 = value_1;
    scale_1 = scale;
    offset_1 = offset;
    float _e6 = scale_1;
    float _e7 = value_2;
    value_2 = (_e7 * _e6);
    float _e9 = value_2;
    int _e10 = offset_1;
    return (_e9 + float(_e10));
}

bool nonnegative(int value_3)
{
    int value_4 = (int)0;

    value_4 = value_3;
    int _e3 = value_4;
    return (_e3 >= int(0));
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

uint sum_steps(uint count, bool include_even)
{
    uint count_1 = (uint)0;
    bool include_even_1 = (bool)0;
    uint total = (uint)0;
    uint i_1 = (uint)0;
    bool local = (bool)0;

    count_1 = count;
    include_even_1 = include_even;
    total = 0u;
    {
        i_1 = 0u;
        uint2 loop_bound = uint2(4294967295u, 4294967295u);
        bool loop_init = true;
        while(true) {
            if (all(loop_bound == uint2(0u, 0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                uint _e11 = i_1;
                i_1 = (_e11 + uint(int(1)));
            }
            loop_init = false;
            uint _e8 = i_1;
            uint _e9 = count_1;
            if ((_e8 < _e9)) {
            } else {
                break;
            }
            bool _e15 = include_even_1;
            local = _e15;
            if (_e15) {
            } else {
                uint _e18 = i_1;
                local = (naga_mod(_e18, 2u) != 0u);
            }
            bool _e22 = local;
            if (_e22) {
                uint _e23 = i_1;
                uint _e24 = total;
                total = (_e24 + _e23);
            }
        }
    }
    uint _e26 = total;
    return _e26;
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

int choose(int value_5)
{
    int value_6 = (int)0;

    value_6 = value_5;
    int _e2 = value_6;
    const bool _e3 = nonnegative(_e2);
    if (_e3) {
        int _e4 = value_6;
        const float _e8 = affine(float(_e4), 2.0, int(1));
        return naga_f2i32(_e8);
    }
    int _e10 = value_6;
    return naga_neg(_e10);
}

int pair(int a, int b)
{
    int a_1 = (int)0;
    int b_1 = (int)0;

    a_1 = a;
    b_1 = b;
    int _e5 = a_1;
    int _e7 = b_1;
    return asint(asuint(asint(asuint(_e5) * asuint(int(10)))) + asuint(_e7));
}

[numthreads(64, 1, 1)]
void device_functions(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;
    int value = (int)0;
    int argument = (int)0;

    uint _e3 = n_1.value;
    n = _e3;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e13 = i;
    uint _e14 = n;
    if ((_e13 < _e14)) {
        uint _e18 = i;
        base = (_e18 * 10u);
        uint _e21 = i;
        value = asint(asuint(int(_e21)) - asuint(int(35)));
        argument = int(3);
        int _e29 = value;
        const int _e30 = choose(_e29);
        uint _e27 = base;
        out_.Store(_e27*4, asuint(_e30));
        int _e35 = value;
        const float _e39 = affine(float(_e35), 1.5, int(2));
        uint _e32 = base;
        out_.Store((_e32 + 1u)*4, asuint(naga_f2i32(_e39)));
        const uint _e48 = sum_steps(uint(int(5)), true);
        uint _e42 = base;
        out_.Store((_e42 + 2u)*4, asuint(int(_e48)));
        const uint _e58 = sum_steps(5u, (int(0) != int(0)));
        uint _e51 = base;
        out_.Store((_e51 + 3u)*4, asuint(int(_e58)));
        int _e64 = value;
        const float _e70 = affine(float(_e64), float(2u), int(true));
        uint _e61 = base;
        out_.Store((_e61 + 4u)*4, asuint(naga_f2i32(_e70)));
        int _e76 = value;
        uint _e73 = base;
        out_.Store((_e73 + 5u)*4, asuint(_e76));
        int _e81 = value;
        const int _e82 = choose(_e81);
        int _e84 = value;
        const int _e86 = choose(asint(asuint(_e84) + asuint(int(1))));
        uint _e78 = base;
        out_.Store((_e78 + 6u)*4, asuint(asint(asuint(_e82) + asuint(_e86))));
        int _e92 = value;
        const bool _e93 = nonnegative(_e92);
        uint _e89 = base;
        out_.Store((_e89 + 7u)*4, asuint(int(_e93)));
        int _e99 = argument;
        int _e100 = argument;
        argument = asint(asuint(_e100) + asuint(int(1)));
        const int _e103 = pair(_e99, _e100);
        uint _e96 = base;
        out_.Store((_e96 + 8u)*4, asuint(_e103));
        int _e108 = argument;
        uint _e105 = base;
        out_.Store((_e105 + 9u)*4, asuint(_e108));
    }
}
