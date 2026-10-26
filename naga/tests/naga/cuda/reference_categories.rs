#![cfg(feature = "cuda-in")]

use super::helpers::validate;

#[test]
fn cuda_discarded_reference_evaluates_its_index_without_loading_storage() {
    let source = "__global__ void kernel(int *out) {
        int index = 0; out[index++]; out[2] = index;
    }";
    let (module, _) = validate(source);
    let function = &module.entry_points[0].function;
    for (_, expression) in function.expressions.iter() {
        if let naga::Expression::Load { mut pointer } = *expression {
            loop {
                match function.expressions[pointer] {
                    naga::Expression::Access { base, .. }
                    | naga::Expression::AccessIndex { base, .. } => pointer = base,
                    naga::Expression::GlobalVariable(_) => panic!("discarded storage load"),
                    _ => break,
                }
            }
        }
    }
}

#[test]
fn cuda_address_cancellation_preserves_constant_expression_side_effects() {
    validate("__global__ void kernel(int *out) { __shared__ int values[3]; constexpr int Count = (&*(&values[3]), 4); int storage[Count]{}; out[0] = storage[3]; }");
    let source = "__global__ void kernel(int *out) { __shared__ int value; int count = 0; constexpr int Count = (&(++count, *(&value)), 4); int storage[Count]{}; out[0] = storage[3]; }";
    let error = naga::front::cuda::parse_str(source, &naga::front::cuda::Options::default())
        .expect_err("address cancellation must preserve earlier side effects");
    assert!(error.message.contains("constant"), "{error}");
}

#[test]
fn cuda_pointer_projections_preserve_constant_expression_provenance() {
    validate("struct Record { int field; }; __global__ void kernel() { __shared__ Record record; constexpr int Count = ((*(&record)).field, 4); int storage[Count]{}; }");
    validate("__global__ void kernel() { __shared__ int values[2]; constexpr int Count = ((*(&values))[1], 4); int storage[Count]{}; }");
    for source in [
        "__global__ void kernel(int runtime) { __shared__ int values[2]; constexpr int Count = ((*(&values))[runtime], 4); int storage[Count]{}; }",
        "struct Record { int field; }; __global__ void kernel() { __shared__ Record record; int count = 0; constexpr int Count = ((++count, *(&record)).field, 4); int storage[Count]{}; }",
    ] {
        let error = naga::front::cuda::parse_str(source, &naga::front::cuda::Options::default())
            .expect_err("pointer projections must preserve constant expression provenance");
        assert!(error.message.contains("constant"), "{error}");
    }
}
