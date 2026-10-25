use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/preprocessing-conditions.cu");

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
fn branch(expression: &str, expected: bool) {
    let source = format!(
        "#if {expression}\n__device__ int selected() {{ return 7; }}\n\
         #else\n__device__ int selected() {{ return 11; }}\n#endif\n\
         __global__ void kernel() {{}}\n"
    );
    let (module, _) = validate(&source);
    let function = module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some("selected"))
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
    assert_eq!(
        function.expressions[value],
        naga::Expression::Literal(naga::Literal::I32(if expected { 7 } else { 11 })),
        "{expression}",
    );
}

#[track_caller]
fn invalid_condition(expression: &str) {
    rejected(&kernel(&format!("#if {expression}\n#endif"), ""));
}

#[test]
fn cuda_preprocessing_conditions_fixture_preserves_launch_storage_and_helper_values() {
    let (module, _) = super::helpers::validate_with_options(
        FIXTURE,
        &Options {
            workgroup_size: [2, 1, 1],
        },
    );
    assert_eq!(module.entry_points[0].name, "preprocessing_conditions");
    assert_eq!(module.entry_points[0].workgroup_size, [2, 1, 1]);
    assert_eq!(module.functions.len(), 1);
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
            access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE,
        }
    );
}

#[test]
fn cuda_preprocessing_conditions_keep_integer_bases_separators_and_wide_signed_values() {
    for expression in [
        "0b1'0000 == 020 && 020 == 16 && 0x10 == 16",
        "0'7 == 7 && 0B10'01 == 9 && 0Xf'f == 255",
        "2147483648 > 2147483647",
        "4294967295 > 0 && 0xffffffff > 0",
        "-1 < 0xffffffff",
        "9223372036854775807 == 0x7fff'ffff'ffff'ffff",
        "0xffffffffffffffff > 0",
        "18446744073709551615u == 0xffffffffffffffff",
    ] {
        branch(expression, true);
    }
    for expression in ["0", "0b0", "00", "0x0", "0'0", "-1 < 0xffffffffu"] {
        branch(expression, false);
    }
}

#[test]
fn cuda_preprocessing_conditions_accept_standard_integer_suffix_orders_and_case() {
    for suffix in ["", "l", "L", "ll", "LL", "z", "Z"] {
        branch(&format!("7{suffix} == 7 && -1 < 7{suffix}"), true);
    }
    for suffix in [
        "u", "U", "ul", "uL", "Ul", "UL", "lu", "lU", "Lu", "LU", "ull", "uLL", "Ull", "ULL",
        "llu", "llU", "LLu", "LLU", "uz", "uZ", "Uz", "UZ", "zu", "zU", "Zu", "ZU",
    ] {
        branch(&format!("7{suffix} == 7 && !(-1 < 7{suffix})"), true);
        branch(&format!("0xffffffffffffffff{suffix} > 0"), true);
    }
    for suffix in ["", "l", "L", "ll", "LL", "z", "Z"] {
        branch(&format!("0xffffffffffffffff{suffix} > 0"), true);
    }
}

#[test]
fn cuda_preprocessing_conditions_reject_invalid_number_spellings_suffixes_and_ranges() {
    for literal in [
        "09",
        "0x",
        "0b2",
        "1''2",
        "0x'1",
        "1'",
        "7lL",
        "7Ll",
        "7uu",
        "7lul",
        "7lll",
        "7zz",
        "7lz",
        "7uul",
        "7_z",
        "1.0",
        "1.0f",
        "1e2",
        "0x1p0",
        "0x1e+2",
        "18446744073709551616u",
        "0x10000000000000000",
        "9223372036854775808",
        "9223372036854775808LL",
        "9223372036854775808z",
    ] {
        invalid_condition(literal);
    }
    branch("0x1e + 2 == 32", true);
    branch("-9223372036854775807LL - 1 < 0", true);
}

