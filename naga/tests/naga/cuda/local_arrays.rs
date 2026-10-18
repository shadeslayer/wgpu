use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

#[test]
fn cuda_local_arrays_use_fixed_scalar_types_and_common_layouts() {
    for (name, scalar) in [
        ("bool", naga::Scalar::BOOL),
        ("int", naga::Scalar::I32),
        ("unsigned", naga::Scalar::U32),
        ("float", naga::Scalar::F32),
    ] {
        let (module, _) = validate(&format!("__global__ void kernel() {{ {name} values[3]; }}"));
        let function = &module.entry_points[0].function;
        let local = function.local_variables.iter().next().unwrap().1;
        let naga::TypeInner::Array {
            base,
            size: naga::ArraySize::Constant(size),
            stride,
        } = module.types[local.ty].inner
        else {
            panic!("expected a fixed local array");
        };
        let mut layouter = naga::proc::Layouter::default();
        layouter.update(module.to_ctx()).unwrap();
        assert_eq!(module.types[base].inner, naga::TypeInner::Scalar(scalar));
        assert_eq!(size.get(), 3);
        assert_eq!(stride, layouter[base].to_stride());
        assert_eq!(layouter[local.ty].size, 3 * stride);
        assert_eq!(local.init, None);
        assert!(module.global_variables.is_empty());
    }
}

#[test]
fn cuda_bare_local_arrays_do_not_emit_initializers_or_stores() {
    let (module, _) = validate("__global__ void kernel() { int unused[2]; }");
    let function = &module.entry_points[0].function;
    assert!(!function
        .expressions
        .iter()
        .any(|(_, expression)| matches!(expression, naga::Expression::ZeroValue(_))));
    assert!(!naga::proc::any_statement(
        &function.body,
        &mut |statement| { matches!(statement, naga::Statement::Store { .. }) }
    ));
    assert!(function
        .local_variables
        .iter()
        .all(|(_, variable)| variable.init.is_none()));
}

#[test]
fn cuda_local_arrays_accept_both_empty_initialization_forms() {
    for scalar in ["bool", "int", "unsigned", "float"] {
        for initializer in ["{}", "= {}"] {
            let (module, _) = validate(&format!(
                "__global__ void kernel() {{ {scalar} values[2] {initializer}; }}"
            ));
            let function = &module.entry_points[0].function;
            let (local, variable) = function.local_variables.iter().next().unwrap();
            assert_eq!(variable.init, None);
            let [naga::Statement::Store { pointer, value }] = &function.body[..] else {
                panic!("empty initialization must store at the declaration");
            };
            assert!(
                matches!(function.expressions[*pointer], naga::Expression::LocalVariable(actual) if actual == local)
            );
            assert!(
                matches!(function.expressions[*value], naga::Expression::ZeroValue(ty) if ty == variable.ty)
            );
        }
    }
}

#[test]
fn cuda_local_arrays_preserve_alias_const_and_element_conversions() {
    validate(
        "using Sample = float; using Frozen = const int; using Flag = bool;
        __global__ void kernel(int *out) {
            Sample samples[2]; Frozen frozen[2]{}; Flag flags[2] = {};
            samples[0] = 3; flags[1] = samples[0];
            out[0] = samples[0]; out[1] = frozen[1]; out[2] = flags[1];
        }",
    );
    for declaration in [
        "const int values[2]{};",
        "int const values[2] = {};",
        "Frozen values[2]{};",
    ] {
        for operation in [
            "values[0] = 1;",
            "values[0]++;",
            "++values[1];",
            "values[0] += 1;",
        ] {
            let source = format!(
                "using Frozen = const int; __global__ void kernel() {{ {declaration} {operation} }}"
            );
            assert!(rejected(&source).message.contains("const"));
        }
    }
}

#[test]
fn cuda_const_local_arrays_require_an_initializer() {
    for declaration in ["const int values[2];", "Frozen values[2];"] {
        let source =
            format!("using Frozen = const int; __global__ void kernel() {{ {declaration} }}");
        let error = rejected(&source);
        assert_eq!(&source[error.span.to_range().unwrap()], ";");
    }
    let source = "__global__ void kernel() { const int first[2]{}, second[2]; }";
    assert_eq!(&source[rejected(source).span.to_range().unwrap()], ";");
}

