struct type_2 {
    int value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
cbuffer value_1 : register(b1) { type_2 value_1; }

[numthreads(1, 1, 1)]
void updates(uint3 threadIdx : SV_GroupThreadID)
{
    int value = (int)0;
    int previous = (int)0;

    int _e3 = value_1.value;
    value = _e3;
    int _e6 = value;
    value = asint(asuint(_e6) + asuint(int(1)));
    previous = _e6;
    int _e12 = previous;
    int _e13 = asint(out_.Load(threadIdx.x*4));
    out_.Store(threadIdx.x*4, asuint(asint(asuint(_e13) + asuint(_e12))));
    int _e17 = value;
    value = asint(asuint(_e17) + asuint(int(1)));
    int _e20 = value;
    out_.Store(threadIdx.x*4, asuint(_e20));
}
