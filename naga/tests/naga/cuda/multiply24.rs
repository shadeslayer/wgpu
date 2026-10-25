use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/multiply24.cu");
const INTRINSICS: [(&str, &str, naga::Scalar); 2] = [
    ("__mul24", "int", naga::Scalar::I32),
    ("__umul24", "unsigned", naga::Scalar::U32),
];

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

fn stored_value(function: &naga::Function) -> naga::Handle<naga::Expression> {
    function
        .body
        .iter()
        .rev()
        .find_map(|statement| match *statement {
            naga::Statement::Store { value, .. } => Some(value),
            _ => None,
        })
        .unwrap()
}

fn named_calls(module: &naga::Module, function: &naga::Function, name: &str) -> usize {
    function.body.iter().filter(|statement| matches!(
        **statement,
        naga::Statement::Call { function, .. } if module.functions[function].name.as_deref() == Some(name)
    )).count()
}

#[test]
fn cuda_multiply24_converts_scalar_pairs_to_declared_parameter_and_result_types() {
    let arguments = [
        "bool first = true",
        "int first = -3",
        "unsigned first = 5u",
        "float first = 3.75f",
    ];
    for (name, result_type, result_scalar) in INTRINSICS {
        for first in arguments {
            for second in arguments {
                let second = second.replace("first", "second");
                let source = format!("__global__ void kernel({result_type} *out) {{ {first}; {second}; out[0] = {name}(first, second); }}");
                let (module, info) = validate(&source);
                let function = &module.entry_points[0].function;
                assert_eq!(
                    info.get_entry_point(0)[stored_value(function)]
                        .ty
                        .inner_with(&module.types),
                    &naga::TypeInner::Scalar(result_scalar),
                    "{source}",
                );
            }
        }
    }
}

#[test]
fn cuda_multiply24_accepts_signed24_edges_and_both_operand_positions() {
    for value in [
        "0",
        "1",
        "-1",
        "-8388608",
        "8388607",
        "0x00800001",
        "0x00ffffff",
    ] {
        for arguments in [format!("{value}, -3"), format!("-3, {value}")] {
            validate(&format!(
                "__global__ void kernel(int *out) {{ out[0] = __mul24({arguments}); }}"
            ));
        }
    }
}

#[test]
fn cuda_multiply24_accepts_unsigned24_edges_and_both_operand_positions() {
    for value in [
        "0u",
        "1u",
        "0x007fffffu",
        "0x00800000u",
        "0x00fffffeu",
        "0x00ffffffu",
    ] {
        for arguments in [format!("{value}, 3u"), format!("3u, {value}")] {
            validate(&format!(
                "__global__ void kernel(unsigned *out) {{ out[0] = __umul24({arguments}); }}"
            ));
        }
    }
}

#[test]
fn cuda_multiply24_accepts_products_requiring_low32_bit_results() {
    validate(
        "__global__ void kernel(unsigned *out) {
            int signed_values[3] = {
                __mul24(8388607, 8388607), __mul24(-8388608, -8388608), __mul24(-8388608, 8388607)
            };
            unsigned unsigned_values[3] = {
                __umul24(0x00ffffffu, 0x00ffffffu), __umul24(65536u, 65536u), __umul24(0x00800000u, 0x00ffffffu)
            };
            for (int index = 0; index < 3; ++index) {
                out[index] = unsigned(signed_values[index]); out[index + 3] = unsigned_values[index];
            }
        }",
    );
}

#[test]
fn cuda_multiply24_accepts_all_high_bytes_including_high_only_words() {
    for word in [
        "0x01000000u",
        "0x80000000u",
        "0xff000000u",
        "0x01800000u",
        "0x7f800001u",
        "0xffffffffu",
    ] {
        validate(&format!(
            "__global__ void kernel(unsigned *out) {{
            out[0] = unsigned(__mul24(int({word}), 3)); out[1] = __umul24({word}, 3u);
            out[2] = unsigned(__mul24(3, int({word}))); out[3] = __umul24(3u, {word});
        }}"
        ));
    }
}

#[test]
fn cuda_multiply24_numerically_converts_safe_float_arguments_before_normalization() {
    for (name, result_type, _) in INTRINSICS {
        for arguments in [
            "3.75f, 2.75f",
            "true, 3.75f",
            "3.75f, false",
            "16777215.0f, 1.0f",
        ] {
            validate(&format!(
                "__global__ void kernel({result_type} *out) {{ out[0] = {name}({arguments}); }}"
            ));
        }
    }
    validate("__global__ void kernel(int *out) { out[0] = __mul24(-3.75f, 2.75f); }");
}

