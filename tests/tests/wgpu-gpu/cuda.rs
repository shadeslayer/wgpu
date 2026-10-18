use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu_test::{
    apply, gpu_test, GpuTestConfiguration, GpuTestInitializer, TestParameters, TestingContext,
};

pub fn all_tests(tests: &mut Vec<GpuTestInitializer>) {
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
    ]);
}

fn buffer(ctx: &TestingContext, contents: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
    ctx.device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents,
        usage,
    })
}

async fn run_kernel(
    ctx: &TestingContext,
    source: &str,
    buffers: &[wgpu::Buffer],
    output_binding: usize,
    workgroups: u32,
) -> Vec<u8> {
    let module = naga::front::cuda::parse_str(
        source,
        &naga::front::cuda::Options {
            workgroup_size: [64, 1, 1],
        },
    )
    .unwrap();
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    let wgsl =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(wgsl.into()),
        });
    let pipeline = ctx
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &shader,
            entry_point: None,
            compilation_options: Default::default(),
            cache: None,
        });
    let entries: Vec<_> = buffers
        .iter()
        .enumerate()
        .map(|(binding, buffer)| wgpu::BindGroupEntry {
            binding: binding as u32,
            resource: buffer.as_entire_binding(),
        })
        .collect();
    let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &entries,
    });
    let output = &buffers[output_binding];
    let readback = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: output.size(),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(workgroups, 1, 1);
    }
    encoder.copy_buffer_to_buffer(output, 0, &readback, 0, output.size());
    ctx.queue.submit(Some(encoder.finish()));
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, |result| result.unwrap());
    ctx.async_poll(wgpu::PollType::wait_indefinitely())
        .await
        .unwrap();
    let bytes = readback.slice(..).get_mapped_range().unwrap().to_vec();
    readback.unmap();
    bytes
}

#[apply(gpu_test!)]
static CUDA_SCALAR_PARAMETER: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[0u32; 64]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[73u32, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            "__global__ void scalar(unsigned int *out, unsigned int value) { out[threadIdx.x] = value; }",
            &buffers,
            0,
            1,
        )
        .await;
        assert_eq!(bytemuck::cast_slice::<_, u32>(&result), &[73; 64]);
    });

#[apply(gpu_test!)]
static CUDA_GUARDED_VECTOR_ADD: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let left_input: [f32; 128] = std::array::from_fn(|index| index as f32 + 0.5);
        let right_input: [f32; 128] = std::array::from_fn(|index| (index % 13) as f32 - 6.0);
        let sentinel = -9876.5f32;
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&left_input),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&right_input),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/vector-add-guarded.cu"),
            &buffers,
            2,
            2,
        )
        .await;
        let actual: &[f32] = bytemuck::cast_slice(&result);
        for (index, &value) in actual.iter().enumerate() {
            let expected = if index < length as usize {
                left_input[index] + right_input[index] * 2.0
            } else {
                sentinel
            };
            assert_eq!(value, expected, "element {index}");
        }
    });

#[apply(gpu_test!)]
static CUDA_NUMERIC_CONVERSIONS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let buffers = [buffer(
            &ctx,
            bytemuck::cast_slice(&[99u32; 64 * 8]),
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        )];
        let result = run_kernel(
            &ctx,
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
            &buffers,
            0,
            1,
        )
        .await;
        let actual: &[u32] = bytemuck::cast_slice(&result);
        for (invocation, values) in actual.as_chunks::<8>().0.iter().enumerate() {
            assert_eq!(
                values,
                &[u32::MAX, 2, 0, 3, 5, 1, 8, 3],
                "thread {invocation}"
            );
        }
    });

#[apply(gpu_test!)]
static CUDA_LOOPS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let buffers = [buffer(
            &ctx,
            bytemuck::cast_slice(&[99i32; 64 * 10]),
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        )];
        let result = run_kernel(
            &ctx,
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
            &buffers,
            0,
            1,
        )
        .await;
        let actual: &[i32] = bytemuck::cast_slice(&result);
        for (invocation, values) in actual.as_chunks::<10>().0.iter().enumerate() {
            let tail = if invocation < 32 { 99 } else { 19 };
            assert_eq!(
                values,
                &[13, 30, 5, 300, 5, 10, 3, 5, 1, tail],
                "thread {invocation}"
            );
        }
    });

