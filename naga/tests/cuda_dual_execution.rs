#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

mod cuda_helpers;
use cuda_helpers::{execute, validate};

fn function<'a>(
    module: &'a naga::Module,
    name: &str,
) -> (naga::Handle<naga::Function>, &'a naga::Function) {
    module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some(name))
        .unwrap()
}

fn result_scalar(module: &naga::Module, name: &str) -> Option<naga::Scalar> {
    function(module, name).1.result.as_ref().map(|result| {
        assert_eq!(result.binding, None);
        let naga::TypeInner::Scalar(scalar) = module.types[result.ty].inner else {
            panic!("expected a scalar helper result");
        };
        scalar
    })
}

fn call_targets(body: &naga::Block) -> Vec<naga::Handle<naga::Function>> {
    body.iter()
        .filter_map(|statement| match *statement {
            naga::Statement::Call { function, .. } => Some(function),
            _ => None,
        })
        .collect()
}

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert!(!error.message.is_empty(), "{source}");
    assert!(
        source.get(error.span.to_range().unwrap()).is_some(),
        "{source}: {error}"
    );
    error
}

fn rejected_at(source: &str, token: &str, occurrence: usize) {
    let error = rejected(source);
    let start = source.match_indices(token).nth(occurrence).unwrap().0;
    assert_eq!(
        error.span,
        naga::Span::from(start..start + token.len()),
        "{source}: {error}"
    );
}

#[test]
fn cuda_dual_execution_helpers_accept_each_execution_and_type_order() {
    for header in [
        "__host__ __device__ int",
        "__device__ __host__ int",
        "__host__ int __device__",
        "__device__ int __host__",
        "int __host__ __device__",
        "int __device__ __host__",
    ] {
        let (module, _) = validate(&format!(
            "{header} helper(int value) {{ return value + 1; }}
            __global__ void kernel(int *out) {{ out[0] = helper(3); }}"
        ));
        let (helper, _) = function(&module, "helper");
        assert_eq!(module.functions.len(), 1);
        assert_eq!(result_scalar(&module, "helper"), Some(naga::Scalar::I32));
        assert_eq!(
            call_targets(&module.entry_points[0].function.body),
            [helper]
        );
    }
}

#[test]
fn cuda_dual_execution_helpers_accept_inline_at_each_header_position() {
    for header in [
        ["__host__", "__device__", "int"],
        ["__device__", "__host__", "int"],
        ["__host__", "int", "__device__"],
        ["__device__", "int", "__host__"],
        ["int", "__host__", "__device__"],
        ["int", "__device__", "__host__"],
    ] {
        for position in 0..=header.len() {
            let mut specifiers = header.to_vec();
            specifiers.insert(position, "inline");
            let (module, _) = validate(&format!(
                "{} helper(int value) {{ return value; }}
                __global__ void kernel(int *out) {{ out[0] = helper(3); }}",
                specifiers.join(" ")
            ));
            assert_eq!(result_scalar(&module, "helper"), Some(naga::Scalar::I32));
        }
    }
}

#[test]
fn cuda_dual_execution_helpers_keep_each_supported_scalar_result() {
    for (ty, value, scalar) in [
        ("bool", "true", naga::Scalar::BOOL),
        ("int", "-3", naga::Scalar::I32),
        ("unsigned", "0xffffffff", naga::Scalar::U32),
        ("float", "3.5f", naga::Scalar::F32),
    ] {
        let output_type = if scalar == naga::Scalar::BOOL {
            "int"
        } else {
            ty
        };
        let (module, _) = validate(&format!(
            "{ty} __host__ __device__ helper({ty} value) {{ return value; }}
            __global__ void kernel({output_type} *out) {{ out[0] = helper({value}); }}"
        ));
        assert_eq!(result_scalar(&module, "helper"), Some(scalar));
        let helper = function(&module, "helper").1;
        assert_eq!(helper.arguments.len(), 1);
        assert_eq!(helper.arguments[0].binding, None);
        assert_eq!(
            module.types[helper.arguments[0].ty].inner,
            naga::TypeInner::Scalar(scalar)
        );
    }
}

#[test]
fn cuda_dual_execution_headers_allow_interleaved_unsigned_int_and_const() {
    for header in [
        "unsigned __host__ int __device__",
        "int __device__ unsigned __host__",
        "__host__ unsigned inline __device__ int",
        "__device__ int inline __host__ unsigned",
        "const unsigned __device__ inline int __host__",
        "unsigned __host__ const int __device__ inline",
        "__host__ int __device__ inline const unsigned",
    ] {
        let (module, _) = validate(&format!(
            "{header} helper(unsigned value) {{ return ++value; }}
            __global__ void kernel(unsigned *out) {{ out[0] = helper(3); }}"
        ));
        assert_eq!(result_scalar(&module, "helper"), Some(naga::Scalar::U32));
    }
}

