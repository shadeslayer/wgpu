use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/logical-operators.cu");
const BOX: &str = "struct Box { int value; };";

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
fn cuda_logical_operators_support_all_vector_types_and_nominal_records() {
    for ty in [
        "int2", "int3", "int4", "uint2", "uint3", "uint4", "float2", "float3", "float4", "Box",
    ] {
        let (module, _) = validate(&format!("{BOX} __device__ {ty} operator&&({ty} left, {ty} right) {{ return left; }} __device__ {ty} operator||({ty} left, {ty} right) {{ return right; }} __global__ void kernel() {{ {ty} value{{}}; value && value; value || value; }}"));
        assert_eq!(module.functions.len(), 2);
        assert_eq!(calls(&module.entry_points[0].function).len(), 2);
        assert!(module
            .functions
            .iter()
            .all(|(_, function)| function.arguments.len() == 2));
    }
}

#[test]
fn cuda_logical_operators_fixture_keeps_record_array_copies_and_value_signatures() {
    let (module, _) = validate(FIXTURE);
    assert_eq!(module.functions.len(), 6);
    assert!(module
        .functions
        .iter()
        .all(|(_, function)| function.arguments.len() == 2
            && function
                .arguments
                .iter()
                .all(|argument| argument.binding.is_none())));
    assert!(module.types.iter().any(|(_, ty)| matches!(ty.inner, naga::TypeInner::Struct { ref members, .. } if members.iter().any(|member| matches!(module.types[member.ty].inner, naga::TypeInner::Array { .. })))));
}

#[test]
fn cuda_logical_operators_accept_scalar_and_class_operands_on_either_side() {
    for symbol in ["&&", "||"] {
        for (ty, value) in [
            ("bool", "false"),
            ("int", "2"),
            ("unsigned", "3u"),
            ("float", "2.5f"),
        ] {
            validate(&format!("{BOX} __device__ int operator{symbol}({ty} left, Box right) {{ return 7; }} __device__ int operator{symbol}(Box left, {ty} right) {{ return 9; }} __global__ void kernel(int *out) {{ out[0] = {value} {symbol} Box{{1}}; out[1] = Box{{2}} {symbol} {value}; }}"));
        }
    }
}

#[test]
fn cuda_logical_operators_preserve_original_scalar_types_for_overload_ranking() {
    for symbol in ["&&", "||"] {
        for class_first in [false, true] {
            let mut source = BOX.to_owned();
            for (ty, result) in [("bool", 1), ("int", 2), ("unsigned", 3), ("float", 4)] {
                let parameters = if class_first {
                    format!("Box left, {ty} right")
                } else {
                    format!("{ty} left, Box right")
                };
                source.push_str(&format!(
                    "__device__ int operator{symbol}({parameters}) {{ return {result}; }}"
                ));
            }
            source.push_str("__global__ void kernel(int *out) {");
            for (index, value) in ["false", "2", "3u", "2.5f"].into_iter().enumerate() {
                let expression = if class_first {
                    format!("Box{{1}} {symbol} {value}")
                } else {
                    format!("{value} {symbol} Box{{1}}")
                };
                source.push_str(&format!("out[{index}] = {expression};"));
            }
            source.push('}');
            let (module, _) = validate(&source);
            let emitted = calls(&module.entry_points[0].function);
            assert_eq!(emitted.len(), 4);
            for ((selected, _), scalar) in emitted.into_iter().zip([
                naga::Scalar::BOOL,
                naga::Scalar::I32,
                naga::Scalar::U32,
                naga::Scalar::F32,
            ]) {
                let parameter = usize::from(class_first);
                assert_eq!(
                    module.types[module.functions[selected].arguments[parameter].ty].inner,
                    naga::TypeInner::Scalar(scalar)
                );
            }
        }
    }
}

