use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        canonical(),
        prescan_and_rescan(),
        conditions_and_opaque_arguments(),
    ]
}

fn canonical() -> ShaderTest {
    let mut expected = [SENTINEL; 56];
    for lane in 0..2 {
        expected[lane * 24..lane * 24 + 24].copy_from_slice(&[
            7,
            11,
            5,
            9,
            11,
            15,
            11,
            13,
            3,
            lane as u32 + 2,
            17,
            19,
            1,
            2,
            19,
            2,
            23,
            23,
            if lane == 0 { 29 } else { 31 },
            lane as u32 + 3,
            37,
            0x8000_0000,
            0xffff_ffff,
            lane as u32 + 41,
        ]);
    }
    module_case(
        "function macros preserve helpers, records, vectors, textual effects and literal bits",
        include_str!("../../../../../naga/tests/in/cuda/function-macros.cu"),
        [2, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 56],
        )],
        0,
        [1, 1, 1],
    )
}

fn prescan_and_rescan() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        5, 7, 9, 10, 11, 9, 61, 12, 1, 2, 61, 2, 0, 2, 1, 3, 13, 21, 32, 47, 5, 3, 67, 3, 107, 116,
        79, 61, 61, 83, 89, 97,
    ]);
    module_case(
        "prescan and source continuations retain suppression and defined runtime sequencing",
        r###"
__device__ unsigned self(unsigned value) { return value + 10u; }
__device__ unsigned left(unsigned value) { return value + 20u; }
__device__ unsigned right(unsigned value) { return value + 30u; }
__device__ unsigned P(unsigned value) { return value + 100u; }
__device__ unsigned T(unsigned value) { return value + 100u; }
__device__ int f(int value) { return value + 1; }
__device__ int t(int value) { return value + 2; }
#define x 3
#define f(value) f(x * (value))
#undef x
#define x 2
#define g f
#define t(value) value
#define ID(value) value
#define INC(value) ((value) + 1u)
#define ALIAS INC
#define RETURN_INC() INC
#define APPLY(function, value) function(value)
#define COMMA 4u, 9u
#define SECOND(first, second) second
#define EXPAND_ARGUMENT(value) SECOND(value)
#define DROP(value) 61u
#define BAD(first, second) first
#define DUPLICATE(value) ((value), (value))
#define BOTH(left, right) ((left) && (right))
#define self(value) self(value)
#define left(value) right(value)
#define right(value) left(value)
#define NOT_CALLED(value) value
#define PLUS(value) value +
#define ZERO()
#define EMPTY(value)
#define P(value) Q(value)
#define Q(value) value
#define T(value) value + T
#define A(value) B
#define B(value) A(value)
#define ONE_EMPTY(value) 89u
#define TWO_EMPTY(first, second) 97u
__global__ void prescan_and_rescan(unsigned *out) {
    unsigned counter = 0u;
    unsigned NOT_CALLED = 47u;
    unsigned value = 3u;
    unsigned B = 79u;
    int y = 3, z[1]{5};
#define z z[0]
    out[0] = ID(ID(5u));
    out[1] = INC(INC(5u));
    out[2] = ALIAS (8u);
    out[3] = RETURN_INC()(9u);
    out[4] = APPLY(INC, 10u);
    out[5] = EXPAND_ARGUMENT(COMMA);
    out[6] = DROP(__LINE__);
    out[7] = f(y + 1) + f(f(z)) % t(t(g)(0) + t)(1);
    out[8] = DUPLICATE(counter++);
    out[9] = counter;
    out[10] = DROP(counter++);
    out[11] = counter;
    out[12] = BOTH(false, ++counter);
    out[13] = counter;
    out[14] = BOTH(true, ++counter);
    out[15] = counter;
    out[16] = self(3u);
    out[17] = left(1u);
    out[18] = right(2u);
    out[19] = NOT_CALLED;
    out[20] = PLUS(value)+ 2u;
    out[21] = value;
    out[22] = ZERO() 67u;
    EMPTY(__LINE__) out[23] = counter;
    out[24] = ID(P(P))(7u);
    out[25] = T(7u)(9u);
    out[26] = A(1)(2);
    out[27] = DROP(u8R"tag((, __LINE__, ))tag");
    out[28] = DROP(09);
    out[29] = SECOND(, 83u);
    out[30] = ONE_EMPTY();
    out[31] = TWO_EMPTY(,);
}
"###,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 40],
        )],
        0,
        [1, 1, 1],
    )
}

fn conditions_and_opaque_arguments() -> ShaderTest {
    let mut expected = [SENTINEL; 24];
    expected[..16].copy_from_slice(&[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53]);
    module_case(
        "condition calls share fixed64 semantics while unused arguments remain opaque",
        r###"
#define ID(value) value
#define ADD(left, right) ((left) + (right))
#define DROP(value) 1
#define EMPTY(value)
#define CODE() 'A'
#define ALIAS ADD
#define APPLY(function, left, right) function(left, right)
#define NAMED(defined) defined
#define PROVIDER(__LINE__) __LINE__
#define FEATURE(__has_include) __has_include
#define RETURN_ID() ID
#if ADD(0xffffffffffffffffu, 1u) != 0
invalid;
#endif
#if !defined(ID) || !defined CODE || DROP(defined()) != 1
invalid;
#endif
#if !DROP(defined(__LINE__)) || !DROP(__has_include(<not_read.h>))
invalid;
#endif
#if CODE() != 65 || ALIAS(2, 3) != 5 || APPLY(ADD, 3, 4) != 7
invalid;
#endif
#if NAMED(7) != 7 || PROVIDER(11) != 11 || FEATURE(13) != 13
invalid;
#endif
#if RETURN_ID()(17) != 17 || DROP(09) != 1 || (ID(0) && ID(1 / 0))
invalid;
#endif
#if ID(0)
invalid;
#elif ID(1)
#define SELECTED 53u
#elif __LINE__
invalid;
#endif
__global__ void conditions_and_opaque_arguments(unsigned *out) {
    out[0] = ID(2u);
    out[1] = ID(/* comma , */ 3u);
    out[2] = ADD(2u, 3u);
    out[3] = NAMED(7u);
    out[4] = PROVIDER(11u);
    out[5] = FEATURE(13u);
    out[6] = RETURN_ID()(17u);
    EMPTY("), __LINE__, (") out[7] = 19u;
    EMPTY(')') out[8] = 23u;
    EMPTY(R"mark((, __LINE__, ))mark") out[9] = 29u;
    out[10] = ID(31u);
    out[11] = ID
    (
        37u
    );
    out[12] = ID(41u);
    out[13] = ID(43u);
    out[14] = ID(47u);
    out[15] = SELECTED;
}
"###,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 24],
        )],
        0,
        [1, 1, 1],
    )
}
