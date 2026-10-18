#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::{execute, validate};

fn literal(text: &str) -> naga::Literal {
    let source = format!("__global__ void kernel() {{ unsigned int value = {text}; }}");
    let (module, _) = validate(&source);
    let function = &module.entry_points[0].function;
    let (handle, value) = function
        .expressions
        .iter()
        .find_map(|(handle, expression)| match *expression {
            naga::Expression::Literal(value) => Some((handle, value)),
            _ => None,
        })
        .unwrap();
    assert_eq!(
        &source[function.expressions.get_span(handle).to_range().unwrap()],
        text
    );
    value
}

fn rejected(text: &str) -> (String, naga::front::cuda::ParseError) {
    let source = format!("/* 漢 */ __global__ void kernel() {{ unsigned int value = {text}; }}");
    let error = parse_str(&source, &Options::default()).unwrap_err();
    assert!(!error.message.is_empty(), "{text}");
    assert!(
        source.get(error.span.to_range().unwrap()).is_some(),
        "{text}"
    );
    (source, error)
}

fn rejected_token(text: &str) {
    let (source, error) = rejected(text);
    assert_eq!(&source[error.span.to_range().unwrap()], text, "{error}");
}

#[test]
fn cuda_decimal_integer_literal_types_and_limits_are_preserved() {
    for (text, expected) in [
        ("0", naga::Literal::I32(0)),
        ("2147483647", naga::Literal::I32(i32::MAX)),
        ("0u", naga::Literal::U32(0)),
        ("2147483648u", naga::Literal::U32(2_147_483_648)),
        ("4294967295U", naga::Literal::U32(u32::MAX)),
    ] {
        assert_eq!(literal(text), expected, "{text}");
    }
}

#[test]
fn cuda_integer_literals_recognize_octal_hexadecimal_and_binary_bases() {
    for (text, value) in [
        ("00", 0),
        ("0007", 7),
        ("010", 8),
        ("077", 63),
        ("0x10", 16),
        ("0X7f", 127),
        ("0b101", 5),
        ("0B00111", 7),
    ] {
        assert_eq!(literal(text), naga::Literal::I32(value), "{text}");
    }
}

#[test]
fn cuda_unsuffixed_nondecimal_literals_select_int_then_unsigned_int() {
    for (text, expected) in [
        ("017777777777", naga::Literal::I32(i32::MAX)),
        ("020000000000", naga::Literal::U32(2_147_483_648)),
        ("037777777777", naga::Literal::U32(u32::MAX)),
        ("0x7fffffff", naga::Literal::I32(i32::MAX)),
        ("0x80000000", naga::Literal::U32(2_147_483_648)),
        ("0xFFFFFFFF", naga::Literal::U32(u32::MAX)),
        (
            "0b1111111111111111111111111111111",
            naga::Literal::I32(i32::MAX),
        ),
        (
            "0b10000000000000000000000000000000",
            naga::Literal::U32(2_147_483_648),
        ),
        (
            "0B11111111111111111111111111111111",
            naga::Literal::U32(u32::MAX),
        ),
    ] {
        assert_eq!(literal(text), expected, "{text}");
    }
}

#[test]
fn cuda_unsigned_suffixes_apply_to_every_integer_base() {
    for text in ["7u", "07U", "0x7u", "0X7U", "0b111u", "0B111U"] {
        assert_eq!(literal(text), naga::Literal::U32(7), "{text}");
    }
}

#[test]
fn cuda_digit_separators_preserve_integer_values_and_types() {
    for (text, expected) in [
        ("1'000'000", naga::Literal::I32(1_000_000)),
        ("4'294'967'295u", naga::Literal::U32(u32::MAX)),
        ("0'10", naga::Literal::I32(8)),
        ("037'777'777'777", naga::Literal::U32(u32::MAX)),
        ("0x7fff'ffff", naga::Literal::I32(i32::MAX)),
        ("0XFFFF'FFFFU", naga::Literal::U32(u32::MAX)),
        ("0b1'010", naga::Literal::I32(10)),
        (
            "0B1000'0000'0000'0000'0000'0000'0000'0000",
            naga::Literal::U32(2_147_483_648),
        ),
    ] {
        assert_eq!(literal(text), expected, "{text}");
    }
}

#[test]
fn cuda_hexadecimal_f_digits_are_not_float_suffixes() {
    for text in ["0xf", "0xF", "0Xff", "0xF'F", "0xffu", "0XFFU"] {
        let expected = if text.ends_with(['u', 'U']) {
            naga::Literal::U32(255)
        } else if text.len() == 3 {
            naga::Literal::I32(15)
        } else {
            naga::Literal::I32(255)
        };
        assert_eq!(literal(text), expected, "{text}");
    }
}

#[test]
fn cuda_nondecimal_signedness_controls_arithmetic_and_comparisons() {
    let source = "__global__ void kernel(int *out) {
        out[0] = -1 < 0xffffffff;
        out[1] = -1 < 0x7fffffff;
        out[2] = (037777777777 + 1u) == 0u;
        out[3] = (-0b11111111111111111111111111111111) == 1u;
    }";
    assert_eq!(execute(source, &[9; 4]), [0, 1, 1, 1]);
}

#[test]
fn cuda_integer_bases_and_separators_preserve_arithmetic_precedence() {
    let source = "__global__ void kernel(int *out) {
        out[0] = 010 + 0x10 * 0b10 - 1'0;
        out[1] = 0x1e + 2;
        out[2] = 0x1E/* separated */+2;
    }";
    assert_eq!(execute(source, &[0; 3]), [30, 32, 32]);
}

