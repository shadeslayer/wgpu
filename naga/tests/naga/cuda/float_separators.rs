#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

fn returned_bits(module: &naga::Module, name: &str) -> u32 {
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
            naga::Statement::Return { value: Some(value) } => Some(value),
            _ => None,
        })
        .unwrap();
    let (value, negative) = match function.expressions[value] {
        naga::Expression::Unary {
            op: naga::UnaryOperator::Negate,
            expr,
        } => (expr, true),
        _ => (value, false),
    };
    let naga::Expression::Literal(naga::Literal::F32(value)) = function.expressions[value] else {
        panic!("expected a direct f32 literal");
    };
    if negative {
        (-value).to_bits()
    } else {
        value.to_bits()
    }
}

fn literal_bits(text: &str) -> u32 {
    let source = format!(
        "/* 漢 */ __device__ float value() {{ return {text}; }} __global__ void kernel() {{}}"
    );
    let module =
        parse_str(&source, &Options::default()).unwrap_or_else(|error| panic!("{text}: {error:?}"));
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    let function = module.functions.iter().next().unwrap().1;
    let (handle, _) = function
        .expressions
        .iter()
        .find(|(_, expression)| {
            matches!(expression, naga::Expression::Literal(naga::Literal::F32(_)))
        })
        .unwrap();
    assert_eq!(
        &source[function.expressions.get_span(handle).to_range().unwrap()],
        text.strip_prefix('-').unwrap_or(text)
    );
    returned_bits(&module, "value")
}

fn rejected(text: &str) -> (String, naga::front::cuda::ParseError) {
    let source = format!(
        "/* café 漢 */ __device__ float value() {{ return {text}; }} __global__ void kernel() {{}}"
    );
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
    assert_eq!(
        &source[error.span.to_range().unwrap()],
        text,
        "{text}: {error}"
    );
}

#[test]
fn cuda_previously_unsupported_decimal_separator_spellings_are_f32_values() {
    for (text, bits) in [
        ("1'0.0f", 0x4120_0000),
        ("1.0'1f", 0x3f81_47ae),
        ("1e1'0f", 0x5015_02f9),
    ] {
        assert_eq!(literal_bits(text), bits, "{text}");
    }
}

#[test]
fn cuda_decimal_float_separators_work_in_each_digit_sequence() {
    for (text, bits) in [
        ("1'2.5f", 0x4148_0000),
        (".1'25F", 0x3e00_0000),
        ("1'0.f", 0x4120_0000),
        ("1.25e+0'2f", 0x42fa_0000),
        ("1'2.3'75E+0'1F", 0x42f7_8000),
    ] {
        assert_eq!(literal_bits(text), bits, "{text}");
    }
}

#[test]
fn cuda_decimal_float_separators_do_not_make_leading_zeros_octal() {
    for (text, bits) in [
        ("0'01.25F", 0x3fa0_0000),
        ("0'9.0f", 0x4110_0000),
        ("0'9e+1F", 0x42b4_0000),
        ("0'0.0f", 0),
    ] {
        assert_eq!(literal_bits(text), bits, "{text}");
    }
}

#[test]
fn cuda_decimal_float_separators_preserve_round_to_nearest_ties_to_even() {
    for (text, bits) in [
        ("1.000'000'059'604'644'775'390'624f", 0x3f80_0000),
        ("1.000'000'059'604'644'775'390'625f", 0x3f80_0000),
        ("1.000'000'059'604'644'775'390'626f", 0x3f80_0001),
        ("1.000'000'178'813'934'326'171'875f", 0x3f80_0002),
        ("16'777'217.0f", 0x4b80_0000),
        ("16'777'219.0f", 0x4b80_0002),
    ] {
        assert_eq!(literal_bits(text), bits, "{text}");
    }
}

#[test]
fn cuda_decimal_float_separators_do_not_introduce_f64_double_rounding() {
    assert_eq!(
        literal_bits("1.000'000'059'604'644'775'390'625'000'000'000'000'1f"),
        0x3f80_0001
    );
}

#[test]
fn cuda_decimal_float_separators_preserve_normal_and_subnormal_literal_bits() {
    for (text, bits) in [
        ("1.401'298'464'324'817e-4'5f", 1),
        ("1.175'494'210'692'441'1e-3'8f", 0x007f_ffff),
        ("1.175'494'350'822'287'5e-3'8f", 0x0080_0000),
        ("7.006'492'321'624'085e-46f", 0),
        ("7.006'492'321'624'086e-46f", 1),
    ] {
        assert_eq!(literal_bits(text), bits, "{text}");
    }
}

#[test]
fn cuda_decimal_float_separators_preserve_underflow_and_signed_zero() {
    for (text, bits) in [
        ("1'0.0e-999'999f", 0),
        ("0'0.0e+999'999F", 0),
        ("-0'0.0f", 0x8000_0000),
        ("-1.0e-999'999f", 0x8000_0000),
    ] {
        assert_eq!(literal_bits(text), bits, "{text}");
    }
}

