use wgpu::{DownlevelFlags, Limits};
use wgpu_test::{apply, gpu_test, GpuTestConfiguration, GpuTestInitializer, TestParameters};

use super::{shader_input_output_test, InputStorageType, ShaderTest};

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

        shader_input_output_test(ctx, InputStorageType::Storage, vec![module, template]).await;
    });
