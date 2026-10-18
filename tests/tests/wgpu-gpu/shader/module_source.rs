use wgpu::{BufferBindingType, DownlevelFlags, Limits};
use wgpu_test::{apply, gpu_test, GpuTestConfiguration, GpuTestInitializer, TestParameters};

use super::{shader_input_output_test, InputStorageType, ShaderBuffer, ShaderTest};

pub fn all_tests(tests: &mut Vec<GpuTestInitializer>) {
    tests.push(SHADER_MODULE_SOURCE);
}

const SOURCE: &str = r#"
@group(0) @binding(0) var<storage, read> input: array<i32>;
@group(0) @binding(1) var<storage, read_write> output: array<i32>;

@compute @workgroup_size(1)
fn apply_values() {
    output[0] += input[0];
    output[1] *= 3;
    output[2] += output[0];
}
"#;

const EXPLICIT_SOURCE: &str = r#"
struct Parameters {
    factor: u32,
    count: u32,
    addend: u32,
    input_base: u32,
}

@group(0) @binding(7) var<uniform> parameters: Parameters;
@group(0) @binding(2) var<storage, read> input: array<u32>;
@group(0) @binding(4) var<storage, read_write> output: array<u32>;
@group(0) @binding(0) var<storage, read> bias: array<u32>;
var<workgroup> lanes: array<u32, 8>;

@compute @workgroup_size(8)
fn explicit_values(
    @builtin(local_invocation_index) lane: u32,
    @builtin(workgroup_id) group: vec3<u32>,
) {
    let index = group.x * 8u + lane;
    lanes[lane] = input[parameters.input_base + index] + parameters.addend;
    workgroupBarrier();
    if index < parameters.count {
        output[index] += lanes[(lane + 1u) % 8u] * parameters.factor + bias[index];
    }
}
"#;

fn explicit_buffer_test() -> ShaderTest {
    let mut input = [0xdead_beef_u32; 48];
    input[32..].copy_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let mut initial = [0xdead_beef_u32; 48];
    initial[..14].fill(10);
    let mut expected = [0xdead_beef_u32; 48];
    expected[..14].copy_from_slice(&[
        118, 121, 124, 127, 130, 133, 136, 123, 142, 145, 148, 151, 154, 157,
    ]);

    ShaderTest::new(
        "explicit buffers, dispatch and complete output readback".into(),
        String::new(),
        String::new(),
        &[0_u32],
        &expected,
    )
    .module_source(EXPLICIT_SOURCE.into(), "explicit_values".into())
    .module_buffers(
        vec![
            ShaderBuffer::new(7, BufferBindingType::Uniform, &[2_u32, 14, 3, 32]),
            ShaderBuffer::new(4, BufferBindingType::Storage { read_only: false }, &initial),
            ShaderBuffer::new(
                0,
                BufferBindingType::Storage { read_only: true },
                &[
                    100_u32, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114,
                    115,
                ],
            ),
            ShaderBuffer::new(2, BufferBindingType::Storage { read_only: true }, &input),
        ],
        4,
        [2, 1, 1],
    )
}

