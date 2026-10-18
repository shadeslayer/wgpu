struct type_3 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
ByteAddressBuffer input : register(t1);
cbuffer n_1 : register(b2) { type_3 n_1; }

void consume(int value_1)
{
    int value_2 = (int)0;

    value_2 = value_1;
    int _e3 = value_2;
    if ((_e3 < int(0))) {
        return;
    } else {
        return;
    }
}

int pair_add(int left_1, int right_1)
{
    int left_2 = (int)0;
    int right_2 = (int)0;

    left_2 = left_1;
    right_2 = right_1;
    int _e4 = left_2;
    int _e5 = right_2;
    return asint(asuint(_e4) + asuint(_e5));
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

[numthreads(64, 1, 1)]
void comma(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;
    int value = (int)0;
    bool choose = (bool)0;
    int x = (int)0;
    int sequenced = (int)0;
    int target = (int)0;
    int rhs = (int)0;
    int middle = (int)0;
    int selected = (int)0;
    int local = (int)0;
    int left = (int)0;
    int right = (int)0;
    int identity = (int)0;
    uint position = (uint)0;
    int effect = (int)0;
    int total = (int)0;
    int iteration = (int)0;
    int current = (int)0;
    int ticks = (int)0;
    int passes = (int)0;
    int argument = (int)0;
    int first = (int)0;
    int second = (int)0;

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
        int _e32 = value;
        x = _e32;
        int _e35 = x;
        x = asint(asuint(_e35) + asuint(int(2)));
        int _e38 = x;
        x = asint(asuint(_e38) * asuint(int(3)));
        int _e41 = x;
        sequenced = asint(asuint(_e41) - asuint(int(1)));
        int _e45 = sequenced;
        uint _e43 = base;
        out_.Store(_e43*4, asuint(_e45));
        int _e50 = x;
        uint _e47 = base;
        out_.Store((_e47 + 1u)*4, asuint(_e50));
        target = int(0);
        int _e53 = value;
        target = _e53;
        int _e55 = target;
        target = asint(asuint(_e55) + asuint(int(4)));
        int _e61 = target;
        uint _e58 = base;
        out_.Store((_e58 + 2u)*4, asuint(_e61));
        int _e64 = value;
        target = asint(asuint(_e64) + asuint(int(1)));
        int _e67 = target;
        rhs = asint(asuint(_e67) + asuint(int(3)));
        int _e74 = rhs;
        int _e76 = target;
        uint _e70 = base;
        out_.Store((_e70 + 3u)*4, asuint(asint(asuint(asint(asuint(_e74) * asuint(int(10)))) + asuint(_e76))));
        middle = int(0);
        bool _e81 = choose;
        if (_e81) {
            int _e83 = value;
            middle = asint(asuint(_e83) + asuint(int(2)));
            int _e86 = middle;
            local = asint(asuint(_e86) + asuint(int(3)));
        } else {
            int _e89 = value;
            local = asint(asuint(_e89) - asuint(int(2)));
        }
        int _e92 = local;
        selected = _e92;
        int _e98 = selected;
        int _e100 = middle;
        uint _e94 = base;
        out_.Store((_e94 + 4u)*4, asuint(asint(asuint(asint(asuint(_e98) * asuint(int(10)))) + asuint(_e100))));
        int _e103 = value;
        left = _e103;
        int _e106 = value;
        right = asint(asuint(_e106) + asuint(int(10)));
        int _e108 = value;
        consume(_e108);
        bool _e109 = choose;
        if (_e109) {
            int _e111 = left;
            left = asint(asuint(_e111) + asuint(int(3)));
        } else {
            int _e113 = right;
            right = asint(asuint(_e113) + asuint(int(3)));
        }
        int _e120 = left;
        int _e122 = right;
        uint _e116 = base;
        out_.Store((_e116 + 5u)*4, asuint(asint(asuint(asint(asuint(_e120) * asuint(int(100)))) + asuint(_e122))));
        int _e125 = value;
        identity = _e125;
        int _e126 = identity;
        identity = asint(asuint(_e126) + asuint(int(1)));
        consume(_e126);
        int _e129 = identity;
        identity = asint(asuint(_e129) + asuint(int(1)));
        int _e136 = identity;
        uint _e133 = base;
        out_.Store((_e133 + 6u)*4, asuint(_e136));
        uint _e139 = base;
        position = (_e139 + 4u);
        uint _e148 = position;
        position = (_e148 + 2u);
        int _e151 = value;
        uint _e141 = position;
        position = (_e141 + uint(int(1)));
        uint _e145 = position;
        out_.Store(_e145*4, asuint(asint(asuint(asint(asuint(_e151) * asuint(int(2)))) + asuint(int(2)))));
        int _e156 = value;
        effect = _e156;
        int _e157 = effect;
        effect = asint(asuint(_e157) + asuint(int(1)));
        consume(_e157);
        int _e160 = effect;
        effect = asint(asuint(_e160) + asuint(int(1)));
        consume(_e160);
        int _e163 = effect;
        effect = asint(asuint(_e163) + asuint(int(1)));
        consume(_e163);
        int _e166 = effect;
        effect = asint(asuint(_e166) + asuint(int(1)));
        consume(_e166);
        int _e173 = effect;
        uint _e170 = base;
        out_.Store((_e170 + 8u)*4, asuint(_e173));
        total = int(0);
        {
            iteration = int(0);
            int _e179 = value;
            current = _e179;
            uint2 loop_bound = uint2(4294967295u, 4294967295u);
            bool loop_init = true;
            while(true) {
                if (all(loop_bound == uint2(0u, 0u))) { break; }
                loop_bound -= uint2(loop_bound.y == 0u, 1u);
                if (!loop_init) {
                    int _e183 = iteration;
                    iteration = asint(asuint(_e183) + asuint(int(1)));
                    int _e187 = current;
                    current = asint(asuint(_e187) + asuint(int(2)));
                }
                loop_init = false;
                int _e181 = iteration;
                if ((_e181 < int(4))) {
                } else {
                    break;
                }
                int _e190 = iteration;
                if ((_e190 != int(1))) {
                    int _e192 = current;
                    int _e193 = total;
                    total = asint(asuint(_e193) + asuint(_e192));
                } else {
                    continue;
                }
            }
        }
        int _e199 = total;
        uint _e196 = base;
        out_.Store((_e196 + 9u)*4, asuint(_e199));
        int _e201 = value;
        ticks = _e201;
        passes = int(0);
        {
            int _e205 = value;
            ticks = asint(asuint(_e205) + asuint(int(1)));
            passes = int(0);
            uint2 loop_bound_1 = uint2(4294967295u, 4294967295u);
            bool loop_init_1 = true;
            while(true) {
                if (all(loop_bound_1 == uint2(0u, 0u))) { break; }
                loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
                if (!loop_init_1) {
                    int _e215 = ticks;
                    ticks = asint(asuint(_e215) + asuint(int(2)));
                }
                loop_init_1 = false;
                int _e209 = passes;
                passes = asint(asuint(_e209) + asuint(int(1)));
                int _e212 = passes;
                if ((_e212 < int(4))) {
                } else {
                    break;
                }
            }
        }
        int _e222 = ticks;
        int _e224 = passes;
        uint _e218 = base;
        out_.Store((_e218 + 10u)*4, asuint(asint(asuint(asint(asuint(_e222) * asuint(int(10)))) + asuint(_e224))));
        int _e227 = value;
        argument = _e227;
        int _e230 = argument;
        argument = asint(asuint(_e230) + asuint(int(1)));
        int _e232 = argument;
        const int _e234 = pair_add(_e232, int(2));
        first = _e234;
        int _e236 = first;
        const int _e238 = pair_add(_e236, int(3));
        second = _e238;
        int _e244 = second;
        int _e246 = argument;
        uint _e240 = base;
        out_.Store((_e240 + 11u)*4, asuint(asint(asuint(asint(asuint(_e244) * asuint(int(10)))) + asuint(_e246))));
    }
}
