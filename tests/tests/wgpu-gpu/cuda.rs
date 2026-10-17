use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu_test::{
    apply, gpu_test, GpuTestConfiguration, GpuTestInitializer, TestParameters, TestingContext,
};

pub fn all_tests(tests: &mut Vec<GpuTestInitializer>) {
    tests.extend([
        CUDA_SCALAR_PARAMETER,
        CUDA_GUARDED_VECTOR_ADD,
        CUDA_NUMERIC_CONVERSIONS,
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
