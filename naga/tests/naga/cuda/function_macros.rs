use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/function-macros.cu");

fn kernel(definitions: &str, body: &str) -> String {
    format!("{definitions}\n__global__ void kernel(unsigned *out) {{\n{body}\n}}\n")
}

#[track_caller]
fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert!(!error.message.is_empty());
    if let Some(range) = error.span.to_range() {
        assert!(source.get(range).is_some(), "{error}");
    }
    error
}

#[track_caller]
fn rejected_at(source: &str, spelling: &str) {
    let error = rejected(source);
    assert_eq!(
        source.get(error.span.to_range().unwrap()),
        Some(spelling),
        "{error}"
    );
}

#[track_caller]
fn selects(definitions: &str, expression: &str) {
    validate(&kernel(
        &format!("{definitions}\n#if !({expression})\ninvalid;\n#endif"),
        "",
    ));
}

#[track_caller]
fn bounded(source: &str) {
    let error = rejected(source);
    assert!(
        error.message.contains("limit") || error.message.contains("large"),
        "{error}"
    );
}

#[test]
fn cuda_function_macros_fixture_preserves_launch_storage_and_value_types() {
    let (module, _) = super::helpers::validate_with_options(
        FIXTURE,
        &Options {
            workgroup_size: [2, 1, 1],
        },
    );
    assert_eq!(module.entry_points[0].name, "function_macros");
    assert_eq!(module.entry_points[0].workgroup_size, [2, 1, 1]);
    assert_eq!(module.functions.len(), 1);
    assert!(module.types.iter().any(|(_, ty)| matches!(&ty.inner,
        naga::TypeInner::Struct { members, span: 8 }
        if members.len() == 2 && members[0].name.as_deref() == Some("first")
            && members[1].name.as_deref() == Some("second")
            && members[0].offset == 0 && members[1].offset == 4)));
    let resources: Vec<_> = module
        .global_variables
        .iter()
        .filter(|(_, variable)| variable.binding.is_some())
        .collect();
    assert_eq!(resources.len(), 1);
    assert_eq!(
        resources[0].1.binding,
        Some(naga::ResourceBinding {
            group: 0,
            binding: 0
        })
    );
    assert_eq!(
        resources[0].1.space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE
        }
    );
    assert!(module.types.iter().all(|(_, ty)| match ty.inner {
        naga::TypeInner::Scalar(scalar) | naga::TypeInner::Vector { scalar, .. } =>
            scalar.width <= 4,
        _ => true,
    }));
}

#[test]
fn cuda_function_macros_distinguish_unseparated_headers_from_object_replacements() {
    selects("#define F(value)value\n#define OBJECT (7)\n#define COMMENT /* gap */ (9)\n#define SPLI\\\nCED(value) value", "F(3) == 3 && OBJECT == 7 && COMMENT == 9 && SPLICED(11) == 11");
    selects(
        "#define F\\\n(value) value\n#define G/**/(7)",
        "F(5) == 5 && G == 7",
    );
    rejected("#define INVALID+1\n__global__ void kernel() {}");
}

#[test]
fn cuda_function_macros_expand_zero_parameters_but_leave_uninvoked_names() {
    selects(
        "#define ZERO() 7\n#define F(value) value",
        "ZERO() == 7 && !F && defined(F)",
    );
    let (module, _) = validate(&kernel(
        "#define F(value) value",
        "unsigned F = 11u; out[0] = F;",
    ));
    assert!(module.entry_points[0]
        .function
        .local_variables
        .iter()
        .any(|(_, local)| local.name.as_deref() == Some("F")));
    validate(&kernel("#define EMPTY()", "EMPTY() out[0] = 7u;"));
}

