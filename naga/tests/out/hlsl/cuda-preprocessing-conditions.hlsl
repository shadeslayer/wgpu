RWByteAddressBuffer out_ : register(u0);

uint later(uint value)
{
    uint value_1 = (uint)0;

    value_1 = value;
    uint _e3 = value_1;
    return (_e3 + 73u);
}

[numthreads(2, 1, 1)]
void preprocessing_conditions(uint3 threadIdx : SV_GroupThreadID)
{
    uint lane = (uint)0;
    uint start = (uint)0;
    uint counter = (uint)0;

    lane = threadIdx.x;
    uint _e5 = lane;
    if ((_e5 >= 2u)) {
        return;
    }
    uint _e9 = lane;
    start = (_e9 * 24u);
    uint _e11 = start;
    out_.Store(_e11*4, asuint(7u));
    uint _e15 = start;
    out_.Store((_e15 + 1u)*4, asuint(11u));
    uint _e20 = start;
    out_.Store((_e20 + 2u)*4, asuint(13u));
    uint _e25 = start;
    out_.Store((_e25 + 3u)*4, asuint(17u));
    uint _e30 = start;
    out_.Store((_e30 + 4u)*4, asuint(19u));
    uint _e35 = start;
    out_.Store((_e35 + 5u)*4, asuint(23u));
    uint _e40 = start;
    out_.Store((_e40 + 6u)*4, asuint(29u));
    uint _e45 = start;
    out_.Store((_e45 + 7u)*4, asuint(31u));
    uint _e50 = start;
    out_.Store((_e50 + 8u)*4, asuint(37u));
    uint _e55 = start;
    out_.Store((_e55 + 9u)*4, asuint(41u));
    uint _e60 = start;
    out_.Store((_e60 + 10u)*4, asuint(43u));
    uint _e65 = start;
    out_.Store((_e65 + 11u)*4, asuint(47u));
    uint _e70 = start;
    out_.Store((_e70 + 12u)*4, asuint(53u));
    uint _e75 = start;
    out_.Store((_e75 + 13u)*4, asuint(59u));
    uint _e80 = start;
    out_.Store((_e80 + 14u)*4, asuint(61u));
    uint _e85 = start;
    out_.Store((_e85 + 15u)*4, asuint(67u));
    uint _e90 = start;
    out_.Store((_e90 + 16u)*4, asuint(71u));
    uint _e98 = lane;
    const uint _e99 = later(_e98);
    uint _e95 = start;
    out_.Store((_e95 + 17u)*4, asuint(_e99));
    counter = 0u;
    uint _e106 = counter;
    counter = (_e106 + uint(int(1)));
    uint _e103 = start;
    out_.Store((_e103 + 18u)*4, asuint(_e106));
    uint _e114 = counter;
    uint _e111 = start;
    out_.Store((_e111 + 19u)*4, asuint(_e114));
    uint _e116 = start;
    out_.Store((_e116 + 20u)*4, asuint(uint(true)));
    uint _e125 = lane;
    uint _e122 = start;
    out_.Store((_e122 + 21u)*4, asuint(_e125));
    uint _e127 = start;
    out_.Store((_e127 + 22u)*4, asuint(asuint(-(0.0))));
    uint _e134 = start;
    out_.Store((_e134 + 23u)*4, asuint(4294967295u));
}
