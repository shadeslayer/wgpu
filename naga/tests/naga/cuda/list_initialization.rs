use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

#[test]
fn cuda_scalar_lists_support_direct_copy_empty_and_trailing_comma_forms() {
    for (name, expression, scalar) in [
        ("bool", "true", naga::Scalar::BOOL),
        ("int", "-2", naga::Scalar::I32),
        ("unsigned", "3u", naga::Scalar::U32),
        ("float", "1.5f", naga::Scalar::F32),
    ] {
        for initializer in [
            "{}".into(),
            "= {}".into(),
            format!("{{{expression}}}"),
            format!("= {{{expression},}}"),
        ] {
            let (module, _) = validate(&format!(
                "__global__ void kernel() {{ {name} value {initializer}; }}"
            ));
            let variable = module.entry_points[0]
                .function
                .local_variables
                .iter()
                .next()
                .unwrap()
                .1;
            assert_eq!(
                module.types[variable.ty].inner,
                naga::TypeInner::Scalar(scalar)
            );
        }
    }
}

#[test]
fn cuda_scalar_constexpr_lists_work_at_file_and_local_scope() {
    for declaration in ["constexpr int value{2};", "constexpr int value = {2};"] {
        validate(&format!(
            "{declaration} __global__ void kernel(int *out) {{ out[0] = value; }}"
        ));
        validate(&format!(
            "__global__ void kernel(int *out) {{ {declaration} out[0] = value; }}"
        ));
    }
    validate("using Count = const int; constexpr Count Size{3}, Next = {Size + 1}; constexpr float Scale{0.5f}; constexpr bool Ready{}; __global__ void kernel(int *out) { constexpr unsigned Count{2u}; constexpr float Local{Scale}; int values[Next]{1}; out[0] = values[Size] + int(Local) + int(Ready) + int(Count); } constexpr int Last{Next};");
}

#[test]
fn cuda_braced_integral_constants_preserve_lazy_normalization_and_bound_eligibility() {
    validate("constexpr int Global{true ? 3 : (1 / 0)}; __global__ void kernel(int *out, int runtime) { const int Size{true ? Global : (1 / 0)}; const unsigned More = {unsigned(Size) + 1u}; constexpr int Selected{true ? 2 : runtime}; int values[More]{1}; out[0] = values[Size] + Selected; }");
    for body in [
        "int mutable_value{2}; constexpr int result{mutable_value};",
        "const int runtime_value{runtime}; constexpr int result{runtime_value};",
        "const float ordinary{2.0f}; constexpr float result{ordinary};",
        "constexpr int result{result};",
    ] {
        rejected(&format!("__global__ void kernel(int runtime) {{ {body} }}"));
    }
    validate("__global__ void kernel(int runtime) { constexpr int self{true ? 3 : self}; constexpr int discarded{(runtime, 4)}; int values[self + discarded]{1}; }");
}

#[test]
fn cuda_runtime_scalar_lists_apply_cpp_narrowing_rules() {
    for (destination, source, allowed) in [
        ("bool", "flag", true),
        ("bool", "integer", false),
        ("bool", "count", false),
        ("bool", "sample", false),
        ("int", "flag", true),
        ("int", "integer", true),
        ("int", "count", false),
        ("int", "sample", false),
        ("unsigned", "flag", true),
        ("unsigned", "integer", false),
        ("unsigned", "count", true),
        ("unsigned", "sample", false),
        ("float", "flag", false),
        ("float", "integer", false),
        ("float", "count", false),
        ("float", "sample", true),
    ] {
        let program = format!("__global__ void kernel(int integer, unsigned count, float sample) {{ bool flag = integer != 0; {destination} value{{{source}}}; }}");
        if allowed {
            validate(&program);
        } else {
            rejected(&program);
        }
    }
}

