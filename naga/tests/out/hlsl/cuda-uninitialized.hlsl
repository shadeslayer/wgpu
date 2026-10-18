struct type_5 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
ByteAddressBuffer input : register(t1);
cbuffer n_1 : register(b2) { type_5 n_1; }

int select_value(int value_1, bool choose_first)
{
    int value_2 = (int)0;
    bool choose_first_1 = (bool)0;
    int result = (int)0;

    value_2 = value_1;
    choose_first_1 = choose_first;
    bool _e5 = choose_first_1;
    if (_e5) {
        int _e7 = value_2;
        result = asint(asuint(_e7) + asuint(int(3)));
    } else {
        int _e10 = value_2;
        result = asint(asuint(_e10) - asuint(int(2)));
    }
    int _e12 = result;
    return _e12;
}

float scaled(float value_3)
{
    float value_4 = (float)0;
    float result_1 = (float)0;

    value_4 = value_3;
    float _e4 = value_4;
    result_1 = ((_e4 * 2.0) + 4.0);
    float _e8 = result_1;
    return _e8;
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

uint accumulate(uint limit)
{
    uint limit_1 = (uint)0;
    uint sum = (uint)0;
    uint i_1 = (uint)0;
    uint term = (uint)0;

    limit_1 = limit;
    sum = 0u;
    {
        i_1 = 0u;
        uint2 loop_bound = uint2(4294967295u, 4294967295u);
        bool loop_init = true;
        while(true) {
            if (all(loop_bound == uint2(0u, 0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                uint _e9 = i_1;
                i_1 = (_e9 + uint(int(1)));
            }
            loop_init = false;
            uint _e6 = i_1;
            uint _e7 = limit_1;
            if ((_e6 < _e7)) {
            } else {
                break;
            }
            uint _e15 = i_1;
            if ((naga_mod(_e15, 2u) == 0u)) {
                uint _e20 = i_1;
                term = (_e20 + 1u);
            } else {
                uint _e23 = i_1;
                term = (_e23 * 2u);
            }
            uint _e25 = term;
            uint _e26 = sum;
            sum = (_e26 + _e25);
        }
    }
    uint _e28 = sum;
    return _e28;
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

[numthreads(64, 1, 1)]
void uninitialized(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;
    int value = (int)0;
    int chosen = (int)0;
    float weighted = (float)0;
    bool negative = (bool)0;
    uint magnitude = (uint)0;
    int total = (int)0;
    int iteration = (int)0;
    int copied = (int)0;
    int descending = (int)0;
    int iteration_1 = (int)0;
    int staged = (int)0;
    int remaining = (int)0;
    int partial = (int)0;
    int next = (int)0;
    int shadow = (int)0;
    int shadow_1 = (int)0;

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
        if ((naga_mod(_e27, 2u) == 0u)) {
            int _e32 = value;
            chosen = asint(asuint(_e32) + asuint(int(7)));
        } else {
            int _e35 = value;
            chosen = asint(asuint(_e35) - asuint(int(4)));
        }
        int _e39 = chosen;
        uint _e37 = base;
        out_.Store(_e37*4, asuint(_e39));
        int _e44 = value;
        uint _e46 = i;
        const int _e50 = select_value(_e44, (naga_mod(_e46, 3u) == 0u));
        uint _e41 = base;
        out_.Store((_e41 + 1u)*4, asuint(_e50));
        int _e52 = value;
        weighted = ((float(_e52) * 1.5) + 2.0);
        float _e62 = weighted;
        uint _e59 = base;
        out_.Store((_e59 + 2u)*4, asuint(naga_f2i32(_e62)));
        int _e66 = value;
        negative = (_e66 < int(0));
        bool _e72 = negative;
        uint _e69 = base;
        out_.Store((_e69 + 3u)*4, asuint(int(_e72)));
        int _e76 = value;
        magnitude = uint(asint(asuint(_e76) + asuint(int(5))));
        uint _e84 = magnitude;
        uint _e80 = base;
        out_.Store((_e80 + 4u)*4, asuint(int((_e84 * 3u))));
        uint _e92 = i;
        const uint _e94 = accumulate(naga_mod(_e92, 5u));
        uint _e88 = base;
        out_.Store((_e88 + 5u)*4, asuint(int(_e94)));
        total = int(0);
        {
            iteration = int(0);
            uint2 loop_bound_1 = uint2(4294967295u, 4294967295u);
            bool loop_init_1 = true;
            while(true) {
                if (all(loop_bound_1 == uint2(0u, 0u))) { break; }
                loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
                if (!loop_init_1) {
                    int _e103 = iteration;
                    iteration = asint(asuint(_e103) + asuint(int(1)));
                }
                loop_init_1 = false;
                int _e101 = iteration;
                if ((_e101 < int(3))) {
                } else {
                    break;
                }
                int _e107 = value;
                int _e108 = iteration;
                copied = asint(asuint(_e107) + asuint(_e108));
                int _e110 = copied;
                int _e111 = total;
                total = asint(asuint(_e111) + asuint(_e110));
            }
        }
        int _e117 = total;
        uint _e114 = base;
        out_.Store((_e114 + 6u)*4, asuint(_e117));
        descending = int(0);
        iteration_1 = int(0);
        {
            uint2 loop_bound_2 = uint2(4294967295u, 4294967295u);
            bool loop_init_2 = true;
            while(true) {
                if (all(loop_bound_2 == uint2(0u, 0u))) { break; }
                loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
                if (!loop_init_2) {
                    int _e126 = iteration_1;
                    iteration_1 = asint(asuint(_e126) + asuint(int(1)));
                }
                loop_init_2 = false;
                int _e124 = iteration_1;
                if ((_e124 < int(3))) {
                } else {
                    break;
                }
                int _e129 = value;
                int _e130 = iteration_1;
                staged = asint(asuint(_e129) - asuint(_e130));
                int _e132 = staged;
                int _e133 = descending;
                descending = asint(asuint(_e133) + asuint(_e132));
            }
        }
        int _e139 = descending;
        uint _e136 = base;
        out_.Store((_e136 + 7u)*4, asuint(_e139));
        remaining = int(2);
        int _e143 = value;
        partial = _e143;
        uint2 loop_bound_3 = uint2(4294967295u, 4294967295u);
        while(true) {
            if (all(loop_bound_3 == uint2(0u, 0u))) { break; }
            loop_bound_3 -= uint2(loop_bound_3.y == 0u, 1u);
            int _e145 = remaining;
            if ((_e145 > int(0))) {
            } else {
                break;
            }
            {
                int _e148 = partial;
                int _e149 = remaining;
                next = asint(asuint(_e148) + asuint(_e149));
                int _e151 = next;
                partial = _e151;
                int _e152 = remaining;
                remaining = asint(asuint(_e152) - asuint(int(1)));
            }
        }
        int _e159 = partial;
        uint _e156 = base;
        out_.Store((_e156 + 8u)*4, asuint(_e159));
        int _e161 = value;
        shadow = _e161;
        {
            int _e164 = value;
            shadow_1 = asint(asuint(_e164) + asuint(int(10)));
            int _e170 = shadow_1;
            uint _e167 = base;
            out_.Store((_e167 + 9u)*4, asuint(_e170));
        }
        int _e175 = shadow;
        uint _e172 = base;
        out_.Store((_e172 + 10u)*4, asuint(_e175));
        int _e180 = value;
        const float _e182 = scaled(float(_e180));
        uint _e177 = base;
        out_.Store((_e177 + 11u)*4, asuint(naga_f2i32(_e182)));
    }
}