#[test]
fn cuda_preprocessing_conditions_apply_all_binary_precedence_levels_and_associativity() {
    for expression in [
        "1 + 2 * 3 == 7",
        "20 - 6 - 4 == 10",
        "24 / 3 / 2 == 4",
        "17 % 5 == 2",
        "1 + 2 << 3 == 24",
        "16 >> 1 + 1 == 4",
        "1 < 2 == 1",
        "3 > 2 == 1 && 2 >= 2 && 1 <= 2",
        "(7 & 3 == 3) == 1",
        "(1 | 2 ^ 3 & 1) == 3",
        "1 || 0 && 0",
        "0 || 1 && 1",
        "0 ? 3 : 0 ? 5 : 7",
    ] {
        branch(expression, true);
    }
    branch("2 < 1 < 1", true);
    branch("1 > 2 > 0", false);
}

#[test]
fn cuda_preprocessing_conditions_preserve_unary_types_and_promote_boolean_results() {
    for expression in [
        "+7 == 7 && -7 < 0",
        "~0 == -1",
        "~0u == 0xffffffffffffffffu",
        "-1u == 0xffffffffffffffffu",
        "!!7 == 1 && !0 == 1",
        "~true == -2 && -true == -1",
        "(0u == 1u) - 1 < 0",
        "(1u < 2u) - 2 < 0",
        "(1u && 7u) - 2 < 0",
        "(0u || 7u) - 2 < 0",
    ] {
        branch(expression, true);
    }
}

#[test]
fn cuda_preprocessing_conditions_convert_arithmetic_to_unsigned_and_wrap_modulo_64_bits() {
    for expression in [
        "0xffffffffffffffffu + 1u == 0",
        "0u - 1 == 0xffffffffffffffffu",
        "0x8000000000000000u * 2u == 0",
        "-1 == 0xffffffffffffffffu",
        "-1 + 1u == 0",
        "-5 / 2 == -2 && -5 % 2 == -1",
        "18446744073709551615u / 3u == 6148914691236517205u",
        "18446744073709551615u % 2u == 1u",
    ] {
        branch(expression, true);
    }
    branch("-1 < 1u", false);
}

#[test]
fn cuda_preprocessing_conditions_shift_64_bits_with_independent_operand_types() {
    for expression in [
        "1 << 31 == 2147483648",
        "1 << 32 == 4294967296",
        "(1LL << 63) < 0",
        "(-1 << 1) == -2",
        "(1 << 63) == (-9223372036854775807 - 1)",
        "(-3 >> 1) == -2",
        "(-1 >> 63u) == -1",
        "(-1 >> 1u) < 0",
        "(0xffffffffffffffffu >> 63) == 1",
        "((1 << 63u) >> 63) == -1",
        "((1u << 63) >> 63) == 1",
    ] {
        branch(expression, true);
    }
}

#[test]
fn cuda_preprocessing_conditions_reject_live_signed_arithmetic_overflow() {
    for expression in [
        "9223372036854775807 + 1",
        "(-9223372036854775807 - 1) - 1",
        "9223372036854775807 * 2",
        "-(-9223372036854775807 - 1)",
        "(-9223372036854775807 - 1) / -1",
        "(-9223372036854775807 - 1) % -1",
    ] {
        invalid_condition(expression);
    }
}

#[test]
fn cuda_preprocessing_conditions_reject_live_zero_divisors_and_invalid_shift_counts() {
    for expression in [
        "1 / 0",
        "1u / 0u",
        "1 % 0",
        "1u % 0u",
        "1 << -1",
        "1 >> -1",
        "1 << 64",
        "1u >> 64u",
        "1 << 0xffffffffffffffffu",
    ] {
        invalid_condition(expression);
    }
}

