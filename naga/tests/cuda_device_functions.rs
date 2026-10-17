#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

fn validate(source: &str) -> naga::Module {
    let module = parse_str(source, &Options::default()).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    module
}

fn function<'a>(
    module: &'a naga::Module,
    name: &str,
) -> (naga::Handle<naga::Function>, &'a naga::Function) {
    module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some(name))
        .unwrap()
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

fn assert_rejected(source: &str) {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(source.get(range).is_some(), "{source}: {error}");
    assert!(!error.message.is_empty());
}

#[test]
fn cuda_device_scalar_signatures_preserve_the_kernel_resource_abi() {
    let module = validate(
        "__device__ float mix(float a, int b, unsigned c, bool d) { return a + b + c + d; }
        __device__ int integer(float value) { return value; }
        __device__ unsigned natural(int value) { return value; }
        __device__ bool truth(float value) { return value; }
        __global__ void kernel(float *out, int count, const float *input) {
            out[0] = mix(input[0], integer(3.5f), natural(count), truth(-1.0f));
        }",
    );
    assert_eq!(module.functions.len(), 4);
    assert_eq!(module.entry_points.len(), 1);
    let (_, mix) = function(&module, "mix");
    for (argument, scalar) in mix.arguments.iter().zip([
        naga::Scalar::F32,
        naga::Scalar::I32,
        naga::Scalar::U32,
        naga::Scalar::BOOL,
    ]) {
        assert_eq!(
            module.types[argument.ty].inner,
            naga::TypeInner::Scalar(scalar)
        );
        assert_eq!(argument.binding, None);
    }
    for (name, scalar) in [
        ("mix", naga::Scalar::F32),
        ("integer", naga::Scalar::I32),
        ("natural", naga::Scalar::U32),
        ("truth", naga::Scalar::BOOL),
    ] {
        let result = function(&module, name).1.result.as_ref().unwrap();
        assert_eq!(
            module.types[result.ty].inner,
            naga::TypeInner::Scalar(scalar)
        );
        assert_eq!(result.binding, None);
    }
    assert_eq!(module.global_variables.len(), 3);
    for (binding, (_, variable)) in module.global_variables.iter().enumerate() {
        assert_eq!(
            variable.binding,
            Some(naga::ResourceBinding {
                group: 0,
                binding: binding as u32
            })
        );
    }
    assert_eq!(module.entry_points[0].name, "kernel");
}

#[test]
fn cuda_device_calls_convert_each_argument_to_its_parameter_type() {
    validate(
        "__device__ int select(float a, int b, unsigned int c, bool d) {
            if (d) return int(a) + b + int(c);
            return -1;
        }
        __global__ void kernel(int *out) { out[0] = select(2, 3.75f, -1, 0.5f); }",
    );
}

#[test]
fn cuda_device_parameters_are_mutable_local_copies() {
    let module = validate(
        "__device__ int advance(int value) { value += 2; return value++; }
        __global__ void kernel(int *out) { int value = 3; out[0] = advance(value); out[1] = value; }",
    );
    let (_, helper) = function(&module, "advance");
    assert_eq!(helper.arguments.len(), 1);
    let (local, _) = helper
        .local_variables
        .iter()
        .find(|(_, local)| local.name.as_deref() == Some("value"))
        .unwrap();
    let pointer = helper
        .expressions
        .iter()
        .find_map(|(handle, expression)| {
            matches!(expression, naga::Expression::LocalVariable(candidate) if *candidate == local)
                .then_some(handle)
        })
        .unwrap();
    assert!(statements(&helper.body).iter().any(|statement| {
        matches!(statement, naga::Statement::Store { pointer: stored, value } if *stored == pointer && matches!(helper.expressions[*value], naga::Expression::FunctionArgument(0)))
    }));
    assert!(helper
        .expressions
        .iter()
        .all(|(_, expression)| !matches!(expression, naga::Expression::GlobalVariable(_))));
}

#[test]
fn cuda_device_const_parameters_accept_both_qualifier_positions() {
    validate(
        "__device__ int sum(const int a, int const b) { return a + b; }
        __global__ void kernel(int *out) { out[0] = sum(1, 2); }",
    );
    for body in ["value = 1;", "++value;", "value += 1;"] {
        assert_rejected(&format!(
            "__device__ int fixed(const int value) {{ {body} return value; }} __global__ void kernel() {{}}"
        ));
    }
}

