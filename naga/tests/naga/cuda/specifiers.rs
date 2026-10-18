#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert!(!error.message.is_empty(), "{source}");
    assert!(source.get(error.span.to_range().unwrap()).is_some());
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

fn helper<'a>(module: &'a naga::Module, name: &str) -> &'a naga::Function {
    module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some(name))
        .unwrap()
        .1
}

#[test]
fn cuda_static_helpers_accept_each_header_order() {
    for header in [
        "static __device__ int",
        "static int __device__",
        "__device__ static int",
        "__device__ int static",
        "int static __device__",
        "int __device__ static",
    ] {
        let (module, _) = validate(&format!(
            "{header} advance(int value) {{ return value + 1; }}
            __global__ void kernel(int *out) {{ out[0] = advance(3); }}"
        ));
        assert_eq!(module.functions.len(), 1);
        assert_eq!(helper(&module, "advance").arguments.len(), 1);
    }
}

#[test]
fn cuda_static_and_standard_inline_can_coexist() {
    for header in [
        "static inline __device__ int",
        "__device__ inline static int",
        "int static __host__ inline __device__",
        "inline __device__ __host__ int static",
    ] {
        validate(&format!(
            "{header} helper(int value) {{ return value; }}
            static inline __global__ void kernel(int *out) {{ out[0] = helper(3); }}"
        ));
    }
}

#[test]
fn cuda_inlining_hints_preserve_all_scalar_result_types() {
    for hint in ["__forceinline__", "__noinline__"] {
        for (ty, value, scalar) in [
            ("bool", "true", naga::Scalar::BOOL),
            ("int", "3", naga::Scalar::I32),
            ("unsigned", "3u", naga::Scalar::U32),
            ("float", "3.5f", naga::Scalar::F32),
        ] {
            let (module, _) = validate(&format!(
                "{hint} __device__ {ty} helper({ty} value) {{ return value; }}
                __global__ void kernel(int *out) {{ out[0] = helper({value}); }}"
            ));
            let result = helper(&module, "helper").result.as_ref().unwrap();
            assert_eq!(
                module.types[result.ty].inner,
                naga::TypeInner::Scalar(scalar)
            );
            assert_eq!(result.binding, None);
        }
    }
}

#[test]
fn cuda_inlining_hints_allow_interleaved_headers_and_unsigned_int() {
    for header in [
        "__forceinline__ int __device__",
        "int __device__ __noinline__",
        "__host__ static int __forceinline__ __device__",
        "__device__ __noinline__ static int __host__",
        "unsigned __device__ __forceinline__ int",
        "int __noinline__ unsigned __host__ __device__ static",
    ] {
        let (module, _) = validate(&format!(
            "{header} helper(int value) {{ return value + 1; }}
            __global__ void kernel(int *out) {{ out[0] = helper(3); }}"
        ));
        let expected = if header.contains("unsigned") {
            naga::Scalar::U32
        } else {
            naga::Scalar::I32
        };
        let result = helper(&module, "helper").result.as_ref().unwrap();
        assert_eq!(
            module.types[result.ty].inner,
            naga::TypeInner::Scalar(expected)
        );
    }
}

#[test]
fn cuda_function_specifiers_preserve_const_aliases_and_mutable_parameter_copies() {
    let (module, _) = validate(
        "using Whole = int; using Frozen = const Whole;
        const static Frozen __forceinline__ __device__ advance(Whole value) { return ++value; }
        __host__ Frozen __device__ __noinline__ static fixed(const Whole value) { return value; }
        static __global__ void kernel(int *out) {
            int value = 3; out[0] = advance(fixed(value)); out[1] = value;
        }",
    );
    for name in ["advance", "fixed"] {
        let function = helper(&module, name);
        let result = function.result.as_ref().unwrap();
        assert_eq!(
            module.types[result.ty].inner,
            naga::TypeInner::Scalar(naga::Scalar::I32)
        );
        assert!(function
            .arguments
            .iter()
            .all(|argument| argument.binding.is_none()));
        assert!(function
            .expressions
            .iter()
            .all(|(_, expression)| !matches!(expression, naga::Expression::GlobalVariable(_))));
    }
    rejected("__device__ __forceinline__ int helper(const int value) { return ++value; } __global__ void kernel() {}");
}

#[test]
fn cuda_function_specifiers_support_cv_void_and_dual_helpers() {
    for header in [
        "static __device__ const void",
        "__forceinline__ void const __host__ __device__ static",
        "const static __device__ __noinline__ void __host__",
    ] {
        let (module, _) = validate(&format!(
            "{header} consume(int value) {{ if (value) return; }}
            static __device__ __noinline__ void forward(int value) {{ return consume(value); }}
            static __global__ const void kernel() {{ forward(3); }}"
        ));
        assert!(helper(&module, "consume").result.is_none());
        assert!(helper(&module, "forward").result.is_none());
        assert!(module.entry_points[0].function.result.is_none());
    }
}

