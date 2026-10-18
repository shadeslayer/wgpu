#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::{execute, validate};

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert!(!error.message.is_empty(), "{source}");
    assert!(
        source.get(error.span.to_range().unwrap()).is_some(),
        "{source}"
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

fn helper_result(module: &naga::Module, name: &str) -> Option<naga::Scalar> {
    let function = module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some(name))
        .unwrap()
        .1;
    function.result.as_ref().map(|result| {
        let naga::TypeInner::Scalar(scalar) = module.types[result.ty].inner else {
            panic!("expected scalar helper result");
        };
        scalar
    })
}

#[test]
fn cuda_inline_helper_specifiers_accept_each_header_order() {
    for header in [
        "inline __device__ int",
        "inline int __device__",
        "__device__ inline int",
        "__device__ int inline",
        "int inline __device__",
        "int __device__ inline",
    ] {
        let (module, _) = validate(&format!(
            "{header} helper(int value) {{ return value + 1; }}
            __global__ void kernel(int *out) {{ out[0] = helper(3); }}"
        ));
        assert_eq!(helper_result(&module, "helper"), Some(naga::Scalar::I32));
    }
}

#[test]
fn cuda_inline_kernel_specifiers_accept_each_header_order_and_keep_the_body() {
    for header in [
        "inline __global__ void",
        "inline void __global__",
        "__global__ inline void",
        "__global__ void inline",
        "void inline __global__",
        "void __global__ inline",
    ] {
        let source = format!("{header} kernel(int *out) {{ out[0] = 7; }}");
        assert_eq!(execute(&source, &[0]), [7], "{header}");
    }
}

#[test]
fn cuda_inline_helpers_keep_each_supported_scalar_result_type() {
    for (ty, value, scalar) in [
        ("bool", "true", naga::Scalar::BOOL),
        ("int", "3", naga::Scalar::I32),
        ("unsigned", "3u", naga::Scalar::U32),
        ("float", "3.5f", naga::Scalar::F32),
    ] {
        let (module, _) = validate(&format!(
            "{ty} inline __device__ helper({ty} value) {{ return value; }}
            inline __global__ void kernel(int *out) {{ out[0] = helper({value}); }}"
        ));
        assert_eq!(helper_result(&module, "helper"), Some(scalar));
    }
}

#[test]
fn cuda_function_headers_allow_interleaved_unsigned_and_int_specifiers() {
    for header in [
        "__device__ unsigned inline int",
        "__device__ int inline unsigned",
        "unsigned __device__ inline int",
        "int __device__ unsigned inline",
        "inline int unsigned __device__",
        "unsigned inline __device__ int",
        "__device__ unsigned int",
        "int unsigned __device__",
    ] {
        let (module, _) = validate(&format!(
            "{header} helper(unsigned value) {{ return value; }}
            __global__ void kernel(unsigned *out) {{ out[0] = helper(0xffffffff); }}"
        ));
        assert_eq!(helper_result(&module, "helper"), Some(naga::Scalar::U32));
    }
}

#[test]
fn cuda_function_return_const_qualifiers_can_interleave_with_header_specifiers() {
    for header in [
        "const inline __device__ int",
        "inline const int __device__",
        "__device__ int const inline",
        "int inline const __device__",
        "__device__ unsigned const inline int",
        "__device__ int inline const unsigned",
    ] {
        let (module, _) = validate(&format!(
            "{header} helper(int value) {{ return value; }}
            __global__ void kernel(int *out) {{ out[0] = helper(3); }}"
        ));
        let expected = if header.contains("unsigned") {
            naga::Scalar::U32
        } else {
            naga::Scalar::I32
        };
        assert_eq!(helper_result(&module, "helper"), Some(expected));
    }
}

#[test]
fn cuda_inline_alias_returns_preserve_inherited_const_and_parameter_mutability() {
    let (module, _) = validate(
        "using Whole = int;
        using Frozen = const Whole;
        const inline Frozen __device__ increment(Whole value) { return ++value; }
        __device__ Frozen inline fixed(const Whole value) { return value; }
        inline __global__ void kernel(Whole *out) { int value = 3; out[0] = increment(fixed(value)); out[1] = value; }",
    );
    assert_eq!(helper_result(&module, "increment"), Some(naga::Scalar::I32));
    assert_eq!(helper_result(&module, "fixed"), Some(naga::Scalar::I32));
    for (_, function) in module.functions.iter() {
        assert!(function
            .arguments
            .iter()
            .all(|argument| argument.binding.is_none()));
    }
    rejected("__device__ inline int helper(const int value) { return ++value; } __global__ void kernel() {}");
}

#[test]
fn cuda_inline_void_helpers_drop_top_level_const_and_keep_void_return_rules() {
    for header in [
        "__device__ inline const void",
        "void const __device__ inline",
        "inline void __device__",
    ] {
        let (module, _) = validate(&format!(
            "{header} consume(int value) {{ if (value) return; }}
            inline __device__ void forward(int value) {{ return consume(value); }}
            __global__ void kernel() {{ (forward(3)); }}"
        ));
        assert_eq!(helper_result(&module, "consume"), None);
        assert_eq!(helper_result(&module, "forward"), None);
    }
}

