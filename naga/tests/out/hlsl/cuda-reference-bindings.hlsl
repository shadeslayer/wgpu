struct Record_cuda_record_0_ {
    int value;
    int samples[2];
};

RWByteAddressBuffer out_ : register(u0);

int adjust(int value_1)
{
    int value_2 = (int)0;
    int local_20 = (int)0;

    value_2 = value_1;
    int _e3 = value_2;
    value_2 = asint(asuint(_e3) + asuint(int(2)));
    int _e6 = value_2;
    local_20 = _e6;
    int _e7 = local_20;
    return _e7;
}

int ZeroValueint() {
    return (int)0;
}

uint naga_f2u32(float value) {
    return uint(clamp(value, 0.0, 4294967000.0));
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

[numthreads(1, 1, 1)]
void reference_bindings()
{
    int value = (int)0;
    int local = (int)0;
    int local_1 = (int)0;
    int local_2 = (int)0;
    int left = (int)0;
    int right = (int)0;
    int local_3 = (int)0;
    int source = (int)0;
    float local_4 = (float)0;
    float local_5 = (float)0;
    int local_6 = (int)0;
    int uninitialized = (int)0;
    uint local_7 = (uint)0;
    int original = (int)0;
    int replacement = (int)0;
    int local_8 = (int)0;
    int local_9 = (int)0;
    float local_10 = (float)0;
    int local_11 = (int)0;
    int local_12 = (int)0;
    float local_13 = (float)0;
    int local_14 = (int)0;
    int updated = (int)0;
    int local_15 = (int)0;
    int local_16 = (int)0;
    int comma_left = (int)0;
    int comma_right = (int)0;
    int counter = (int)0;
    bool enabled = (bool)0;
    uint bits = (uint)0;
    uint local_17 = (uint)0;
    float floating = (float)0;
    Record_cuda_record_0_ record = (Record_cuda_record_0_)0;
    int local_18 = (int)0;
    int local_19 = (int)0;

    value = int(1);
    value = int(5);
    int _e5 = value;
    value = asint(asuint(_e5) + asuint(int(2)));
    int _e8 = value;
    local = _e8;
    int _e9 = local;
    int _e10 = value;
    value = asint(asuint(_e10) + asuint(_e9));
    int _e15 = value;
    local_1 = _e15;
    int _e16 = local_1;
    out_.Store(int(0)*4, asuint(uint(_e16)));
    int _e21 = value;
    local_2 = _e21;
    int _e22 = local_2;
    out_.Store(int(1)*4, asuint(uint(_e22)));
    int _e26 = value;
    out_.Store(int(2)*4, asuint(uint(_e26)));
    left = int(1);
    right = int(2);
    left = int(5);
    int _e34 = right;
    right = asint(asuint(_e34) + asuint(int(2)));
    int _e37 = right;
    local_3 = _e37;
    int _e38 = local_3;
    int _e39 = left;
    left = asint(asuint(_e39) + asuint(_e38));
    int _e43 = left;
    out_.Store(int(3)*4, asuint(uint(_e43)));
    int _e47 = right;
    out_.Store(int(4)*4, asuint(uint(_e47)));
    source = int(3);
    int _e51 = source;
    local_4 = float(_e51);
    source = int(9);
    float _e58 = local_4;
    local_5 = _e58;
    float _e59 = local_5;
    out_.Store(int(5)*4, asuint(naga_f2u32(_e59)));
    int _e64 = source;
    local_6 = _e64;
    int _e65 = local_6;
    out_.Store(int(6)*4, asuint(uint(_e65)));
    int _e69 = source;
    out_.Store(int(7)*4, asuint(uint(_e69)));
    uninitialized = int(7);
    local_7 = uint(int(8));
    int _e76 = uninitialized;
    uint _e78 = local_7;
    out_.Store(_e78*4, asuint(uint(_e76)));
    original = int(2);
    replacement = int(8);
    int _e84 = replacement;
    original = _e84;
    replacement = int(10);
    int _e88 = original;
    out_.Store(int(9)*4, asuint(uint(_e88)));
    int _e93 = original;
    local_8 = _e93;
    int _e94 = local_8;
    out_.Store(int(10)*4, asuint(uint(_e94)));
    int _e98 = replacement;
    out_.Store(int(11)*4, asuint(uint(_e98)));
    local_9 = ZeroValueint();
    local_10 = 1.5;
    local_11 = naga_f2i32(3.75);
    int _e110 = local_9;
    local_12 = _e110;
    int _e111 = local_12;
    out_.Store(int(12)*4, asuint(uint(_e111)));
    float _e116 = local_10;
    local_13 = _e116;
    float _e117 = local_13;
    out_.Store(int(13)*4, asuint(asuint(_e117)));
    int _e122 = local_11;
    local_14 = _e122;
    int _e123 = local_14;
    out_.Store(int(14)*4, asuint(uint(_e123)));
    updated = int(1);
    int _e127 = updated;
    updated = asint(asuint(_e127) + asuint(int(1)));
    int _e131 = updated;
    updated = asint(asuint(_e131) + asuint(int(3)));
    int _e136 = updated;
    local_15 = _e136;
    int _e137 = local_15;
    out_.Store(int(15)*4, asuint(uint(_e137)));
    updated = int(9);
    int _e141 = updated;
    updated = asint(asuint(_e141) + asuint(int(2)));
    int _e146 = updated;
    local_16 = _e146;
    int _e147 = local_16;
    out_.Store(int(16)*4, asuint(uint(_e147)));
    comma_left = int(3);
    comma_right = int(4);
    counter = int(0);
    int _e155 = counter;
    counter = asint(asuint(_e155) + asuint(int(1)));
    int _e159 = comma_right;
    comma_right = asint(asuint(_e159) + asuint(int(5)));
    int _e163 = comma_left;
    out_.Store(int(17)*4, asuint(uint(_e163)));
    int _e167 = comma_right;
    out_.Store(int(18)*4, asuint(uint(_e167)));
    int _e171 = counter;
    out_.Store(int(19)*4, asuint(uint(_e171)));
    enabled = false;
    enabled = true;
    bool _e178 = enabled;
    out_.Store(int(20)*4, asuint(uint(_e178)));
    bits = 4294901760u;
    uint _e183 = bits;
    bits = (_e183 + 3u);
    uint _e188 = bits;
    local_17 = _e188;
    uint _e189 = local_17;
    out_.Store(int(21)*4, asuint(_e189));
    floating = 2.0;
    float _e193 = floating;
    floating = (_e193 * 0.5);
    float _e197 = floating;
    out_.Store(int(22)*4, asuint(asuint(_e197)));
    record.value = int(10);
    record.samples[0] = int(2);
    record.samples[1] = int(3);
    int _e209 = record.value;
    local_18 = _e209;
    int _e210 = local_18;
    const int _e211 = adjust(_e210);
    record.value = _e211;
    int _e217 = record.value;
    local_19 = _e217;
    int _e218 = local_19;
    out_.Store(int(23)*4, asuint(uint(_e218)));
}
