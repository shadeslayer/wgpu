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

fn helper<'a>(
    module: &'a naga::Module,
    name: &str,
) -> (naga::Handle<naga::Function>, &'a naga::Function) {
    module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some(name))
        .unwrap()
}

fn calls(
    function: &naga::Function,
) -> Vec<(
    naga::Handle<naga::Function>,
    Option<naga::Handle<naga::Expression>>,
)> {
    let mut result = Vec::new();
    naga::proc::any_statement(&function.body, &mut |statement| {
        if let naga::Statement::Call {
            function,
            result: value,
            ..
        } = *statement
        {
            result.push((function, value));
        }
        false
    });
    result
}

fn assert_rejected(source: &str) {
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert!(source.get(error.span.to_range().unwrap()).is_some());
    assert!(!error.message.is_empty());
}

#[test]
fn cuda_void_helpers_and_calls_have_no_result_values() {
    let module = validate(
        "__device__ void empty() {}
        __device__ void done(void) { return; }
        __global__ void kernel(int *out) { empty(); done(); out[0] = 1; }",
    );
    assert_eq!(module.functions.len(), 2);
    let (empty, _) = helper(&module, "empty");
    let (done, _) = helper(&module, "done");
    for (_, function) in module.functions.iter() {
        assert!(function.result.is_none());
        assert!(function.arguments.is_empty());
    }
    let kernel = &module.entry_points[0].function;
    assert_eq!(calls(kernel), [(empty, None), (done, None)]);
    assert!(!kernel
        .expressions
        .iter()
        .any(|(_, expression)| matches!(expression, naga::Expression::CallResult(_))));
}

#[test]
fn cuda_void_scalar_parameters_preserve_the_kernel_resource_abi() {
    let module = validate(
        "__device__ void consume(const int fixed, int value, unsigned count, float scale, bool stop) {
            value += fixed;
            if (stop) return;
            value *= scale;
            for (unsigned i = 0u; i < count; ++i) value += 1;
        }
        __global__ void kernel(int *out, const float *input, int count) {
            consume(1u, input[0], count, 2, 0.5f);
            out[0] = count;
        }",
    );
    let (_, function) = helper(&module, "consume");
    assert!(function.result.is_none());
    assert_eq!(function.arguments.len(), 5);
    for (argument, scalar) in function.arguments.iter().zip([
        naga::Scalar::I32,
        naga::Scalar::I32,
        naga::Scalar::U32,
        naga::Scalar::F32,
        naga::Scalar::BOOL,
    ]) {
        assert_eq!(argument.binding, None);
        assert_eq!(
            module.types[argument.ty].inner,
            naga::TypeInner::Scalar(scalar)
        );
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
}

#[test]
fn cuda_void_helpers_accept_fallthrough_and_partial_returns() {
    for body in [
        "",
        "return;",
        "if (value) return; value += 1;",
        "while (value > 0) { if (value == 1) return; --value; }",
        "if (value) { return; } else { int local = 2; local += 1; }",
    ] {
        validate(&format!(
            "__device__ void consume(int value) {{ {body} }} __global__ void kernel() {{ consume(2); }}"
        ));
    }
}

#[test]
fn cuda_void_returns_can_call_another_void_helper() {
    let module = validate(
        "__device__ void consume(int value) { if (value) return; }
        __device__ void forward(int value) { return (consume(value)); }
        __global__ void kernel() { return forward(2); }",
    );
    let (consume, _) = helper(&module, "consume");
    let (forward, function) = helper(&module, "forward");
    assert_eq!(calls(function), [(consume, None)]);
    assert_eq!(calls(&module.entry_points[0].function), [(forward, None)]);
    for function in [function, &module.entry_points[0].function] {
        assert!(matches!(
            function.body.last(),
            Some(naga::Statement::Return { value: None })
        ));
    }
}

#[test]
fn cuda_void_calls_accept_nested_scalar_arguments_and_void_callers() {
    let module = validate(
        "__device__ int increment(int value) { return value + 1; }
        __device__ void consume(int value) { value += 1; }
        __device__ void forward(int value) { consume(increment(increment(value))); }
        __global__ void kernel() { forward(increment(1)); increment(2); }",
    );
    let (consume, _) = helper(&module, "consume");
    for (_, function) in module.functions.iter() {
        assert!(!function.expressions.iter().any(|(_, expression)| matches!(expression, naga::Expression::CallResult(callee) if *callee == consume)));
    }
}

#[test]
fn cuda_void_calls_are_allowed_in_discarded_expression_contexts() {
    for body in [
        "consume(1);",
        "(consume(1));",
        "((consume(1)));",
        "if (true) consume(1); else consume(2);",
        "for (consume(0); false; consume(1)) {}",
        "for (int i = 0; i < 2; consume(i++)) { consume(i); }",
        "while (false) { consume(1); } do { consume(2); } while (false);",
    ] {
        validate(&format!(
            "__device__ void consume(int value) {{ value += 1; }} __global__ void kernel() {{ {body} }}"
        ));
    }
}

#[test]
fn cuda_void_calls_preserve_argument_updates_and_caller_continuation() {
    validate(
        "__device__ void consume(int first, int second) { first += second; }
        __global__ void kernel(int *out) {
            int value = 1;
            consume(value++, ++value);
            out[0] = value;
            for (consume(value++, value); value < 6; consume(++value, value)) { out[1] = value; }
            out[2] = value;
        }",
    );
}

