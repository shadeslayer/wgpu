use wgpu_test::{apply, gpu_test, GpuTestConfiguration, GpuTestInitializer, TestParameters};

use super::super::{shader_input_output_test, InputStorageType};
use super::{module_case, ShaderBuffer, ShaderTest};

pub(super) fn all_tests(tests: &mut Vec<GpuTestInitializer>) {
    tests.extend([
        CUDA_SCALAR_PARAMETER,
        CUDA_GUARDED_VECTOR_ADD,
        CUDA_NUMERIC_CONVERSIONS,
        CUDA_LOOPS,
        CUDA_UPDATES,
        CUDA_INTEGER_OPERATORS,
        CUDA_SHARED_STORAGE,
        CUDA_BARRIERS,
        CUDA_DEVICE_FUNCTIONS,
        CUDA_VOID_FUNCTIONS,
        CUDA_CONDITIONALS,
        CUDA_SCALAR_ALIASES,
        CUDA_INTEGER_LITERALS,
        CUDA_INLINE_FUNCTIONS,
        CUDA_FLOAT_SEPARATORS,
        CUDA_DUAL_EXECUTION,
        CUDA_UNINITIALIZED,
        CUDA_LOCAL_DECLARATORS,
        CUDA_COMMA,
        CUDA_FUNCTION_SPECIFIERS,
        CUDA_SCALAR_OVERLOADS,
    ]);
}

fn configuration(case: fn() -> ShaderTest) -> GpuTestConfiguration {
    GpuTestConfiguration::new()
        .parameters(TestParameters::default().test_features_limits())
        .run_async(move |ctx| {
            shader_input_output_test(ctx, InputStorageType::Storage, vec![case()])
        })
}

#[apply(gpu_test!)]
static CUDA_SCALAR_PARAMETER: GpuTestConfiguration = configuration(scalar_parameter);

