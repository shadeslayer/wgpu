#![cfg(feature = "cuda-in")]

use naga::front::cuda::{parse_str, Options};

fn validate(source: &str) -> (naga::Module, naga::valid::ModuleInfo) {
    let module = parse_str(source, &Options::default()).unwrap();
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    (module, info)
}

const TYPES: [(&str, &str, naga::Scalar); 4] = [
    ("bool", "true", naga::Scalar::BOOL),
    ("int", "3", naga::Scalar::I32),
    ("unsigned", "3u", naga::Scalar::U32),
    ("float", "3.0f", naga::Scalar::F32),
];

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

fn calls(block: &naga::Block) -> Vec<naga::Handle<naga::Function>> {
    statements(block)
        .into_iter()
        .filter_map(|statement| match *statement {
            naga::Statement::Call { function, .. } => Some(function),
            _ => None,
        })
        .collect()
}

fn signature(module: &naga::Module, function: naga::Handle<naga::Function>) -> Vec<naga::Scalar> {
    module.functions[function]
        .arguments
        .iter()
        .map(|argument| match module.types[argument.ty].inner {
            naga::TypeInner::Scalar(scalar) => scalar,
            ref other => panic!("expected scalar parameter, got {other:?}"),
        })
        .collect()
}

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}: {error}");
    assert!(source.get(range).is_some(), "{source}: {error}");
    assert!(!error.message.is_empty(), "{source}");
    error
}

fn rejected_at(source: &str, token: &str, occurrence: usize) {
    let error = rejected(source);
    let start = source.match_indices(token).nth(occurrence).unwrap().0;
    assert_eq!(error.span, naga::Span::from(start..start + token.len()));
}

#[test]
fn cuda_overloads_select_exact_scalar_parameter_types() {
    let (module, _) = validate(
        "__device__ bool choose(bool value) { return value; }
        __device__ int choose(int value) { return value; }
        __device__ unsigned choose(unsigned value) { return value; }
        __device__ float choose(float value) { return value; }
        __global__ void kernel(float *out) {
            out[0] = choose(true); out[1] = choose(3);
            out[2] = choose(3u); out[3] = choose(3.0f);
        }",
    );
    assert_eq!(module.functions.len(), 4);
    let targets = calls(&module.entry_points[0].function.body);
    assert_eq!(targets.len(), 4);
    for (target, (_, _, scalar)) in targets.into_iter().zip(TYPES) {
        assert_eq!(signature(&module, target), [scalar]);
        let result = module.functions[target].result.as_ref().unwrap();
        assert_eq!(
            module.types[result.ty].inner,
            naga::TypeInner::Scalar(scalar)
        );
    }
}

#[test]
fn cuda_overloads_rank_exact_promotion_and_conversion_for_all_scalar_pairs() {
    for (first_type_index, _) in TYPES.iter().enumerate() {
        for (second_type_index, _) in TYPES.iter().enumerate().skip(first_type_index + 1) {
            for (argument_type_index, (_, argument, _)) in TYPES.iter().enumerate() {
                let source = format!(
                    "__device__ int choose({} value) {{ return 1; }}
                    __device__ int choose({} value) {{ return 2; }}
                    __global__ void kernel(int *out) {{ out[0] = choose({argument}); }}",
                    TYPES[first_type_index].0, TYPES[second_type_index].0,
                );
                let conversion_rank = |target_type_index| {
                    if argument_type_index == target_type_index {
                        0
                    } else if argument_type_index == 0 && target_type_index == 1 {
                        1
                    } else {
                        2
                    }
                };
                let conversion_ranks = [
                    conversion_rank(first_type_index),
                    conversion_rank(second_type_index),
                ];
                if conversion_ranks[0] == conversion_ranks[1] {
                    rejected(&source);
                } else {
                    let (module, _) = validate(&source);
                    let selected_function = calls(&module.entry_points[0].function.body)[0];
                    let expected_type_index = if conversion_ranks[0] < conversion_ranks[1] {
                        first_type_index
                    } else {
                        second_type_index
                    };
                    assert_eq!(
                        signature(&module, selected_function),
                        [TYPES[expected_type_index].2],
                        "{source}"
                    );
                }
            }
        }
    }
}

