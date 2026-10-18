#![cfg(feature = "cuda-in")]

mod adversarial;
mod barriers;
mod comma;
mod conditionals;
mod control_flow;
mod conversions;
mod declarations;
mod device_functions;
mod dual_execution;
mod float_math;
mod float_separators;
mod helpers;
mod inline;
mod integer_math;
mod integer_operators;
mod literals;
mod local_arrays;
mod local_declarators;
mod loops;
mod overloads;
mod parameters;
mod reference_categories;
mod shared;
mod specifiers;
mod streaming;
mod type_aliases;
mod uninitialized;
mod updates;
mod void_functions;

use naga::front::cuda::{parse_str, Options};

const VECTOR_ADD: &str = include_str!("../../in/cuda/vector-add.cu");

fn validate(source: &str, options: &Options) -> (naga::Module, naga::valid::ModuleInfo) {
    let module = parse_str(source, options).unwrap();
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    (module, info)
}

#[test]
fn cuda_vector_add_has_compute_entry_and_storage_bindings() {
    let options = Options {
        workgroup_size: [64, 1, 1],
    };
    let (module, _) = validate(VECTOR_ADD, &options);
    assert_eq!(module.entry_points.len(), 1);
    let entry = &module.entry_points[0];
    assert_eq!(entry.name, "add");
    assert_eq!(entry.stage, naga::ShaderStage::Compute);
    assert_eq!(entry.workgroup_size, options.workgroup_size);
    assert_eq!(module.global_variables.len(), 3);
    for (index, (_, global)) in module.global_variables.iter().enumerate() {
        assert_eq!(
            global.binding,
            Some(naga::ResourceBinding {
                group: 0,
                binding: index as u32
            })
        );
        let access = if index == 2 {
            naga::StorageAccess::LOAD | naga::StorageAccess::STORE
        } else {
            naga::StorageAccess::LOAD
        };
        assert_eq!(global.space, naga::AddressSpace::Storage { access });
    }
}

#[test]
fn cuda_grid_builtins_map_to_compute_inputs() {
    let (module, _) = validate(
        "__global__ void ids(unsigned int *out) { out[threadIdx.z] = blockIdx.y + gridDim.z + blockDim.y; }",
        &Options { workgroup_size: [4, 8, 2] },
    );
    let arguments = &module.entry_points[0].function.arguments;
    for builtin in [
        naga::BuiltIn::LocalInvocationId,
        naga::BuiltIn::WorkGroupId,
        naga::BuiltIn::NumWorkGroups,
    ] {
        assert!(arguments
            .iter()
            .any(|argument| argument.binding == Some(naga::Binding::BuiltIn(builtin))));
    }
    assert!(!arguments
        .iter()
        .any(|argument| argument.binding
            == Some(naga::Binding::BuiltIn(naga::BuiltIn::WorkGroupSize))));
}

#[test]
fn cuda_empty_kernel_comments_and_return() {
    validate(
        "/* kernel */ __global__ void empty(void) { // done\n return; }",
        &Options::default(),
    );
}

#[test]
fn cuda_integer_arithmetic_and_parentheses() {
    validate(
        "__global__ void calculate(int *out) { int value = (8 - 2) / 3; out[0] = value; }",
        &Options::default(),
    );
}

#[test]
fn cuda_sequential_buffer_stores_validate() {
    validate(
        "__global__ void update(unsigned int *out) { out[0] = 1u; out[1] = out[0] + 2u; }",
        &Options::default(),
    );
}

#[test]
fn cuda_rejects_unsupported_or_invalid_source() {
    for source in [
        "",
        "__device__ void helper() {}",
        "__global__ int value() { return 1; }",
        "__global__ void scalar(double value) {}",
        "__global__ void bad(float **out) {}",
        "__global__ void bad(double *out) {}",
        "#include <cuda_runtime.h>\n__global__ void empty() {}",
        "__global__ void bad(float *out) { out[0] = 1.0; }",
        "__global__ void bad(float *out) { out[0] = 1L; }",
        "__global__ void bad(const float *out) { out[0] = 1.0f; }",
        "__global__ void bad(float *out, float *out) {}",
        "__global__ void bad(unsigned int *out) { unsigned int out = 1u; }",
        "__global__ void bad(unsigned int *out) { out[0] = missing; }",
        "__global__ void bad(unsigned int *out) { out[0] = threadIdx.w; }",
        "__global__ void bad(unsigned int *out) { out[0.5f] = 1u; }",
        "__global__ void bad(unsigned int *out) { out[0] = threadIdx.x ? 1u 2u; }",
        "__global__ void bad(unsigned int *out) { out[0] = 4294967296u; }",
        "__global__ void bad(unsigned int *out) { out[0] = 08u; }",
        "__global__ void bad(unsigned int *out) { switch (threadIdx.x) { case 0: out[0] = 1u; } }",
        "__global__ void bad(unsigned int *out) { out[0] = atomicAdd(out, 1u); }",
        "__global__ void bad(unsigned int *out) { return 1u; }",
        "__global__ void empty() {} trailing",
        "__global__ void first() {} __global__ void second() {}",
        "__global__ void empty() { /* unterminated",
    ] {
        assert!(
            parse_str(source, &Options::default()).is_err(),
            "accepted unsupported source: {source}"
        );
    }
}

#[test]
fn cuda_error_span_points_into_original_source() {
    let source = "__global__ void bad(unsigned int *out) { out[0] = missing; }";
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert_eq!(&source[range], "missing");
    assert!(!error.message.is_empty());
}