#[test]
fn cuda_function_macros_substitute_complete_formal_identifiers_before_special_names() {
    selects("#define VALUE 99\n#define F(VALUE) VALUE\n#define KEYWORD(int) int\n#define QUERY(defined) defined\n#define PROVIDER(__LINE__) __LINE__\n#define FEATURE(__has_include) __has_include", "F(7) == 7 && KEYWORD(11) == 11 && QUERY(13) == 13 && PROVIDER(17) == 17 && FEATURE(19) == 19");
    validate(&kernel("#define NAME(value) saved_value\n#define FIELD(record, field) ((record).field)", "struct Record { unsigned value; }; Record record{7u}; unsigned saved_value = 9u; out[0] = NAME(unknown); out[1] = FIELD(record, value);"));
}

#[test]
fn cuda_function_macros_accept_invocation_whitespace_comments_and_ordinary_newlines() {
    validate(&kernel(
        "#define ID(value) value",
        "out[0] = ID /* gap */\n(\n7u\n); out[1] = ID\\\n(9u);",
    ));
    selects("#define ID(value) value", "ID /* gap */ (7) == 7");
    rejected("#define ID(value) value\n#if ID(\n7)\n#endif\n__global__ void kernel() {}");
}

#[test]
fn cuda_function_macros_collect_raw_arguments_with_parentheses_only() {
    selects("#define SUM(first, second) ((first) + (second))\n#define COMMA 1, 2\n#define ID(value) value", "SUM((1, 3), 4) == 7 && ID((COMMA)) == 2");
    for invocation in ["ID({1, 2})", "ID([1, 2])", "ID(Type<1, 2>)", "SUM(COMMA)"] {
        rejected(&kernel(
            "#define SUM(first, second) first\n#define ID(value) value\n#define COMMA 1, 2",
            invocation,
        ));
    }
    validate(&kernel("#define ID(value) value", "struct Pair { unsigned first; unsigned second; }; Pair pair = ID((Pair{7u, 9u})); out[0] = pair.second;"));
}

#[test]
fn cuda_function_macros_preserve_empty_argument_positions_and_zero_arity() {
    selects("#define ZERO() 7\n#define ONE(value) 11\n#define TWO(first, second) 13\n#define FIRST(first, second) first\n#define SECOND(first, second) second", "ZERO() == 7 && ONE() == 11 && ONE(/* empty */) == 11 && TWO(,) == 13 && FIRST(17,) == 17 && SECOND(,19) == 19");
    for invocation in ["ZERO(,)", "TWO()", "TWO(1)", "ONE(,)"] {
        rejected(&kernel(
            "#define ZERO() 7\n#define ONE(value) 11\n#define TWO(first, second) 13",
            invocation,
        ));
    }
}

#[test]
fn cuda_function_macros_prescan_nested_arguments_before_disabling_the_outer_call() {
    selects("#define ID(value) value\n#define INC(value) ((value) + 1)\n#define INNER 7\n#define ALIAS INNER", "ID(ID(7)) == 7 && INC(INC(5)) == 7 && ID(ALIAS) == 7");
    selects("#define ID(value) value\n#define INNER 7\n#define ALIAS INNER\n#undef INNER\n#define INNER 11", "ID(ALIAS) == 11");
}

#[test]
fn cuda_function_macros_isolate_prescan_then_rescan_new_argument_calls() {
    selects("#define ID(value) value\n#define INC(value) ((value) + 1)\n#define APPLY(function, value) function(value)\n#define COMMA 4, 9\n#define OUTER(value) SECOND(value)\n#define SECOND(first, second) second", "ID(INC)(7) == 8 && APPLY(INC, 9) == 10 && OUTER(COMMA) == 9");
    rejected(&kernel(
        "#define COMMA 1, 2\n#define OUTER(value) INNER(value)\n#define INNER(value) value",
        "out[0] = OUTER(COMMA);",
    ));
}