#[test]
fn cuda_overloads_require_a_candidate_no_worse_on_every_argument() {
    let (module, _) = validate(
        "__device__ int choose(int first, int second) { return 1; }
        __device__ int choose(unsigned first, float second) { return 2; }
        __global__ void kernel(int *out) { out[0] = choose(true, 3); }",
    );
    let target = calls(&module.entry_points[0].function.body)[0];
    assert_eq!(signature(&module, target), [naga::Scalar::I32; 2]);
    rejected(
        "__device__ int choose(int first, float second) { return 1; }
        __device__ int choose(unsigned first, int second) { return 2; }
        __global__ void kernel(int *out) { out[0] = choose(true, 3); }",
    );
}

#[test]
fn cuda_overloads_selection_is_independent_of_definition_order() {
    for headers in [["int", "unsigned", "float"], ["float", "unsigned", "int"]] {
        let mut source = String::new();
        for header in headers {
            source.push_str(&format!(
                "__device__ int choose({header} value) {{ return 1; }}"
            ));
        }
        source.push_str(
            "__global__ void kernel(int *out) { out[0] = choose(true); out[1] = choose(4u); }",
        );
        let (module, _) = validate(&source);
        let targets = calls(&module.entry_points[0].function.body);
        assert_eq!(signature(&module, targets[0]), [naga::Scalar::I32]);
        assert_eq!(signature(&module, targets[1]), [naga::Scalar::U32]);
    }
    let parameters = [
        "int first, float second",
        "float first, int second",
        "int first, int second",
    ];
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let mut source = String::new();
        for index in order {
            source.push_str(&format!(
                "__device__ int choose({}) {{ return 1; }}",
                parameters[index]
            ));
        }
        source.push_str("__global__ void kernel(int *out) { out[0] = choose(3, 4); }");
        let (module, _) = validate(&source);
        let target = calls(&module.entry_points[0].function.body)[0];
        assert_eq!(signature(&module, target), [naga::Scalar::I32; 2]);
    }
}

#[test]
fn cuda_overloads_filter_by_arity_before_comparing_conversions() {
    let (module, _) = validate(
        "__device__ int choose() { return 1; }
        __device__ int choose(float value) { return 2; }
        __device__ int choose(int first, unsigned second) { return 3; }
        __global__ void kernel(int *out) {
            out[0] = choose(); out[1] = choose(3); out[2] = choose(true, 4);
        }",
    );
    let targets = calls(&module.entry_points[0].function.body);
    assert_eq!(signature(&module, targets[0]), []);
    assert_eq!(signature(&module, targets[1]), [naga::Scalar::F32]);
    assert_eq!(
        signature(&module, targets[2]),
        [naga::Scalar::I32, naga::Scalar::U32]
    );
    for arguments in ["1, 2, 3", ""] {
        rejected(&format!(
            "__device__ int choose(int value) {{ return value; }}
            __device__ int choose(int first, int second) {{ return first; }}
            __global__ void kernel() {{ choose({arguments}); }}"
        ));
    }
}

#[test]
fn cuda_overloads_do_not_use_the_destination_type_to_break_ties() {
    for destination in ["bool", "int", "unsigned", "float"] {
        rejected(&format!(
            "__device__ unsigned choose(unsigned value) {{ return value; }}
            __device__ float choose(float value) {{ return value; }}
            __global__ void kernel() {{ {destination} value = choose(3); }}"
        ));
    }
}

#[test]
fn cuda_overloads_resolve_aliases_and_ignore_top_level_const_parameters() {
    let (module, _) = validate(
        "using Whole = int; typedef const float Weight;
        __device__ int choose(const Whole value) { return value; }
        __device__ float choose(Weight value) { return value; }
        __global__ void kernel(float *out) { const int x = 3; out[0] = choose(x); out[1] = choose(2.0f); }",
    );
    let targets = calls(&module.entry_points[0].function.body);
    assert_eq!(signature(&module, targets[0]), [naga::Scalar::I32]);
    assert_eq!(signature(&module, targets[1]), [naga::Scalar::F32]);
    for parameter in ["int", "const int", "int const", "Whole"] {
        let source = format!(
            "using Whole = int;
            __device__ int choose(int value) {{ return value; }}
            __device__ int choose({parameter} other) {{ return other; }}
            __global__ void kernel() {{}}"
        );
        rejected_at(&source, "choose", 1);
    }
}

