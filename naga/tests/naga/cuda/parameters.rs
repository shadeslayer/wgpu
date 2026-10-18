#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

const SCALE: &str = include_str!("../../in/cuda/scalar-parameters.cu");

use super::helpers::validate;

fn uniform_scalar(module: &naga::Module, global: &naga::GlobalVariable) -> naga::Scalar {
    assert_eq!(global.space, naga::AddressSpace::Uniform);
    let naga::TypeInner::Struct { members, span } = &module.types[global.ty].inner else {
        panic!("scalar parameter must use a uniform block");
    };
    assert_eq!(*span, 16);
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].offset, 0);
    assert_eq!(members[0].binding, None);
    let naga::TypeInner::Scalar(scalar) = module.types[members[0].ty].inner else {
        panic!("parameter block must contain a scalar");
    };
    scalar
}

#[test]
fn cuda_scalar_parameters_use_padded_uniform_blocks() {
    let (module, _) =
        validate("__global__ void parameters(float factor, int offset, unsigned int count) {}");
    assert_eq!(module.global_variables.len(), 3);
    let expected = [naga::Scalar::F32, naga::Scalar::I32, naga::Scalar::U32];
    for ((_, global), scalar) in module.global_variables.iter().zip(expected) {
        assert_eq!(uniform_scalar(&module, global), scalar);
    }
    assert_eq!(module.entry_points[0].function.local_variables.len(), 3);
}

#[test]
fn cuda_mixed_parameters_keep_source_binding_order() {
    let (module, _) = validate(
        "__global__ void mixed(float factor, const float *input, int offset, float *output, unsigned count) {\n\
            output[threadIdx.x] = input[threadIdx.x] * factor;\n\
        }",
    );
    let expected = ["factor", "input", "offset", "output", "count"];
    assert_eq!(module.global_variables.len(), expected.len());
    for ((_, global), (index, name)) in module
        .global_variables
        .iter()
        .zip(expected.into_iter().enumerate())
    {
        assert_eq!(global.name.as_deref(), Some(name));
        assert_eq!(
            global.binding,
            Some(naga::ResourceBinding {
                group: 0,
                binding: index as u32
            })
        );
        match index {
            0 => assert_eq!(uniform_scalar(&module, global), naga::Scalar::F32),
            1 => assert_eq!(
                global.space,
                naga::AddressSpace::Storage {
                    access: naga::StorageAccess::LOAD
                }
            ),
            2 => assert_eq!(uniform_scalar(&module, global), naga::Scalar::I32),
            3 => assert_eq!(
                global.space,
                naga::AddressSpace::Storage {
                    access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE
                }
            ),
            4 => assert_eq!(uniform_scalar(&module, global), naga::Scalar::U32),
            _ => unreachable!(),
        }
    }
}

#[test]
fn cuda_scalar_parameters_are_copied_before_local_mutation() {
    let (module, _) = validate(SCALE);
    let function = &module.entry_points[0].function;
    let (local, variable) = function.local_variables.iter().next().unwrap();
    assert_eq!(variable.name.as_deref(), Some("factor"));
    let stores: Vec<_> = function
        .body
        .iter()
        .filter_map(|statement| {
            let naga::Statement::Store { pointer, value } = *statement else {
                return None;
            };
            match function.expressions[pointer] {
                naga::Expression::LocalVariable(target) if target == local => Some(value),
                _ => None,
            }
        })
        .collect();
    assert_eq!(stores.len(), 2);
    let naga::Expression::Load { pointer } = function.expressions[stores[0]] else {
        panic!("parameter must be copied from its uniform field");
    };
    let naga::Expression::AccessIndex { base, index: 0 } = function.expressions[pointer] else {
        panic!("parameter must read its scalar field");
    };
    let naga::Expression::GlobalVariable(global) = function.expressions[base] else {
        panic!("parameter must read its uniform block");
    };
    assert_eq!(
        module.global_variables[global].space,
        naga::AddressSpace::Uniform
    );
    assert!(matches!(
        function.expressions[stores[1]],
        naga::Expression::Binary {
            op: naga::BinaryOperator::Multiply,
            ..
        }
    ));
}

#[test]
fn cuda_const_scalar_parameters_are_readable() {
    for source in [
        "__global__ void copy(const int value, int *out) { out[0] = value; }",
        "__global__ void copy(int const value, int *out) { out[0] = value; }",
        "__global__ void copy(const float value, float *out) { out[0] = value; }",
        "__global__ void copy(unsigned int const value, unsigned int *out) { out[0] = value; }",
    ] {
        validate(source);
    }
}

#[test]
fn cuda_scalar_parameter_errors_retain_source_spans() {
    for (source, invalid) in [
        (
            "__global__ void bad(const int value) { value = 1; }",
            "value",
        ),
        ("__global__ void bad(int value, float value) {}", "value"),
        ("__global__ void bad(int *value, int value) {}", "value"),
        ("__global__ void bad(int value, int *value) {}", "value"),
        ("__global__ void bad(int value) { int value = 1; }", "value"),
    ] {
        let error = parse_str(source, &Options::default()).unwrap_err();
        assert_eq!(source.get(error.span.to_range().unwrap()), Some(invalid));
    }
}

#[test]
fn cuda_scalar_parameters_reject_unsupported_forms() {
    for source in [
        "__global__ void bad(double value) {}",
        "__global__ void bad(int value = 1) {}",
        "__global__ void bad(const int const value) {}",
        "__global__ void bad(int value) { value = 1.0; }",
        "__global__ void bad(int value) { int x = value[0]; }",
    ] {
        assert!(parse_str(source, &Options::default()).is_err(), "{source}");
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_scalar_parameters_roundtrip_through_wgsl() {
    let (module, info) = validate(SCALE);
    super::helpers::wgsl_roundtrip(&module, &info, naga::back::wgsl::WriterFlags::empty());
}