#[test]
fn cuda_function_macros_duplicate_or_discard_runtime_argument_text() {
    let (module, _) = validate(&kernel("#define ID(value) value\n#define TWICE(value) ((value), (value))\n#define DROP(value) 7u", "unsigned counter = 0u; out[0] = ID(counter++); out[1] = TWICE(counter++); out[2] = DROP(counter++); out[3] = counter;"));
    let function = &module.entry_points[0].function;
    let counter = function
        .local_variables
        .iter()
        .find(|(_, local)| local.name.as_deref() == Some("counter"))
        .unwrap()
        .0;
    let writes = function.body.iter().filter(|statement| matches!(**statement, naga::Statement::Store { pointer, .. } if matches!(function.expressions[pointer], naga::Expression::LocalVariable(local) if local == counter))).count();
    assert_eq!(writes, 4);
}

#[test]
fn cuda_function_macros_do_not_prescan_unused_arguments() {
    selects("#define DROP(value) 7\n#define BAD(first, second) first", "DROP(__LINE__) == 7 && DROP(BAD(1)) == 7 && DROP(09) == 7 && DROP(defined()) == 7 && DROP(defined(__LINE__)) == 7");
    validate(&kernel(
        "#define EMPTY(value)\n#define DROP(value) 7u",
        "EMPTY(__has_include(<never_read.h>)) out[0] = DROP(\"), __LINE__, (\");",
    ));
    rejected(&kernel(
        "#define DROP(value) 7u",
        "out[0] = DROP(\"unterminated);",
    ));
}

#[test]
fn cuda_function_macros_rescan_replacement_tails_with_following_source_arguments() {
    selects("#define INC(value) ((value) + 1)\n#define ALIAS INC\n#define RETURN() INC\n#define ID(value) value\n#define EMPTY()", "ALIAS(7) == 8 && RETURN()(9) == 10 && ID(INC)(11) == 12 && EMPTY() INC(13) == 14");
    validate(&kernel(
        "#define ID(value) value\n#define EMPTY()\n#define ALIAS ID\n#define FORWARD(value) value",
        "out[0] = FORWARD(EMPTY() ALIAS)(7u);",
    ));
}

#[test]
fn cuda_function_macros_do_not_revisit_emitted_names_or_cross_directive_barriers() {
    for body in [
        "out[0] = F OPEN 7u);",
        "out[0] = F\n#define OTHER 1\n(7u);",
        "out[0] = F\n#undef F\n(7u);",
        "out[0] = F\n#undef F\n#define F(value) 99u\n(7u);",
    ] {
        let source = format!(
            "__device__ unsigned F(unsigned value) {{ return value; }}\n{}",
            kernel("#define F(value) 99u\n#define OPEN (", body)
        );
        let (module, _) = validate(&source);
        assert!(module.entry_points[0]
            .function
            .body
            .iter()
            .any(|statement| matches!(statement, naga::Statement::Call { .. })));
    }
}

#[test]
fn cuda_function_macros_keep_disabled_tokens_unavailable_through_later_rescans() {
    let source = "__device__ unsigned P(unsigned value) { return value + 100u; }\n__device__ unsigned T(unsigned value) { return value + 100u; }\n#define ID(value) value\n#define P(value) Q(value)\n#define Q(value) value\n#define T(value) value + T\n#define A(value) B\n#define B(value) A(value)\n__global__ void kernel(unsigned *out) { unsigned B = 79u; out[0] = ID(P(P))(7u); out[1] = T(7u)(9u); out[2] = A(1)(2); }";
    let (module, _) = validate(source);
    assert_eq!(
        module.entry_points[0]
            .function
            .body
            .iter()
            .filter(|statement| matches!(statement, naga::Statement::Call { .. }))
            .count(),
        2
    );
    validate("__device__ unsigned F(unsigned value) { return value; }\n#define F(value) G(value) + 1u\n#define G(value) F(value) + 2u\n__global__ void kernel(unsigned *out) { out[0] = F(7u); }");
    validate("__device__ int f(int value) { return value + 1; }\n__device__ int t(int value) { return value + 2; }\n#define x 3\n#define f(value) f(x * (value))\n#undef x\n#define x 2\n#define g f\n#define t(value) value\n__global__ void kernel(unsigned *out) { int y = 3, z[1]{5};\n#define z z[0]\nout[0] = f(y + 1) + f(f(z)) % t(t(g)(0) + t)(1); }");
}

