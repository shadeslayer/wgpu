use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/prototypes.cu");

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty() && source.get(range).is_some(), "{error}");
    error
}

fn statements(block: &naga::Block) -> Vec<&naga::Statement> {
    let mut result = Vec::new();
    for statement in block {
        result.push(statement);
        match statement {
            naga::Statement::Block(body) => result.extend(statements(body)),
            naga::Statement::If { accept, reject, .. } => {
                result.extend(statements(accept));
                result.extend(statements(reject));
            }
            naga::Statement::Switch { cases, .. } => {
                for case in cases {
                    result.extend(statements(&case.body));
                }
            }
            naga::Statement::Loop {
                body, continuing, ..
            } => {
                result.extend(statements(body));
                result.extend(statements(continuing));
            }
            _ => {}
        }
    }
    result
}

fn calls(function: &naga::Function) -> Vec<naga::Handle<naga::Function>> {
    statements(&function.body)
        .into_iter()
        .filter_map(|statement| match statement {
            naga::Statement::Call { function, .. } => Some(*function),
            _ => None,
        })
        .collect()
}

fn helper<'a>(module: &'a naga::Module, name: &str) -> &'a naga::Function {
    module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some(name))
        .unwrap()
        .1
}

fn assert_calls_are_ordered(module: &naga::Module) {
    for (caller, function) in module.functions.iter() {
        assert!(calls(function)
            .iter()
            .all(|callee| callee.index() < caller.index()));
        for (_, expression) in function.expressions.iter() {
            if let naga::Expression::CallResult(callee) = expression {
                assert!(callee.index() < caller.index());
            }
        }
    }
    for entry in &module.entry_points {
        assert!(calls(&entry.function)
            .iter()
            .all(|callee| callee.index() < module.functions.len()));
    }
}

#[test]
fn cuda_prototypes_resolve_definitions_before_and_after_the_kernel() {
    for source in [
        "__device__ int increment(int); __device__ int increment(int value) { return value + 1; } __global__ void kernel(int *out) { out[0] = increment(4); }",
        "__device__ int increment(int); __global__ void kernel(int *out) { out[0] = increment(4); } __device__ int increment(int value) { return value + 1; }",
        "__global__ void kernel() {} __device__ int increment(int value) { return value + 1; }",
    ] {
        let (module, _) = validate(source);
        assert_eq!(module.entry_points.len(), 1);
        assert_eq!(module.functions.len(), 1);
        assert_eq!(helper(&module, "increment").arguments.len(), 1);
        assert_calls_are_ordered(&module);
    }
}

#[test]
fn cuda_prototypes_merge_aliases_parameter_names_and_top_level_const() {
    let (module, _) = validate(
        "using Whole = int; using Frozen = const Whole;
        __device__ Whole advance(Whole);
        __device__ int advance(const int prototype_name);
        __device__ Whole advance(Frozen another_name);
        __global__ void kernel(int *out) { out[0] = advance(4); }
        __device__ int advance(Whole value) { return ++value; }",
    );
    assert_eq!(module.functions.len(), 1);
    assert_eq!(
        helper(&module, "advance").arguments[0].name.as_deref(),
        Some("value")
    );
}

#[test]
fn cuda_prototypes_allow_optional_parameter_names_and_both_empty_spellings() {
    for source in [
        "__device__ int pair(int, float second); __global__ void kernel(int *out) { out[0] = pair(2, 3.0f); } __device__ int pair(int first, float second) { return first + int(second); }",
        "__device__ int fixed(int); __global__ void kernel(int *out) { out[0] = fixed(9); } __device__ int fixed(int) { return 3; }",
        "__device__ int fixed(void); __device__ int fixed(); __global__ void kernel(int *out) { out[0] = fixed(); } __device__ int fixed() { return 7; }",
        "__device__ void consume(); __device__ void consume(void); __global__ void kernel() { consume(); } __device__ void consume(void) {}",
    ] {
        validate(source);
    }
}

