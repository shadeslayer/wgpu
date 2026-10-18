struct float3_ {
    float x;
    float y;
    float z;
};

ByteAddressBuffer input : register(t0);
RWByteAddressBuffer output : register(u1);

float3_ shift(float3_ value)
{
    float3_ value_1 = (float3_)0;

    value_1 = value;
    float _e4 = value_1.x;
    value_1.x = (_e4 + 1.0);
    float _e9 = value_1.y;
    float _e10 = value_1.z;
    value_1.z = (_e9 + _e10);
    float3_ _e12 = value_1;
    const float3_ float3_1 = _e12;
    return float3_1;
}

float pair(float3_ value_2, int digit)
{
    float3_ value_3 = (float3_)0;
    int digit_1 = (int)0;

    value_3 = value_2;
    digit_1 = digit;
    float _e6 = value_3.x;
    int _e8 = digit_1;
    return ((_e6 * 10.0) + float(_e8));
}

float3_ Constructfloat3_(float arg0, float arg1, float arg2) {
    float3_ ret = (float3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

[numthreads(1, 1, 1)]
void vector_resources()
{
    int i = (int)0;
    float3_ copied = (float3_)0;
    int index = (int)0;
    bool choose_first = (bool)0;
    float local = (float)0;
    float local_1 = (float)0;
    float3_ local_2 = (float3_)0;
    bool pick_local = (bool)0;
    float3_ snapshot = (float3_)0;

    {
        i = int(0);
        uint2 loop_bound = uint2(4294967295u, 4294967295u);
        bool loop_init = true;
        while(true) {
            if (all(loop_bound == uint2(0u, 0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e7 = i;
                i = asint(asuint(_e7) + asuint(int(1)));
            }
            loop_init = false;
            int _e5 = i;
            if ((_e5 < int(3))) {
            } else {
                break;
            }
            int _e11 = i;
            float3_ _e13 = Constructfloat3_(asfloat(input.Load(_e11*12+0)), asfloat(input.Load(_e11*12+4)), asfloat(input.Load(_e11*12+8)));
            copied = _e13;
            float3_ _e16 = copied;
            const float3_ _e17 = shift(_e16);
            int _e14 = i;
            {
                float3_ _value4 = _e17;
                output.Store(_e14*12+0, asuint(_value4.x));
                output.Store(_e14*12+4, asuint(_value4.y));
                output.Store(_e14*12+8, asuint(_value4.z));
            }
        }
    }
    float3_ _e22 = Constructfloat3_(asfloat(input.Load(int(0)*12+0)), asfloat(input.Load(int(0)*12+4)), asfloat(input.Load(int(0)*12+8)));
    {
        float3_ _value2 = _e22;
        output.Store(int(3)*12+0, asuint(_value2.x));
        output.Store(int(3)*12+4, asuint(_value2.y));
        output.Store(int(3)*12+8, asuint(_value2.z));
    }
    float3_ _e28 = Constructfloat3_(asfloat(input.Load(int(1)*12+0)), asfloat(input.Load(int(1)*12+4)), asfloat(input.Load(int(1)*12+8)));
    const float3_ _e29 = shift(_e28);
    output.Store(4+int(3)*12, asuint(_e29.x));
    index = int(0);
    index = int(1);
    float _e42 = asfloat(input.Load(0+int(2)*12));
    int _e33 = index;
    index = asint(asuint(_e33) + asuint(int(1)));
    output.Store(0+_e33*12, asuint(_e42));
    int _e45 = index;
    float _e49 = asfloat(input.Load(0+int(0)*12));
    float _e53 = asfloat(input.Load(0+int(1)*12));
    {
        float3_ _value2 = Constructfloat3_(float(_e45), _e49, _e53);
        output.Store(int(4)*12+0, asuint(_value2.x));
        output.Store(int(4)*12+4, asuint(_value2.y));
        output.Store(int(4)*12+8, asuint(_value2.z));
    }
    choose_first = true;
    bool _e58 = choose_first;
    if (_e58) {
        float _e66 = asfloat(output.Load(4+int(0)*12));
        output.Store(4+int(0)*12, asuint((_e66 + 2.0)));
    } else {
        float _e68 = asfloat(output.Load(4+int(2)*12));
        output.Store(4+int(2)*12, asuint((_e68 + 2.0)));
    }
    bool _e73 = choose_first;
    if (_e73) {
        float _e81 = asfloat(output.Load(8+int(0)*12));
        output.Store(8+int(0)*12, asuint((_e81 + float(int(1)))));
        local = _e81;
    } else {
        float _e85 = asfloat(output.Load(8+int(2)*12));
        output.Store(8+int(2)*12, asuint((_e85 + float(int(1)))));
        local = _e85;
    }
    float _e89 = local;
    output.Store(0+int(4)*12, asuint(_e89));
    int _e93 = index;
    output.Store(8+int(4)*12, asuint(float(_e93)));
    float3_ _e102 = Constructfloat3_(asfloat(input.Load(int(1)*12+0)), asfloat(input.Load(int(1)*12+4)), asfloat(input.Load(int(1)*12+8)));
    bool _e95 = choose_first;
    if (_e95) {
        {
            float3_ _value3 = _e102;
            output.Store(int(0)*12+0, asuint(_value3.x));
            output.Store(int(0)*12+4, asuint(_value3.y));
            output.Store(int(0)*12+8, asuint(_value3.z));
        }
        float _e106 = asfloat(output.Load(0+int(0)*12));
        output.Store(0+int(0)*12, asuint((_e106 + float(int(1)))));
        local_1 = _e106;
    } else {
        {
            float3_ _value3 = _e102;
            output.Store(int(2)*12+0, asuint(_value3.x));
            output.Store(int(2)*12+4, asuint(_value3.y));
            output.Store(int(2)*12+8, asuint(_value3.z));
        }
        float _e110 = asfloat(output.Load(0+int(2)*12));
        output.Store(0+int(2)*12, asuint((_e110 + float(int(1)))));
        local_1 = _e110;
    }
    float3_ _e118 = Constructfloat3_(asfloat(output.Load(int(0)*12+0)), asfloat(output.Load(int(0)*12+4)), asfloat(output.Load(int(0)*12+8)));
    {
        float3_ _value2 = _e118;
        output.Store(int(5)*12+0, asuint(_value2.x));
        output.Store(int(5)*12+4, asuint(_value2.y));
        output.Store(int(5)*12+8, asuint(_value2.z));
    }
    local_2 = Constructfloat3_(float(int(20)), float(int(30)), float(int(40)));
    pick_local = false;
    pick_local = true;
    bool _e129 = pick_local;
    if (_e129) {
        local_2.z = 7.0;
    } else {
        output.Store(8+int(1)*12, asuint(7.0));
    }
    float3_ _e138 = local_2;
    {
        float3_ _value2 = _e138;
        output.Store(int(6)*12+0, asuint(_value2.x));
        output.Store(int(6)*12+4, asuint(_value2.y));
        output.Store(int(6)*12+8, asuint(_value2.z));
    }
    float _e145 = asfloat(output.Load(8+int(1)*12));
    output.Store(0+int(6)*12, asuint(_e145));
    float _e151 = asfloat(output.Load(0+int(0)*12));
    float _e155 = asfloat(output.Load(0+int(1)*12));
    float _e159 = asfloat(output.Load(0+int(2)*12));
    {
        float3_ _value2 = Constructfloat3_(_e151, _e155, _e159);
        output.Store(int(7)*12+0, asuint(_value2.x));
        output.Store(int(7)*12+4, asuint(_value2.y));
        output.Store(int(7)*12+8, asuint(_value2.z));
    }
    float3_ _e164 = Constructfloat3_(asfloat(input.Load(int(0)*12+0)), asfloat(input.Load(int(0)*12+4)), asfloat(input.Load(int(0)*12+8)));
    snapshot = _e164;
    float3_ _e167 = snapshot;
    snapshot.x = 9.0;
    const float _e171 = pair(_e167, int(4));
    float _e173 = snapshot.x;
    float _e177 = asfloat(input.Load(8+int(2)*12));
    {
        float3_ _value2 = Constructfloat3_(_e171, _e173, _e177);
        output.Store(int(8)*12+0, asuint(_value2.x));
        output.Store(int(8)*12+4, asuint(_value2.y));
        output.Store(int(8)*12+8, asuint(_value2.z));
    }
}
