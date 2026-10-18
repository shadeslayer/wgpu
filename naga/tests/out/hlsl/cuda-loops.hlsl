struct type_2 {
    int value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
cbuffer n_1 : register(b1) { type_2 n_1; }

[numthreads(1, 1, 1)]
void loops(uint3 threadIdx : SV_GroupThreadID)
{
    int n = (int)0;
    int sum = (int)0;
    int i = (int)0;
    int i_1 = (int)0;

    int _e3 = n_1.value;
    n = _e3;
    sum = int(0);
    {
        i = int(0);
        uint2 loop_bound = uint2(4294967295u, 4294967295u);
        bool loop_init = true;
        while(true) {
            if (all(loop_bound == uint2(0u, 0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e13 = i;
                i = asint(asuint(_e13) + asuint(int(1)));
            }
            loop_init = false;
            int _e9 = i;
            int _e10 = n;
            if ((_e9 < _e10)) {
            } else {
                break;
            }
            int _e16 = i;
            if ((_e16 == int(2))) {
                continue;
            }
            int _e19 = i;
            if ((_e19 == int(7))) {
                break;
            }
            int _e21 = sum;
            int _e22 = i;
            sum = asint(asuint(_e21) + asuint(_e22));
        }
    }
    i_1 = int(0);
    uint2 loop_bound_1 = uint2(4294967295u, 4294967295u);
    while(true) {
        if (all(loop_bound_1 == uint2(0u, 0u))) { break; }
        loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
        int _e27 = i_1;
        if ((_e27 < int(3))) {
        } else {
            break;
        }
        {
            int _e30 = i_1;
            i_1 = asint(asuint(_e30) + asuint(int(1)));
            int _e33 = i_1;
            if ((_e33 == int(2))) {
                continue;
            }
            int _e36 = sum;
            sum = asint(asuint(_e36) + asuint(int(10)));
        }
    }
    uint2 loop_bound_2 = uint2(4294967295u, 4294967295u);
    bool loop_init_1 = true;
    while(true) {
        if (all(loop_bound_2 == uint2(0u, 0u))) { break; }
        loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
        if (!loop_init_1) {
            int _e45 = sum;
            if (!((_e45 < int(150)))) {
                break;
            }
        }
        loop_init_1 = false;
        int _e39 = sum;
        sum = asint(asuint(_e39) + asuint(int(100)));
        int _e42 = sum;
        if ((_e42 > int(200))) {
            break;
        }
    }
    int _e51 = sum;
    out_.Store(threadIdx.x*4, asuint(_e51));
}