fn scalar_parameter() -> ShaderTest {
    let expected = vec![73_u32; 64];
    let source = "__global__ void scalar(unsigned int *out, unsigned int value) { out[threadIdx.x] = value; }";
    module_case("scalar_parameter", source, [64, 1, 1], &expected).module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[0u32; 64],
            ),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Uniform, &[73u32, 0, 0, 0]),
        ],
        0,
        [1, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_GUARDED_VECTOR_ADD: GpuTestConfiguration = configuration(guarded_vector_add);

fn guarded_vector_add() -> ShaderTest {
    let length = 70u32;
    let left_input: [f32; 128] = std::array::from_fn(|index| index as f32 + 0.5);
    let right_input: [f32; 128] = std::array::from_fn(|index| (index % 13) as f32 - 6.0);
    let sentinel = -9876.5f32;

    let expected: Vec<f32> = (0..128)
        .map(|index| {
            if index < length as usize {
                left_input[index] + right_input[index] * 2.0
            } else {
                sentinel
            }
        })
        .collect();
    module_case(
        "guarded_vector_add",
        include_str!("../../../../../naga/tests/in/cuda/vector-add-guarded.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: true },
                &left_input,
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &right_input,
            ),
            ShaderBuffer::new(
                2,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128],
            ),
            ShaderBuffer::new(3, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        2,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_NUMERIC_CONVERSIONS: GpuTestConfiguration = configuration(numeric_conversions);

fn numeric_conversions() -> ShaderTest {
    let expected: Vec<u32> = [u32::MAX, 2, 0, 3, 5, 1, 8, 3].repeat(64);
    module_case(
        "numeric_conversions",
        "__global__ void conversions(unsigned int *out) {
                unsigned int base = threadIdx.x * 8u;
                out[base + 0u] = -1u;
                out[base + 1u] = true + true;
                out[base + 2u] = unsigned(-3 < 1u);
                out[base + 3u] = static_cast<unsigned int>((float)7 / 2);
                out[base + 4u] = unsigned((int)3.75f + 2);
                out[base + 5u] = bool(-0.5f);
                out[base + 6u] = (int)-2.75f + 10u;
                out[base + 7u] = float(3u) + 0.75f;
            }",
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[99u32; 64 * 8],
        )],
        0,
        [1, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_LOOPS: GpuTestConfiguration = configuration(loops);

fn loops() -> ShaderTest {
    let expected: Vec<i32> = (0..64)
        .flat_map(|invocation| {
            let tail = if invocation < 32 { 99 } else { 19 };
            [13, 30, 5, 300, 5, 10, 3, 5, 1, tail]
        })
        .collect();
    module_case(
        "loops",
        "__global__ void loops(int *out) {
                unsigned int base = threadIdx.x * 10u;
                int sum = 0;
                for (int i = 0; i < 8; i++) {
                    if (i == 2) continue;
                    if (i == 6) break;
                    sum += i;
                }
                out[base] = sum;
                sum = 0;
                int i = 0;
                while (i++ < 4) {
                    if (i == 2) continue;
                    sum += 10;
                }
                out[base + 1u] = sum;
                out[base + 2u] = i;
                sum = 0;
                do {
                    sum += 100;
                    if (sum < 200) continue;
                } while (sum < 250);
                out[base + 3u] = sum;
                do { sum += 100; break; } while (++i);
                out[base + 4u] = i;
                sum = 0;
                for (int row = 0; row < 3; ++row) {
                    for (int col = 0; col < 4; ++col) {
                        if (col == 1) continue;
                        if (row == 1) break;
                        sum += col;
                    }
                }
                out[base + 5u] = sum;
                sum = 0;
                for (;;) { ++sum; if (sum == 3) break; }
                out[base + 6u] = sum;
                for (int i = 0; i < 2; ++i) {}
                out[base + 7u] = i;
                sum = 0;
                while (false) { sum = 40; }
                for (;false;) { sum = 50; }
                do { ++sum; } while (false);
                out[base + 8u] = sum;
                for (int k = 0; k < 3; ++k) {
                    if (threadIdx.x < 32u) return;
                }
                out[base + 9u] = 19;
            }",
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[99i32; 64 * 10],
        )],
        0,
        [1, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_UPDATES: GpuTestConfiguration = configuration(updates);

fn updates() -> ShaderTest {
    let expected: Vec<i32> = [1, 3, 2, 2, 1, 9, 4, 12, 12, 14, 99, 1, 2, 10, 21, 2].repeat(64);
    module_case(
        "updates",
        "__global__ void updates(int *out) {
                unsigned int base = threadIdx.x * 16u;
                int i = 1;
                out[base] = i++;
                out[base + 1u] = ++i;
                out[base + 2u] = --i;
                out[base + 3u] = i--;
                out[base + 4u] = i;
                out[base + 5u] = (i += 2) *= 3;
                out[base + 6u] = i = 4;
                out[base + 7u] = i += (i = 6);
                false && (++i > 0);
                true || (++i > 0);
                out[base + 8u] = i;
                true && (++i > 0);
                false || (++i > 0);
                out[base + 9u] = i;
                int j = 0;
                out[base + 10u + j++] = ++j;
                out[base + 12u] = j;
                out[base + 13u] = 10;
                out[base + 14u] = 20;
                j = 0;
                out[base + 13u + j++] += ++j;
                out[base + 15u] = j;
            }",
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[99i32; 64 * 16],
        )],
        0,
        [1, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_INTEGER_OPERATORS: GpuTestConfiguration = configuration(integer_operators);

fn integer_operators() -> ShaderTest {
    let expected: Vec<u32> = [
        u32::MAX,
        1,
        u32::MAX,
        u32::MAX - 1,
        1 << 31,
        u32::MAX - 3,
        2_147_483_644,
        1,
        2,
        1,
        4,
        8,
        4,
        1,
        u32::MAX - 1,
        u32::MAX,
    ]
    .repeat(64);
    module_case(
        "integer_operators",
        "__global__ void integer_ops(unsigned int *out) {
                unsigned int base = threadIdx.x * 16u;
                out[base] = -7 % 3;
                out[base + 1u] = 7 % -3;
                out[base + 2u] = ~0u;
                out[base + 3u] = ~true;
                out[base + 4u] = 1 << 31;
                out[base + 5u] = -8 >> 1u;
                out[base + 6u] = -8u >> 1;
                out[base + 7u] = 1 | 2 ^ 3 & 6;
                out[base + 8u] = 8 >> 1 + 1;
                out[base + 9u] = 1 << 2 < 5;
                int i = 1;
                out[base + 10u] = i << (i = 2);
                i = 1;
                out[base + 11u] = i <<= (i = 2);
                int value = 37;
                value %= 5;
                value |= 8;
                value ^= 3;
                value &= 7;
                value <<= 4;
                value >>= 2;
                out[base + 12u] = value;
                out[base + 13u] = (true | false) + (true & false);
                out[base + 14u] = 4294967295u << 1;
                out[base + 15u] = -7 % -3;
            }",
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[99u32; 64 * 16],
        )],
        0,
        [1, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_SHARED_STORAGE: GpuTestConfiguration = configuration(shared_storage);

fn shared_storage() -> ShaderTest {
    let mut expected = vec![u32::MAX; 256];
    for group in 0..2 {
        for lane in 0..64 {
            expected[group * 128 + lane] = group as u32 * 1000 + lane as u32 + 10;
        }
        expected[group * 128 + 64..group * 128 + 66].copy_from_slice(&[3, 7]);
    }
    module_case(
        "shared_storage",
        include_str!("../../../../../naga/tests/in/cuda/shared-storage.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[u32::MAX; 256],
        )],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_BARRIERS: GpuTestConfiguration = configuration(barriers);

fn barriers() -> ShaderTest {
    let mut expected = vec![u32::MAX; 512 + 16];
    for group in 0..2 {
        let values = &mut expected[group * 256..(group + 1) * 256];
        for lane in 0..64 {
            values[lane] = group as u32 * 1000 + lane as u32;
            values[64 + lane] = group as u32 * 2000 + 63 - lane as u32 + 7;
            values[128 + lane] = group as u32 * 1000 + (lane as u32 + 1) % 64;
            values[192 + lane] = group as u32 * 100 + 11;
        }
    }
    module_case(
        "barriers",
        include_str!("../../../../../naga/tests/in/cuda/barriers.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[u32::MAX; 512 + 16],
        )],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_DEVICE_FUNCTIONS: GpuTestConfiguration = configuration(device_functions);

fn device_functions() -> ShaderTest {
    let length = 70u32;
    let sentinel = -9876i32;

    let choose = |value: i32| if value >= 0 { value * 2 + 1 } else { -value };
    let expected: Vec<i32> = (0..128)
        .flat_map(|index| {
            if index >= length as usize {
                return [sentinel; 10];
            }
            let value = index as i32 - 35;
            [
                choose(value),
                (value as f32 * 1.5 + 2.0) as i32,
                10,
                4,
                value * 2 + 1,
                value,
                choose(value) + choose(value + 1),
                i32::from(value >= 0),
                // The frontend chooses left-to-right argument capture.
                33,
                4,
            ]
        })
        .collect();
    module_case(
        "device_functions",
        include_str!("../../../../../naga/tests/in/cuda/device-functions.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 10],
            ),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_VOID_FUNCTIONS: GpuTestConfiguration = configuration(void_functions);

fn void_functions() -> ShaderTest {
    let length = 70u32;
    let sentinel = -9876i32;

    let expected: Vec<i32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                [3, 4, 5, 9, 3, 10]
            } else {
                [sentinel; 6]
            }
        })
        .collect();
    module_case(
        "void_functions",
        include_str!("../../../../../naga/tests/in/cuda/void-functions.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 6],
            ),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_CONDITIONALS: GpuTestConfiguration = configuration(conditionals);

fn conditionals() -> ShaderTest {
    let length = 70u32;
    let sentinel = -9876i32;
    let input_a: [i32; 128] = std::array::from_fn(|index| index as i32 + 100);
    let input_b: [i32; 128] = std::array::from_fn(|index| index as i32 * -3 + 17);

    let expected: Vec<i32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                let choose_a = index % 2 == 0;
                [
                    if choose_a {
                        input_a[index]
                    } else {
                        input_b[index]
                    },
                    if choose_a { 13 } else { 15 },
                    if choose_a { 45 } else { 36 },
                    if choose_a { 55 } else { 37 },
                    if choose_a { 10 } else { 1 },
                    if choose_a { 75 } else { 37 },
                    if choose_a { 95 } else { 39 },
                    10,
                    10,
                    if choose_a { 1105 } else { 311 },
                    0,
                    if choose_a { 17 } else { 1 },
                    14,
                    if index % 4 == 0 {
                        546
                    } else if choose_a {
                        276
                    } else {
                        249
                    },
                    if choose_a { 1520 } else { 1025 },
                    if choose_a { sentinel } else { 30 },
                    if choose_a { 30 } else { 25 },
                    if choose_a { 1 } else { 3 },
                    if choose_a { -1 } else { 2 },
                    i32::from(choose_a),
                    if choose_a { 1 } else { 2 },
                    if choose_a { 4 } else { 8 },
                    if choose_a { 4 } else { 6 },
                    // The frontend chooses left-to-right argument capture.
                    if choose_a { 33 } else { 73 },
                    if choose_a { 303 } else { 33 },
                ]
            } else {
                [sentinel; 25]
            }
        })
        .collect();
    module_case(
        "conditionals",
        include_str!("../../../../../naga/tests/in/cuda/conditionals.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 25],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input_a,
            ),
            ShaderBuffer::new(
                2,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input_b,
            ),
            ShaderBuffer::new(3, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_SCALAR_ALIASES: GpuTestConfiguration = configuration(scalar_aliases);

fn scalar_aliases() -> ShaderTest {
    let length = 70u32;
    let sentinel = 0xdeadbeefu32;
    let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 9 - 4);

    let expected: Vec<u32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                [
                    index as u32 + 10,
                    (input[index] * 2 + 1) as u32,
                    (-3i32) as u32,
                    3,
                    (-5i32) as u32,
                    5,
                    u32::from(index % 2 == 0),
                    3,
                    3,
                    length,
                ]
            } else {
                [sentinel; 10]
            }
        })
        .collect();
    module_case(
        "scalar_aliases",
        include_str!("../../../../../naga/tests/in/cuda/scalar-aliases.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 10],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_INTEGER_LITERALS: GpuTestConfiguration = configuration(integer_literals);

fn integer_literals() -> ShaderTest {
    let length = 70u32;
    let sentinel = 0x1357_9bdfu32;
    let values = [
        0,
        1,
        2,
        3,
        i32::MAX as u32,
        0x8000_0000,
        0x8000_0001,
        u32::MAX,
    ];
    let input: [u32; 128] = std::array::from_fn(|index| values[index % values.len()]);

    let expected: Vec<u32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                let value = input[index];
                let shift = value & 3;
                [
                    value ^ 0xff,
                    value & 0xff,
                    value & 0xf0,
                    value.wrapping_add(1000),
                    0x8000_0000u32 >> shift,
                    (i32::MIN >> shift) as u32,
                    u32::MAX ^ value,
                    u32::from(value < 0x8000_0000),
                    u32::from((value as i32) < i32::MAX),
                    0x8000_0000u32 >> shift,
                    value.wrapping_add(255),
                    value.wrapping_add(28),
                    value.wrapping_add(15),
                    u32::MAX.wrapping_sub(value),
                    0xdead_beef ^ value,
                    u32::from(i32::MIN < value as i32),
                    value.wrapping_add(32),
                    value.wrapping_add(28),
                ]
            } else {
                [sentinel; 18]
            }
        })
        .collect();
    module_case(
        "integer_literals",
        include_str!("../../../../../naga/tests/in/cuda/integer-literals.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 18],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_INLINE_FUNCTIONS: GpuTestConfiguration = configuration(inline_functions);

fn inline_functions() -> ShaderTest {
    let length = 70u32;
    let sentinel = -123456789i32;
    let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);

    let expected: Vec<i32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                let value = input[index];
                [
                    value + 2,
                    value,
                    (value + 2) * 2,
                    (value as u32).wrapping_add(3) as i32,
                    (value as u32).wrapping_sub(1) as i32,
                    (value as f32 * 1.5 + 2.0) as i32,
                    1,
                    value + 1,
                    value + 2,
                    0,
                    value + 6,
                    value + 2,
                ]
            } else {
                [sentinel; 12]
            }
        })
        .collect();
    module_case(
        "inline_functions",
        include_str!("../../../../../naga/tests/in/cuda/inline-functions.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 12],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_FLOAT_SEPARATORS: GpuTestConfiguration = configuration(float_separators);

fn float_separators() -> ShaderTest {
    let length = 70u32;
    let sentinel = 0xdead_beefu32;
    let input: [f32; 128] = std::array::from_fn(|index| (index as i32 % 17 - 8) as f32 * 0.25);

    let expected: Vec<u32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                [
                    0x4480_1000,
                    0x3e00_0000,
                    0x4140_0000,
                    0x41c8_0000,
                    0x3f00_0000,
                    0x40c8_0000,
                    0xc480_1000,
                    0x3e00_0000,
                    0x4049_0fdb,
                    0x3f80_0000,
                    0x3f80_0002,
                    (input[index] * 0.25 + 10.0).to_bits(),
                ]
            } else {
                [sentinel; 12]
            }
        })
        .collect();
    module_case(
        "float_separators",
        include_str!("../../../../../naga/tests/in/cuda/float-separators.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 12],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_DUAL_EXECUTION: GpuTestConfiguration = configuration(dual_execution);

fn dual_execution() -> ShaderTest {
    let length = 70u32;
    let sentinel = -123456789i32;
    let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);

    let expected: Vec<i32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                let value = input[index];
                [
                    value + 3,
                    value,
                    (value + 3) * 2,
                    (value as u32).wrapping_add(5) as i32,
                    (value as u32).wrapping_sub(2) as i32,
                    value * 2 + 4,
                    1,
                    value + 1,
                    value + 2,
                    0,
                    value + 9,
                    value + 2,
                ]
            } else {
                [sentinel; 12]
            }
        })
        .collect();
    module_case(
        "dual_execution",
        include_str!("../../../../../naga/tests/in/cuda/dual-execution.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 12],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_UNINITIALIZED: GpuTestConfiguration = configuration(uninitialized);

fn uninitialized() -> ShaderTest {
    let length = 70u32;
    let sentinel = -123456789i32;
    let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);

    let expected: Vec<i32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                let value = input[index];
                [
                    if index % 2 == 0 { value + 7 } else { value - 4 },
                    if index % 3 == 0 { value + 3 } else { value - 2 },
                    (value as f32 * 1.5 + 2.0) as i32,
                    i32::from(value < 0),
                    (value + 5) * 3,
                    [0, 1, 3, 6, 12][index % 5],
                    value * 3 + 3,
                    value * 3 - 3,
                    value + 3,
                    value + 10,
                    value,
                    value * 2 + 4,
                ]
            } else {
                [sentinel; 12]
            }
        })
        .collect();
    module_case(
        "uninitialized",
        include_str!("../../../../../naga/tests/in/cuda/uninitialized.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 12],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_LOCAL_DECLARATORS: GpuTestConfiguration = configuration(local_declarators);

fn local_declarators() -> ShaderTest {
    let length = 70u32;
    let sentinel = -123456789i32;
    let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);

    let expected: Vec<i32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                let value = input[index];
                [
                    value * 111 + 12,
                    value + 2,
                    value * 2 + 12,
                    value + 5,
                    value + 7,
                    i32::from(value + 2 > 0),
                    value + 5,
                    index as i32 % 5 + 3,
                    value * 2 + 6,
                    value + 9,
                    value * 3 + 11,
                    value * 2 + 9,
                ]
            } else {
                [sentinel; 12]
            }
        })
        .collect();
    module_case(
        "local_declarators",
        include_str!("../../../../../naga/tests/in/cuda/local-declarators.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 12],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_COMMA: GpuTestConfiguration = configuration(comma);

fn comma() -> ShaderTest {
    let length = 70u32;
    let sentinel = -123456789i32;
    let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);

    let expected: Vec<i32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                let value = input[index];
                let choose = index % 2 == 0;
                [
                    value * 3 + 5,
                    value * 3 + 6,
                    value + 4,
                    value * 11 + 41,
                    if choose {
                        value * 11 + 52
                    } else {
                        value * 10 - 20
                    },
                    if choose {
                        value * 101 + 310
                    } else {
                        value * 101 + 13
                    },
                    value + 2,
                    value * 2 + 2,
                    value + 4,
                    value * 3 + 10,
                    value * 10 + 74,
                    value * 11 + 61,
                ]
            } else {
                [sentinel; 12]
            }
        })
        .collect();
    module_case(
        "comma",
        include_str!("../../../../../naga/tests/in/cuda/comma.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 12],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_FUNCTION_SPECIFIERS: GpuTestConfiguration = configuration(function_specifiers);

fn function_specifiers() -> ShaderTest {
    let length = 70u32;
    let sentinel = -123456789i32;
    let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);

    let expected: Vec<i32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                let value = input[index];
                [
                    value + 2,
                    value * 2 + 4,
                    value * 2 + 5,
                    value - 3,
                    (value as f32 * 1.5 + 0.5) as i32,
                    0,
                    value + 1,
                    value + 2,
                    value * 2 + 9,
                    value + 19,
                    1,
                    value + 2,
                ]
            } else {
                [sentinel; 12]
            }
        })
        .collect();
    module_case(
        "function_specifiers",
        include_str!("../../../../../naga/tests/in/cuda/function-specifiers.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 12],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}

#[apply(gpu_test!)]
static CUDA_SCALAR_OVERLOADS: GpuTestConfiguration = configuration(scalar_overloads);

fn scalar_overloads() -> ShaderTest {
    let length = 70u32;
    let sentinel = -123456789i32;
    let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);

    let expected: Vec<i32> = (0..128)
        .flat_map(|index| {
            if index < length as usize {
                let value = input[index];
                let choose = i32::from(index % 2 == 0);
                [
                    1 - choose,
                    value + 10,
                    value + 36,
                    (value as f32 * 1.5 + 0.5) as i32,
                    choose * 10 + (value as f32 + 0.5) as i32,
                    100 + (value as f32 + 0.5) as i32 * 10 + choose,
                    value * 2 + 216,
                    value + 1,
                    value + 14,
                    (value as f32 + 0.75) as i32 + 2,
                    (value as f32 + 0.75) as i32 + 100,
                    value * 11 + 242,
                    value * 1101 + 8809,
                ]
            } else {
                [sentinel; 13]
            }
        })
        .collect();
    module_case(
        "scalar_overloads",
        include_str!("../../../../../naga/tests/in/cuda/scalar-overloads.cu"),
        [64, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![
            ShaderBuffer::new(
                0,
                wgpu::BufferBindingType::Storage { read_only: false },
                &[sentinel; 128 * 13],
            ),
            ShaderBuffer::new(
                1,
                wgpu::BufferBindingType::Storage { read_only: true },
                &input,
            ),
            ShaderBuffer::new(2, wgpu::BufferBindingType::Uniform, &[length, 0, 0, 0]),
        ],
        0,
        [2, 1, 1],
    )
}