#[test]
fn cuda_logical_operators_reject_ambiguous_scalar_conversions_and_nominal_mismatches() {
    for symbol in ["&&", "||"] {
        for class_first in [false, true] {
            let parameters = if class_first {
                ["Box left, int right", "Box left, bool right"]
            } else {
                ["int left, Box right", "bool left, Box right"]
            };
            let expression = if class_first {
                format!("Box{{1}} {symbol} 3u")
            } else {
                format!("3u {symbol} Box{{1}}")
            };
            let error = rejected(&format!("{BOX} __device__ int operator{symbol}({}) {{ return 1; }} __device__ int operator{symbol}({}) {{ return 2; }} __global__ void kernel() {{ {expression}; }}", parameters[0], parameters[1]));
            assert!(error.message.contains("ambiguous"), "{error}");
        }
        rejected(&format!("struct First {{ int value; }}; struct Second {{ int value; }}; __device__ int operator{symbol}(First left, First right) {{ return 1; }} __global__ void kernel() {{ Second value{{1}}; value {symbol} value; }}"));
        rejected(&format!("__device__ int operator{symbol}(int2 left, int2 right) {{ return 1; }} __global__ void kernel() {{ float2 value = make_float2(1.0f, 2.0f); value {symbol} value; }}"));
    }
}

#[test]
fn cuda_logical_operators_merge_aliases_and_respect_definition_constness() {
    for symbol in ["&&", "||"] {
        validate(&format!("{BOX} using Value = Box; using Fixed = const Value; __device__ int operator{symbol}(Fixed, Fixed); __device__ int operator{symbol}(Box, Box); __global__ void kernel(int *out) {{ const Value value{{3}}; out[0] = value {symbol} value; }} __device__ int operator{symbol}(Value left, Value right) {{ return ++left.value + right.value; }}"));
        rejected(&format!("{BOX} __device__ int operator{symbol}(Box, Box); __global__ void kernel() {{ Box{{1}} {symbol} Box{{2}}; }} __device__ int operator{symbol}(const Box left, Box right) {{ return ++left.value + right.value; }}"));
        for header in [
            "static __device__ inline",
            "__host__ __device__",
            "__device__ __forceinline__",
            "__device__ __noinline__",
        ] {
            validate(&format!("{BOX} {header} Box operator{symbol}(Box left, Box right) {{ return left; }} __global__ void kernel() {{ Box{{1}} {symbol} Box{{2}}; }}"));
        }
    }
}

#[test]
fn cuda_logical_operators_allow_arbitrary_supported_value_results() {
    for symbol in ["&&", "||"] {
        for (ty, result) in [
            ("int", "7"),
            ("unsigned", "8u"),
            ("float", "2.5f"),
            ("bool", "false"),
            ("int2", "make_int2(3, 4)"),
            ("Box", "Box{5}"),
        ] {
            validate(&format!("{BOX} __device__ {ty} operator{symbol}(Box left, Box right) {{ return {result}; }} __global__ void kernel() {{ {ty} result = Box{{1}} {symbol} Box{{2}}; }}"));
        }
    }
    validate("struct Record { int value; int values[2]; }; __device__ Record operator&&(Record left, Record right) { left.values[1] += right.values[0]; return left; } __global__ void kernel(int *out) { const Record original{2, {3, 4}}; Record result = original && Record{5, {6, 7}}; out[0] = result.values[1]; out[1] = original.values[1]; }");
}

#[test]
fn cuda_logical_operators_support_void_discard_comma_return_and_conditional_contexts() {
    for symbol in ["&&", "||"] {
        validate(&format!("{BOX} __device__ void operator{symbol}(Box left, Box right) {{}} __device__ void forward(Box left, Box right) {{ return left {symbol} right; }} __global__ void kernel(int *out) {{ Box{{1}} {symbol} Box{{2}}; operator{symbol}(Box{{3}}, Box{{4}}); out[0] = (Box{{5}} {symbol} Box{{6}}, 7); true ? (Box{{1}} {symbol} Box{{2}}) : (Box{{3}} {symbol} Box{{4}}); for (Box{{1}} {symbol} Box{{2}}; false; Box{{3}} {symbol} Box{{4}}) {{}} forward(Box{{1}}, Box{{2}}); }}"));
    }
}

#[test]
fn cuda_logical_operators_reject_void_values_even_in_skipped_builtin_operands() {
    for symbol in ["&&", "||"] {
        for statement in [
            format!("int result = Box{{1}} {symbol} Box{{2}};"),
            format!("if (Box{{1}} {symbol} Box{{2}}) {{}}"),
            format!("false && (Box{{1}} {symbol} Box{{2}});"),
            format!("true || (Box{{1}} {symbol} Box{{2}});"),
            format!("(Box{{1}} {symbol} Box{{2}}) && true;"),
        ] {
            rejected(&format!("{BOX} __device__ void operator{symbol}(Box left, Box right) {{}} __global__ void kernel() {{ {statement} }}"));
        }
    }
}