#[test]
fn cuda_preprocessing_conditions_short_circuit_values_without_discarding_syntax() {
    for fault in [
        "1 / 0",
        "1 % 0",
        "9223372036854775807 + 1",
        "(-9223372036854775807 - 1) / -1",
        "-(-9223372036854775807 - 1)",
        "1 << -1",
        "1 >> 64",
    ] {
        branch(&format!("0 && ({fault})"), false);
        branch(&format!("1 || ({fault})"), true);
        invalid_condition(&format!("1 && ({fault})"));
        invalid_condition(&format!("0 || ({fault})"));
    }
    for malformed in ["09", "1.0", "1 +", "(1", "unknown()", "1 = 2"] {
        invalid_condition(&format!("0 && ({malformed})"));
        invalid_condition(&format!("1 || ({malformed})"));
    }
}

#[test]
fn cuda_preprocessing_conditions_type_both_ternary_arms_and_evaluate_only_one() {
    for expression in [
        "(0 ? 1u : -1) == 0xffffffffffffffffu",
        "(1 ? -1 : 0u) == 0xffffffffffffffffu",
        "(1 ? 7 : (1u / 0)) == 7u",
        "(1 ? -1 : (1u / 0)) == 0xffffffffffffffffu",
        "(0 ? (1u / 0) : -1) > 0",
        "(0 ? (1 / 0) : 9) == 9",
        "(0 ? (1 << 64) : 11u) == 11u",
        "(1 ? (0 ? 3 : 5) : 7) == 5",
        "(0 ? 3 : 1 ? 5 : 7) == 5",
        "(1 ? (1 ? -1 : 0u) : 0) > 0",
    ] {
        branch(expression, true);
    }
    branch("(0 ? 1u : -1) < 0", false);
    branch("(1 ? -1 : 0u) < 0", false);
    for expression in ["1 ? 7 : 09", "0 ? 1.0 : 7", "1 ? 7 : (1 +)", "1 ? 7"] {
        invalid_condition(expression);
    }
}

#[test]
fn cuda_preprocessing_conditions_comma_uses_right_type_and_checks_live_discarded_values() {
    for expression in [
        "(0, 7) == 7",
        "(1, 0, 9) == 9",
        "(1u, -1) < 0",
        "(1 ? 0u, -1 : 0) < 0",
        "(0 ? 0 : (1u, -1)) < 0",
        "(1 ? 0, 7 : 0) == 7",
        "1 || (1 / 0, 7)",
        "!(0 && (1 / 0, 7))",
    ] {
        branch(expression, true);
    }
    for expression in ["1, 2", "1 ? 2 : 3, 4", "(1 / 0, 7)", "(1,)", "(,1)"] {
        invalid_condition(expression);
    }
}

#[test]
fn cuda_preprocessing_conditions_defined_protects_names_and_observes_source_order() {
    validate(&kernel(
        "#define ALIAS ABSENT\n#define EMPTY\n\
         #if !defined ALIAS || !defined(ALIAS) || defined(ABSENT) || !defined EMPTY\ninvalid;\n#endif\n\
         #if defined(defined) || defined defined\ninvalid;\n#endif\n\
         #undef ALIAS\n#if defined(ALIAS)\ninvalid;\n#endif\n\
         #define ALIAS 0\n#if !defined(ALIAS) || ALIAS\ninvalid;\n#endif",
        "out[0] = 7u;",
    ));
    for malformed in [
        "defined",
        "defined()",
        "defined(1)",
        "defined((NAME))",
        "defined(NAME extra)",
        "defined(NAME",
        "defined + NAME",
    ] {
        invalid_condition(malformed);
    }
    rejected(&kernel(
        "#define CLOSE )\n#if defined(NAME CLOSE\n#endif",
        "",
    ));
    validate(&kernel("#define NAME 1\n#if !de\\\nfined(NAME)\ninvalid;\n#endif\n#ifdef defined\ninvalid;\n#endif\n#if 0\n#elifdef defined\ninvalid;\n#elifndef defined\n#define VALUE 7u\n#endif", "out[0] = VALUE;"));
}