#[test]
fn cuda_decimal_float_separators_keep_the_existing_finite_range_policy() {
    for text in ["3.402'823'466'385'288'6e+3'8f", "3.402'823'5e38f"] {
        assert_eq!(literal_bits(text), 0x7f7f_ffff, "{text}");
    }
    for text in ["3.402'823'6e+3'8f", "1.0e999'999f"] {
        rejected_token(text);
    }
}

#[test]
fn cuda_decimal_float_separators_reject_invalid_placement_near_dot_exponent_and_suffix() {
    for text in [
        "1'.0f", "1.'0f", ".'1f", "1.0'e1f", "1.0e'1f", "1.0e+'1f", "1.0e-'1f", "1.0e1'f", "1.0'f",
        "1.0f'1",
    ] {
        rejected(text);
    }
}

#[test]
fn cuda_decimal_float_separators_reject_repeated_quotes_underscores_and_non_ascii() {
    for text in [
        "1.0''1f",
        "1''0.0f",
        "1.0e1''0f",
        "1.0_f",
        "1.0e_1f",
        "1_0.0f",
        "1.0'漢f",
        "1.0漢f",
    ] {
        rejected(text);
    }
}

#[test]
fn cuda_malformed_decimal_float_preprocessing_numbers_retain_complete_spans() {
    for text in ["1'0.0ff", "1'0.0e+f", "1'0.0e1.0f", "1.0e+'1f", "1.0'f"] {
        rejected_token(text);
    }
}

#[test]
fn cuda_decimal_float_separators_do_not_enable_double_hexadecimal_or_extended_suffixes() {
    for text in [
        "1'0f",
        "1'0.0",
        "1'0.0L",
        "1'0.0f32",
        "1'0.0h",
        "0x1p+0f",
        "0x1'0p+2f",
        "0x1.fp-2F",
    ] {
        rejected_token(text);
    }
}

#[test]
fn cuda_decimal_float_separators_do_not_make_floats_shared_array_extents() {
    for extent in ["1'6.0f", "1.6e0'1f", "0'1.0f"] {
        let source =
            format!("/* 漢 */ __global__ void kernel() {{ __shared__ int values[{extent}]; }}");
        let error = parse_str(&source, &Options::default()).unwrap_err();
        assert_eq!(&source[error.span.to_range().unwrap()], extent, "{error}");
    }
}

#[test]
fn cuda_malformed_decimal_float_separators_are_checked_in_unreachable_source() {
    let source = "__global__ void kernel() { return; float value = 1.0e+'1f; }";
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert_eq!(&source[error.span.to_range().unwrap()], "1.0e+'1f");
}

#[test]
fn cuda_decimal_float_separators_handle_long_significands_and_exponents() {
    for (text, bits) in [
        (format!("{}1.0f", "0'".repeat(1024)), 0x3f80_0000),
        (format!("1.{}1f", "0'".repeat(1024)), 0x3f80_0000),
        (format!("1.0e+{}1f", "0'".repeat(1024)), 0x4120_0000),
        (format!("1.0e-{}9f", "9'".repeat(1024)), 0),
        (format!("0.0e+{}9f", "9'".repeat(1024)), 0),
    ] {
        assert_eq!(literal_bits(&text), bits);
    }
    rejected_token(&format!("1.0e+{}9f", "9'".repeat(1024)));
}

#[test]
fn cuda_decimal_float_separators_work_with_scalar_calls_and_conversions() {
    let (module, _) = validate(
        "__device__ float adjust(float value, unsigned amount) { return value + amount + 1'0.0f; }
        __global__ void kernel(float *out, int count) {
            float value = adjust(.1'25F, count);
            out[0] = value;
            out[1] = static_cast<float>(int(1'0.5f));
            bool enabled = 0.0'1f;
            out[2] = enabled ? 2'0.0f : 0.0f;
        }",
    );
    let result = module
        .functions
        .iter()
        .next()
        .unwrap()
        .1
        .result
        .as_ref()
        .unwrap();
    assert_eq!(
        module.types[result.ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::F32)
    );
}

#[test]
#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
fn cuda_decimal_float_separator_values_roundtrip_through_wgsl() {
    let (module, info) = validate(
        "__device__ float tiny() { return 1.401'298'464'324'817e-4'5f; }
        __device__ float halfway() { return 1.000'000'059'604'644'775'390'625f; }
        __global__ void kernel(float *out) { out[0] = tiny(); out[1] = halfway(); out[2] = 1'2.5f; }",
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
    assert_eq!(returned_bits(&reparsed, "tiny"), 1);
    assert_eq!(returned_bits(&reparsed, "halfway"), 0x3f80_0000);
}