#[test]
fn cuda_dual_execution_alias_returns_keep_inherited_const_and_parameter_copies() {
    let (module, _) = validate(
        "using Whole = int;
        using Frozen = const Whole;
        const __host__ Frozen inline __device__ advance(Whole value) { return ++value; }
        __device__ __host__ Frozen fixed(const Whole value) { return value; }
        __global__ void kernel(Whole *out) { Whole value = 3; out[0] = advance(fixed(value)); out[1] = value; }",
    );
    assert_eq!(result_scalar(&module, "advance"), Some(naga::Scalar::I32));
    assert_eq!(result_scalar(&module, "fixed"), Some(naga::Scalar::I32));
    let helper = function(&module, "advance").1;
    assert!(helper
        .local_variables
        .iter()
        .any(|(_, local)| local.name.as_deref() == Some("value")));
    assert!(helper
        .expressions
        .iter()
        .all(|(_, expression)| !matches!(expression, naga::Expression::GlobalVariable(_))));
}

#[test]
fn cuda_dual_execution_void_helpers_keep_early_implicit_and_forward_returns() {
    for header in [
        "__host__ __device__ void",
        "void __device__ __host__",
        "const inline __device__ void __host__",
        "__host__ void const inline __device__",
    ] {
        let (module, _) = validate(&format!(
            "{header} consume(int value) {{ if (value) return; }}
            __device__ __host__ void forward(int value) {{ return consume(value); }}
            __global__ void kernel() {{ (forward(3)); }}"
        ));
        let (consume, _) = function(&module, "consume");
        let (forward, helper) = function(&module, "forward");
        assert_eq!(result_scalar(&module, "consume"), None);
        assert_eq!(result_scalar(&module, "forward"), None);
        assert_eq!(call_targets(&helper.body), [consume]);
        assert_eq!(
            call_targets(&module.entry_points[0].function.body),
            [forward]
        );
    }
}

#[test]
fn cuda_dual_helpers_can_call_earlier_device_only_helpers_in_device_compilation() {
    let (module, _) = validate(
        "__device__ int first(int value) { return value + 1; }
        __host__ __device__ int second(int value) { return first(value); }
        __device__ int third(int value) { return second(value); }
        __device__ __host__ int fourth(int value) { return second(third(value)); }
        __global__ void kernel(int *out) { out[0] = fourth(3); }",
    );
    let (first, _) = function(&module, "first");
    let (second, helper) = function(&module, "second");
    assert_eq!(call_targets(&helper.body), [first]);
    let (third, helper) = function(&module, "third");
    assert_eq!(call_targets(&helper.body), [second]);
    let (fourth, helper) = function(&module, "fourth");
    assert_eq!(call_targets(&helper.body), [third, second]);
    assert_eq!(
        call_targets(&module.entry_points[0].function.body),
        [fourth]
    );
}

#[test]
fn cuda_dual_execution_helpers_preserve_kernel_bindings_and_workgroup_size() {
    let source = "__host__ inline __device__ float scale(float value, int amount) { return value * amount; }
        extern \"C\" __global__ void kernel(float factor, const float *input, int amount, float *out) {
            unsigned i = threadIdx.x; out[i] = scale(input[i], amount) + factor;
        }";
    let module = parse_str(
        source,
        &Options {
            workgroup_size: [8, 4, 2],
        },
    )
    .unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    assert_eq!(module.entry_points.len(), 1);
    assert_eq!(module.entry_points[0].workgroup_size, [8, 4, 2]);
    assert_eq!(module.entry_points[0].stage, naga::ShaderStage::Compute);
    assert!(module.entry_points[0].function.result.is_none());
    assert_eq!(module.global_variables.len(), 4);
    for (binding, (_, variable)) in module.global_variables.iter().enumerate() {
        assert_eq!(
            variable.binding,
            Some(naga::ResourceBinding {
                group: 0,
                binding: binding as u32
            })
        );
        match binding {
            0 | 2 => assert_eq!(variable.space, naga::AddressSpace::Uniform),
            1 => assert_eq!(
                variable.space,
                naga::AddressSpace::Storage {
                    access: naga::StorageAccess::LOAD
                }
            ),
            3 => assert_eq!(
                variable.space,
                naga::AddressSpace::Storage {
                    access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE
                }
            ),
            _ => unreachable!(),
        }
    }
    assert!(function(&module, "scale")
        .1
        .arguments
        .iter()
        .all(|argument| argument.binding.is_none()));
    assert_eq!(execute("__host__ __device__ int unused(int value) { return value; } __global__ void kernel(int *out) { int value = 3; out[0] = ++value * 2; }", &[0]), [8]);
}

