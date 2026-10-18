ByteAddressBuffer a : register(t0);
ByteAddressBuffer b : register(t1);
RWByteAddressBuffer out_ : register(u2);

[numthreads(1, 1, 1)]
void add(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint i = (uint)0;

    i = ((blockIdx.x * 1u) + threadIdx.x);
    uint _e13 = i;
    uint _e15 = i;
    float _e18 = asfloat(b.Load(_e15*4));
    float _e20 = asfloat(a.Load(_e13*4));
    uint _e11 = i;
    out_.Store(_e11*4, asuint((_e20 + (_e18 * 2.0))));
}
