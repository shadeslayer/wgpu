use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/operator-addition.cu");
const INT3_ADDITION: &str = "__device__ int3 operator+(int3 left, int3 right) {
    return make_int3(left.x + right.x, left.y + right.y, left.z + right.z);
}";

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

fn calls(
    function: &naga::Function,
) -> Vec<(
    naga::Handle<naga::Function>,
    &[naga::Handle<naga::Expression>],
)> {
    function
        .body
        .iter()
        .filter_map(|statement| match statement {
            naga::Statement::Call {
                function,
                arguments,
                ..
            } => Some((*function, arguments.as_slice())),
            _ => None,
        })
        .collect()
}

#[test]
fn cuda_operator_addition_supports_all_vector_types_and_operand_forms() {
    let (module, _) = validate(FIXTURE);
    assert_eq!(module.functions.len(), 27);
    assert_eq!(calls(&module.entry_points[0].function).len(), 27);
    for (_, function) in module.functions.iter() {
        assert_eq!(function.arguments.len(), 2);
        assert!(function
            .arguments
            .iter()
            .all(|argument| argument.binding.is_none()));
        let result_type = function.result.as_ref().unwrap().ty;
        assert!(function
            .arguments
            .iter()
            .any(|argument| argument.ty == result_type));
    }
}

#[test]
fn cuda_operator_function_ids_accept_token_whitespace_and_comments() {
    for spelling in ["operator+", "operator +", "operator /* λ */ +"] {
        validate(&format!(
            "__device__ int2 {spelling}(int2 left, int2 right) {{ return make_int2(left.x + right.x, left.y + right.y); }}
            __global__ void kernel(int *out) {{ int2 left = make_int2(1, 2), right = make_int2(3, 4); out[0] = ({spelling}(left, right)).x; out[1] = (left + right).y; }}"
        ));
    }
}

#[test]
fn cuda_operator_addition_accepts_aliases_and_existing_function_specifiers() {
    for header in [
        "__device__",
        "static __device__ inline",
        "__host__ inline __device__",
        "__forceinline__ __device__",
        "__device__ __noinline__",
    ] {
        validate(&format!(
            "using Vector = int4; using Fixed = const Vector;
            {header} Vector operator+(Fixed left, Fixed right) {{ return make_int4(left.x + right.x, left.y, right.z, right.w); }}
            __global__ void kernel(int *out) {{ Vector value = make_int4(1, 2, 3, 4); out[0] = (value + value).x; }}"
        ));
    }
}

#[test]
fn cuda_operator_addition_requires_two_parameters_and_a_source_vector_class() {
    for parameters in [
        "",
        "int2 first, int2 second, int2 third",
        "int left, int right",
        "float left, unsigned right",
        "bool left, int right",
    ] {
        rejected(&format!(
            "__device__ int operator+({parameters}) {{ return 1; }} __global__ void kernel() {{}}"
        ));
    }
    for width in [2, 3, 4] {
        validate(&format!("__device__ int operator+(int{width} left, int right) {{ return left.x + right; }} __global__ void kernel(int *out) {{ out[0] = make_int{width}({}) + 2; }}", vec!["1"; width].join(", ")));
    }
}

#[test]
fn cuda_operator_addition_rejects_canonical_and_return_only_duplicates() {
    for result in ["int", "float", "void"] {
        let body = if result == "void" { "" } else { "return 2;" };
        let source = format!(
            "using Vector = const int2;
            __device__ int operator+(int2 left, int right) {{ return left.x + right; }}
            __device__ {result} operator+(Vector other, const int value) {{ {body} }}
            __global__ void kernel() {{}}"
        );
        let error = rejected(&source);
        assert!(error.message.contains("duplicate"), "{error}");
        assert_eq!(&source[error.span.to_range().unwrap()], "operator+");
    }
}

