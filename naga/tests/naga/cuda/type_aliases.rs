#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const ALIASES: &str = "typedef unsigned int Count, Position;
using Sample = float;
using ReadOnlySample = const Sample;
__device__ Sample scale(Sample value, Count amount) { return value * amount; }
__global__ void kernel(Sample *out, ReadOnlySample *input, Count count) {
    __shared__ Sample values[4];
    Position lane = threadIdx.x;
    if (lane < count) {
        using Local = Sample;
        Local value = scale(input[lane], Count(2));
        values[lane] = value;
        out[lane] = values[lane];
    }
}";

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert!(!error.message.is_empty());
    assert!(source.get(error.span.to_range().unwrap()).is_some());
    error
}

fn local_scalar(module: &naga::Module, name: &str) -> naga::Scalar {
    let local = module.entry_points[0]
        .function
        .local_variables
        .iter()
        .find(|(_, local)| local.name.as_deref() == Some(name))
        .unwrap()
        .1;
    let naga::TypeInner::Scalar(scalar) = module.types[local.ty].inner else {
        panic!("expected scalar local {name}");
    };
    scalar
}

#[test]
fn cuda_both_alias_syntaxes_support_each_existing_scalar_type() {
    for (ty, value, expected) in [
        ("bool", "true", naga::Scalar::BOOL),
        ("int", "3", naga::Scalar::I32),
        ("unsigned int", "3u", naga::Scalar::U32),
        ("float", "3.0f", naga::Scalar::F32),
    ] {
        let (module, _) = validate(&format!(
            "typedef {ty} Element;
            using Measurement = Element;
            __global__ void kernel() {{ Measurement value = {value}; }}"
        ));
        assert_eq!(local_scalar(&module, "value"), expected);
    }
}

#[test]
fn cuda_global_aliases_can_appear_between_helpers_and_after_the_kernel() {
    let (module, _) = validate(
        "typedef int Whole;
        __device__ Whole first(Whole value) { return value + 1; }
        using Measure = float;
        __device__ Measure second(Measure value) { return value * 2.0f; }
        typedef unsigned Count;
        __global__ void kernel(Whole *out, Count count) { out[0] = first(count) + second(count); }
        using Later = Measure;
        typedef Later Last;",
    );
    assert_eq!(module.functions.len(), 2);
    assert_eq!(module.entry_points.len(), 1);
}

#[test]
fn cuda_alias_parameters_preserve_bindings_access_and_underlying_abi() {
    let (module, _) = validate(
        "using Element = float;
        typedef const Element Frozen;
        typedef unsigned int Count;
        __global__ void kernel(Element *out, Frozen *input, Count count) {
            if (threadIdx.x < count) out[threadIdx.x] = input[threadIdx.x];
        }",
    );
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
    let globals: Vec<_> = module
        .global_variables
        .iter()
        .map(|(_, global)| global)
        .collect();
    assert_eq!(
        globals[0].space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE
        }
    );
    assert_eq!(
        globals[1].space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD
        }
    );
    assert_eq!(globals[2].space, naga::AddressSpace::Uniform);
    let naga::TypeInner::Struct { ref members, span } = module.types[globals[2].ty].inner else {
        panic!("scalar parameter ABI")
    };
    assert_eq!(span, 16);
    assert_eq!(members[0].offset, 0);
    assert_eq!(
        module.types[members[0].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::U32)
    );
}

#[test]
fn cuda_aliases_work_in_device_results_and_const_parameter_copies() {
    let (module, _) = validate(
        "typedef int Whole;
        using Frozen = const Whole;
        __device__ Whole advance(Whole value) { return ++value; }
        __device__ Frozen fixed(Frozen value) { return value; }
        __global__ void kernel(Whole *out) { out[0] = advance(fixed(3)); }",
    );
    for (_, function) in module.functions.iter() {
        assert_eq!(
            module.types[function.arguments[0].ty].inner,
            naga::TypeInner::Scalar(naga::Scalar::I32)
        );
        assert_eq!(
            module.types[function.result.as_ref().unwrap().ty].inner,
            naga::TypeInner::Scalar(naga::Scalar::I32)
        );
    }
    rejected(
        "using Frozen = const int;
        __device__ int bad(Frozen value) { return ++value; }
        __global__ void kernel() {}",
    );
}