#[test]
fn cuda_function_macros_follow_source_order_undef_and_fresh_definition_bindings() {
    validate("#define F(value) value\n__device__ unsigned first() { return F(7u); }\n#undef F\n#define F 11u\n__device__ unsigned second() { return F; }\n#undef F\n#define F(value) ((value) + 1u)\n__global__ void kernel(unsigned *out) { out[0] = F(13u); {\n#define LOCAL(value) value\n} out[1] = LOCAL(17u); }");
    rejected(&kernel(
        "",
        "out[0] = LATER(7u);\n#define LATER(value) value",
    ));
}

#[test]
fn cuda_function_macros_accept_only_spelling_compatible_redefinitions() {
    for definitions in ["#define F(value) ( value )\n#define F( value )( /* gap */ value /* tail */ )", "#define F(first,second) (first + second)\n#define F( first , /* gap */ second ) (first\t+  second)", "#define F()\n#define F( /* empty */ ) /* empty */"] {
        validate(&kernel(definitions, ""));
    }
    selects(
        "#define F(value) value\n#define F( value )value",
        "F(7) == 7",
    );
}

#[test]
fn cuda_function_macros_reject_changed_parameters_kinds_or_replacement_lists() {
    for (first, second) in [
        ("F(value) value", "F(other) other"),
        ("F(value) value", "F(value, other) value"),
        ("F(value) value", "F(value) (value)"),
        ("F(value) (value+1)", "F(value) (value + 1)"),
        ("F(value) 1u", "F(value) 1U"),
        ("F(value) value", "F (value)"),
        ("F (value)", "F(value) value"),
    ] {
        let source = kernel(&format!("#define {first}\n#define {second}"), "");
        let error = rejected(&source);
        assert!(error.message.contains("redef"), "{error}");
        assert!(error.span.to_range().unwrap().start >= source.rfind("#define").unwrap());
    }
}

#[test]
fn cuda_function_macros_reject_malformed_or_duplicate_formal_parameters() {
    for header in [
        "F(value,value) 7",
        "F(7) 7",
        "F(value other) 7",
        "F(,value) 7",
        "F(value,) 7",
        "F(value 7",
        "F(and) 7",
        "F(not_eq) 7",
    ] {
        rejected(&kernel(&format!("#define {header}"), ""));
    }
    selects(
        "#define F(int, defined, __FILE__) ((int) + (defined) + (__FILE__))",
        "F(2, 3, 5) == 10",
    );
}

#[test]
fn cuda_function_macros_reject_stringization_pasting_and_fixed_variadic_spellings() {
    for definition in [
        "F(value) #value",
        "F(value) value ## tail",
        "F(...) __VA_ARGS__",
        "F(value, ...) value",
        "F(value...) value",
        "F(__VA_ARGS__) 7",
        "F(__VA_OPT__) 7",
        "F(value) __VA_ARGS__",
        "F(value) __VA_OPT__(value)",
    ] {
        rejected(&kernel(&format!("#define {definition}"), ""));
    }
    validate(&kernel(
        "#define HASH #\n#define DROP(value) 7u\n#define EMPTY(value)",
        "EMPTY(##) out[0] = DROP(#);",
    ));
    rejected_at(&kernel("#define ID(value) value", "ID(##);"), "ID(##)");
}

#[test]
fn cuda_function_macros_report_complete_raw_arity_and_unterminated_invocations() {
    for invocation in ["PAIR(1)", "PAIR(1, 2, 3)", "ZERO(1)", "ONE(1, 2)"] {
        rejected_at(
            &kernel(
                "#define PAIR(first, second) first\n#define ZERO() 7\n#define ONE(value) value",
                &format!("{invocation};"),
            ),
            invocation,
        );
    }
    rejected_at("#define PAIR(first, second) first\nPAIR(1", "PAIR(1");
    rejected_at("#define ID(value) value\nID((7)", "ID((7)");
}

