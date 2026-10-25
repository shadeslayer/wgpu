struct float3_ {
    float x;
    float y;
    float z;
};

struct Sample_cuda_record_0_ {
    float3_ position;
    uint tag;
};

struct Packet_cuda_record_1_ {
    uint serial;
    int _pad1_0;
    float2 uv;
    Sample_cuda_record_0_ samples[2];
    float weights[2][2];
    float4 colour;
};

struct Packet_cuda_record_2_ {
    int w;
    int x;
};

ByteAddressBuffer input : register(t0);
RWByteAddressBuffer output : register(u1);

Packet_cuda_record_1_ adjust(Packet_cuda_record_1_ value, int bias)
{
    Packet_cuda_record_1_ value_1 = (Packet_cuda_record_1_)0;
    int bias_1 = (int)0;

    value_1 = value;
    bias_1 = bias;
    int _e5 = bias_1;
    uint _e7 = value_1.serial;
    value_1.serial = (_e7 + uint(_e5));
    float _e19 = value_1.weights[int(0)][int(1)];
    float _e20 = value_1.samples[int(1)].position.x;
    value_1.samples[int(1)].position.x = (_e20 + _e19);
    uint _e31 = value_1.samples[int(0)].tag;
    value_1.weights[int(1)][int(0)] = float(_e31);
    Packet_cuda_record_1_ _e33 = value_1;
    const Packet_cuda_record_1_ packet_cuda_record_1_ = _e33;
    return packet_cuda_record_1_;
}

Packet_cuda_record_1_ operator_u002b(Packet_cuda_record_1_ left, Packet_cuda_record_1_ right)
{
    Packet_cuda_record_1_ left_1 = (Packet_cuda_record_1_)0;
    Packet_cuda_record_1_ right_1 = (Packet_cuda_record_1_)0;

    left_1 = left;
    right_1 = right;
    uint _e6 = right_1.serial;
    uint _e7 = left_1.serial;
    left_1.serial = (_e7 + _e6);
    float _e13 = right_1.uv.y;
    float _e14 = left_1.uv.x;
    left_1.uv.x = (_e14 + _e13);
    Packet_cuda_record_1_ _e16 = left_1;
    const Packet_cuda_record_1_ packet_cuda_record_1_1 = _e16;
    return packet_cuda_record_1_1;
}

float3_ Constructfloat3_(float arg0, float arg1, float arg2) {
    float3_ ret = (float3_)0;
    ret.x = arg0;
    ret.y = arg1;
    ret.z = arg2;
    return ret;
}

Sample_cuda_record_0_ ConstructSample_cuda_record_0_(float3_ arg0, uint arg1) {
    Sample_cuda_record_0_ ret = (Sample_cuda_record_0_)0;
    ret.position = arg0;
    ret.tag = arg1;
    return ret;
}

typedef Sample_cuda_record_0_ ret_Constructarray2_Sample_cuda_record_0__[2];
ret_Constructarray2_Sample_cuda_record_0__ Constructarray2_Sample_cuda_record_0__(Sample_cuda_record_0_ arg0, Sample_cuda_record_0_ arg1) {
    Sample_cuda_record_0_ ret[2] = { arg0, arg1 };
    return ret;
}

typedef float ret_Constructarray2_float_[2];
ret_Constructarray2_float_ Constructarray2_float_(float arg0, float arg1) {
    float ret[2] = { arg0, arg1 };
    return ret;
}

typedef float ret_Constructarray2_array2_float__[2][2];
ret_Constructarray2_array2_float__ Constructarray2_array2_float__(float arg0[2], float arg1[2]) {
    float ret[2][2] = { arg0, arg1 };
    return ret;
}

Packet_cuda_record_1_ ConstructPacket_cuda_record_1_(uint arg0, float2 arg1, Sample_cuda_record_0_ arg2[2], float arg3[2][2], float4 arg4) {
    Packet_cuda_record_1_ ret = (Packet_cuda_record_1_)0;
    ret.serial = arg0;
    ret.uv = arg1;
    ret.samples = arg2;
    ret.weights = arg3;
    ret.colour = arg4;
    return ret;
}

