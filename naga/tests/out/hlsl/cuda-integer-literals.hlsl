struct type_2 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
ByteAddressBuffer input : register(t1);
cbuffer n_1 : register(b2) { type_2 n_1; }
groupshared uint shared_values[64];

[numthreads(64, 1, 1)]
void integer_literals(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID, uint local_invocation_index : SV_GroupIndex)
{
    if (local_invocation_index == 0) {
        shared_values = (uint[64])0;
    }
    GroupMemoryBarrierWithGroupSync();
    uint n = (uint)0;
    uint i = (uint)0;
    uint value = (uint)0;
    uint shift = (uint)0;
    uint base = (uint)0;

    uint _e4 = n_1.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e15 = i;
    uint _e16 = n;
    if ((_e15 < _e16)) {
        uint _e19 = i;
        uint _e21 = asuint(input.Load(_e19*4));
        value = _e21;
        uint _e24 = value;
        shift = (_e24 & 3u);
        uint _e28 = i;
        base = (_e28 * 18u);
        uint _e33 = value;
        shared_values[min(uint(threadIdx.x), 63u)] = (_e33 ^ uint(int(255)));
        uint _e40 = shared_values[min(uint(threadIdx.x), 63u)];
        uint _e36 = base;
        out_.Store(_e36*4, asuint(_e40));
        uint _e46 = value;
        uint _e42 = base;
        out_.Store((_e42 + 1u)*4, asuint((_e46 & uint(int(255)))));
        uint _e54 = value;
        uint _e50 = base;
        out_.Store((_e50 + 2u)*4, asuint((_e54 & 240u)));
        uint _e61 = value;
        uint _e57 = base;
        out_.Store((_e57 + 3u)*4, asuint((_e61 + 1000u)));
        uint _e68 = shift;
        uint _e64 = base;
        out_.Store((_e64 + 4u)*4, asuint((2147483648u >> _e68)));
        uint _e76 = shift;
        uint _e71 = base;
        out_.Store((_e71 + 5u)*4, asuint(uint((int(2147483648u) >> _e76))));
        uint _e84 = value;
        uint _e80 = base;
        out_.Store((_e80 + 6u)*4, asuint((4294967295u ^ _e84)));
        uint _e91 = value;
        uint _e87 = base;
        out_.Store((_e87 + 7u)*4, asuint(uint((_e91 < 2147483648u))));
        uint _e98 = value;
        uint _e95 = base;
        out_.Store((_e95 + 8u)*4, asuint(uint((int(_e98) < int(2147483647)))));
        uint _e108 = shift;
        uint _e104 = base;
        out_.Store((_e104 + 9u)*4, asuint((2147483648u >> _e108)));
        uint _e115 = value;
        uint _e111 = base;
        out_.Store((_e111 + 10u)*4, asuint((255u + _e115)));
        uint _e122 = value;
        uint _e118 = base;
        out_.Store((_e118 + 11u)*4, asuint((((_e122 + 10u) + 8u) + 10u)));
        uint _e133 = value;
        uint _e129 = base;
        out_.Store((_e129 + 12u)*4, asuint((_e133 + uint(int(15)))));
        uint _e141 = value;
        uint _e137 = base;
        out_.Store((_e137 + 13u)*4, asuint((4294967295u - _e141)));
        uint _e148 = value;
        uint _e144 = base;
        out_.Store((_e144 + 14u)*4, asuint((3735928559u ^ _e148)));
        uint _e156 = value;
        uint _e151 = base;
        out_.Store((_e151 + 15u)*4, asuint(uint((~(int(2147483647)) < int(_e156)))));
        uint _e165 = value;
        uint _e161 = base;
        out_.Store((_e161 + 16u)*4, asuint(((_e165 + uint(int(30))) + uint(int(2)))));
        uint _e176 = value;
        uint _e172 = base;
        out_.Store((_e172 + 17u)*4, asuint(((_e176 + uint(int(30))) - uint(int(2)))));
    }
}