#[test]
fn cuda_quoted_hexadecimal_exponent_digits_do_not_absorb_adjacent_operators() {
    let source = "__global__ void kernel(int *out) {
        out[0] = 0x1'e+2;
        out[1] = 0x1'E-2;
    }";
    assert_eq!(execute(source, &[0; 2]), [32, 28]);
}

#[test]
fn cuda_integer_literal_range_depends_on_the_token_type() {
    for text in [
        "2147483648",
        "4294967295",
        "4294967296u",
        "0x100000000",
        "040000000000",
        "0b100000000000000000000000000000000",
        "4'294'967'296U",
        "0x1'0000'0000u",
    ] {
        rejected_token(text);
    }
    let (source, error) = rejected("-2147483648");
    assert_eq!(&source[error.span.to_range().unwrap()], "2147483648");
}

#[test]
fn cuda_integer_literals_reject_invalid_base_digits_and_missing_digits() {
    for text in [
        "0x", "0X", "0b", "0B", "08", "09u", "0b102", "0B2", "0xg", "0x1g", "0b1f", "0o7",
    ] {
        rejected_token(text);
    }
}

#[test]
fn cuda_integer_literals_reject_malformed_digit_separators() {
    for text in [
        "1''2", "1'", "1'u", "0x'1", "0b'1", "0''7", "0'8", "0x1'u", "0b1'2", "1'_0",
    ] {
        rejected(text);
    }
}

#[test]
fn cuda_integer_literals_reject_unsupported_and_repeated_suffixes() {
    for text in [
        "1L", "1l", "1LL", "1ll", "1uL", "1LU", "1ULL", "1llU", "1z", "1UZ", "1uu", "1uU", "1lul",
        "1lL", "0xffffuu", "1i", "1_kg",
    ] {
        rejected_token(text);
    }
}

#[test]
fn cuda_invalid_preprocessing_numbers_are_not_split_into_expressions() {
    for text in [
        "0x1e+2", "0x1E-2", "0x1p+2", "0X1P-2", "1e+", "1e-", "1p+2", "1P-2", "1e+2foo", "1.0ff",
        "1.0f.5",
    ] {
        rejected_token(text);
    }
}

#[test]
fn cuda_decimal_float_syntax_is_preserved() {
    for (text, expected) in [
        ("0.0f", 0.0),
        (".5F", 0.5),
        ("1.f", 1.0),
        ("01.25f", 1.25),
        ("09e+1F", 90.0),
        ("1e-2f", 0.01),
        ("1.e+2f", 100.0),
    ] {
        assert_eq!(literal(text), naga::Literal::F32(expected), "{text}");
    }
}

#[test]
fn cuda_float_extensions_and_nonfinite_literals_remain_unsupported() {
    for text in [
        "1f",
        "1.0",
        "1.0L",
        "1.0f32",
        "0x1p+0f",
        "0x1.fp-2F",
        "1e9999f",
    ] {
        rejected_token(text);
    }
}

#[test]
fn cuda_shared_array_extents_use_integer_literal_values() {
    for extent in [
        "16", "020", "0x10", "0X10U", "0b10000u", "0B1'0000", "0'20", "1'6U",
    ] {
        let (module, _) = validate(&format!(
            "__global__ void kernel() {{ __shared__ int values[{extent}]; }}"
        ));
        let global = &module.global_variables.iter().next().unwrap().1;
        let naga::TypeInner::Array {
            size: naga::ArraySize::Constant(size),
            stride,
            ..
        } = module.types[global.ty].inner
        else {
            panic!("expected shared array: {extent}");
        };
        assert_eq!(size.get(), 16, "{extent}");
        assert_eq!(stride, 4);
        assert_eq!(global.space, naga::AddressSpace::WorkGroup);
        assert_eq!(global.binding, None);
    }
}

#[test]
fn cuda_shared_integer_extents_reject_zero_overflow_and_invalid_tokens() {
    for extent in [
        "00",
        "0x0",
        "0b0U",
        "0'0",
        "0x20000000",
        "04000000000",
        "0x100000000",
        "08",
        "0b2",
        "1UL",
        "1.0f",
    ] {
        let source =
            format!("/* 漢 */ __global__ void kernel() {{ __shared__ int values[{extent}]; }}");
        let error = parse_str(&source, &Options::default()).unwrap_err();
        assert_eq!(&source[error.span.to_range().unwrap()], extent, "{error}");
    }
}

#[test]
fn cuda_invalid_literals_are_checked_after_unreachable_jumps() {
    let source = "__global__ void kernel() { return; unsigned int value = 0x1p+2; }";
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert_eq!(&source[error.span.to_range().unwrap()], "0x1p+2");
}

#[test]
fn cuda_long_integer_tokens_do_not_overflow_or_require_recursive_parsing() {
    for text in [
        "9".repeat(4096),
        format!("0x{}", "f".repeat(4096)),
        format!("0b1{}", "0".repeat(4096)),
    ] {
        rejected_token(&text);
    }
    assert_eq!(
        literal(&format!("{}7", "0".repeat(4096))),
        naga::Literal::I32(7)
    );
}

#[test]
#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
fn cuda_integer_literal_types_and_shared_extents_roundtrip_through_wgsl() {
    let (module, info) = validate(
        "__device__ unsigned mask(unsigned value) { return value & 0xffff'ffff; }
        __global__ void kernel(unsigned *out, unsigned count) {
            __shared__ unsigned values[0x10];
            unsigned i = threadIdx.x;
            if (i < count && i < 020u) {
                values[i] = mask(0B1000'0000'0000'0000'0000'0000'0000'0000 + i);
                out[i] = values[i] + 1'000U;
            }
        }",
    );
    let text =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let reparsed = naga::front::wgsl::parse_str(&text).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&reparsed)
    .unwrap();
}
