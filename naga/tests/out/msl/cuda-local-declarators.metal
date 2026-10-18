// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
};

typedef int type_1[1];
struct type_3 {
    uint value;
    char _pad1[12];
};

int cached_type(
    int value_1
) {
    int value_2 = {};
    int Number = {};
    int next = {};
    int result = {};
    value_2 = value_1;
    int _e3 = value_2;
    Number = _e3;
    int _e6 = Number;
    next = as_type<int>(as_type<uint>(_e6) + as_type<uint>(1));
    int _e10 = next;
    result = as_type<int>(as_type<uint>(_e10) * as_type<uint>(2));
    int _e12 = result;
    return _e12;
}

int pair_add(
    int left,
    int right
) {
    int left_1 = {};
    int right_1 = {};
    left_1 = left;
    right_1 = right;
    int _e4 = left_1;
    int _e5 = right_1;
    return as_type<int>(as_type<uint>(_e4) + as_type<uint>(_e5));
}
int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}


struct local_declaratorsInput {
};
kernel void local_declarators(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, device type_1 const& input [[user(fake0)]]
, constant type_3& n_1 [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    uint n = {};
    uint i = {};
    uint base = {};
    int value = {};
    int first = {};
    int second = {};
    int third = {};
    int omitted = {};
    int initialized = {};
    int final = {};
    int fixed = {};
    int fixed_next = {};
    int alias_first = {};
    int alias_next = {};
    bool positive = {};
    bool same = {};
    bool local = {};
    float low = {};
    float high = {};
    uint start = {};
    uint finish = {};
    int outside = {};
    int outside_1 = {};
    int derived = {};
    int total = {};
    int iteration = {};
    int seed = {};
    int term = {};
    int copied = {};
    int from_call = {};
    int from_prior = {};
    uint _e4 = n_1.value;
    n = _e4;
    i = (blockIdx.x * 64u) + threadIdx.x;
    uint _e14 = i;
    uint _e15 = n;
    if (_e14 < _e15) {
        uint _e19 = i;
        base = _e19 * 12u;
        uint _e22 = i;
        int _e24 = input[_e22];
        value = _e24;
        int _e26 = value;
        value = as_type<int>(as_type<uint>(_e26) + as_type<uint>(1));
        first = _e26;
        int _e30 = value;
        value = as_type<int>(as_type<uint>(_e30) + as_type<uint>(1));
        second = _e30;
        int _e34 = value;
        third = _e34;
        int _e38 = first;
        int _e41 = second;
        int _e44 = third;
        uint _e35 = base;
        out[_e35] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e38) * as_type<uint>(100))) + as_type<uint>(as_type<int>(as_type<uint>(_e41) * as_type<uint>(10))))) + as_type<uint>(_e44));
        int _e50 = value;
        uint _e47 = base;
        out[_e47 + 1u] = _e50;
        int _e54 = value;
        initialized = as_type<int>(as_type<uint>(_e54) + as_type<uint>(3));
        int _e58 = initialized;
        omitted = as_type<int>(as_type<uint>(_e58) + as_type<uint>(2));
        int _e60 = omitted;
        int _e61 = initialized;
        final = as_type<int>(as_type<uint>(_e60) + as_type<uint>(_e61));
        int _e67 = final;
        uint _e64 = base;
        out[_e64 + 2u] = _e67;
        int _e70 = value;
        fixed = as_type<int>(as_type<uint>(_e70) + as_type<uint>(1));
        int _e74 = fixed;
        fixed_next = as_type<int>(as_type<uint>(_e74) + as_type<uint>(2));
        int _e80 = fixed_next;
        uint _e77 = base;
        out[_e77 + 3u] = _e80;
        int _e83 = value;
        alias_first = as_type<int>(as_type<uint>(_e83) + as_type<uint>(4));
        int _e87 = alias_first;
        alias_next = as_type<int>(as_type<uint>(_e87) + as_type<uint>(1));
        int _e93 = alias_next;
        uint _e90 = base;
        out[_e90 + 4u] = _e93;
        int _e96 = value;
        positive = _e96 > 0;
        bool _e99 = positive;
        local = _e99;
        if (_e99) {
            uint _e101 = i;
            int _e104 = input[_e101];
            int _e106 = value;
            local = _e106 == as_type<int>(as_type<uint>(_e104) + as_type<uint>(2));
        }
        bool _e108 = local;
        same = _e108;
        bool _e113 = same;
        uint _e110 = base;
        out[_e110 + 5u] = static_cast<int>(_e113);
        int _e116 = value;
        low = static_cast<float>(_e116) * 0.5;
        float _e122 = low;
        high = _e122 + 1.5;
        float _e129 = high;
        uint _e125 = base;
        out[_e125 + 6u] = naga_f2i32(_e129 * 2.0);
        uint _e134 = i;
        start = naga_mod(_e134, 5u);
        uint _e138 = start;
        finish = _e138 + 3u;
        uint _e144 = finish;
        uint _e141 = base;
        out[_e141 + 7u] = static_cast<int>(_e144);
        int _e150 = value;
        int _e151 = cached_type(_e150);
        uint _e147 = base;
        out[_e147 + 8u] = _e151;
        int _e153 = value;
        outside = _e153;
        {
            int _e156 = value;
            outside_1 = as_type<int>(as_type<uint>(_e156) + as_type<uint>(5));
            int _e160 = outside_1;
            derived = as_type<int>(as_type<uint>(_e160) + as_type<uint>(2));
            int _e166 = derived;
            uint _e163 = base;
            out[_e163 + 9u] = _e166;
        }
        total = 0;
        {
            iteration = 0;
            int _e172 = value;
            seed = _e172;
            uint2 loop_bound = uint2(4294967295u);
            bool loop_init = true;
            while(true) {
                if (metal::all(loop_bound == uint2(0u))) { break; }
                loop_bound -= uint2(loop_bound.y == 0u, 1u);
                if (!loop_init) {
                    int _e176 = iteration;
                    iteration = as_type<int>(as_type<uint>(_e176) + as_type<uint>(1));
                }
                loop_init = false;
                int _e174 = iteration;
                if (_e174 < 4) {
                } else {
                    break;
                }
                int _e180 = iteration;
                if (_e180 == 1) {
                    continue;
                }
                int _e183 = seed;
                int _e184 = iteration;
                term = as_type<int>(as_type<uint>(_e183) + as_type<uint>(_e184));
                int _e187 = term;
                copied = _e187;
                int _e188 = copied;
                int _e189 = total;
                total = as_type<int>(as_type<uint>(_e189) + as_type<uint>(_e188));
            }
        }
        int _e195 = total;
        uint _e192 = base;
        out[_e192 + 10u] = _e195;
        int _e197 = value;
        int _e199 = pair_add(_e197, 2);
        from_call = _e199;
        int _e201 = from_call;
        int _e203 = pair_add(_e201, 3);
        from_prior = _e203;
        int _e208 = from_prior;
        int _e209 = outside;
        uint _e205 = base;
        out[_e205 + 11u] = as_type<int>(as_type<uint>(_e208) + as_type<uint>(_e209));
    }
}
