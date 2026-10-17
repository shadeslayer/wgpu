struct type_5 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
ByteAddressBuffer input : register(t1);
cbuffer n_1 : register(b2) { type_5 n_1; }
groupshared uint shared_values[64];

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

int transform(float value, float multiplier, int offset)
{
    float value_1 = (float)0;
    float multiplier_1 = (float)0;
    int offset_1 = (int)0;

    value_1 = value;
    multiplier_1 = multiplier;
    offset_1 = offset;
    float _e6 = value_1;
    float _e7 = multiplier_1;
    int _e9 = offset_1;
    return naga_f2i32(((_e6 * _e7) + float(_e9)));
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

uint naga_f2u32(float value) {
    return uint(clamp(value, 0.0, 4294967000.0));
}

[numthreads(64, 1, 1)]
void scalar_aliases(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID, uint local_invocation_index : SV_GroupIndex)
{
    if (local_invocation_index == 0) {
        shared_values = (uint[64])0;
    }
    GroupMemoryBarrierWithGroupSync();
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;
    uint lane = (uint)0;
    int signed_count = (int)0;
    float fraction = (float)0;
    int after_block = (int)0;
    float coefficient = (float)0;
    bool selected = (bool)0;
    int total = (int)0;
    int current = (int)0;
    float after_loop = (float)0;

    uint _e4 = n_1.value;
    n = _e4;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e15 = i;
    uint _e16 = n;
    if ((_e15 < _e16)) {
        uint _e20 = i;
        base = (_e20 * 10u);
        lane = threadIdx.x;
        uint _e27 = i;
        uint _e24 = lane;
        shared_values[min(uint(_e24), 63u)] = (_e27 + 10u);
        uint _e31 = lane;
        uint _e33 = shared_values[min(uint(_e31), 63u)];
        uint _e29 = base;
        out_.Store(_e29*4, asuint(_e33));
        uint _e38 = i;
        int _e40 = asint(input.Load(_e38*4));
        const int _e44 = transform(float(_e40), 2.0, int(1));
        uint _e35 = base;
        out_.Store((_e35 + 1u)*4, asuint(uint(_e44)));
        signed_count = naga_neg(int(3));
        int _e53 = signed_count;
        uint _e50 = base;
        out_.Store((_e50 + 2u)*4, asuint(uint(_e53)));
        {
            fraction = 3.75;
            float _e61 = fraction;
            uint _e58 = base;
            out_.Store((_e58 + 3u)*4, asuint(naga_f2u32(_e61)));
        }
        after_block = naga_neg(int(5));
        int _e70 = after_block;
        uint _e67 = base;
        out_.Store((_e67 + 4u)*4, asuint(uint(_e70)));
        coefficient = 1.25;
        float _e79 = coefficient;
        uint _e75 = base;
        out_.Store((_e75 + 5u)*4, asuint(naga_f2u32((_e79 * 4.0))));
        uint _e84 = i;
        selected = (naga_mod(_e84, 2u) == 0u);
        bool _e92 = selected;
        uint _e89 = base;
        out_.Store((_e89 + 6u)*4, asuint(uint(_e92)));
        total = int(0);
        {
            uint2 loop_bound = uint2(4294967295u, 4294967295u);
            while(true) {
                if (all(loop_bound == uint2(0u, 0u))) { break; }
                loop_bound -= uint2(loop_bound.y == 0u, 1u);
                int _e97 = total;
                if ((_e97 < int(3))) {
                } else {
                    break;
                }
                int _e100 = total;
                current = _e100;
                int _e102 = current;
                total = asint(asuint(_e102) + asuint(int(1)));
            }
        }
        int _e108 = total;
        uint _e105 = base;
        out_.Store((_e105 + 7u)*4, asuint(uint(_e108)));
        after_loop = 1.5;
        float _e117 = after_loop;
        uint _e113 = base;
        out_.Store((_e113 + 8u)*4, asuint(naga_f2u32((_e117 * 2.0))));
        uint _e124 = n;
        uint _e121 = base;
        out_.Store((_e121 + 9u)*4, asuint(naga_f2u32(float(_e124))));
    }
}
