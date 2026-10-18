struct float3_ {
    float x;
    float y;
    float z;
};

RWByteAddressBuffer output : register(u0);
groupshared float3_ values[8];
groupshared float3_ broadcast;

float3_ shift(float3_ value)
{
    float3_ value_1 = (float3_)0;

    value_1 = value;
    float _e4 = value_1.x;
    value_1.x = (_e4 + 1.0);
    float _e9 = value_1.z;
    float _e10 = value_1.y;
    value_1.y = (_e9 - _e10);
    float3_ _e12 = value_1;
    const float3_ float3_1 = _e12;
    return float3_1;
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

float3_ Constructfloat3_(float arg0, float arg1, float arg2) {
    float3_ ret = (float3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

[numthreads(8, 1, 1)]
void shared_vectors(uint3 threadIdx : SV_GroupThreadID, uint3 blockIdx : SV_GroupID, uint local_invocation_index : SV_GroupIndex)
{
    if (local_invocation_index == 0) {
        values = (float3_[8])0;
        broadcast = (float3_)0;
    }
    GroupMemoryBarrierWithGroupSync();
    uint lane = (uint)0;
    uint group = (uint)0;
    float base = (float)0;
    uint next = (uint)0;
    float3_ copied = (float3_)0;
    int offset = (int)0;
    bool choose_shared = (bool)0;
    uint row = (uint)0;

    lane = threadIdx.x;
    group = blockIdx.x;
    uint _e11 = group;
    uint _e13 = lane;
    base = float(((_e11 * 16u) + _e13));
    float _e19 = base;
    float _e21 = base;
    float _e26 = base;
    uint _e16 = lane;
    values[min(uint(_e16), 7u)] = Constructfloat3_((_e19 + 0.5), (-(_e21) - 1.25), (_e26 + 2.5));
    uint _e30 = lane;
    if ((_e30 == 0u)) {
        uint _e32 = group;
        uint _e36 = group;
        uint _e40 = group;
        broadcast = Constructfloat3_((float(_e32) + 0.25), (float(_e36) + 0.5), (float(_e40) + 0.75));
    }
    DeviceMemoryBarrierWithGroupSync();
    GroupMemoryBarrierWithGroupSync();
    uint _e47 = lane;
    next = naga_mod((_e47 + 1u), 8u);
    uint _e52 = next;
    float3_ _e54 = values[min(uint(_e52), 7u)];
    const float3_ _e55 = shift(_e54);
    copied = _e55;
    {
        offset = int(1);
        int _e59 = offset;
        float _e60 = copied.x;
        copied.x = (_e60 + float(_e59));
    }
    DeviceMemoryBarrierWithGroupSync();
    GroupMemoryBarrierWithGroupSync();
    uint _e65 = lane;
    choose_shared = ((_e65 & 1u) != 0u);
    float _e75 = broadcast.x;
    bool _e69 = choose_shared;
    if (_e69) {
        uint _e70 = lane;
        float _e76 = values[min(uint(_e70), 7u)].y;
        values[min(uint(_e70), 7u)].y = (_e76 + _e75);
    } else {
        float _e78 = copied.y;
        copied.y = (_e78 + _e75);
    }
    float _e84 = copied.x;
    uint _e80 = lane;
    values[min(uint(_e80), 7u)].z = _e84;
    uint _e87 = group;
    uint _e89 = lane;
    row = (((_e87 * 8u) + _e89) * 9u);
    float _e96 = copied.x;
    uint _e93 = row;
    output.Store(_e93*4, asuint(_e96));
    float _e102 = copied.y;
    uint _e98 = row;
    output.Store((_e98 + 1u)*4, asuint(_e102));
    float _e108 = copied.z;
    uint _e104 = row;
    output.Store((_e104 + 2u)*4, asuint(_e108));
    DeviceMemoryBarrierWithGroupSync();
    GroupMemoryBarrierWithGroupSync();
    uint _e113 = next;
    float _e116 = values[min(uint(_e113), 7u)].x;
    uint _e110 = row;
    output.Store((_e110 + 3u)*4, asuint(_e116));
    uint _e121 = next;
    float _e124 = values[min(uint(_e121), 7u)].y;
    uint _e118 = row;
    output.Store((_e118 + 4u)*4, asuint(_e124));
    uint _e129 = next;
    float _e132 = values[min(uint(_e129), 7u)].z;
    uint _e126 = row;
    output.Store((_e126 + 5u)*4, asuint(_e132));
    float _e138 = broadcast.x;
    uint _e134 = row;
    output.Store((_e134 + 6u)*4, asuint(_e138));
    float _e144 = broadcast.y;
    uint _e140 = row;
    output.Store((_e140 + 7u)*4, asuint(_e144));
    float _e150 = broadcast.z;
    uint _e146 = row;
    output.Store((_e146 + 8u)*4, asuint(_e150));
}
