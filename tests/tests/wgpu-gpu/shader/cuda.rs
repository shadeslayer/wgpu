use naga::front::cuda::{parse_str, Options};
use naga::valid::{Capabilities, ValidationFlags, Validator};
use wgpu::{DownlevelFlags, Limits};
use wgpu_test::{apply, gpu_test, GpuTestConfiguration, GpuTestInitializer, TestParameters};

use super::{shader_input_output_test, InputStorageType, ShaderTest, MAX_BUFFER_SIZE};

mod arithmetic;
mod comma;
mod conditionals;
mod declarations;
mod dual_execution;
mod float_math;
mod float_separators;
mod inline;
mod integer_math;
mod literals;
mod local_declarators;
mod reference_categories;
mod specifiers;
mod type_aliases;
mod uninitialized;
mod updates;

pub fn all_tests(tests: &mut Vec<GpuTestInitializer>) {
    tests.extend([
        CUDA_ARITHMETIC,
        CUDA_COMMA,
        CUDA_CONDITIONALS,
        CUDA_DECLARATIONS,
        CUDA_DUAL_EXECUTION,
        CUDA_FLOAT_MATH,
        CUDA_FLOAT_SEPARATORS,
        CUDA_INLINE,
        CUDA_INTEGER_MATH,
        CUDA_LITERALS,
        CUDA_LOCAL_DECLARATORS,
        CUDA_REFERENCE_CATEGORIES,
        CUDA_SPECIFIERS,
        CUDA_TYPE_ALIASES,
        CUDA_UNINITIALIZED,
        CUDA_UPDATES,
    ]);
}

fn configuration(cases: fn() -> Vec<ShaderTest>) -> GpuTestConfiguration {
    GpuTestConfiguration::new()
        .parameters(
            TestParameters::default()
                .downlevel_flags(DownlevelFlags::COMPUTE_SHADERS)
                .limits(Limits::downlevel_defaults()),
        )
        .run_async(move |ctx| shader_input_output_test(ctx, InputStorageType::Storage, cases()))
}

fn case(name: &str, source: &str, initial: &[i32], expected: &[i32]) -> ShaderTest {
    scalar_case(name, source, initial, expected, naga::Scalar::I32)
}

fn float_case(name: &str, source: &str, initial: &[u32], expected: &[u32]) -> ShaderTest {
    let initial: Vec<i32> = initial.iter().map(|&bits| bits as i32).collect();
    let expected: Vec<i32> = expected.iter().map(|&bits| bits as i32).collect();
    scalar_case(name, source, &initial, &expected, naga::Scalar::F32)
}

fn scalar_case(
    name: &str,
    source: &str,
    initial: &[i32],
    expected: &[i32],
    scalar: naga::Scalar,
) -> ShaderTest {
    assert_eq!(initial.len(), expected.len(), "{name}");
    let mut full_expected = vec![-1_i32; MAX_BUFFER_SIZE as usize / 4];
    assert!(expected.len() <= full_expected.len(), "{name}");
    full_expected[..expected.len()].copy_from_slice(expected);

    let mut module = parse_str(
        source,
        &Options {
            workgroup_size: [1, 1, 1],
        },
    )
    .unwrap_or_else(|error| panic!("{name}: {error}"));
    Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&module)
        .unwrap_or_else(|error| panic!("{name}: {error}"));
    assert_eq!(module.entry_points.len(), 1, "{name}");
    assert_eq!(module.entry_points[0].workgroup_size, [1, 1, 1], "{name}");
    assert!(
        module.entry_points[0].function.arguments.is_empty(),
        "{name}"
    );
    assert_eq!(module.global_variables.len(), 1, "{name}");
    let (_, output) = module.global_variables.iter_mut().next().unwrap();
    assert_eq!(
        output.binding,
        Some(naga::ResourceBinding {
            group: 0,
            binding: 0,
        }),
        "{name}"
    );
    assert_eq!(
        output.space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE,
        },
        "{name}"
    );
    let naga::TypeInner::Array {
        base,
        size: naga::ArraySize::Dynamic,
        stride: 4,
    } = module.types[output.ty].inner
    else {
        panic!("{name}: expected a scalar output array");
    };
    assert_eq!(
        module.types[base].inner,
        naga::TypeInner::Scalar(scalar),
        "{name}"
    );
    output.binding.as_mut().unwrap().binding = 1;
    let info = Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&module)
        .unwrap_or_else(|error| panic!("{name}: {error}"));
    let wgsl = naga::back::wgsl::write_string(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    )
    .unwrap_or_else(|error| panic!("{name}: {error}"));
    let roundtrip =
        naga::front::wgsl::parse_str(&wgsl).unwrap_or_else(|error| panic!("{name}: {error}"));
    Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&roundtrip)
        .unwrap_or_else(|error| panic!("{name}: {error}"));

    ShaderTest::new(
        name.into(),
        String::new(),
        String::new(),
        &[0_u32],
        &full_expected,
    )
    .module_source(wgsl, roundtrip.entry_points[0].name.clone())
    .output_initial_values(initial)
}

#[apply(gpu_test!)]
static CUDA_ARITHMETIC: GpuTestConfiguration = configuration(arithmetic::cases);
#[apply(gpu_test!)]
static CUDA_COMMA: GpuTestConfiguration = configuration(comma::cases);
#[apply(gpu_test!)]
static CUDA_CONDITIONALS: GpuTestConfiguration = configuration(conditionals::cases);
#[apply(gpu_test!)]
static CUDA_DECLARATIONS: GpuTestConfiguration = configuration(declarations::cases);
#[apply(gpu_test!)]
static CUDA_DUAL_EXECUTION: GpuTestConfiguration = configuration(dual_execution::cases);
#[apply(gpu_test!)]
static CUDA_FLOAT_MATH: GpuTestConfiguration = configuration(float_math::cases);
#[apply(gpu_test!)]
static CUDA_FLOAT_SEPARATORS: GpuTestConfiguration = configuration(float_separators::cases);
#[apply(gpu_test!)]
static CUDA_INLINE: GpuTestConfiguration = configuration(inline::cases);
#[apply(gpu_test!)]
static CUDA_INTEGER_MATH: GpuTestConfiguration = configuration(integer_math::cases);
#[apply(gpu_test!)]
static CUDA_LITERALS: GpuTestConfiguration = configuration(literals::cases);
#[apply(gpu_test!)]
static CUDA_LOCAL_DECLARATORS: GpuTestConfiguration = configuration(local_declarators::cases);
#[apply(gpu_test!)]
static CUDA_REFERENCE_CATEGORIES: GpuTestConfiguration = configuration(reference_categories::cases);
#[apply(gpu_test!)]
static CUDA_SPECIFIERS: GpuTestConfiguration = configuration(specifiers::cases);
#[apply(gpu_test!)]
static CUDA_TYPE_ALIASES: GpuTestConfiguration = configuration(type_aliases::cases);
#[apply(gpu_test!)]
static CUDA_UNINITIALIZED: GpuTestConfiguration = configuration(uninitialized::cases);
#[apply(gpu_test!)]
static CUDA_UPDATES: GpuTestConfiguration = configuration(updates::cases);