#[test]
fn cuda_constant_integer_lists_allow_only_representable_narrowing_exceptions() {
    for declaration in [
        "bool value{0};",
        "bool value{1u};",
        "unsigned value{0};",
        "unsigned value{2147483647};",
        "int value{2147483647u};",
        "float value{16777216};",
        "float value{16777218};",
        "float value{-2147483647 - 1};",
        "float value{0xffffff00u};",
        "float value{true};",
        "const int Exact = 16777218; float value{Exact};",
        "constexpr bool Enabled{true}; float value{Enabled};",
    ] {
        validate(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
    for declaration in [
        "bool value{2};",
        "bool value{-1};",
        "unsigned value{-1};",
        "int value{2147483648u};",
        "float value{16777217};",
        "float value{2147483647};",
        "float value{0xffffffffu};",
        "const int Inexact = 16777217; float value{Inexact};",
    ] {
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
}

#[test]
fn cuda_float_to_integral_lists_narrow_even_when_the_source_is_constant() {
    for destination in ["bool", "int", "unsigned"] {
        for expression in ["0.0f", "1.0f", "3.0f", "-0.0f"] {
            rejected(&format!(
                "__global__ void kernel() {{ {destination} value{{{expression}}}; }}"
            ));
        }
    }
    validate("__global__ void kernel(int integer, float sample) { int truncated{int(sample)}; unsigned wrapped{unsigned(integer)}; float rounded{float(integer)}; bool flag{bool(sample)}; }");
}

#[test]
fn cuda_scalar_lists_reject_extra_brace_nesting_and_multiple_elements() {
    for declaration in [
        "int value{{1}};",
        "int value{{}};",
        "int value{1, 2};",
        "int value = {1, 2};",
        "int values[1]{{{1}}};",
        "int3 value{{{1}}, 2, 3};",
    ] {
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
    validate("__global__ void kernel() { int values[2]{{1}, {}}; int3 value{{1}, {}, {3}}; }");
}

#[test]
fn cuda_all_numeric_vectors_support_partial_and_complete_lists() {
    for base in ["int", "uint", "float"] {
        for width in [2, 3, 4] {
            let element = if base == "float" { "1.5f" } else { "1" };
            let components = vec![element; width].join(", ");
            validate(&format!("__global__ void kernel() {{ {base}{width} complete{{{components}}}; {base}{width} partial = {{{element},}}; const {base}{width} frozen{{{components}}}; }}"));
        }
    }
}

#[test]
fn cuda_vector_lists_preserve_physical_types_and_layouts() {
    for base in ["int", "uint", "float"] {
        for (width, stride, alignment) in [(2, 8, 8), (3, 12, 4), (4, 16, 16)] {
            let (module, _) = validate(&format!(
                "__global__ void kernel() {{ {base}{width} values[2]{{{{1}}, {{2}}}}; }}"
            ));
            let variable = module.entry_points[0]
                .function
                .local_variables
                .iter()
                .next()
                .unwrap()
                .1;
            let naga::TypeInner::Array {
                base: element,
                stride: actual,
                ..
            } = module.types[variable.ty].inner
            else {
                panic!("expected an array");
            };
            let mut layouter = naga::proc::Layouter::default();
            layouter.update(module.to_ctx()).unwrap();
            assert_eq!(actual, stride);
            assert_eq!(layouter[element].size, stride);
            assert_eq!(
                layouter[element].alignment,
                naga::proc::Alignment::new(alignment).unwrap()
            );
        }
    }
}

#[test]
fn cuda_vector_list_copy_accepts_exact_alias_helper_and_conditional_values() {
    validate("using Triple = int3; using Frozen = const Triple; __device__ Triple produce(int seed) { Triple result{seed, seed + 1, seed + 2}; return result; } __global__ void kernel(int *out, int choose) { Frozen original{1, 2, 3}; Triple copy{original}; Triple from_call = {produce(4)}; Triple selected{choose ? copy : from_call}; Triple values[3]{original, {from_call}, selected}; copy.x = 9; out[0] = values[0].x + values[1].y + values[2].z; }");
    for declaration in [
        "int3 copied{{original}};",
        "int2 copied{original};",
        "uint3 copied{original};",
        "int copied{original};",
    ] {
        rejected(&format!(
            "__global__ void kernel() {{ int3 original{{1, 2, 3}}; {declaration} }}"
        ));
    }
}

#[test]
fn cuda_scalar_arrays_support_partial_lists_wrapped_elements_and_zero_tails() {
    for (base, value) in [
        ("bool", "true"),
        ("int", "2"),
        ("unsigned", "3u"),
        ("float", "1.5f"),
    ] {
        for initializer in [
            format!("{{{value}}}"),
            format!("= {{{value},}}"),
            format!("{{{{{value}}}, {{}}}}"),
        ] {
            validate(&format!(
                "__global__ void kernel() {{ const {base} values[4] {initializer}; }}"
            ));
        }
    }
    for declaration in [
        "int values[2] = {1};",
        "int values[2]{1};",
        "int values[2] = {{}};",
    ] {
        validate(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
}

#[test]
fn cuda_vector_array_lists_support_nested_flat_and_mixed_brace_elision() {
    for base in ["int", "uint", "float"] {
        for width in [2, 3, 4] {
            let components = (1..=width)
                .map(|number| number.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            for initializer in [
                format!("{{{{{components}}}, {{1}}}}"),
                format!("{{{components}, 1}}"),
                format!("{{{{{components}}}, 1}}"),
            ] {
                validate(&format!(
                    "__global__ void kernel() {{ {base}{width} values[3]{initializer}; }}"
                ));
            }
        }
    }
}

#[test]
fn cuda_list_elements_are_initialized_in_order_and_remain_visible_to_later_clauses() {
    validate("__global__ void kernel(int *out) { int values[3]{1, values[0] + 1, values[1] + 1}; int3 point{4, point.x + 1, point.y + 1}; int changed[3]{1, (changed[0] = 7, 2), changed[0]}; int3 changed_point{1, (changed_point.x = 9, 2), changed_point.x}; out[0] = values[2] + point.z + changed[0] + changed[2] + changed_point.x + changed_point.z; }");
}

#[test]
fn cuda_list_expressions_keep_calls_argument_commas_and_parenthesized_comma_expressions() {
    let (module, _) = validate("__device__ int pair(int first, int second) { return first * 10 + second; } __global__ void kernel(int *out) { int counter = 1; int values[3]{pair(counter++, 4), (counter++, counter), pair(counter++, 5)}; out[0] = values[0] + values[1] + values[2] + counter; }");
    let function = &module.entry_points[0].function;
    let mut calls = 0;
    naga::proc::any_statement(&function.body, &mut |statement| {
        if matches!(statement, naga::Statement::Call { .. }) {
            calls += 1;
        }
        false
    });
    assert_eq!(calls, 2);
}

#[test]
fn cuda_list_declarators_cache_types_and_observe_point_of_declaration() {
    validate("using Number = int; using Triple = int3; __global__ void kernel(int *out) { Number Number{3}, values[3]{Number, Number + 1}, last{values[1]}; Triple Triple{1, 2, 3}, copies[2]{Triple}, other{copies[0]}; out[0] = Number + last + other.z; { int Number{5}; out[1] = Number; } }");
    for declaration in [
        "int first = 1, second{};",
        "int value{};",
        "int value = {};",
    ] {
        validate(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
    let (module, _) = validate("__global__ void kernel() { int value{value}; }");
    assert_eq!(module.entry_points[0].function.local_variables.len(), 1);
    for declaration in [
        "int first{1}, first{2};",
        "int class{1};",
        "using Number = int; int Number{1};",
    ] {
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
}

#[test]
fn cuda_list_initializers_reset_in_loops_for_initializers_and_helpers() {
    validate("__device__ int sum(int seed) { int values[3]{seed, seed + 1}; return values[0] + values[1] + values[2]; } __device__ void consume(int seed) { int3 value{seed}; if (value.y == 0) return; } __global__ void kernel(int *out) { int total{0}; for (int iteration{0}, limit{3}; iteration < limit; ++iteration) { int values[3]{iteration}; int3 point{iteration}; total += values[2] + point.y; values[2] = 9; point.y = 8; if (iteration == 1) continue; total += sum(iteration); } consume(total); out[0] = total; }");
    let source = "__global__ void kernel() { for (int hidden{0}; false;) {} hidden; }";
    assert_eq!(&source[rejected(source).span.to_range().unwrap()], "hidden");
}

#[test]
fn cuda_const_lists_keep_scalar_vector_and_array_elements_read_only() {
    for (declaration, operation) in [
        ("const int value{1};", "value = 2;"),
        ("const int values[2]{1};", "values[0]++;"),
        (
            "using Frozen = const int3; Frozen value{1, 2};",
            "value.y += 1;",
        ),
        ("const int3 values[2]{{1}, {2}};", "++values[1].x;"),
    ] {
        let error = rejected(&format!(
            "__global__ void kernel() {{ {declaration} {operation} }}"
        ));
        assert!(error.message.contains("const"));
    }
}

#[test]
fn cuda_list_narrowing_is_checked_in_nested_and_elided_leaves() {
    for declaration in [
        "int values[2]{1.0f};",
        "bool flags[2]{2};",
        "unsigned values[2]{-1};",
        "int3 value{1, 2.0f, 3};",
        "float3 value{1, runtime, 3};",
        "int3 values[2]{{1, 2, 3}, 4.0f};",
        "uint3 values[2]{1, 2, 3, -1};",
        "float3 values[2]{{16777217}};",
    ] {
        rejected(&format!(
            "__global__ void kernel(int runtime) {{ {declaration} }}"
        ));
    }
    validate("__global__ void kernel(int runtime) { float3 value{1, float(runtime), 3}; unsigned values[2]{0, unsigned(runtime)}; }");
}

#[test]
fn cuda_lists_reject_excess_clauses_at_the_current_aggregate_boundary() {
    for declaration in [
        "int value{1, 2};",
        "int values[2]{1, 2, 3};",
        "int2 value{1, 2, 3};",
        "int3 values[2]{{1, 2, 3, 4}, {5}};",
        "int2 values[2]{1, 2, 3, 4, 5};",
        "int3 source{1, 2, 3}; int3 values[1]{source, 4};",
        "int source[2]{}; int values[2]{source};",
    ] {
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
}

#[test]
fn cuda_lists_validate_malformed_and_unreachable_source_with_utf8_spans() {
    for declaration in [
        "int value{,};",
        "int value{1,,};",
        "int values[2]{1,,2};",
        "int value{1;",
        "int values[2]{1, 2;",
        "int value{} extra;",
        "int first{1},;",
        "int values[2]{{1};",
    ] {
        rejected(&format!(
            "/* café 漢 */ __global__ void kernel() {{ return; {declaration} }}"
        ));
    }
    let source = "/* café 漢 */ __global__ void kernel() { int values[2]{1, missing}; }";
    assert_eq!(
        &source[rejected(source).span.to_range().unwrap()],
        "missing"
    );
}

#[test]
fn cuda_lists_reject_empty_inferred_arrays_and_unsupported_initialization_contexts() {
    for body in [
        "int values[]{};",
        "int values[] = {};",
        "int values[2][2]{{1, 2, 3}};",
        "int *pointer{0};",
        "__shared__ int values[2]{1};",
        "__shared__ int3 point{1};",
        "int3 point{.x = 1};",
        "int3 point = int3{1, 2, 3};",
        "int value = int{1};",
        "int value = 0; value = {1};",
        "constexpr int values[2]{1};",
        "constexpr int3 point{1};",
    ] {
        rejected(&format!("__global__ void kernel() {{ {body} }}"));
    }
    rejected("__device__ int helper() { return {1}; } __global__ void kernel() {}");
    rejected("__device__ void consume(int value) {} __global__ void kernel() { consume({1}); }");
}

#[test]
fn cuda_partial_large_array_initializers_have_bounded_ir_size() {
    let count = naga::valid::MAX_TYPE_SIZE / 4;
    let (module, _) = validate(&format!(
        "__global__ void kernel() {{ int values[{count}u]{{1}}; }}"
    ));
    let function = &module.entry_points[0].function;
    assert!(function.expressions.len() < 256);
    assert!(function.local_variables.len() < 16);
    let mut statements = 0;
    naga::proc::any_statement(&function.body, &mut |_| {
        statements += 1;
        false
    });
    assert!(statements < 64);
    rejected(&format!(
        "__global__ void kernel() {{ int values[{}u]{{1}}; }}",
        count + 1
    ));
}

#[test]
fn cuda_initializer_nesting_and_expression_graph_depth_are_bounded() {
    let excessive_braces = format!("{}1{}", "{".repeat(160), "}".repeat(160));
    rejected(&format!(
        "__global__ void kernel() {{ int values[1]{excessive_braces}; }}"
    ));
    let deep_expression = "1 + ".repeat(80) + "1";
    let error = rejected(&format!(
        "__global__ void kernel() {{ int values[1]{{{deep_expression}}}; }}"
    ));
    assert!(error.message.contains("depth") || error.message.contains("nesting"));
}

#[test]
fn cuda_list_initialization_preserves_bindings_and_barrier_uniformity() {
    let (module, _) = validate("__global__ void kernel(const int *input, int factor, int *out) { int values[4]{1}; __syncthreads(); int3 point{2}; out[0] = values[3] + point.z + input[0] * factor; }");
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
    assert_eq!(module.entry_points[0].function.arguments.len(), 0);
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_list_initializers_roundtrip_through_wgsl_with_constants_and_aggregate_copies() {
    let (module, info) = validate(include_str!("../../in/cuda/list-initialization.cu"));
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}