#[test]
fn cuda_inline_cv_void_kernels_remain_void_compute_entry_points() {
    for header in [
        "inline __global__ const void",
        "const inline void __global__",
        "void const __global__ inline",
        "__global__ void inline const",
    ] {
        let source = format!("{header} kernel(int *out) {{ out[0] = 9; }}");
        assert_eq!(execute(&source, &[0]), [9], "{header}");
    }
}

#[test]
fn cuda_inline_kernels_keep_the_extern_c_prefix() {
    for header in [
        "inline __global__ void",
        "__global__ inline void",
        "void inline __global__",
    ] {
        validate(&format!("extern \"C\" {header} kernel() {{}}"));
    }
    for source in [
        "inline extern \"C\" __global__ void kernel() {}",
        "__global__ extern \"C\" inline void kernel() {}",
        "extern \"C\" inline __device__ int helper() { return 1; } __global__ void kernel() {}",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_inline_headers_preserve_kernel_resources_workgroup_size_and_helper_bindings() {
    let source = "using Sample = float;
        using Frozen = const Sample;
        inline __device__ Sample scale(Sample value, Sample factor) { value *= factor; return value; }
        extern \"C\" void inline __global__ kernel(Sample factor, Frozen *input, int offset, Sample *out, unsigned count) {
            unsigned i = threadIdx.x;
            if (i < count) out[i] = scale(input[i], factor) + offset;
        }";
    let module = parse_str(
        source,
        &Options {
            workgroup_size: [4, 2, 1],
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
    assert_eq!(module.entry_points[0].workgroup_size, [4, 2, 1]);
    assert_eq!(module.entry_points[0].stage, naga::ShaderStage::Compute);
    assert!(module.entry_points[0].function.result.is_none());
    assert_eq!(module.global_variables.len(), 5);
    for (binding, (_, global)) in module.global_variables.iter().enumerate() {
        assert_eq!(
            global.binding,
            Some(naga::ResourceBinding {
                group: 0,
                binding: binding as u32
            })
        );
        match binding {
            0 | 2 | 4 => assert_eq!(global.space, naga::AddressSpace::Uniform),
            1 => assert_eq!(
                global.space,
                naga::AddressSpace::Storage {
                    access: naga::StorageAccess::LOAD
                }
            ),
            3 => assert_eq!(
                global.space,
                naga::AddressSpace::Storage {
                    access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE
                }
            ),
            _ => unreachable!(),
        }
    }
    assert!(module.functions.iter().all(|(_, function)| function
        .arguments
        .iter()
        .all(|argument| argument.binding.is_none())));
}

#[test]
fn cuda_inline_function_scopes_do_not_mask_later_global_aliases() {
    let (module, _) = validate(
        "using Element = int;
        int inline __device__ first(int Element) { using Local = float; return Element; }
        Element __device__ inline second(Element value) { return first(value); }
        using KernelElement = Element;
        inline __global__ void kernel(KernelElement *out) { out[0] = second(3); }
        using After = Element;",
    );
    assert_eq!(helper_result(&module, "second"), Some(naga::Scalar::I32));
    rejected("__device__ inline int first() { using Local = int; return 1; } Local inline __device__ second() { return 2; } __global__ void kernel() {}");
    rejected("using Element = int; __device__ inline int helper(int Element, Element other) { return other; } __global__ void kernel() {}");
}

#[test]
fn cuda_function_header_type_names_after_a_defining_type_are_declarator_names() {
    for source in [
        "using Element = int; __device__ inline int Element(int value) { return value; } __global__ void kernel() {}",
        "using Element = int; inline __global__ void Element() {}",
    ] {
        rejected_at(source, "Element", 1);
    }
}

#[test]
fn cuda_duplicate_inline_and_explicit_const_report_the_repeated_keyword() {
    for source in [
        "inline inline __device__ int helper() { return 1; } __global__ void kernel() {}",
        "inline __device__ int inline helper() { return 1; } __global__ void kernel() {}",
        "inline __global__ void inline kernel() {}",
    ] {
        rejected_at(source, "inline", 1);
    }
    for source in [
        "/* 漢 */ const inline __device__ int const helper() { return 1; } __global__ void kernel() {}",
        "inline __global__ const void const kernel() {}",
        "using Frozen = const int; const __device__ inline Frozen const helper() { return 1; } __global__ void kernel() {}",
    ] {
        let occurrence = source.match_indices("const").count() - 1;
        rejected_at(source, "const", occurrence);
    }
}

#[test]
fn cuda_function_headers_reject_missing_duplicate_or_conflicting_execution_spaces() {
    for (source, token, occurrence) in [
        (
            "__device__ inline int __device__ helper() { return 1; } __global__ void kernel() {}",
            "__device__",
            1,
        ),
        (
            "inline __global__ void __global__ kernel() {}",
            "__global__",
            1,
        ),
        (
            "inline __global__ __device__ void kernel() {}",
            "__device__",
            0,
        ),
        (
            "__device__ inline __global__ void kernel() {}",
            "__global__",
            0,
        ),
    ] {
        rejected_at(source, token, occurrence);
    }
    for source in [
        "inline int helper() { return 1; } __global__ void kernel() {}",
        "inline void kernel() {}",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_function_headers_reject_incompatible_or_repeated_return_types() {
    for header in [
        "int int",
        "unsigned unsigned",
        "int float",
        "unsigned float",
        "bool int",
        "void int",
        "float unsigned",
        "int void",
    ] {
        rejected(&format!(
            "__device__ inline {header} helper() {{ return 1; }} __global__ void kernel() {{}}"
        ));
    }
    rejected("__global__ inline int kernel() { return 1; }");
    rejected("__device__ inline helper() { return 1; } __global__ void kernel() {}");
}

#[test]
fn cuda_inline_is_not_accepted_in_local_parameter_type_id_or_trailing_positions() {
    for source in [
        "__global__ void kernel() { inline int value = 1; }",
        "__global__ void kernel() { for (inline int i = 0; false;) {} }",
        "__device__ inline int helper(inline int value) { return value; } __global__ void kernel() {}",
        "__global__ inline void kernel(int inline value) {}",
        "__global__ void kernel() { int value = (inline int)1; }",
        "__global__ void kernel() { int value = static_cast<int inline>(1); }",
        "typedef inline int Element; __global__ void kernel() {}",
        "typedef int inline Element; __global__ void kernel() {}",
        "using Element = inline int; __global__ void kernel() {}",
        "__device__ int helper(int value) inline { return value; } __global__ void kernel() {}",
        "__device__ int helper inline(int value) { return value; } __global__ void kernel() {}",
        "__global__ void kernel() inline {}",
        "__global__ void kernel() { return; inline int value = 1; }",
    ] { rejected(source); }
}

#[test]
fn cuda_inline_dual_execution_helpers_are_ordinary_functions() {
    for source in [
        "__host__ inline __device__ int helper() { return 1; } __global__ void kernel() {}",
        "__device__ inline __host__ int helper() { return 1; } __global__ void kernel() {}",
    ] {
        let (module, _) = validate(source);
        assert_eq!(module.functions.len(), 1);
        assert_eq!(helper_result(&module, "helper"), Some(naga::Scalar::I32));
        assert_eq!(module.entry_points.len(), 1);
    }
}

#[test]
fn cuda_inline_accepts_static_and_cuda_inlining_specifiers() {
    for source in [
        "static inline __device__ int helper() { return 1; } __global__ void kernel() {}",
        "__device__ inline static int helper() { return 1; } __global__ void kernel() {}",
        "__device__ __forceinline__ int helper() { return 1; } __global__ void kernel() {}",
        "__device__ __noinline__ int helper() { return 1; } __global__ void kernel() {}",
    ] {
        let (module, _) = validate(source);
        assert_eq!(module.functions.len(), 1);
        assert_eq!(helper_result(&module, "helper"), Some(naga::Scalar::I32));
    }
}

#[test]
fn cuda_inline_does_not_enable_other_function_directives_or_signatures() {
    for source in [
        "__device__ inline __forceinline__ int helper() { return 1; } __global__ void kernel() {}",
        "__device__ __inline__ int helper() { return 1; } __global__ void kernel() {}",
        "__device__ inline int helper(int value); __global__ void kernel() {}",
        "__device__ inline int first() { return later(); } __device__ inline int later() { return 1; } __global__ void kernel() {}",
        "__device__ inline int helper(int value) { return helper(value); } __global__ void kernel() {}",
        "__device__ inline int helper(int value) { return value; } __device__ inline int helper(int other) { return other; } __global__ void kernel() {}",
        "__device__ inline int helper(int *value) { return value[0]; } __global__ void kernel() {}",
        "__device__ inline int *helper(int value) { return value; } __global__ void kernel() {}",
        "__device__ inline int helper() { return threadIdx.x; } __global__ void kernel() {}",
        "__device__ inline void helper() { __shared__ int value; } __global__ void kernel() {}",
    ] { rejected(source); }
}

#[test]
fn cuda_inline_preserves_the_required_sole_kernel_and_complete_source_checks() {
    for source in [
        "__device__ inline int helper() { return 1; }",
        "inline __global__ void first() {} __global__ inline void second() {}",
        "__global__ void kernel() {} __device__ inline int later() { return 1; }",
        "inline __global__ void kernel() {} /* 漢 */ @",
    ] {
        rejected(source);
    }
}

#[test]
#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
fn cuda_inline_functions_and_cv_void_kernel_roundtrip_through_wgsl() {
    let (module, info) = validate(
        "using Frozen = const float;
        const inline Frozen __device__ scale(float value) { return value * 2.0f; }
        unsigned inline int __device__ index(unsigned value) { return ++value; }
        inline void const __device__ consume(bool stop) { if (stop) return; }
        extern \"C\" const inline void __global__ kernel(float *out, unsigned count) {
            unsigned i = threadIdx.x;
            if (i < count) { consume(false); out[i] = scale(index(i)); }
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
}