#[test]
fn cuda_overloads_reject_return_only_and_qualifier_only_redefinitions() {
    for header in [
        "__device__ float",
        "__device__ void",
        "__device__ const int",
        "inline __device__ int",
        "__host__ __device__ int",
        "static __device__ int",
    ] {
        let body = if header.ends_with("void") {
            "return;"
        } else {
            "return value;"
        };
        let source = format!(
            "__device__ int choose(int value) {{ return value; }}
            {header} choose(int value) {{ {body} }} __global__ void kernel() {{}}"
        );
        rejected_at(&source, "choose", 1);
    }
}

#[test]
fn cuda_overloads_preserve_each_definition_parameter_qualifiers_and_scope() {
    validate(
        "__device__ int choose(int value) { value += 2; using Local = int; Local result = value; return result; }
        __device__ float choose(const float value) { int Local = 3; return value + Local; }
        __global__ void kernel(float *out) { int value = 4; out[0] = choose(value); out[1] = choose(2.0f); out[2] = value; }",
    );
    rejected(
        "__device__ int choose(int value) { return value; }
        __device__ float choose(const float value) { ++value; return value; }
        __global__ void kernel() {}",
    );
    rejected(
        "__device__ int choose(int value) { using Local = int; return value; }
        __device__ float choose(float value) { Local other = 1; return value; }
        __global__ void kernel() {}",
    );
}

#[test]
fn cuda_overloads_mix_static_inline_and_dual_device_definitions() {
    let (module, _) = validate(
        "static inline __device__ int choose(int value) { return value + 1; }
        __host__ __device__ __forceinline__ float choose(float value) { return value + 2.0f; }
        __global__ void kernel(float *out) { out[0] = choose(3); out[1] = choose(3.0f); }",
    );
    assert_eq!(module.functions.len(), 2);
    let targets = calls(&module.entry_points[0].function.body);
    assert_ne!(targets[0], targets[1]);
    assert_eq!(signature(&module, targets[0]), [naga::Scalar::I32]);
    assert_eq!(signature(&module, targets[1]), [naga::Scalar::F32]);
    validate("static __device__ int helper(int value) { return value; } static __device__ float helper(float value) { return value; } __global__ void kernel() {}");
    validate("__device__ inline int helper(int value) { return value; } __device__ inline float helper(float value) { return value; } __global__ void kernel() {}");
    validate("__host__ __device__ int helper(int value) { return value; } __host__ __device__ float helper(float value) { return value; } __global__ void kernel() {}");
}

#[test]
fn cuda_overloads_support_void_results_and_discarded_scalar_calls() {
    validate("__device__ void consume(int value) {} __device__ void consume(float value) {} __global__ void kernel() {}");
    let (module, _) = validate(
        "__device__ void consume(int value) { if (value) return; }
        __device__ int consume(float value) { return value; }
        __device__ void forward() { return consume(3); }
        __global__ void kernel(int *out) { consume(1); consume(2.0f); forward(); out[0] = consume(3.0f); }",
    );
    let targets = calls(&module.entry_points[0].function.body);
    assert!(module.functions[targets[0]].result.is_none());
    assert!(module.functions[targets[1]].result.is_some());
    rejected(
        "__device__ void consume(int value) {}
        __device__ int consume(float value) { return value; }
        __global__ void kernel(int *out) { out[0] = consume(3); }",
    );
}

