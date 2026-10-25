#define DEVICE __device__
#define KERNEL __global__
#define WORD unsigned
#define BUFFER WORD *out
#define INDEX threadIdx.x
#define LANES 2u
#define COUNT 3
#define TOTAL 2u + 3u
#define PLUS +
#define SHIFT <<
#define CAST static_cast<WORD>
#define EMPTY
#define ENABLED
#define FORWARD later
#define SELF SELF
#define LEFT RIGHT
#define RIGHT LEFT

struct Packet { WORD first; WORD second; };
#define RECORD Packet
DEVICE WORD FORWARD(WORD value);

#ifdef ENABLED
#define BASE 7u
#elif unsupported(())
#define BASE 99u
#endif
#ifndef ABSENT
#define CHOICE 31u
#else
#define CHOICE 99u
#endif
#ifdef ABSENT
#include "not-read.h"
#define FUNCTION(value) value
#if unsupported_expression
this is not supported CUDA;
#endif
#endif

KERNEL void preprocessing(BUFFER) {
    WORD lane = INDEX;
    if (lane >= LANES) return;
    WORD start = lane * 24u;
    out[start + 0u] = BASE;
#define VALUE 11u
    out[start + 1u] = VALUE;
#undef VALUE
#define VALUE 22u
    out[start + 2u] = VALUE;
    out[start + 3u] = TOTAL * 4u;
    out[start + 4u] = 2u PLUS 3u;
    out[start + 5u] = 3u SHIFT 2u;
    out[start + 6u] = FORWARD(lane);
    RECORD original{lane + 1u, lane + 2u};
    RECORD copied = original;
    out[start + 7u] = copied.first;
    out[start + 8u] = copied.second;
    WORD SELF = 17u, LEFT = 19u, RIGHT = 23u;
    out[start + 9u] = SELF;
    out[start + 10u] = LEFT;
    out[start + 11u] = RIGHT;
    WORD counter = 0u;
#define STEP ++counter
    RECORD ordered{STEP, STEP};
    out[start + 12u] = ordered.first;
    out[start + 13u] = ordered.second;
    out[start + 14u] = counter;
    out[start + 15u] = CHOICE;
    WORD values[COUNT]{2u, 4u, 6u};
    out[start + 16u] = values[lane];
    WORD index = 0u;
    out[start + 17u] = values[index++];
    out[start + 18u] = index;
    out[start + 19u] = CAST(true);
#ifndef EMPTY
    out[start + 20u] = 99u;
#else
    out[start + 20u] = 61u;
#endif
    out[start + 21u] = lane;
    out[start + 22u] = __float_as_uint(-0.0f);
    out[start + 23u] = 0xffff'ffffU;
    EMPTY
}

DEVICE WORD FORWARD(WORD value) { return value + 5u; }
