#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

#[test]
fn cuda_reports_the_first_encountered_syntax_error() {
    for (source, expected) in [
        ("__global__ int wrong() {} @", "int"),
        ("__global__ void wrong() { int value = ; } 漢", ";"),
        (
            "__global__ void wrong() { unknown; } \"unterminated",
            "unknown",
        ),
        ("__global__ void empty() {} second @", "second"),
    ] {
        let error = parse_str(source, &Options::default()).unwrap_err();
        assert_eq!(&source[error.span.to_range().unwrap()], expected, "{error}");
    }
}

#[test]
fn cuda_source_phase_checks_precede_scanning() {
    for (source, expected) in [
        ("__global__ int wrong() {} // hidden \\\n", "\\\n"),
        ("__global__ int wrong() {} /* ??/ */", "??/"),
    ] {
        let error = parse_str(source, &Options::default()).unwrap_err();
        assert_eq!(&source[error.span.to_range().unwrap()], expected);
    }
}

#[test]
fn cuda_unreachable_and_trailing_source_still_reports_lexical_errors() {
    for (source, expected) in [
        ("__global__ void wrong() { return; @ }", "@"),
        (
            "__global__ void wrong() { while (true) { break; 漢 } }",
            "漢",
        ),
        ("__global__ void wrong() { for (;;) { continue; @ } }", "@"),
        ("__global__ void empty() {} /* 漢 */ @", "@"),
        (
            "__global__ void empty() {} /* unterminated",
            "/* unterminated",
        ),
        ("__global__ void wrong() { int @ }", "@"),
        ("__global__ void wrong(const @) {}", "@"),
        ("__global__ void wrong() { int value = (int @); }", "@"),
    ] {
        let error = parse_str(source, &Options::default()).unwrap_err();
        assert_eq!(&source[error.span.to_range().unwrap()], expected, "{error}");
    }
}

#[test]
fn cuda_cast_checkpoints_and_declaration_lookahead_preserve_parsing() {
    let source = "__global__ void casts(int *out) {
        int value = (int(3.5f));
        int(7.0f);
        unsigned(value);
        for (int(0); false; unsigned(value)) {}
        out[0] = ((unsigned(value))) + (int)(value + 1);
        while (true) { if (value) break; continue; }
    }";
    let module = parse_str(source, &Options::default()).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
}

#[test]
fn cuda_missing_closing_brace_uses_the_end_of_source_span() {
    let source = "__global__ void wrong() { return; /* 漢 */ // tail\r\n";
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert_eq!(error.span, naga::Span::from(source.len()..source.len()));
}