#[test]
fn cuda_local_array_lists_cache_the_declared_type_and_preserve_source_order() {
    let (module, _) = validate(
        "using Item = int;
        __global__ void kernel(int *out) {
            Item Item = 3, values[2]{}, last = Item + 1;
            values[0] = last; out[0] = values[0];
        }",
    );
    let function = &module.entry_points[0].function;
    let variables: Vec<_> = function
        .local_variables
        .iter()
        .map(|(_, value)| value)
        .collect();
    assert_eq!(variables.len(), 3);
    assert_eq!(
        module.types[variables[0].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::I32)
    );
    assert!(matches!(
        module.types[variables[1].ty].inner,
        naga::TypeInner::Array { .. }
    ));
    assert_eq!(
        module.types[variables[2].ty].inner,
        naga::TypeInner::Scalar(naga::Scalar::I32)
    );
    validate("__global__ void kernel() { int value[2]; }");
    validate("__global__ void kernel() { int first = 1, second[2]; }");
}

#[test]
fn cuda_local_arrays_have_lexical_and_for_initializer_scopes() {
    let (module, _) = validate(
        "__global__ void kernel(int *out) {
            int values[2]{};
            { float values[3]{}; values[1] = 2.0f; out[0] = values[1]; }
            for (int scratch[2]{}, index = 0; index < 2; ++index) {
                scratch[index] = index + 1; values[index] = scratch[index];
            }
            out[1] = values[1];
        }",
    );
    assert_eq!(module.entry_points[0].function.local_variables.len(), 4);
    let source = "__global__ void kernel() { for (int hidden[2]; false;) {} hidden[0] = 1; }";
    assert_eq!(&source[rejected(source).span.to_range().unwrap()], "hidden");
}

#[test]
fn cuda_local_arrays_reset_empty_initializers_inside_loops() {
    let (module, _) = validate(
        "__global__ void kernel(int *out) {
            for (int index = 0; index < 3; ++index) {
                int values[2]{}; out[index] = values[1]; values[1] = 9;
            }
        }",
    );
    let function = &module.entry_points[0].function;
    assert!(function
        .local_variables
        .iter()
        .all(|(_, value)| value.init.is_none()));
    assert!(naga::proc::any_statement(
        &function.body,
        &mut |statement| {
            let naga::Statement::Loop { body, .. } = statement else {
                return false;
            };
            naga::proc::any_statement(body, &mut |statement| {
                matches!(*statement, naga::Statement::Store { value, .. }
                if matches!(function.expressions[value], naga::Expression::ZeroValue(_)))
            })
        }
    ));
}

#[test]
fn cuda_local_arrays_work_in_scalar_and_void_device_helpers() {
    let (module, _) = validate(
        "__device__ int helper(int value) {
            int scratch[2]; scratch[0] = value; scratch[1] = value + 1;
            return scratch[0] + scratch[1];
        }
        __host__ __device__ void consume(int value) { bool flags[2]{}; flags[0] = value; if (flags[0]) return; }
        __global__ void kernel(int *out) { consume(1); out[0] = helper(3); }",
    );
    assert_eq!(module.functions.len(), 2);
    assert!(module
        .functions
        .iter()
        .all(|(_, function)| function
            .local_variables
            .iter()
            .any(|(_, variable)| matches!(
                module.types[variable.ty].inner,
                naga::TypeInner::Array { .. }
            ))));
}

#[test]
fn cuda_local_array_elements_reuse_updates_and_conditional_references() {
    validate(
        "__global__ void kernel(int *out, int choose) {
            int local[2]{}; __shared__ int shared[2]; shared[0] = 4;
            local[0] = 3; out[0] = 5;
            ++(choose ? local[0] : out[0]);
            (choose ? shared[0] : local[1]) += 2;
            out[1] = (choose ? local[0] : shared[0])++;
            int index = 0; local[index++] += 2; out[2] = index;
        }",
    );
}

#[test]
fn cuda_local_array_index_calls_execute_once_after_assignment_rhs() {
    let (module, _) = validate(
        "__device__ int next(int value) { return value; }
        __global__ void kernel() { int values[3]; values[next(1)] = next(2); }",
    );
    let function = &module.entry_points[0].function;
    let arguments: Vec<_> = function
        .body
        .iter()
        .filter_map(|statement| match statement {
            naga::Statement::Call { arguments, .. } => Some(arguments[0]),
            _ => None,
        })
        .collect();
    assert_eq!(arguments.len(), 2);
    for (handle, expected) in arguments.into_iter().zip([2, 1]) {
        assert!(
            matches!(function.expressions[handle], naga::Expression::Literal(naga::Literal::I32(actual)) if actual == expected)
        );
    }
}