#[test]
fn cuda_inlining_hints_keep_ordinary_functions_and_calls() {
    let (module, _) = validate(
        "static __device__ __forceinline__ int advance(int value) { return ++value; }
        static __host__ __device__ __noinline__ int twice(int value) { return advance(advance(value)); }
        static __global__ void kernel(int *out) { out[0] = twice(3); advance(4); }",
    );
    let functions: Vec<_> = module.functions.iter().map(|(handle, _)| handle).collect();
    assert_eq!(functions.len(), 2);
    let targets = |function: &naga::Function| {
        function
            .body
            .iter()
            .filter_map(|statement| match statement {
                naga::Statement::Call { function, .. } => Some(*function),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        targets(helper(&module, "twice")),
        [functions[0], functions[0]]
    );
    assert_eq!(
        targets(&module.entry_points[0].function),
        [functions[1], functions[0]]
    );
}

#[test]
fn cuda_static_preserves_kernel_resource_bindings_and_launch_dimensions() {
    let source =
        "static __device__ __noinline__ int add(int value, int offset) { return value + offset; }
        inline static __global__ void kernel(int *out, const int *input, int offset) {
            unsigned i = threadIdx.x; out[i] = add(input[i], offset) + blockDim.y;
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
    assert_eq!(module.global_variables.len(), 3);
    for (binding, (_, global)) in module.global_variables.iter().enumerate() {
        assert_eq!(
            global.binding,
            Some(naga::ResourceBinding {
                group: 0,
                binding: binding as u32
            })
        );
    }
    assert_eq!(
        module
            .global_variables
            .iter()
            .map(|(_, global)| global.name.as_deref().unwrap())
            .collect::<Vec<_>>(),
        ["out", "input", "offset"]
    );
    assert!(helper(&module, "add")
        .arguments
        .iter()
        .all(|argument| argument.binding.is_none()));
}

#[test]
fn cuda_function_specifiers_keep_function_scopes_and_name_collisions() {
    validate(
        "using Whole = int;
        static __device__ __forceinline__ Whole first(Whole value) { int local = value + 1; return local; }
        static __device__ __noinline__ Whole second(Whole value) { int local = first(value); return local; }
        static __global__ void kernel(int *out) { Whole local = 3; out[0] = second(local); }",
    );
    for body in [
        "int helper = 1; helper(2);",
        "using helper = int; helper = 2;",
    ] {
        rejected(&format!(
            "static __device__ __forceinline__ int helper(int value) {{ return value; }}
            __global__ void kernel() {{ {body} }}"
        ));
    }
    rejected("using helper = int; static __device__ int helper() { return 1; } __global__ void kernel() {}");
}

#[test]
fn cuda_duplicate_static_is_reported_at_the_second_specifier() {
    for source in [
        "static static __device__ int helper() { return 1; } __global__ void kernel() {}",
        "static __device__ int static helper() { return 1; } __global__ void kernel() {}",
        "static inline __global__ void static kernel() {}",
    ] {
        rejected_at(source, "static", 1);
    }
}

#[test]
fn cuda_duplicate_and_conflicting_inlining_hints_report_the_second_keyword() {
    for (first, second) in [
        ("__forceinline__", "__forceinline__"),
        ("__noinline__", "__noinline__"),
        ("__forceinline__", "__noinline__"),
        ("__noinline__", "__forceinline__"),
    ] {
        let source = format!("/* 漢 */ {first} static __host__ int __device__ {second} helper() {{ return 1; }} __global__ void kernel() {{}}");
        rejected_at(&source, second, usize::from(first == second));
    }
}

#[test]
fn cuda_explicit_inline_conflicts_with_each_cuda_hint_in_both_orders() {
    for hint in ["__forceinline__", "__noinline__"] {
        for (first, second) in [("inline", hint), (hint, "inline")] {
            let source = format!("{first} __device__ static int {second} helper() {{ return 1; }} __global__ void kernel() {{}}");
            let start = source.match_indices(second).last().unwrap().0;
            let error = rejected(&source);
            assert_eq!(error.span, naga::Span::from(start..start + second.len()));
        }
    }
}

#[test]
fn cuda_inlining_hints_on_kernels_are_reported_at_the_hint() {
    for hint in ["__forceinline__", "__noinline__"] {
        for header in [
            format!("{hint} static __global__ void"),
            format!("__global__ {hint} static void"),
            format!("void static __global__ {hint}"),
        ] {
            rejected_at(&format!("{header} kernel() {{}}"), hint, 0);
        }
    }
}

#[test]
fn cuda_static_conflicts_with_direct_extern_c_kernel_declarations() {
    validate("extern \"C\" inline __global__ void kernel() {}");
    for header in [
        "static __global__ void",
        "__global__ static void",
        "void inline __global__ static",
    ] {
        rejected_at(&format!("extern \"C\" {header} kernel() {{}}"), "static", 0);
    }
}

#[test]
fn cuda_function_specifiers_remain_unsupported_outside_function_headers() {
    for specifier in ["static", "__forceinline__", "__noinline__"] {
        for source in [
            format!("{specifier} int value = 1; __global__ void kernel() {{}}"),
            format!("__global__ void kernel() {{ {specifier} int value = 1; }}"),
            format!("__global__ void kernel() {{ for ({specifier} int value = 0; false;) {{}} }}"),
            format!("__device__ int helper({specifier} int value) {{ return value; }} __global__ void kernel() {{}}"),
            format!("__global__ void kernel() {{ int value = ({specifier} int)1; }}"),
            format!("typedef {specifier} int Whole; __global__ void kernel() {{}}"),
            format!("using Whole = {specifier} int; __global__ void kernel() {{}}"),
            format!("__device__ int helper() {specifier} {{ return 1; }} __global__ void kernel() {{}}"),
            format!("__global__ void kernel() {{ return; {specifier} int value = 1; }}"),
        ] {
            rejected(&source);
        }
    }
    rejected("__device__ __inline_hint__ int helper() { return 1; } __global__ void kernel() {}");
}

#[test]
fn cuda_recognized_inlining_hints_are_not_available_as_names() {
    for name in ["__forceinline__", "__noinline__"] {
        for source in [
            format!("__global__ void kernel() {{ int {name} = 1; }}"),
            format!("__global__ void kernel(int {name}) {{}}"),
            format!("using {name} = int; __global__ void kernel() {{}}"),
            format!("__global__ void kernel() {{ typedef int {name}; }}"),
            format!("__device__ int {name}(int value) {{ return value; }} __global__ void kernel() {{}}"),
        ] {
            rejected(&source);
        }
    }
}

#[test]
fn cuda_function_specifiers_preserve_unsupported_signatures_and_bodies() {
    for source in [
        "static __host__ int helper() { return 1; } __global__ void kernel() {}",
        "__host__ __forceinline__ int helper() { return 1; } __global__ void kernel() {}",
        "static __device__ int helper(int value); __global__ void kernel() {}",
        "static __device__ int helper(int *value) { return value[0]; } __global__ void kernel() {}",
        "static __device__ int *helper(int value) { return value; } __global__ void kernel() {}",
        "__device__ __forceinline__ int helper(int value) { return helper(value); } __global__ void kernel() {}",
        "static __device__ int helper(int value) { return value; } static __device__ int helper(int other) { return other; } __global__ void kernel() {}",
        "static __device__ void helper() { __shared__ int value; } __global__ void kernel() {}",
        "static __device__ void helper() { __syncthreads(); } __global__ void kernel() {}",
        "static __device__ int helper() { return threadIdx.x; } __global__ void kernel() {}",
    ] {
        rejected(source);
    }
    rejected_at(
        "/* λ */ static __host__ int helper() { return 1; } __global__ void kernel() {}",
        "__host__",
        0,
    );
}

#[test]
fn cuda_function_specifiers_keep_sole_kernel_and_complete_source_checks() {
    for source in [
        "static __device__ int helper() { return 1; }",
        "static __global__ void first() {} static __global__ void second() {}",
        "static __global__ void kernel() {} static __device__ int helper() { return 1; }",
    ] {
        rejected(source);
    }
    rejected_at("static __global__ void kernel() {} /* 漢 */ @", "@", 0);
    validate("static __device__ __noinline__ int helper() { return 1; } __global__ void kernel() { return; { helper(); } }");
}

#[test]
#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
fn cuda_function_specifiers_roundtrip_through_wgsl() {
    let (module, info) = validate(
        "using Frozen = const float;
        static __device__ Frozen __forceinline__ scale(float value) { return value * 2.0f; }
        __host__ static __noinline__ unsigned int __device__ next(unsigned value) { return ++value; }
        static __device__ void consume(int value) { if (value) return; }
        static inline __global__ void kernel(float *out) { consume(1); out[0] = scale(next(3u)); }",
    );
    super::helpers::wgsl_roundtrip(&module, &info, naga::back::wgsl::WriterFlags::empty());
}