#[test]
fn cuda_operator_addition_reuses_scalar_conversion_ranking() {
    let (module, _) = validate(
        "__device__ int operator+(int2 left, bool right) { return 1; }
        __device__ int operator+(int2 left, int right) { return 2; }
        __device__ int operator+(int2 left, unsigned right) { return 3; }
        __device__ int operator+(int2 left, float right) { return 4; }
        __global__ void kernel(int *out) { int2 value = make_int2(5, 6); out[0] = value + true; out[1] = value + 3; out[2] = value + 3u; out[3] = value + 3.0f; }",
    );
    for ((selected, _), expected) in calls(&module.entry_points[0].function).into_iter().zip([
        naga::Scalar::BOOL,
        naga::Scalar::I32,
        naga::Scalar::U32,
        naga::Scalar::F32,
    ]) {
        assert_eq!(
            module.types[module.functions[selected].arguments[1].ty].inner,
            naga::TypeInner::Scalar(expected)
        );
    }
    let (module, _) = validate("__device__ int operator+(int2 left, int right) { return 1; } __device__ int operator+(int2 left, float right) { return 2; } __global__ void kernel(int *out) { out[0] = make_int2(1, 2) + true; }");
    let selected = calls(&module.entry_points[0].function)[0].0;
    assert_eq!(
        module.types[module.functions[selected].arguments[1].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::I32)
    );
    for argument in ["3u", "3.0f"] {
        let definitions = if argument == "3u" { "int" } else { "unsigned" };
        let source = format!("__device__ int operator+(int2 left, {definitions} right) {{ return 1; }} __device__ int operator+(int2 left, bool right) {{ return 2; }} __global__ void kernel() {{ make_int2(1, 2) + {argument}; }}");
        assert!(rejected(&source).message.contains("ambiguous"));
    }
}

#[test]
fn cuda_operator_addition_preserves_prior_definition_visibility() {
    let (module, _) = validate(
        "__device__ int operator+(int2 left, int right) { return 1; }
        __device__ int earlier(unsigned value) { return make_int2(1, 2) + value; }
        __device__ int operator+(int2 left, unsigned right) { return 2; }
        __global__ void kernel(int *out) { out[0] = earlier(3u); out[1] = make_int2(1, 2) + 3u; }",
    );
    let earlier = module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some("earlier"))
        .unwrap()
        .1;
    let selected = calls(earlier)[0].0;
    assert_eq!(
        module.types[module.functions[selected].arguments[1].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::I32)
    );
    let selected = calls(&module.entry_points[0].function)[1].0;
    assert_eq!(
        module.types[module.functions[selected].arguments[1].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::U32)
    );
    rejected("__device__ int earlier(int2 value) { return value + value; } __device__ int operator+(int2 left, int2 right) { return 1; } __global__ void kernel() {}");
}

#[test]
fn cuda_operator_addition_distinguishes_current_recursion_from_other_overloads() {
    validate("__device__ float operator+(float2 left, float right) { return left.x + right; } __device__ float operator+(float2 left, int right) { return left + float(right); } __global__ void kernel(float *out) { out[0] = make_float2(1, 2) + 3; }");
    for expression in ["left + right", "operator+(left, right)"] {
        let error = rejected(&format!("__device__ int operator+(int2 left, int right) {{ return {expression}; }} __global__ void kernel() {{}}"));
        assert!(error.message.contains("recurs"));
    }
}

#[test]
fn cuda_operator_addition_has_no_implicit_vector_fallback_or_vector_conversions() {
    for scalar in ["float", "int", "uint"] {
        for width in [2, 3, 4] {
            rejected(&format!("__global__ void kernel() {{ {scalar}{width} value = make_{scalar}{width}({}); value + value; }}", vec!["1"; width].join(", ")));
        }
    }
    for argument in [
        "make_uint3(1, 2, 3)",
        "make_float3(1, 2, 3)",
        "make_int2(1, 2)",
        "3",
    ] {
        rejected(&format!("{INT3_ADDITION} __global__ void kernel() {{ int3 value = make_int3(1, 2, 3); operator+(value, {argument}); }}"));
    }
}

#[test]
fn cuda_operator_addition_returns_prvalues_and_mutates_only_parameter_copies() {
    validate("__device__ int3 operator+(int3 left, int3 right) { left.x += right.x; left.y++; return left; } __global__ void kernel(int *out) { const int3 value = make_int3(1, 2, 3); out[0] = (value + value).x; out[1] = value.x; }");
    for statement in [
        "(value + value) = value;",
        "(value + value).x = 3;",
        "++(value + value).x;",
    ] {
        rejected(&format!("{INT3_ADDITION} __global__ void kernel() {{ int3 value = make_int3(1, 2, 3); {statement} }}"));
    }
    rejected("__device__ int2 operator+(const int2 left, int2 right) { ++left.x; return right; } __global__ void kernel() {}");
    validate("__device__ int operator+(float2 left, float2 right) { return left.x + right.y; } __global__ void kernel(int *out) { out[0] = make_float2(1, 2) + make_float2(3, 4); out[1] = operator+(make_float2(1, 2), make_float2(3, 4)) * 2 + 1; }");
}