#[test]
fn cuda_preprocessing_conditions_keep_macro_and_source_token_boundaries() {
    validate(&kernel(
        "#define PLUS +\n#define SUM 1 + 2\n#if 1 PLUS+ 2 != 3 || SUM * 3 != 7\ninvalid;\n#endif",
        "out[0] = 7u;",
    ));
    for (replacement, expression) in [
        ("&", "1 TOKEN& 1"),
        (">", "1 TOKEN> 0"),
        ("1", "TOKEN 2"),
        ("#", "TOKEN 1"),
    ] {
        rejected(&kernel(
            &format!("#define TOKEN {replacement}\n#if {expression}\n#endif"),
            "",
        ));
    }
    rejected_at(&kernel("#define HEX 0x\n#if HEX 10\n#endif", ""), "HEX");
}

#[test]
fn cuda_preprocessing_conditions_expand_aliases_and_replace_residual_identifiers_with_zero() {
    validate(&kernel(
        "#define OUTER INNER\n#define INNER 7\n#define SELF SELF\n#define LEFT RIGHT\n#define RIGHT LEFT\n\
         #if OUTER != 7 || SELF || LEFT || RIGHT\ninvalid;\n#endif\n\
         #undef INNER\n#define INNER 9\n#if OUTER != 9\ninvalid;\n#endif\n\
         #undef INNER\n#if OUTER || !defined(OUTER)\ninvalid;\n#endif",
        "out[0] = 11u;",
    ));
    rejected(&kernel("#define EMPTY\n#if EMPTY\n#endif", ""));
    rejected(&kernel(
        "#define QUERY defined(NAME)\n#if QUERY\n#endif",
        "",
    ));
    rejected(&kernel(
        "#define QUERY defined\n#if 1 || QUERY(NAME)\n#endif",
        "",
    ));
    validate(&kernel(
        "#define ALIAS defined\n#if !defined(ALIAS)\ninvalid;\n#endif",
        "unsigned defined = 7u; out[0] = ALIAS;",
    ));
    rejected(&kernel("#define ALIAS defined\n#if ALIAS\n#endif", ""));
}

#[test]
fn cuda_preprocessing_conditions_keep_boolean_words_and_zero_ordinary_keyword_names() {
    for expression in [
        "true && !false",
        "!int && !sizeof && !alignof && !unknown",
        "!__has_embed && !defined(__has_embed)",
    ] {
        branch(expression, true);
    }
    validate(&kernel("#define __has_embed 7\n#define __CUDACC__ 1\n#if __has_embed != 7 || !defined(__CUDACC__)\ninvalid;\n#endif", "out[0] = __has_embed;"));
    validate("__device__ int defined(int value) { return value; }\n#if !defined(defined)\n__global__ void kernel() { defined(7); }\n#endif");
}

#[test]
fn cuda_preprocessing_conditions_preserve_alternative_operator_spellings() {
    for expression in [
        "not false",
        "true and true",
        "false or true",
        "(3 bitand 1) == 1",
        "(1 bitor 2) == 3",
        "(3 xor 1) == 2",
        "(compl 0) == -1",
        "1 not_eq 2",
        "not false and (3 bitand 1) == 1 or false",
    ] {
        branch(expression, true);
    }
    for expression in ["1 and_eq 1", "1 or_eq 1", "1 xor_eq 1"] {
        invalid_condition(expression);
        invalid_condition(&format!("0 && ({expression})"));
    }
}

#[test]
fn cuda_preprocessing_conditions_reject_reserved_macro_names_and_operator_name_queries() {
    for name in [
        "defined", "and", "or", "not", "bitand", "bitor", "xor", "compl", "not_eq", "and_eq",
        "or_eq", "xor_eq",
    ] {
        rejected(&kernel(&format!("#define {name} 1"), ""));
        rejected(&kernel(&format!("#undef {name}"), ""));
        if name != "defined" {
            invalid_condition(&format!("defined({name})"));
            for directive in ["ifdef", "ifndef", "elifdef", "elifndef"] {
                let prefix = if directive.starts_with("elif") {
                    "#if 0\n"
                } else {
                    ""
                };
                rejected(&kernel(&format!("{prefix}#{directive} {name}\n#endif"), ""));
            }
        }
        validate(&kernel(
            &format!(
                "#if 0\n#define {name} 1\n#undef {name}\n#ifdef {name}\ninvalid;\n#endif\n#endif"
            ),
            "",
        ));
    }
}

