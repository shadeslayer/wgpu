use naga::front::cuda::{parse_str, Options};

use super::helpers::{validate, validate_with_options};

const BUILTINS: [&str; 4] = ["threadIdx", "blockIdx", "blockDim", "gridDim"];
const FIXTURE: &str = include_str!("../../in/cuda/helper-builtins.cu");

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

#[test]
fn cuda_helper_builtin_components_have_unsigned_type() {
    for builtin in BUILTINS {
        for component in ["x", "y", "z"] {
            let (module, _) = validate(&format!(
                "__device__ int kind(unsigned value) {{ return 1; }}
                __device__ int kind(int value) {{ return 2; }}
                __device__ int kind(float value) {{ return 3; }}
                __device__ int coordinate() {{ return kind({builtin}.{component}); }}
                __global__ void kernel(int *out) {{ out[0] = coordinate(); }}"
            ));
            let coordinate = module
                .functions
                .iter()
                .find(|(_, function)| function.name.as_deref() == Some("coordinate"))
                .unwrap()
                .1;
            let selected = coordinate
                .body
                .iter()
                .find_map(|statement| match *statement {
                    naga::Statement::Call { function, .. } => Some(function),
                    _ => None,
                })
                .unwrap();
            assert_eq!(
                module.types[module.functions[selected].arguments[0].ty].inner,
                naga::TypeInner::Scalar(naga::Scalar::U32)
            );
        }
    }
}

#[test]
fn cuda_helper_builtins_preserve_chains_overloads_and_declared_arguments() {
    let (module, _) = validate(
        "using Coordinate = const unsigned;
        static __device__ inline Coordinate component(Coordinate axis) {
            if (axis == 0u) return threadIdx.x;
            if (axis == 1u) return blockIdx.y;
            return gridDim.z + blockDim.z;
        }
        __device__ float component(float offset) { return float(threadIdx.y) + offset; }
        __host__ __device__ unsigned chain(unsigned axis) { return component(axis) + component(axis); }
        __device__ void observe() { if (threadIdx.z) return; unsigned width = blockDim.x; }
        __global__ void kernel(unsigned *out) { observe(); out[0] = chain(2u); out[1] = unsigned(component(1.5f)); }",
    );
    for (_, function) in module.functions.iter() {
        let expected_arguments = usize::from(function.name.as_deref() != Some("observe"));
        assert_eq!(function.arguments.len(), expected_arguments);
        assert!(function
            .arguments
            .iter()
            .all(|argument| argument.binding.is_none()));
    }
}

#[test]
fn cuda_helper_and_kernel_builtin_reads_share_compute_inputs() {
    let (module, _) = validate(
        "__device__ unsigned coordinate() { return threadIdx.x + threadIdx.y + blockIdx.z + gridDim.x + blockDim.y; }
        __global__ void kernel(unsigned *out) { out[threadIdx.z] = coordinate() + coordinate() + blockIdx.y + gridDim.z; }",
    );
    let arguments = &module.entry_points[0].function.arguments;
    for builtin in [
        naga::BuiltIn::LocalInvocationId,
        naga::BuiltIn::WorkGroupId,
        naga::BuiltIn::NumWorkGroups,
    ] {
        assert_eq!(
            arguments
                .iter()
                .filter(|argument| argument.binding == Some(naga::Binding::BuiltIn(builtin)))
                .count(),
            1
        );
    }
    assert_eq!(arguments.len(), 3);
}

#[test]
fn cuda_helper_builtin_globals_do_not_consume_resource_bindings() {
    let (module, _) = validate(FIXTURE);
    let resources: Vec<_> = module
        .global_variables
        .iter()
        .filter_map(|(_, global)| global.binding.map(|binding| (binding, global)))
        .collect();
    assert_eq!(resources.len(), 5);
    for (index, (binding, _)) in resources.iter().enumerate() {
        assert_eq!(
            *binding,
            naga::ResourceBinding {
                group: 0,
                binding: index as u32
            }
        );
    }
    for index in [0, 2] {
        assert_eq!(
            resources[index].1.space,
            naga::AddressSpace::Storage {
                access: naga::StorageAccess::LOAD
            }
        );
    }
    for index in [1, 3] {
        assert_eq!(resources[index].1.space, naga::AddressSpace::Uniform);
        assert!(matches!(
            module.types[resources[index].1.ty].inner,
            naga::TypeInner::Struct { span: 16, .. }
        ));
    }
    assert_eq!(
        resources[4].1.space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE
        }
    );
    assert!(matches!(
        module.types[resources[2].1.ty].inner,
        naga::TypeInner::Array { stride: 12, .. }
    ));
}