#[test]
fn cuda_multiply24_accepts_aliases_const_values_and_nested_helper_results() {
    validate(
        "using Word = unsigned; typedef const int Signed;
        __device__ int signed_product(Signed first, int second) { return __mul24(first, second); }
        __host__ __device__ Word unsigned_product(Word first, Word second) { return __umul24(first, second); }
        __device__ void consume(int value) { if (value) return; }
        __global__ void kernel(Word *out) {
            Signed first = -3; const Word second = 5u;
            out[0] = Word(signed_product(first, int(second)));
            out[1] = unsigned_product(Word(first), second);
            out[2] = Word(__mul24(signed_product(2, 3), 7));
            consume(__mul24(2, 3)); __umul24(2u, 3u);
        }",
    );
}

#[test]
fn cuda_multiply24_ranks_ordinary_overloads_and_reports_cross_parameter_ambiguity() {
    validate(
        "__device__ int __mul24(unsigned first, unsigned second) { return int(first + second); }
        __device__ int __umul24(int first, int second) { return first + second; }
        __global__ void kernel(unsigned *out) {
            out[0] = unsigned(__mul24(true, true)); out[1] = unsigned(__mul24(2u, 3u));
            out[2] = unsigned(__umul24(true, true)); out[3] = __umul24(2u, 3u);
        }",
    );
    for (name, ordinary_type, _) in [
        ("__mul24", "unsigned", naga::Scalar::I32),
        ("__umul24", "int", naga::Scalar::U32),
    ] {
        let source = format!(
            "__device__ int {name}({ordinary_type} first, {ordinary_type} second) {{ return 7; }}
            __global__ void kernel() {{ {name}(1, 2u); }}"
        );
        let error = rejected(&source);
        assert!(error.message.contains("ambiguous"));
        assert_eq!(&source[error.span.to_range().unwrap()], name);
    }
    validate(
        "__device__ int __mul24(bool first, bool second) { return first ? int(second) : 0; }
        __device__ int __mul24(float first, float second) { return int(first + second); }
        __global__ void kernel(int *out) {
            out[0] = __mul24(true, false); out[1] = __mul24(2.5f, 3.5f); out[2] = __mul24(2, 3);
        }",
    );
    let source =
        "__device__ int __mul24(bool first, bool second) { return int(first) + int(second); }
        __global__ void kernel() { __mul24(3, true); }";
    assert!(rejected(source).message.contains("ambiguous"));
}

#[test]
fn cuda_multiply24_binds_calls_at_source_point_and_resolves_forward_helpers() {
    let (module, _) = validate(
        "__device__ int before(bool first, bool second) { return __mul24(first, second); }
        __device__ int __mul24(bool first, bool second);
        __device__ int after(bool first, bool second) { return __mul24(first, second); }
        __global__ void kernel(int *out) { out[0] = before(true, true); out[1] = after(true, true); }
        __device__ int __mul24(bool first, bool second) { return first ? int(second) : 7; }",
    );
    for (name, expected_calls) in [("before", 0), ("after", 1)] {
        let (_, function) = module
            .functions
            .iter()
            .find(|(_, function)| function.name.as_deref() == Some(name))
            .unwrap();
        assert_eq!(named_calls(&module, function, "__mul24"), expected_calls);
    }
    validate(FIXTURE);
}

