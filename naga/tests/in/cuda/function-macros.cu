struct Pair { unsigned first; unsigned second; };
__device__ unsigned advance(unsigned value) { return value + 3u; }

#define ZERO() 7u
#define ID(value) value
#define ADD(left, right) ((left) + (right))
#define TIMES(value) ((value) * 3u)
#define ALIAS ADD
#define FORWARD(name) name
#define MAKE_PAIR(left, right) Pair{left, right}
#define FIELD(record, field) ((record).field)
#define SIZE() 3
#define VECTOR(left, right) make_uint2(left, right)
#define DUPLICATE(value) ((value), (value))
#define DROP(value) 19u
#define ASSIGN(slot, value) ((slot) = (value))
#define CHOOSE(flag, yes, no) ((flag) ? (yes) : (no))
#define CALL(function, value) function(value)
#define DECLARE(type, name, value) type name = (value)
#define APPLIER() ADD

__global__ void function_macros(unsigned *out) {
    unsigned lane = threadIdx.x;
    if (lane >= 2u) return;
    unsigned start = lane * 24u;
    Pair pair = MAKE_PAIR(11u, 13u);
    unsigned values[SIZE()]{2u, 3u, 5u};
    uint2 vector = VECTOR(17u, 19u);
    unsigned counter = 0u;
    out[start] = ZERO();
    out[start + 1u] = ID(11u);
    out[start + 2u] = ADD(2u, 3u);
    out[start + 3u] = TIMES(ADD(1u, 2u));
    out[start + 4u] = ALIAS(5u, 6u);
    out[start + 5u] = FORWARD(ADD)(7u, 8u);
    out[start + 6u] = FIELD(pair, first);
    out[start + 7u] = FIELD(pair, second);
    out[start + 8u] = SIZE();
    out[start + 9u] = values[lane];
    out[start + 10u] = vector.x;
    out[start + 11u] = vector.y;
    out[start + 12u] = DUPLICATE(counter++);
    out[start + 13u] = counter;
    out[start + 14u] = DROP(counter++);
    out[start + 15u] = counter;
    out[start + 16u] = ASSIGN(values[0], 23u);
    out[start + 17u] = values[0];
    out[start + 18u] = CHOOSE(lane == 0u, 29u, 31u);
    out[start + 19u] = CALL(advance, lane);
    DECLARE(unsigned, declared, 37u);
    out[start + 20u] = declared;
    out[start + 21u] = __float_as_uint(ID(-0.0f));
    out[start + 22u] = ID(0xffff'ffffU);
    out[start + 23u] = APPLIER()(41u, lane);
}
