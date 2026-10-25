struct Packet_cuda_record_0_ {
    uint first;
    uint second;
};

RWByteAddressBuffer out_ : register(u0);

uint later(uint value)
{
    uint value_1 = (uint)0;

    value_1 = value;
    uint _e3 = value_1;
    return (_e3 + 5u);
}

[numthreads(2, 1, 1)]
void preprocessing(uint3 threadIdx : SV_GroupThreadID)
{
    uint lane = (uint)0;
    uint start = (uint)0;
    uint local = (uint)0;
    Packet_cuda_record_0_ original = (Packet_cuda_record_0_)0;
    Packet_cuda_record_0_ copied = (Packet_cuda_record_0_)0;
    uint SELF = (uint)0;
    uint LEFT = (uint)0;
    uint RIGHT = (uint)0;
    uint counter = (uint)0;
    Packet_cuda_record_0_ ordered = (Packet_cuda_record_0_)0;
    uint values[3] = (uint[3])0;
    uint index = (uint)0;

    lane = threadIdx.x;
    uint _e5 = lane;
    if ((_e5 >= 2u)) {
        return;
    }
    uint _e9 = lane;
    start = (_e9 * 24u);
    uint _e12 = start;
    out_.Store((_e12 + 0u)*4, asuint(7u));
    uint _e17 = start;
    out_.Store((_e17 + 1u)*4, asuint(11u));
    uint _e22 = start;
    out_.Store((_e22 + 2u)*4, asuint(22u));
    uint _e27 = start;
    out_.Store((_e27 + 3u)*4, asuint((2u + (3u * 4u))));
    uint _e36 = start;
    out_.Store((_e36 + 4u)*4, asuint((2u + 3u)));
    local = 2u;
    uint _e49 = local;
    uint _e43 = start;
    out_.Store((_e43 + 5u)*4, asuint((3u << _e49)));
    uint _e55 = lane;
    const uint _e56 = later(_e55);
    uint _e52 = start;
    out_.Store((_e52 + 6u)*4, asuint(_e56));
    uint _e59 = lane;
    original.first = (_e59 + 1u);
    uint _e64 = lane;
    original.second = (_e64 + 2u);
    Packet_cuda_record_0_ _e67 = original;
    copied = _e67;
    uint _e73 = copied.first;
    uint _e69 = start;
    out_.Store((_e69 + 7u)*4, asuint(_e73));
    uint _e79 = copied.second;
    uint _e75 = start;
    out_.Store((_e75 + 8u)*4, asuint(_e79));
    SELF = 17u;
    LEFT = 19u;
    RIGHT = 23u;
    uint _e90 = SELF;
    uint _e87 = start;
    out_.Store((_e87 + 9u)*4, asuint(_e90));
    uint _e95 = LEFT;
    uint _e92 = start;
    out_.Store((_e92 + 10u)*4, asuint(_e95));
    uint _e100 = RIGHT;
    uint _e97 = start;
    out_.Store((_e97 + 11u)*4, asuint(_e100));
    counter = 0u;
    uint _e104 = counter;
    counter = (_e104 + uint(int(1)));
    uint _e108 = counter;
    ordered.first = _e108;
    uint _e111 = counter;
    counter = (_e111 + uint(int(1)));
    uint _e115 = counter;
    ordered.second = _e115;
    uint _e121 = ordered.first;
    uint _e117 = start;
    out_.Store((_e117 + 12u)*4, asuint(_e121));
    uint _e127 = ordered.second;
    uint _e123 = start;
    out_.Store((_e123 + 13u)*4, asuint(_e127));
    uint _e132 = counter;
    uint _e129 = start;
    out_.Store((_e129 + 14u)*4, asuint(_e132));
    uint _e134 = start;
    out_.Store((_e134 + 15u)*4, asuint(31u));
    values[0] = 2u;
    values[1] = 4u;
    values[2] = 6u;
    uint _e149 = lane;
    uint _e151 = values[min(uint(_e149), 2u)];
    uint _e146 = start;
    out_.Store((_e146 + 16u)*4, asuint(_e151));
    index = 0u;
    uint _e158 = index;
    index = (_e158 + uint(int(1)));
    uint _e163 = values[min(uint(_e158), 2u)];
    uint _e155 = start;
    out_.Store((_e155 + 17u)*4, asuint(_e163));
    uint _e168 = index;
    uint _e165 = start;
    out_.Store((_e165 + 18u)*4, asuint(_e168));
    uint _e170 = start;
    out_.Store((_e170 + 19u)*4, asuint(uint(true)));
    uint _e176 = start;
    out_.Store((_e176 + 20u)*4, asuint(61u));
    uint _e184 = lane;
    uint _e181 = start;
    out_.Store((_e181 + 21u)*4, asuint(_e184));
    uint _e186 = start;
    out_.Store((_e186 + 22u)*4, asuint(asuint(-(0.0))));
    uint _e193 = start;
    out_.Store((_e193 + 23u)*4, asuint(4294967295u));
}
