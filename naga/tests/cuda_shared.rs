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

#[test]
fn cuda_shared_scalars_use_unbound_workgroup_variables() {
    let module = validate(
        "__global__ void shared_scalars(float *a, int *b, unsigned int *c) {
        __shared__ float f;
        __shared__ int i;
        __shared__ unsigned int u;
        f = 1.5f;
        i = -3;
        u = 7u;
        f++;
        i += 2;
        ++u;
        a[0] = f;
        b[0] = i;
        c[0] = u;
    }",
    );
    let shared: Vec<_> = module
        .global_variables
        .iter()
        .filter(|(_, variable)| variable.space == naga::AddressSpace::WorkGroup)
        .collect();
    assert_eq!(shared.len(), 3);
    for ((_, variable), scalar) in
        shared
            .into_iter()
            .zip([naga::Scalar::F32, naga::Scalar::I32, naga::Scalar::U32])
    {
        assert_eq!(variable.binding, None);
        assert_eq!(variable.init, None);
        assert_eq!(
            module.types[variable.ty].inner,
            naga::TypeInner::Scalar(scalar)
        );
    }
    assert!(module.entry_points[0].function.local_variables.is_empty());
}

#[test]
fn cuda_shared_array_layout_uses_the_common_type_size_limit() {
    let count = naga::valid::MAX_TYPE_SIZE / 4;
    let module = validate(&format!(
        "__global__ void large_shared() {{ __shared__ int values[{count}u]; }}"
    ));
    let (_, variable) = module.global_variables.iter().next().unwrap();
    assert_eq!(
        module.types[variable.ty].inner.try_size(module.to_ctx()),
        Some(count * 4)
    );
}

#[test]
fn cuda_shared_arrays_have_fixed_scalar_layouts() {
    let module = validate(
        "__global__ void shared_arrays(int *out) {
        __shared__ float floats[3];
        __shared__ int integers[4u];
        __shared__ unsigned values[5U];
        floats[0] = 2.5f;
        integers[1] = floats[0];
        values[2] = integers[1];
        values[2] += 3u;
        out[0] = values[2]++;
    }",
    );
    let shared: Vec<_> = module
        .global_variables
        .iter()
        .filter(|(_, variable)| variable.space == naga::AddressSpace::WorkGroup)
        .collect();
    assert_eq!(shared.len(), 3);
    let mut layouter = naga::proc::Layouter::default();
    layouter.update(module.to_ctx()).unwrap();
    for ((_, variable), (count, scalar)) in shared.into_iter().zip([
        (3, naga::Scalar::F32),
        (4, naga::Scalar::I32),
        (5, naga::Scalar::U32),
    ]) {
        let naga::TypeInner::Array {
            base,
            size: naga::ArraySize::Constant(size),
            stride,
        } = module.types[variable.ty].inner
        else {
            panic!("shared array must have a fixed size");
        };
        assert_eq!(size.get(), count);
        assert_eq!(stride, 4);
        assert_eq!(module.types[base].inner, naga::TypeInner::Scalar(scalar));
        assert_eq!(layouter[variable.ty].size, count * 4);
        assert_eq!(variable.binding, None);
        assert_eq!(variable.init, None);
    }
}

#[test]
fn cuda_shared_declarations_do_not_emit_initialization_stores() {
    let module = validate("__global__ void shared_declarations() { __shared__ int scalar; __shared__ float array[1]; }");
    assert_eq!(module.global_variables.len(), 2);
    assert!(module.entry_points[0].function.body.is_empty());
    for (_, variable) in module.global_variables.iter() {
        assert_eq!(variable.init, None);
    }
}

#[test]
fn cuda_shared_storage_preserves_parameter_bindings() {
    let module = validate(
        "__global__ void shared_bindings(float factor, const float *input, float *output) {
        __shared__ float values[64];
        __shared__ int scalar;
        unsigned int i = threadIdx.x;
        values[i] = input[i] * factor;
        output[i] = values[i];
    }",
    );
    for (ordinal, (_, variable)) in module.global_variables.iter().enumerate() {
        assert_eq!(
            variable.binding,
            if ordinal < 3 {
                Some(naga::ResourceBinding {
                    group: 0,
                    binding: ordinal as u32,
                })
            } else {
                None
            }
        );
    }
}