[numthreads(2, 1, 1)]
void structures(uint3 threadIdx : SV_GroupThreadID)
{
    uint lane = (uint)0;
    Packet_cuda_record_1_ original = (Packet_cuda_record_1_)0;
    Packet_cuda_record_1_ changed = (Packet_cuda_record_1_)0;
    Packet_cuda_record_2_ local = (Packet_cuda_record_2_)0;
    Packet_cuda_record_1_ combined = (Packet_cuda_record_1_)0;
    uint slot = (uint)0;
    int sample_ = (int)0;
    int row = (int)0;
    int column = (int)0;

    lane = threadIdx.x;
    uint _e6 = lane;
    if ((_e6 >= 2u)) {
        return;
    }
    uint _e9 = lane;
    Packet_cuda_record_1_ _e11 = ConstructPacket_cuda_record_1_(asuint(input.Load(_e9*80+0)), asfloat(input.Load2(_e9*80+8)), Constructarray2_Sample_cuda_record_0__(ConstructSample_cuda_record_0_(Constructfloat3_(asfloat(input.Load(_e9*80+16+0+0+0)), asfloat(input.Load(_e9*80+16+0+0+4)), asfloat(input.Load(_e9*80+16+0+0+8))), asuint(input.Load(_e9*80+16+0+12))), ConstructSample_cuda_record_0_(Constructfloat3_(asfloat(input.Load(_e9*80+16+16+0+0)), asfloat(input.Load(_e9*80+16+16+0+4)), asfloat(input.Load(_e9*80+16+16+0+8))), asuint(input.Load(_e9*80+16+16+12)))), Constructarray2_array2_float__(Constructarray2_float_(asfloat(input.Load(_e9*80+48+0+0)), asfloat(input.Load(_e9*80+48+0+4))), Constructarray2_float_(asfloat(input.Load(_e9*80+48+8+0)), asfloat(input.Load(_e9*80+48+8+4)))), asfloat(input.Load4(_e9*80+64)));
    original = _e11;
    Packet_cuda_record_1_ _e13 = original;
    uint _e15 = lane;
    const Packet_cuda_record_1_ _e18 = adjust(_e13, asint(asuint(int(3)) + asuint(int(_e15))));
    changed = _e18;
    {
        local.w = int(7);
        local.x = int(11);
        int _e30 = local.w;
        int _e31 = local.x;
        uint _e34 = changed.samples[int(0)].tag;
        changed.samples[int(0)].tag = (_e34 + uint(asint(asuint(_e30) + asuint(_e31))));
    }
    Packet_cuda_record_1_ _e37 = changed;
    Packet_cuda_record_1_ _e38 = original;
    const Packet_cuda_record_1_ _e39 = operator_u002b(_e37, _e38);
    combined = _e39;
    uint _e46 = original.serial;
    float _e48 = combined.weights[int(0)][int(0)];
    combined.weights[int(0)][int(0)] = (_e48 + float(_e46));
    uint _e53 = combined.serial;
    combined.colour.w = float(_e53);
    uint _e57 = lane;
    slot = (_e57 + 1u);
    uint _e63 = combined.serial;
    uint _e59 = slot;
    output.Store(0+_e59*80, asuint(_e63));
    float2 _e68 = combined.uv;
    uint _e64 = slot;
    output.Store2(8+_e64*80, asuint(_e68));
    {
        sample_ = int(0);
        uint2 loop_bound = uint2(4294967295u, 4294967295u);
        bool loop_init = true;
        while(true) {
            if (all(loop_bound == uint2(0u, 0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e74 = sample_;
                sample_ = asint(asuint(_e74) + asuint(int(1)));
            }
            loop_init = false;
            int _e72 = sample_;
            if ((_e72 < int(2))) {
            } else {
                break;
            }
            int _e83 = sample_;
            Sample_cuda_record_0_ _e85 = combined.samples[min(uint(_e83), 1u)];
            uint _e77 = slot;
            int _e80 = sample_;
            {
                Sample_cuda_record_0_ _value4 = _e85;
                {
                    float3_ _value5 = _value4.position;
                    output.Store(_e80*16+16+_e77*80+0+0, asuint(_value5.x));
                    output.Store(_e80*16+16+_e77*80+0+4, asuint(_value5.y));
                    output.Store(_e80*16+16+_e77*80+0+8, asuint(_value5.z));
                }
                output.Store(_e80*16+16+_e77*80+12, asuint(_value4.tag));
            }
        }
    }
    {
        row = int(0);
        uint2 loop_bound_1 = uint2(4294967295u, 4294967295u);
        bool loop_init_1 = true;
        while(true) {
            if (all(loop_bound_1 == uint2(0u, 0u))) { break; }
            loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
            if (!loop_init_1) {
                int _e91 = row;
                row = asint(asuint(_e91) + asuint(int(1)));
            }
            loop_init_1 = false;
            int _e89 = row;
            if ((_e89 < int(2))) {
            } else {
                break;
            }
            {
                column = int(0);
                uint2 loop_bound_2 = uint2(4294967295u, 4294967295u);
                bool loop_init_2 = true;
                while(true) {
                    if (all(loop_bound_2 == uint2(0u, 0u))) { break; }
                    loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
                    if (!loop_init_2) {
                        int _e99 = column;
                        column = asint(asuint(_e99) + asuint(int(1)));
                    }
                    loop_init_2 = false;
                    int _e97 = column;
                    if ((_e97 < int(2))) {
                    } else {
                        break;
                    }
                    int _e110 = row;
                    int _e112 = column;
                    float _e114 = combined.weights[min(uint(_e110), 1u)][min(uint(_e112), 1u)];
                    uint _e102 = slot;
                    int _e105 = row;
                    int _e107 = column;
                    output.Store(_e107*4+_e105*8+48+_e102*80, asuint(_e114));
                }
            }
        }
    }
    float4 _e119 = combined.colour;
    uint _e115 = slot;
    output.Store4(64+_e115*80, asuint(_e119));
}
