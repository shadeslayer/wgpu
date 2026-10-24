use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

fn array_type(module: &naga::Module, name: &str) -> (naga::Handle<naga::Type>, u32, u32) {
    let local = module.entry_points[0]
        .function
        .local_variables
        .iter()
        .find_map(|(_, local)| (local.name.as_deref() == Some(name)).then_some(local))
        .unwrap();
    let naga::TypeInner::Array {
        base,
        size: naga::ArraySize::Constant(count),
        stride,
    } = module.types[local.ty].inner
    else {
        panic!("expected a complete fixed array for {name}");
    };
    (base, count.get(), stride)
}

#[test]
fn cuda_inferred_scalar_arrays_support_direct_copy_wrapped_and_trailing_lists() {
    for (name, element, scalar) in [
        ("bool", "true", naga::Scalar::BOOL),
        ("int", "2", naga::Scalar::I32),
        ("unsigned", "3u", naga::Scalar::U32),
        ("float", "1.5f", naga::Scalar::F32),
    ] {
        for initializer in [
            format!("{{{element}, {element}}}"),
            format!("= {{{element}, {element},}}"),
            format!("{{{{{element}}}, {{}}}}"),
        ] {
            let (module, _) = validate(&format!(
                "__global__ void kernel() {{ {name} values[] {initializer}; }}"
            ));
            let (base, count, stride) = array_type(&module, "values");
            let mut layouter = naga::proc::Layouter::default();
            layouter.update(module.to_ctx()).unwrap();
            assert_eq!(module.types[base].inner, naga::TypeInner::Scalar(scalar));
            assert_eq!(count, 2);
            assert_eq!(stride, layouter[base].to_stride());
        }
    }
    validate("__global__ void kernel() { int values[]{1, 2}; }");
    validate("__global__ void kernel() { int values[] = {1, 2}; }");
}

#[test]
fn cuda_inferred_vector_arrays_count_elements_after_brace_elision_and_whole_copies() {
    for base in ["int", "uint", "float"] {
        for (width, stride, alignment) in [(2, 8, 8), (3, 12, 4), (4, 16, 16)] {
            let components = (1..=width)
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            for (declarations, expected_count) in [
                (format!("{base}{width} values[]{{{{1}}, {{2}}}};"), 2),
                (format!("{base}{width} values[]{{{components}, 1}};"), 2),
                (format!("{base}{width} original{{{components}}}; {base}{width} values[]{{original, {{original}}, 1}};"), 3),
            ] {
                let (module, _) = validate(&format!("__global__ void kernel() {{ {declarations} }}"));
                let (element, count, actual_stride) = array_type(&module, "values");
                let mut layouter = naga::proc::Layouter::default();
                layouter.update(module.to_ctx()).unwrap();
                assert_eq!(count, expected_count);
                assert_eq!(actual_stride, stride);
                assert_eq!(layouter[element].size, stride);
                assert_eq!(layouter[element].alignment, naga::proc::Alignment::new(alignment).unwrap());
            }
        }
    }
}

#[test]
fn cuda_inferred_arrays_preserve_aliases_const_qualification_and_cached_declarator_types() {
    let (module, _) = validate("using Number = int; using Triple = int3; using Frozen = const Triple; __global__ void kernel(int *out) { Number Number[]{3, 4}, next[]{Number[1], Number[0]}, scalar{next[0]}; Frozen fixed[]{{1, 2, 3}, {4}}; Triple copies[]{fixed[0], fixed[1]}; copies[0].x = 9; out[0] = scalar + fixed[0].x + copies[0].x; }");
    assert_eq!(array_type(&module, "Number").1, 2);
    assert_eq!(array_type(&module, "next").1, 2);
    assert_eq!(array_type(&module, "fixed").1, 2);
    for (declaration, operation) in [
        ("const int values[]{1, 2};", "values[0] = 3;"),
        ("const int3 values[]{{1}, {2}};", "values[1].y++;"),
        (
            "using Frozen = const unsigned; Frozen values[]{1u};",
            "++values[0];",
        ),
    ] {
        assert!(rejected(&format!(
            "__global__ void kernel() {{ {declaration} {operation} }}"
        ))
        .message
        .contains("const"));
    }
}

#[test]
fn cuda_inferred_array_self_reads_and_earlier_mutations_preserve_element_order() {
    validate("__global__ void kernel(int *out) { int values[]{1, values[0] + 1, values[1] + 1}; int changed[]{1, (changed[0] = 7, 2), changed[0]}; int3 vectors[]{{1, vectors[0].x + 1, vectors[0].y + 1}, {vectors[0]}}; out[0] = values[2] + changed[0] + changed[2] + vectors[1].z; }");
}

#[test]
fn cuda_inferred_array_clauses_snapshot_calls_once_before_following_clauses() {
    let (module, _) = validate("__device__ int pair(int first, int second) { return first * 10 + second; } __device__ int3 triple(int seed) { int3 value{seed, seed + 1, seed + 2}; return value; } __global__ void kernel(int *out) { int counter = 1; int values[]{pair(counter++, counter++), counter++, (counter++, counter)}; int3 vectors[]{triple(counter++), triple(counter++)}; out[0] = values[0] + values[2] + vectors[0].x + vectors[1].z + counter; }");
    let mut calls = 0;
    naga::proc::any_statement(&module.entry_points[0].function.body, &mut |statement| {
        if matches!(statement, naga::Statement::Call { .. }) {
            calls += 1;
        }
        false
    });
    assert_eq!(calls, 3);
}

