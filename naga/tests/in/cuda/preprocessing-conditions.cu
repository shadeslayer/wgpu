#define ENABLED
#define WIDTH 32
#define ALIAS ABSENT
#define STEP counter++

#if defined ENABLED && defined(ALIAS) && !defined(ABSENT)
#define BASE 7u
#else
#define BASE 99u
#endif

__device__ unsigned later(unsigned value);

__global__ void preprocessing_conditions(unsigned *out) {
    unsigned lane = threadIdx.x;
    if (lane >= 2u) return;
    unsigned start = lane * 24u;
    out[start] = BASE;
#if WIDTH * 2 == 64 && 0xffff'ffff'ffff'ffffULL > 0
    out[start + 1u] = 11u;
#else
    out[start + 1u] = 99u;
#endif
#if -1 < 0xffffffff
    out[start + 2u] = 13u;
#else
    out[start + 2u] = 99u;
#endif
#if -1 < 0xffffffffu
    out[start + 3u] = 99u;
#else
    out[start + 3u] = 17u;
#endif
#if (1u << 63) > 0
    out[start + 4u] = 19u;
#else
    out[start + 4u] = 99u;
#endif
#if (1LL << 63) < 0
    out[start + 5u] = 23u;
#else
    out[start + 5u] = 99u;
#endif
#if (-3 >> 1) == -2
    out[start + 6u] = 29u;
#else
    out[start + 6u] = 99u;
#endif
#if 0 && (1 / 0)
    out[start + 7u] = 99u;
#else
    out[start + 7u] = 31u;
#endif
#if 1 || (9223372036854775807LL + 1)
    out[start + 8u] = 37u;
#else
    out[start + 8u] = 99u;
#endif
#if (0 ? 1u : -1) < 0
    out[start + 9u] = 99u;
#else
    out[start + 9u] = 41u;
#endif
#if (1 ? 7 : (1u / 0)) == 7u
    out[start + 10u] = 43u;
#else
    out[start + 10u] = 99u;
#endif
#if (0, 5) == 5
    out[start + 11u] = 47u;
#else
    out[start + 11u] = 99u;
#endif
#if (1 ? 0, 9 : 0) == 9
    out[start + 12u] = 53u;
#else
    out[start + 12u] = 99u;
#endif
#if '\x41' == 'A'
    out[start + 13u] = 59u;
#else
    out[start + 13u] = 99u;
#endif
#if not false and (3 bitand 1) == 1
    out[start + 14u] = 61u;
#else
    out[start + 14u] = 99u;
#endif
#if defined(ALIAS) && !ALIAS
    out[start + 15u] = 67u;
#else
    out[start + 15u] = 99u;
#endif
#undef WIDTH
#if WIDTH
    out[start + 16u] = 99u;
#elifdef ABSENT
    out[start + 16u] = 99u;
#elifndef WIDTH
    out[start + 16u] = 71u;
#else
    out[start + 16u] = 99u;
#endif
    out[start + 17u] = later(lane);
    unsigned counter = 0u;
    out[start + 18u] = STEP;
    out[start + 19u] = counter;
    out[start + 20u] = static_cast<unsigned>(true);
    out[start + 21u] = lane;
    out[start + 22u] = __float_as_uint(-0.0f);
    out[start + 23u] = 0xffff'ffffU;
#if 0
#if __LINE__ + invalid((
    unknown source is ignored;
#endif
#endif
}

__device__ unsigned later(unsigned value) { return value + 73u; }
