struct type_3 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
ByteAddressBuffer input : register(t1);
cbuffer n_1 : register(b2) { type_3 n_1; }

int cached_type(int value_1)
{
    int value_2 = (int)0;
    int Number = (int)0;
    int next = (int)0;
    int result = (int)0;

    value_2 = value_1;
    int _e3 = value_2;
    Number = _e3;
    int _e6 = Number;
    next = asint(asuint(_e6) + asuint(int(1)));
    int _e10 = next;
    result = asint(asuint(_e10) * asuint(int(2)));
    int _e12 = result;
    return _e12;
}

int pair_add(int left, int right)
{
    int left_1 = (int)0;
    int right_1 = (int)0;

    left_1 = left;
    right_1 = right;
    int _e4 = left_1;
    int _e5 = right_1;
    return asint(asuint(_e4) + asuint(_e5));
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

[numthreads(64, 1, 1)]
void local_declarators(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;
    int value = (int)0;
    int first = (int)0;
    int second = (int)0;
    int third = (int)0;
    int omitted = (int)0;
    int initialized = (int)0;
    int final = (int)0;
    int fixed = (int)0;
    int fixed_next = (int)0;
    int alias_first = (int)0;
    int alias_next = (int)0;
    bool positive = (bool)0;
    bool same = (bool)0;
    bool local = (bool)0;
    float low = (float)0;
    float high = (float)0;
    uint start = (uint)0;
    uint finish = (uint)0;
    int outside = (int)0;
    int outside_1 = (int)0;
    int derived = (int)0;
    int total = (int)0;
    int iteration = (int)0;
    int seed = (int)0;
    int term = (int)0;
    int copied = (int)0;
    int from_call = (int)0;
    int from_prior = (int)0;

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
        int _e26 = value;
        value = asint(asuint(_e26) + asuint(int(1)));
        first = _e26;
        int _e30 = value;
        value = asint(asuint(_e30) + asuint(int(1)));
        second = _e30;
        int _e34 = value;
        third = _e34;
        int _e38 = first;
        int _e41 = second;
        int _e44 = third;
        uint _e35 = base;
        out_.Store(_e35*4, asuint(asint(asuint(asint(asuint(asint(asuint(_e38) * asuint(int(100)))) + asuint(asint(asuint(_e41) * asuint(int(10)))))) + asuint(_e44))));
        int _e50 = value;
        uint _e47 = base;
        out_.Store((_e47 + 1u)*4, asuint(_e50));
        int _e54 = value;
        initialized = asint(asuint(_e54) + asuint(int(3)));
        int _e58 = initialized;
        omitted = asint(asuint(_e58) + asuint(int(2)));
        int _e60 = omitted;
        int _e61 = initialized;
        final = asint(asuint(_e60) + asuint(_e61));
        int _e67 = final;
        uint _e64 = base;
        out_.Store((_e64 + 2u)*4, asuint(_e67));
        int _e70 = value;
        fixed = asint(asuint(_e70) + asuint(int(1)));
        int _e74 = fixed;
        fixed_next = asint(asuint(_e74) + asuint(int(2)));
        int _e80 = fixed_next;
        uint _e77 = base;
        out_.Store((_e77 + 3u)*4, asuint(_e80));
        int _e83 = value;
        alias_first = asint(asuint(_e83) + asuint(int(4)));
        int _e87 = alias_first;
        alias_next = asint(asuint(_e87) + asuint(int(1)));
        int _e93 = alias_next;
        uint _e90 = base;
        out_.Store((_e90 + 4u)*4, asuint(_e93));
        int _e96 = value;
        positive = (_e96 > int(0));
        bool _e99 = positive;
        local = _e99;
        if (_e99) {
            uint _e101 = i;
            int _e104 = asint(input.Load(_e101*4));
            int _e106 = value;
            local = (_e106 == asint(asuint(_e104) + asuint(int(2))));
        }
        bool _e108 = local;
        same = _e108;
        bool _e113 = same;
        uint _e110 = base;
        out_.Store((_e110 + 5u)*4, asuint(int(_e113)));
        int _e116 = value;
        low = (float(_e116) * 0.5);
        float _e122 = low;
        high = (_e122 + 1.5);
        float _e129 = high;
        uint _e125 = base;
        out_.Store((_e125 + 6u)*4, asuint(naga_f2i32((_e129 * 2.0))));
        uint _e134 = i;
        start = naga_mod(_e134, 5u);
        uint _e138 = start;
        finish = (_e138 + 3u);
        uint _e144 = finish;
        uint _e141 = base;
        out_.Store((_e141 + 7u)*4, asuint(int(_e144)));
        int _e150 = value;
        const int _e151 = cached_type(_e150);
        uint _e147 = base;
        out_.Store((_e147 + 8u)*4, asuint(_e151));
        int _e153 = value;
        outside = _e153;
        {
            int _e156 = value;
            outside_1 = asint(asuint(_e156) + asuint(int(5)));
            int _e160 = outside_1;
            derived = asint(asuint(_e160) + asuint(int(2)));
            int _e166 = derived;
            uint _e163 = base;
            out_.Store((_e163 + 9u)*4, asuint(_e166));
        }
        total = int(0);
        {
            iteration = int(0);
            int _e172 = value;
            seed = _e172;
            uint2 loop_bound = uint2(4294967295u, 4294967295u);
            bool loop_init = true;
            while(true) {
                if (all(loop_bound == uint2(0u, 0u))) { break; }
                loop_bound -= uint2(loop_bound.y == 0u, 1u);
                if (!loop_init) {
                    int _e176 = iteration;
                    iteration = asint(asuint(_e176) + asuint(int(1)));
                }
                loop_init = false;
                int _e174 = iteration;
                if ((_e174 < int(4))) {
                } else {
                    break;
                }
                int _e180 = iteration;
                if ((_e180 == int(1))) {
                    continue;
                }
                int _e183 = seed;
                int _e184 = iteration;
                term = asint(asuint(_e183) + asuint(_e184));
                int _e187 = term;
                copied = _e187;
                int _e188 = copied;
                int _e189 = total;
                total = asint(asuint(_e189) + asuint(_e188));
            }
        }
        int _e195 = total;
        uint _e192 = base;
        out_.Store((_e192 + 10u)*4, asuint(_e195));
        int _e197 = value;
        const int _e199 = pair_add(_e197, int(2));
        from_call = _e199;
        int _e201 = from_call;
        const int _e203 = pair_add(_e201, int(3));
        from_prior = _e203;
        int _e208 = from_prior;
        int _e209 = outside;
        uint _e205 = base;
        out_.Store((_e205 + 11u)*4, asuint(asint(asuint(_e208) + asuint(_e209))));
    }
}
