struct Sample_cuda_record_0_ {
    int value;
    int payload_[2];
};

RWByteAddressBuffer out_ : register(u0);

Sample_cuda_record_0_ operator_u0026_u0026_(Sample_cuda_record_0_ left_1, Sample_cuda_record_0_ right_1)
{
    Sample_cuda_record_0_ left_2 = (Sample_cuda_record_0_)0;
    Sample_cuda_record_0_ right_2 = (Sample_cuda_record_0_)0;

    left_2 = left_1;
    right_2 = right_1;
    int _e7 = left_2.value;
    int _e10 = right_2.value;
    left_2.value = asint(asuint(asint(asuint(_e7) * asuint(int(10)))) + asuint(_e10));
    int _e18 = right_2.payload_[int(0)];
    int _e19 = left_2.payload_[int(1)];
    left_2.payload_[int(1)] = asint(asuint(_e19) + asuint(_e18));
    Sample_cuda_record_0_ _e21 = left_2;
    const Sample_cuda_record_0_ sample_cuda_record_0_ = _e21;
    return sample_cuda_record_0_;
}

Sample_cuda_record_0_ operator_u007c_u007c(Sample_cuda_record_0_ left_3, Sample_cuda_record_0_ right_3)
{
    Sample_cuda_record_0_ left_4 = (Sample_cuda_record_0_)0;
    Sample_cuda_record_0_ right_4 = (Sample_cuda_record_0_)0;

    left_4 = left_3;
    right_4 = right_3;
    int _e7 = left_4.value;
    int _e10 = right_4.value;
    left_4.value = asint(asuint(asint(asuint(_e7) * asuint(int(100)))) + asuint(_e10));
    int _e18 = right_4.payload_[int(1)];
    int _e19 = left_4.payload_[int(0)];
    left_4.payload_[int(0)] = asint(asuint(_e19) + asuint(_e18));
    Sample_cuda_record_0_ _e21 = left_4;
    const Sample_cuda_record_0_ sample_cuda_record_0_1 = _e21;
    return sample_cuda_record_0_1;
}

int operator_u0026_u0026_1(Sample_cuda_record_0_ left_5, int right_5)
{
    Sample_cuda_record_0_ left_6 = (Sample_cuda_record_0_)0;
    int right_6 = (int)0;

    left_6 = left_5;
    right_6 = right_5;
    int _e6 = left_6.value;
    int _e8 = right_6;
    return asint(asuint(asint(asuint(_e6) * asuint(int(10)))) + asuint(_e8));
}

int operator_u007c_u007c_1(int left_7, Sample_cuda_record_0_ right_7)
{
    int left_8 = (int)0;
    Sample_cuda_record_0_ right_8 = (Sample_cuda_record_0_)0;

    left_8 = left_7;
    right_8 = right_7;
    int _e5 = left_8;
    int _e8 = right_8.value;
    return asint(asuint(asint(asuint(_e5) * asuint(int(100)))) + asuint(_e8));
}

int2 operator_u0026_u0026_2(int2 left_9, int2 right_9)
{
    int2 left_10 = (int2)0;
    int2 right_10 = (int2)0;

    left_10 = left_9;
    right_10 = right_9;
    int _e6 = left_10.x;
    int _e9 = right_10.x;
    int _e13 = left_10.y;
    int _e16 = right_10.y;
    return int2(asint(asuint(asint(asuint(_e6) * asuint(int(10)))) + asuint(_e9)), asint(asuint(asint(asuint(_e13) * asuint(int(10)))) + asuint(_e16)));
}

int2 operator_u007c_u007c_2(int2 left_11, int2 right_11)
{
    int2 left_12 = (int2)0;
    int2 right_12 = (int2)0;

    left_12 = left_11;
    right_12 = right_11;
    int _e6 = left_12.x;
    int _e9 = right_12.x;
    int _e13 = left_12.y;
    int _e16 = right_12.y;
    return int2(asint(asuint(asint(asuint(_e6) * asuint(int(100)))) + asuint(_e9)), asint(asuint(asint(asuint(_e13) * asuint(int(100)))) + asuint(_e16)));
}