#[test]
fn cuda_function_macros_generate_existing_declarations_lvalues_and_helper_calls() {
    let source = "#define DECLARE(type, name, value) type name = value\n#define PROTOTYPE(type, name) __device__ type name(type)\n#define READONLY(type) const type\n#define AT(buffer, index) buffer[index]\nPROTOTYPE(unsigned, increment);\n__global__ void kernel(READONLY(unsigned) *input, unsigned *output) { DECLARE(unsigned, values[2], {7u}); AT(output, 0) = increment(AT(input, 0)) + values[0]; }\n__device__ unsigned increment(unsigned value) { return value + 1u; }";
    let (module, _) = validate(source);
    let resources: Vec<_> = module
        .global_variables
        .iter()
        .filter(|(_, variable)| variable.binding.is_some())
        .map(|(_, variable)| variable.space)
        .collect();
    assert_eq!(
        resources,
        [
            naga::AddressSpace::Storage {
                access: naga::StorageAccess::LOAD
            },
            naga::AddressSpace::Storage {
                access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE
            }
        ]
    );
}

#[test]
fn cuda_function_macros_share_existing_fixed64_condition_and_lazy_value_rules() {
    selects("#define ID(value) value\n#define ADD(left, right) ((left) + (right))\n#define CODE() 'A'", "ADD(0xffffffffffffffffu, 1u) == 0 && ID(1 << 63) < 0 && CODE() == 65 && defined(ID) && (1 || ID(1 / 0))");
    validate(&kernel("#define ID(value) value\n#if ID(0)\ninvalid;\n#elif ID(1)\n#define VALUE 7u\n#elif __LINE__\ninvalid;\n#endif", "out[0] = VALUE;"));
    rejected(&kernel(
        "#define ID(value) value\n#if 1 || ID(09)\n#endif",
        "",
    ));
    rejected(&kernel(
        "#define ID(value) value\n#if ID(1 / 0)\n#endif",
        "",
    ));
}

#[test]
fn cuda_function_macros_keep_defined_and_provider_handling_in_the_correct_mode() {
    selects(
        "#define DROP(value) 1\n#define F(value) value",
        "DROP(defined()) && DROP(defined(__LINE__)) && defined(F) && !defined(MISSING)",
    );
    for definitions in [
        "#define NAME 7\n#define ID(value) value\n#if ID(defined(NAME))\n#endif",
        "#define NAME 7\n#define ID(value) value\n#if ID(defined)(NAME)\n#endif",
        "#define QUERY(value) defined(value)\n#if QUERY(NAME)\n#endif",
        "#define ID(value) value\n#if 1 || ID(__has_include(<never_read.h>))\n#endif",
    ] {
        rejected(&kernel(definitions, ""));
    }
    validate(&kernel(
        "#define ID(value) value\n#define DROP(value)\n#define ALIAS defined",
        "unsigned defined = 7u; DROP(defined()) out[0] = ID(ALIAS);",
    ));
    selects(
        "#define F(defined, __LINE__, __has_include) ((defined) + (__LINE__) + (__has_include))",
        "F(2, 3, 5) == 10",
    );
}

#[test]
fn cuda_function_macros_preserve_opaque_literals_phases_and_token_boundaries() {
    validate(&kernel("#define DROP(value)\n#define ID(value) value\n#define PLUS(value) value +", "DROP(u8R\"tag((, __LINE__, ))tag\") DROP('\"') DROP(\"), (\") unsigned value = 3u; out[0] = PLUS(value)+ 2u; out[1] = ID(1'024.5f); out[2] = ID(0xffff'ffffU); out[3] = ID\\\n(7u);"));
    for replacement in ["0x", "1e", "/"] {
        rejected(&kernel(
            &format!("#define PREFIX(value) {replacement}"),
            "out[0] = PREFIX(0) 10;",
        ));
    }
    rejected_at(
        &kernel("#define ID(value) value", "out[0] = ID(0x1e+2);"),
        "ID(0x1e+2)",
    );
}

