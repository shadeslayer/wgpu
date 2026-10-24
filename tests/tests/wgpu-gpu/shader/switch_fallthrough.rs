use wgpu::{BufferBindingType, DownlevelFlags, Limits};
use wgpu_test::{apply, gpu_test, GpuTestConfiguration, GpuTestInitializer, TestParameters};

use super::{shader_input_output_test, InputStorageType, ShaderBuffer, ShaderTest};

pub fn all_tests(tests: &mut Vec<GpuTestInitializer>) {
    tests.push(SWITCH_FALLTHROUGH);
}

fn cases() -> Vec<ShaderTest> {
    let source = include_str!("../../../../naga/tests/in/glsl/switch-fallthrough.comp");
    let module = naga::front::glsl::Frontend::default()
        .parse(
            &naga::front::glsl::Options::from(naga::ShaderStage::Compute),
            source,
        )
        .unwrap();
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    let output = naga::back::wgsl::write_string(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    )
    .unwrap();
    let reparsed = naga::front::wgsl::parse_str(&output).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&reparsed)
    .unwrap();
    assert_eq!(reparsed.entry_points.len(), 1);
    assert_eq!(reparsed.entry_points[0].workgroup_size, [1, 1, 1]);
    let expected = [
        111_i32, 110, 100, 10000, 11000, 8, 8, 5, 7, 131, 121, 100, 900, 3028, 4030, 400, 3, 6, 5,
        19, 10, 1, 2, 1, 134, 82, 16, 6, 23, 7, -1, -1,
    ];
    vec![ShaderTest::new(
        "fallthrough, middle default, nested switches, loop jumps, returns and selector evaluation"
            .into(),
        String::new(),
        String::new(),
        &[0_u32],
        &expected,
    )
    .module_source(output, reparsed.entry_points[0].name.clone())
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            BufferBindingType::Storage { read_only: false },
            &[-1_i32; 32],
        )],
        0,
        [1, 1, 1],
    )]
}

#[apply(gpu_test!)]
static SWITCH_FALLTHROUGH: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(
        TestParameters::default()
            .downlevel_flags(DownlevelFlags::COMPUTE_SHADERS)
            .limits(Limits::downlevel_defaults()),
    )
    .run_async(|ctx| shader_input_output_test(ctx, InputStorageType::Storage, cases()));