[numthreads(2, 1, 1)]
void logical_operators(uint3 threadIdx : SV_GroupThreadID)
{
    uint base = (uint)0;
    Sample_cuda_record_0_ left = (Sample_cuda_record_0_)0;
    Sample_cuda_record_0_ right = (Sample_cuda_record_0_)0;
    Sample_cuda_record_0_ both = (Sample_cuda_record_0_)0;
    Sample_cuda_record_0_ either = (Sample_cuda_record_0_)0;
    int2 first = (int2)0;
    int2 second = (int2)0;
    Sample_cuda_record_0_ local = (Sample_cuda_record_0_)0;
    Sample_cuda_record_0_ local_1 = (Sample_cuda_record_0_)0;
    int counter = (int)0;
    Sample_cuda_record_0_ evaluated = (Sample_cuda_record_0_)0;
    Sample_cuda_record_0_ local_2 = (Sample_cuda_record_0_)0;
    Sample_cuda_record_0_ local_3 = (Sample_cuda_record_0_)0;
    Sample_cuda_record_0_ local_4 = (Sample_cuda_record_0_)0;
    bool local_5 = (bool)0;
    bool local_6 = (bool)0;
    bool local_7 = (bool)0;
    bool local_8 = (bool)0;

    base = (threadIdx.x * 24u);
    left.value = int(2);
    left.payload_[0] = int(3);
    left.payload_[1] = int(4);
    right.value = int(5);
    right.payload_[0] = int(6);
    right.payload_[1] = int(7);
    Sample_cuda_record_0_ _e23 = left;
    Sample_cuda_record_0_ _e24 = right;
    const Sample_cuda_record_0_ _e25 = operator_u0026_u0026_(_e23, _e24);
    both = _e25;
    Sample_cuda_record_0_ _e27 = left;
    Sample_cuda_record_0_ _e28 = right;
    const Sample_cuda_record_0_ _e29 = operator_u007c_u007c(_e27, _e28);
    either = _e29;
    int _e35 = both.value;
    uint _e31 = base;
    out_.Store((_e31 + 0u)*4, asuint(uint(_e35)));
    int _e44 = both.payload_[int(1)];
    uint _e38 = base;
    out_.Store((_e38 + 1u)*4, asuint(uint(_e44)));
    int _e51 = either.value;
    uint _e47 = base;
    out_.Store((_e47 + 2u)*4, asuint(uint(_e51)));
    int _e60 = either.payload_[int(0)];
    uint _e54 = base;
    out_.Store((_e54 + 3u)*4, asuint(uint(_e60)));
    int _e67 = left.value;
    uint _e63 = base;
    out_.Store((_e63 + 4u)*4, asuint(uint(_e67)));
    int _e76 = left.payload_[int(1)];
    uint _e70 = base;
    out_.Store((_e70 + 5u)*4, asuint(uint(_e76)));
    Sample_cuda_record_0_ _e82 = left;
    const int _e84 = operator_u0026_u0026_1(_e82, int(9));
    uint _e79 = base;
    out_.Store((_e79 + 6u)*4, asuint(uint(_e84)));
    Sample_cuda_record_0_ _e91 = right;
    const int _e92 = operator_u007c_u007c_1(int(0), _e91);
    uint _e87 = base;
    out_.Store((_e87 + 7u)*4, asuint(uint(_e92)));
    Sample_cuda_record_0_ _e98 = left;
    Sample_cuda_record_0_ _e99 = right;
    const Sample_cuda_record_0_ _e100 = operator_u0026_u0026_(_e98, _e99);
    uint _e95 = base;
    out_.Store((_e95 + 8u)*4, asuint(uint(_e100.value)));
    Sample_cuda_record_0_ _e107 = left;
    Sample_cuda_record_0_ _e108 = right;
    const Sample_cuda_record_0_ _e109 = operator_u007c_u007c(_e107, _e108);
    uint _e104 = base;
    out_.Store((_e104 + 9u)*4, asuint(uint(_e109.payload_[int(0)])));
    first = int2(int(2), int(3));
    second = int2(int(4), int(5));
    int2 _e126 = first;
    int2 _e127 = second;
    const int2 _e128 = operator_u0026_u0026_2(_e126, _e127);
    uint _e123 = base;
    out_.Store((_e123 + 10u)*4, asuint(uint(_e128.x)));
    int2 _e135 = first;
    int2 _e136 = second;
    const int2 _e137 = operator_u007c_u007c_2(_e135, _e136);
    uint _e132 = base;
    out_.Store((_e132 + 11u)*4, asuint(uint(_e137.y)));
    int2 _e144 = first;
    int2 _e145 = second;
    const int2 _e146 = operator_u0026_u0026_2(_e144, _e145);
    int2 _e147 = first;
    const int2 _e148 = operator_u0026_u0026_2(_e146, _e147);
    uint _e141 = base;
    out_.Store((_e141 + 12u)*4, asuint(uint(_e148.x)));
    int2 _e155 = first;
    int2 _e156 = second;
    int2 _e157 = first;
    const int2 _e158 = operator_u0026_u0026_2(_e156, _e157);
    const int2 _e159 = operator_u007c_u007c_2(_e155, _e158);
    uint _e152 = base;
    out_.Store((_e152 + 13u)*4, asuint(uint(_e159.y)));
    int2 _e166 = first;
    int2 _e167 = second;
    const int2 _e168 = operator_u007c_u007c_2(_e166, _e167);
    int2 _e169 = first;
    const int2 _e170 = operator_u0026_u0026_2(_e168, _e169);
    uint _e163 = base;
    out_.Store((_e163 + 14u)*4, asuint(uint(_e170.x)));
    if (true) {
        Sample_cuda_record_0_ _e179 = left;
        local = _e179;
    } else {
        Sample_cuda_record_0_ _e180 = right;
        local = _e180;
    }
    Sample_cuda_record_0_ _e181 = local;
    const int _e183 = operator_u0026_u0026_1(_e181, int(1));
    uint _e174 = base;
    out_.Store((_e174 + 15u)*4, asuint(uint(_e183)));
    if (true) {
        Sample_cuda_record_0_ _e190 = left;
        Sample_cuda_record_0_ _e191 = right;
        const Sample_cuda_record_0_ _e192 = operator_u0026_u0026_(_e190, _e191);
        local_1 = _e192;
    } else {
        Sample_cuda_record_0_ _e193 = right;
        Sample_cuda_record_0_ _e194 = left;
        const Sample_cuda_record_0_ _e195 = operator_u007c_u007c(_e193, _e194);
        local_1 = _e195;
    }
    Sample_cuda_record_0_ _e197 = local_1;
    uint _e186 = base;
    out_.Store((_e186 + 16u)*4, asuint(uint(_e197.value)));
    counter = int(0);
    if (false) {
        int _e205 = counter;
        counter = asint(asuint(_e205) + asuint(int(1)));
        local_2.value = _e205;
        local_2.payload_[0] = int(1);
        local_2.payload_[1] = int(2);
        Sample_cuda_record_0_ _e214 = local_2;
        Sample_cuda_record_0_ _e215 = right;
        const Sample_cuda_record_0_ _e216 = operator_u0026_u0026_(_e214, _e215);
        local_4 = _e216;
    } else {
        Sample_cuda_record_0_ _e217 = left;
        int _e219 = counter;
        counter = asint(asuint(_e219) + asuint(int(1)));
        local_3.value = _e219;
        local_3.payload_[0] = int(1);
        local_3.payload_[1] = int(2);
        Sample_cuda_record_0_ _e228 = local_3;
        const Sample_cuda_record_0_ _e229 = operator_u007c_u007c(_e217, _e228);
        local_4 = _e229;
    }
    Sample_cuda_record_0_ _e231 = local_4;
    evaluated = _e231;
    int _e237 = evaluated.value;
    uint _e233 = base;
    out_.Store((_e233 + 17u)*4, asuint(uint(_e237)));
    int _e243 = counter;
    uint _e240 = base;
    out_.Store((_e240 + 18u)*4, asuint(uint(_e243)));
    local_5 = false;
    if (false) {
        Sample_cuda_record_0_ _e250 = left;
        Sample_cuda_record_0_ _e251 = right;
        const Sample_cuda_record_0_ _e252 = operator_u0026_u0026_(_e250, _e251);
        local_5 = (_e252.value != int(0));
    }
    bool _e257 = local_5;
    uint _e246 = base;
    out_.Store((_e246 + 19u)*4, asuint(uint(_e257)));
    local_6 = true;
    if (true) {
    } else {
        Sample_cuda_record_0_ _e264 = left;
        Sample_cuda_record_0_ _e265 = right;
        const Sample_cuda_record_0_ _e266 = operator_u007c_u007c(_e264, _e265);
        local_6 = (_e266.value != int(0));
    }
    bool _e271 = local_6;
    uint _e260 = base;
    out_.Store((_e260 + 20u)*4, asuint(uint(_e271)));
    bool _e280 = (int(2) != int(0));
    local_7 = _e280;
    if (_e280) {
        local_7 = (int(3) != int(0));
    }
    bool _e284 = local_7;
    uint _e274 = base;
    out_.Store((_e274 + 21u)*4, asuint(uint(_e284)));
    bool _e293 = (int(0) != int(0));
    local_8 = _e293;
    if (_e293) {
    } else {
        local_8 = (int(4) != int(0));
    }
    bool _e297 = local_8;
    uint _e287 = base;
    out_.Store((_e287 + 22u)*4, asuint(uint(_e297)));
    uint _e300 = base;
    out_.Store((_e300 + 23u)*4, asuint(threadIdx.x));
}