#[test]
fn cuda_dual_execution_function_scopes_restore_global_aliases_and_names() {
    let (module, _) = validate(
        "using Whole = int;
        __host__ __device__ int first(int Whole) { using Local = float; return Whole; }
        __device__ __host__ Whole second(Whole value) { return first(value); }
        __global__ void kernel(Whole *out) { Whole value = 3; out[0] = second(value); }
        using After = Whole;",
    );
    assert_eq!(result_scalar(&module, "second"), Some(naga::Scalar::I32));
    for source in [
        "__host__ __device__ int first() { using Local = int; return 1; } Local __host__ __device__ second() { return 2; } __global__ void kernel() {}",
        "using Whole = int; __host__ __device__ int helper(int Whole, Whole other) { return other; } __global__ void kernel() {}",
        "__host__ __device__ int helper(int value) { int helper = 3; return helper(value); } __global__ void kernel() {}",
    ] { rejected(source); }
}

#[test]
fn cuda_dual_execution_declarator_names_still_collide_with_global_types() {
    for header in [
        "__host__ int __device__",
        "unsigned __host__ __device__",
        "__device__ inline __host__ float",
    ] {
        rejected_at(&format!("using Whole = int; {header} Whole(int value) {{ return value; }} __global__ void kernel() {{}}"), "Whole", 1);
    }
}

#[test]
fn cuda_dual_execution_duplicates_report_the_repeated_execution_keyword() {
    for (source, token) in [
        ("/* 漢 */ __host__ __host__ __device__ int helper() { return 1; } __global__ void kernel() {}", "__host__"),
        ("__device__ __host__ inline int __host__ helper() { return 1; } __global__ void kernel() {}", "__host__"),
        ("__host__ __device__ int __device__ helper() { return 1; } __global__ void kernel() {}", "__device__"),
        ("__device__ __host__ __device__ void helper() {} __global__ void kernel() {}", "__device__"),
    ] { rejected_at(source, token, 1); }
}

#[test]
fn cuda_dual_execution_helpers_reject_repeated_inline_const_and_return_types() {
    for source in [
        "inline __host__ __device__ int inline helper() { return 1; } __global__ void kernel() {}",
        "__device__ inline __host__ inline int helper() { return 1; } __global__ void kernel() {}",
    ] {
        rejected_at(source, "inline", 1);
    }
    for source in [
        "const __host__ __device__ int const helper() { return 1; } __global__ void kernel() {}",
        "using Frozen = const int; const __host__ Frozen __device__ const helper() { return 1; } __global__ void kernel() {}",
    ] { rejected_at(source, "const", source.match_indices("const").count() - 1); }
    for ty in [
        "int int",
        "unsigned unsigned",
        "int float",
        "void int",
        "float unsigned",
    ] {
        rejected(&format!(
            "__host__ __device__ {ty} helper() {{ return 1; }} __global__ void kernel() {{}}"
        ));
    }
}

#[test]
fn cuda_dual_execution_rejects_host_only_headers_at_the_host_keyword() {
    for header in [
        "__host__ int",
        "inline int __host__",
        "const __host__ void inline",
    ] {
        rejected_at(
            &format!("/* 漢 */ {header} helper() {{}} __global__ void kernel() {{}}"),
            "__host__",
            0,
        );
    }
}

#[test]
fn cuda_dual_execution_rejects_global_conflicts_at_the_conflicting_keyword() {
    for (header, token) in [
        ("__host__ __global__ void", "__global__"),
        ("__global__ __host__ void", "__host__"),
        ("__host__ __device__ __global__ void", "__global__"),
        ("__device__ __host__ __global__ void", "__global__"),
        ("__host__ __global__ __device__ void", "__global__"),
        ("__device__ __global__ __host__ void", "__global__"),
        ("__global__ __host__ __device__ void", "__host__"),
        ("__global__ __device__ __host__ void", "__device__"),
    ] {
        rejected_at(&format!("{header} kernel() {{}}"), token, 0);
    }
}