#[test]
fn cuda_function_macros_preserve_complete_outer_invocation_and_original_source_spans() {
    for (definitions, body, expected) in [
        ("#define ID(value) value", "out[0] = ID(09u);", "ID(09u)"),
        (
            "#define INNER(value) __LINE__\n#define OUTER(value) value",
            "out[0] = OUTER(INNER(7));",
            "OUTER(INNER(7))",
        ),
        (
            "#define BAD(value) 09u\n#define ALIAS BAD",
            "out[0] = ALIAS /* gap */ (7);",
            "ALIAS /* gap */ (7)",
        ),
        (
            "#define BAD(value) 09u\n#define WRAP() BAD(7)",
            "out[0] = WRAP();",
            "WRAP()",
        ),
        (
            "#define BAD(value) 09u\n#define OBJECT BAD(7)",
            "out[0] = OBJECT;",
            "OBJECT",
        ),
        ("#define BAD 09u", "out[0] = BAD;", "BAD"),
        (
            "#define ID(value) value",
            "out[0] = I\\\nD(09u);",
            "I\\\nD(09u)",
        ),
        (
            "#define ID(value) value",
            "out[0] = ID(7u) + absent;",
            "absent",
        ),
    ] {
        rejected_at(&kernel(&format!("/* 漢 */\n{definitions}"), body), expected);
    }
}

#[test]
fn cuda_function_macros_skip_inactive_calls_but_reject_directives_inside_open_calls() {
    validate(&kernel("#if 0\n#define BAD(value, ...) value ## __VA_ARGS__\nBAD(unclosed\n#if unknown((\n#else ignored\n#endif ignored\n#endif\n#define ID(value) value", "out[0] = ID(7u);"));
    rejected(&kernel(
        "#define ID(value) value",
        "out[0] = ID(\n#define VALUE 7\nVALUE);",
    ));
    rejected(&kernel(
        "#define ID(value) value",
        "out[0] = ID(\n#undef ID\n7u);",
    ));
    rejected(&kernel("#define ID(value) value\n#if ID(\n7)\n#endif", ""));
}

#[test]
fn cuda_function_macros_preserve_parser_lookahead_and_independent_parse_state() {
    let source = kernel("#define ID(value) value\n#define TYPE() int\n#define RECORD(value) value", "struct Record { int value; }; using Alias = Record; Alias first{3}, second{5}; int values[2]{7, 9}; TYPE() result = ID((int) values[0]); out[0] = unsigned(RECORD((true ? first : second)).value + result); { Alias local = ID((Alias{11})); out[1] = local.value; }");
    validate(&source);
    rejected(&kernel("#define ID(value) value", "ID(09u);"));
    let (first, _) = validate(&source);
    let (second, _) = validate(&source);
    assert_eq!(first.entry_points[0].name, second.entry_points[0].name);
    rejected(&kernel("", "out[0] = ID(7u);"));
}

