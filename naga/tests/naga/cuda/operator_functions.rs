use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/operator-functions.cu");
const BOX: &str = "struct Box { int value; };";
const BINARY: &[&str] = &["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>"];
const UNARY: &[&str] = &["+", "-", "*", "&", "~", "!"];

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty() && source.get(range).is_some(), "{error}");
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
fn cuda_operator_functions_support_all_forms_for_vectors_and_nominal_records() {
    for ty in [
        "int2", "int3", "int4", "uint2", "uint3", "uint4", "float2", "float3", "float4", "Box",
    ] {
        let mut source = BOX.to_owned();
        for symbol in UNARY {
            source.push_str(&format!(
                "__device__ {ty} operator{symbol}({ty} value) {{ return value; }}"
            ));
        }
        for symbol in BINARY {
            source.push_str(&format!(
                "__device__ {ty} operator{symbol}({ty} left, {ty} right) {{ return left; }}"
            ));
        }
        source.push_str(&format!("__global__ void kernel() {{ {ty} value{{}};"));
        for symbol in UNARY {
            source.push_str(&format!("{symbol}value;"));
        }
        for symbol in BINARY {
            source.push_str(&format!("value {symbol} value;"));
        }
        source.push('}');
        let (module, _) = validate(&source);
        assert_eq!(module.functions.len(), 16, "{ty}");
        assert_eq!(calls(&module.entry_points[0].function).len(), 16, "{ty}");
    }
}

#[test]
fn cuda_operator_functions_fixture_preserves_value_signatures_and_storage_access() {
    let (module, _) = validate(FIXTURE);
    assert_eq!(module.functions.len(), 20);
    assert!(module.functions.iter().all(|(_, function)| {
        (1..=2).contains(&function.arguments.len())
            && function
                .arguments
                .iter()
                .all(|argument| argument.binding.is_none())
    }));
    let storage_access: Vec<_> = module
        .global_variables
        .iter()
        .filter_map(|(_, global)| match global.space {
            naga::AddressSpace::Storage { access } => Some(access),
            _ => None,
        })
        .collect();
    assert_eq!(
        storage_access,
        [
            naga::StorageAccess::LOAD,
            naga::StorageAccess::LOAD | naga::StorageAccess::STORE
        ]
    );
}

#[test]
fn cuda_operator_functions_accept_arbitrary_supported_result_and_scalar_parameter_types() {
    for symbol in ["%", "&", "|", "^", "<<", ">>"] {
        for result in ["float", "int2", "Box", "bool"] {
            let value = match result {
                "float" => "2.5f",
                "int2" => "make_int2(2, 3)",
                "Box" => "Box{4}",
                _ => "true",
            };
            validate(&format!("{BOX} __device__ {result} operator{symbol}(Box left, float right) {{ return {value}; }} __global__ void kernel() {{ Box{{1}} {symbol} 2.0f; }}"));
        }
    }
    for symbol in ["!", "~", "*", "&"] {
        validate(&format!("{BOX} __device__ Box operator{symbol}(Box value) {{ return value; }} __global__ void kernel(int *out) {{ out[0] = ({symbol}Box{{7}}).value; }}"));
    }
}

#[test]
fn cuda_operator_functions_share_names_without_inventing_other_arities() {
    for symbol in ["+", "-", "*", "&"] {
        let unary = format!("__device__ Box operator{symbol}(Box value) {{ return value; }}");
        let binary =
            format!("__device__ Box operator{symbol}(Box left, Box right) {{ return left; }}");
        validate(&format!("{BOX} {unary} {binary} __global__ void kernel() {{ Box value{{1}}; {symbol}value; value {symbol} value; operator{symbol}(value); operator{symbol}(value, value); }}"));
        rejected(&format!(
            "{BOX} {unary} __global__ void kernel() {{ Box value{{1}}; value {symbol} value; }}"
        ));
        rejected(&format!(
            "{BOX} {binary} __global__ void kernel() {{ Box value{{1}}; {symbol}value; }}"
        ));
    }
}