#[test]
fn cuda_preprocessing_conditions_diagnose_needed_providers_before_arithmetic_laziness() {
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
        "__has_include",
        "__has_cpp_attribute",
    ] {
        for expression in [
            name.to_owned(),
            format!("defined({name})"),
            format!("0 && {name}"),
            format!("1 || defined({name})"),
        ] {
            invalid_condition(&expression);
        }
        for directive in ["define", "undef", "ifdef", "ifndef"] {
            let suffix = if directive == "define" { " 1" } else { "" };
            let close = if matches!(directive, "ifdef" | "ifndef") {
                "#endif\n"
            } else {
                ""
            };
            rejected(&kernel(
                &format!("#{directive} {name}{suffix}\n{close}"),
                "",
            ));
        }
        for directive in ["elifdef", "elifndef"] {
            rejected(&kernel(&format!("#if 0\n#{directive} {name}\n#endif"), ""));
        }
        validate(&kernel(
            &format!(
                "#if 0\n#define {name} 1\n#undef {name}\n#if {name}\ninvalid;\n#endif\n#endif"
            ),
            "out[0] = 7u;",
        ));
    }
    for expression in ["__has_include(<file.h>)", "__has_cpp_attribute(nodiscard)"] {
        invalid_condition(expression);
        invalid_condition(&format!("0 && {expression}"));
    }
    for name in ["__has_include", "__has_cpp_attribute"] {
        let error = rejected(&kernel("", &format!("out[0] = {name};")));
        assert!(error.message.contains("unsupported"), "{error}");
    }
    validate(&kernel(
        "#define LOCATION __LINE__\n#if 1\n#elif LOCATION\ninvalid;\n#endif",
        "",
    ));
    validate(&kernel(
        "#define UNUSED_INCLUDE __has_include\n#define UNUSED_ATTRIBUTE __has_cpp_attribute",
        "",
    ));
}

#[test]
fn cuda_preprocessing_conditions_accept_ordinary_ascii_characters_and_simple_escapes() {
    for expression in [
        "'A' == 65",
        "' ' == 32",
        "'~' == 126",
        r"'\'' == 39",
        r#"'\"' == 34"#,
        r"'\?' == 63",
        r"'\\' == 92",
        r"'\a' == 7",
        r"'\b' == 8",
        r"'\f' == 12",
        r"'\n' == 10",
        r"'\r' == 13",
        r"'\t' == 9",
        r"'\v' == 11",
        "'A' - 66 < 0",
        "~'A' == -66",
    ] {
        branch(expression, true);
    }
}

#[test]
fn cuda_preprocessing_conditions_decode_ascii_octal_and_hexadecimal_escapes() {
    for expression in [
        r"'\0' == 0",
        r"'\00' == 0",
        r"'\000' == 0",
        r"'\101' == 'A'",
        r"'\177' == 127",
        r"'\x41' == 'A'",
        r"'\x000041' == 65",
        r"'\x7f' == 127",
        r"'\x7F' == '\177'",
    ] {
        branch(expression, true);
    }
    for literal in [
        "''",
        "'AB'",
        "'é'",
        "'\u{80}'",
        "L'A'",
        "u'A'",
        "U'A'",
        "u8'A'",
        r"'\200'",
        r"'\x80'",
        r"'\xff'",
        r"'\x100'",
        r"'\1234'",
        r"'\08'",
        r"'\x'",
        r"'\8'",
        r"'\e'",
        r"'\u0041'",
        r"'\U00000041'",
        r"'\x{41}'",
        r"'\o{101}'",
        "'A'_suffix",
        "\"A\"",
        "R\"(A)\"",
    ] {
        invalid_condition(literal);
        invalid_condition(&format!("1 || {literal}"));
    }
}

