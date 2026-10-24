use std::process::{Command, Output, Stdio};

use super::*;

fn normalize_assembly(source: &str) -> String {
    source
        .split_inclusive('\n')
        .map(|line| normalize_spirv_assembly_line(line))
        .collect()
}

fn assemble(source: &str) -> Output {
    let mut assembler = Command::new("spirv-as")
        .args(["--target-env", "spv1.1", "-o", "-", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run spirv-as");
    assembler
        .stdin
        .take()
        .unwrap()
        .write_all(source.as_bytes())
        .expect("failed to write assembly to spirv-as");
    assembler.wait_with_output().unwrap()
}

fn validate_binary(binary: &[u8]) {
    let mut validator = Command::new("spirv-val")
        .args(["--target-env", "spv1.1", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run spirv-val");
    validator.stdin.take().unwrap().write_all(binary).unwrap();
    let result = validator.wait_with_output().unwrap();
    assert!(
        result.status.success(),
        "spirv-val rejected assembly:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

fn switch_assembly(width: u32, signed: bool, labels: &[&str]) -> String {
    let capability = match width {
        16 => "OpCapability Int16\n",
        32 => "",
        64 => "OpCapability Int64\n",
        _ => unreachable!(),
    };
    let cases = labels
        .iter()
        .map(|label| format!("{label} %case"))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "OpCapability Shader
{capability}OpMemoryModel Logical GLSL450
OpEntryPoint GLCompute %main \"main\"
OpExecutionMode %main LocalSize 1 1 1
%void = OpTypeVoid
%function = OpTypeFunction %void
%integer = OpTypeInt {width} {signedness}
%selector = OpConstant %integer 0
%main = OpFunction %void None %function
%entry = OpLabel
OpSelectionMerge %merge None
OpSwitch %selector %merge {cases}
%case = OpLabel
OpBranch %merge
%merge = OpLabel
OpReturn
OpFunctionEnd
",
        signedness = u32::from(signed),
    )
}

fn assert_assembly_bits_match(width: u32, signed: bool, labels: &[&str], reference: &[&str]) {
    let source = switch_assembly(width, signed, labels);
    let expected = assemble(&switch_assembly(width, signed, reference));
    assert!(
        expected.status.success(),
        "reference failed to assemble:\n{}",
        String::from_utf8_lossy(&expected.stderr)
    );
    let actual = assemble(&normalize_assembly(&source));
    assert!(
        actual.status.success(),
        "normalized assembly failed:\n{}",
        String::from_utf8_lossy(&actual.stderr)
    );
    assert_eq!(
        actual.stdout, expected.stdout,
        "{width}-bit signed={signed}"
    );
    validate_binary(&actual.stdout);
}

#[test]
fn spirv_switch_literals_normalize_only_high_32_bit_case_operands() {
    let source = "  OpSwitch %4294967295 %default 0 %zero 2147483647 %maximum 2147483648 %minimum 4294967295 %minus_one -1 %negative 0xffffffff %hex 4294967296 %wide ; 4294967295 %comment\n";
    let normalized = normalize_spirv_assembly_line(source);
    assert_eq!(
        normalized.split_whitespace().collect::<Vec<_>>(),
        "OpSwitch %4294967295 %default 0 %zero 2147483647 %maximum 0x80000000 %minimum 0xffffffff %minus_one -1 %negative 0xffffffff %hex 4294967296 %wide ; 4294967295 %comment"
            .split_whitespace()
            .collect::<Vec<_>>()
    );
    assert!(normalized.ends_with('\n'));
    assert_eq!(
        normalize_spirv_assembly_line(
            "OpSwitch %selector %default 4294967295 %case ; OpExtInst DebugPrintf comment\n"
        ),
        "OpSwitch %selector %default 0xffffffff %case ; OpExtInst DebugPrintf comment\n"
    );
}

#[test]
fn spirv_switch_literals_preserve_other_text_and_debug_printf_normalization() {
    for source in [
        "; OpSwitch %selector %default 4294967295 %case\n",
        "%name = OpString \"OpSwitch %selector %default 4294967295 %case\"\n",
        "OpName %value \"4294967295\"\n",
        "%value = OpConstant %unsigned 4294967295\n",
        "OpSwitch %selector %default\n",
        "OpSwitch %selector %default -1 %negative 0X80000000 %hex 18446744073709551615 %wide\n",
    ] {
        assert_eq!(normalize_spirv_assembly_line(source), source);
    }
    assert_eq!(
        normalize_spirv_assembly_line("%17 = OpExtInst %void %15 DebugPrintf %16 %12\n"),
        "%17 = OpExtInst %void %15 1 %16 %12\n"
    );
}

#[cfg_attr(miri, ignore)]
#[test]
fn spirv_switch_literals_signed_32_bit_cases_assemble_with_exact_bits() {
    assert_assembly_bits_match(
        32,
        true,
        &["0", "2147483647", "2147483648", "4294967295"],
        &["0", "2147483647", "-2147483648", "-1"],
    );
}

#[cfg_attr(miri, ignore)]
#[test]
fn spirv_switch_literals_unsigned_32_bit_cases_keep_their_bits() {
    assert_assembly_bits_match(
        32,
        false,
        &["0", "2147483647", "2147483648", "4294967295"],
        &["0", "2147483647", "2147483648", "4294967295"],
    );
}

#[cfg_attr(miri, ignore)]
#[test]
fn spirv_switch_literals_do_not_truncate_other_integer_widths() {
    for (width, signed, labels) in [
        (16, true, vec!["0", "32767", "-32768", "-1"]),
        (16, false, vec!["0", "32767", "32768", "65535"]),
        (
            64,
            true,
            vec![
                "0",
                "2147483648",
                "4294967295",
                "4294967296",
                "9223372036854775807",
                "-1",
            ],
        ),
        (
            64,
            false,
            vec![
                "0",
                "2147483648",
                "4294967295",
                "4294967296",
                "18446744073709551615",
            ],
        ),
    ] {
        assert_assembly_bits_match(width, signed, &labels, &labels);
    }
}

#[cfg_attr(miri, ignore)]
#[test]
fn spirv_switch_literals_keep_out_of_range_assembly_invalid() {
    for (width, signed, label) in [
        (16, false, "4294967295"),
        (32, true, "4294967296"),
        (32, false, "4294967296"),
        (32, true, "18446744073709551615"),
    ] {
        let source = switch_assembly(width, signed, &[label]);
        let original = assemble(&source);
        let normalized = assemble(&normalize_assembly(&source));
        assert!(!original.status.success(), "control should be out of range");
        assert!(
            !normalized.status.success(),
            "normalization accepted an out-of-range literal: {source}"
        );
    }
}