#[test]
fn cuda_operator_functions_reject_invalid_arity_and_scalar_only_signatures() {
    for symbol in ["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>", "~", "!"] {
        for parameters in [
            "",
            "Box first, Box second, Box third",
            "int value",
            "float left, unsigned right",
        ] {
            let source = format!("{BOX} __device__ int operator{symbol}({parameters}) {{ return 1; }} __global__ void kernel() {{}}");
            rejected(&source);
        }
    }
    for symbol in ["/", "%", "|", "^", "<<", ">>"] {
        rejected(&format!("{BOX} __device__ int operator{symbol}(Box value) {{ return 1; }} __global__ void kernel() {{}}"));
    }
    for symbol in ["!", "~"] {
        rejected(&format!("{BOX} __device__ int operator{symbol}(Box left, Box right) {{ return 1; }} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_operator_functions_canonicalize_token_whitespace_and_comments() {
    for symbol in ["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>", "~", "!"] {
        let parameters = if ["~", "!"].contains(&symbol) {
            "Box value"
        } else {
            "Box value, Box other"
        };
        let arguments = if ["~", "!"].contains(&symbol) {
            "value"
        } else {
            "value, value"
        };
        for gap in ["", " ", " /* λ */ "] {
            validate(&format!("{BOX} __device__ Box operator{gap}{symbol}({parameters}) {{ return value; }} __global__ void kernel() {{ Box value{{1}}; operator{gap}{symbol}({arguments}); }}"));
        }
    }
    rejected(&format!("{BOX} __device__ Box operator < <(Box left, Box right) {{ return left; }} __global__ void kernel() {{}}"));
}

#[test]
fn cuda_operator_functions_preserve_complete_operator_id_diagnostic_spans() {
    for symbol in ["-", "*", "/", "%", "&", "|", "^", "<<", ">>", "~", "!"] {
        let parameters = if ["~", "!"].contains(&symbol) {
            "Box value"
        } else {
            "Box value, Box other"
        };
        let source = format!("/* λ */ {BOX} __device__ int operator{symbol}({parameters}) {{ return 1; }} __device__ float operator{symbol}({parameters}) {{ return 2.0f; }} __global__ void kernel() {{}}");
        let error = rejected(&source);
        assert!(error.message.contains("duplicate"), "{error}");
        assert_eq!(
            &source[error.span.to_range().unwrap()],
            format!("operator{symbol}")
        );
    }
}

#[test]
fn cuda_operator_functions_leave_mutating_and_comparison_operators_unsupported() {
    for symbol in [
        "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "==", "!=", "<", "<=", ">",
        ">=", ",", "=",
    ] {
        rejected(&format!("{BOX} __device__ Box operator{symbol}(Box left, Box right) {{ return left; }} __global__ void kernel() {{}}"));
    }
    for symbol in ["&&", "||"] {
        validate(&format!("{BOX} __device__ Box operator{symbol}(Box left, Box right) {{ return left; }} __global__ void kernel() {{}}"));
    }
    for symbol in ["++", "--"] {
        rejected(&format!("{BOX} __device__ Box operator{symbol}(Box value) {{ return value; }} __global__ void kernel() {{}}"));
    }
    for expression in [
        "value += value",
        "value++",
        "++value",
        "value && value",
        "value || value",
        "value == value",
    ] {
        rejected(&format!("{BOX} __device__ Box operator+(Box left, Box right) {{ return left; }} __global__ void kernel() {{ Box value{{1}}; {expression}; }}"));
    }
}

#[test]
fn cuda_operator_functions_do_not_change_scalar_builtin_operations() {
    let mut source = BOX.to_owned();
    for symbol in BINARY {
        source.push_str(&format!(
            "__device__ Box operator{symbol}(Box left, Box right) {{ return left; }}"
        ));
    }
    for symbol in UNARY {
        source.push_str(&format!(
            "__device__ Box operator{symbol}(Box value) {{ return value; }}"
        ));
    }
    source.push_str("__global__ void kernel(unsigned *out, int input) { constexpr int Count = 1 + 2 * 3; int values[Count]{}; out[0] = input + 3; out[1] = input - 3; out[2] = input * 3; out[3] = input / 3; out[4] = input % 3; out[5] = input & 3; out[6] = input | 3; out[7] = input ^ 3; out[8] = input << 2; out[9] = input >> 2; out[10] = +input; out[11] = -input; out[12] = ~input; out[13] = !input; out[14] = true + true; out[15] = 0xffffffffu + 1u; out[16] = values[6]; }");
    let (module, _) = validate(&source);
    assert!(calls(&module.entry_points[0].function).is_empty());
    for symbol in ["*", "&"] {
        rejected(&format!("{BOX} __device__ Box operator{symbol}(Box value) {{ return value; }} __global__ void kernel(int input) {{ {symbol}input; }}"));
    }
}

#[test]
fn cuda_operator_functions_use_ordinary_scalar_conversion_ranking() {
    for symbol in ["-", "*", "/", "%", "&", "|", "^", "<<", ">>"] {
        let (module, _) = validate(&format!("{BOX}
            __device__ int operator{symbol}(Box left, bool right) {{ return 1; }}
            __device__ int operator{symbol}(Box left, int right) {{ return 2; }}
            __device__ int operator{symbol}(Box left, unsigned right) {{ return 3; }}
            __device__ int operator{symbol}(Box left, float right) {{ return 4; }}
            __global__ void kernel(int *out) {{ Box value{{5}}; out[0] = value {symbol} true; out[1] = value {symbol} 3; out[2] = value {symbol} 3u; out[3] = value {symbol} 3.0f; }}"));
        let emitted = calls(&module.entry_points[0].function);
        assert_eq!(emitted.len(), 4);
        for ((selected, _), expected) in emitted.into_iter().zip([
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
    }
    let (module, _) = validate(&format!("{BOX} __device__ int operator-(Box left, int right) {{ return 1; }} __device__ int operator-(Box left, float right) {{ return 2; }} __global__ void kernel() {{ Box{{1}} - true; }}"));
    let selected = calls(&module.entry_points[0].function)[0].0;
    assert_eq!(
        module.types[module.functions[selected].arguments[1].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::I32)
    );
}

#[test]
fn cuda_operator_functions_reject_ambiguity_and_nominal_type_mismatches() {
    assert!(rejected(&format!("{BOX} __device__ int operator/(Box left, int right) {{ return 1; }} __device__ int operator/(Box left, bool right) {{ return 2; }} __global__ void kernel() {{ Box{{1}} / 3u; }}")).message.contains("ambiguous"));
    for source in [
        "struct First { int value; }; struct Second { int value; }; __device__ int operator*(First left, First right) { return 1; } __global__ void kernel() { Second value{1}; value * value; }",
        "__device__ int operator^(int2 left, int2 right) { return 1; } __global__ void kernel() { float2 value = make_float2(1.0f, 2.0f); value ^ value; }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_operator_functions_merge_aliases_and_respect_definition_constness() {
    validate(&format!("{BOX} using Value = Box; using Fixed = const Value; __device__ int operator-(Fixed); __device__ int operator-(Box); __global__ void kernel(int *out) {{ const Value value{{3}}; out[0] = -value; }} __device__ int operator-(Value value) {{ return ++value.value; }}"));
    rejected(&format!("{BOX} __device__ int operator*(Box); __global__ void kernel() {{ *Box{{3}}; }} __device__ int operator*(const Box value) {{ return ++value.value; }}"));
    for header in [
        "static __device__ inline",
        "__host__ __device__",
        "__device__ __forceinline__",
        "__device__ __noinline__",
    ] {
        validate(&format!("{BOX} {header} Box operator&(Box value) {{ return value; }} __global__ void kernel() {{ &Box{{3}}; }}"));
    }
}

#[test]
fn cuda_operator_functions_copy_const_records_arrays_and_temporary_values() {
    validate("struct Record { int value; int values[2]; }; __device__ Record operator*(Record value) { ++value.value; value.values[1] += 3; return value; } __device__ int operator&(Record value) { return value.value + value.values[1]; } __global__ void kernel(int *out) { const Record original{2, {3, 4}}; Record result = *original; out[0] = &*original; out[1] = (&Record{3, {4, 5}}); out[2] = result.values[1]; out[3] = original.values[1]; out[4] = operator&(*Record{5, {6, 7}}); }");
}

#[test]
fn cuda_operator_functions_accept_void_discarded_comma_and_conditional_calls() {
    for symbol in ["!", "~", "*", "&"] {
        validate(&format!("{BOX} __device__ void operator{symbol}(Box value) {{ value.value; }} __global__ void kernel(int *out) {{ {symbol}Box{{1}}; operator{symbol}(Box{{2}}); true ? {symbol}Box{{3}} : {symbol}Box{{4}}; out[0] = ({symbol}Box{{5}}, 7); }}"));
        for use_value in [
            format!("int value = {symbol}Box{{1}};"),
            format!("if ({symbol}Box{{1}}) {{}}"),
            format!("int values[1]{{{symbol}Box{{1}}}};"),
        ] {
            rejected(&format!("{BOX} __device__ void operator{symbol}(Box value) {{}} __global__ void kernel() {{ {use_value} }}"));
        }
    }
    validate(&format!("{BOX} __device__ void operator%(Box left, int right) {{}} __global__ void kernel(int *out) {{ Box{{1}} % 2; out[0] = (operator%(Box{{1}}, 3), 7); }}"));
}

#[test]
fn cuda_operator_functions_class_prefix_address_and_dereference_are_value_calls() {
    let (module, _) = validate(&format!("{BOX} __device__ Box operator*(Box value) {{ ++value.value; return value; }} __device__ Box operator&(Box value) {{ value.value *= 2; return value; }} __global__ void kernel(int *out) {{ const Box value{{3}}; out[0] = (*&value).value; out[1] = (&*value).value; out[2] = (&Box{{5}}).value; out[3] = (*Box{{5}}).value; }}"));
    for (_, function) in module.functions.iter() {
        assert!(function.arguments.iter().all(|argument| matches!(
            module.types[argument.ty].inner,
            naga::TypeInner::Struct { .. }
        )));
        assert!(matches!(
            module.types[function.result.as_ref().unwrap().ty].inner,
            naga::TypeInner::Struct { .. }
        ));
    }
    assert_eq!(calls(&module.entry_points[0].function).len(), 6);
    for target in ["(*value).value", "(&value).value"] {
        rejected(&format!("{BOX} __device__ Box operator*(Box value) {{ return value; }} __device__ Box operator&(Box value) {{ return value; }} __global__ void kernel() {{ Box value{{1}}; {target} = 7; }}"));
    }
}

#[test]
fn cuda_operator_functions_do_not_enable_pointer_reference_or_member_signatures() {
    for declaration in [
        "__device__ Box operator*(Box *value) { return value[0]; }",
        "__device__ Box operator&(Box &value) { return value; }",
        "__device__ Box *operator&(Box value) { return 0; }",
        "__device__ Box &operator*(Box value) { return value; }",
        "__device__ Box operator/(Box value, int scale = 2) { return value; }",
        "__global__ void operator-(Box value) {}",
    ] {
        rejected(&format!(
            "{BOX} {declaration} __global__ void kernel() {{}}"
        ));
    }
    rejected("struct Box { int value; __device__ int operator*() { return value; } }; __global__ void kernel() {}");
}

#[test]
fn cuda_operator_functions_resolve_forward_declarations_and_late_definitions() {
    let (module, _) = validate(&format!("{BOX} __device__ Box operator-(Box); __device__ Box operator*(Box, int); __device__ Box transform(Box value) {{ return -value * 3; }} __global__ void kernel(int *out) {{ out[0] = transform(Box{{2}}).value; }} __device__ Box operator-(Box value) {{ value.value = -value.value; return value; }} __device__ Box operator*(Box value, int scale) {{ value.value *= scale; return value; }}"));
    for (caller, function) in module.functions.iter() {
        assert!(calls(function)
            .iter()
            .all(|(callee, _)| callee.index() < caller.index()));
    }
    rejected(&format!("{BOX} __device__ int earlier(Box value) {{ return *value; }} __device__ int operator*(Box value) {{ return value.value; }} __global__ void kernel() {{}}"));
}

#[test]
fn cuda_operator_functions_bind_overloads_at_each_source_call() {
    let (module, _) = validate(&format!("{BOX} __device__ int operator<<(Box left, int right); __device__ int earlier(unsigned right) {{ return Box{{1}} << right; }} __device__ int operator<<(Box left, unsigned right); __global__ void kernel(int *out) {{ out[0] = earlier(3u); out[1] = Box{{1}} << 3u; }} __device__ int operator<<(Box left, int right) {{ return 10 + right; }} __device__ int operator<<(Box left, unsigned right) {{ return 20 + int(right); }}"));
    let earlier = module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some("earlier"))
        .unwrap()
        .1;
    let earlier_selected = calls(earlier)[0].0;
    let later_selected = calls(&module.entry_points[0].function)[1].0;
    assert_eq!(
        module.types[module.functions[earlier_selected].arguments[1].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::I32)
    );
    assert_eq!(
        module.types[module.functions[later_selected].arguments[1].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::U32)
    );
}

#[test]
fn cuda_operator_functions_preserve_nominal_local_scopes_and_global_declarations() {
    validate(&format!("{BOX} __device__ int operator~(Box value) {{ return value.value; }} __global__ void kernel(int *out) {{ Box outer{{1}}; {{ struct Box {{ int value; }}; Box local{{2}}; out[0] = local.value; }} out[1] = ~outer; }}"));
    for source in [
        "struct Box { int value; }; __device__ int operator~(Box value) { return value.value; } __global__ void kernel() { struct Box { int value; }; Box local{2}; ~local; }",
        "__device__ int operator~(int2 value) { return value.x; } __global__ void kernel() { struct int2 { int x; int y; }; int2 local{2, 3}; ~local; }",
        "struct Box { int value; }; __global__ void kernel() { __device__ int operator~(Box); }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_operator_functions_reject_called_missing_definitions_even_in_dead_callers() {
    validate(&format!("{BOX} __device__ Box operator~(Box); __device__ Box operator/(Box, Box); __global__ void kernel() {{}}"));
    for statement in [
        "*value;",
        "if (false) { *value; }",
        "return; *value;",
        "false && (*value, true);",
        "true ? 1 : (*value, 2);",
    ] {
        rejected(&format!("{BOX} __device__ int operator*(Box); __device__ void unused() {{ Box value{{1}}; {statement} }} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_operator_functions_reject_direct_mutual_and_mixed_call_cycles() {
    for definitions in [
        "__device__ Box operator-(Box value) { return -value; }",
        "__device__ Box operator-(Box); __device__ Box operator~(Box); __device__ Box operator-(Box value) { return ~value; } __device__ Box operator~(Box value) { return -value; }",
        "__device__ Box helper(Box); __device__ Box operator*(Box value) { return helper(value); } __device__ Box helper(Box value) { return *value; }",
        "__device__ Box operator-(Box); __device__ Box operator~(Box); __device__ Box operator-(Box value) { if (false) return ~value; return value; } __device__ Box operator~(Box value) { return -value; }",
    ] {
        rejected(&format!("{BOX} {definitions} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_operator_functions_remap_generated_math_and_late_compute_helpers() {
    let (module, _) = validate(&format!("{BOX} __device__ float operator~(Box); __device__ int operator*(Box); __global__ void kernel(unsigned *out) {{ out[0] = __float_as_uint(~Box{{4}}); out[1] = *Box{{2}}; }} __device__ float operator~(Box value) {{ return sqrtf(float(value.value)) + sinf(0.0f) + cosf(0.0f); }} __device__ int operator*(Box value) {{ return value.value + int(threadIdx.x + blockIdx.y + blockDim.z + gridDim.x); }}"));
    for (caller, function) in module.functions.iter() {
        assert!(calls(function)
            .iter()
            .all(|(callee, _)| callee.index() < caller.index()));
        for (_, expression) in function.expressions.iter() {
            if let naga::Expression::CallResult(callee) = expression {
                assert!(callee.index() < caller.index());
            }
        }
    }
}

#[test]
fn cuda_operator_functions_preserve_precedence_and_left_associativity() {
    let (module, _) = validate(&format!("{BOX} __device__ Box operator-(Box left, Box right) {{ return left; }} __device__ Box operator*(Box left, Box right) {{ return right; }} __device__ Box operator<<(Box left, Box right) {{ return left; }} __global__ void kernel() {{ Box value{{1}}; value - value - value; value - value * value; value << value - value; }}"));
    let function = &module.entry_points[0].function;
    let emitted = calls(function);
    let selected: Vec<_> = emitted
        .iter()
        .map(|(handle, _)| module.functions[*handle].name.as_deref().unwrap())
        .collect();
    assert_eq!(
        selected,
        [
            "operator-",
            "operator-",
            "operator*",
            "operator-",
            "operator-",
            "operator<<"
        ]
    );
    assert!(
        matches!(function.expressions[emitted[1].1[0]], naga::Expression::CallResult(handle) if handle == emitted[0].0)
    );
}

#[test]
fn cuda_operator_functions_snapshot_shift_left_operands_before_right_effects() {
    for (declarations, left, right, first_type) in [
        (
            "int left = 1; Box right{2};",
            "left",
            "(left = 7, right)",
            "int",
        ),
        ("Box left{3};", "left", "(left.value = 7)", "Box"),
        (
            "Box left{3}, other{4}; bool choose = true;",
            "(choose ? left : other)",
            "(left.value = 7)",
            "Box",
        ),
        (
            "int index = 0; Box values[2]{Box{3}, Box{4}};",
            "values[index++]",
            "(index = 7)",
            "Box",
        ),
    ] {
        let second_type = if first_type == "int" { "Box" } else { "int" };
        let (module, _) = validate(&format!("{BOX} __device__ int operator<<({first_type} left, {second_type} right) {{ return 1; }} __global__ void kernel(int *out) {{ {declarations} out[0] = {left} << {right}; }}"));
        let function = &module.entry_points[0].function;
        let argument = calls(function).last().unwrap().1[0];
        let emitted = function
            .body
            .iter()
            .position(|statement| match statement {
                naga::Statement::Emit(range) => range.clone().any(|handle| handle == argument),
                _ => false,
            })
            .unwrap();
        let mutation = function
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
        assert!(emitted < mutation);
    }
}

#[test]
fn cuda_operator_functions_explicit_calls_keep_the_frontend_argument_order() {
    let (module, _) = validate(&format!("{BOX} __device__ int operator<<(Box left, int right) {{ return left.value * 100 + right; }} __global__ void kernel(int *out) {{ Box left{{3}}; out[0] = operator<<(left, (left.value = 7)); }}"));
    let function = &module.entry_points[0].function;
    let argument = calls(function)[0].1[0];
    let emitted = function
        .body
        .iter()
        .position(|statement| match statement {
            naga::Statement::Emit(range) => range.clone().any(|handle| handle == argument),
            _ => false,
        })
        .unwrap();
    let mutation = function
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
    assert!(emitted < mutation);
}

#[test]
fn cuda_operator_functions_preserve_lazy_branches_arrays_and_conditional_operands() {
    validate(&format!("{BOX} __device__ int operator*(Box value) {{ return value.value; }} __device__ Box operator&(Box value) {{ return value; }} __device__ Box operator/(Box left, Box right) {{ return left; }} __global__ void kernel(int *out, unsigned choose) {{ int index = 0; Box values[]{{Box{{2}}, Box{{3}}}}; out[0] = *values[index++]; out[1] = *(choose ? values[0] : values[1]); out[2] = (choose ? &values[0] : &values[1]).value; out[3] = false && *Box{{index++}}; out[4] = true || *Box{{index++}}; out[5] = ((choose ? values[0] : values[1]) / values[0]).value; }}"));
}

#[test]
fn cuda_operator_functions_remain_calls_in_constant_expression_contexts() {
    validate(&format!("{BOX} __device__ int operator*(Box value) {{ return value.value; }} __global__ void kernel() {{ constexpr int Count = true ? 3 : *Box{{1}}; int values[Count]{{}}; constexpr bool Skip = false && *Box{{2}}; constexpr int Also = (Skip ? *Box{{1}} : 2); int other[Also]{{}}; }}"));
    for declaration in [
        "constexpr int Count = *Box{3};",
        "int values[*Box{3}]{};",
        "switch (1) { case *Box{3}: break; }",
    ] {
        rejected(&format!("{BOX} __device__ int operator*(Box value) {{ return value.value; }} __global__ void kernel() {{ {declaration} }}"));
    }
    for expression in ["false && *missing", "true ? 1 : *missing"] {
        rejected(&format!("{BOX} __device__ int operator*(Box value) {{ return value.value; }} __global__ void kernel() {{ {expression}; }}"));
    }
}

#[test]
fn cuda_operator_functions_reject_malformed_unicode_calls_and_bound_expression_depth() {
    for statement in [
        "operator*();",
        "operator*(value,);",
        "operator*(,value);",
        "operator*(value;",
        "operator<< value;",
        "operator<<();",
        "return; operator*(missing);",
        "if (false) { operator*(missing); }",
    ] {
        rejected(&format!("/* λ */ {BOX} __device__ Box operator*(Box value) {{ return value; }} __global__ void kernel() {{ Box value{{1}}; {statement} }}"));
    }
    let chain = vec!["value"; 96].join(" ^ ");
    let (module, _) = validate(&format!("{BOX} __device__ Box operator^(Box left, Box right) {{ return left; }} __global__ void kernel() {{ Box value{{1}}; {chain}; }}"));
    assert_eq!(calls(&module.entry_points[0].function).len(), 95);
    let nested = format!("{}value{}", "operator~(".repeat(88), ")".repeat(88));
    rejected(&format!("{BOX} __device__ Box operator~(Box value) {{ return value; }} __global__ void kernel() {{ Box value{{1}}; {nested}; }}"));
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_operator_functions_fixture_roundtrips_through_shared_wgsl_helpers() {
    let (module, info) = validate(FIXTURE);
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}

#[cfg(spv_out)]
#[cfg_attr(miri, ignore)]
#[test]
fn cuda_operator_functions_fixture_emits_valid_spirv_binary() {
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
        entry_point: "operator_functions".to_string(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
    crate::helpers::validate_spirv(&words);
}

#[test]
fn cuda_operator_functions_accept_record_temporaries_as_expression_statements() {
    for statement in [
        "Box{counter++};",
        "Alias{counter++};",
        "if (counter == 0) Box{counter++}; else Alias{counter++};",
        "while (counter < 2) Box{counter++};",
        "do Alias{counter++}; while (counter < 2);",
        "for (int iteration = 0; iteration < 2; ++iteration) Box{counter++};",
        "for (Alias{counter++}; counter < 2; ++counter) {}",
        "for (Box{counter++}; false;) { ++counter; }",
        "return; Box{counter++};",
    ] {
        validate(&format!("{BOX} using Alias = Box; __global__ void kernel(int *out) {{ int counter = 0; {statement} }}"));
    }
    validate(&format!("{BOX} using Alias = Box; __global__ void kernel(int *out) {{ Box named{{1}}; Alias copied = named; const Box frozen{{2}}; Box values[2]{{Box{{3}}, Box{{4}}}}; out[0] = copied.value + frozen.value + values[1].value; }}"));
    validate("__global__ void kernel() { int counter = 0; int(counter++); float(counter++); unsigned(counter++); for (int(counter++) + 1; counter < 6; int(counter++)) {} }");
    for statement in [
        "int(value);",
        "int((value));",
        "int(out[3]);",
        "int(value) = 1;",
        "Box(value);",
    ] {
        rejected(&format!(
            "{BOX} __global__ void kernel(int *out) {{ int value = 3; {statement} }}"
        ));
    }
}

#[test]
fn cuda_operator_functions_distinguish_grouped_record_values_from_cast_types() {
    for statement in [
        "(Box{counter++});",
        "((Alias{counter++}));",
        "out[0] = (Box{3}).value;",
        "out[0] = ((Alias{4})).value;",
        "out[0] = read((Box{counter++}));",
        "out[0] = (Box{counter++}, counter);",
        "(Box{counter++}, Alias{counter++});",
        "(Alias{counter++}).value;",
        "for ((Alias{counter++}); counter < 2; ++counter) {}",
        "if (true) (Box{counter++}); else (Alias{counter++});",
    ] {
        validate(&format!("{BOX} using Alias = Box; __device__ int read(Box value) {{ return value.value; }} __global__ void kernel(int *out) {{ int counter = 0; {statement} }}"));
    }
    validate("using Whole = int; __global__ void kernel(int *out) { int counter = 1; out[0] = (int)(counter + 1); out[1] = (Whole)counter; out[2] = int(counter++); out[3] = int((float)counter); out[4] = (unsigned)(counter); }");
    for statement in [
        "(Box)value;",
        "(Alias)Box{2};",
        "(int2)make_int2(1, 2);",
        "(Vector)make_float2(1.0f, 2.0f);",
    ] {
        rejected(&format!("{BOX} using Alias = Box; using Vector = float2; __global__ void kernel() {{ Box value{{1}}; {statement} }}"));
    }
}