#[test]
fn cuda_preprocessing_conditions_reject_malformed_and_unsupported_lazy_arm_syntax() {
    for expression in [
        "",
        "()",
        "1 2",
        "1 +",
        "*1",
        "&1",
        "1 ?",
        "1 ? : 2",
        "1 ? 2 3",
        "1 ? 2 : 3 : 4",
        "1 = 2",
        "1 += 2",
        "1++",
        "++1",
        "call(1)",
        "value.member",
        "value[0]",
        "(int)1",
        "int(1)",
        "sizeof(int)",
        "alignof(int)",
        "{1}",
        "\"text\"",
        "1.0f",
    ] {
        invalid_condition(expression);
        invalid_condition(&format!("1 ? 7 : ({expression})"));
    }
}

#[test]
fn cuda_preprocessing_conditions_select_ordered_if_elif_and_defined_alternatives() {
    validate(&kernel(
        "#define MODE 2\n#if MODE == 1\ninvalid;\n#elif MODE == 2\n#define VALUE 7u\n\
         #elif 1 / 0\ninvalid;\n#else\ninvalid;\n#endif\n\
         #if 0\ninvalid;\n#elifdef MISSING\ninvalid;\n#elifndef MISSING\n#define NEXT 11u\n\
         #else\ninvalid;\n#endif\n\
         #undef MODE\n#if MODE\ninvalid;\n#elifndef MODE\n#define LAST 13u\n#endif",
        "out[0] = VALUE + NEXT + LAST;",
    ));
    validate(&kernel(
        "#if 0\n#elif 0\n#else\n#define VALUE 17u\n#endif",
        "out[0] = VALUE;",
    ));
    for malformed in [
        "#elif 1",
        "#elifdef NAME",
        "#elifndef NAME",
        "#if 1\n#else\n#elif 1\n#endif",
        "#if 0\n#else\n#else\n#endif",
    ] {
        rejected(&kernel(malformed, ""));
    }
}

#[test]
fn cuda_preprocessing_conditions_skip_complete_ineligible_payloads_but_check_structure() {
    validate(&kernel(
        "#if 0\n#if invalid(( __LINE__\n#elifdef not a name\n#else ignored payload\n#endif ignored payload\n#endif\n\
         #if 1\n#elif 09 + (1 / 0) + __LINE__\n#elifdef malformed names\n#elifndef malformed names\n#endif",
        "out[0] = 7u;",
    ));
    for definitions in [
        "#if 1\n#else extra\n#endif",
        "#if 0\n#endif extra",
        "#if 0\n#if 1\n#else\n#else\n#endif\n#endif",
    ] {
        rejected(&kernel(definitions, ""));
    }
}

#[test]
fn cuda_preprocessing_conditions_diagnostics_use_original_operator_and_literal_spans() {
    for (expression, spelling) in [
        ("1 / 0", "/"),
        ("1 << 64", "<<"),
        ("09", "09"),
        ("1.0", "1.0"),
    ] {
        let source = kernel(&format!("/* 漢 */\n#if {expression}\n#endif"), "");
        rejected_at(&source, spelling);
    }
    for (replacement, invocation) in [("1 / 0", "FAULT"), ("09", "FAULT"), ("1 << 64", "FAULT")] {
        let source = kernel(
            &format!("#define FAULT {replacement}\n#if FAULT\n#endif"),
            "",
        );
        rejected_at(&source, invocation);
    }
    rejected_at(
        &kernel(
            "#define OUTER INNER\n#define INNER 09\n#if OUTER\n#endif",
            "",
        ),
        "OUTER",
    );
    rejected_at(
        &kernel("#define FAULT 09\n#if FA\\\nULT\n#endif", ""),
        "FA\\\nULT",
    );
    rejected_at(&kernel("#if 0\\\n9\n#endif", ""), "0\\\n9");
    rejected_at(
        &kernel("#define NAME __has_include\n#if NAME\n#endif", ""),
        "NAME",
    );
}