#[test]
fn cuda_inherited_const_is_preserved_and_redundant_qualification_is_ignored() {
    for body in ["value = 2;", "value += 1;", "++value;", "value--;"] {
        rejected(&format!(
            "typedef const int Frozen;
            __global__ void kernel() {{ Frozen value = 1; {body} }}"
        ));
    }
    rejected("using Frozen = const int; __global__ void kernel(Frozen *out) { out[0] = 1; }");
}

#[test]
fn cuda_aliases_do_not_allow_repeated_explicit_const_qualifiers() {
    for ty in [
        "const Frozen const",
        "const const Frozen",
        "Frozen const const",
    ] {
        rejected(&format!(
            "using Frozen = const int; __global__ void kernel() {{ {ty} value = 1; }}"
        ));
    }
}

#[test]
fn cuda_alias_casts_use_all_existing_scalar_cast_forms() {
    rejected("using Whole = int; __global__ void kernel() { Whole(1) = 2; }");
}

#[test]
fn cuda_alias_expression_contexts_keep_functional_casts_unambiguous() {
    validate(
        "using Whole = int;
        __global__ void kernel() { int count = 0; for (; count < 1; Whole(count)) ++count; }",
    );
}

#[test]
fn cuda_alias_statement_lookahead_does_not_turn_declarations_into_casts() {
    for statement in [
        "Whole(value);",
        "Whole((value));",
        "Whole(out[1]);",
        "int(value);",
    ] {
        rejected(&format!(
            "using Whole = int;
            __global__ void kernel(int *out) {{ int value = 3; {statement} }}"
        ));
    }
}

#[test]
fn cuda_alias_parameter_declarators_are_distinguished_from_cast_arguments() {
    validate(
        "using Whole = int;
        typedef float Weight;
        __device__ int helper(int value) { return value; }
        __global__ void kernel() {
            int value = 3;
            Whole(helper(Whole(value + 1)));
            Whole(helper(Weight(value + 1)));
        }",
    );
    for statement in [
        "Whole(helper(Whole(value)));",
        "Whole(helper(Weight(value)));",
    ] {
        rejected(&format!(
            "using Whole = int;
            typedef float Weight;
            __device__ int helper(int value) {{ return value; }}
            __global__ void kernel() {{ int value = 3; {statement} }}"
        ));
    }
}

#[test]
fn cuda_local_values_can_hide_global_aliases() {
    validate(
        "using Element = int;
        __device__ int identity(int Element) { return Element; }
        __global__ void kernel(int *out) { out[0] = identity(3); }",
    );
}

#[test]
fn cuda_alias_declaration_locus_preserves_outer_type_lookup() {
    rejected(
        "using Element = int;
        __global__ void kernel() { int Element = Element(1); }",
    );
}

#[test]
fn cuda_hidden_aliases_are_not_rediscovered_in_type_positions() {
    for statement in [
        "Element other = 2;",
        "int other = static_cast<Element>(2);",
        "int other = Element(2);",
    ] {
        rejected(&format!(
            "using Element = int;
            __global__ void kernel() {{ int Element = 1; {statement} }}"
        ));
    }
    rejected(
        "using Element = int;
        __device__ int bad(int Element, Element other) { return other; }
        __global__ void kernel() {}",
    );
}

#[test]
fn cuda_alias_redeclarations_reject_changed_types_or_const_qualification() {
    for declarations in [
        "typedef int Element; using Element = float;",
        "using Element = int; typedef unsigned Element;",
        "using Element = const int; typedef int Element;",
        "typedef int Element; typedef const int Element;",
    ] {
        rejected(&format!("{declarations} __global__ void kernel() {{}}"));
        rejected(&format!("__global__ void kernel() {{ {declarations} }}"));
    }
}

#[test]
fn cuda_aliases_reject_same_scope_value_function_and_kernel_collisions() {
    for source in [
        "__global__ void kernel() { int Element = 1; using Element = int; }",
        "__global__ void kernel() { typedef int Element; int Element = 1; }",
        "using helper = int; __device__ int helper(int value) { return value; } __global__ void kernel() {}",
        "__device__ int helper(int value) { return value; } typedef int helper; __global__ void kernel() {}",
        "using kernel = int; __global__ void kernel() {}",
        "__global__ void kernel() {} typedef int kernel;",
        "__global__ void kernel(int Element) { typedef int Element; }",
    ] { rejected(source); }
}

