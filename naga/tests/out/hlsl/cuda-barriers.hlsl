RWByteAddressBuffer out_ : register(u0);
groupshared uint values[64];
groupshared uint broadcast;

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

[numthreads(64, 1, 1)]
void barriers(uint3 threadIdx : SV_GroupThreadID, uint3 blockIdx : SV_GroupID, uint local_invocation_index : SV_GroupIndex)
{
    if (local_invocation_index == 0) {
        values = (uint[64])0;
        broadcast = (uint)0;
    }
    GroupMemoryBarrierWithGroupSync();
    uint lane = (uint)0;
    uint base = (uint)0;

    lane = threadIdx.x;
    base = (blockIdx.x * 256u);
    uint _e16 = lane;
    uint _e11 = lane;
    values[min(uint(_e11), 63u)] = (((blockIdx.x * 2000u) + _e16) + 7u);
    uint _e27 = lane;
    uint _e20 = base;
    uint _e21 = lane;
    out_.Store((_e20 + _e21)*4, asuint(((blockIdx.x * 1000u) + _e27)));
    uint _e30 = lane;
    if ((_e30 == 0u)) {
        broadcast = ((blockIdx.x * 100u) + 11u);
    }
    DeviceMemoryBarrierWithGroupSync();
    GroupMemoryBarrierWithGroupSync();
    uint _e44 = lane;
    uint _e47 = values[min(uint((63u - _e44)), 63u)];
    uint _e38 = base;
    uint _e40 = lane;
    out_.Store(((_e38 + 64u) + _e40)*4, asuint(_e47));
    uint _e55 = lane;
    uint _e59 = base;
    uint _e62 = asuint(out_.Load((_e59 + naga_mod((_e55 + 1u), 64u))*4));
    uint _e49 = base;
    uint _e51 = lane;
    out_.Store(((_e49 + 128u) + _e51)*4, asuint(_e62));
    uint _e69 = broadcast;
    uint _e64 = base;
    uint _e66 = lane;
    out_.Store(((_e64 + 192u) + _e66)*4, asuint(_e69));
}