#[apply(gpu_test!)]
static CUDA_UPDATES: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let buffers = [buffer(
            &ctx,
            bytemuck::cast_slice(&[99i32; 64 * 16]),
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        )];
        let result = run_kernel(
            &ctx,
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
            &buffers,
            0,
            1,
        )
        .await;
        let actual: &[i32] = bytemuck::cast_slice(&result);
        for (invocation, values) in actual.as_chunks::<16>().0.iter().enumerate() {
            assert_eq!(
                values,
                &[1, 3, 2, 2, 1, 9, 4, 12, 12, 14, 99, 1, 2, 10, 21, 2],
                "thread {invocation}"
            );
        }
    });

#[apply(gpu_test!)]
static CUDA_INTEGER_OPERATORS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let buffers = [buffer(
            &ctx,
            bytemuck::cast_slice(&[99u32; 64 * 16]),
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        )];
        let result = run_kernel(
            &ctx,
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
            &buffers,
            0,
            1,
        )
        .await;
        let actual: &[u32] = bytemuck::cast_slice(&result);
        for (invocation, values) in actual.as_chunks::<16>().0.iter().enumerate() {
            assert_eq!(
                values,
                &[
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
                ],
                "thread {invocation}"
            );
        }
    });

#[apply(gpu_test!)]
static CUDA_SHARED_STORAGE: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let buffers = [buffer(
            &ctx,
            bytemuck::cast_slice(&[u32::MAX; 256]),
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        )];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/shared-storage.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[u32] = bytemuck::cast_slice(&result);
        for (group, values) in actual.as_chunks::<128>().0.iter().enumerate() {
            let expected: Vec<_> = (0..64)
                .map(|lane| group as u32 * 1000 + lane + 10)
                .collect();
            assert_eq!(&values[..64], expected, "workgroup {group}");
            assert_eq!(&values[64..66], &[3, 7], "workgroup {group}");
            assert!(values[66..].iter().all(|&value| value == u32::MAX));
        }
    });

#[apply(gpu_test!)]
static CUDA_BARRIERS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let buffers = [buffer(
            &ctx,
            bytemuck::cast_slice(&[u32::MAX; 512 + 16]),
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        )];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/barriers.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[u32] = bytemuck::cast_slice(&result);
        for group in 0..2 {
            let values = &actual[group * 256..(group + 1) * 256];
            for lane in 0..64 {
                assert_eq!(values[lane], group as u32 * 1000 + lane as u32);
                assert_eq!(
                    values[64 + lane],
                    group as u32 * 2000 + 63 - lane as u32 + 7,
                    "shared exchange in workgroup {group}, lane {lane}",
                );
                assert_eq!(
                    values[128 + lane],
                    group as u32 * 1000 + (lane as u32 + 1) % 64,
                    "storage exchange in workgroup {group}, lane {lane}",
                );
                assert_eq!(values[192 + lane], group as u32 * 100 + 11);
            }
        }
        assert!(actual[512..].iter().all(|&value| value == u32::MAX));
    });

#[apply(gpu_test!)]
static CUDA_DEVICE_FUNCTIONS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let sentinel = -9876i32;
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 10]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/device-functions.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[i32] = bytemuck::cast_slice(&result);
        let choose = |value: i32| if value >= 0 { value * 2 + 1 } else { -value };
        for (index, values) in actual.as_chunks::<10>().0.iter().enumerate() {
            if index >= length as usize {
                assert_eq!(values, &[sentinel; 10], "guarded thread {index}");
                continue;
            }
            let value = index as i32 - 35;
            assert_eq!(
                values,
                &[
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
                ],
                "thread {index}",
            );
        }
    });

#[apply(gpu_test!)]
static CUDA_VOID_FUNCTIONS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let sentinel = -9876i32;
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 6]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/void-functions.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[i32] = bytemuck::cast_slice(&result);
        for (index, values) in actual.as_chunks::<6>().0.iter().enumerate() {
            let expected = if index < length as usize {
                [3, 4, 5, 9, 3, 10]
            } else {
                [sentinel; 6]
            };
            assert_eq!(values, &expected, "thread {index}");
        }
    });

#[apply(gpu_test!)]
static CUDA_CONDITIONALS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let sentinel = -9876i32;
        let input_a: [i32; 128] = std::array::from_fn(|index| index as i32 + 100);
        let input_b: [i32; 128] = std::array::from_fn(|index| index as i32 * -3 + 17);
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 25]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&input_a),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&input_b),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/conditionals.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[i32] = bytemuck::cast_slice(&result);
        for (index, values) in actual.as_chunks::<25>().0.iter().enumerate() {
            let expected = if index < length as usize {
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
            };
            assert_eq!(values, &expected, "thread {index}");
        }
    });

