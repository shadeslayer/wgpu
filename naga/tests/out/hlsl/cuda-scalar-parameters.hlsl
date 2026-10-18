struct type_1 {
    float value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

cbuffer factor_1 : register(b0) { type_1 factor_1; }
ByteAddressBuffer input : register(t1);
RWByteAddressBuffer output : register(u2);

[numthreads(1, 1, 1)]
void scale(uint3 threadIdx : SV_GroupThreadID)
{
    float factor = (float)0;

    float _e2 = factor_1.value;
    factor = _e2;
    float _e7 = factor;
    factor = (_e7 * 2.0);
    float _e14 = asfloat(input.Load(threadIdx.x*4));
    float _e15 = factor;
    output.Store(threadIdx.x*4, asuint((_e14 * _e15)));
}