#[test]
fn cuda_function_macros_bound_pending_activations_and_raw_inner_parentheses() {
    for depth in [64, 65] {
        let invocation = format!("{}7u{}", "ID(".repeat(depth), ")".repeat(depth));
        let source = kernel(
            "#define ID(value) value",
            &format!("out[0] = {invocation};"),
        );
        if depth == 64 {
            validate(&source);
        } else {
            bounded(&source);
        }
        let invocation = format!("DROP({}7u{})", "(".repeat(depth), ")".repeat(depth));
        let source = kernel("#define DROP(value) 7u", &format!("out[0] = {invocation};"));
        if depth == 64 {
            validate(&source);
        } else {
            bounded(&source);
        }
        let mut definitions = String::from("#define Layer0() 7u\n");
        for level in 1..depth {
            definitions.push_str(&format!("#define Layer{level}() Layer{}()\n", level - 1));
        }
        let source = kernel(&definitions, &format!("out[0] = Layer{}();", depth - 1));
        if depth == 64 {
            validate(&source);
        } else {
            bounded(&source);
        }
    }
    let mut definitions = String::from("#define ID(value) value\n#define BASE 7u\n");
    for level in 0..32 {
        definitions.push_str(&format!(
            "#define Alias{level} {}\n",
            if level == 0 {
                "BASE".to_owned()
            } else {
                format!("Alias{}", level - 1)
            }
        ));
    }
    for identity_depth in [31, 32] {
        let source = kernel(
            &definitions,
            &format!(
                "out[0] = {}Alias31{};",
                "ID(".repeat(identity_depth),
                ")".repeat(identity_depth)
            ),
        );
        if identity_depth == 31 {
            validate(&source);
        } else {
            bounded(&source);
        }
    }
    selects(
        "#define ID(value) value",
        &core::iter::repeat_n("ID(1)", 2048)
            .collect::<Vec<_>>()
            .join(" && "),
    );
}

#[test]
fn cuda_function_macros_allow_wide_arity_and_bound_cumulative_definition_storage() {
    let parameters: Vec<_> = (0..257).map(|index| format!("parameter{index}")).collect();
    let arguments: Vec<_> = (0..257).map(|index| index.to_string()).collect();
    selects(
        &format!(
            "#define WIDE({}) ({})",
            parameters.join(","),
            parameters.join(" + ")
        ),
        &format!("WIDE({}) == 32896", arguments.join(",")),
    );
    let mut definitions = String::new();
    for index in 0..4096 {
        definitions.push_str(&format!("#define Function{index}() 7u\n"));
    }
    validate(&kernel(&definitions, "out[0] = Function4095();"));
    definitions.push_str("#define Extra() 7u\n");
    bounded(&kernel(&definitions, ""));
    let mut definitions = String::new();
    for _ in 0..8193 {
        definitions.push_str(&format!("#define F() {}\n#undef F\n", "+0".repeat(64)));
    }
    bounded(&kernel(&definitions, ""));
}

#[test]
fn cuda_function_macros_share_prescan_emission_work_and_spelling_budgets() {
    let mut definitions = format!(
        "#define DROP(value) 1u\n#define OUTER(value) DROP(value)\n#define Base(value) value {}\n",
        "+ 0 ".repeat(1024)
    );
    for level in 0..9 {
        definitions.push_str(&format!(
            "#define Level{level}(value) {}(value) + {}(value)\n",
            if level == 0 {
                "Base".to_owned()
            } else {
                format!("Level{}", level - 1)
            },
            if level == 0 {
                "Base".to_owned()
            } else {
                format!("Level{}", level - 1)
            }
        ));
    }
    bounded(&kernel(&definitions, "out[0] = OUTER(Level8(1u));"));
    let mut source = String::from("#define DROP(value)\n");
    source.push_str(&format!("DROP({})\n", "x ".repeat(1_048_577)));
    source.push_str("__global__ void kernel() {}\n");
    bounded(&source);
    let long_name = format!("Empty{}", "x".repeat(1024 * 1024));
    let mut definitions = format!("#define {long_name}()\n#define Invoke0() {long_name}()\n");
    for level in 1..8 {
        definitions.push_str(&format!(
            "#define Invoke{level}() Invoke{}() Invoke{}()\n",
            level - 1,
            level - 1
        ));
    }
    bounded(&kernel(&definitions, "Invoke7()"));
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_function_macros_fixture_roundtrips_through_shared_wgsl_helpers() {
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
fn cuda_function_macros_fixture_emits_valid_spirv_binary() {
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
        entry_point: "function_macros".to_owned(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
    crate::helpers::validate_spirv(&words);
}