#[apply(gpu_test!)]
static CUDA_SCALAR_ALIASES: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let sentinel = 0xdeadbeefu32;
        let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 9 - 4);
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 10]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&input),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/scalar-aliases.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[u32] = bytemuck::cast_slice(&result);
        for (index, values) in actual.as_chunks::<10>().0.iter().enumerate() {
            let expected = if index < length as usize {
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
            };
            assert_eq!(values, &expected, "thread {index}");
        }
    });

#[apply(gpu_test!)]
static CUDA_INTEGER_LITERALS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
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
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 18]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&input),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/integer-literals.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[u32] = bytemuck::cast_slice(&result);
        for (index, row) in actual.as_chunks::<18>().0.iter().enumerate() {
            let expected = if index < length as usize {
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
            };
            assert_eq!(row, &expected, "thread {index}");
        }
    });

#[apply(gpu_test!)]
static CUDA_INLINE_FUNCTIONS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let sentinel = -123456789i32;
        let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 12]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&input),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/inline-functions.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[i32] = bytemuck::cast_slice(&result);
        for (index, row) in actual.as_chunks::<12>().0.iter().enumerate() {
            let expected = if index < length as usize {
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
            };
            assert_eq!(row, &expected, "thread {index}");
        }
    });

#[apply(gpu_test!)]
static CUDA_FLOAT_SEPARATORS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let sentinel = 0xdead_beefu32;
        let input: [f32; 128] = std::array::from_fn(|index| (index as i32 % 17 - 8) as f32 * 0.25);
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 12]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&input),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/float-separators.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[u32] = bytemuck::cast_slice(&result);
        for (index, row) in actual.as_chunks::<12>().0.iter().enumerate() {
            let expected = if index < length as usize {
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
            };
            assert_eq!(row, &expected, "thread {index}");
        }
    });

#[apply(gpu_test!)]
static CUDA_DUAL_EXECUTION: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let sentinel = -123456789i32;
        let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 12]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&input),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/dual-execution.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[i32] = bytemuck::cast_slice(&result);
        for (index, row) in actual.as_chunks::<12>().0.iter().enumerate() {
            let expected = if index < length as usize {
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
            };
            assert_eq!(row, &expected, "thread {index}");
        }
    });

#[apply(gpu_test!)]
static CUDA_UNINITIALIZED: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let sentinel = -123456789i32;
        let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 12]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&input),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/uninitialized.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[i32] = bytemuck::cast_slice(&result);
        for (index, row) in actual.as_chunks::<12>().0.iter().enumerate() {
            let expected = if index < length as usize {
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
            };
            assert_eq!(row, &expected, "thread {index}");
        }
    });

#[apply(gpu_test!)]
static CUDA_LOCAL_DECLARATORS: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let sentinel = -123456789i32;
        let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 12]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&input),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/local-declarators.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[i32] = bytemuck::cast_slice(&result);
        for (index, row) in actual.as_chunks::<12>().0.iter().enumerate() {
            let expected = if index < length as usize {
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
            };
            assert_eq!(row, &expected, "thread {index}");
        }
    });

#[apply(gpu_test!)]
static CUDA_COMMA: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(TestParameters::default().test_features_limits())
    .run_async(|ctx| async move {
        let length = 70u32;
        let sentinel = -123456789i32;
        let input: [i32; 128] = std::array::from_fn(|index| index as i32 % 11 - 5);
        let buffers = [
            buffer(
                &ctx,
                bytemuck::cast_slice(&[sentinel; 128 * 12]),
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&input),
                wgpu::BufferUsages::STORAGE,
            ),
            buffer(
                &ctx,
                bytemuck::cast_slice(&[length, 0, 0, 0]),
                wgpu::BufferUsages::UNIFORM,
            ),
        ];
        let result = run_kernel(
            &ctx,
            include_str!("../../../naga/tests/in/cuda/comma.cu"),
            &buffers,
            0,
            2,
        )
        .await;
        let actual: &[i32] = bytemuck::cast_slice(&result);
        for (index, row) in actual.as_chunks::<12>().0.iter().enumerate() {
            let expected = if index < length as usize {
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
            };
            assert_eq!(row, &expected, "thread {index}");
        }
    });
