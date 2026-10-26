use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/multidimensional-arrays.cu");

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

fn array_shape(module: &naga::Module, name: &str) -> (Vec<(u32, u32)>, naga::Handle<naga::Type>) {
    let mut ty = module.entry_points[0]
        .function
        .local_variables
        .iter()
        .find(|(_, variable)| variable.name.as_deref() == Some(name))
        .unwrap()
        .1
        .ty;
    let mut dimensions = Vec::new();
    while let naga::TypeInner::Array {
        base,
        size: naga::ArraySize::Constant(count),
        stride,
    } = module.types[ty].inner
    {
        dimensions.push((count.get(), stride));
        ty = base;
    }
    (dimensions, ty)
}

#[test]
fn cuda_multidimensional_scalar_arrays_have_nested_common_layouts() {
    for (name, scalar) in [
        ("bool", naga::Scalar::BOOL),
        ("int", naga::Scalar::I32),
        ("unsigned", naga::Scalar::U32),
        ("float", naga::Scalar::F32),
    ] {
        let (module, _) = validate(&format!(
            "__global__ void kernel() {{ {name} matrix[2][3]; {name} volume[2][3][4]; }}"
        ));
        let mut layouter = naga::proc::Layouter::default();
        layouter.update(module.to_ctx()).unwrap();
        for (variable, counts) in [("matrix", &[2, 3][..]), ("volume", &[2, 3, 4][..])] {
            let (dimensions, leaf) = array_shape(&module, variable);
            assert_eq!(module.types[leaf].inner, naga::TypeInner::Scalar(scalar));
            assert_eq!(
                dimensions
                    .iter()
                    .map(|&(count, _)| count)
                    .collect::<Vec<_>>(),
                counts
            );
            let mut stride = layouter[leaf].to_stride();
            for &(count, actual_stride) in dimensions.iter().rev() {
                assert_eq!(actual_stride, stride);
                stride *= count;
            }
        }
        assert!(module.entry_points[0]
            .function
            .local_variables
            .iter()
            .all(|(_, variable)| variable.init.is_none()));
        assert!(!naga::proc::any_statement(
            &module.entry_points[0].function.body,
            &mut |statement| matches!(statement, naga::Statement::Store { .. })
        ));
    }
}

#[test]
fn cuda_multidimensional_vectors_preserve_packed_three_component_rows() {
    for base in ["int", "uint", "float"] {
        for width in [2, 3, 4] {
            let name = format!("{base}{width}");
            let (module, _) = validate(&format!(
                "__global__ void kernel() {{ {name} matrix[2][3]; {name} volume[2][2][3]; }}"
            ));
            let element_stride = width * 4;
            assert_eq!(
                array_shape(&module, "matrix").0,
                [(2, 3 * element_stride), (3, element_stride)]
            );
            assert_eq!(
                array_shape(&module, "volume").0,
                [
                    (2, 6 * element_stride),
                    (2, 3 * element_stride),
                    (3, element_stride)
                ]
            );
            let leaf = array_shape(&module, "matrix").1;
            if width == 3 {
                let naga::TypeInner::Struct { ref members, span } = module.types[leaf].inner else {
                    panic!("expected packed vector class");
                };
                assert_eq!(span, 12);
                assert_eq!(
                    members
                        .iter()
                        .map(|member| member.offset)
                        .collect::<Vec<_>>(),
                    [0, 4, 8]
                );
            } else {
                assert!(
                    matches!(module.types[leaf].inner, naga::TypeInner::Vector { size, .. } if size as u32 == width)
                );
            }
        }
    }
}