#[test]
fn cuda_local_arrays_keep_name_collisions_and_hiding_rules() {
    for source in [
        "__global__ void kernel() { int value[2]; int value; }",
        "__global__ void kernel(int value) { int value[2]; }",
        "__global__ void kernel() { int first[2], first[3]; }",
        "__device__ int helper() { return 1; } __global__ void kernel() { int helper[2]; helper(); }",
        "__global__ void kernel() { int class[2]; }",
    ] {
        rejected(source);
    }
    validate("__device__ int helper() { return 1; } __global__ void kernel() { int first = helper(), helper[2]; }");
}

#[test]
fn cuda_local_array_extents_share_integer_literal_grammar() {
    for extent in ["2", "2u", "02", "0x2", "0b10", "0'2U"] {
        let (module, _) = validate(&format!(
            "__global__ void kernel() {{ int values[{extent}]; }}"
        ));
        let variable = module.entry_points[0]
            .function
            .local_variables
            .iter()
            .next()
            .unwrap()
            .1;
        assert!(
            matches!(module.types[variable.ty].inner, naga::TypeInner::Array { size: naga::ArraySize::Constant(size), .. } if size.get() == 2)
        );
    }
    for extent in [
        "0",
        "0u",
        "-1",
        "+",
        "bool(size)",
        "size",
        "1 +",
        "1.0f",
        "08",
        "0x",
        "4294967296u",
        "1UL",
    ] {
        rejected(&format!(
            "/* 漢 */ __global__ void kernel(int size) {{ int values[{extent}]; }}"
        ));
    }
}

#[test]
fn cuda_local_array_layout_limits_do_not_allocate_per_element() {
    for (name, width) in [("int", 4), ("bool", 1)] {
        let count = naga::valid::MAX_TYPE_SIZE / width;
        let (module, _) = validate(&format!(
            "__global__ void kernel() {{ {name} values[{count}u]; }}"
        ));
        let variable = module.entry_points[0]
            .function
            .local_variables
            .iter()
            .next()
            .unwrap()
            .1;
        assert_eq!(
            module.types[variable.ty].inner.try_size(module.to_ctx()),
            Some(count * width)
        );
        let source = format!(
            "__global__ void kernel() {{ {name} values[{}u]; }}",
            count + 1
        );
        let error = rejected(&source);
        assert!(error.message.contains("size"));
        assert_eq!(
            &source[error.span.to_range().unwrap()],
            format!("{}u", count + 1)
        );
    }
}

#[test]
fn cuda_local_arrays_reject_unsupported_declarators_and_initializers() {
    for declaration in [
        "int values[];",
        "int values[]{};",
        "int values[2][2];",
        "int *values[2];",
        "double values[2];",
        "void values[2];",
        "int (values)[2];",
        "static int values[2];",
        "int values[2] = {1, 2, 3};",
        "int values[2]{1, 2, 3};",
        "int values[2] = {{{1}}};",
        "int values[2] = 1;",
        "int values[2]();",
        "int values[2] = {;",
        "int values[2] trailing;",
    ] {
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
    for operation in [
        "values = 1;",
        "int value = values;",
        "values++;",
        "int value = values[true];",
        "int value = values[0][0];",
        "consume(values);",
        "int *pointer = values;",
        "&values[0];",
    ] {
        rejected(&format!("__device__ void consume(int value) {{}} __global__ void kernel() {{ int values[2]{{}}; {operation} }}"));
    }
}

#[test]
fn cuda_local_arrays_validate_unreachable_declarations_and_preserve_resource_abi() {
    let (module, _) = validate(
        "__global__ void kernel(const int *input, int factor, int *out) {
            int values[2]{}; values[0] = input[0] * factor; out[0] = values[0];
            return; int unused[3]{};
        }",
    );
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
    rejected("__global__ void kernel() { return; int values[0]; }");
    rejected("__global__ void kernel() { return; const int values[2]; }");
}

#[test]
fn cuda_local_array_declarations_are_flat_and_bounded_by_type_size() {
    let declarations = (0..128)
        .map(|index| format!("values{index}[2]{{}}"))
        .collect::<Vec<_>>()
        .join(", ");
    let (module, _) = validate(&format!(
        "__global__ void kernel() {{ int {declarations}; }}"
    ));
    assert_eq!(module.entry_points[0].function.local_variables.len(), 128);
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_local_arrays_roundtrip_through_wgsl() {
    let (module, info) = validate(
        "__device__ float helper(float value) { float local[2]{}; local[1] = value; return local[1]; }
        __global__ void kernel(int *out) { bool flags[2]{}; const int frozen[2] = {}; flags[1] = true;
            for (int local[2]{}, index = 0; index < 2; ++index) {
                local[index] = helper(index); out[index] = local[index] + frozen[index] + flags[index];
            }
        }",
    );
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}
