// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size0;
    uint size1;
    uint size2;
};

typedef int type_1[1];
struct type_3 {
    uint value;
    char _pad1[12];
};
struct type_4 {
    int inner[64];
};

int shifted(
    int value_1
) {
    int value_2 = {};
    value_2 = value_1;
    int _e3 = value_2;
    return as_type<int>(as_type<uint>(_e3) + as_type<uint>(10));
}

void discard(
    int value_3
) {
    int value_4 = {};
    value_4 = value_3;
    int _e3 = value_4;
    value_4 = as_type<int>(as_type<uint>(_e3) + as_type<uint>(1));
    return;
}

int pair(
    int x,
    int y
) {
    int x_1 = {};
    int y_1 = {};
    x_1 = x;
    y_1 = y;
    int _e5 = x_1;
    int _e7 = y_1;
    return as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e5) * as_type<uint>(10))) + as_type<uint>(_e7));
}
uint naga_mod(uint lhs, uint rhs) {
    return lhs % metal::select(rhs, 1u, rhs == 0u);
}

int naga_f2i32(float value) {
    return static_cast<int>(metal::clamp(value, -2147483600.0, 2147483500.0));
}

int naga_neg(int val) {
    return as_type<int>(-as_type<uint>(val));
}


struct conditionalsInput {
};
kernel void conditionals(
  metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, uint __local_invocation_index [[thread_index_in_threadgroup]]
, device type_1& out [[user(fake0)]]
, device type_1 const& input_a [[user(fake0)]]
, device type_1 const& input_b [[user(fake0)]]
, constant type_3& n_1 [[user(fake0)]]
, threadgroup type_4& shared_values
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    if (__local_invocation_index == 0u) {
        shared_values = {};
    }
    metal::threadgroup_barrier(metal::mem_flags::mem_threadgroup);
    uint n = {};
    uint i = {};
    uint base = {};
    bool choose_a = {};
    int local = {};
    int a = {};
    int b = {};
    int local_1 = {};
    int index_a = {};
    int index_b = {};
    int local_2 = {};
    int local_3 = {};
    int c = {};
    int local_4 = {};
    int condition = {};
    uint lane = {};
    int local_value = {};
    float local_5 = {};
    uint local_6 = {};
    bool local_7 = {};
    int local_8 = {};
    uint local_9 = {};
    int shift_value = {};
    int local_10 = {};
    uint local_11 = {};
    int value = {};
    int other = {};
    int local_12 = {};
    int count = {};
    int left = {};
    int right = {};
    uint _e5 = n_1.value;
    n = _e5;
    i = (blockIdx.x * 64u) + threadIdx.x;
    uint _e16 = i;
    uint _e17 = n;
    if (_e16 < _e17) {
        uint _e21 = i;
        base = _e21 * 25u;
        uint _e25 = i;
        choose_a = naga_mod(_e25, 2u) == 0u;
        bool _e31 = choose_a;
        if (_e31) {
            uint _e32 = i;
            int _e37 = input_a[_e32];
            local = _e37;
        } else {
            uint _e34 = i;
            int _e38 = input_b[_e34];
            local = _e38;
        }
        int _e39 = local;
        uint _e29 = base;
        out[_e29] = _e39;
        a = 3;
        b = 5;
        bool _e48 = choose_a;
        if (_e48) {
            int _e49 = a;
            a = as_type<int>(as_type<uint>(_e49) + as_type<uint>(1));
            int _e52 = shifted(_e49);
            local_1 = _e52;
        } else {
            int _e53 = b;
            b = as_type<int>(as_type<uint>(_e53) + as_type<uint>(1));
            int _e56 = shifted(_e53);
            local_1 = _e56;
        }
        int _e58 = local_1;
        uint _e45 = base;
        out[_e45 + 1u] = _e58;
        int _e64 = a;
        int _e66 = b;
        uint _e60 = base;
        out[_e60 + 2u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e64) * as_type<uint>(10))) + as_type<uint>(_e66));
        bool _e68 = choose_a;
        if (_e68) {
            int _e69 = a;
            a = as_type<int>(as_type<uint>(_e69) + as_type<uint>(1));
            discard(_e69);
        } else {
            int _e72 = b;
            b = as_type<int>(as_type<uint>(_e72) + as_type<uint>(1));
            discard(_e72);
        }
        int _e80 = a;
        int _e82 = b;
        uint _e76 = base;
        out[_e76 + 3u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e80) * as_type<uint>(10))) + as_type<uint>(_e82));
        index_a = 0;
        index_b = 0;
        bool _e88 = choose_a;
        if (_e88) {
            int _e89 = index_a;
            index_a = as_type<int>(as_type<uint>(_e89) + as_type<uint>(1));
        } else {
            int _e92 = index_b;
            index_b = as_type<int>(as_type<uint>(_e92) + as_type<uint>(1));
        }
        int _e100 = index_a;
        int _e102 = index_b;
        uint _e96 = base;
        out[_e96 + 4u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e100) * as_type<uint>(10))) + as_type<uint>(_e102));
        a = 3;
        b = 5;
        bool _e106 = choose_a;
        if (_e106) {
            a = 7;
        } else {
            b = 7;
        }
        int _e113 = a;
        int _e115 = b;
        uint _e109 = base;
        out[_e109 + 5u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e113) * as_type<uint>(10))) + as_type<uint>(_e115));
        bool _e117 = choose_a;
        if (_e117) {
            int _e119 = a;
            a = as_type<int>(as_type<uint>(_e119) + as_type<uint>(2));
        } else {
            int _e121 = b;
            b = as_type<int>(as_type<uint>(_e121) + as_type<uint>(2));
        }
        int _e128 = a;
        int _e130 = b;
        uint _e124 = base;
        out[_e124 + 6u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e128) * as_type<uint>(10))) + as_type<uint>(_e130));
        bool _e136 = choose_a;
        if (_e136) {
            int _e137 = a;
            a = as_type<int>(as_type<uint>(_e137) + as_type<uint>(1));
            int _e144 = a;
            local_2 = _e144;
        } else {
            int _e140 = b;
            b = as_type<int>(as_type<uint>(_e140) + as_type<uint>(1));
            int _e145 = b;
            local_2 = _e145;
        }
        int _e146 = local_2;
        uint _e133 = base;
        out[_e133 + 7u] = _e146;
        bool _e151 = choose_a;
        if (_e151) {
            int _e153 = a;
            a = as_type<int>(as_type<uint>(_e153) + as_type<uint>(1));
            local_3 = _e153;
        } else {
            int _e156 = b;
            b = as_type<int>(as_type<uint>(_e156) + as_type<uint>(1));
            local_3 = _e156;
        }
        int _e159 = local_3;
        uint _e148 = base;
        out[_e148 + 8u] = _e159;
        int _e165 = a;
        int _e167 = b;
        uint _e161 = base;
        out[_e161 + 9u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e165) * as_type<uint>(100))) + as_type<uint>(_e167));
        bool _e170 = choose_a;
        c = static_cast<int>(_e170);
        b = 7;
        int _e177 = c;
        if (_e177 != 0) {
            c = 0;
            int _e182 = c;
            c = as_type<int>(as_type<uint>(_e182) + as_type<uint>(1));
            local_4 = _e182;
        } else {
            b = 0;
            int _e185 = b;
            b = as_type<int>(as_type<uint>(_e185) + as_type<uint>(1));
            local_4 = _e185;
        }
        int _e188 = local_4;
        uint _e174 = base;
        out[_e174 + 10u] = _e188;
        int _e194 = c;
        int _e196 = b;
        uint _e190 = base;
        out[_e190 + 11u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e194) * as_type<uint>(10))) + as_type<uint>(_e196));
        condition = 0;
        a = 2;
        b = 4;
        condition = 1;
        int _e206 = condition;
        int _e202 = condition;
        if (_e202 != 0) {
            a = _e206;
        } else {
            b = _e206;
        }
        int _e212 = a;
        int _e214 = b;
        uint _e208 = base;
        out[_e208 + 12u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e212) * as_type<uint>(10))) + as_type<uint>(_e214));
        a = 2;
        b = 4;
        c = 6;
        bool _e219 = choose_a;
        if (_e219) {
            uint _e221 = i;
            if (naga_mod(_e221, 4u) == 0u) {
                int _e226 = a;
                a = as_type<int>(as_type<uint>(_e226) + as_type<uint>(3));
            } else {
                int _e228 = b;
                b = as_type<int>(as_type<uint>(_e228) + as_type<uint>(3));
            }
        } else {
            int _e230 = c;
            c = as_type<int>(as_type<uint>(_e230) + as_type<uint>(3));
        }
        int _e237 = a;
        int _e240 = b;
        int _e243 = c;
        uint _e233 = base;
        out[_e233 + 13u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e237) * as_type<uint>(100))) + as_type<uint>(as_type<int>(as_type<uint>(_e240) * as_type<uint>(10))))) + as_type<uint>(_e243));
        lane = threadIdx.x;
        uint _e247 = lane;
        shared_values.inner[_e247] = 20;
        local_value = 10;
        bool _e252 = choose_a;
        if (_e252) {
            int _e256 = local_value;
            local_value = as_type<int>(as_type<uint>(_e256) + as_type<uint>(5));
        } else {
            uint _e253 = lane;
            int _e258 = shared_values.inner[_e253];
            shared_values.inner[_e253] = as_type<int>(as_type<uint>(_e258) + as_type<uint>(5));
        }
        int _e265 = local_value;
        uint _e267 = lane;
        int _e269 = shared_values.inner[_e267];
        uint _e261 = base;
        out[_e261 + 14u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e265) * as_type<uint>(100))) + as_type<uint>(_e269));
        bool _e271 = choose_a;
        if (_e271) {
            uint _e272 = lane;
            shared_values.inner[_e272] = 30;
        } else {
            uint _e275 = base;
            out[_e275 + 15u] = 30;
        }
        uint _e283 = lane;
        int _e285 = shared_values.inner[_e283];
        uint _e280 = base;
        out[_e280 + 16u] = _e285;
        bool _e290 = choose_a;
        if (_e290) {
            local_5 = 1.5;
        } else {
            local_5 = static_cast<float>(3);
        }
        float _e295 = local_5;
        uint _e287 = base;
        out[_e287 + 17u] = naga_f2i32(_e295);
        bool _e301 = choose_a;
        if (_e301) {
            local_6 = static_cast<uint>(naga_neg(1));
        } else {
            local_6 = 2u;
        }
        uint _e307 = local_6;
        uint _e298 = base;
        out[_e298 + 18u] = static_cast<int>(_e307);
        bool _e313 = choose_a;
        if (_e313) {
            local_7 = true;
        } else {
            local_7 = false;
        }
        bool _e317 = local_7;
        uint _e310 = base;
        out[_e310 + 19u] = static_cast<int>(_e317);
        bool _e323 = choose_a;
        if (_e323) {
            local_8 = static_cast<int>(true);
        } else {
            local_8 = 2;
        }
        int _e328 = local_8;
        uint _e320 = base;
        out[_e320 + 20u] = _e328;
        bool _e333 = choose_a;
        if (_e333) {
            local_9 = 4u;
        } else {
            local_9 = 8u;
        }
        uint _e337 = local_9;
        uint _e330 = base;
        out[_e330 + 21u] = static_cast<int>(_e337);
        a = 3;
        shift_value = 2;
        bool _e346 = choose_a;
        if (_e346) {
            int _e348 = shift_value;
            local_10 = _e348;
        } else {
            int _e349 = a;
            local_10 = _e349;
        }
        int _e350 = local_10;
        shift_value = 1;
        int _e352 = shift_value;
        local_11 = static_cast<uint>(_e352);
        uint _e355 = local_11;
        uint _e343 = base;
        out[_e343 + 22u] = _e350 << _e355;
        value = 3;
        other = 7;
        bool _e365 = choose_a;
        if (_e365) {
            int _e367 = value;
            local_12 = _e367;
        } else {
            int _e368 = other;
            local_12 = _e368;
        }
        int _e369 = local_12;
        int _e370 = value;
        value = as_type<int>(as_type<uint>(_e370) + as_type<uint>(1));
        int _e373 = pair(_e369, _e370);
        uint _e362 = base;
        out[_e362 + 23u] = _e373;
        count = 0;
        left = 0;
        right = 0;
        {
            uint2 loop_bound = uint2(4294967295u);
            bool loop_init = true;
            while(true) {
                if (metal::all(loop_bound == uint2(0u))) { break; }
                loop_bound -= uint2(loop_bound.y == 0u, 1u);
                if (!loop_init) {
                    bool _e383 = choose_a;
                    if (_e383) {
                        int _e384 = left;
                        left = as_type<int>(as_type<uint>(_e384) + as_type<uint>(1));
                    } else {
                        int _e387 = right;
                        right = as_type<int>(as_type<uint>(_e387) + as_type<uint>(1));
                    }
                }
                loop_init = false;
                int _e381 = count;
                if (_e381 < 3) {
                } else {
                    break;
                }
                int _e390 = count;
                count = as_type<int>(as_type<uint>(_e390) + as_type<uint>(1));
                int _e394 = count;
                if (_e394 == 2) {
                    continue;
                }
            }
        }
        int _e401 = left;
        int _e404 = right;
        int _e407 = count;
        uint _e397 = base;
        out[_e397 + 24u] = as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(as_type<int>(as_type<uint>(_e401) * as_type<uint>(100))) + as_type<uint>(as_type<int>(as_type<uint>(_e404) * as_type<uint>(10))))) + as_type<uint>(_e407));
    }
}
