// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
};

struct type_3 {
    int inner[3];
};
typedef int type_4[1];
typedef uint type_5[1];
struct type_6 {
    int inner[5];
};
struct float3_ {
    float x;
    float y;
    float z;
};
struct type_8 {
    float3_ inner[2];
};
struct type_9 {
    int inner[2];
};
constant int Base_1 = 2;
constant uint High = 2147483648u;
constant float Scale = 0.5;
constant int Second = 3;
constant int After = 4;

int accumulate(
    int runtime_1
) {
    int runtime_2 = {};
    int Size_1 = {};
    int local_8 = {};
    type_3 local_9 = {};
    int total = {};
    int i_1 = {};
    runtime_2 = runtime_1;
    Size_1 = 3;
    local_9 = type_3 {};
    total = 0;
    {
        i_1 = 0;
        uint2 loop_bound = uint2(4294967295u);
        bool loop_init = true;
        while(true) {
            if (metal::all(loop_bound == uint2(0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e13 = i_1;
                i_1 = as_type<int>(as_type<uint>(_e13) + as_type<uint>(1));
            }
            loop_init = false;
            int _e10 = i_1;
            int _e11 = Size_1;
            if (_e10 < _e11) {
            } else {
                break;
            }
            int _e18 = runtime_2;
            int _e19 = i_1;
            int _e16 = i_1;
            local_9.inner[_e16] = as_type<int>(as_type<uint>(_e18) + as_type<uint>(_e19));
            int _e21 = i_1;
            int _e23 = local_9.inner[_e21];
            int _e24 = total;
            total = as_type<int>(as_type<uint>(_e24) + as_type<uint>(_e23));
        }
    }
    int _e26 = total;
    return _e26;
}
uint naga_f2u32(float value) {
    return static_cast<uint>(metal::clamp(value, 0.0, 4294967000.0));
}


kernel void constant_expressions(
  device type_4 const& input [[user(fake0)]]
, device type_5& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int runtime = {};
    int Chosen = {};
    int local = {};
    int Size = {};
    type_6 values = {};
    int SignedHigh = {};
    uint local_1 = {};
    int Negative = {};
    uint local_2 = {};
    uint All = {};
    uint Wrapped = {};
    uint local_3 = {};
    uint TruncatedZero = {};
    int Minimum = {};
    uint Large = {};
    bool Ready = {};
    int Comma = {};
    int Lazy = {};
    int local_4 = {};
    bool Skipped = {};
    bool local_5 = {};
    bool Yes = {};
    bool local_6 = {};
    int VectorCount = {};
    type_8 vectors = {};
    type_9 Base = {};
    int i = {};
    int Count = {};
    type_3 scratch = {};
    int Safe = {};
    int local_7 = {};
    int _e5 = input[0];
    runtime = _e5;
    Chosen = 3;
    Size = 4;
    values = type_6 {};
    int _e14 = runtime;
    values.inner[0] = _e14;
    int _e19 = input[1];
    int _e15 = Size;
    values.inner[_e15] = _e19;
    out[0] = static_cast<uint>(Base_1);
    out[1] = High;
    out[2] = as_type<uint>(Scale);
    int _e33 = Chosen;
    out[3] = static_cast<uint>(_e33);
    int _e37 = runtime;
    int _e38 = accumulate(_e37);
    out[4] = static_cast<uint>(_e38);
    int _e42 = Size;
    int _e44 = values.inner[_e42];
    out[5] = static_cast<uint>(_e44);
    int _e50 = values.inner[1];
    out[6] = static_cast<uint>(_e50);
    int _e54 = runtime;
    out[7] = naga_f2u32(static_cast<float>(_e54) * Scale);
    SignedHigh = (-2147483647 - 1);
    Negative = -2;
    All = 4294967295u;
    Wrapped = 0u;
    int _e69 = SignedHigh;
    out[8] = static_cast<uint>(_e69);
    int _e73 = Negative;
    out[9] = static_cast<uint>(_e73);
    uint _e77 = All;
    out[10] = _e77;
    uint _e80 = Wrapped;
    out[11] = _e80;
    TruncatedZero = 0u;
    Minimum = (-2147483647 - 1);
    Large = 4294967040u;
    Ready = true;
    uint _e91 = TruncatedZero;
    out[12] = _e91;
    int _e94 = Minimum;
    out[13] = static_cast<uint>(_e94);
    uint _e98 = Large;
    out[14] = _e98;
    bool _e101 = Ready;
    out[15] = static_cast<uint>(_e101);
    Comma = 4;
    Lazy = 6;
    int _e109 = Comma;
    out[16] = static_cast<uint>(_e109);
    int _e113 = Lazy;
    out[17] = static_cast<uint>(_e113);
    int _e117 = runtime;
    out[18] = static_cast<uint>(_e117);
    Skipped = false;
    Yes = true;
    bool _e125 = Skipped;
    out[19] = static_cast<uint>(_e125);
    bool _e129 = Yes;
    out[20] = static_cast<uint>(_e129);
    int _e133 = runtime;
    out[21] = static_cast<uint>(_e133);
    VectorCount = 2;
    vectors = type_8 {};
    int _e144 = input[1];
    vectors.inner[1].y = static_cast<float>(_e144);
    float _e151 = vectors.inner[1].y;
    out[22] = naga_f2u32(_e151);
    {
        Base = type_9 {};
        int _e159 = Base.inner[1];
        out[23] = static_cast<uint>(_e159);
    }
    {
        i = 0;
        uint2 loop_bound_1 = uint2(4294967295u);
        bool loop_init_1 = true;
        while(true) {
            if (metal::all(loop_bound_1 == uint2(0u))) { break; }
            loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
            if (!loop_init_1) {
                int _e166 = i;
                i = as_type<int>(as_type<uint>(_e166) + as_type<uint>(1));
            }
            loop_init_1 = false;
            int _e164 = i;
            if (_e164 < 3) {
            } else {
                break;
            }
            Count = 3;
            scratch = type_3 {};
            int _e179 = scratch.inner[2];
            int _e174 = i;
            out[as_type<int>(as_type<uint>(24) + as_type<uint>(_e174))] = static_cast<uint>(_e179);
            int _e183 = runtime;
            int _e184 = i;
            scratch.inner[2] = as_type<int>(as_type<uint>(_e183) + as_type<uint>(_e184));
        }
    }
    int _e190 = input[1];
    int _e191 = accumulate(_e190);
    out[27] = static_cast<uint>(_e191);
    out[28] = static_cast<uint>(Second);
    Safe = 3;
    int _e203 = runtime;
    int _e204 = input[1];
    int _e206 = Safe;
    out[29] = static_cast<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e203) + as_type<uint>(_e204))) + as_type<uint>(_e206))) - as_type<uint>(3)));
}