#[test]
fn cuda_multidimensional_shared_arrays_keep_workgroup_storage_and_leaf_types() {
    for name in [
        "int", "unsigned", "float", "int2", "int3", "int4", "uint2", "uint3", "uint4", "float2",
        "float3", "float4",
    ] {
        let projection = if matches!(name.chars().last(), Some('2' | '3' | '4')) {
            ".x"
        } else {
            ""
        };
        let (module, _) = validate(&format!(
            "__global__ void kernel(int *out) {{ __shared__ {name} tile[2][3]; __shared__ {name} volume[2][2][3]; tile[1][2]{projection} = 4; volume[1][0][2]{projection} = 7; __syncthreads(); out[0] = int(tile[1][2]{projection} + volume[1][0][2]{projection}); }}"
        ));
        let shared: Vec<_> = module
            .global_variables
            .iter()
            .filter(|(_, variable)| variable.space == naga::AddressSpace::WorkGroup)
            .collect();
        assert_eq!(shared.len(), 2);
        assert!(shared
            .iter()
            .all(|(_, variable)| variable.binding.is_none() && variable.init.is_none()));
    }
}

#[test]
fn cuda_multidimensional_declarations_cache_aliases_and_preserve_resource_ordinals() {
    let (module, _) = validate(
        "using Count = int; using Frozen = const unsigned; constexpr int Width = 3;
        __global__ void kernel(int *out, int seed, float3 *points) {
            Count Count = seed, matrix[2][Width]{}, next[][2]{{Count, 2}};
            Frozen frozen[2][2]{{1u}, {2u}};
            matrix[1][2] = next[0][0]; out[0] = matrix[1][2] + int(frozen[1][0]);
            points[0] = make_float3(1, 2, 3);
        }",
    );
    assert_eq!(
        array_shape(&module, "matrix")
            .0
            .iter()
            .map(|&(count, _)| count)
            .collect::<Vec<_>>(),
        [2, 3]
    );
    assert_eq!(
        array_shape(&module, "next")
            .0
            .iter()
            .map(|&(count, _)| count)
            .collect::<Vec<_>>(),
        [1, 2]
    );
    assert_eq!(
        module
            .global_variables
            .iter()
            .filter_map(|(_, variable)| variable.binding.map(|binding| binding.binding))
            .collect::<Vec<_>>(),
        [0, 1, 2]
    );
}