#[test]
fn cuda_logical_operators_require_two_value_parameters_and_a_source_class() {
    for symbol in ["&&", "||"] {
        for parameters in [
            "",
            "Box value",
            "Box first, Box second, Box third",
            "int left, int right",
            "bool left, float right",
        ] {
            rejected(&format!("{BOX} __device__ int operator{symbol}({parameters}) {{ return 1; }} __global__ void kernel() {{}}"));
        }
        for declaration in [
            format!("__device__ int operator{symbol}(Box *left, Box right) {{ return 1; }}"),
            format!("__device__ int operator{symbol}(Box &left, Box right) {{ return 1; }}"),
            format!("__device__ int operator{symbol}(Box left, int right = 2) {{ return 1; }}"),
            format!("__global__ void operator{symbol}(Box left, Box right) {{}}"),
        ] {
            rejected(&format!(
                "{BOX} {declaration} __global__ void kernel() {{}}"
            ));
        }
    }
}

#[test]
fn cuda_logical_operators_canonicalize_ids_and_preserve_duplicate_diagnostic_spans() {
    for symbol in ["&&", "||"] {
        for gap in ["", " ", " /* λ */ "] {
            let spelling = format!("operator{gap}{symbol}");
            validate(&format!("{BOX} __device__ Box {spelling}(Box left, Box right) {{ return left; }} __global__ void kernel() {{ {spelling}(Box{{1}}, Box{{2}}); }}"));
            let source = format!("/* λ */ {BOX} __device__ int {spelling}(Box left, Box right) {{ return 1; }} __device__ float {spelling}(Box left, Box right) {{ return 2.0f; }} __global__ void kernel() {{}}");
            let error = rejected(&source);
            assert!(error.message.contains("duplicate"), "{error}");
            assert_eq!(&source[error.span.to_range().unwrap()], spelling);
        }
    }
    for spelling in [
        "operator & &",
        "operator | |",
        "operator &/* λ */&",
        "operator |/* λ */|",
    ] {
        rejected(&format!("{BOX} __device__ Box {spelling}(Box left, Box right) {{ return left; }} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_logical_operators_use_prototypes_and_definitions_after_the_kernel() {
    let (module, _) = validate(&format!("{BOX} __device__ Box operator&&(Box, Box); __device__ Box operator||(Box, Box); __device__ Box choose(Box value) {{ return value && value || value; }} __global__ void kernel(int *out) {{ out[0] = choose(Box{{2}}).value; }} __device__ Box operator||(Box left, Box right) {{ return left; }} __device__ Box operator&&(Box left, Box right) {{ return right; }}"));
    for (caller, function) in module.functions.iter() {
        assert!(calls(function)
            .iter()
            .all(|(callee, _)| callee.index() < caller.index()));
    }
}

#[test]
fn cuda_logical_operators_bind_overloads_at_the_source_call_and_keep_other_current_overloads() {
    for symbol in ["&&", "||"] {
        let (module, _) = validate(&format!("{BOX} __device__ int operator{symbol}(Box, int); __device__ int earlier(unsigned value) {{ return Box{{1}} {symbol} value; }} __device__ int operator{symbol}(Box, unsigned); __global__ void kernel(int *out) {{ out[0] = earlier(2u); out[1] = Box{{1}} {symbol} 2u; }} __device__ int operator{symbol}(Box left, int right) {{ return right; }} __device__ int operator{symbol}(Box left, unsigned right) {{ return int(right); }}"));
        let earlier = module
            .functions
            .iter()
            .find(|(_, function)| function.name.as_deref() == Some("earlier"))
            .unwrap()
            .1;
        let first = calls(earlier)[0].0;
        let second = calls(&module.entry_points[0].function)[1].0;
        assert_eq!(
            module.types[module.functions[first].arguments[1].ty].inner,
            naga::TypeInner::Scalar(naga::Scalar::I32)
        );
        assert_eq!(
            module.types[module.functions[second].arguments[1].ty].inner,
            naga::TypeInner::Scalar(naga::Scalar::U32)
        );
        validate(&format!("{BOX} __device__ int operator{symbol}(Box left, int right) {{ return right; }} __device__ int operator{symbol}(Box left, Box right) {{ return left {symbol} right.value; }} __global__ void kernel(int *out) {{ out[0] = Box{{1}} {symbol} Box{{2}}; }}"));
        rejected(&format!("{BOX} __device__ int earlier(Box value) {{ return value {symbol} value; }} __device__ int operator{symbol}(Box left, Box right) {{ return 1; }} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_logical_operators_reject_called_missing_definitions_in_dead_source() {
    let (module, _) = validate(&format!("{BOX} __device__ int operator&&(Box, Box); __device__ void operator||(Box, Box); __global__ void kernel() {{}}"));
    assert!(module.functions.is_empty());
    for statement in [
        "false && Box{1};",
        "if (false) { false && Box{1}; }",
        "return; false && Box{1};",
        "false && (Box{1} && Box{2});",
        "true || (Box{1} && Box{2});",
    ] {
        rejected(&format!("{BOX} __device__ bool operator&&(Box, Box); __device__ int operator&&(bool, Box); __device__ void unused() {{ {statement} }} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_logical_operators_reject_direct_mutual_and_mixed_source_cycles() {
    for definitions in [
        "__device__ Box operator&&(Box left, Box right) { return left && right; }",
        "__device__ Box operator&&(Box, Box); __device__ Box operator||(Box, Box); __device__ Box operator&&(Box left, Box right) { return left || right; } __device__ Box operator||(Box left, Box right) { return left && right; }",
        "__device__ Box helper(Box); __device__ Box operator||(Box left, Box right) { return helper(left); } __device__ Box helper(Box value) { return value || value; }",
        "__device__ bool operator&&(Box, Box); __device__ bool operator||(Box, Box); __device__ bool operator&&(Box left, Box right) { return false && (left || right); } __device__ bool operator||(Box left, Box right) { return left && right; }",
    ] {
        rejected(&format!("{BOX} {definitions} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_logical_operators_keep_scalar_builtins_lazy_with_unrelated_visible_overloads() {
    let (module, _) = validate(&format!("{BOX} __device__ Box operator&&(Box left, Box right) {{ return left; }} __device__ Box operator||(Box left, Box right) {{ return right; }} __global__ void kernel(unsigned *out) {{ int counter = 0; out[0] = false && ++counter; out[1] = true || ++counter; out[2] = true && ++counter; out[3] = false || ++counter; out[4] = -2 && 3.5f; out[5] = 0u || 4; }}"));
    assert!(calls(&module.entry_points[0].function).is_empty());
    assert_eq!(
        module.entry_points[0]
            .function
            .body
            .iter()
            .filter(|statement| matches!(statement, naga::Statement::If { .. }))
            .count(),
        6
    );
}

#[test]
fn cuda_logical_operators_branch_scalar_valued_class_calls_under_outer_builtin_logic() {
    let (module, _) = validate(&format!("{BOX} __device__ int operator&&(Box left, Box right) {{ return left.value; }} __device__ int operator||(Box left, Box right) {{ return right.value; }} __global__ void kernel(int *out, unsigned choose) {{ out[0] = choose && (Box{{1}} && Box{{2}}); out[1] = choose || (Box{{3}} || Box{{4}}); }}"));
    let function = &module.entry_points[0].function;
    assert!(calls(function).is_empty());
    assert_eq!(
        statements(&function.body)
            .iter()
            .filter(|statement| matches!(statement, naga::Statement::Call { .. }))
            .count(),
        2
    );
}

#[test]
fn cuda_logical_operators_stay_runtime_calls_but_preserve_outer_builtin_constant_laziness() {
    let definitions = format!("{BOX} __device__ bool operator&&(Box left, Box right) {{ return true; }} __device__ bool operator||(Box left, Box right) {{ return false; }} __device__ int operator&&(bool left, Box right) {{ return 3; }} __device__ int operator||(bool left, Box right) {{ return 4; }}");
    validate(&format!("{definitions} __global__ void kernel() {{ int counter = 0; constexpr bool Both = false && (Box{{counter++}} && Box{{counter++}}); constexpr bool Either = true || (Box{{counter++}} || Box{{counter++}}); constexpr int Count = true ? 3 : (Box{{1}} && Box{{2}}); int values[Count]{{}}; }}"));
    for declaration in [
        "constexpr int Count = false && Box{1};",
        "constexpr int Count = true || Box{1};",
        "constexpr bool Result = Box{1} && Box{2};",
        "int values[false && Box{1}]{};",
        "switch (1) { case true || Box{1}: break; }",
    ] {
        rejected(&format!(
            "{definitions} __global__ void kernel() {{ {declaration} }}"
        ));
    }
}

#[test]
fn cuda_logical_operators_type_check_every_unselected_source_operand() {
    for expression in [
        "false && missing",
        "true || missing",
        "false && (Box{1} && Other{2})",
        "true || (Box{1} || Other{2})",
    ] {
        rejected(&format!("{BOX} struct Other {{ int value; }}; __device__ int operator&&(Box left, Box right) {{ return 1; }} __device__ int operator||(Box left, Box right) {{ return 2; }} __global__ void kernel() {{ {expression}; }}"));
    }
    for declaration in [
        "constexpr bool Result = false && missing;",
        "constexpr bool Result = true || missing;",
    ] {
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
}

#[test]
fn cuda_logical_operators_snapshot_original_left_values_before_right_side_effects() {
    for symbol in ["&&", "||"] {
        for (declarations, left, right, left_type) in [
            ("int left = 2;", "left", "(left = 7, Box{3})", "int"),
            ("Box left{3};", "left", "(left.value = 7, Box{4})", "Box"),
            (
                "Box left{3}, other{4}; bool choose = true;",
                "(choose ? left : other)",
                "(left.value = 7, Box{4})",
                "Box",
            ),
            (
                "int index = 0; Box values[2]{Box{3}, Box{4}};",
                "values[index++]",
                "(index = 7, Box{4})",
                "Box",
            ),
        ] {
            let (module, _) = validate(&format!("{BOX} __device__ int operator{symbol}({left_type} left, Box right) {{ return 1; }} __global__ void kernel(int *out) {{ {declarations} out[0] = {left} {symbol} {right}; }}"));
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
}

#[test]
fn cuda_logical_operators_keep_delayed_scalar_reference_reads_inside_builtin_rhs() {
    validate(&format!("{BOX} __device__ Box operator&&(Box left, Box right) {{ return left; }} __device__ Box operator||(Box left, Box right) {{ return right; }} __global__ void kernel(int *out, unsigned gate, unsigned choose) {{ int index = 0; Box values[2]{{Box{{4}}, Box{{5}}}}; out[0] = gate && (choose ? values[index++].value : values[index++].value); out[1] = gate || (choose ? values[index++].value : values[index++].value); out[2] = index; }}"));
}

#[test]
fn cuda_logical_operators_do_not_invent_class_truthiness_or_rewritten_overloads() {
    for statement in [
        "if (value) {}",
        "value && true;",
        "false && value;",
        "value || 1;",
        "true || value;",
    ] {
        rejected(&format!("{BOX} __device__ bool operator!(Box value) {{ return false; }} __global__ void kernel() {{ Box value{{1}}; {statement} }}"));
    }
    rejected(&format!("{BOX} __device__ int operator&&(Box left, Box right) {{ return 1; }} __global__ void kernel() {{ Box{{1}} || Box{{2}}; }}"));
    rejected(&format!("{BOX} __device__ int operator||(Box left, Box right) {{ return 1; }} __global__ void kernel() {{ Box{{1}} && Box{{2}}; }}"));
}

#[test]
fn cuda_logical_operators_preserve_precedence_associativity_and_result_type_transitions() {
    let (module, _) = validate(&format!("{BOX} __device__ Box operator&&(Box left, Box right) {{ return left; }} __device__ Box operator||(Box left, Box right) {{ return right; }} __global__ void kernel() {{ Box value{{1}}; value || value && value; value && value && value; }}"));
    let selected: Vec<_> = calls(&module.entry_points[0].function)
        .iter()
        .map(|(handle, _)| module.functions[*handle].name.as_deref().unwrap())
        .collect();
    assert_eq!(
        selected,
        ["operator&&", "operator||", "operator&&", "operator&&"]
    );
    validate(&format!("{BOX} __device__ int operator&&(Box left, Box right) {{ return 3; }} __device__ Box operator||(int left, Box right) {{ return right; }} __global__ void kernel(int *out) {{ out[0] = (Box{{1}} && Box{{2}} || Box{{3}}).value; }}"));
}

#[test]
fn cuda_logical_operators_reuse_nominal_scopes_and_readonly_storage_values() {
    let (module, _) = validate(&format!("{BOX} __device__ Box operator&&(Box left, Box right) {{ ++left.value; return left; }} __device__ int operator||(Box left, int right) {{ return left.value + right; }} __global__ void kernel(const Box *input, int *out) {{ const Box value = input[0]; out[0] = (input[1] && value).value; out[1] = value || 2; {{ struct Box {{ int value; }}; Box local{{3}}; out[2] = local.value; }} out[3] = value || 4; }}"));
    let storage: Vec<_> = module
        .global_variables
        .iter()
        .filter_map(|(_, global)| match global.space {
            naga::AddressSpace::Storage { access } => Some(access),
            _ => None,
        })
        .collect();
    assert_eq!(
        storage,
        [
            naga::StorageAccess::LOAD,
            naga::StorageAccess::LOAD | naga::StorageAccess::STORE
        ]
    );
    rejected(&format!("{BOX} __device__ int operator&&(Box left, Box right) {{ return 1; }} __global__ void kernel() {{ struct Box {{ int value; }}; Box local{{3}}; local && local; }}"));
}

#[test]
fn cuda_logical_operators_remap_generated_math_calls_and_late_compute_inputs() {
    let (module, _) = validate(&format!("{BOX} __device__ float operator&&(Box, Box); __global__ void kernel(float *out) {{ out[0] = Box{{4}} && Box{{0}}; }} __device__ float operator&&(Box left, Box right) {{ return sqrtf(float(left.value)) + sinf(float(right.value)) + cosf(0.0f) + float(threadIdx.x + blockIdx.y + blockDim.z + gridDim.x); }}"));
    for (caller, function) in module.functions.iter() {
        for statement in statements(&function.body) {
            if let naga::Statement::Call { function, .. } = statement {
                assert!(function.index() < caller.index());
            }
        }
        for (_, expression) in function.expressions.iter() {
            if let naga::Expression::CallResult(callee) = expression {
                assert!(callee.index() < caller.index());
            }
        }
    }
}

#[test]
fn cuda_logical_operators_reject_malformed_unicode_calls_and_bound_nested_depth() {
    for statement in [
        "operator&&();",
        "operator&&(value);",
        "operator||(value, value, value);",
        "operator&&(value,);",
        "operator||(,value);",
        "operator&&(value;",
        "return; operator&&(missing, value);",
        "if (false) { operator||(value, missing); }",
    ] {
        rejected(&format!("/* λ */ {BOX} __device__ Box operator&&(Box left, Box right) {{ return left; }} __device__ Box operator||(Box left, Box right) {{ return right; }} __global__ void kernel() {{ Box value{{1}}; {statement} }}"));
    }
    for symbol in ["&&", "||"] {
        let chain = vec!["value"; 96].join(&format!(" {symbol} "));
        let (module, _) = validate(&format!("{BOX} __device__ Box operator{symbol}(Box left, Box right) {{ return left; }} __global__ void kernel() {{ Box value{{1}}; {chain}; }}"));
        assert_eq!(calls(&module.entry_points[0].function).len(), 95);
    }
    let nested = format!("{}value{}", "operator&&(value, ".repeat(88), ")".repeat(88));
    rejected(&format!("{BOX} __device__ Box operator&&(Box left, Box right) {{ return left; }} __global__ void kernel() {{ Box value{{1}}; {nested}; }}"));
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_logical_operators_fixture_roundtrips_through_shared_wgsl_helpers() {
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
fn cuda_logical_operators_fixture_emits_valid_spirv_binary() {
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
        entry_point: "logical_operators".to_string(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
    crate::helpers::validate_spirv(&words);
}
