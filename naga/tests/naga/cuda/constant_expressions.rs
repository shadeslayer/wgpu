use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

fn array_count(module: &naga::Module, name: &str) -> u32 {
    let function = &module.entry_points[0].function;
    function
        .local_variables
        .iter()
        .find_map(|(_, local)| {
            if local.name.as_deref() != Some(name) {
                return None;
            }
            match module.types[local.ty].inner {
                naga::TypeInner::Array {
                    size: naga::ArraySize::Constant(count),
                    ..
                } => Some(count.get()),
                _ => None,
            }
        })
        .unwrap()
}

#[test]
fn cuda_constant_expressions_support_scalar_constexpr_variables_and_assignment_conversions() {
    for (name, scalar) in [
        ("bool", naga::Scalar::BOOL),
        ("int", naga::Scalar::I32),
        ("unsigned", naga::Scalar::U32),
        ("float", naga::Scalar::F32),
    ] {
        for value in ["true", "3", "5u", "2.75f"] {
            let (module, _) = validate(&format!("__global__ void kernel(int *out) {{ constexpr {name} frozen = {value}; out[0] = int(frozen); }}"));
            let local = module.entry_points[0]
                .function
                .local_variables
                .iter()
                .find_map(|(_, local)| (local.name.as_deref() == Some("frozen")).then_some(local))
                .unwrap();
            assert_eq!(
                module.types[local.ty].inner,
                naga::TypeInner::Scalar(scalar)
            );
        }
    }
    for declaration in [
        "constexpr const int",
        "const constexpr int",
        "int constexpr",
        "unsigned constexpr int",
    ] {
        validate(&format!("{declaration} Global = 2; __global__ void kernel() {{ {declaration} Local = Global + 1; int values[Local]; }}"));
    }
    rejected("__global__ void kernel() { const constexpr int const value = 2; }");
}

#[test]
fn cuda_constant_expressions_global_constants_preserve_types_and_resource_bindings() {
    let (module, _) = validate(
        "constexpr bool Ready = true; constexpr int Count = 2; constexpr unsigned Mask = 7u; constexpr float Scale = 0.5f;
        __global__ void kernel(const int *input, int factor, int *out) { int values[Count]{}; out[0] = input[0] * factor + int(Ready) + int(Mask) + int(Scale); }",
    );
    for (name, scalar) in [
        ("Ready", naga::Scalar::BOOL),
        ("Count", naga::Scalar::I32),
        ("Mask", naga::Scalar::U32),
        ("Scale", naga::Scalar::F32),
    ] {
        let constant = module
            .constants
            .iter()
            .find_map(|(_, constant)| (constant.name.as_deref() == Some(name)).then_some(constant))
            .unwrap();
        assert_eq!(
            module.types[constant.ty].inner,
            naga::TypeInner::Scalar(scalar)
        );
    }
    assert_eq!(module.global_variables.len(), 3);
    for (index, (_, variable)) in module.global_variables.iter().enumerate() {
        assert_eq!(
            variable.binding,
            Some(naga::ResourceBinding {
                group: 0,
                binding: index as u32
            })
        );
    }
    assert_eq!(array_count(&module, "values"), 2);
}

#[test]
fn cuda_constant_expressions_allow_global_declarations_before_between_and_after_functions() {
    validate(
        "constexpr int First = 2;
        __device__ int first() { return First; }
        constexpr int Second = First + 1;
        __device__ int second() { return first() + Second; }
        __global__ void kernel(int *out) { constexpr int Hidden = 9; out[0] = second(); }
        constexpr int Last = Second + 1;",
    );
    let source =
        "__global__ void kernel() { constexpr int Hidden = 9; } constexpr int Last = Hidden;";
    assert_eq!(&source[rejected(source).span.to_range().unwrap()], "Hidden");
    let source =
        "constexpr int First = Later; constexpr int Later = 2; __global__ void kernel() {}";
    assert_eq!(&source[rejected(source).span.to_range().unwrap()], "Later");
}