#[test]
fn cuda_helper_compute_inputs_are_initialized_before_source_control_flow() {
    let (module, _) = validate(
        "__device__ unsigned coordinate() { return threadIdx.x + blockIdx.y + gridDim.z; }
        __global__ void kernel(unsigned *out) { if (coordinate()) return; out[0] = coordinate(); }",
    );
    let entry = &module.entry_points[0].function;
    let first_call = entry
        .body
        .iter()
        .position(|statement| matches!(statement, naga::Statement::Call { .. }))
        .unwrap();
    assert_eq!(
        module
            .global_variables
            .iter()
            .filter(|(_, global)| global.space == naga::AddressSpace::Private)
            .count(),
        3
    );
    for (handle, global) in module
        .global_variables
        .iter()
        .filter(|(_, global)| global.space == naga::AddressSpace::Private)
    {
        let initialized_before_call = entry.body.iter().take(first_call).any(|statement| match *statement {
            naga::Statement::Store { pointer, .. } => matches!(entry.expressions[pointer], naga::Expression::GlobalVariable(stored) if stored == handle),
            _ => false,
        });
        assert!(initialized_before_call, "{:?}", global.name);
    }
}

#[test]
fn cuda_helper_block_dimensions_follow_options_without_entry_inputs() {
    let (module, _) = validate_with_options(
        "__device__ unsigned width() { return blockDim.x; }
        __device__ unsigned height() { return blockDim.y; }
        __device__ unsigned depth() { return blockDim.z; }
        __global__ void kernel(unsigned *out) { out[0] = width(); out[1] = height(); out[2] = depth(); }",
        &Options { workgroup_size: [2, 3, 5] },
    );
    assert!(module.entry_points[0].function.arguments.is_empty());
    for (name, dimension) in [("width", 2), ("height", 3), ("depth", 5)] {
        let function = module
            .functions
            .iter()
            .find(|(_, function)| function.name.as_deref() == Some(name))
            .unwrap()
            .1;
        assert!(function.expressions.iter().any(|(_, expression)| matches!(expression, naga::Expression::Literal(naga::Literal::U32(value)) if *value == dimension)));
    }
}

#[test]
fn cuda_helper_builtin_reads_remain_ineligible_constant_expressions() {
    for builtin in BUILTINS {
        for declaration in [
            format!("constexpr unsigned Count = {builtin}.x;"),
            format!("const unsigned Count = {builtin}.x; int values[Count];"),
        ] {
            rejected(&format!("__device__ void helper() {{ {declaration} }} __global__ void kernel() {{ helper(); }}"));
        }
        validate(&format!(
            "__device__ int helper() {{ constexpr int Count = true ? 3 : int({builtin}.x); int values[Count]{{}}; return values[2]; }}
            __global__ void kernel(int *out) {{ out[0] = helper(); }}"
        ));
    }
}

#[test]
fn cuda_helper_builtin_components_are_read_only() {
    for builtin in BUILTINS {
        for component in ["x", "y", "z"] {
            for operation in [
                format!("{builtin}.{component} = 1u"),
                format!("{builtin}.{component} += 1u"),
                format!("++{builtin}.{component}"),
                format!("{builtin}.{component}++"),
            ] {
                rejected(&format!("__device__ void helper() {{ {operation}; }} __global__ void kernel() {{ helper(); }}"));
            }
        }
    }
}

#[test]
fn cuda_helper_builtin_projection_errors_preserve_source_spans() {
    for builtin in BUILTINS {
        let source = format!("/* λ */ __device__ unsigned helper() {{ return {builtin}.w; }} __global__ void kernel() {{}}");
        let error = rejected(&source);
        assert_eq!(&source[error.span.to_range().unwrap()], "w");
        for expression in [
            builtin.to_owned(),
            format!("{builtin}.xy"),
            format!("{builtin}[0]"),
            format!("&{builtin}.x"),
        ] {
            rejected(&format!("__device__ void helper() {{ {expression}; }} __global__ void kernel() {{ helper(); }}"));
        }
    }
}

#[test]
fn cuda_helper_builtin_identifiers_remain_reserved_in_source_scopes() {
    for builtin in BUILTINS {
        for source in [
            format!("using {builtin} = unsigned; __global__ void kernel() {{}}"),
            format!("__device__ unsigned {builtin}() {{ return 1u; }} __global__ void kernel() {{}}"),
            format!("__device__ void helper(unsigned {builtin}) {{}} __global__ void kernel() {{}}"),
            format!("__device__ void helper() {{ unsigned {builtin} = 1u; }} __global__ void kernel() {{}}"),
            format!("__device__ void helper() {{ using {builtin} = unsigned; }} __global__ void kernel() {{}}"),
        ] {
            let error = rejected(&source);
            assert_eq!(&source[error.span.to_range().unwrap()], builtin);
        }
        validate(&format!("__device__ unsigned helper() {{ unsigned {builtin}_value = {builtin}.x; return {builtin}_value; }} __global__ void kernel(unsigned *out) {{ out[0] = helper(); }}"));
    }
}