#[test]
fn cuda_multidimensional_initializers_accept_nested_flat_mixed_and_empty_forms() {
    for declaration in [
        "int matrix[2][3]{};",
        "int matrix[2][3] = {};",
        "int matrix[2][3]{{1}, {2, 3}};",
        "int matrix[2][3] = {1, 2, 3, 4};",
        "int matrix[2][2]{1, {2}, 3, 4,};",
        "int volume[2][2][2]{{{1}, {2}}, {{3}}};",
        "bool flags[2][2]{{true}, {false, true}};",
        "float samples[2][2]{{1.5f}, {2, 3}};",
        "int3 original{1, 2, 3}; int3 points[2][2]{{original, {}}, {original, 4}};",
        "int3 points[1][1]{{{{1}, {2}, {3}}}};",
    ] {
        validate(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
    for source in [
        "__global__ void kernel() { int values[2][2]; }",
        "__global__ void kernel() { int values[][2]{{1, 2}}; }",
        "__global__ void kernel() { int values[2][2]{{1}}; }",
        "__global__ void kernel() { __shared__ int values[2][2]; }",
        "__global__ void kernel() { __shared__ float3 values[2][2]; }",
    ] {
        validate(source);
    }
}

#[test]
fn cuda_multidimensional_inference_counts_completed_outer_rows() {
    for (declaration, count) in [
        ("int matrix[][2] = {1, 2, 3};", 2),
        ("int matrix[][2]{{1}, 2, 3, 4};", 3),
        ("int matrix[][2]{{}, {1}, {2, 3},};", 3),
        ("int matrix[][2][2]{1, 2, 3, 4, 5};", 2),
        ("int matrix[][2][2]{{{1}}, {{2}, {3}}};", 2),
        (
            "int3 original{1, 2, 3}; int3 matrix[][2]{{original}, 4, 5, 6, original};",
            2,
        ),
    ] {
        let (module, _) = validate(&format!("__global__ void kernel() {{ {declaration} }}"));
        assert_eq!(array_shape(&module, "matrix").0[0].0, count);
    }
}

#[test]
fn cuda_multidimensional_const_reaches_every_element_and_selected_reference() {
    for name in ["int", "float3"] {
        let projection = if name == "float3" { ".x" } else { "" };
        for operation in ["= 1", "+= 1", "++"] {
            let source = format!("using Frozen = const {name}; __global__ void kernel(int choose) {{ Frozen matrix[2][2]{{}}; matrix[1][0]{projection} {operation}; }}");
            assert!(rejected(&source).message.contains("const"));
        }
        let source = format!("__global__ void kernel(int choose) {{ const {name} frozen[2][2]{{}}; {name} mutable_values[2][2]{{}}; (choose ? frozen[0][1]{projection} : mutable_values[1][0]{projection}) = 1; }}");
        assert!(rejected(&source).message.contains("const"));
    }
    rejected("__global__ void kernel() { const int matrix[2][2]; }");
}

#[test]
fn cuda_multidimensional_arrays_reuse_helper_block_and_for_scopes() {
    validate("__device__ int sum(int seed) { int matrix[][2]{{seed}, {seed + 1}}; return matrix[0][0] + matrix[1][0]; } __host__ __device__ void consume(int seed) { bool flags[2][2]{}; flags[1][1] = seed; if (flags[1][1]) return; } __global__ void kernel(int *out) { int matrix[2][2]{}; { float matrix[1][3]{{1.0f}}; out[0] = int(matrix[0][0]); } for (int scratch[][2]{{1, 2}}, iteration = 0; iteration < 2; ++iteration) { int reset[2][2]{{iteration}, {}}; matrix[iteration][0] = scratch[0][1] + reset[1][1]; } out[1] = matrix[1][0] + sum(3); consume(1); }");
    let source = "__global__ void kernel() { for (int hidden[2][2]{}; false;) {} hidden[0][0]; }";
    assert_eq!(&source[rejected(source).span.to_range().unwrap()], "hidden");
    for declaration in ["int matrix[2][2], matrix[1][1];", "int class[2][2];"] {
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
}

#[test]
fn cuda_multidimensional_index_calls_execute_once_after_assignment_rhs() {
    let (module, _) = validate(
        "__device__ int mark(int value) { return value; } __global__ void kernel() { int matrix[2][3]; matrix[mark(1)][mark(2)] = mark(3); }",
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
    assert_eq!(arguments.len(), 3);
    for (argument, expected) in arguments.into_iter().zip([3, 1, 2]) {
        assert!(
            matches!(function.expressions[argument], naga::Expression::Literal(naga::Literal::I32(value)) if value == expected)
        );
    }
}

#[test]
fn cuda_multidimensional_leaf_references_reuse_updates_fields_and_lazy_selection() {
    validate("__global__ void kernel(int *out, int choose) { int local[2][2]{}; __shared__ int shared[2][2]; shared[0][1] = 4; int3 points[2][2]{}; int row = 0; int column = 0; local[row++][column++] += 3; ++(choose ? local[0][1] : shared[0][1]); out[0] = (choose ? local[1][0] : out[1])++; (choose ? points[0][1] : points[1][0]).z += 2; false && ++local[row++][0]; true || ++local[0][column++]; out[2] = row + column; }");
}

#[test]
fn cuda_nested_constant_addresses_check_every_completed_dimension() {
    for expression in [
        "(matrix[1][2], 3)",
        "+(matrix[0][1], 3)",
        "true ? 3 : (matrix[100][100], 4)",
        "(true ? matrix[1][2] : matrix[100][100], 3)",
    ] {
        validate(&format!("__global__ void kernel() {{ int matrix[2][3]{{}}; constexpr int Count = {expression}; int scratch[Count][2]{{}}; }}"));
    }
    for (row, column) in [
        ("-1", "0"),
        ("2", "0"),
        ("0", "-1"),
        ("0", "3"),
        ("0", "100"),
        ("0xffffffffu", "0"),
        ("0", "0xffffffffu"),
    ] {
        for expression in [
            format!("(matrix[{row}][{column}], 3)"),
            format!("false ? 3 : (matrix[{row}][{column}], 4)"),
        ] {
            rejected(&format!("__global__ void kernel() {{ int matrix[2][3]{{}}; constexpr int Count = {expression}; }}"));
        }
    }
    rejected("__global__ void kernel(int row) { int matrix[2][3]{}; constexpr int Count = (matrix[row][0], 3); }");
    validate("__global__ void kernel() { float3 matrix[2][3]{}; constexpr int Count = (matrix[1][2].z, 3); int scratch[Count][2]{}; }");
}

#[test]
fn cuda_unknown_outer_bounds_keep_zero_offset_and_known_inner_address_rules() {
    for expression in [
        "(matrix[0][0], 2)",
        "(matrix[0][1], 2)",
        "+(matrix[0][1], 2)",
        "unsigned((matrix[0][1], 2))",
        "true ? 2 : (matrix[1][0], 3)",
        "(true ? matrix[0][1] : matrix[100][0], 2)",
    ] {
        validate(&format!(
            "__global__ void kernel() {{ float matrix[][2]{{{{1.0f, {expression}}}}}; }}"
        ));
    }
    validate("__global__ void kernel() { float3 matrix[][2]{{{1.0f, (matrix[0][1].x, 2)}}}; }");
    for expression in [
        "(matrix[1][0], 2)",
        "(matrix[0][2], 2)",
        "(matrix[-1][0], 2)",
        "(matrix[0][-1], 2)",
        "false ? 2 : (matrix[1][0], 3)",
    ] {
        rejected(&format!(
            "__global__ void kernel() {{ float matrix[][2]{{{{1.0f, {expression}}}, {{3.0f}}}}; }}"
        ));
    }
    validate("__global__ void kernel() { float matrix[][2]{{1.0f}, {2.0f}}; constexpr int Count = (matrix[1][1], 2); int scratch[Count][2]{}; }");
}

#[test]
fn cuda_ordinary_nested_addresses_do_not_require_constant_eligibility() {
    for expression in [
        "(matrix[1][0], 2)",
        "(matrix[0][100], 2)",
        "(matrix[-1][0], 2)",
    ] {
        validate(&format!(
            "__global__ void kernel() {{ int matrix[][2]{{{{1, {expression}}}}}; }}"
        ));
    }
    validate("__global__ void kernel(int row, int column) { int matrix[][2]{{1, (matrix[row][column], 2)}}; }");
    validate(
        "__global__ void kernel() { float matrix[][2]{{1.0f, float((matrix[100][100], 2))}}; }",
    );
}

#[test]
fn cuda_nested_lists_preserve_narrowing_excess_and_vector_copy_rejections() {
    for declaration in [
        "int matrix[2][2]{{1.0f}};",
        "unsigned matrix[2][2]{{-1}};",
        "bool matrix[2][2]{{2}};",
        "float matrix[2][2]{{16777217}};",
        "float matrix[2][2]{{runtime}};",
        "int matrix[2][2]{{1, 2, 3}};",
        "int matrix[2][2]{1, 2, 3, 4, 5};",
        "int matrix[2][2]{{{{1}}}};",
        "int3 original{1, 2, 3}; uint3 matrix[2][2]{{original}};",
        "int3 matrix[2][2]{{{1, 2, 3, 4}}};",
    ] {
        rejected(&format!(
            "__global__ void kernel(int runtime) {{ {declaration} }}"
        ));
    }
    validate(
        "__global__ void kernel(int runtime) { float matrix[2][2]{{16777218, float(runtime)}}; }",
    );
}

#[test]
fn cuda_multidimensional_arrays_reject_incomplete_shapes_and_value_copies() {
    for declaration in [
        "int matrix[2][];",
        "int matrix[][]{{1}};",
        "int matrix[][2]{};",
        "int matrix[][2];",
        "int matrix[2][0];",
        "int matrix[2][-1];",
        "int matrix[2][1.0f];",
        "int matrix[2][runtime];",
        "constexpr int matrix[2][2]{};",
        "int *matrix[2][2];",
        "int (*matrix)[2];",
        "__shared__ int matrix[][2];",
        "__shared__ int matrix[2][];",
        "__shared__ bool matrix[2][2];",
        "__shared__ const int matrix[2][2];",
        "__shared__ int matrix[2][2]{};",
        "__shared__ int matrix[2][2] = {};",
        "extern __shared__ int matrix[][2];",
    ] {
        rejected(&format!(
            "__global__ void kernel(int runtime) {{ {declaration} }}"
        ));
    }
    for operation in [
        "matrix[0] = matrix[1];",
        "int row = matrix[0];",
        "consume(matrix[0]);",
        "matrix[0][0][0];",
        "matrix[0][true];",
    ] {
        rejected(&format!("__device__ void consume(int value) {{}} __global__ void kernel() {{ int matrix[2][2]{{}}; {operation} }}"));
    }
    validate("__device__ void consume(int value) {} __global__ void kernel() { int matrix[2][2]{}; int *row = matrix[0]; }");
    rejected("__device__ void consume(int matrix[2][2]) {} __global__ void kernel() {}");
    rejected("__global__ void kernel(int matrix[2][2]) {}");
    validate("__global__ void kernel() { int matrix[2][2]{}; matrix; matrix[0]; &matrix[0][0]; }");
}

#[test]
fn cuda_multidimensional_errors_keep_dead_source_and_utf8_spans() {
    for declaration in [
        "int matrix[2][;",
        "int matrix[2][2]{{1},, {2}};",
        "int matrix[2][2]{{1};",
        "int matrix[2][2]{{1}} extra;",
        "int matrix[2][2]{.x = 1};",
    ] {
        rejected(&format!(
            "/* café 漢 */ __global__ void kernel() {{ return; {declaration} }}"
        ));
    }
    let source = "/* café 漢 */ __global__ void kernel() { int matrix[2][missing]; }";
    assert_eq!(
        &source[rejected(source).span.to_range().unwrap()],
        "missing"
    );
}

#[test]
fn cuda_multidimensional_layout_and_lowering_have_bounded_resource_cost() {
    for declaration in [
        "int matrix[2][16777216];",
        "int matrix[2][16777216]{};",
        "int matrix[2][16777216]{{1}};",
    ] {
        let (module, _) = validate(&format!("__global__ void kernel() {{ {declaration} }}"));
        let function = &module.entry_points[0].function;
        assert!(function.expressions.len() < 1024);
        assert!(function.local_variables.len() < 64);
        assert!(module.types.len() < 64);
    }
    let inner_count = naga::valid::MAX_TYPE_SIZE / 8 + 1;
    let error = rejected(&format!(
        "__global__ void kernel() {{ int matrix[2][{inner_count}u]; }}"
    ));
    assert!(error.message.contains("size"));
    let excessive_rank = "[1]".repeat(65);
    let error = rejected(&format!(
        "__global__ void kernel() {{ int matrix{excessive_rank}; }}"
    ));
    assert!(
        error.message.contains("depth")
            || error.message.contains("rank")
            || error.message.contains("nesting")
    );
    let nested = "{".repeat(160) + "1" + &"}".repeat(160);
    rejected(&format!(
        "__global__ void kernel() {{ int matrix[2][2]{nested}; }}"
    ));
}

#[test]
fn cuda_multidimensional_arrays_roundtrip_through_the_common_wgsl_framework() {
    let (module, info) = validate(FIXTURE);
    #[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    #[cfg(not(all(feature = "wgsl-in", feature = "wgsl-out")))]
    let _ = (module, info);
    #[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
    for initializer in ["{}", "{{1}}"] {
        let (module, info) = validate(&format!(
            "__global__ void kernel(int *out) {{ int matrix[2][4096]{initializer}; out[0] = matrix[1][4095]; }}"
        ));
        let (output, _) = super::helpers::wgsl_roundtrip(
            &module,
            &info,
            naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
        );
        assert!(
            output.len() < 20_000,
            "nested zero initialization expanded to {} bytes",
            output.len()
        );
    }
}