#[test]
fn cuda_constant_expressions_preserve_alias_const_and_declarator_scopes() {
    let (module, _) = validate(
        "using Count = const int; constexpr Count Size = 2, Other = Size + 1;
        __global__ void kernel(int *out) {
            constexpr Count first = Other, next = first + 1;
            { constexpr int Size = 5; int local[Size]{}; out[0] = local[0]; }
            int Size[Size]{}; int values[next]{}; out[1] = values[0];
            for (constexpr int Limit = 2; false;) { int scoped[Limit]; }
        }",
    );
    assert_eq!(array_count(&module, "local"), 5);
    assert_eq!(array_count(&module, "Size"), 2);
    assert_eq!(array_count(&module, "values"), 4);
    let source =
        "__global__ void kernel() { for (constexpr int Limit = 2; false;) {} int values[Limit]; }";
    assert_eq!(&source[rejected(source).span.to_range().unwrap()], "Limit");
}

#[test]
fn cuda_constant_expressions_use_const_integral_locals_but_not_runtime_or_const_float_values() {
    let (module, _) = validate(
        "__global__ void kernel() {
            const int Count = 2 + 1; const unsigned More = unsigned(Count) + 1u; const bool Enabled = true;
            int values[Count]; float samples[More]; bool flags[Enabled];
            constexpr float Scale = 2.0f; int explicit_bound[int(Scale)];
        }",
    );
    for (name, count) in [
        ("values", 3),
        ("samples", 4),
        ("flags", 1),
        ("explicit_bound", 2),
    ] {
        assert_eq!(array_count(&module, name), count);
    }
    for body in [
        "int value = 2; constexpr int result = value;",
        "const int value = runtime; constexpr int result = value;",
        "const bool value = runtime; constexpr bool result = value;",
        "const float value = 2.0f; constexpr float result = value;",
        "const float value = 2.0f; int values[int(value)];",
        "const int value = runtime; int values[value];",
    ] {
        rejected(&format!("__global__ void kernel(int runtime) {{ {body} }}"));
    }
    let source = "__global__ void kernel(int runtime) { constexpr int value = runtime; }";
    assert!(rejected(source)
        .message
        .contains("supported scalar constant expression"));
}

#[test]
fn cuda_constant_expressions_track_unary_binary_cast_logical_and_comma_operations() {
    for expression in [
        "+2",
        "-(-2)",
        "~(-3)",
        "1 + 1",
        "6 - 4",
        "2 * 1",
        "8 / 4",
        "5 % 3",
        "6 & 3",
        "2 | 0",
        "3 ^ 1",
        "8 >> 2",
        "1 << 1",
        "int(2u)",
        "(int)2.75f",
        "static_cast<int>(2.75f)",
        "true ? 2 : 3",
        "int(!false) + 1",
        "int(true && true) + 1",
        "int(false || true) + 1",
        "int(1 < 2) + 1",
        "int(2 == 2) + 1",
        "(1, 2)",
    ] {
        let (module, _) = validate(&format!(
            "__global__ void kernel() {{ constexpr int count = {expression}; int values[count]; }}"
        ));
        assert_eq!(array_count(&module, "values"), 2, "{expression}");
    }
}

#[test]
fn cuda_constant_expressions_use_selected_lazy_branches_and_still_typecheck_others() {
    for expression in [
        "true ? 3 : runtime",
        "false ? runtime : 3",
        "true ? 3 : helper()",
        "true ? 3 : (runtime = 8)",
        "true ? 3 : (1 / 0)",
        "int(false && helper()) + 3",
        "int(true || (runtime++)) + 2",
    ] {
        let (module, _) = validate(&format!("__device__ int helper() {{ return 9; }} __global__ void kernel(int runtime) {{ constexpr int count = {expression}; int values[count]; }}"));
        assert_eq!(array_count(&module, "values"), 3, "{expression}");
    }
    for expression in [
        "false ? 3 : runtime",
        "true ? helper() : 3",
        "true && runtime",
        "false || helper()",
    ] {
        rejected(&format!("__device__ int helper() {{ return 9; }} __global__ void kernel(int runtime) {{ constexpr int count = {expression}; }}"));
    }
    let source = "__global__ void kernel() { constexpr int count = true ? 3 : missing; }";
    assert_eq!(
        &source[rejected(source).span.to_range().unwrap()],
        "missing"
    );
}

