struct type_3 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

ByteAddressBuffer input : register(t0);
RWByteAddressBuffer output : register(u1);
cbuffer n_1 : register(b2) { type_3 n_1; }

[numthreads(1, 1, 1)]
void convert(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    int i = (int)0;
    float offset = (float)0;

    uint _e4 = n_1.value;
    n = _e4;
    i = int(((blockIdx.x * 1u) + threadIdx.x));
    int _e15 = i;
    uint _e17 = n;
    if ((uint(_e15) < _e17)) {
        int _e20 = i;
        offset = float(_e20);
        int _e24 = i;
        float _e26 = asfloat(input.Load(_e24*4));
        float _e27 = offset;
        int _e22 = i;
        output.Store(_e22*4, asuint((_e26 + _e27)));
    }
}
