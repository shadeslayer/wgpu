RWByteAddressBuffer out_ : register(u0);

int naga_extractBits(
    int e,
    uint offset,
    uint count
) {
    uint w = 32;
    uint o = min(offset, w);
    uint c = min(count, w - o);
    return (c == 0 ? 0 : (e << (w - c - o)) >> (w - c));
}
int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

int signed_conversion(float first_1, float second_1)
{
    float first_2 = (float)0;
    float second_2 = (float)0;

    first_2 = first_1;
    second_2 = second_1;
    float _e4 = first_2;
    float _e5 = second_2;
    return asint((asuint(naga_extractBits(naga_f2i32(_e4), 0u, 24u)) * asuint(naga_extractBits(naga_f2i32(_e5), 0u, 24u))));
}

uint naga_extractBits(
    uint e,
    uint offset,
    uint count
) {
    uint w = 32;
    uint o = min(offset, w);
    uint c = min(count, w - o);
    return (c == 0 ? 0 : (e << (w - c - o)) >> (w - c));
}
uint naga_f2u32(float value) {
    return uint(clamp(value, 0.0, 4294967000.0));
}

uint unsigned_conversion(float first_3, float second_3)
{
    float first_4 = (float)0;
    float second_4 = (float)0;

    first_4 = first_3;
    second_4 = second_3;
    float _e4 = first_4;
    float _e5 = second_4;
    return (naga_extractBits(naga_f2u32(_e4), 0u, 24u) * naga_extractBits(naga_f2u32(_e5), 0u, 24u));
}

int _mul24_(bool first_5, bool second_5)
{
    bool first_6 = (bool)0;
    bool second_6 = (bool)0;
    int local_2 = (int)0;
    int local_3 = (int)0;

    first_6 = first_5;
    second_6 = second_5;
    bool _e4 = first_6;
    if (_e4) {
        bool _e5 = second_6;
        if (_e5) {
            local_2 = int(71);
        } else {
            local_2 = int(73);
        }
        int _e9 = local_2;
        local_3 = _e9;
    } else {
        local_3 = int(79);
    }
    int _e12 = local_3;
    return _e12;
}

int _mul24_1(float first_7, float second_7)
{
    float first_8 = (float)0;
    float second_8 = (float)0;

    first_8 = first_7;
    second_8 = second_7;
    float _e5 = first_8;
    float _e7 = second_8;
    return naga_f2i32(((_e5 * 10.0) + _e7));
}

uint _umul24_(int first_9, int second_9)
{
    int first_10 = (int)0;
    int second_10 = (int)0;

    first_10 = first_9;
    second_10 = second_9;
    int _e4 = first_10;
    int _e5 = second_10;
    return uint(asint(asuint(asint(asuint(_e4) + asuint(_e5))) + asuint(int(100))));
}

