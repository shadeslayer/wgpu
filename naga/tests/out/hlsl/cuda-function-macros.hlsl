struct Pair_cuda_record_0_ {
    uint first;
    uint second;
};

RWByteAddressBuffer out_ : register(u0);

uint advance(uint value)
{
    uint value_1 = (uint)0;

    value_1 = value;
    uint _e3 = value_1;
    return (_e3 + 3u);
}

[numthreads(2, 1, 1)]
void function_macros(uint3 threadIdx : SV_GroupThreadID)
{
    uint lane = (uint)0;
    uint start = (uint)0;
    Pair_cuda_record_0_ pair = (Pair_cuda_record_0_)0;
    Pair_cuda_record_0_ local = (Pair_cuda_record_0_)0;
    uint values[3] = (uint[3])0;
    uint2 vector_ = (uint2)0;
    uint counter = (uint)0;
    uint local_1 = (uint)0;
    uint declared = (uint)0;

    lane = threadIdx.x;
    uint _e5 = lane;
    if ((_e5 >= 2u)) {
        return;
    }
    uint _e9 = lane;
    start = (_e9 * 24u);
    local.first = 11u;
    local.second = 13u;
    Pair_cuda_record_0_ _e17 = local;
    pair = _e17;
    values[0] = 2u;
    values[1] = 3u;
    values[2] = 5u;
    vector_ = uint2(17u, 19u);
    counter = 0u;
    uint _e31 = start;
    out_.Store(_e31*4, asuint(7u));
    uint _e35 = start;
    out_.Store((_e35 + 1u)*4, asuint(11u));
    uint _e40 = start;
    out_.Store((_e40 + 2u)*4, asuint((2u + 3u)));
    uint _e47 = start;
    out_.Store((_e47 + 3u)*4, asuint(((1u + 2u) * 3u)));
    uint _e56 = start;
    out_.Store((_e56 + 4u)*4, asuint((5u + 6u)));
    uint _e63 = start;
    out_.Store((_e63 + 5u)*4, asuint((7u + 8u)));
    uint _e74 = pair.first;
    uint _e70 = start;
    out_.Store((_e70 + 6u)*4, asuint(_e74));
    uint _e80 = pair.second;
    uint _e76 = start;
    out_.Store((_e76 + 7u)*4, asuint(_e80));
    uint _e82 = start;
    out_.Store((_e82 + 8u)*4, asuint(uint(int(3))));
    uint _e91 = lane;
    uint _e93 = values[min(uint(_e91), 2u)];
    uint _e88 = start;
    out_.Store((_e88 + 9u)*4, asuint(_e93));
    uint _e99 = vector_.x;
    uint _e95 = start;
    out_.Store((_e95 + 10u)*4, asuint(_e99));
    uint _e105 = vector_.y;
    uint _e101 = start;
    out_.Store((_e101 + 11u)*4, asuint(_e105));
    uint _e110 = counter;
    counter = (_e110 + uint(int(1)));
    uint _e114 = counter;
    counter = (_e114 + uint(int(1)));
    uint _e107 = start;
    out_.Store((_e107 + 12u)*4, asuint(_e114));
    uint _e122 = counter;
    uint _e119 = start;
    out_.Store((_e119 + 13u)*4, asuint(_e122));
    uint _e124 = start;
    out_.Store((_e124 + 14u)*4, asuint(19u));
    uint _e132 = counter;
    uint _e129 = start;
    out_.Store((_e129 + 15u)*4, asuint(_e132));
    values[int(0)] = 23u;
    uint _e140 = values[int(0)];
    uint _e134 = start;
    out_.Store((_e134 + 16u)*4, asuint(_e140));
    uint _e147 = values[int(0)];
    uint _e142 = start;
    out_.Store((_e142 + 17u)*4, asuint(_e147));
    uint _e153 = lane;
    if ((_e153 == 0u)) {
        local_1 = 29u;
    } else {
        local_1 = 31u;
    }
    uint _e158 = local_1;
    uint _e149 = start;
    out_.Store((_e149 + 18u)*4, asuint(_e158));
    uint _e163 = lane;
    const uint _e164 = advance(_e163);
    uint _e160 = start;
    out_.Store((_e160 + 19u)*4, asuint(_e164));
    declared = 37u;
    uint _e171 = declared;
    uint _e168 = start;
    out_.Store((_e168 + 20u)*4, asuint(_e171));
    uint _e173 = start;
    out_.Store((_e173 + 21u)*4, asuint(asuint(-(0.0))));
    uint _e180 = start;
    out_.Store((_e180 + 22u)*4, asuint(4294967295u));
    uint _e189 = lane;
    uint _e185 = start;
    out_.Store((_e185 + 23u)*4, asuint((41u + _e189)));
}