#[test]
fn cuda_overloads_snapshot_argument_values_before_later_side_effects() {
    let (module, _) = validate(
        "__device__ int pair(float first, int second) { return int(first) * 10 + second; }
        __device__ int pair(unsigned first, float second) { return int(first) + int(second); }
        __global__ void kernel(int *out) { int value = 3; out[0] = pair(value, value++); out[1] = value; }",
    );
    let kernel = &module.entry_points[0].function;
    let body = statements(&kernel.body);
    let (call_index, arguments) = body
        .iter()
        .enumerate()
        .find_map(|(index, statement)| {
            if let naga::Statement::Call {
                function,
                arguments,
                ..
            } = statement
            {
                assert_eq!(
                    signature(&module, *function),
                    [naga::Scalar::F32, naga::Scalar::I32]
                );
                Some((index, arguments))
            } else {
                None
            }
        })
        .unwrap();
    let naga::Expression::As { expr: loaded, .. } = kernel.expressions[arguments[0]] else {
        panic!("expected conversion of the first argument snapshot");
    };
    let naga::Expression::Load { pointer } = kernel.expressions[loaded] else {
        panic!("expected the first argument's local load");
    };
    let load_index = body.iter().position(|statement| {
        matches!(statement, naga::Statement::Emit(range) if range.clone().any(|handle| handle == loaded))
    }).unwrap();
    let mutation_index = body.iter().enumerate().find_map(|(index, statement)| {
        (index > load_index && matches!(statement, naga::Statement::Store { pointer: stored, .. } if *stored == pointer)).then_some(index)
    }).unwrap();
    assert!(load_index < mutation_index && mutation_index < call_index);
}

#[test]
fn cuda_overloads_keep_nested_calls_in_left_to_right_argument_order() {
    let (module, _) = validate(
        "__device__ int next(int value) { return value + 1; }
        __device__ float next(float value) { return value + 2.0f; }
        __device__ int pair(int first, int second) { return first * 10 + second; }
        __device__ float pair(float first, float second) { return first + second; }
        __global__ void kernel(int *out) { int value = 1; out[0] = pair(next(value++), next(value++)); }",
    );
    let targets = calls(&module.entry_points[0].function.body);
    assert_eq!(targets.len(), 3);
    assert_eq!(targets[0], targets[1]);
    assert_eq!(signature(&module, targets[0]), [naga::Scalar::I32]);
    assert_eq!(signature(&module, targets[2]), [naga::Scalar::I32; 2]);
}

#[test]
fn cuda_overloads_use_only_visible_definitions_and_reject_recursion() {
    let (module, _) = validate(
        "__device__ int choose(int value) { return value; }
        __device__ float choose(float value) { return choose(1); }
        __global__ void kernel(float *out) { out[0] = choose(2.0f); }",
    );
    let functions: Vec<_> = module.functions.iter().map(|(handle, _)| handle).collect();
    assert_eq!(calls(&module.functions[functions[1]].body), [functions[0]]);
    for source in [
        "__device__ int choose(int value) { return value; } __device__ float choose(float value) { return choose(value); } __global__ void kernel() {}",
        "__device__ int choose(int value) { return value; } __device__ float choose(float value) { return later(value); } __device__ float later(float value) { return value; } __global__ void kernel() {}",
    ] { rejected(source); }
    let (module, _) = validate(
        "__device__ int choose(int value) { return value; }
        __device__ int earlier() { return choose(3.0f); }
        __device__ float choose(float value) { return value; }
        __global__ void kernel(int *out) { out[0] = earlier(); }",
    );
    let functions: Vec<_> = module.functions.iter().map(|(handle, _)| handle).collect();
    assert_eq!(calls(&module.functions[functions[1]].body), [functions[0]]);
}

#[test]
fn cuda_overloads_local_names_mask_the_whole_function_set() {
    for body in [
        "int choose = 1; choose(3);",
        "int choose = choose(3);",
        "for (int choose = choose(3); choose < 4; ++choose) {}",
    ] {
        rejected(&format!(
            "__device__ int choose(int value) {{ return value; }}
            __device__ float choose(float value) {{ return value; }}
            __global__ void kernel() {{ {body} }}"
        ));
    }
    rejected(
        "__device__ int choose(int value) { return value; }
        __device__ float choose(float value) { return value; }
        __device__ int masked(int choose) { return choose(3); }
        __global__ void kernel() {}",
    );
    let (module, _) = validate(
        "__device__ int choose(int value) { return value; }
        __device__ float choose(float value) { return value; }
        __global__ void kernel(int *out) {
            { using choose = int; out[0] = choose(3.0f); }
            { int choose = 4; out[1] = choose; }
            out[2] = choose(3);
        }",
    );
    assert_eq!(calls(&module.entry_points[0].function.body).len(), 1);
}