#[test]
fn cuda_multiply24_merges_compatible_intrinsic_prototypes_and_rejects_redefinitions() {
    for (name, parameter, _) in INTRINSICS {
        validate(&format!(
            "using Argument = const {parameter};
            __device__ {parameter} {name}(Argument, {parameter});
            __device__ {parameter} {name}({parameter} first, Argument second);
            __global__ void kernel({parameter} *out) {{ out[0] = {name}(2, 3); }}"
        ));
        let wrong_result = if parameter == "int" {
            "unsigned"
        } else {
            "int"
        };
        for result in [wrong_result, "void"] {
            let source = format!("__device__ {result} {name}({parameter}, {parameter}); __global__ void kernel() {{}}");
            let error = rejected(&source);
            assert!(error.message.contains("result"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
        let source = format!("__device__ {parameter} {name}(const {parameter} first, {parameter} second) {{ return first; }} __global__ void kernel() {{}}");
        let error = rejected(&source);
        assert!(error.message.contains("duplicate"));
        assert_eq!(&source[error.span.to_range().unwrap()], name);
    }
}

#[test]
fn cuda_multiply24_preserves_intrinsic_redeclaration_metadata() {
    for (name, parameter, _) in INTRINSICS {
        validate(&format!(
            "static __device__ __forceinline__ {parameter} {name}({parameter}, {parameter});
            __device__ {parameter} {name}({parameter}, {parameter});
            __device__ __forceinline__ {parameter} {name}({parameter}, {parameter});
            __global__ void kernel({parameter} *out) {{ out[0] = {name}(2, 3); }}"
        ));
        for declarations in [
            format!("__device__ {parameter} {name}({parameter}, {parameter}); __host__ __device__ {parameter} {name}({parameter}, {parameter});"),
            format!("__device__ {parameter} {name}({parameter}, {parameter}); static __device__ {parameter} {name}({parameter}, {parameter});"),
            format!("__device__ __forceinline__ {parameter} {name}({parameter}, {parameter}); __device__ __noinline__ {parameter} {name}({parameter}, {parameter});"),
        ] {
            rejected(&format!("{declarations} __global__ void kernel() {{}}"));
        }
    }
}

#[test]
fn cuda_multiply24_respects_local_hiding_and_global_name_collisions() {
    for (name, _, _) in INTRINSICS {
        for source in [
            format!("__global__ void kernel() {{ int {name} = 1; {name}(1, 2); }}"),
            format!("__device__ int helper(int {name}) {{ return {name}(1, 2); }} __global__ void kernel() {{}}"),
            format!("__global__ void {name}() {{}}"),
            format!("using {name} = int; __global__ void kernel() {{}}"),
        ] {
            assert_eq!(&source[rejected(&source).span.to_range().unwrap()], name);
        }
        validate(&format!(
            "__global__ void kernel(unsigned *out) {{
            {{ using {name} = int; out[0] = unsigned({name}(2.75f)); }}
            out[1] = unsigned({name}(2, 3));
        }}"
        ));
    }
}

#[test]
fn cuda_multiply24_evaluates_each_nested_argument_once() {
    for (name, _, _) in INTRINSICS {
        let (module, _) = validate(&format!(
            "__device__ int next(int value) {{ return value; }}
            __device__ int pair(int first, int second) {{ return first * 100 + second; }}
            __global__ void kernel(int *out) {{
                int counter = 2;
                out[0] = pair(int({name}(next(counter++), next(counter++))), counter++);
                out[1] = counter;
            }}"
        ));
        let function = &module.entry_points[0].function;
        assert_eq!(named_calls(&module, function, "next"), 2);
        assert_eq!(named_calls(&module, function, "pair"), 1);
    }
}

#[test]
fn cuda_multiply24_snapshots_scalar_array_and_conditional_left_arguments() {
    validate(
        "__global__ void kernel(unsigned *out) {
            int value = 3;
            out[0] = unsigned(__mul24(value, (value = 7)));
            out[1] = __umul24(unsigned(value), unsigned(value = 9));
            int values[2] = {2, 11}; int index = 0; bool choose = true;
            out[2] = unsigned(__mul24(values[index++], (values[0] = 7)));
            out[3] = unsigned(__mul24((choose ? values[0] : values[1]), (values[0] = 13)));
            choose = false;
            out[4] = __umul24(unsigned(choose ? values[0] : values[1]), unsigned(values[1] = 17));
            out[5] = unsigned(index + values[0] + values[1]);
        }",
    );
}

#[test]
fn cuda_multiply24_stays_inside_selected_control_flow_and_array_initializers() {
    validate(
        "__global__ void kernel(unsigned *out, int choose) {
            int counter = 2;
            int values[] = {__mul24(counter++, 2), __mul24(counter++, 3)};
            out[0] = unsigned(choose ? __mul24(counter++, 5) : __mul24(counter++, 7));
            out[1] = unsigned(false && __mul24(counter++, 2));
            out[2] = unsigned(true || __umul24(unsigned(counter++), 2u));
            for (int index = 0; index < 2; ++index) out[index + 3] = unsigned(values[index]);
            out[5] = unsigned(counter);
        }",
    );
}