#[test]
fn cuda_device_nested_calls_reference_previously_defined_functions() {
    let module = validate(
        "__device__ int increment(int value) { return value + 1; }
        __device__ int twice(int value) { return increment(increment(value)); }
        __global__ void kernel(int *out) { increment(4); out[0] = twice(increment(1)); }",
    );
    let (increment, _) = function(&module, "increment");
    let (twice, helper) = function(&module, "twice");
    let targets: Vec<_> = statements(&helper.body)
        .into_iter()
        .filter_map(|statement| match statement {
            naga::Statement::Call { function, .. } => Some(*function),
            _ => None,
        })
        .collect();
    assert_eq!(targets, [increment, increment]);
    let targets: Vec<_> = statements(&module.entry_points[0].function.body)
        .into_iter()
        .filter_map(|statement| match statement {
            naga::Statement::Call { function, .. } => Some(*function),
            _ => None,
        })
        .collect();
    assert_eq!(targets, [increment, increment, twice]);
}

#[test]
fn cuda_device_argument_values_are_captured_before_following_arguments() {
    let module = validate(
        "__device__ int pair(int first, int second) { return first * 10 + second; }
        __global__ void kernel(int *out) { int value = 3; out[0] = pair(value, value++); }",
    );
    let kernel = &module.entry_points[0].function;
    let body = statements(&kernel.body);
    let (call_index, arguments) = body
        .iter()
        .enumerate()
        .find_map(|(index, statement)| match statement {
            naga::Statement::Call { arguments, .. } => Some((index, arguments)),
            _ => None,
        })
        .unwrap();
    let naga::Expression::Load { pointer } = kernel.expressions[arguments[0]] else {
        panic!("first argument must snapshot the local value")
    };
    let load_index = body.iter().position(|statement| {
        matches!(statement, naga::Statement::Emit(range) if range.clone().any(|handle| handle == arguments[0]))
    }).unwrap();
    let mutation_index = body.iter().enumerate().find_map(|(index, statement)| {
        (index > load_index && matches!(statement, naga::Statement::Store { pointer: stored, .. } if *stored == pointer)).then_some(index)
    }).unwrap();
    assert!(load_index < mutation_index && mutation_index < call_index);
}

#[test]
fn cuda_device_calls_work_in_branch_loop_and_short_circuit_scopes() {
    validate(
        "__device__ bool positive(int value) { return value > 0; }
        __device__ int accumulated(int count) {
            int sum = 0;
            for (int i = 0; positive(count - i); ++i) {
                if (positive(i) && positive(sum += i)) continue;
                sum += 1;
            }
            return sum;
        }
        __global__ void kernel(int *out) { if (positive(3)) out[0] = accumulated(3); }",
    );
}

#[test]
fn cuda_device_returns_accept_tail_and_complete_branch_paths() {
    validate(
        "__device__ int absolute(int value) { if (value < 0) return -value; return value; }
        __device__ bool sign(int value) { if (value < 0) { return false; } else { return true; } }
        __device__ int nested(int value) { { if (value) return absolute(value); else { return 0; } } }
        __global__ void kernel(int *out) { out[0] = nested(2) + sign(-1); }",
    );
}

#[test]
fn cuda_device_return_values_convert_to_the_declared_type() {
    validate(
        "__device__ float floating(int value) { return value; }
        __device__ int integral(float value) { return value; }
        __device__ unsigned natural(bool value) { return value; }
        __device__ bool truth(int value) { return value; }
        __global__ void kernel(float *out) { out[0] = floating(2) + integral(3.75f) + natural(true) + truth(-2); }",
    );
}

#[test]
fn cuda_device_calls_in_unreachable_code_leave_valid_ir() {
    for source in [
        "__device__ int helper() { return 1; } __global__ void kernel() { return; helper(); }",
        "__device__ int first(int value) { return value; } __device__ int second(int value) { return value; first(value); } __global__ void kernel() { second(1); }",
        "__device__ int helper() { return 1; } __global__ void kernel() { return; { helper(); { int value = helper(); } } }",
    ] {
        let module = validate(source);
        for (_, helper) in module.functions.iter() {
            if helper.name.as_deref() == Some("second") {
                assert!(!statements(&helper.body).iter().any(|statement| matches!(statement, naga::Statement::Call { .. })));
            }
        }
        if !source.contains("second") {
            assert!(!statements(&module.entry_points[0].function.body).iter().any(|statement| matches!(statement, naga::Statement::Call { .. })));
        }
    }
}