#[test]
fn cuda_overloads_reject_global_type_and_kernel_name_collisions() {
    for source in [
        "using choose = int; __device__ int choose(int value) { return value; } __global__ void kernel() {}",
        "__device__ int choose(int value) { return value; } __device__ float choose(float value) { return value; } using choose = int; __global__ void kernel() {}",
        "__device__ int choose(int value) { return value; } __global__ void choose() {}",
        "__global__ void kernel() {} __global__ void kernel(int *out) {}",
    ] { rejected(source); }
}

#[test]
fn cuda_overloads_return_prvalues_and_convert_after_selection() {
    let (module, _) = validate(
        "__device__ int choose(int value) { return value; }
        __device__ float choose(float value) { return value + 0.5f; }
        __global__ void kernel(int *out) { out[0] = choose(3.0f); }",
    );
    let target = calls(&module.entry_points[0].function.body)[0];
    assert_eq!(signature(&module, target), [naga::Scalar::F32]);
    for expression in ["choose(3) = 4", "++choose(3)", "choose(3)++"] {
        rejected(&format!(
            "__device__ int choose(int value) {{ return value; }}
            __device__ float choose(float value) {{ return value; }}
            __global__ void kernel() {{ {expression}; }}"
        ));
    }
}

#[test]
fn cuda_overloads_keep_dead_calls_valid_and_check_dead_source_errors() {
    validate(
        "__device__ int choose(int value) { return value; }
        __device__ float choose(float value) { return value; }
        __device__ int earlier() { return choose(3); { choose(3.0f); } }
        __global__ void kernel() { return; { choose(3); choose(3.0f); earlier(); } }",
    );
    rejected(
        "__device__ int choose(unsigned value) { return value; }
        __device__ int choose(float value) { return value; }
        __global__ void kernel() { return; { choose(3); } }",
    );
}

#[test]
fn cuda_overloads_preserve_kernel_bindings_and_workgroup_configuration() {
    let source = "__device__ int choose(int value) { return value; }
        __device__ float choose(float value) { return value; }
        __global__ void kernel(float *out, int count, const float *input) {
            out[0] = choose(count) + choose(input[0]);
        }";
    let options = Options {
        workgroup_size: [4, 2, 1],
    };
    let module = parse_str(source, &options).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    assert_eq!(module.entry_points.len(), 1);
    assert_eq!(module.entry_points[0].workgroup_size, [4, 2, 1]);
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
    assert!(module.functions.iter().all(|(_, function)| function
        .arguments
        .iter()
        .all(|argument| argument.binding.is_none())));
}

#[test]
fn cuda_overloads_reject_malformed_and_unsupported_signatures_with_safe_spans() {
    for source in [
        "// λ\n__device__ int choose(int value) { return value; } __device__ float choose(float value,) { return value; } __global__ void kernel() {}",
        "__device__ int choose(int value) { return value; } __device__ float choose(float *value) { return value[0]; } __global__ void kernel() {}",
        "__device__ int choose(int value) { return value; } __device__ float choose(float value); __global__ void kernel() {}",
        "extern \"C\" __device__ int choose(int value) { return value; } __global__ void kernel() {}",
        "__device__ int choose(int value) { return value; } __device__ float choose(float value) { return value; } __global__ void kernel() { choose(1,); }",
    ] { rejected(source); }
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_overloads_round_trip_distinct_helper_signatures_through_wgsl() {
    let (module, info) = validate(
        "__device__ int choose(int value) { return value + 1; }
        __device__ float choose(float value) { return value + 2.0f; }
        __device__ void choose(bool value) { if (value) return; }
        __device__ int nested(int value) { return choose(value); }
        __global__ void kernel(float *out) { choose(true); out[0] = nested(3) + choose(2.0f); }",
    );
    let text = naga::back::wgsl::write_string(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    )
    .unwrap();
    let reparsed = naga::front::wgsl::parse_str(&text).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&reparsed)
    .unwrap();
}