int signed_pair(int first_11, int second_11)
{
    int first_12 = (int)0;
    int second_12 = (int)0;

    first_12 = first_11;
    second_12 = second_11;
    int _e4 = first_12;
    int _e5 = second_12;
    return asint((asuint(naga_extractBits(_e4, 0u, 24u)) * asuint(naga_extractBits(_e5, 0u, 24u))));
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

typedef int ret_ZeroValuearray2_int_[2];
ret_ZeroValuearray2_int_ ZeroValuearray2_int_() {
    return (int[2])0;
}

[numthreads(1, 1, 1)]
void multiply24_()
{
    int first = (int)0;
    uint second = (uint)0;
    int local = (int)0;
    int values[2] = (int[2])0;
    int Count = (int)0;
    int local_1 = (int)0;
    int scratch[2] = (int[2])0;
    int counter = (int)0;
    int total = (int)0;
    int index = (int)0;

    out_.Store(int(0)*4, asuint(uint(asint((asuint(naga_extractBits(int(8388608), 0u, 24u)) * asuint(naga_extractBits(int(1), 0u, 24u)))))));
    out_.Store(int(1)*4, asuint((naga_extractBits(16777215u, 0u, 24u) * naga_extractBits(16777215u, 0u, 24u))));
    out_.Store(int(2)*4, asuint(uint(asint((asuint(naga_extractBits(int(8388607), 0u, 24u)) * asuint(naga_extractBits(int(8388607), 0u, 24u)))))));
    out_.Store(int(3)*4, asuint((naga_extractBits(16777216u, 0u, 24u) * naga_extractBits(13u, 0u, 24u))));
    const int _e50 = signed_conversion(3.75, -(2.75));
    out_.Store(int(4)*4, asuint(uint(_e50)));
    const int _e57 = signed_conversion(-(3.75), 2.75);
    out_.Store(int(5)*4, asuint(uint(_e57)));
    const uint _e63 = unsigned_conversion(3.75, 2.75);
    out_.Store(int(6)*4, asuint(_e63));
    const uint _e68 = unsigned_conversion(16777215.0, 1.0);
    out_.Store(int(7)*4, asuint(_e68));
    const int _e73 = _mul24_(true, true);
    out_.Store(int(8)*4, asuint(uint(_e73)));
    const int _e79 = _mul24_(false, true);
    out_.Store(int(9)*4, asuint(uint(_e79)));
    out_.Store(int(10)*4, asuint(uint(asint((asuint(naga_extractBits(int(3), 0u, 24u)) * asuint(naga_extractBits(int(true), 0u, 24u)))))));
    out_.Store(int(11)*4, asuint(uint(asint((asuint(naga_extractBits(int(true), 0u, 24u)) * asuint(naga_extractBits(int(3), 0u, 24u)))))));
    const int _e113 = _mul24_1(3.75, 2.0);
    out_.Store(int(12)*4, asuint(uint(_e113)));
    const uint _e119 = _umul24_(int(3), int(4));
    out_.Store(int(13)*4, asuint(_e119));
    out_.Store(int(14)*4, asuint((naga_extractBits(3u, 0u, 24u) * naga_extractBits(4u, 0u, 24u))));
    const uint _e135 = _umul24_(int(true), int(true));
    out_.Store(int(15)*4, asuint(_e135));
    out_.Store(int(16)*4, asuint((naga_extractBits(3u, 0u, 24u) * naga_extractBits(uint(int(4)), 0u, 24u))));
    const int _e151 = signed_pair(naga_neg(int(7)), int(3));
    out_.Store(int(17)*4, asuint(uint(_e151)));
    out_.Store(int(18)*4, asuint(uint(asint((asuint(naga_extractBits(asint((asuint(naga_extractBits(int(3), 0u, 24u)) * asuint(naga_extractBits(int(4), 0u, 24u)))), 0u, 24u)) * asuint(naga_extractBits(int(5), 0u, 24u)))))));
    out_.Store(int(19)*4, asuint((naga_extractBits((naga_extractBits(16777215u, 0u, 24u) * naga_extractBits(2u, 0u, 24u)), 0u, 24u) * naga_extractBits(2u, 0u, 24u))));
    first = int(-3);
    second = 5u;
    int _e196 = first;
    uint _e197 = second;
    out_.Store(int(20)*4, asuint(uint(asint((asuint(naga_extractBits(_e196, 0u, 24u)) * asuint(naga_extractBits(int(_e197), 0u, 24u)))))));
    int _e210 = first;
    uint _e212 = second;
    out_.Store(int(21)*4, asuint((naga_extractBits(uint(_e210), 0u, 24u) * naga_extractBits(_e212, 0u, 24u))));
    out_.Store(int(22)*4, asuint(uint(asint((asuint(naga_extractBits(int(25165824), 0u, 24u)) * asuint(naga_extractBits(int(16777217), 0u, 24u)))))));
    out_.Store(int(23)*4, asuint((naga_extractBits(4286578688u, 0u, 24u) * naga_extractBits(4278190081u, 0u, 24u))));
    if (true) {
        local = asint((asuint(naga_extractBits(naga_neg(int(3)), 0u, 24u)) * asuint(naga_extractBits(int(7), 0u, 24u))));
    } else {
        local = asint((asuint(naga_extractBits(int(0), 0u, 24u)) * asuint(naga_extractBits(int(0), 0u, 24u))));
    }
    int _e265 = local;
    out_.Store(int(24)*4, asuint(uint(_e265)));
    values[0] = asint((asuint(naga_extractBits(int(2), 0u, 24u)) * asuint(naga_extractBits(int(3), 0u, 24u))));
    values[1] = asint((asuint(naga_extractBits(int(3), 0u, 24u)) * asuint(naga_extractBits(int(4), 0u, 24u))));
    int _e296 = values[int(0)];
    int _e297 = values[int(1)];
    out_.Store(int(25)*4, asuint(uint(asint(asuint(_e296) + asuint(_e297)))));
    Count = int(2);
    scratch = ZeroValuearray2_int_();
    int _e308 = Count;
    int _e309 = scratch[int(0)];
    out_.Store(int(26)*4, asuint(uint(asint(asuint(_e308) + asuint(_e309)))));
    counter = int(2);
    int _e316 = counter;
    counter = asint(asuint(_e316) + asuint(int(1)));
    int _e319 = counter;
    counter = asint(asuint(_e319) + asuint(int(1)));
    out_.Store(int(27)*4, asuint(uint(asint((asuint(naga_extractBits(_e316, 0u, 24u)) * asuint(naga_extractBits(_e319, 0u, 24u)))))));
    int _e333 = counter;
    out_.Store(int(28)*4, asuint(uint(_e333)));
    total = int(0);
    {
        index = int(0);
        uint2 loop_bound = uint2(4294967295u, 4294967295u);
        bool loop_init = true;
        while(true) {
            if (all(loop_bound == uint2(0u, 0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e342 = index;
                index = asint(asuint(_e342) + asuint(int(1)));
            }
            loop_init = false;
            int _e340 = index;
            if ((_e340 < int(3))) {
            } else {
                break;
            }
            int _e346 = index;
            int _e357 = total;
            total = asint(asuint(_e357) + asuint(asint((asuint(naga_extractBits(asint(asuint(_e346) + asuint(int(1))), 0u, 24u)) * asuint(naga_extractBits(int(2), 0u, 24u))))));
        }
    }
    int _e361 = total;
    out_.Store(int(29)*4, asuint(uint(_e361)));
    {
        out_.Store(int(30)*4, asuint(uint(naga_f2i32(9.75))));
    }
    out_.Store(int(31)*4, asuint(uint(asint((asuint(naga_extractBits(int(16777215), 0u, 24u)) * asuint(naga_extractBits(int(3), 0u, 24u)))))));
}