#[test]
fn cuda_constant_expressions_resolve_pending_self_names_only_in_unselected_branches() {
    validate("constexpr int Global = true ? 3 : Global; __global__ void kernel() { constexpr int local = false ? local : Global; int values[local]; }");
    for source in [
        "constexpr int Global = Global; __global__ void kernel() {}",
        "constexpr int Global = false ? 3 : Global; __global__ void kernel() {}",
        "__global__ void kernel() { constexpr int local = local; }",
        "constexpr int Outer = 2; __global__ void kernel() { constexpr int Outer = Outer; }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_constant_expressions_distinguish_discarded_glvalues_from_value_computations() {
    for expression in [
        "(runtime, 3)",
        "((true ? runtime : other), 3)",
        "(local[0], 3)",
        "((true ? local[0] : input[0]), 3)",
    ] {
        let (module, _) = validate(&format!("__global__ void kernel(const int *input, int runtime, int other) {{ int local[2]{{}}; constexpr int count = {expression}; int values[count]; }}"));
        assert_eq!(array_count(&module, "values"), 3, "{expression}");
    }
    for expression in [
        "(+runtime, 3)",
        "(runtime++, 3)",
        "(runtime = 3, 3)",
        "(helper(), 3)",
        "(3, runtime)",
        "((true ? runtime : 0), 3)",
        "(local[runtime], 3)",
        "(input[0], 3)",
    ] {
        rejected(&format!("__device__ int helper() {{ return 1; }} __global__ void kernel(const int *input, int runtime) {{ int local[2]{{}}; constexpr int count = {expression}; }}"));
    }
}

#[test]
fn cuda_constant_expressions_check_discarded_local_array_addresses() {
    for (declaration, field) in [("int local[2]{};", ""), ("float3 local[2]{};", ".x")] {
        for index in ["-1", "2", "3", "100", "0xffffffffu"] {
            let invalid = format!("local[{index}]{field}");
            let first = format!("local[0]{field}");
            let last = format!("local[1]{field}");
            for expression in [
                format!("({invalid}, 3)"),
                format!("((true ? {invalid} : {first}), 3)"),
                format!("((false ? {last} : {invalid}), 3)"),
            ] {
                rejected(&format!("__global__ void kernel() {{ {declaration} constexpr int count = {expression}; }}"));
            }
            for expression in [
                format!("((true ? {first} : {invalid}), 3)"),
                format!("((false ? {invalid} : {last}), 3)"),
                format!("true ? 3 : ({invalid}, 4)"),
            ] {
                let (module, _) = validate(&format!("__global__ void kernel() {{ {declaration} constexpr int count = {expression}; int values[count]; }}"));
                assert_eq!(array_count(&module, "values"), 3, "{expression}");
            }
        }
        let (module, _) = validate(&format!("__global__ void kernel() {{ {declaration} constexpr int count = (local[1]{field}, 3); int values[count]; }}"));
        assert_eq!(array_count(&module, "values"), 3);
    }
}

#[test]
fn cuda_constant_expressions_do_not_treat_foldable_intrinsics_or_grid_values_as_constexpr() {
    for expression in [
        "helper()",
        "min(1, 2)",
        "__clz(8)",
        "__ffs(8)",
        "__popc(3u)",
        "__brev(1u)",
        "__float_as_int(1.0f)",
        "__float_as_uint(1.0f)",
        "__int_as_float(1)",
        "__uint_as_float(1u)",
        "threadIdx.x",
        "blockIdx.x",
        "gridDim.x",
        "blockDim.x",
    ] {
        rejected(&format!("__device__ int helper() {{ return 1; }} __global__ void kernel() {{ constexpr int count = int({expression}); }}"));
    }
}

#[test]
fn cuda_constant_expressions_reject_signed_overflow_division_and_invalid_shift_counts() {
    for expression in [
        "2147483647 + 1",
        "(-2147483647 - 1) - 1",
        "1073741824 * 2",
        "-(-2147483647 - 1)",
        "1 / 0",
        "1 % 0",
        "(-2147483647 - 1) / -1",
        "(-2147483647 - 1) % -1",
        "1 << -1",
        "1 << 32",
        "1u >> 32",
        "-1 >> -1",
    ] {
        rejected(&format!(
            "__global__ void kernel() {{ constexpr int value = {expression}; }}"
        ));
    }
    validate("__global__ void kernel() { constexpr int safe = true ? 2 : (2147483647 + 1); int values[safe]; }");
}

#[test]
fn cuda_constant_expressions_allow_cpp20_wrapping_shifts_and_unsigned_arithmetic() {
    for (name, expression) in [
        ("int", "1 << 31"),
        ("int", "-1 << 1"),
        ("int", "-2 >> 1"),
        ("unsigned", "0x80000000u << 1"),
        ("unsigned", "0xffffffffu + 1u"),
        ("unsigned", "0u - 1u"),
        ("unsigned", "0x80000000u * 2u"),
        ("int", "int(0xffffffffu)"),
        ("unsigned", "unsigned(-1)"),
    ] {
        validate(&format!("__global__ void kernel({name} *out) {{ constexpr {name} value = {expression}; out[0] = value; }}"));
    }
}

#[test]
fn cuda_constant_expressions_check_float_to_integer_truncated_ranges_and_finite_results() {
    for (name, expression) in [
        ("int", "2147483520.0f"),
        ("int", "-2147483648.0f"),
        ("unsigned", "4294967040.0f"),
        ("unsigned", "-0.5f"),
        ("float", "1.0e-30f * 1.0e-30f"),
        ("bool", "2.75f"),
    ] {
        validate(&format!("__global__ void kernel(int *out) {{ constexpr {name} value = {expression}; out[0] = int(value); }}"));
    }
    for (name, expression) in [
        ("int", "2147483648.0f"),
        ("int", "-2147483904.0f"),
        ("unsigned", "4294967296.0f"),
        ("unsigned", "-1.0f"),
        ("float", "1.0f / 0.0f"),
        ("float", "3.4e38f * 2.0f"),
    ] {
        rejected(&format!(
            "__global__ void kernel() {{ constexpr {name} value = {expression}; }}"
        ));
    }
}

#[test]
fn cuda_constant_expressions_accept_integral_local_array_bounds_and_old_positive_transitions() {
    for (extent, expected) in [
        ("+1", 1),
        ("true", 1),
        ("1 + 1", 2),
        ("(1, 3)", 3),
        ("int(3.75f)", 3),
        ("false ? 9 : 4u", 4),
    ] {
        let (module, _) = validate(&format!(
            "/* 漢 */ __global__ void kernel(int size) {{ int values[{extent}]; }}"
        ));
        assert_eq!(array_count(&module, "values"), expected);
    }
    for extent in ["+1", "1 + 1"] {
        validate(&format!(
            "__global__ void kernel() {{ __shared__ int values[{extent}]; }}"
        ));
    }
}

#[test]
fn cuda_constant_expressions_array_bounds_reject_runtime_float_zero_negative_and_ub_values() {
    for extent in [
        "runtime",
        "0",
        "false",
        "-1",
        "1.0f",
        "1 / 0",
        "2147483647 + 1",
        "1 << 32",
        "(runtime++, 2)",
        "(runtime = 2, 2)",
    ] {
        for storage in ["", "__shared__ "] {
            rejected(&format!(
                "__global__ void kernel(int runtime) {{ {storage}int values[{extent}]; }}"
            ));
        }
    }
    let source =
        "__global__ void kernel(int runtime) { const int frozen = runtime; int values[frozen]; }";
    rejected(source);
}

#[test]
fn cuda_constant_expressions_vector_array_bounds_use_existing_cuda_layouts() {
    for scalar in ["float", "int", "uint"] {
        for (width, stride) in [(2, 8), (3, 12), (4, 16)] {
            let (module, _) = validate(&format!("constexpr int Count = 1 + 2; __global__ void kernel() {{ {scalar}{width} local[Count]; __shared__ {scalar}{width} shared[Count - 1]; }}"));
            assert_eq!(array_count(&module, "local"), 3);
            let local = module.entry_points[0]
                .function
                .local_variables
                .iter()
                .find_map(|(_, local)| (local.name.as_deref() == Some("local")).then_some(local))
                .unwrap();
            assert!(
                matches!(module.types[local.ty].inner, naga::TypeInner::Array { stride: actual, .. } if actual == stride)
            );
            let shared = module
                .global_variables
                .iter()
                .find_map(|(_, global)| {
                    (global.space == naga::AddressSpace::WorkGroup).then_some(global)
                })
                .unwrap();
            assert!(
                matches!(module.types[shared.ty].inner, naga::TypeInner::Array { size: naga::ArraySize::Constant(count), stride: actual, .. } if count.get() == 2 && actual == stride)
            );
        }
    }
}

#[test]
fn cuda_constant_expressions_preserve_array_type_size_and_depth_limits() {
    for (scalar, width) in [("bool", 1), ("int", 4)] {
        let maximum = naga::valid::MAX_TYPE_SIZE / width;
        let (module, _) = validate(&format!("__global__ void kernel() {{ constexpr unsigned Count = {maximum}u - 1u + 1u; {scalar} local[Count]; }}"));
        assert_eq!(array_count(&module, "local"), maximum);
        rejected(&format!("__global__ void kernel() {{ constexpr unsigned Count = {maximum}u + 1u; {scalar} local[Count]; }}"));
    }
    rejected(&format!(
        "__global__ void kernel() {{ constexpr int value = {}1{}; }}",
        "(".repeat(128),
        ")".repeat(128)
    ));
}

#[test]
fn cuda_constant_expressions_are_readonly_lvalues_including_selected_global_references() {
    for declaration in ["constexpr int value = 2;", "const int value = 2;"] {
        for operation in ["value = 3;", "value++;", "++value;", "value += 1;"] {
            rejected(&format!(
                "__global__ void kernel() {{ {declaration} {operation} }}"
            ));
        }
    }
    validate("constexpr int Global = 2; __global__ void kernel(int *out, int choose) { int local = 3; out[0] = choose ? Global : local; (choose ? Global : local); }");
    for operation in [
        "Global = 3;",
        "(true ? Global : local) = 3;",
        "++(false ? local : Global);",
    ] {
        let source = format!(
            "constexpr int Global = 2; __global__ void kernel() {{ int local = 3; {operation} }}"
        );
        assert!(rejected(&source).message.contains("const"));
    }
}

#[test]
fn cuda_constant_expressions_preserve_namespace_collisions_and_hiding() {
    for source in [
        "constexpr int Name = 1; constexpr int Name = 2; __global__ void kernel() {}",
        "using Name = int; constexpr int Name = 2; __global__ void kernel() {}",
        "constexpr int Name = 2; using Name = int; __global__ void kernel() {}",
        "__device__ int Name() { return 1; } constexpr int Name = 2; __global__ void kernel() {}",
        "constexpr int Name = 2; __device__ int Name() { return 1; } __global__ void kernel() {}",
        "constexpr int kernel = 2; __global__ void kernel() {}",
        "constexpr int min = 2; __global__ void kernel() {}",
        "__global__ void kernel(int value) { constexpr int value = 2; }",
    ] {
        rejected(source);
    }
    validate("constexpr int Global = 2; __device__ int helper() { return Global; } __global__ void kernel(int *out) { { constexpr int Global = 3; int local[Global]; } constexpr int helper = 4; out[0] = helper + Global; }");
}

#[test]
fn cuda_constant_expressions_reject_unsupported_constexpr_declarations_and_missing_initializers() {
    for declaration in [
        "constexpr int value;",
        "constexpr int values[2] = {};",
        "constexpr float3 value = make_float3(1, 2, 3);",
        "constexpr int *pointer = 0;",
        "constexpr int value{2, 3};",
        "constexpr int value = {2, 3};",
        "constexpr constexpr int value = 2;",
    ] {
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
    for source in [
        "constexpr __device__ int helper() { return 2; } __global__ void kernel() {}",
        "__device__ int helper(constexpr int value) { return value; } __global__ void kernel() {}",
        "using Value = constexpr int; __global__ void kernel() {}",
        "__global__ void kernel() { int values[] = {}; }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_constant_expressions_validate_unreachable_definitions_and_utf8_error_spans() {
    validate("__global__ void kernel() { return; constexpr int value = 2; int local[value]; }");
    for body in [
        "return; constexpr int value = runtime;",
        "return; int local[runtime];",
        "constexpr int value = missing;",
    ] {
        let source = format!("/* 漢 */ __global__ void kernel(int runtime) {{ {body} }}");
        rejected(&source);
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_constant_expressions_roundtrip_ordinary_const_integral_initializers() {
    for (name, expression, expected) in [
        ("int", "true ? 3 : (1 / 0)", "3"),
        ("int", "false ? (1 / 0) : 3", "3"),
        ("int", "true ? 3 : (2147483647 + 1)", "3"),
        ("unsigned", "true ? 3u : unsigned(1 / 0)", "3u"),
        ("unsigned", "false ? unsigned(1 / 0) : 3u", "3u"),
        ("unsigned", "0xffffffffu + 1u", "0u"),
        ("unsigned", "0u - 1u", "0xffffffffu"),
        ("bool", "true ? true : bool(1 / 0)", "true"),
        ("bool", "false ? bool(1 / 0) : true", "true"),
        ("bool", "false && (1 / 0)", "false"),
        ("bool", "true || (1 / 0)", "true"),
    ] {
        let (module, info) = validate(&format!("__global__ void kernel(int *out) {{ const {name} value = {expression}; constexpr bool Matches = value == {expected}; int values[Matches]{{}}; out[0] = int(value); }}"));
        assert_eq!(array_count(&module, "values"), 1, "{expression}");
        super::helpers::wgsl_roundtrip(
            &module,
            &info,
            naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
        );
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_constant_expressions_roundtrip_constants_lazy_branches_and_vector_layouts() {
    let (module, info) = validate(
        "constexpr int Count = 2; constexpr unsigned Wrapped = 0x80000000u << 1;
        constexpr float Scale = 0.5f;
        __device__ int helper(int runtime) { constexpr int Size = true ? Count : runtime; int values[Size]{}; return values[0]; }
        __global__ void kernel(unsigned *out, int runtime) {
            constexpr int chosen = true ? 3 : (1 / 0); float3 vectors[chosen]{};
            out[0] = unsigned(helper(runtime)) + Wrapped + unsigned(Scale); out[1] = unsigned(vectors[0].x);
        }",
    );
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}

#[test]
fn cuda_constant_expressions_bound_borrowed_pointer_addresses() {
    for expression in ["(&values, 4)", "(values + 3, 4)", "(*(&values[3] - 1), 4)"] {
        let (module, _) = validate(&format!(
            "__global__ void kernel(int *out) {{ __shared__ int values[3]; constexpr int Count = {expression}; int storage[Count]{{}}; out[0] = storage[3]; }}"
        ));
        assert_eq!(array_count(&module, "storage"), 4, "{expression}");
    }
    for expression in ["(values + 4 - 1, 4)", "(*(&values[3]), 4)"] {
        rejected(&format!(
            "__global__ void kernel(int *out) {{ __shared__ int values[3]; constexpr int Count = {expression}; int storage[Count]{{}}; out[0] = storage[3]; }}"
        ));
    }
    validate("__global__ void kernel(int *out) { __shared__ int values[3]; values[2] = 7; out[0] = *(&values[3] - 1); }");
    validate("__global__ void kernel(int *out) { __shared__ int values[3]; values[1] = 7; int count = 0; out[0] = *(++count, values + 1); }");
}