#[test]
fn cuda_device_names_can_be_shadowed_as_values() {
    validate(
        "__device__ int helper(int value) { return value; }
        __device__ int parameter(int helper) { { int helper = 3; helper += 1; } return helper; }
        __global__ void kernel(int *out, int helper) { out[0] = parameter(helper); { int helper = 2; out[1] = helper; } }",
    );
    validate(
        "__device__ int helper(int value) { return value; }
        __global__ void kernel(int *out) { { int helper = 2; out[0] = helper; } out[1] = helper(3); }",
    );
}

#[test]
fn cuda_device_local_names_hide_helpers_in_their_initializers() {
    for source in [
        "__device__ int helper(int value) { return value; } __global__ void kernel(int *out) { int helper = helper(1); out[0] = helper; }",
        "__device__ int helper(int value) { return value; } __device__ int shadow() { int helper = helper(1); return helper; } __global__ void kernel() {}",
        "__device__ int helper(int value) { return value; } __global__ void kernel() { for (int helper = helper(1); helper < 2; ++helper) {} }",
        "__device__ int helper(int value) { return value; } __global__ void kernel() { return; { int helper = helper(1); } }",
    ] {
        assert_rejected(source);
    }
}

#[test]
fn cuda_device_functions_reset_scope_expression_depth_and_builtin_state() {
    let arithmetic = core::iter::repeat_n("value", 40)
        .collect::<Vec<_>>()
        .join(" + ");
    let source = format!(
        "__device__ int first(int value) {{ {{ {{ int local = {arithmetic}; return local; }} }} }}
        __device__ int second(int value) {{ {{ {{ int local = {arithmetic}; return first(local); }} }} }}
        __device__ int third(int value) {{ {{ {{ int local = {arithmetic}; return second(local); }} }} }}
        __global__ void kernel(int *out) {{ unsigned lane = threadIdx.x; out[lane] = third(1); }}"
    );
    let module = validate(&source);
    for (_, helper) in module.functions.iter() {
        assert_eq!(helper.arguments.len(), 1);
        assert_eq!(helper.arguments[0].binding, None);
        assert!(helper
            .expressions
            .iter()
            .all(|(_, expression)| !matches!(expression, naga::Expression::GlobalVariable(_))));
    }
    assert_eq!(module.entry_points[0].function.arguments.len(), 1);
    assert_eq!(
        module.entry_points[0].function.arguments[0].binding,
        Some(naga::Binding::BuiltIn(naga::BuiltIn::LocalInvocationId))
    );
}

#[test]
fn cuda_device_calls_reject_incorrect_arity() {
    for call in ["sum()", "sum(1)", "sum(1, 2, 3)"] {
        assert_rejected(&format!(
            "__device__ int sum(int a, int b) {{ return a + b; }} __global__ void kernel() {{ {call}; }}"
        ));
    }
}

#[test]
fn cuda_device_functions_require_a_value_on_every_supported_path() {
    for body in [
        "",
        "return;",
        "if (value) return 1;",
        "while (value) { return 1; }",
        "for (;;) { return 1; }",
    ] {
        assert_rejected(&format!(
            "__device__ int helper(int value) {{ {body} }} __global__ void kernel() {{}}"
        ));
    }
    assert_rejected(
        "__device__ int helper() { return 1; } __global__ void kernel() { return helper(); }",
    );
}

#[test]
fn cuda_device_unknown_and_shadowed_calls_retain_identifier_spans() {
    for (body, expected) in [
        ("missing(1);", "missing"),
        ("int helper = 1; helper(2);", "helper(2)"),
        ("return; missing(1);", "missing"),
    ] {
        let source = format!("__device__ int helper(int value) {{ return value; }} __global__ void kernel() {{ {body} }}");
        let error = parse_str(&source, &Options::default()).unwrap_err();
        let start = source.rfind(expected).unwrap();
        assert_eq!(
            error.span.to_range().unwrap(),
            start..start + expected.split('(').next().unwrap().len()
        );
    }
}