fn check_explicit_buffer_configuration() {
    let output = || ShaderBuffer::new(4, BufferBindingType::Storage { read_only: false }, &[0_u32]);
    let test = |buffers, output_binding, workgroups| {
        ShaderTest::new(
            "explicit buffer bounds".into(),
            String::new(),
            String::new(),
            &[0_u32],
            &[0_u32],
        )
        .module_source(EXPLICIT_SOURCE.into(), "explicit_values".into())
        .module_buffers(buffers, output_binding, workgroups)
    };

    assert!(explicit_buffer_test().buffer_sizes_valid());
    assert!(!test(vec![output(), output()], 4, [1; 3]).buffer_sizes_valid());
    assert!(!test(vec![output()], 9, [1; 3]).buffer_sizes_valid());
    for ty in [
        BufferBindingType::Uniform,
        BufferBindingType::Storage { read_only: true },
    ] {
        assert!(
            !test(vec![ShaderBuffer::new(4, ty, &[0_u32; 4])], 4, [1; 3],).buffer_sizes_valid()
        );
    }
    assert!(!test(
        vec![ShaderBuffer::new(
            4,
            BufferBindingType::Storage { read_only: false },
            &[0_u32; 0],
        )],
        4,
        [1; 3],
    )
    .buffer_sizes_valid());
    for initial in [&[][..], &[0_u8, 1, 2][..]] {
        assert!(!test(
            vec![
                output(),
                ShaderBuffer::new(0, BufferBindingType::Storage { read_only: true }, initial),
            ],
            4,
            [1; 3],
        )
        .buffer_sizes_valid());
    }
    for workgroups in [[0, 1, 1], [1, 0, 1], [1, 1, 0]] {
        assert!(!test(vec![output()], 4, workgroups).buffer_sizes_valid());
    }
    assert!(!test(vec![output()], 4, [1; 3])
        .extra_output_values(&[0_u32; 2])
        .buffer_sizes_valid());
    assert!(!ShaderTest::new(
        "explicit output must fit its expected values".into(),
        String::new(),
        String::new(),
        &[0_u32],
        &[0_u32; 2],
    )
    .module_source(EXPLICIT_SOURCE.into(), "explicit_values".into())
    .module_buffers(vec![output()], 4, [1; 3])
    .buffer_sizes_valid());
    assert!(!test(vec![output()], 4, [1; 3])
        .output_initial_values(&[0_u32])
        .buffer_sizes_valid());
    assert!(!ShaderTest::new(
        "explicit buffers require a module source".into(),
        String::new(),
        String::new(),
        &[0_u32],
        &[0_u32],
    )
    .module_buffers(vec![output()], 4, [1; 3])
    .buffer_sizes_valid());
}

fn check_buffer_bounds() {
    let test = || {
        ShaderTest::new(
            "buffer bounds".into(),
            String::new(),
            String::new(),
            &[0_u32],
            &[0_u32],
        )
    };
    assert!(!ShaderTest::new(
        "oversized input".into(),
        String::new(),
        String::new(),
        &[0_u32; 33],
        &[0_u32],
    )
    .buffer_sizes_valid());
    assert!(!ShaderTest::new(
        "oversized expected output".into(),
        String::new(),
        String::new(),
        &[0_u32],
        &[0_u32; 33],
    )
    .buffer_sizes_valid());
    assert!(!test()
        .extra_output_values(&[0_u32; 33])
        .buffer_sizes_valid());
    assert!(!test()
        .output_initial_values(&[0_u32; 33])
        .buffer_sizes_valid());
    assert!(ShaderTest::new(
        "full buffer".into(),
        String::new(),
        String::new(),
        &[0_u32; 32],
        &[0_u32; 32],
    )
    .output_initial_values(&[0_u32; 32])
    .buffer_sizes_valid());
    let mut alternatives = test();
    for _ in 0..32 {
        alternatives = alternatives.extra_output_values(&[0_u32]);
    }
    assert!(alternatives.buffer_sizes_valid());
}

#[apply(gpu_test!)]
static SHADER_MODULE_SOURCE: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(
        TestParameters::default()
            .downlevel_flags(DownlevelFlags::COMPUTE_SHADERS)
            .limits(Limits::downlevel_defaults()),
    )
    .run_async(|ctx| async move {
        check_buffer_bounds();
        check_explicit_buffer_configuration();

        let mut expected = [-1_i32; 32];
        expected[..4].copy_from_slice(&[-2, 33, 0, -19]);
        let module = ShaderTest::new(
            "complete module with typed initial values".into(),
            String::new(),
            String::new(),
            &[5_i32],
            &expected,
        )
        .module_source(SOURCE.into(), "apply_values".into())
        .output_initial_values(&[-7_i32, 11, 2, -19]);

        let mut expected = [-1_i32; 32];
        expected[0] = 7;
        let template = ShaderTest::new(
            "default template and output fill".into(),
            "value: i32".into(),
            "output[0] = input.value;".into(),
            &[7_i32],
            &expected,
        )
        .output_type("array<i32>".into());

        shader_input_output_test(
            ctx,
            InputStorageType::Storage,
            vec![module, template, explicit_buffer_test()],
        )
        .await;
    });
