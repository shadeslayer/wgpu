#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct float3_ {
    float x;
    float y;
    float z;
};
const int Base_1 = 2;
const uint High = 2147483648u;
const float Scale = 0.5;
const int Second = 3;
const int After = 4;

layout(std430) readonly buffer type_4_block_0Compute { int _group_0_binding_0_cs[]; };

layout(std430) buffer type_5_block_1Compute { uint _group_0_binding_1_cs[]; };


int accumulate(int runtime_1) {
    int runtime_2 = 0;
    int Size_1 = 0;
    int local_8 = 0;
    int local_9[3] = int[3](0, 0, 0);
    int total = 0;
    int i_1 = 0;
    runtime_2 = runtime_1;
    Size_1 = 3;
    local_9 = int[3](0, 0, 0);
    total = 0;
    {
        i_1 = 0;
        bool loop_init = true;
        while(true) {
            if (!loop_init) {
                int _e13 = i_1;
                i_1 = (_e13 + 1);
            }
            loop_init = false;
            int _e10 = i_1;
            int _e11 = Size_1;
            if ((_e10 < _e11)) {
            } else {
                break;
            }
            int _e18 = runtime_2;
            int _e19 = i_1;
            int _e16 = i_1;
            local_9[_e16] = (_e18 + _e19);
            int _e21 = i_1;
            int _e23 = local_9[_e21];
            int _e24 = total;
            total = (_e24 + _e23);
        }
    }
    int _e26 = total;
    return _e26;
}

void main() {
    int runtime = 0;
    int Chosen = 0;
    int local = 0;
    int Size = 0;
    int values[5] = int[5](0, 0, 0, 0, 0);
    int SignedHigh = 0;
    uint local_1 = 0u;
    int Negative = 0;
    uint local_2 = 0u;
    uint All = 0u;
    uint Wrapped = 0u;
    uint local_3 = 0u;
    uint TruncatedZero = 0u;
    int Minimum = 0;
    uint Large = 0u;
    bool Ready = false;
    int Comma = 0;
    int Lazy = 0;
    int local_4 = 0;
    bool Skipped = false;
    bool local_5 = false;
    bool Yes = false;
    bool local_6 = false;
    int VectorCount = 0;
    float3_ vectors[2] = float3_[2](float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0));
    int Base[2] = int[2](0, 0);
    int i = 0;
    int Count = 0;
    int scratch[3] = int[3](0, 0, 0);
    int Safe = 0;
    int local_7 = 0;
    int _e5 = _group_0_binding_0_cs[0];
    runtime = _e5;
    Chosen = 3;
    Size = 4;
    values = int[5](0, 0, 0, 0, 0);
    int _e14 = runtime;
    values[0] = _e14;
    int _e19 = _group_0_binding_0_cs[1];
    int _e15 = Size;
    values[_e15] = _e19;
    _group_0_binding_1_cs[0] = uint(Base_1);
    _group_0_binding_1_cs[1] = High;
    _group_0_binding_1_cs[2] = floatBitsToUint(Scale);
    int _e33 = Chosen;
    _group_0_binding_1_cs[3] = uint(_e33);
    int _e37 = runtime;
    int _e38 = accumulate(_e37);
    _group_0_binding_1_cs[4] = uint(_e38);
    int _e42 = Size;
    int _e44 = values[_e42];
    _group_0_binding_1_cs[5] = uint(_e44);
    int _e50 = values[1];
    _group_0_binding_1_cs[6] = uint(_e50);
    int _e54 = runtime;
    _group_0_binding_1_cs[7] = uint((float(_e54) * Scale));
    SignedHigh = -2147483648;
    Negative = -2;
    All = 4294967295u;
    Wrapped = 0u;
    int _e69 = SignedHigh;
    _group_0_binding_1_cs[8] = uint(_e69);
    int _e73 = Negative;
    _group_0_binding_1_cs[9] = uint(_e73);
    uint _e77 = All;
    _group_0_binding_1_cs[10] = _e77;
    uint _e80 = Wrapped;
    _group_0_binding_1_cs[11] = _e80;
    TruncatedZero = 0u;
    Minimum = -2147483648;
    Large = 4294967040u;
    Ready = true;
    uint _e91 = TruncatedZero;
    _group_0_binding_1_cs[12] = _e91;
    int _e94 = Minimum;
    _group_0_binding_1_cs[13] = uint(_e94);
    uint _e98 = Large;
    _group_0_binding_1_cs[14] = _e98;
    bool _e101 = Ready;
    _group_0_binding_1_cs[15] = uint(_e101);
    Comma = 4;
    Lazy = 6;
    int _e109 = Comma;
    _group_0_binding_1_cs[16] = uint(_e109);
    int _e113 = Lazy;
    _group_0_binding_1_cs[17] = uint(_e113);
    int _e117 = runtime;
    _group_0_binding_1_cs[18] = uint(_e117);
    Skipped = false;
    Yes = true;
    bool _e125 = Skipped;
    _group_0_binding_1_cs[19] = uint(_e125);
    bool _e129 = Yes;
    _group_0_binding_1_cs[20] = uint(_e129);
    int _e133 = runtime;
    _group_0_binding_1_cs[21] = uint(_e133);
    VectorCount = 2;
    vectors = float3_[2](float3_(0.0, 0.0, 0.0), float3_(0.0, 0.0, 0.0));
    int _e144 = _group_0_binding_0_cs[1];
    vectors[1].y = float(_e144);
    float _e151 = vectors[1].y;
    _group_0_binding_1_cs[22] = uint(_e151);
    {
        Base = int[2](0, 0);
        int _e159 = Base[1];
        _group_0_binding_1_cs[23] = uint(_e159);
    }
    {
        i = 0;
        bool loop_init_1 = true;
        while(true) {
            if (!loop_init_1) {
                int _e166 = i;
                i = (_e166 + 1);
            }
            loop_init_1 = false;
            int _e164 = i;
            if ((_e164 < 3)) {
            } else {
                break;
            }
            Count = 3;
            scratch = int[3](0, 0, 0);
            int _e179 = scratch[2];
            int _e174 = i;
            _group_0_binding_1_cs[(24 + _e174)] = uint(_e179);
            int _e183 = runtime;
            int _e184 = i;
            scratch[2] = (_e183 + _e184);
        }
    }
    int _e190 = _group_0_binding_0_cs[1];
    int _e191 = accumulate(_e190);
    _group_0_binding_1_cs[27] = uint(_e191);
    _group_0_binding_1_cs[28] = uint(Second);
    Safe = 3;
    int _e203 = runtime;
    int _e204 = _group_0_binding_0_cs[1];
    int _e206 = Safe;
    _group_0_binding_1_cs[29] = uint((((_e203 + _e204) + _e206) - 3));
}