#[test]
fn cuda_preprocessing_conditions_do_not_enable_wide_or_character_shader_values() {
    for literal in ["2147483648", "1LL", "1z", "0x100000000", "'A'", "'\\n'"] {
        rejected(&kernel("#if 1\n#endif", &format!("out[0] = {literal};")));
    }
    let (module, _) = validate(FIXTURE);
    assert!(module.types.iter().all(|(_, ty)| match ty.inner {
        naga::TypeInner::Scalar(scalar) | naga::TypeInner::Vector { scalar, .. } =>
            scalar.width <= 4,
        _ => true,
    }));
    for function in module
        .functions
        .iter()
        .map(|(_, function)| function)
        .chain(module.entry_points.iter().map(|entry| &entry.function))
    {
        assert!(!function.expressions.iter().any(|(_, expression)| matches!(
            expression,
            naga::Expression::Literal(naga::Literal::I64(_) | naga::Literal::U64(_))
        )));
    }
}

#[test]
fn cuda_preprocessing_conditions_preserve_parser_lookahead_and_independent_parse_state() {
    let source = kernel(
        "#if 1\nstruct Record { int value; };\nusing Alias = Record;\n#define CHOICE 1\n#endif",
        "int value = (int(3.5f)); Alias record{7}; (Alias{9}).value;\n\
         for (Alias{11}; false; int(0)) {}\n\
         out[0] = (CHOICE ? record : Alias{13}).value + value;",
    );
    validate(&source);
    rejected(&kernel("#define STALE 1\n#if 1 +\n#endif", ""));
    branch("!defined(STALE)", true);
    validate(&source);
}

#[test]
fn cuda_preprocessing_conditions_bound_syntactic_nesting_without_limiting_flat_chains() {
    for expression in [
        format!("{}1{}", "(".repeat(64), ")".repeat(64)),
        format!("{}1", "!".repeat(64)),
        format!("{}1", "0?0:".repeat(64)),
        vec!["1"; 2048].join("+"),
        format!("({})", vec!["1"; 2048].join(",")),
    ] {
        branch(&expression, true);
    }
    for expression in [
        format!("{}1{}", "(".repeat(65), ")".repeat(65)),
        format!("{}1", "!".repeat(65)),
        format!("{}1", "0?0:".repeat(65)),
    ] {
        let error = rejected(&kernel(&format!("#if {expression}\n#endif"), ""));
        assert!(error.message.contains("limit"), "{error}");
    }
}

#[cfg_attr(miri, ignore)]
#[test]
fn cuda_preprocessing_conditions_share_expansion_token_work_and_spelling_budgets() {
    fn bounded(definitions: String) {
        let error = rejected(&kernel(&definitions, ""));
        assert!(
            error.message.contains("limit") || error.message.contains("large"),
            "{error}"
        );
    }
    let mut definitions = format!("#define Level0 {}\n", "+ 0 ".repeat(2048));
    for level in 1..=7 {
        definitions.push_str(&format!(
            "#define Level{level} Level{} Level{}\n",
            level - 1,
            level - 1
        ));
    }
    definitions.push_str("#if 1 Level7\n#endif\n#if 1 Level7\n#endif");
    bounded(definitions);
    let long_name = format!("Empty{}", "x".repeat(1024 * 1024));
    let mut definitions = format!("#define {long_name}\n#define Short0 {long_name}\n");
    for level in 1..=7 {
        definitions.push_str(&format!(
            "#define Short{level} Short{} Short{}\n",
            level - 1,
            level - 1
        ));
    }
    definitions.push_str("#if 0 && (1 Short7)\n#endif");
    bounded(definitions);
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_preprocessing_conditions_fixture_roundtrips_through_shared_wgsl_helpers() {
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
fn cuda_preprocessing_conditions_fixture_emits_valid_spirv_binary() {
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
        entry_point: "preprocessing_conditions".to_owned(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
    crate::helpers::validate_spirv(&words);
}