#[test]
fn cuda_void_and_scalar_functions_reset_return_state_and_scopes() {
    let module = validate(
        "__device__ void consume(int value) { if (value) return; }
        __device__ int increment(int value) { return value + 1; }
        __device__ void forward(int value) { int local = increment(value); consume(local); }
        __global__ void kernel(int *out) { consume(1); out[0] = increment(2); forward(3); }",
    );
    assert!(helper(&module, "consume").1.result.is_none());
    assert!(helper(&module, "increment").1.result.is_some());
    assert!(helper(&module, "forward").1.result.is_none());
    assert_eq!(module.global_variables.len(), 1);
}

#[test]
fn cuda_void_dead_calls_and_scalar_subcalls_leave_valid_ir() {
    for source in [
        "__device__ int id(int value) { return value; } __device__ void consume(int value) {} __global__ void kernel() { return; consume(id(1)); }",
        "__device__ int id(int value) { return value; } __device__ void consume(int value) {} __device__ void forward() { return; { consume(id(id(1))); } } __global__ void kernel() { forward(); }",
        "__device__ void consume(int value) {} __device__ int scalar() { return 1; consume(2); } __global__ void kernel() { scalar(); }",
        "__device__ void consume(int value) {} __global__ void kernel() { return consume(1); { consume(2); } }",
    ] {
        validate(source);
    }
}

#[test]
fn cuda_void_value_errors_cover_the_call_span() {
    for body in [
        "int value = sink(1);",
        "int value = 0; value = sink(1);",
        "int value = 0; value += sink(1);",
        "id(sink(1));",
        "sink(sink(1));",
        "if (sink(1)) {}",
        "while (sink(1)) {}",
        "do {} while (sink(1));",
        "for (; sink(1);) {}",
        "!sink(1);",
        "sink(1) + 2;",
        "2 + sink(1);",
        "(sink(1)) + 2;",
        "int(sink(1));",
        "static_cast<int>(sink(1));",
        "bool value = false && sink(1);",
    ] {
        let source = format!(
            "__device__ int id(int value) {{ return value; }} __device__ void sink(int value) {{}} __global__ void kernel() {{ {body} }}"
        );
        let error = parse_str(&source, &Options::default()).unwrap_err();
        assert!(error.message.contains("void"), "{source}: {error}");
        let start = source.rfind("sink(1)").unwrap();
        assert_eq!(
            error.span.to_range().unwrap(),
            start..start + "sink(1)".len(),
            "{source}: {error}"
        );
    }
    let source = "__device__ void sink(int value) {} __device__ int scalar() { return sink(1); } __global__ void kernel() {}";
    let error = parse_str(source, &Options::default()).unwrap_err();
    assert_eq!(&source[error.span.to_range().unwrap()], "sink(1)");
}

#[test]
fn cuda_void_helpers_reject_returned_values_and_invalid_parameters() {
    for source in [
        "__device__ void consume(int value) { return value; } __global__ void kernel() {}",
        "__device__ void consume() { return true; } __global__ void kernel() {}",
        "__device__ int scalar() { return 1; } __device__ void consume() { return scalar(); } __global__ void kernel() {}",
        "__device__ void consume(void value) {} __global__ void kernel() {}",
        "__device__ void consume(int *value) {} __global__ void kernel() {}",
        "__device__ void consume(const int value) { ++value; } __global__ void kernel() {}",
        "__device__ void consume(int value) {} __global__ void kernel(void value) {}",
        "__device__ void consume(int value) {} __device__ int scalar(int value) { if (value) return 1; } __global__ void kernel() {}",
    ] {
        assert_rejected(source);
    }
}

#[test]
fn cuda_void_helpers_preserve_function_lookup_and_arity_checks() {
    for source in [
        "__device__ void consume(int value) {} __global__ void kernel() { consume(); }",
        "__device__ void consume(int value) {} __global__ void kernel() { consume(1, 2); }",
        "__device__ void consume(int value) {} __global__ void kernel() { int consume = 1; consume(2); }",
        "__device__ void consume(int value) {} __device__ void forward(int consume) { consume(1); } __global__ void kernel() {}",
        "__device__ void consume(int value) { consume(value); } __global__ void kernel() {}",
        "__device__ void forward(int value) { consume(value); } __device__ void consume(int value) {} __global__ void kernel() {}",
        "__device__ void consume(int value) {} __device__ void consume(float value) {} __global__ void kernel() {}",
    ] {
        assert_rejected(source);
    }
}

#[test]
fn cuda_void_helpers_still_reject_resources_builtins_and_barriers() {
    for body in [
        "__shared__ int value;",
        "__syncthreads();",
        "unsigned value = threadIdx.x;",
        "unsigned value = blockIdx.x;",
        "unsigned value = blockDim.x;",
        "unsigned value = gridDim.x;",
        "output[0] = 1;",
    ] {
        assert_rejected(&format!(
            "__device__ void consume() {{ {body} }} __global__ void kernel(int *output) {{ consume(); }}"
        ));
    }
}

#[test]
fn cuda_void_unreachable_source_remains_checked() {
    for body in [
        "return; consume(missing);",
        "return; consume(1, 2);",
        "return; int value = consume(1);",
        "return; __syncthreads();",
        "return; __shared__ int storage;",
        "return; return 1;",
    ] {
        assert_rejected(&format!(
            "__device__ void consume(int value) {{}} __device__ void forward() {{ {body} }} __global__ void kernel() {{}}"
        ));
    }
}

#[test]
#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
fn cuda_void_calls_and_returns_roundtrip_through_wgsl() {
    let module = validate(include_str!("in/cuda/void-functions.cu"));
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
