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