#[test]
fn cuda_multiply24_is_not_a_source_constant_expression() {
    for (name, parameter, _) in INTRINSICS {
        for source in [
            format!("constexpr {parameter} Result = {name}(2, 3); __global__ void kernel() {{}}"),
            format!("__global__ void kernel() {{ constexpr {parameter} result = {name}(2, 3); }}"),
            format!("__global__ void kernel() {{ const int count = int({name}(2, 3)); int values[count]; }}"),
            format!("__global__ void kernel() {{ constexpr int count = ({name}(2, 3), 2); }}"),
            format!("__global__ void kernel() {{ switch (1) {{ case {name}(2, 3): break; }} }}"),
        ] {
            assert!(rejected(&source).message.contains("constant expression"));
        }
        validate(&format!(
            "constexpr int Count = true ? 2 : int({name}(2, 3));
            __global__ void kernel(int *out) {{
                constexpr int Other = false && {name}(2, 3) ? 3 : 2;
                int values[Count] = {{}}; int more[Other] = {{}};
                out[0] = values[0] + more[0];
            }}"
        ));
    }
}

#[test]
fn cuda_multiply24_reports_arity_and_malformed_arguments_with_original_spans() {
    for (name, _, _) in INTRINSICS {
        for arguments in ["", "1", "1, 2, 3"] {
            let source = format!("/* π */ __global__ void kernel() {{ {name}({arguments}); }}");
            let error = rejected(&source);
            assert!(error.message.contains("no matching"));
            assert_eq!(&source[error.span.to_range().unwrap()], name);
        }
        for body in [
            format!("{name}(1,);"),
            format!("{name}(1, 2;"),
            format!("{name}(missing, 2);"),
        ] {
            rejected(&format!("/* λ */ __global__ void kernel() {{ {body} }}"));
        }
    }
}