#[test]
fn cuda_inferred_arrays_work_in_helpers_for_initializers_and_repeated_blocks() {
    validate("__device__ int sum(int seed) { int values[]{seed, seed + 1}; return values[0] + values[1]; } __device__ void consume(int value) { bool flags[]{value != 0, false}; if (flags[0]) return; } __global__ void kernel(int *out) { int total = 0; for (int values[]{1, 2}, iteration = 0; iteration < 3; ++iteration) { int3 reset[]{{iteration}, {}}; total += reset[0].y + reset[1].x; reset[0].y = 9; if (iteration == 1) continue; total += values[0] + sum(iteration); } { int values[]{total}; out[0] = values[0]; } consume(total); }");
    let source = "__global__ void kernel() { for (int hidden[]{1}; false;) {} hidden[0]; }";
    assert_eq!(&source[rejected(source).span.to_range().unwrap()], "hidden");
    for body in [
        "int values[]{1}, values[]{2};",
        "int class[]{1};",
        "using Value = int; int Value[]{1};",
    ] {
        rejected(&format!("__global__ void kernel() {{ {body} }}"));
    }
}

#[test]
fn cuda_inferred_bounds_allow_zero_offset_constant_addresses_while_bound_is_unknown() {
    for expression in [
        "(values[0], 2)",
        "(values[-0], 2)",
        "(values[0u], 2)",
        "+(values[0], 2)",
        "unsigned((values[0], 2))",
        "(values[0], 1) + 1",
        "(values[0], 4) >> 1",
        "(values[0], true) ? 2 : 3",
        "(values[0], false) || true",
        "(values[0], true) && true",
        "((values[0], 1), 2)",
        "((true ? values[0] : values[100]), 2)",
    ] {
        let (module, _) = validate(&format!(
            "__global__ void kernel() {{ float values[]{{1.0f, {expression}}}; }}"
        ));
        assert_eq!(array_type(&module, "values").1, 2);
    }
    validate("__global__ void kernel() { float3 values[]{{1.0f, (values[0].x, 2)}, {3.0f}}; }");
    for declaration in [
        "unsigned values[]{0u, (values[0], 2)};",
        "int values[]{0, (values[0], 2u)};",
        "bool values[]{true, (values[0], 1)};",
    ] {
        validate(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
}

#[test]
fn cuda_inferred_bounds_reject_nonzero_offsets_for_constant_narrowing() {
    for index in ["-1", "1", "2", "3", "100", "0xffffffffu"] {
        for expression in [
            format!("(values[{index}], 2)"),
            format!("unsigned((values[{index}], 2))"),
            format!("+(values[{index}], 2)"),
            format!("(values[{index}], 1) + 1"),
            format!("(values[{index}], true) ? 2 : 3"),
            format!("(values[{index}], false) || true"),
        ] {
            rejected(&format!(
                "__global__ void kernel() {{ float values[]{{1.0f, {expression}}}; }}"
            ));
        }
        let declaration = format!("float3 values[]{{{{1.0f, (values[{index}].x, 2)}}, {{3.0f}}}};");
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
        for declaration in [
            format!("unsigned values[]{{0u, (values[{index}], 2)}};"),
            format!("int values[]{{0, (values[{index}], 2u)}};"),
            format!("bool values[]{{true, (values[{index}], 1)}};"),
        ] {
            rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
        }
    }
    for index in ["1", "2"] {
        rejected(&format!(
            "__global__ void kernel() {{ float values[]{{1.0f, 2.0f, (values[{index}], 3)}}; }}"
        ));
    }
    rejected("__global__ void kernel(int index) { float values[]{1.0f, (values[index], 2)}; }");
}

#[test]
fn cuda_inferred_bound_facts_follow_selected_lazy_branches_without_evaluating_skipped_addresses() {
    for expression in [
        "true ? 2 : (values[100], 3)",
        "false ? (values[100], 3) : 2",
        "true ? (values[0], 2) : (values[100], 3)",
        "false && (values[100], true)",
        "true || (values[100], false)",
    ] {
        validate(&format!(
            "__global__ void kernel() {{ float values[]{{1.0f, {expression}}}; }}"
        ));
    }
    for expression in [
        "false ? 2 : (values[100], 3)",
        "true ? (values[100], 3) : 2",
        "((false ? values[0] : values[100]), 2)",
    ] {
        rejected(&format!(
            "__global__ void kernel() {{ float values[]{{1.0f, {expression}}}; }}"
        ));
    }
}

#[test]
fn cuda_ordinary_inferred_array_expressions_do_not_require_constant_address_eligibility() {
    for expression in ["(values[2], 2)", "(values[-1], 2)", "(values[100], 2)"] {
        validate(&format!(
            "__global__ void kernel() {{ int values[]{{1, {expression}}}; }}"
        ));
    }
    validate("__global__ void kernel() { float values[]{1.0f, float((values[100], 2))}; }");
    validate("__global__ void kernel(int index) { int values[]{1, (values[index], 2)}; }");
    validate("__global__ void kernel() { int values[]{0, (values[100], true)}; }");
    validate("__global__ void kernel() { unsigned values[]{0u, (values[100], false)}; }");
}

#[test]
fn cuda_completed_inferred_bounds_supply_the_existing_constant_address_checks() {
    validate("__global__ void kernel() { int values[]{1, 2}; constexpr int Count = (values[1], 3); float result{(values[0], 2)}; int scratch[Count]{}; }");
    for index in ["-1", "2", "100"] {
        rejected(&format!("__global__ void kernel() {{ int values[]{{1, 2}}; constexpr int Count = (values[{index}], 3); }}"));
    }
}

#[test]
fn cuda_inferred_arrays_preserve_list_narrowing_and_exact_vector_copy_rules() {
    for body in [
        "int values[]{1.0f};",
        "unsigned values[]{-1};",
        "bool values[]{2};",
        "float values[]{16777217};",
        "float values[]{runtime};",
        "float3 values[]{{1, runtime}};",
        "int3 values[]{{1, 2, 3, 4}};",
        "int3 original{1, 2, 3}; uint3 values[]{original};",
        "int3 original{1, 2, 3}; int values[]{original};",
        "int values[]{{{1}}};",
    ] {
        rejected(&format!("__global__ void kernel(int runtime) {{ {body} }}"));
    }
    validate("__global__ void kernel(int runtime) { float values[]{16777218, float(runtime)}; unsigned counts[]{unsigned(runtime)}; }");
}

#[test]
fn cuda_inferred_arrays_still_require_nonempty_lists_and_supported_local_element_types() {
    for body in [
        "int values[];",
        "const int values[];",
        "int values[]{};",
        "int values[] = {};",
        "int values[] = 1;",
        "int values[0]{1};",
        "int values[][]{{1, 2}};",
        "int *values[]{0};",
        "constexpr int values[]{1};",
        "__shared__ int values[]{1};",
        "__shared__ float3 values[]{{1}};",
        "extern __shared__ int values[];",
        "double values[]{1.0f};",
        "void values[]{1};",
        "int values[]{1}; int *pointer = values;",
        "int source[]{1}; int values[] = source;",
    ] {
        rejected(&format!("__global__ void kernel() {{ {body} }}"));
    }
}

#[test]
fn cuda_inferred_arrays_check_malformed_and_unreachable_initializers_with_utf8_spans() {
    for body in [
        "int values[]{,};",
        "int values[]{1,,2};",
        "int values[]{1;",
        "int values[] = {1;",
        "int values[]{1} extra;",
        "int first[]{1},;",
        "int values[]{.x = 1};",
    ] {
        rejected(&format!(
            "/* café 漢 */ __global__ void kernel() {{ return; {body} }}"
        ));
    }
    let source = "/* café 漢 */ __global__ void kernel() { int values[]{1, missing}; }";
    assert_eq!(
        &source[rejected(source).span.to_range().unwrap()],
        "missing"
    );
}

#[test]
fn cuda_inferred_array_lowering_is_linear_and_preserves_nesting_limits() {
    for count in [8, 1024] {
        let clauses = vec!["1"; count].join(", ");
        let (module, _) = validate(&format!(
            "__global__ void kernel() {{ int values[]{{{clauses}}}; }}"
        ));
        assert_eq!(array_type(&module, "values").1, count as u32);
        let function = &module.entry_points[0].function;
        assert!(function.expressions.len() < count * 8 + 64);
        assert!(function.local_variables.len() < 16);
        assert!(module.types.len() < 32);
    }
    let nested = "{".repeat(160) + "1" + &"}".repeat(160);
    rejected(&format!(
        "__global__ void kernel() {{ int values[]{nested}; }}"
    ));
    let expression = "1 + ".repeat(80) + "1";
    let error = rejected(&format!(
        "__global__ void kernel() {{ int values[]{{{expression}}}; }}"
    ));
    assert!(error.message.contains("depth") || error.message.contains("nesting"));
}

#[test]
fn cuda_inferred_arrays_preserve_resource_bindings_and_complete_constant_ir_types() {
    let (module, _) = validate("__global__ void kernel(const int *input, int factor, int *out) { int values[]{input[0], input[1]}; int3 vectors[]{{1}, {2}}; out[0] = values[1] * factor + vectors[1].x; }");
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
    assert!(module.entry_points[0]
        .function
        .local_variables
        .iter()
        .all(|(_, local)| !matches!(
            module.types[local.ty].inner,
            naga::TypeInner::Array {
                size: naga::ArraySize::Dynamic | naga::ArraySize::Pending(_),
                ..
            }
        )));
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_inferred_arrays_roundtrip_through_wgsl_with_self_references_and_vector_elision() {
    let (module, info) = validate(include_str!("../../in/cuda/inferred-arrays.cu"));
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}