#[test]
fn cuda_host_specifier_remains_invalid_outside_function_headers() {
    for source in [
        "__global__ void kernel() { __host__ int value = 1; }",
        "__global__ void kernel() { for (__host__ int value = 0; false;) {} }",
        "__host__ __device__ int helper(__host__ int value) { return value; } __global__ void kernel() {}",
        "__global__ void kernel(int __host__ value) {}",
        "__global__ void kernel() { int value = (__host__ int)1; }",
        "__global__ void kernel() { int value = static_cast<int __host__>(1); }",
        "typedef __host__ int Whole; __global__ void kernel() {}",
        "using Whole = int __host__; __global__ void kernel() {}",
        "__device__ int helper() __host__ { return 1; } __global__ void kernel() {}",
        "__device__ int helper __host__() { return 1; } __global__ void kernel() {}",
        "__global__ void kernel() { return; __host__ int value = 1; }",
    ] { rejected(source); }
}

#[test]
fn cuda_dual_execution_accepts_static_and_cuda_inlining_specifiers() {
    for source in [
        "static __host__ __device__ int helper() { return 1; } __global__ void kernel() {}",
        "__host__ __device__ __forceinline__ int helper() { return 1; } __global__ void kernel() {}",
        "__device__ __host__ __noinline__ int helper() { return 1; } __global__ void kernel() {}",
    ] {
        let (module, _) = validate(source);
        assert_eq!(module.functions.len(), 1);
        assert_eq!(result_scalar(&module, "helper"), Some(naga::Scalar::I32));
    }
}

#[test]
fn cuda_dual_execution_keeps_existing_helper_body_and_signature_restrictions() {
    for source in [
        "extern \"C\" __host__ __device__ int helper() { return 1; } __global__ void kernel() {}",
        "__host__ __device__ int helper(int value); __global__ void kernel() {}",
        "__host__ __device__ int first() { return later(); } __host__ __device__ int later() { return 1; } __global__ void kernel() {}",
        "__host__ __device__ int helper(int value) { return helper(value); } __global__ void kernel() {}",
        "__host__ __device__ int helper(int value) { return value; } __host__ __device__ int helper(int other) { return other; } __global__ void kernel() {}",
        "__host__ __device__ int helper(int *value) { return value[0]; } __global__ void kernel() {}",
        "__host__ __device__ int *helper(int value) { return value; } __global__ void kernel() {}",
        "__host__ __device__ int helper() { return threadIdx.x; } __global__ void kernel() {}",
        "__host__ __device__ void helper() { __shared__ int value; } __global__ void kernel() {}",
        "__host__ __device__ void helper() { __syncthreads(); } __global__ void kernel() {}",
        "__host__ __device__ int helper(const int value) { return ++value; } __global__ void kernel() {}",
        "__host__ __device__ int helper(int value) { if (value) return value; } __global__ void kernel() {}",
        "__host__ __device__ void helper() { return 1; } __global__ void kernel() {}",
        "__host__ __device__ void helper() {} __global__ void kernel() { int value = helper(); }",
    ] { rejected(source); }
}

#[test]
fn cuda_dual_execution_keeps_sole_kernel_and_complete_source_requirements() {
    for source in [
        "__host__ __device__ int helper() { return 1; }",
        "__host__ __device__ int helper() { return 1; } __global__ void first() {} __global__ void second() {}",
        "__global__ void kernel() {} __host__ __device__ int later() { return 1; }",
        "__host__ __device__ int helper() { return 1; } __global__ void kernel() {} /* 漢 */ @",
        "__host__ __device__ int helper() { return 1; @ } __global__ void kernel() {}",
    ] { rejected(source); }
    validate("__host__ __device__ int helper() { return 1; } __global__ void kernel() { return; { helper(); } }");
}

#[test]
#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
fn cuda_dual_execution_helpers_roundtrip_through_wgsl() {
    let (module, info) = validate(
        "using Frozen = const float;
        const inline Frozen __device__ __host__ scale(float value) { return value * 2.0f; }
        int __host__ unsigned inline __device__ advance(unsigned value) { return ++value; }
        __host__ void const __device__ consume(bool stop) { if (stop) return; }
        extern \"C\" __global__ void kernel(float *out, unsigned count) {
            unsigned i = threadIdx.x;
            if (i < count) { consume(false); out[i] = scale(advance(i)); }
        }",
    );
    let text =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let reparsed = naga::front::wgsl::parse_str(&text).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&reparsed)
    .unwrap();
    assert_eq!(reparsed.functions.len(), 3);
    assert_eq!(reparsed.entry_points.len(), 1);
}
