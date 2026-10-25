use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/preprocessing.cu");

fn kernel(definitions: &str, body: &str) -> String {
    format!("{definitions}\n__global__ void kernel(unsigned *out) {{\n{body}\n}}\n")
}

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert!(!error.message.is_empty());
    if let Some(range) = error.span.to_range() {
        assert!(source.get(range).is_some(), "{error}");
    }
    error
}

fn rejected_at(source: &str, spelling: &str) {
    let error = rejected(source);
    assert_eq!(
        source.get(error.span.to_range().unwrap()),
        Some(spelling),
        "{error}"
    );
}

fn returned_literal(module: &naga::Module, name: &str) -> naga::Literal {
    let function = module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some(name))
        .unwrap()
        .1;
    let value = function
        .body
        .iter()
        .find_map(|statement| match *statement {
            naga::Statement::Return { value } => value,
            _ => None,
        })
        .unwrap();
    match function.expressions[value] {
        naga::Expression::Literal(literal) => literal,
        ref other => panic!("expected a literal, got {other:?}"),
    }
}

#[test]
fn cuda_preprocessing_fixture_preserves_helper_values_storage_and_launch_options() {
    let (module, _) = super::helpers::validate_with_options(
        FIXTURE,
        &Options {
            workgroup_size: [2, 1, 1],
        },
    );
    assert_eq!(module.entry_points[0].name, "preprocessing");
    assert_eq!(module.entry_points[0].workgroup_size, [2, 1, 1]);
    assert_eq!(module.functions.len(), 1);
    assert!(module.types.iter().any(|(_, ty)| matches!(&ty.inner,
        naga::TypeInner::Struct { members, span: 8 }
        if members.len() == 2 && members[0].name.as_deref() == Some("first")
            && members[1].name.as_deref() == Some("second")
            && members[0].offset == 0 && members[1].offset == 4
            && members.iter().all(|member| module.types[member.ty].inner == naga::TypeInner::Scalar(naga::Scalar::U32))
    )));
    let storage = module
        .global_variables
        .iter()
        .find(|(_, global)| global.binding.is_some())
        .unwrap()
        .1;
    assert_eq!(
        storage.space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE
        }
    );
    let (readonly, _) = validate("#define READONLY const\n#define ELEMENT unsigned\n__global__ void kernel(READONLY ELEMENT *input, ELEMENT *output) { output[0] = input[0]; }");
    let access: Vec<_> = readonly
        .global_variables
        .iter()
        .filter(|(_, global)| global.binding.is_some())
        .map(|(_, global)| global.space)
        .collect();
    assert_eq!(
        access,
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
fn cuda_preprocessing_definitions_follow_source_order_and_undef_allows_replacement() {
    let (module, _) = validate("#define VALUE 11u\n__device__ unsigned first() { return VALUE; }\n#undef VALUE\n#define VALUE 22u\n__global__ void kernel() {}\n__device__ unsigned second() { return VALUE; }\n#undef ABSENT\n");
    assert_eq!(returned_literal(&module, "first"), naga::Literal::U32(11));
    assert_eq!(returned_literal(&module, "second"), naga::Literal::U32(22));
    rejected(&kernel("", "out[0] = LATER;\n#define LATER 3u"));
    validate(&kernel("", "{\n#define OUTSIDE 3u\n}\nout[0] = OUTSIDE;"));
}

#[test]
fn cuda_preprocessing_alias_replacements_resolve_at_each_use() {
    let (module, _) = validate("#define OUTER INNER\n#define INNER 1u\n__device__ unsigned first() { return OUTER; }\n#undef INNER\n#define INNER 2u\n__device__ unsigned second() { return OUTER; }\n#undef INNER\n__global__ void kernel() { unsigned INNER = 3u; OUTER; }\n");
    assert_eq!(returned_literal(&module, "first"), naga::Literal::U32(1));
    assert_eq!(returned_literal(&module, "second"), naga::Literal::U32(2));
    let names: Vec<_> = module.entry_points[0]
        .function
        .local_variables
        .iter()
        .filter_map(|(_, local)| local.name.as_deref())
        .collect();
    assert!(names.contains(&"INNER"));
}

#[test]
fn cuda_preprocessing_empty_macros_and_spaced_parentheses_remain_object_definitions() {
    validate(&kernel(
        "#define EMPTY\n#define VALUE (3u)\n#define PAREN /* spacing */ (4u)\n#\n# /* comment */\n",
        "EMPTY out[0] = VALUE + PAREN;\n#ifdef EMPTY\nout[1] = 1u;\n#else\nmissing;\n#endif",
    ));
    let (module, _) = validate(&kernel("#define SUM 2u + 3u", "out[0] = SUM * 4u;"));
    assert_eq!(module.entry_points.len(), 1);
}

#[test]
fn cuda_preprocessing_self_and_mutual_macros_stop_only_the_current_rescan() {
    let source = kernel("#define self self\n#define left right\n#define right left", "unsigned self = 7u, left = 11u, right = 13u; out[0] = self; out[1] = left; out[2] = right; out[3] = left;");
    let (module, _) = validate(&source);
    for name in ["self", "left", "right"] {
        assert!(module.entry_points[0]
            .function
            .local_variables
            .iter()
            .any(|(_, local)| local.name.as_deref() == Some(name)));
    }
}

#[test]
fn cuda_preprocessing_compatible_redefinitions_preserve_spelling_and_spacing() {
    for definitions in [
        "#define VALUE (1-1)\n#define VALUE /* leading */ (1-1) /* trailing */",
        "#define VALUE (1 - 1)\n#define VALUE (1\t-  1)",
        "#define VALUE (1/**/-/**/1)\n#define VALUE (1 - 1)",
        "#define VALUE\n#define VALUE /* empty */",
    ] {
        validate(&kernel(definitions, "VALUE;"));
    }
}

#[test]
fn cuda_preprocessing_rejects_conflicting_spellings_tokens_and_whitespace_separation() {
    for (first, second) in [
        ("1", "01"),
        ("1u", "1U"),
        ("(1-1)", "(1 - 1)"),
        ("1", "2"),
        ("", "1"),
        ("ALIAS", "OTHER"),
    ] {
        let source = kernel(
            &format!("#define VALUE {first}\n#define VALUE {second}"),
            "",
        );
        let error = rejected(&source);
        assert!(error.message.contains("redef"), "{error}");
        assert!(error.span.to_range().unwrap().start >= source.rfind("#define").unwrap());
    }
}

#[test]
fn cuda_preprocessing_conditionals_query_names_without_expanding_their_replacements() {
    validate(&kernel("#define NAME ABSENT\n#define EMPTY\n#ifdef NAME\n#ifndef ABSENT\n#define SELECTED 7u\n#else\ninvalid;\n#endif\n#else\ninvalid;\n#endif\n#ifdef EMPTY\n#define COUNT 2\n#endif", "unsigned values[COUNT]{SELECTED, 9u}; out[0] = values[0];"));
    validate("#ifndef GUARD\n#define GUARD\n__global__ void kernel() {}\n#endif\n#ifndef GUARD\n__global__ void duplicate() {}\n#endif\n");
    validate(&kernel("#ifndef __CUDACC__\n#define __CUDACC__ 1\n#endif\n#ifdef __CUDACC__\n#define SELECTED 3u\n#else\ninvalid;\n#endif", "out[0] = SELECTED;"));
}

#[test]
fn cuda_preprocessing_skipped_groups_ignore_unsupported_directives_and_conditional_payloads() {
    validate(&kernel("#ifdef ABSENT\n#define FUNCTION(value, ...) value ## __VA_ARGS__\n#include <not_read.h>\n#unknown tokens\n#if invalid(())\nthis is not CUDA -> syntax;\n#elif 1 / 0\ninvalid;\n#else\ninvalid;\n#endif\n#ifdef not an operand\n#ifndef also not an operand\ninvalid;\n#endif\n#endif\n#endif", "out[0] = 7u;"));
    validate(&kernel("#ifdef ABSENT\n#ifdef not an operand\n#else ignored payload\n#endif ignored payload\n#endif", "out[0] = 9u;"));
}

#[test]
fn cuda_preprocessing_already_selected_groups_skip_all_elif_expression_forms() {
    for alternative in [
        "elif invalid(())",
        "elifdef invalid tokens",
        "elifndef invalid tokens",
    ] {
        validate(&kernel(&format!("#define SELECTED\n#ifdef SELECTED\n#define VALUE 7u\n#{alternative}\ninvalid;\n#else\ninvalid;\n#endif"), "out[0] = VALUE;"));
    }
}

#[test]
fn cuda_preprocessing_evaluates_only_eligible_if_and_elif_conditions() {
    for definitions in [
        "#if 0\n#endif",
        "#if 1\n#endif",
        "#ifdef ABSENT\n#elif 1\n#endif",
        "#ifdef ABSENT\n#elifdef PRESENT\n#endif",
        "#ifdef ABSENT\n#elifndef ABSENT\n#endif",
    ] {
        validate(&kernel(definitions, ""));
    }
    rejected(&kernel("#if 0\n#elif 1 / 0\n#endif", ""));
    rejected(&kernel("#if 0\n#elifdef malformed names\n#endif", ""));
    validate(&kernel(
        "#if 1\n#elif 1 / 0\n#elifdef malformed names\n#endif",
        "",
    ));
}

#[test]
fn cuda_preprocessing_reports_active_deferred_directives_and_macro_features() {
    for directive in [
        "#define FUNCTION(value) value",
        "#define FUNCTION(...) __VA_ARGS__",
        "#define PASTE left ## right",
        "#include <cuda_runtime.h>",
        "#include \"never-read.h\"",
        "#line 12 \"other.cu\"",
        "#pragma once",
        "#error message",
        "#warning message",
        "#unknown directive",
    ] {
        let error = rejected(&kernel(directive, ""));
        assert!(error.message.contains("unsupported"), "{error}");
    }
}

#[test]
fn cuda_preprocessing_predefined_providers_are_explicitly_unsupported_when_needed() {
    for name in [
        "__LINE__",
        "__FILE__",
        "__DATE__",
        "__TIME__",
        "__cplusplus",
        "__STDC__",
        "__STDC_HOSTED__",
        "__STDCPP_DEFAULT_NEW_ALIGNMENT__",
        "__STDCPP_THREADS__",
    ] {
        for source in [
            kernel("", &format!("{name};")),
            kernel(&format!("#ifdef {name}\n#endif"), ""),
            kernel(&format!("#ifndef {name}\n#endif"), ""),
            kernel(&format!("#define {name} 1"), ""),
            kernel(&format!("#undef {name}"), ""),
        ] {
            let error = rejected(&source);
            assert!(error.message.contains("predefined"), "{name}: {error}");
        }
    }
    validate(&kernel("#define LOCATION __LINE__\n#ifdef ABSENT\n__LINE__;\n#ifdef __FILE__\n#endif\n#define __TIME__ ignored\n#undef __DATE__\n#endif", "out[0] = 1u;"));
    let error = rejected(&kernel("#define LOCATION __LINE__", "LOCATION;"));
    assert!(error.message.contains("predefined"), "{error}");
}

#[test]
fn cuda_preprocessing_macro_names_are_ascii_and_unsupported_lexical_forms_are_diagnostic() {
    for directive in [
        "#define Café 3",
        "#define 漢 3",
        "#define 123 3",
        r"#define \u0056ALUE 3",
        "%:define VALUE 3",
        "#define PASTE left %:%: right",
    ] {
        rejected(&kernel(directive, ""));
    }
    validate(&kernel(
        "#define Name_123 3u\n/* Café 漢 */",
        "out[0] = Name_123;",
    ));
    rejected(&kernel("#ifdef ABSENT\n%:define VALUE 3\n#endif", ""));
    for declaration in [
        "container<::Type> value;",
        "container<\\\n::Type> value;",
        "container<:\\\n:Type> value;",
        "container<::\\\nType> value;",
    ] {
        validate(&kernel(
            &format!("#ifdef ABSENT\n{declaration}\n#endif"),
            "",
        ));
    }
    for declaration in ["container<::> value;", "container<:::Type> value;"] {
        rejected(&kernel(
            &format!("#ifdef ABSENT\n{declaration}\n#endif"),
            "",
        ));
    }
}

#[test]
fn cuda_preprocessing_directives_begin_at_logical_line_starts_after_comments() {
    validate(&kernel(" /* preceding */ #define VALUE 3u\n\t# /* between */ define OTHER 4u\n#define MULTILINE 1u /* physical\n newline */ + 2u", "out[0] = VALUE + OTHER + MULTILINE;"));
    rejected("__global__ void kernel() {} /* physical\n newline */ #define VALUE 3u\n");
    rejected("__global__ void kernel() {} #define VALUE 3u\n");
}

#[test]
fn cuda_preprocessing_handles_bom_line_endings_and_an_implicit_final_newline() {
    for ending in ["\n", "\r\n", "\r"] {
        let source = format!("\u{feff}#define VALUE 31u{ending}__device__ unsigned sample() {{ return VALUE; }}{ending}__global__ void kernel() {{}}");
        let (module, _) = validate(&source);
        assert_eq!(returned_literal(&module, "sample"), naga::Literal::U32(31));
    }
    validate("__global__ void kernel() {}\n#define EMPTY");
}

#[test]
fn cuda_preprocessing_splices_directive_names_identifiers_and_numbers_before_tokenization() {
    for ending in ["\n", "\r\n", "\r"] {
        for whitespace in ["", " ", "\t", "\u{b}", "\u{c}"] {
            let splice = format!("\\{whitespace}{ending}");
            let source = format!("#defi{splice}ne NUM{splice}BER 0b10'{splice}01u{ending}__device__ unsigned sam{splice}ple() {{ return NUM{splice}BER; }}{ending}__global__ void kernel() {{}}{ending}");
            let (module, _) = validate(&source);
            assert_eq!(returned_literal(&module, "sample"), naga::Literal::U32(9));
        }
    }
}

#[test]
fn cuda_preprocessing_splicing_precedes_comment_recognition_and_preserves_separation() {
    validate(&kernel(
        "#define VALUE 7u",
        "out[0] = VALUE; /\\\n/ missing;\nout[1] = 9u; /\\\n* missing; */\nout[2] = 11u;",
    ));
    validate(&kernel(
        "#define A 1u /* multiline\n comment */ + 2u",
        "out[0] = A;",
    ));
    for body in ["unsigned value = 1/**/2;", "unsigned value = 0b10/**/01u;"] {
        rejected(&kernel("#", body));
    }
}

#[test]
fn cuda_preprocessing_quoted_tokens_are_opaque_without_granting_literal_value_support() {
    validate(&kernel(
        r###"#define NAME missing
#define QUOTED "NAME\" #endif // /*"
#define CHARACTER '\''
#define WIDE L"#else"
#define UTF8 u8"__LINE__"
#ifdef ABSENT
"#endif" 'N' '\'' u"#else" U'X' L"/*"
#endif
"###,
        "out[0] = 7u;",
    ));
    validate("#define C missing\nextern \"C\" __global__ void kernel() {}\n");
    validate("#define LINKAGE \"C\"\nextern LINKAGE __global__ void kernel() {}\n");
    for replacement in [
        "\"C\"1'2",
        "'C'1'2",
        "R\"tag(C)tag\"1'2",
        "\"C\"_suffix2",
        "'C'_suffix2",
        "R\"tag(C)tag\"_suffix2",
    ] {
        validate(&kernel(
            &format!("#define UNUSED {replacement}"),
            "out[0] = 7u;",
        ));
    }
    for expression in ["'X'", "L\"text\"", "u8\"text\"", "\"escaped\\n\""] {
        rejected(&kernel("#define ACTIVE", &format!("{expression};")));
    }
}

#[test]
fn cuda_preprocessing_raw_literals_shield_directives_and_revert_body_splicing() {
    validate(&kernel(
        r###"#define RAW R"tag(begin
)ta\
g"
#endif
#define HIDDEN 99u
end)tag"
#define PREFIXED u8R"delim(#else
)delim"
#ifdef ABSENT
LR"raw(
#endif
#define ALSO_HIDDEN 99u
)raw"
#endif
#ifdef HIDDEN
invalid;
#endif
#ifdef ALSO_HIDDEN
invalid;
#endif
"###,
        "out[0] = 7u;",
    ));
    validate(&kernel(
        r###"#define TEXT "a\
b"
#define TEXT "ab"
#define RAW R"tag(a\
b)tag"
#define RAW R"tag(a\
b)tag"
#define DELIMITER R"abcdefghijklmnop(body)abcdefghijklmnop"
"###,
        "out[0] = 9u;",
    ));
    let error = rejected(&kernel(
        r###"#define RAW R"tag(a\
b)tag"
#define RAW R"tag(ab)tag"
"###,
        "",
    ));
    assert!(error.message.contains("redef"), "{error}");
    rejected_at(&kernel("#define RAW R\"tag(body)tag\"", "RAW;"), "RAW");
    for source in [
        "#define RAW R\"tag(unterminated\n",
        "#define RAW R\"abcdefghijklmnopq(body)abcdefghijklmnopq\"\n__global__ void kernel() {}",
        "#ifdef ABSENT\nR\"bad delimiter(body)bad delimiter\"\n#endif\n__global__ void kernel() {}",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_preprocessing_keeps_raw_number_types_values_and_float_bits() {
    for (ty, spelling, expected) in [
        ("int", "0b10'01", naga::Literal::I32(9)),
        ("int", "017", naga::Literal::I32(15)),
        ("unsigned", "0xffff'ffffU", naga::Literal::U32(u32::MAX)),
        ("float", "1'024.5f", naga::Literal::F32(1024.5)),
        ("float", "1.25e+0'2f", naga::Literal::F32(125.0)),
    ] {
        let source = format!("#define NUMBER {spelling}\n__device__ {ty} sample() {{ return NUMBER; }}\n__global__ void kernel() {{}}\n");
        let (module, _) = validate(&source);
        assert_eq!(returned_literal(&module, "sample"), expected);
    }
}

#[test]
fn cuda_preprocessing_preserves_invalid_numbers_until_active_language_conversion() {
    for spelling in [
        "09u",
        "0b102u",
        "0xe+value",
        "1e+f",
        "1'2'foo",
        "0x1.fp+3f",
        "1.0",
        "1L",
    ] {
        validate(&kernel(&format!("#define BAD {spelling}"), "out[0] = 7u;"));
        let source = kernel(&format!("#define BAD {spelling}"), "out[0] = BAD;");
        rejected_at(&source, "BAD");
    }
    for (spelling, invalid_number) in [
        ("1'e+2", "1'e"),
        ("1'E-2", "1'E"),
        ("1'p+2", "1'p"),
        ("1'P-2", "1'P"),
    ] {
        let body = format!("float value = {spelling};");
        let direct_source = kernel("", &body);
        let prepared_source = kernel("#", &body);
        rejected_at(&direct_source, invalid_number);
        rejected_at(&prepared_source, invalid_number);
    }
}

#[test]
fn cuda_preprocessing_never_merges_replacement_and_source_tokens() {
    validate(&kernel(
        "#define PLUS +\n#define MINUS -\n#define SHIFT <<",
        "unsigned value = 3u; out[0] = value PLUS+ 2u; out[1] = 7 MINUS- 2; out[2] = 3u SHIFT 2u;",
    ));
    for (definitions, body) in [
        ("#define PLUS +", "unsigned value = 1u; value PLUS+;"),
        ("#define PLUS +", "unsigned value = 1u; value PLUS= 2u;"),
        ("#define LESS <", "out[0] = 1 LESS= 2;"),
        ("#define GREATER >", "out[0] = 8 GREATER> 1;"),
        ("#define EMPTY", "/EMPTY/ out[0] = 7u;"),
        ("#define SLASH /", "SLASH* hidden; */ out[0] = 7u;"),
        ("#define EXPONENT e+2f", "float value = 1 EXPONENT;"),
        ("#define HEAD val", "unsigned value = 7u; out[0] = HEAD ue;"),
    ] {
        rejected(&kernel(definitions, body));
    }
}

#[test]
fn cuda_preprocessing_expanded_hash_tokens_never_become_directives() {
    validate(&kernel("#define HASH #", "out[0] = 7u;"));
    rejected_at(&kernel("#define HASH #", "HASH define VALUE 3u"), "HASH");
    rejected("#define EMPTY\nEMPTY #define VALUE 3u\n__global__ void kernel() {}\n");
}

#[test]
fn cuda_preprocessing_rejects_malformed_directives_and_unbalanced_groups() {
    for definitions in [
        "#define",
        "#define 123 3",
        "#define VALUE+1",
        "#define VALUE\"text\"",
        "#undef",
        "#undef VALUE extra",
        "#ifdef",
        "#ifndef 123",
        "#ifdef VALUE extra\n#endif",
        "#else",
        "#endif",
        "#ifdef VALUE",
        "#ifdef VALUE\n#else\n#else\n#endif",
        "#ifdef VALUE\n#else\n#elif 1\n#endif",
        "#ifdef ABSENT\n#ifdef NESTED\n#else\n#else\n#endif\n#endif",
    ] {
        rejected(&kernel(definitions, ""));
    }
    rejected(&kernel(
        "#define VALUE\n#ifdef VALUE\n#else extra\n#endif",
        "",
    ));
    rejected(&kernel("#ifdef ABSENT\n#endif extra", ""));
    rejected("#define VALUE /* unterminated\n__global__ void kernel() {}\n");
}

#[test]
fn cuda_preprocessing_errors_and_ir_spans_refer_to_original_invocations() {
    for (definitions, body, expected) in [
        ("#define BAD missing", "out[0] = BAD;", "BAD"),
        (
            "#define INNER missing\n#define BAD INNER",
            "out[0] = BAD;",
            "BAD",
        ),
        ("#define BAD missing", "out[0] = BA\\\nD;", "BA\\\nD"),
        ("#define NUMBER 09u", "out[0] = NUM\\\nBER;", "NUM\\\nBER"),
        ("#", "float value = 1e\\\n? 2.0f : 3.0f;", "1e"),
        ("#", "float value = 1e\\\n+? 2.0f : 3.0f;", "1e\\\n+"),
    ] {
        rejected_at(
            &kernel(&format!("/* Café 漢 */\n{definitions}"), body),
            expected,
        );
    }
    let source = kernel(
        "/* 漢 */\n#define VALUE 7u\n#define TYPE unsigned",
        "TYPE value = (VALUE); out[0] = value;",
    );
    let (module, _) = validate(&source);
    let function = &module.entry_points[0].function;
    let (handle, _) = function
        .expressions
        .iter()
        .find(|(_, expression)| {
            matches!(expression, naga::Expression::Literal(naga::Literal::U32(7)))
        })
        .unwrap();
    assert_eq!(
        &source[function.expressions.get_span(handle).to_range().unwrap()],
        "VALUE"
    );
    for (handle, _) in function.expressions.iter() {
        if let Some(range) = function.expressions.get_span(handle).to_range() {
            assert!(source.get(range).is_some());
        }
    }
}

#[test]
fn cuda_preprocessing_preserves_public_parser_lookahead_and_delayed_references() {
    validate(&kernel("#define TYPE Packet\n#define FIELD value\n#define ARGUMENT 3\nstruct Packet { int value; };\n__device__ int read(Packet value) { return value.value; }", "TYPE values[2]{TYPE{ARGUMENT}, TYPE{4}}; int index = 0; TYPE copy = (TYPE{5}); TYPE{}; for (TYPE{index++}; index < 2; ++index) { out[0] = static_cast<unsigned>((true ? values[0] : values[1]).FIELD); } out[1] = read((TYPE{ARGUMENT})); out[2] = false && (true ? values[index++].FIELD : values[index++].FIELD); out[3] = (int)(ARGUMENT);"));
    for body in [
        "Packet value{}; (Packet)value;",
        "float2 value{}; (float2)value;",
        "int (*pointer);",
        "Packet{3",
    ] {
        let source = kernel("#define ACTIVE\nstruct Packet { int value; };", body);
        let first = rejected(&source);
        let second = rejected(&source);
        assert_eq!((first.message, first.span), (second.message, second.span));
    }
}

#[test]
fn cuda_preprocessing_parse_calls_keep_macro_tables_and_error_state_independent() {
    let source = kernel("#define VALUE 7u", "out[0] = VALUE;");
    let first = validate(&source).0;
    rejected(&kernel("", "out[0] = VALUE;"));
    rejected(&kernel("#ifdef UNFINISHED", ""));
    let second = validate(&source).0;
    assert_eq!(first.entry_points[0].name, second.entry_points[0].name);
    assert_eq!(
        first.entry_points[0].function.expressions.len(),
        second.entry_points[0].function.expressions.len()
    );
}

#[test]
fn cuda_preprocessing_bounds_conditional_expansion_depth_and_active_definition_count() {
    let nested = |count: usize| {
        format!(
            "{}{}",
            "#ifndef ABSENT\n".repeat(count),
            "#endif\n".repeat(count)
        )
    };
    validate(&kernel(&nested(64), ""));
    let error = rejected(&kernel(&nested(65), ""));
    assert!(
        error.message.contains("depth") || error.message.contains("limit"),
        "{error}"
    );
    let chain = |count: usize| {
        let mut definitions = String::new();
        for index in 0..count - 1 {
            definitions.push_str(&format!("#define Link{index} Link{}\n", index + 1));
        }
        definitions.push_str(&format!("#define Link{} 7u\n", count - 1));
        kernel(&definitions, "out[0] = Link0;")
    };
    validate(&chain(64));
    let error = rejected(&chain(65));
    assert!(
        error.message.contains("depth") || error.message.contains("limit"),
        "{error}"
    );
    let mut definitions = String::new();
    for index in 0..4096 {
        definitions.push_str(&format!("#define Value{index} 1u\n"));
    }
    validate(&kernel(&definitions, "out[0] = Value4095;"));
    validate(&kernel(
        &format!("{definitions}#undef Value0\n#define Replacement 7u\n"),
        "out[0] = Replacement;",
    ));
    definitions.push_str("#define Overflow 1u\n");
    let error = rejected(&kernel(&definitions, ""));
    assert!(
        error.message.contains("limit") || error.message.contains("many"),
        "{error}"
    );
    let mut recycled = String::new();
    for _ in 0..4097 {
        recycled.push_str("#define Reused 1u\n#undef Reused\n");
    }
    validate(&kernel(&recycled, "out[0] = 7u;"));
}

#[cfg_attr(miri, ignore)]
#[test]
fn cuda_preprocessing_bounds_stored_emitted_empty_and_spelling_expansion_work() {
    let bounded = |source: String| {
        let error = rejected(&source);
        assert!(
            error.message.contains("limit")
                || error.message.contains("large")
                || error.message.contains("budget"),
            "{error}"
        );
    };
    bounded(kernel(
        &format!("#define WIDE {}", "1 ".repeat(1_048_577)),
        "",
    ));
    for empty in [false, true] {
        let mut definitions = format!("#define Level0 {}\n", if empty { "" } else { "1u" });
        for index in 1..=23 {
            definitions.push_str(&format!(
                "#define Level{index} Level{} Level{}\n",
                index - 1,
                index - 1
            ));
        }
        bounded(kernel(&definitions, "Level23;"));
    }
    let long_name = format!("Empty{}", "x".repeat(1024 * 1024));
    let mut definitions = format!("#define {long_name}\n#define Short0 {long_name}\n");
    for index in 1..=7 {
        definitions.push_str(&format!(
            "#define Short{index} Short{} Short{}\n",
            index - 1,
            index - 1
        ));
    }
    bounded(kernel(&definitions, "Short7;"));
    bounded(kernel(
        &format!("#define HUGE {}", "x".repeat(16 * 1024 * 1024)),
        "HUGE HUGE HUGE HUGE HUGE;",
    ));
    bounded(format!(
        "\u{feff}#\n/*{}*/\n__global__ void kernel() {{}}",
        " ".repeat(64 * 1024 * 1024)
    ));
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_preprocessing_fixture_roundtrips_through_shared_wgsl_helpers() {
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
fn cuda_preprocessing_fixture_emits_valid_spirv_binary() {
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
        entry_point: "preprocessing".to_owned(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
    crate::helpers::validate_spirv(&words);
}
