struct Record_cuda_record_0_ {
    int value;
    int history[2];
};

ByteAddressBuffer input : register(t0);
RWByteAddressBuffer out_ : register(u1);

int2 operator_u002b(int2 value)
{
    int2 value_1 = (int2)0;

    value_1 = value;
    int _e3 = value_1.x;
    int _e5 = value_1.y;
    return int2(_e3, _e5);
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

int2 operator_u002d(int2 value_2)
{
    int2 value_3 = (int2)0;

    value_3 = value_2;
    int _e3 = value_3.x;
    int _e6 = value_3.y;
    return int2(naga_neg(_e3), naga_neg(_e6));
}

int2 operator_u007e(int2 value_4)
{
    int2 value_5 = (int2)0;

    value_5 = value_4;
    int _e3 = value_5.x;
    int _e6 = value_5.y;
    return int2(~(_e3), ~(_e6));
}

bool operator_u0021_(int2 value_6)
{
    int2 value_7 = (int2)0;
    bool local_1 = (bool)0;

    value_7 = value_6;
    int _e4 = value_7.x;
    bool _e5 = (_e4 == int(0));
    local_1 = _e5;
    if (_e5) {
        int _e9 = value_7.y;
        local_1 = (_e9 == int(0));
    }
    bool _e11 = local_1;
    return _e11;
}

int operator_u002a(int2 value_8)
{
    int2 value_9 = (int2)0;

    value_9 = value_8;
    int _e4 = value_9.x;
    int _e7 = value_9.y;
    return asint(asuint(asint(asuint(_e4) * asuint(int(10)))) + asuint(_e7));
}

int2 operator_u0026_(int2 value_10)
{
    int2 value_11 = (int2)0;

    value_11 = value_10;
    int _e3 = value_11.y;
    int _e5 = value_11.x;
    return int2(_e3, _e5);
}

int2 operator_u002b_1(int2 left_1, int2 right_1)
{
    int2 left_2 = (int2)0;
    int2 right_2 = (int2)0;

    left_2 = left_1;
    right_2 = right_1;
    int _e6 = left_2.x;
    int _e7 = right_2.x;
    int _e11 = left_2.y;
    int _e12 = right_2.y;
    return int2(asint(asuint(_e6) + asuint(_e7)), asint(asuint(_e11) + asuint(_e12)));
}

int2 operator_u002d_1(int2 left_3, int2 right_3)
{
    int2 left_4 = (int2)0;
    int2 right_4 = (int2)0;

    left_4 = left_3;
    right_4 = right_3;
    int _e6 = left_4.x;
    int _e7 = right_4.x;
    int _e11 = left_4.y;
    int _e12 = right_4.y;
    return int2(asint(asuint(_e6) - asuint(_e7)), asint(asuint(_e11) - asuint(_e12)));
}

int2 operator_u002a_1(int2 left_5, int2 right_5)
{
    int2 left_6 = (int2)0;
    int2 right_6 = (int2)0;

    left_6 = left_5;
    right_6 = right_5;
    int _e6 = left_6.x;
    int _e7 = right_6.x;
    int _e11 = left_6.y;
    int _e12 = right_6.y;
    return int2(asint(asuint(_e6) * asuint(_e7)), asint(asuint(_e11) * asuint(_e12)));
}

int naga_div(int lhs, int rhs) {
    return lhs / (((lhs == int(-2147483647 - 1) & rhs == -1) | (rhs == 0)) ? 1 : rhs);
}

int2 operator_u002f(int2 left_7, int2 right_7)
{
    int2 left_8 = (int2)0;
    int2 right_8 = (int2)0;

    left_8 = left_7;
    right_8 = right_7;
    int _e6 = left_8.x;
    int _e7 = right_8.x;
    int _e11 = left_8.y;
    int _e12 = right_8.y;
    return int2(naga_div(_e6, _e7), naga_div(_e11, _e12));
}

int naga_mod(int lhs, int rhs) {
    int divisor = ((lhs == int(-2147483647 - 1) & rhs == -1) | (rhs == 0)) ? 1 : rhs;
    return lhs - (lhs / divisor) * divisor;
}

int2 operator_u0025_(int2 left_9, int2 right_9)
{
    int2 left_10 = (int2)0;
    int2 right_10 = (int2)0;

    left_10 = left_9;
    right_10 = right_9;
    int _e6 = left_10.x;
    int _e7 = right_10.x;
    int _e11 = left_10.y;
    int _e12 = right_10.y;
    return int2(naga_mod(_e6, _e7), naga_mod(_e11, _e12));
}

int2 operator_u0026_1(int2 left_11, int2 right_11)
{
    int2 left_12 = (int2)0;
    int2 right_12 = (int2)0;

    left_12 = left_11;
    right_12 = right_11;
    int _e6 = left_12.x;
    int _e7 = right_12.x;
    int _e11 = left_12.y;
    int _e12 = right_12.y;
    return int2((_e6 & _e7), (_e11 & _e12));
}

int2 operator_u007c(int2 left_13, int2 right_13)
{
    int2 left_14 = (int2)0;
    int2 right_14 = (int2)0;

    left_14 = left_13;
    right_14 = right_13;
    int _e6 = left_14.x;
    int _e7 = right_14.x;
    int _e11 = left_14.y;
    int _e12 = right_14.y;
    return int2((_e6 | _e7), (_e11 | _e12));
}

int2 operator_u005e(int2 left_15, int2 right_15)
{
    int2 left_16 = (int2)0;
    int2 right_16 = (int2)0;

    left_16 = left_15;
    right_16 = right_15;
    int _e6 = left_16.x;
    int _e7 = right_16.x;
    int _e11 = left_16.y;
    int _e12 = right_16.y;
    return int2((_e6 ^ _e7), (_e11 ^ _e12));
}

int2 operator_(int2 left_17, int2 right_17)
{
    int2 left_18 = (int2)0;
    int2 right_18 = (int2)0;
    uint local_2 = (uint)0;
    uint local_3 = (uint)0;

    left_18 = left_17;
    right_18 = right_17;
    int _e5 = left_18.x;
    int _e7 = right_18.x;
    local_2 = uint(_e7);
    uint _e10 = local_2;
    int _e13 = left_18.y;
    int _e15 = right_18.y;
    local_3 = uint(_e15);
    uint _e18 = local_3;
    return int2((_e5 << _e10), (_e13 << _e18));
}

int2 operator_1(int2 left_19, int2 right_19)
{
    int2 left_20 = (int2)0;
    int2 right_20 = (int2)0;

    left_20 = left_19;
    right_20 = right_19;
    int _e5 = left_20.x;
    int _e7 = right_20.x;
    int _e11 = left_20.y;
    int _e13 = right_20.y;
    return int2((_e5 >> uint(_e7)), (_e11 >> uint(_e13)));
}

Record_cuda_record_0_ operator_u002a_2(Record_cuda_record_0_ value_12)
{
    Record_cuda_record_0_ value_13 = (Record_cuda_record_0_)0;

    value_13 = value_12;
    int _e3 = value_13.value;
    value_13.value = asint(asuint(_e3) + asuint(int(1)));
    Record_cuda_record_0_ _e6 = value_13;
    const Record_cuda_record_0_ record_cuda_record_0_ = _e6;
    return record_cuda_record_0_;
}

int operator_u0026_2(Record_cuda_record_0_ value_14)
{
    Record_cuda_record_0_ value_15 = (Record_cuda_record_0_)0;

    value_15 = value_14;
    int _e3 = value_15.value;
    return _e3;
}

Record_cuda_record_0_ operator_u002d_2(Record_cuda_record_0_ value_16)
{
    Record_cuda_record_0_ value_17 = (Record_cuda_record_0_)0;

    value_17 = value_16;
    int _e4 = value_17.value;
    value_17.value = naga_neg(_e4);
    Record_cuda_record_0_ _e6 = value_17;
    const Record_cuda_record_0_ record_cuda_record_0_1 = _e6;
    return record_cuda_record_0_1;
}

Record_cuda_record_0_ operator_u002a_3(Record_cuda_record_0_ value_18, int factor)
{
    Record_cuda_record_0_ value_19 = (Record_cuda_record_0_)0;
    int factor_1 = (int)0;

    value_19 = value_18;
    factor_1 = factor;
    int _e5 = factor_1;
    int _e6 = value_19.value;
    value_19.value = asint(asuint(_e6) * asuint(_e5));
    int _e11 = factor_1;
    int _e12 = value_19.history[int(1)];
    value_19.history[int(1)] = asint(asuint(_e12) + asuint(_e11));
    Record_cuda_record_0_ _e14 = value_19;
    const Record_cuda_record_0_ record_cuda_record_0_2 = _e14;
    return record_cuda_record_0_2;
}

[numthreads(2, 1, 1)]
void operator_functions(uint3 threadIdx : SV_GroupThreadID)
{
    uint base = (uint)0;
    int2 left = (int2)0;
    int2 right = (int2)0;
    Record_cuda_record_0_ original = (Record_cuda_record_0_)0;
    Record_cuda_record_0_ scaled = (Record_cuda_record_0_)0;
    Record_cuda_record_0_ local = (Record_cuda_record_0_)0;

    base = (threadIdx.x * 36u);
    int2 _e10 = asint(input.Load2(threadIdx.x*8));
    left = _e10;
    right = int2(int(3), int(2));
    int2 _e19 = left;
    const int2 _e20 = operator_u002b(_e19);
    uint _e16 = base;
    out_.Store((_e16 + 0u)*4, asuint(uint(_e20.x)));
    int2 _e27 = left;
    const int2 _e28 = operator_u002d(_e27);
    uint _e24 = base;
    out_.Store((_e24 + 1u)*4, asuint(uint(_e28.y)));
    int2 _e35 = left;
    const int2 _e36 = operator_u007e(_e35);
    uint _e32 = base;
    out_.Store((_e32 + 2u)*4, asuint(uint(_e36.x)));
    int2 _e43 = left;
    const bool _e44 = operator_u0021_(_e43);
    uint _e40 = base;
    out_.Store((_e40 + 3u)*4, asuint(uint(_e44)));
    int2 _e50 = left;
    const int _e51 = operator_u002a(_e50);
    uint _e47 = base;
    out_.Store((_e47 + 4u)*4, asuint(uint(_e51)));
    int2 _e57 = left;
    const int2 _e58 = operator_u0026_(_e57);
    uint _e54 = base;
    out_.Store((_e54 + 5u)*4, asuint(uint(_e58.x)));
    int2 _e65 = left;
    int2 _e66 = right;
    const int2 _e67 = operator_u002b_1(_e65, _e66);
    uint _e62 = base;
    out_.Store((_e62 + 6u)*4, asuint(uint(_e67.x)));
    int2 _e74 = left;
    int2 _e75 = right;
    const int2 _e76 = operator_u002d_1(_e74, _e75);
    uint _e71 = base;
    out_.Store((_e71 + 7u)*4, asuint(uint(_e76.y)));
    int2 _e83 = left;
    int2 _e84 = right;
    const int2 _e85 = operator_u002a_1(_e83, _e84);
    uint _e80 = base;
    out_.Store((_e80 + 8u)*4, asuint(uint(_e85.x)));
    int2 _e92 = left;
    int2 _e93 = right;
    const int2 _e94 = operator_u002f(_e92, _e93);
    uint _e89 = base;
    out_.Store((_e89 + 9u)*4, asuint(uint(_e94.y)));
    int2 _e101 = left;
    int2 _e102 = right;
    const int2 _e103 = operator_u0025_(_e101, _e102);
    uint _e98 = base;
    out_.Store((_e98 + 10u)*4, asuint(uint(_e103.y)));
    int2 _e110 = left;
    int2 _e111 = right;
    const int2 _e112 = operator_u0026_1(_e110, _e111);
    uint _e107 = base;
    out_.Store((_e107 + 11u)*4, asuint(uint(_e112.x)));
    int2 _e119 = left;
    int2 _e120 = right;
    const int2 _e121 = operator_u007c(_e119, _e120);
    uint _e116 = base;
    out_.Store((_e116 + 12u)*4, asuint(uint(_e121.y)));
    int2 _e128 = left;
    int2 _e129 = right;
    const int2 _e130 = operator_u005e(_e128, _e129);
    uint _e125 = base;
    out_.Store((_e125 + 13u)*4, asuint(uint(_e130.x)));
    int2 _e137 = left;
    int2 _e138 = right;
    const int2 _e139 = operator_(_e137, _e138);
    uint _e134 = base;
    out_.Store((_e134 + 14u)*4, asuint(uint(_e139.x)));
    int2 _e146 = left;
    int2 _e147 = right;
    const int2 _e148 = operator_1(_e146, _e147);
    uint _e143 = base;
    out_.Store((_e143 + 15u)*4, asuint(uint(_e148.y)));
    int2 _e155 = left;
    const int2 _e156 = operator_u002b(_e155);
    uint _e152 = base;
    out_.Store((_e152 + 16u)*4, asuint(uint(_e156.y)));
    int2 _e163 = left;
    int2 _e164 = right;
    const int2 _e165 = operator_u002d_1(_e163, _e164);
    uint _e160 = base;
    out_.Store((_e160 + 17u)*4, asuint(uint(_e165.x)));
    int2 _e172 = left;
    const int _e173 = operator_u002a(_e172);
    uint _e169 = base;
    out_.Store((_e169 + 18u)*4, asuint(uint(_e173)));
    int2 _e179 = left;
    int2 _e180 = right;
    const int2 _e181 = operator_u0026_1(_e179, _e180);
    uint _e176 = base;
    out_.Store((_e176 + 19u)*4, asuint(uint(_e181.y)));
    int2 _e188 = left;
    const int2 _e189 = operator_u0026_(_e188);
    const int _e190 = operator_u002a(_e189);
    uint _e185 = base;
    out_.Store((_e185 + 20u)*4, asuint(uint(_e190)));
    int2 _e196 = left;
    const int2 _e197 = operator_u0026_(_e196);
    const int2 _e198 = operator_u0026_(_e197);
    uint _e193 = base;
    out_.Store((_e193 + 21u)*4, asuint(uint(_e198.x)));
    int2 _e208 = left;
    int2 _e205 = right;
    int2 _e206 = right;
    const int2 _e207 = operator_u002a_1(_e205, _e206);
    const int2 _e209 = operator_u002b_1(_e208, _e207);
    uint _e202 = base;
    out_.Store((_e202 + 22u)*4, asuint(uint(_e209.x)));
    int2 _e216 = left;
    int2 _e220 = right;
    const int2 _e221 = operator_u002b_1(_e220, int2(int(1), int(1)));
    const int2 _e222 = operator_(_e216, _e221);
    uint _e213 = base;
    out_.Store((_e213 + 23u)*4, asuint(uint(_e222.x)));
    int2 _e234 = left;
    int2 _e232 = right;
    const int2 _e233 = operator_u0026_1(_e232, int2(int(1), int(1)));
    const int2 _e235 = operator_u007c(_e234, _e233);
    uint _e226 = base;
    out_.Store((_e226 + 24u)*4, asuint(uint(_e235.y)));
    int2 _e242 = left;
    int2 _e243 = right;
    const int2 _e244 = operator_u002d_1(_e242, _e243);
    int2 _e245 = right;
    const int2 _e246 = operator_u002d_1(_e244, _e245);
    uint _e239 = base;
    out_.Store((_e239 + 25u)*4, asuint(uint(_e246.x)));
    original.value = int(9);
    original.history[0] = int(1);
    original.history[1] = int(2);
    Record_cuda_record_0_ _e259 = original;
    const Record_cuda_record_0_ _e260 = operator_u002a_3(_e259, int(3));
    scaled = _e260;
    local.value = int(9);
    local.history[0] = int(1);
    local.history[1] = int(2);
    Record_cuda_record_0_ _e273 = local;
    const int _e274 = operator_u0026_2(_e273);
    uint _e262 = base;
    out_.Store((_e262 + 26u)*4, asuint(uint(_e274)));
    Record_cuda_record_0_ _e280 = original;
    const Record_cuda_record_0_ _e281 = operator_u002a_2(_e280);
    const int _e282 = operator_u0026_2(_e281);
    uint _e277 = base;
    out_.Store((_e277 + 27u)*4, asuint(uint(_e282)));
    int _e289 = scaled.value;
    uint _e285 = base;
    out_.Store((_e285 + 28u)*4, asuint(uint(_e289)));
    int _e298 = scaled.history[int(1)];
    uint _e292 = base;
    out_.Store((_e292 + 29u)*4, asuint(uint(_e298)));
    int _e305 = original.value;
    uint _e301 = base;
    out_.Store((_e301 + 30u)*4, asuint(uint(_e305)));
    int _e314 = original.history[int(1)];
    uint _e308 = base;
    out_.Store((_e308 + 31u)*4, asuint(uint(_e314)));
    Record_cuda_record_0_ _e320 = original;
    const Record_cuda_record_0_ _e321 = operator_u002d_2(_e320);
    uint _e317 = base;
    out_.Store((_e317 + 32u)*4, asuint(uint(_e321.value)));
    Record_cuda_record_0_ _e328 = original;
    const Record_cuda_record_0_ _e330 = operator_u002a_3(_e328, int(2));
    uint _e325 = base;
    out_.Store((_e325 + 33u)*4, asuint(uint(_e330.value)));
    Record_cuda_record_0_ _e337 = original;
    const Record_cuda_record_0_ _e338 = operator_u002a_2(_e337);
    const int _e339 = operator_u0026_2(_e338);
    uint _e334 = base;
    out_.Store((_e334 + 34u)*4, asuint(uint(_e339)));
    const bool _e348 = operator_u0021_(int2(int(0), int(0)));
    uint _e342 = base;
    out_.Store((_e342 + 35u)*4, asuint(uint(_e348)));
}