#[test]
fn cuda_multiply24_rejects_nonscalars_without_an_exact_ordinary_overload() {
    for (name, _, _) in INTRINSICS {
        for scalar in ["float", "int", "uint"] {
            for width in [2, 3, 4] {
                for arguments in ["value, 2", "2, value"] {
                    let source = format!("__global__ void kernel() {{ {scalar}{width} value{{}}; {name}({arguments}); }}");
                    assert!(rejected(&source).message.contains("no matching"));
                }
            }
        }
        for declaration in [
            "struct Value { int member; }; Value value{};",
            "int values[2]{};",
        ] {
            let argument = if declaration.starts_with("struct") {
                "value"
            } else {
                "values"
            };
            rejected(&format!(
                "__global__ void kernel() {{ {declaration} {name}({argument}, 2); }}"
            ));
        }
        rejected(&format!(
            "__global__ void kernel(int *out) {{ {name}(out, 2); }}"
        ));
        let source = format!(
            "__device__ void empty() {{}} __global__ void kernel() {{ {name}(1, empty()); }}"
        );
        assert_eq!(
            &source[rejected(&source).span.to_range().unwrap()],
            "empty()"
        );
        validate(&format!("struct Value {{ int member; }};
            __device__ int {name}(Value first, Value second) {{ return first.member + second.member; }}
            __global__ void kernel(int *out) {{ out[0] = {name}(Value{{2}}, Value{{3}}); }}"));
    }
}

#[test]
fn cuda_multiply24_results_are_prvalues_and_dead_calls_still_validate() {
    for (name, _, _) in INTRINSICS {
        validate(&format!(
            "__global__ void kernel() {{ return; {name}(1, 2); }}"
        ));
        for body in [
            format!("{name}(1, 2) = 3;"),
            format!("++{name}(1, 2);"),
            format!("{name}(1, 2)++;"),
            format!("return; {name}(missing, 2);"),
            format!("return; {name}(1);"),
        ] {
            rejected(&format!("/* π */ __global__ void kernel() {{ {body} }}"));
        }
    }
}

#[test]
fn cuda_multiply24_distinguishes_current_overload_calls_and_rejects_recursion() {
    for (name, parameter, _) in INTRINSICS {
        validate(&format!("__device__ int {name}(bool first, bool second) {{ return int({name}({parameter}(first), {parameter}(second))); }}
            __global__ void kernel(int *out) {{ out[0] = {name}(true, true); }}"));
        let source = format!("__device__ int {name}(bool first, bool second) {{ return {name}(first, second); }} __global__ void kernel() {{}}");
        assert!(rejected(&source).message.contains("recurs"));
        let source = format!(
            "__device__ int {name}(bool, bool);
            __device__ int relay(bool first, bool second) {{ return {name}(first, second); }}
            __device__ int {name}(bool first, bool second) {{ return relay(first, second); }}
            __global__ void kernel() {{}}"
        );
        assert!(rejected(&source).message.contains("recurs"));
    }
}

#[test]
fn cuda_multiply24_uses_shared_typed_integer_ir_and_existing_depth_limits() {
    let (module, info) = validate(
        "__global__ void kernel(unsigned *out, int first, int second, unsigned third, unsigned fourth) {
            out[0] = unsigned(__mul24(first, second)); out[1] = __umul24(third, fourth);
        }",
    );
    let function = &module.entry_points[0].function;
    let mut signed_extractions = 0;
    let mut unsigned_extractions = 0;
    let mut multiplications = 0;
    for (handle, expression) in function.expressions.iter() {
        match *expression {
            naga::Expression::Math {
                fun: naga::MathFunction::ExtractBits,
                arg,
                arg1: Some(offset),
                arg2: Some(count),
                arg3: None,
            } => {
                assert!(matches!(
                    function.expressions[offset],
                    naga::Expression::Literal(naga::Literal::U32(0))
                ));
                assert!(matches!(
                    function.expressions[count],
                    naga::Expression::Literal(naga::Literal::U32(24))
                ));
                let argument_type = info.get_entry_point(0)[arg].ty.inner_with(&module.types);
                assert_eq!(
                    argument_type,
                    info.get_entry_point(0)[handle].ty.inner_with(&module.types)
                );
                match *argument_type {
                    naga::TypeInner::Scalar(naga::Scalar::I32) => signed_extractions += 1,
                    naga::TypeInner::Scalar(naga::Scalar::U32) => unsigned_extractions += 1,
                    _ => panic!("expected declared 32-bit integer parameter type"),
                }
            }
            naga::Expression::Binary {
                op: naga::BinaryOperator::Multiply,
                left,
                right,
            } => {
                for value in [left, right, handle] {
                    assert_eq!(
                        info.get_entry_point(0)[value].ty.inner_with(&module.types),
                        &naga::TypeInner::Scalar(naga::Scalar::U32)
                    );
                }
                multiplications += 1;
            }
            _ => {}
        }
    }
    assert_eq!(
        (signed_extractions, unsigned_extractions, multiplications),
        (2, 2, 2)
    );
    assert!(module.functions.is_empty());
    assert!(module.types.iter().all(|(_, ty)| !matches!(
        ty.inner,
        naga::TypeInner::Scalar(naga::Scalar { width: 8, .. })
    )));
    for (name, parameter, _) in INTRINSICS {
        let mut expression = String::from("value");
        for _ in 0..40 {
            expression = format!("{name}({expression}, 1)");
        }
        let source = format!("__global__ void kernel({parameter} *out, {parameter} value) {{ out[0] = {expression}; }}");
        assert!(rejected(&source).message.contains("graph depth"));
        validate(&format!(
            "__global__ void kernel({parameter} *out) {{ out[0] = {name}({name}(2, 3), 4); }}"
        ));
    }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_multiply24_roundtrips_literals_and_runtime_operands_through_shared_wgsl() {
    let (module, info) = validate(FIXTURE);
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    let (module, info) = validate(
        "__global__ void kernel(unsigned *out, int value) {
            out[0] = unsigned(__mul24(8388607, 8388607));
            out[1] = unsigned(__mul24(-8388608, -8388608));
            out[2] = __umul24(0x00ffffffu, 0x00ffffffu);
            out[3] = unsigned(__mul24(value, 0x01800000));
            out[4] = __umul24(0xff800000u, 0xff000001u);
        }",
    );
    let (output, _) = super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    assert!(output.contains("extractBits"));
    assert!(!output.contains("__mul24"));
    assert!(!output.contains("__umul24"));
}

#[cfg(spv_out)]
#[cfg_attr(miri, ignore)]
#[test]
fn cuda_multiply24_fixture_emits_valid_spirv_binary() {
    let (module, info) = validate(FIXTURE);
    let options = naga::back::spv::Options {
        lang_version: (1, 1),
        ..Default::default()
    };
    let pipeline = naga::back::spv::PipelineOptions {
        entry_point: "multiply24".to_string(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
    crate::helpers::validate_spirv(&words);
}
