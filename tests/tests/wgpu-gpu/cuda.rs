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
        for (invocation, values) in actual.chunks_exact(8).enumerate() {
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
        for (invocation, values) in actual.chunks_exact(10).enumerate() {
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
        for (invocation, values) in actual.chunks_exact(16).enumerate() {
            assert_eq!(
                values,
                &[1, 3, 2, 2, 1, 9, 4, 12, 12, 14, 99, 1, 2, 10, 21, 2],
                "thread {invocation}"
            );
        }
    });