#[test]
fn cuda_operator_addition_supports_void_only_in_discard_and_void_return_contexts() {
    let definition = "__device__ void operator+(int2 left, int2 right) {}";
    validate(&format!("{definition} __device__ void forward(int2 left, int2 right) {{ return left + right; }} __global__ void kernel() {{ int2 value = make_int2(1, 2); value + value; operator+(value, value); true ? (value + value) : (value + value); for (int index = 0; index < 2; value + value) {{ ++index; }} forward(value, value); }}"));
    for statement in [
        "int result = value + value;",
        "(value + value).x;",
        "if (value + value) return;",
        "operator+(value + value, value);",
    ] {
        rejected(&format!(
            "{definition} __global__ void kernel() {{ int2 value = make_int2(1, 2); {statement} }}"
        ));
    }
}

#[test]
fn cuda_operator_addition_remains_nonconstant_with_lazy_constant_eligibility() {
    let definition = "__device__ int operator+(int2 left, int2 right) { return 3; }";
    for declaration in [
        "constexpr int Count = value + value;",
        "const int Count = value + value; int storage[Count];",
    ] {
        rejected(&format!("{definition} __global__ void kernel() {{ int2 value = make_int2(1, 2); {declaration} }}"));
    }
    validate(&format!("{definition} __global__ void kernel(int *out) {{ int2 value = make_int2(1, 2); constexpr int Count = true ? 3 : operator+(value, value); int storage[Count]{{}}; out[0] = storage[2]; }}"));
}