#[test]
fn cuda_rejects_zero_workgroup_dimension() {
    for workgroup_size in [[0, 1, 1], [1, 0, 1], [1, 1, 0]] {
        assert!(parse_str("__global__ void empty() {}", &Options { workgroup_size }).is_err());
    }
}

#[cfg(all(feature = "wgsl-out", feature = "wgsl-in"))]
#[test]
fn cuda_emits_wgsl_that_roundtrips() {
    let (module, info) = validate(VECTOR_ADD, &Options::default());
    let output =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let reparsed = naga::front::wgsl::parse_str(&output).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&reparsed)
    .unwrap();
}

#[cfg(feature = "spv-out")]
#[test]
fn cuda_emits_spirv() {
    let (module, info) = validate(VECTOR_ADD, &Options::default());
    let words = naga::back::spv::write_vec(&module, &info, &Default::default(), None).unwrap();
    assert_eq!(words[0], 0x07230203);
    rspirv::dr::load_words(words).unwrap();
}

#[cfg(feature = "glsl-out")]
#[test]
fn cuda_emits_glsl() {
    let (module, info) = validate(VECTOR_ADD, &Options::default());
    let mut output = String::new();
    let options = naga::back::glsl::Options::default();
    let pipeline = naga::back::glsl::PipelineOptions {
        shader_stage: naga::ShaderStage::Compute,
        entry_point: "add".into(),
        multiview: None,
    };
    naga::back::glsl::Writer::new(
        &mut output,
        &module,
        &info,
        &options,
        &pipeline,
        Default::default(),
    )
    .unwrap()
    .write()
    .unwrap();
    assert!(output.contains("void main()"));
}

#[cfg(feature = "hlsl-out")]
#[test]
fn cuda_emits_hlsl() {
    let (module, info) = validate(VECTOR_ADD, &Options::default());
    let mut output = String::new();
    let options = naga::back::hlsl::Options::default();
    let pipeline = naga::back::hlsl::PipelineOptions::default();
    let reflection = naga::back::hlsl::Writer::new(&mut output, &options, &pipeline)
        .write(&module, &info, None)
        .unwrap();
    for name in reflection.entry_point_names {
        name.unwrap();
    }
    assert!(output.contains("numthreads"));
}

#[cfg(feature = "hlsl-out")]
#[test]
fn cuda_hlsl_grid_dimensions_require_dispatch_constants() {
    let (module, info) = validate(
        "__global__ void grid(unsigned int *out) { out[0] = gridDim.x; }",
        &Options::default(),
    );
    let mut output = String::new();
    let options = naga::back::hlsl::Options::default();
    let pipeline = naga::back::hlsl::PipelineOptions::default();
    let result =
        naga::back::hlsl::Writer::new(&mut output, &options, &pipeline).write(&module, &info, None);
    assert!(matches!(
        result,
        Err(naga::back::hlsl::Error::MissingSpecialConstantsBinding)
    ));
}

#[cfg(feature = "hlsl-out")]
#[test]
fn cuda_hlsl_grid_dimensions_use_dispatch_constants() {
    let (module, info) = validate(
        "__global__ void grid(unsigned int *out) { out[0] = gridDim.x; out[1] = gridDim.y; out[2] = gridDim.z; out[3] = blockIdx.x; }",
        &Options::default(),
    );
    let mut output = String::new();
    let options = naga::back::hlsl::Options {
        special_constants_binding: Some(naga::back::hlsl::BindTarget {
            register: 7,
            space: 3,
            ..Default::default()
        }),
        ..Default::default()
    };
    let pipeline = naga::back::hlsl::PipelineOptions::default();
    let reflection = naga::back::hlsl::Writer::new(&mut output, &options, &pipeline)
        .write(&module, &info, None)
        .unwrap();
    for name in reflection.entry_point_names {
        name.unwrap();
    }
    assert!(output.contains("register(b7, space3)"));
    for component in ["x", "y", "z"] {
        assert!(output.contains(&format!(
            "uint3(_NagaConstants.first_vertex, _NagaConstants.first_instance, _NagaConstants.other).{component}"
        )));
    }
    assert!(output.contains("asuint(blockIdx.x)"));
}

#[cfg(all(feature = "hlsl-out", feature = "wgsl-in"))]
#[test]
fn hlsl_dispatch_constants_are_required_only_for_selected_entry_points() {
    let module = naga::front::wgsl::parse_str(
        "struct Input { @builtin(num_workgroups) counts: vec3<u32> }\n\
         @compute @workgroup_size(1) fn grid(input: Input) {}\n\
         @compute @workgroup_size(1) fn empty() {}",
    )
    .unwrap();
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    let options = naga::back::hlsl::Options::default();
    for entry_point in ["grid", "empty"] {
        let mut output = String::new();
        let pipeline = naga::back::hlsl::PipelineOptions {
            entry_point: Some((naga::ShaderStage::Compute, entry_point.into())),
        };
        let result = naga::back::hlsl::Writer::new(&mut output, &options, &pipeline)
            .write(&module, &info, None);
        assert_eq!(result.is_ok(), entry_point == "empty");
    }
}

#[cfg(feature = "msl-out")]
#[test]
fn cuda_emits_msl() {
    let (module, info) = validate(VECTOR_ADD, &Options::default());
    let (output, reflection) =
        naga::back::msl::write_string(&module, &info, &Default::default(), &Default::default())
            .unwrap();
    for name in reflection.entry_point_names {
        name.unwrap();
    }
    assert!(output.contains("kernel void"));
}
