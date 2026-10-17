RWByteAddressBuffer out_ : register(u0);
groupshared uint values[64];
groupshared int signed_value;
groupshared float float_value;

int naga_neg(int val) {
    return asint(-asuint(val));
}

uint naga_f2u32(float value) {
    return uint(clamp(value, 0.0, 4294967000.0));
}

[numthreads(64, 1, 1)]
void shared_storage(uint3 threadIdx : SV_GroupThreadID, uint3 blockIdx : SV_GroupID, uint local_invocation_index : SV_GroupIndex)
{
    if (local_invocation_index == 0) {
        values = (uint[64])0;
        signed_value = (int)0;
        float_value = (float)0;
    }
    GroupMemoryBarrierWithGroupSync();
    uint lane = (uint)0;
    uint base = (uint)0;

    lane = threadIdx.x;
    base = (blockIdx.x * 128u);
    uint _e17 = lane;
    uint _e12 = lane;
    values[min(uint(_e12), 63u)] = (((blockIdx.x * 1000u) + _e17) + 10u);
    uint _e25 = lane;
    uint _e27 = values[min(uint(_e25), 63u)];
    uint _e21 = base;
    uint _e22 = lane;
    out_.Store((_e21 + _e22)*4, asuint(_e27));
    uint _e29 = lane;
    if ((_e29 == 0u)) {
        signed_value = naga_neg(int(7));
        float_value = 3.5;
        int _e39 = signed_value;
        uint _e35 = base;
        out_.Store((_e35 + 64u)*4, asuint(uint(asint(asuint(_e39) + asuint(int(10))))));
        float _e47 = float_value;
        uint _e43 = base;
        out_.Store((_e43 + 65u)*4, asuint(naga_f2u32((_e47 * 2.0))));
    }
}