#[test]
fn cuda_aliases_support_shared_scalar_and_array_element_types() {
    let (module, _) = validate(
        "using Element = unsigned int;
        __global__ void kernel(Element *out) {
            typedef Element Shared;
            __shared__ Shared scalar;
            __shared__ Shared values[4];
            scalar = 3u; values[threadIdx.x] = scalar; out[threadIdx.x] = values[threadIdx.x];
        }",
    );
    let shared: Vec<_> = module
        .global_variables
        .iter()
        .filter_map(|(_, global)| (global.space == naga::AddressSpace::WorkGroup).then_some(global))
        .collect();
    assert_eq!(shared.len(), 2);
    assert_eq!(
        module.types[shared[0].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::U32)
    );
    let naga::TypeInner::Array { base, .. } = module.types[shared[1].ty].inner else {
        panic!("shared array")
    };
    assert_eq!(
        module.types[base].inner,
        naga::TypeInner::Scalar(naga::Scalar::U32)
    );
}

#[test]
fn cuda_alias_for_initializers_have_loop_scope_and_restore_outer_types() {
    for declaration in ["typedef int Counter, Position;", "using Counter = int;"] {
        let (module, _) = validate(&format!("using Counter = float;
            __global__ void kernel(int *out) {{
                int count = 0;
                for ({declaration} count < 3; ++count) {{ Counter inside = count; out[count] = inside; }}
                Counter after = 1.5f; out[3] = after * 2.0f;
            }}"));
        assert_eq!(local_scalar(&module, "inside"), naga::Scalar::I32);
        assert_eq!(local_scalar(&module, "after"), naga::Scalar::F32);
    }
    rejected("__global__ void kernel() { int count = 0; for (typedef int Counter; count < 1; ++count) {} Counter leaked = 1; }");
}

#[test]
fn cuda_block_and_function_aliases_do_not_leak() {
    for source in [
        "__global__ void kernel() { { using Local = int; } Local value = 1; }",
        "__device__ int helper() { typedef int Local; return 1; } __global__ void kernel() { Local value = 1; }",
        "__global__ void kernel() { using Local = int; } typedef Local Leaked;",
        "__global__ void kernel() { if (true) typedef int Local; Local value = 1; }",
    ] { rejected(source); }
}

#[test]
fn cuda_aliases_preserve_bool_resource_and_shared_storage_restrictions() {
    for source in [
        "using Flag = bool; __global__ void kernel(Flag value) {}",
        "typedef bool Flag; __global__ void kernel(Flag *out) {}",
        "using Flag = bool; __global__ void kernel() { __shared__ Flag flags[4]; }",
        "using Element = int; __device__ int helper(Element *value) { return 1; } __global__ void kernel() {}",
    ] { rejected(source); }
}

#[test]
fn cuda_aliases_reject_unsupported_types_declarators_and_malformed_source() {
    for declaration in [
        "typedef void Nothing;",
        "using Nothing = void;",
        "typedef double Wide;",
        "using Pointer = int *;",
        "typedef int *Pointer;",
        "typedef int Array[4];",
        "using Array = int[4];",
        "typedef int &Reference;",
        "typedef int Function(int);",
        "template<typename T> using Element = T;",
        "using Element = int, Other = float;",
        "typedef int Element = 1;",
        "typedef int;",
        "using Element int;",
        "using Element = ;",
    ] {
        rejected(&format!("{declaration} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_alias_errors_point_to_the_unknown_type_name() {
    for prefix in ["typedef Missing Element;", "using Element = Missing;"] {
        let source = format!("{prefix} __global__ void kernel() {{}}");
        let error = rejected(&source);
        assert_eq!(&source[error.span.to_range().unwrap()], "Missing");
    }
    rejected("using Element = Element; __global__ void kernel() {}");
    rejected("__global__ void kernel(Later *out) {} typedef int Later;");
}

#[test]
fn cuda_aliases_in_dead_code_still_define_names_and_check_source() {
    validate("__global__ void kernel() { return; typedef int Local; Local value = 1; }");
    rejected("__global__ void kernel() { return; using Bad = Missing; }");
    rejected("__global__ void kernel() {} using Bad = Missing;");
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_type_aliases_roundtrip_through_wgsl() {
    let (module, info) = validate(ALIASES);
    let source =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let module = naga::front::wgsl::parse_str(&source).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
}