#[test]
fn cuda_scalar_void_and_qualified_helpers_accept_compute_builtin_reads() {
    for builtin in ["threadIdx.x", "blockIdx.x", "blockDim.x", "gridDim.x"] {
        validate(&format!("__device__ unsigned helper() {{ return {builtin}; }} __global__ void kernel(unsigned *out) {{ out[0] = helper(); }}"));
        validate(&format!("__device__ void consume() {{ unsigned value = {builtin}; }} __global__ void kernel(int *output) {{ consume(); }}"));
    }
    for source in [
        "__device__ inline int helper() { return threadIdx.x; } __global__ void kernel() {}",
        "__host__ __device__ int helper() { return threadIdx.x; } __global__ void kernel() {}",
        "static __device__ int helper() { return threadIdx.x; } __global__ void kernel() {}",
    ] {
        validate(source);
    }
}

#[test]
fn cuda_helper_builtin_errors_remain_checked_in_unused_and_discarded_source() {
    for body in [
        "return; unsigned value = threadIdx.w;",
        "constexpr unsigned Value = true ? 3u : gridDim.w;",
        "if (false) { unsigned value = blockIdx.w; }",
        "unsigned value = blockDim.x + missing;",
    ] {
        rejected(&format!(
            "__device__ void unused() {{ {body} }} __global__ void kernel() {{}}"
        ));
    }
    validate("__device__ void unused() { threadIdx.x; blockIdx.y; gridDim.z; blockDim.y; } __global__ void kernel() { return; }");
}

#[test]
fn cuda_helper_builtin_support_preserves_barrier_restrictions() {
    for source in [
        "__device__ unsigned lane() { __syncthreads(); return threadIdx.x; } __global__ void kernel() {}",
        "__device__ unsigned lane() { __shared__ int value; return threadIdx.x; } __global__ void kernel() {}",
        "__device__ unsigned lane() { return threadIdx.x; } __global__ void kernel() { if (lane()) return; __syncthreads(); }",
        "__device__ unsigned lane() { return threadIdx.x; } __global__ void kernel() { if (lane()) __syncthreads(); }",
    ] {
        rejected(source);
    }
    validate("__device__ unsigned lane() { if (threadIdx.x) return threadIdx.x; return 0u; } __global__ void kernel(unsigned *out) { unsigned index = lane(); __syncthreads(); out[index] = lane(); }");
}

#[test]
fn cuda_helper_builtin_function_context_and_expression_depth_remain_bounded() {
    validate("__device__ unsigned first() { return threadIdx.x; } constexpr unsigned Bias = 3u; __device__ unsigned second() { return gridDim.z + Bias; } __device__ unsigned third() { return first() + second() + blockIdx.y; } __global__ void kernel(unsigned *out) { out[0] = first(); out[1] = second(); out[2] = third(); }");
    let deep_expression = format!("{}threadIdx.x{}", "(".repeat(128), ")".repeat(128));
    rejected(&format!("__device__ unsigned helper() {{ return {deep_expression}; }} __global__ void kernel() {{}}"));
}

#[cfg(feature = "hlsl-out")]
#[test]
fn cuda_helper_grid_dimensions_require_existing_hlsl_dispatch_constants() {
    for source in [
        "__device__ unsigned groups() { return gridDim.x; } __global__ void kernel(unsigned *out) { out[0] = groups(); }",
        "__device__ unsigned unused() { return gridDim.z; } __global__ void kernel() { return; }",
        "__device__ unsigned unused() { constexpr unsigned Value = true ? 3u : gridDim.y; return Value; } __global__ void kernel() {}",
    ] {
        let (module, info) = validate(source);
        assert!(module.entry_points[0].function.arguments.iter().any(|argument| argument.binding == Some(naga::Binding::BuiltIn(naga::BuiltIn::NumWorkGroups))));
        let mut output = String::new();
        let options = naga::back::hlsl::Options::default();
        let pipeline = naga::back::hlsl::PipelineOptions::default();
        let result = naga::back::hlsl::Writer::new(&mut output, &options, &pipeline).write(&module, &info, None);
        assert!(matches!(result, Err(naga::back::hlsl::Error::MissingSpecialConstantsBinding)));
        let options = naga::back::hlsl::Options { special_constants_binding: Some(naga::back::hlsl::BindTarget { register: 7, space: 3, ..Default::default() }), ..Default::default() };
        output.clear();
        naga::back::hlsl::Writer::new(&mut output, &options, &pipeline).write(&module, &info, None).unwrap();
        assert!(output.contains("register(b7, space3)"));
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_helper_builtin_fixture_roundtrips_through_shared_wgsl_helpers() {
    let (module, info) = validate(FIXTURE);
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}

#[test]
fn cuda_helper_builtin_reads_preserve_lazy_calls_and_ordinary_source_lookup() {
    validate("__device__ unsigned lane() { return threadIdx.x; } __device__ unsigned choose(bool selected) { return selected ? lane() : blockIdx.x; } __device__ unsigned combined() { unsigned value = lane(); { unsigned lane = gridDim.y; value += lane; } return value + lane(); } __global__ void kernel(unsigned *out) { unsigned value = 1u; out[0] = value && choose(true); out[1] = value ? combined() : lane(); out[2] = (lane(), choose(false)); }");
    rejected("__device__ unsigned lane() { return threadIdx.x; } __device__ unsigned helper(unsigned lane) { return lane(); } __global__ void kernel() {}");
}
