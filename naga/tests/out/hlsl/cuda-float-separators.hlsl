struct type_3 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
ByteAddressBuffer input : register(t1);
cbuffer n_1 : register(b2) { type_3 n_1; }

[numthreads(64, 1, 1)]
void float_separators(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID)
{
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;

    uint _e4 = n_1.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e14 = i;
    uint _e15 = n;
    if ((_e14 < _e15)) {
        uint _e19 = i;
        base = (_e19 * 12u);
        uint _e21 = base;
        out_.Store(_e21*4, asuint(1024.5));
        uint _e25 = base;
        out_.Store((_e25 + 1u)*4, asuint(0.125));
        uint _e30 = base;
        out_.Store((_e30 + 2u)*4, asuint(12.0));
        uint _e35 = base;
        out_.Store((_e35 + 3u)*4, asuint(25.0));
        uint _e40 = base;
        out_.Store((_e40 + 4u)*4, asuint(0.5));
        uint _e45 = base;
        out_.Store((_e45 + 5u)*4, asuint(6.25));
        uint _e50 = base;
        out_.Store((_e50 + 6u)*4, asuint(-(1024.5)));
        uint _e56 = base;
        out_.Store((_e56 + 7u)*4, asuint(0.125));
        uint _e61 = base;
        out_.Store((_e61 + 8u)*4, asuint(3.1415927));
        uint _e66 = base;
        out_.Store((_e66 + 9u)*4, asuint(1.0));
        uint _e71 = base;
        out_.Store((_e71 + 10u)*4, asuint(1.0000002));
        uint _e79 = i;
        float _e82 = asfloat(input.Load(_e79*4));
        uint _e76 = base;
        out_.Store((_e76 + 11u)*4, asuint(((_e82 * 0.25) + 10.0)));
    }
}