#[test]
fn cuda_device_functions_do_not_leak_parameters_or_locals() {
    for source in [
        "__device__ int first(int private_value) { return private_value; } __device__ int second() { return private_value; } __global__ void kernel() {}",
        "__device__ int first() { int private_value = 1; return private_value; } __global__ void kernel(int *out) { out[0] = private_value; }",
        "__device__ int first(int value) { int value = 1; return value; } __global__ void kernel() {}",
        "__device__ int first(int value, int value) { return value; } __global__ void kernel() {}",
        "__device__ int helper(int value) { return value; } __device__ int shadow(int helper) { return helper(1); } __global__ void kernel() {}",
        "__device__ int helper(int value) { return value; } __global__ void kernel(int helper) { helper(1); }",
    ] {
        assert_rejected(source);
    }
}

#[test]
fn cuda_device_functions_reject_recursion_forward_calls_and_overloads() {
    for source in [
        "__device__ int recurse(int value) { return recurse(value); } __global__ void kernel() {}",
        "__device__ int first(int value) { return second(value); } __device__ int second(int value) { return value; } __global__ void kernel() {}",
        "__device__ int helper(int value); __device__ int helper(int value) { return value; } __global__ void kernel() {}",
        "__device__ int helper(int value) { return value; } __device__ float helper(float value) { return value; } __global__ void kernel() {}",
        "__device__ int helper(int value) { return value; } __global__ void helper() {}",
        "__device__ int helper(int value) { return value; } __global__ void first() {} __global__ void second() {}",
        "__device__ int helper(int value) { return value; }",
        "__global__ void kernel() {} __device__ int helper() { return 1; }",
    ] {
        assert_rejected(source);
    }
}

#[test]
fn cuda_device_functions_reject_unsupported_signatures_and_captures() {
    for source in [
        "__device__ void helper(int *value) { return; } __global__ void kernel() {}",
        "__device__ int *helper(int value) { return value; } __global__ void kernel() {}",
        "__device__ int helper(int *value) { return value[0]; } __global__ void kernel() {}",
        "__device__ int helper(const float *value) { return value[0]; } __global__ void kernel() {}",
        "__device__ double helper(double value) { return value; } __global__ void kernel() {}",
        "__device__ int helper(int value = 1) { return value; } __global__ void kernel() {}",
        "__device__ int helper(int value, ...) { return value; } __global__ void kernel() {}",
        "__device__ int helper() { __shared__ int value; return 1; } __global__ void kernel() {}",
        "__device__ int helper() { __syncthreads(); return 1; } __global__ void kernel() {}",
        "__device__ int helper() { return output[0]; } __global__ void kernel(int *output) {}",
    ] {
        assert_rejected(source);
    }
    for builtin in ["threadIdx.x", "blockIdx.x", "blockDim.x", "gridDim.x"] {
        assert_rejected(&format!(
            "__device__ unsigned helper() {{ return {builtin}; }} __global__ void kernel(unsigned *out) {{ out[0] = helper(); }}"
        ));
    }
}

#[test]
fn cuda_device_helpers_preserve_source_checks_in_unreachable_code() {
    for body in [
        "return value; missing(value);",
        "return value; { missing(value); }",
        "return value; __syncthreads();",
        "return value; __shared__ int storage;",
        "return value; const int fixed = 1; fixed = 2;",
    ] {
        assert_rejected(&format!(
            "__device__ int helper(int value) {{ {body} }} __global__ void kernel() {{}}"
        ));
    }
}

#[test]
fn cuda_device_functions_preserve_nesting_limits() {
    for body in [
        format!("return {}1{};", "(".repeat(128), ")".repeat(128)),
        format!("{}return 1;{}", "{".repeat(128), "}".repeat(128)),
        format!(
            "return {};",
            core::iter::repeat_n("1", 512)
                .collect::<Vec<_>>()
                .join(" + ")
        ),
    ] {
        assert_rejected(&format!(
            "__device__ int helper() {{ {body} }} __global__ void kernel() {{}}"
        ));
    }
}

#[test]
#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
fn cuda_device_function_signatures_and_calls_roundtrip_through_wgsl() {
    let module = validate(include_str!("in/cuda/device-functions.cu"));
    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    );
    let info = validator.validate(&module).unwrap();
    let wgsl =
        naga::back::wgsl::write_string(&module, &info, naga::back::wgsl::WriterFlags::empty())
            .unwrap();
    let roundtrip = naga::front::wgsl::parse_str(&wgsl).unwrap();
    validator.validate(&roundtrip).unwrap();
}