#[test]
fn cuda_shared_shadowing_uses_distinct_workgroup_objects() {
    let module = validate(
        "__global__ void shared_shadowing(int *out) {
        __shared__ int value;
        value = 1;
        { __shared__ int value; value = 2; }
        out[0] = value;
        { __shared__ int out[2]; out[0] = 3; }
        out[1] = value;
    }",
    );
    let shared: Vec<_> = module
        .global_variables
        .iter()
        .filter(|(_, variable)| variable.space == naga::AddressSpace::WorkGroup)
        .map(|(handle, _)| handle)
        .collect();
    assert_eq!(shared.len(), 3);
    let function = &module.entry_points[0].function;
    let last_value = function
        .body
        .iter()
        .filter_map(|statement| match *statement {
            naga::Statement::Store { value, .. } => Some(value),
            _ => None,
        })
        .next_back()
        .unwrap();
    let naga::Expression::Load { pointer } = function.expressions[last_value] else {
        panic!("expected shared scalar load")
    };
    assert!(
        matches!(function.expressions[pointer], naga::Expression::GlobalVariable(global) if global == shared[0])
    );
}

#[test]
fn cuda_shared_names_follow_block_and_loop_scope() {
    validate(
        "__global__ void shared_scopes(int *out) {
        __shared__ int values[2];
        values[0] = 1;
        { int values = 2; out[0] = values; }
        out[1] = values[0];
        for (__shared__ int step; false;) { step = 1; }
    }",
    );
    for source in [
        "__global__ void bad() { { __shared__ int value; } value = 1; }",
        "__global__ void bad() { for (__shared__ int value; false;) {} value = 1; }",
        "__global__ void bad(int *value) { __shared__ int value; }",
        "__global__ void bad() { int value = 1; __shared__ int value[2]; }",
        "__global__ void bad() { __shared__ int value; __shared__ float value; }",
    ] {
        assert!(parse_str(source, &Options::default()).is_err(), "{source}");
    }
}

#[test]
fn cuda_shared_extent_errors_retain_source_spans() {
    for extent in [
        "0",
        "0u",
        "4294967295u",
        "536870912u",
        "4294967296u",
        "1.0f",
        "size",
        "08",
    ] {
        let source = format!(
            "__global__ void bad(unsigned int size) {{ __shared__ int values[{extent}]; }}"
        );
        let error = parse_str(&source, &Options::default()).unwrap_err();
        assert_eq!(&source[error.span.to_range().unwrap()], extent, "{error}");
    }
}

#[test]
fn cuda_shared_storage_rejects_unsupported_declarations_and_uses() {
    for declaration in [
        "__shared__ bool value;",
        "__shared__ double value;",
        "__shared__ int *value;",
        "__shared__ const int value;",
        "const __shared__ int value;",
        "__shared__ int value = 1;",
        "__shared__ int values[2] = {1, 2};",
        "__shared__ int values[2][2];",
        "__shared__ int values[];",
        "extern __shared__ int values[];",
        "__shared__ int values[-1];",
        "__shared__ int values[+1];",
        "__shared__ int values[1 + 1];",
        "__shared__ int values[0x];",
        "__shared__ int values[1UL];",
        "__shared__ int first, second;",
        "__shared__ __shared__ int value;",
        "__shared__ int value; value[0] = 1;",
        "__shared__ int values[2]; values = 1;",
        "__shared__ int values[2]; values[0][0] = 1;",
        "__shared__ int values[2]; values[true] = 1;",
    ] {
        let source = format!("__global__ void bad() {{ {declaration} }}");
        assert!(parse_str(&source, &Options::default()).is_err(), "{source}");
    }
    for source in [
        "__shared__ int value; __global__ void bad() {}",
        "__global__ void bad(__shared__ int value) {}",
    ] {
        assert!(parse_str(source, &Options::default()).is_err(), "{source}");
    }
}

#[test]
fn cuda_shared_specifier_cannot_be_an_identifier() {
    for source in [
        "__global__ void __shared__() {}",
        "__global__ void bad(int __shared__) {}",
        "__global__ void bad() { int __shared__ = 1; }",
    ] {
        assert!(parse_str(source, &Options::default()).is_err(), "{source}");
    }
}

#[test]
fn cuda_shared_declarations_preserve_depth_and_layout_bounds() {
    for source in [
        format!(
            "__global__ void bad() {{ {}__shared__ int values[1];{} }}",
            "{".repeat(128),
            "}".repeat(128)
        ),
        format!(
            "__global__ void bad() {{ __shared__ int values[1]; values[{}0{}] = 1; }}",
            "(".repeat(128),
            ")".repeat(128)
        ),
        format!(
            "__global__ void bad() {{ __shared__ int values[{}]; }}",
            "9".repeat(4096)
        ),
        String::from("__global__ void bad() { return; __shared__ int values[536870912u]; }"),
    ] {
        assert!(parse_str(&source, &Options::default()).is_err());
    }
}