#[test]
fn cuda_prototypes_preserve_scalar_vector_and_record_value_signatures() {
    for (ty, value) in [
        ("int", "2"),
        ("unsigned", "3u"),
        ("float", "4.0f"),
        ("bool", "true"),
    ] {
        validate(&format!("__device__ {ty} identity({ty}); __global__ void kernel() {{ identity({value}); }} __device__ {ty} identity({ty} value) {{ return value; }}"));
    }
    for scalar in ["int", "uint", "float"] {
        for width in [2, 3, 4] {
            let ty = format!("{scalar}{width}");
            let values = vec!["1"; width].join(", ");
            validate(&format!("__device__ {ty} identity({ty}); __global__ void kernel() {{ identity(make_{ty}({values})); }} __device__ {ty} identity({ty} value) {{ return value; }}"));
        }
    }
    validate("struct Record { int key; int values[2]; }; __device__ Record identity(Record); __global__ void kernel(int *out) { Record value = identity(Record{1, {2, 3}}); out[0] = value.values[1]; } __device__ Record identity(Record value) { return value; }");
}

#[test]
fn cuda_prototypes_use_the_definition_parameter_constness() {
    validate("__device__ int advance(const int); __global__ void kernel(int *out) { int value = 4; out[0] = advance(value); out[1] = value; } __device__ int advance(int value) { return ++value; }");
    validate("__device__ int fixed(int); __global__ void kernel(int *out) { out[0] = fixed(4); } __device__ int fixed(const int value) { return value; }");
    for mutation in ["value = 3", "++value", "value += 1"] {
        rejected(&format!("__device__ int fixed(int); __global__ void kernel() {{ fixed(4); }} __device__ int fixed(const int value) {{ {mutation}; return value; }}"));
    }
}

#[test]
fn cuda_prototypes_allow_redeclarations_after_definitions() {
    let (module, _) = validate(
        "__device__ int identity(int value) { return value; }
        __device__ int identity(const int);
        __global__ void kernel(int *out) { out[0] = identity(8); }
        __device__ int identity(int another_name);",
    );
    assert_eq!(module.functions.len(), 1);
}

#[test]
fn cuda_prototypes_bind_overloads_at_the_original_call_point() {
    let (module, _) = validate(
        "__device__ int choose(float);
        __device__ int earlier(int value) { return choose(value); }
        __device__ int choose(int);
        __global__ void kernel(int *out) { out[0] = earlier(3); out[1] = choose(3); }
        __device__ int choose(int value) { return 20 + value; }
        __device__ int choose(float value) { return 10 + int(value); }",
    );
    let earlier = helper(&module, "earlier");
    let chosen = calls(earlier);
    assert_eq!(chosen.len(), 1);
    assert_eq!(
        module.types[module.functions[chosen[0]].arguments[0].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::F32)
    );
    let kernel_calls = calls(&module.entry_points[0].function);
    let chosen = kernel_calls.last().unwrap();
    assert_eq!(
        module.types[module.functions[*chosen].arguments[0].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::I32)
    );
    assert_calls_are_ordered(&module);
}

