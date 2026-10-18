struct type_3 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

ByteAddressBuffer a : register(t0);
ByteAddressBuffer b : register(t1);
RWByteAddressBuffer out_ : register(u2);
cbuffer n_1 : register(b3) { type_3 n_1; }

[numthreads(1, 1, 1)]
void add(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    uint i = (uint)0;

    uint _e5 = n_1.value;
    n = _e5;
    i = ((blockIdx.x * 1u) + threadIdx.x);
    uint _e15 = i;
    uint _e16 = n;
    if ((_e15 < _e16)) {
        uint _e20 = i;
        uint _e22 = i;
        float _e24 = asfloat(a.Load(_e20*4));
        float _e25 = asfloat(b.Load(_e22*4));
        uint _e18 = i;
        out_.Store(_e18*4, asuint((_e24 + _e25)));
    }
}
