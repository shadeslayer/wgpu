// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
};

typedef int type_1[1];
struct type_2 {
    int value;
    char _pad1[12];
};

struct loopsInput {
};
kernel void loops(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, constant type_2& n_1 [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    int n = {};
    int sum = {};
    int i = {};
    int i_1 = {};
    int _e3 = n_1.value;
    n = _e3;
    sum = 0;
    {
        i = 0;
        uint2 loop_bound = uint2(4294967295u);
        bool loop_init = true;
        while(true) {
            if (metal::all(loop_bound == uint2(0u))) { break; }
            loop_bound -= uint2(loop_bound.y == 0u, 1u);
            if (!loop_init) {
                int _e13 = i;
                i = as_type<int>(as_type<uint>(_e13) + as_type<uint>(1));
            }
            loop_init = false;
            int _e9 = i;
            int _e10 = n;
            if (_e9 < _e10) {
            } else {
                break;
            }
            int _e16 = i;
            if (_e16 == 2) {
                continue;
            }
            int _e19 = i;
            if (_e19 == 7) {
                break;
            }
            int _e21 = sum;
            int _e22 = i;
            sum = as_type<int>(as_type<uint>(_e21) + as_type<uint>(_e22));
        }
    }
    i_1 = 0;
    uint2 loop_bound_1 = uint2(4294967295u);
    while(true) {
        if (metal::all(loop_bound_1 == uint2(0u))) { break; }
        loop_bound_1 -= uint2(loop_bound_1.y == 0u, 1u);
        int _e27 = i_1;
        if (_e27 < 3) {
        } else {
            break;
        }
        {
            int _e30 = i_1;
            i_1 = as_type<int>(as_type<uint>(_e30) + as_type<uint>(1));
            int _e33 = i_1;
            if (_e33 == 2) {
                continue;
            }
            int _e36 = sum;
            sum = as_type<int>(as_type<uint>(_e36) + as_type<uint>(10));
        }
    }
    uint2 loop_bound_2 = uint2(4294967295u);
    bool loop_init_1 = true;
    while(true) {
        if (metal::all(loop_bound_2 == uint2(0u))) { break; }
        loop_bound_2 -= uint2(loop_bound_2.y == 0u, 1u);
        if (!loop_init_1) {
            int _e45 = sum;
            if (!((sum < 150))) {
                break;
            }
        }
        loop_init_1 = false;
        int _e39 = sum;
        sum = as_type<int>(as_type<uint>(_e39) + as_type<uint>(100));
        int _e42 = sum;
        if (_e42 > 200) {
            break;
        }
    }
    int _e51 = sum;
    out[threadIdx.x] = _e51;
}