#[test]
fn cuda_prototypes_do_not_make_later_names_or_overloads_visible_earlier() {
    for source in [
        "__device__ int caller() { return later(1); } __device__ int later(int); __global__ void kernel() {} __device__ int later(int value) { return value; }",
        "__global__ void kernel() { later(1); } __device__ int later(int value) { return value; }",
        "__device__ int choose(int); __device__ int choose(float); __device__ int caller() { return choose(1u); } __device__ int choose(unsigned); __global__ void kernel() {} __device__ int choose(int value) { return value; } __device__ int choose(float value) { return int(value); } __device__ int choose(unsigned value) { return int(value); }",
        "__device__ Later helper(Later); using Later = int; __global__ void kernel() {}",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_prototypes_keep_nominal_record_overloads_and_alias_identity() {
    let (module, _) = validate(
        "struct First { int value; }; struct Second { int value; }; using Alias = First;
        __device__ int identify(First); __device__ int identify(Second); __device__ int identify(Alias);
        __global__ void kernel(int *out) { out[0] = identify(First{1}); out[1] = identify(Second{2}); }
        __device__ int identify(Alias value) { return value.value + 10; }
        __device__ int identify(Second value) { return value.value + 20; }",
    );
    assert_eq!(module.functions.len(), 2);
    let selected = calls(&module.entry_points[0].function);
    assert_eq!(selected.len(), 2);
    assert_ne!(
        module.functions[selected[0]].arguments[0].ty,
        module.functions[selected[1]].arguments[0].ty
    );
}

#[test]
fn cuda_prototypes_merge_existing_intrinsics_without_emitting_stubs() {
    let (module, _) = validate(
        "__device__ int min(int, int); __device__ unsigned max(unsigned, unsigned);
        __device__ int __clz(int); __device__ int __ffs(int); __device__ int __popc(unsigned);
        __device__ unsigned __brev(unsigned); __device__ unsigned __float_as_uint(float);
        __device__ float __uint_as_float(unsigned); __device__ int2 make_int2(int, int);
        __host__ __device__ float sqrtf(float); __host__ __device__ float sqrtf(float value);
        __global__ void kernel(unsigned *out) {
            int2 value = make_int2(2, 3);
            out[0] = min(value.x, value.y) + max(1u, 2u);
            out[1] = __clz(1) + __ffs(4) + __popc(3u) + __brev(0u);
            out[2] = __float_as_uint(__uint_as_float(0x3f800000u));
        }",
    );
    assert!(module.functions.is_empty());
}

#[test]
fn cuda_prototypes_ordinary_overloads_coexist_with_intrinsics() {
    let (module, _) = validate(
        "__device__ int fabs(int); __device__ int floorf(int);
        __device__ float fabs(float); __device__ float floorf(float);
        __global__ void kernel(int *out) { out[0] = fabs(-3); out[1] = floorf(3); out[2] = int(fabs(-2.0f)); out[3] = int(floorf(2.5f)); }
        __device__ int fabs(int value) { return 100 - value; }
        __device__ int floorf(int value) { return 200 + value; }",
    );
    assert_eq!(module.functions.len(), 2);
    assert_eq!(calls(&module.entry_points[0].function).len(), 2);
}

#[test]
fn cuda_prototypes_reject_incompatible_result_types() {
    for result in ["float", "unsigned", "bool", "void"] {
        let body = if result == "void" {
            "return;"
        } else {
            "return 1;"
        };
        rejected(&format!("__device__ int helper(int); __device__ {result} helper(int); __global__ void kernel() {{}}"));
        rejected(&format!("__device__ int helper(int); __global__ void kernel() {{}} __device__ {result} helper(int value) {{ {body} }}"));
    }
    rejected("struct First { int value; }; struct Second { int value; }; __device__ First helper(int); __global__ void kernel() {} __device__ Second helper(int value) { return Second{value}; }");
}

#[test]
fn cuda_prototypes_reject_second_definitions_despite_compatible_declarations() {
    for middle in ["", "__device__ int helper(const int);"] {
        rejected(&format!("__device__ int helper(int); __device__ int helper(int value) {{ return value; }} {middle} __global__ void kernel() {{}} __device__ int helper(int another) {{ return another; }}"));
    }
}

#[test]
fn cuda_prototypes_preserve_static_linkage_and_per_header_inline_rules() {
    for prototype in [
        "static __device__ int",
        "inline static __device__ int",
        "static __host__ __device__ __forceinline__ int",
    ] {
        let definition = if prototype.contains("__host__") {
            "__host__ __device__ __forceinline__ int"
        } else {
            "__device__ int"
        };
        validate(&format!("{prototype} helper(int); __global__ void kernel(int *out) {{ out[0] = helper(2); }} {definition} helper(int value) {{ return value; }}"));
    }
    rejected("__device__ int helper(int); static __device__ int helper(int); __global__ void kernel() {}");
    rejected("__device__ int helper(int); __global__ void kernel() {} static __device__ int helper(int value) { return value; }");
    rejected("inline __forceinline__ __device__ int helper(int); __global__ void kernel() {}");
}

#[test]
fn cuda_prototypes_require_consistent_source_execution_spaces() {
    for header in ["__device__", "__host__ __device__"] {
        validate(&format!("{header} int helper(int); {header} int helper(int other); __global__ void kernel() {{ helper(2); }} {header} int helper(int value) {{ return value; }}"));
    }
    for (first, second) in [
        ("__device__", "__host__ __device__"),
        ("__host__ __device__", "__device__"),
    ] {
        rejected(&format!(
            "{first} int helper(int); {second} int helper(int); __global__ void kernel() {{}}"
        ));
        rejected(&format!("{first} int helper(int); __global__ void kernel() {{}} {second} int helper(int value) {{ return value; }}"));
    }
}

#[test]
fn cuda_prototypes_reject_conflicting_intrinsic_declarations_and_definitions() {
    for declaration in [
        "__device__ unsigned __clz(int);",
        "__device__ void sinf(float);",
        "__device__ int make_int2(int, int);",
    ] {
        rejected(&format!("{declaration} __global__ void kernel() {{}}"));
    }
    for body in [
        "__device__ int min(int left, int right) { return left; }",
        "__device__ float sinf(float value) { return value; }",
    ] {
        rejected(&format!("__device__ int min(int, int); __device__ float sinf(float); __global__ void kernel() {{}} {body}"));
    }
}

#[test]
fn cuda_prototypes_reject_called_missing_definitions_before_dead_code_removal() {
    for caller in [
        "__global__ void kernel() { missing(1); }",
        "__global__ void kernel() { return; missing(1); }",
        "__device__ int unused() { return missing(1); } __global__ void kernel() {}",
        "__device__ int unused() { return 3; { missing(1); } } __global__ void kernel() {}",
        "__global__ void kernel(int *out) { out[0] = false ? missing(1) : 4; }",
    ] {
        rejected(&format!("__device__ int missing(int); {caller}"));
    }
}

#[test]
fn cuda_prototypes_leave_unused_unresolved_declarations_out_of_ir() {
    let (module, _) = validate(
        "struct Record { int value; }; __device__ int missing(int); __device__ Record missing(Record);
        __global__ void kernel(int *out) { out[0] = 4; }
        __device__ void unused(float);",
    );
    assert!(module.functions.is_empty());
    assert!(module
        .global_variables
        .iter()
        .all(|(_, global)| matches!(global.space, naga::AddressSpace::Storage { .. })));
}

#[test]
fn cuda_prototypes_reject_direct_recursion_but_allow_other_overloads() {
    for source in [
        "__device__ int recurse(int); __global__ void kernel() {} __device__ int recurse(int value) { return recurse(value); }",
        "__device__ int recurse(int); __device__ int recurse(int value) { return 2; recurse(value); } __global__ void kernel() {}",
    ] {
        rejected(source);
    }
    validate("__device__ int pick(float); __device__ int pick(int value) { return pick(float(value)); } __global__ void kernel(int *out) { out[0] = pick(2); } __device__ int pick(float value) { return int(value); }");
}

#[test]
fn cuda_prototypes_detect_mutual_cycles_in_unused_and_dead_callers() {
    for (first_body, second_body) in [
        ("return second(value);", "return first(value);"),
        ("return 1; second(value);", "return 2; first(value);"),
        (
            "return value ? second(value) : 1;",
            "return value ? first(value) : 2;",
        ),
    ] {
        rejected(&format!("__device__ int first(int); __device__ int second(int); __global__ void kernel() {{}} __device__ int first(int value) {{ {first_body} }} __device__ int second(int value) {{ {second_body} }}"));
    }
}

#[test]
fn cuda_prototypes_order_long_reverse_definition_chains_without_stubs() {
    let mut source = String::new();
    for index in 0..128 {
        source.push_str(&format!("__device__ int helper{index}(int);\n"));
    }
    source.push_str("__global__ void kernel(int *out) { out[0] = helper0(1); }\n");
    for index in (0..128).rev() {
        let expression = if index == 127 {
            String::from("value")
        } else {
            format!("helper{}(value + 1)", index + 1)
        };
        source.push_str(&format!(
            "__device__ int helper{index}(int value) {{ return {expression}; }}\n"
        ));
    }
    let (module, _) = validate(&source);
    assert_eq!(module.functions.len(), 128);
    assert_calls_are_ordered(&module);
}

#[test]
fn cuda_prototypes_keep_nested_calls_discarded_void_and_jump_scopes_valid() {
    validate(
        "__device__ int step(int); __device__ void consume(int);
        __global__ void kernel(int *out) {
            consume(step(1));
            for (int value = step(0); value < 3; value = step(value)) {
                if (value == 1) continue;
                switch (value) { case 0: consume(value); break; default: out[value] = step(value); break; }
            }
            true ? consume(step(2)) : consume(step(3));
            return; { consume(step(4)); }
        }
        __device__ void consume(int value) { if (value) return; }
        __device__ int step(int value) { return value + 1; }",
    );
}

#[test]
fn cuda_prototypes_initialize_late_helper_builtin_globals_before_calls() {
    let (module, _) = validate(
        "__device__ uint3 coordinates();
        __global__ void kernel(unsigned *out) { uint3 value = coordinates(); out[0] = value.x + value.y + value.z; }
        __device__ uint3 coordinates() { return make_uint3(
            threadIdx.x + blockIdx.x + gridDim.x + blockDim.x,
            threadIdx.y + blockIdx.y + gridDim.y + blockDim.y,
            threadIdx.z + blockIdx.z + gridDim.z + blockDim.z); }",
    );
    let kernel = &module.entry_points[0].function;
    let private: Vec<_> = module
        .global_variables
        .iter()
        .filter_map(|(handle, global)| {
            (global.space == naga::AddressSpace::Private).then_some(handle)
        })
        .collect();
    assert_eq!(private.len(), 3);
    let body = statements(&kernel.body);
    let first_call = body
        .iter()
        .position(|statement| matches!(statement, naga::Statement::Call { .. }))
        .unwrap();
    for global in private {
        assert!(body[..first_call].iter().any(|statement| matches!(statement, naga::Statement::Store { pointer, .. } if matches!(kernel.expressions[*pointer], naga::Expression::GlobalVariable(handle) if handle == global))));
    }
    for builtin in [
        naga::BuiltIn::LocalInvocationId,
        naga::BuiltIn::WorkGroupId,
        naga::BuiltIn::NumWorkGroups,
    ] {
        assert!(kernel
            .arguments
            .iter()
            .any(|argument| argument.binding == Some(naga::Binding::BuiltIn(builtin))));
    }
}

#[test]
fn cuda_prototypes_remap_source_and_generated_helper_calls_together() {
    let (module, _) = validate(
        "__device__ float after(float); __device__ float before(float value) { return after(value); }
        __global__ void kernel(float *out) { out[0] = before(0.0f); }
        __device__ float after(float value) { return sqrtf(4.0f) + sinf(value) + cosf(value); }",
    );
    assert_calls_are_ordered(&module);
    assert!(!calls(helper(&module, "before")).is_empty());
    assert!(!calls(helper(&module, "after")).is_empty());
    assert!(module
        .functions
        .iter()
        .all(|(_, function)| !function.body.is_empty()));
}

#[test]
fn cuda_prototypes_keep_prototype_parameter_and_local_shadow_scopes_separate() {
    validate("using Value = int; __device__ int helper(int Value); __global__ void kernel() {} __device__ int other(Value value) { return value; }");
    rejected("using Value = int; __device__ int helper(int Value, Value second); __global__ void kernel() {}");
    validate("__device__ int helper(int prototype_name); __global__ void kernel(int *out) { { int helper = 3; out[0] = helper; } out[1] = helper(4); } __device__ int helper(int value) { return value; }");
    for source in [
        "__device__ int helper(int prototype_name); __global__ void kernel(int *out) { out[0] = prototype_name; } __device__ int helper(int value) { return value; }",
        "__device__ int helper(int); __global__ void kernel() { int helper = helper(3); } __device__ int helper(int value) { return value; }",
        "__device__ int helper(int); struct Record { int helper; int values[false ? helper(1) : 4]; }; __global__ void kernel() {} __device__ int helper(int value) { return value; }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_prototypes_reject_malformed_and_unsupported_declarators_with_utf8_spans() {
    validate("/* λ */ __device__ int helper(int &); __global__ void kernel() {}");
    for declaration in [
        "__device__ int helper(int, );",
        "__device__ int helper(void, int);",
        "__device__ int helper(int value = 1);",
        "__device__ int helper(int, ...);",
        "__device__ int *helper();",
        "__device__ double helper(double);",
        "__host__ int helper(int);",
        "extern \"C\" __device__ int helper(int);",
        "__global__ void kernel();",
    ] {
        rejected(&format!(
            "/* λ */ {declaration} __global__ void kernel() {{}}"
        ));
    }
    let source =
        "/* λ */ __device__ int helper(int); __global__ void kernel() { return; absent(3); }";
    let error = rejected(source);
    assert_eq!(&source[error.span.to_range().unwrap()], "absent");
    validate("__device__ int helper(int *); __global__ void kernel() {}");
    validate("__device__ int helper(int values[2]); __global__ void kernel() {}");
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_prototypes_fixture_roundtrips_through_common_wgsl_helpers() {
    let (module, info) = super::helpers::validate_with_options(
        FIXTURE,
        &Options {
            workgroup_size: [2, 1, 1],
        },
    );
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}

#[cfg(spv_out)]
#[cfg_attr(miri, ignore)]
#[test]
fn cuda_prototypes_fixture_emits_valid_spirv_binary() {
    let (module, info) = super::helpers::validate_with_options(
        FIXTURE,
        &Options {
            workgroup_size: [2, 1, 1],
        },
    );
    let options = naga::back::spv::Options {
        lang_version: (1, 1),
        ..Default::default()
    };
    let pipeline = naga::back::spv::PipelineOptions {
        entry_point: "prototypes".to_string(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
    crate::helpers::validate_spirv(&words);
}

#[test]
fn cuda_prototypes_reject_conflicting_inlining_across_redeclarations() {
    for (first, second) in [
        ("__forceinline__", "__noinline__"),
        ("__noinline__", "__forceinline__"),
        ("inline", "__forceinline__"),
        ("__forceinline__", "inline"),
        ("inline", "__noinline__"),
        ("__noinline__", "inline"),
    ] {
        for source in [
            format!("{first} __device__ int helper(int); {second} __device__ int helper(int); __global__ void kernel() {{}}"),
            format!("{first} __device__ int helper(int); __global__ void kernel() {{}} {second} __device__ int helper(int value) {{ return value; }}"),
            format!("{first} __device__ int helper(int value) {{ return value; }} __global__ void kernel() {{}} {second} __device__ int helper(int);"),
            format!("__device__ int helper(int); {first} __device__ int helper(int); __global__ void kernel() {{}} {second} __device__ int helper(int);"),
            format!("__device__ int helper(int); __global__ void kernel() {{}} {first} __device__ int helper(int value) {{ return value; }} {second} __device__ int helper(int);"),
            format!("{first} __device__ float fabsf(float); {second} __device__ float fabsf(float); __global__ void kernel() {{}}"),
            format!("{first} __device__ float fabsf(float); __global__ void kernel() {{}} {second} __device__ float fabsf(float);"),
            format!("__device__ float fabsf(float); {first} __device__ float fabsf(float); __global__ void kernel() {{}} {second} __device__ float fabsf(float);"),
        ] {
            let error = rejected(&format!("/* λ */ {source}"));
            assert!(error.message.contains("inlin"), "{error}");
        }
    }
    for modifier in ["inline", "__forceinline__", "__noinline__"] {
        validate(&format!(
            "__device__ int helper(int); {modifier} __device__ int helper(int); __device__ int helper(int);

            __global__ void kernel(int *out) {{ out[0] = helper(3); }}

            {modifier} __device__ int helper(int value) {{ return value; }} __device__ int helper(int);"
        ));
        validate(&format!(
            "__device__ float fabsf(float); {modifier} __device__ float fabsf(float); __device__ float fabsf(float);

            __global__ void kernel(float *out) {{ out[0] = fabsf(-2.0f); }}

            {modifier} __device__ float fabsf(float);"
        ));
    }
    validate(
        "__forceinline__ __device__ int helper(int); __noinline__ __device__ int helper(float);

        __forceinline__ __device__ int min(int, int); __noinline__ __device__ float min(float, float);

        __global__ void kernel(int *out) { out[0] = helper(2) + helper(2.0f) + min(1, 2); }

        __forceinline__ __device__ int helper(int value) { return value; }

        __noinline__ __device__ int helper(float value) { return int(value); }",
    );
}