#[test]
fn cuda_operator_addition_does_not_enable_neighboring_operator_syntax() {
    for expression in [
        "value += value",
        "+value",
        "-value",
        "value - value",
        "value * value",
        "value == value",
        "value++",
        "++value",
    ] {
        rejected(&format!("{INT3_ADDITION} __global__ void kernel() {{ int3 value = make_int3(1, 2, 3); {expression}; }}"));
    }
    for declaration in [
        "__global__ void operator+(int2 left, int2 right) {}",
        "__device__ int2 operator+(int2 *left, int2 right) { return right; }",
        "__device__ int2 operator+(int2 &left, int2 right) { return right; }",
        "__device__ int2 operator+(int2 left, int right = 2) { return left; }",
        "__device__ int2 operator+=(int2 left, int2 right) { return left; }",
    ] {
        rejected(&format!("{declaration} __global__ void kernel() {{}}"));
    }
    validate("__device__ int2 operator+(int2 left, int2 right); __global__ void kernel() {}");
    for declaration in [
        "__device__ int operator+(int2 value) { return 1; }",
        "__device__ int2 operator-(int2 left, int2 right) { return left; }",
        "__device__ int2 operator*(int2 left, int2 right) { return left; }",
    ] {
        validate(&format!("{declaration} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_operator_addition_uses_left_associative_call_results() {
    let (module, _) = validate(&format!("{INT3_ADDITION} __global__ void kernel(int *out) {{ int3 value = make_int3(1, 2, 3); out[0] = (value + value + value).x; }}"));
    let function = &module.entry_points[0].function;
    let mut emitted_calls = function
        .body
        .iter()
        .filter_map(|statement| match statement {
            naga::Statement::Call {
                arguments, result, ..
            } => Some((arguments, result)),
            _ => None,
        });
    let (_, first_result) = emitted_calls.next().unwrap();
    let (second_arguments, _) = emitted_calls.next().unwrap();
    assert_eq!(Some(second_arguments[0]), *first_result);
    assert!(emitted_calls.next().is_none());
}

#[test]
fn cuda_operator_addition_preserves_lazy_branches_and_conditional_references() {
    validate(&format!("{INT3_ADDITION} __global__ void kernel(int *out, unsigned choose) {{ int3 first = make_int3(1, 2, 3), second = make_int3(4, 5, 6); out[0] = ((choose ? first : second) + (choose ? second : first)).x; out[1] = (choose ? (first + second) : (second + first)).y; out[2] = choose && (first + second).z; out[3] = (first + second, second + first).x; }}"));
}

#[test]
fn cuda_operator_addition_preserves_inferred_arrays_and_helper_compute_inputs() {
    let (module, _) = validate("__device__ int3 operator+(int3 left, int3 right) { return make_int3(left.x + right.x + int(threadIdx.x), left.y + right.y, left.z + right.z); } __global__ void kernel(const int3 *input, int bias, int *out) { int3 values[]{input[0] + input[1], (values[0], make_int3(1, 2, 3))}; constexpr int Count = (values[1], 2); int storage[Count]{}; for (int index = 0; index < 2; ++index) { if (index == 0) continue; out[0] = (values[0] + values[1]).x + bias + storage[1]; } }");
    let bindings: Vec<_> = module
        .global_variables
        .iter()
        .filter_map(|(_, global)| global.binding)
        .collect();
    assert_eq!(
        bindings,
        (0..3)
            .map(|binding| naga::ResourceBinding { group: 0, binding })
            .collect::<Vec<_>>()
    );
}

#[test]
fn cuda_operator_addition_snapshots_left_operands_before_right_effects_by_frontend_policy() {
    for (declarations, left_operand, right_operand) in [
        (
            "int left = 1; int3 right = make_int3(2, 3, 4);",
            "left",
            "(left = 7, right)",
        ),
        (
            "int3 left = make_int3(1, 2, 3), right = make_int3(4, 5, 6);",
            "left",
            "(left.x = 7, right)",
        ),
        (
            "int3 left = make_int3(1, 2, 3), right = make_int3(4, 5, 6); bool choose = true;",
            "(choose ? left : right)",
            "(left.x = 7, right)",
        ),
        (
            "int index = 0; int3 values[2]{make_int3(1, 2, 3), make_int3(4, 5, 6)};",
            "values[index++]",
            "(index = 7, values[1])",
        ),
    ] {
        let (module, _) = validate(&format!("{INT3_ADDITION} __device__ int3 operator+(int left, int3 right) {{ return make_int3(left + right.x, right.y, right.z); }} __global__ void kernel(int *out) {{ {declarations} out[0] = ({left_operand} + {right_operand}).x; }}"));
        let function = &module.entry_points[0].function;
        let argument = calls(function).last().unwrap().1[0];
        assert!(matches!(
            function.expressions[argument],
            naga::Expression::Load { .. }
        ));
        let load_statement = function
            .body
            .iter()
            .position(|statement| match statement {
                naga::Statement::Emit(range) => range.clone().any(|handle| handle == argument),
                _ => false,
            })
            .unwrap();
        let mutation_statement = function
            .body
            .iter()
            .position(|statement| match *statement {
                naga::Statement::Store { value, .. } => matches!(
                    function.expressions[value],
                    naga::Expression::Literal(naga::Literal::I32(7))
                ),
                _ => false,
            })
            .unwrap();
        assert!(load_statement < mutation_statement);
    }
}

#[test]
fn cuda_operator_addition_keeps_flat_chains_linear_and_nested_calls_bounded() {
    let flat_chain = vec!["value"; 96].join(" + ");
    let (module, _) = validate(&format!("{INT3_ADDITION} __global__ void kernel(int *out) {{ int3 value = make_int3(1, 2, 3); out[0] = ({flat_chain}).x; }}"));
    assert_eq!(calls(&module.entry_points[0].function).len(), 95);
    let nested = format!("{}value{}", "operator+(value, ".repeat(88), ")".repeat(88));
    rejected(&format!(
        "{INT3_ADDITION} __global__ void kernel() {{ int3 value = make_int3(1, 2, 3); {nested}; }}"
    ));
}

#[test]
fn cuda_operator_addition_rejects_malformed_and_dead_calls_with_valid_spans() {
    for statement in [
        "operator+(value);",
        "operator+();",
        "operator+(value, value, value);",
        "operator+(value,);",
        "operator+(, value);",
        "operator+(value, value;",
        "return; operator+(value, missing);",
        "if (false) { operator+(value, missing); }",
    ] {
        rejected(&format!("/* λ */ {INT3_ADDITION} __global__ void kernel() {{ int3 value = make_int3(1, 2, 3); {statement} }}"));
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_operator_addition_roundtrips_without_changing_scalar_addition() {
    let (module, info) = validate(FIXTURE);
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    let (module, info) = validate("__device__ int operator+(int2 left, int2 right) { return 9; } __global__ void kernel(unsigned *out, int input) { constexpr int Count = 1 + 2 * 3; int storage[Count]{}; out[0] = true + true; out[1] = 0xffffffffu + 1u; out[2] = input + storage[6] + Count; }");
    assert!(calls(&module.entry_points[0].function).is_empty());
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}
