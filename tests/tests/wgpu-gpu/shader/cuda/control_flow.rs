use super::super::{ShaderBuffer, MAX_BUFFER_SIZE};
use super::{case, module_case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    let mut cases = Vec::new();
    for (expression, expected) in [
        ("0", false),
        ("-3", true),
        ("0u", false),
        ("2u", true),
        ("0.0f", false),
        ("-0.0f", false),
        ("-0.5f", true),
        ("!0", true),
        ("!3u", false),
        ("!-0.5f", false),
    ] {
        let source = format!("__global__ void truth(int *out) {{ if ({expression}) out[0] = 1; }}");
        cases.push(case(
            &format!(
                "cuda::control_flow::cuda_scalar_conditions_compare_against_zero/{expression}"
            ),
            &source,
            &[0],
            &[i32::from(expected)],
        ));
    }
    for (operator, expected) in [
        ("<", true),
        ("<=", true),
        (">", false),
        (">=", false),
        ("==", false),
        ("!=", true),
    ] {
        let source =
            format!("__global__ void compare(int *out) {{ if (2 {operator} 3) out[0] = 1; }}");
        cases.push(case(
            &format!("cuda::control_flow::cuda_comparisons_produce_boolean_conditions/{operator}"),
            &source,
            &[0],
            &[i32::from(expected)],
        ));
    }
    for operator in ["&&", "||"] {
        let source = format!("__global__ void logical(const int *lhs, const int *rhs, int *out) {{ if (lhs[0] {operator} rhs[0]) out[0] = 1; }}");
        for lhs in [0_i32, 1] {
            let rhs = i32::from(operator == "&&");
            let expected = lhs != 0;
            let mut initial = vec![-1_i32; MAX_BUFFER_SIZE as usize / 4];
            initial[0] = 0;
            let mut output = initial.clone();
            output[0] = i32::from(expected);
            cases.push(
                module_case(
                    &format!("cuda::control_flow::cuda_logical_operators_short_circuit_buffer_loads/{operator}/lhs{lhs}"),
                    &source,
                    [1, 1, 1],
                    &output,
                )
                .module_buffers(
                    vec![
                        ShaderBuffer::new(
                            0,
                            wgpu::BufferBindingType::Storage { read_only: true },
                            &[lhs],
                        ),
                        ShaderBuffer::new(
                            1,
                            wgpu::BufferBindingType::Storage { read_only: true },
                            &[rhs],
                        ),
                        ShaderBuffer::new(
                            2,
                            wgpu::BufferBindingType::Storage { read_only: false },
                            &initial,
                        ),
                    ],
                    2,
                    [1, 1, 1],
                ),
            );
        }
    }
    assert_eq!(cases.len(), 20);
    cases
}
