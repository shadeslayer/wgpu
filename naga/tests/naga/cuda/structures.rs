use naga::front::cuda::{parse_str, Options};

use super::helpers::validate;

const FIXTURE: &str = include_str!("../../in/cuda/structures.cu");

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty() || range.start == source.len(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

fn members(module: &naga::Module, ty: naga::Handle<naga::Type>) -> (&[naga::StructMember], u32) {
    let naga::TypeInner::Struct { ref members, span } = module.types[ty].inner else {
        panic!("expected a record");
    };
    (members, span)
}

fn storage_record(module: &naga::Module, binding: u32) -> (naga::Handle<naga::Type>, u32) {
    let (_, global) = module
        .global_variables
        .iter()
        .find(|(_, global)| global.binding == Some(naga::ResourceBinding { group: 0, binding }))
        .unwrap();
    let naga::TypeInner::Array {
        base,
        size: naga::ArraySize::Dynamic,
        stride,
    } = module.types[global.ty].inner
    else {
        panic!("expected a record storage array");
    };
    (base, stride)
}

#[test]
fn cuda_structures_accept_global_block_definitions_and_completed_aliases() {
    validate(
        "struct Record { int value; }; using Alias = Record; typedef Record Other;
        __global__ void kernel(int *out) {
            Alias first{3}; Other second = first;
            { struct Local { Record nested; }; Local local{second}; out[0] = local.nested.value; }
            out[1] = first.value;
        }",
    );
    validate("__global__ void kernel() {} struct Later { int value; };");
}

#[test]
fn cuda_structures_preserve_nominal_types_and_canonical_aliases() {
    let (module, _) = validate("struct First { int value; }; struct Second { int value; }; using Alias = First;
        __device__ int choose(First value) { return 1; }
        __device__ int choose(Second value) { return 2; }
        __global__ void kernel(int *out) { First first{3}; Alias copy = first; Second second{4}; out[0] = choose(copy); out[1] = choose(second); }");
    let arguments: Vec<_> = module
        .functions
        .iter()
        .filter(|(_, function)| function.name.as_deref() == Some("choose"))
        .map(|(_, function)| function.arguments[0].ty)
        .collect();
    assert_eq!(arguments.len(), 2);
    assert_ne!(arguments[0], arguments[1]);
    for body in [
        "First first{}; Second second = first;",
        "First first{}; Second second{}; first = second;",
    ] {
        rejected(&format!("struct First {{ int value; }}; struct Second {{ int value; }}; __global__ void kernel() {{ {body} }}"));
    }
}

#[test]
fn cuda_structures_local_shadows_keep_distinct_identity_and_restore_scope() {
    let (module, _) = validate("struct Record { int value; };
        __global__ void kernel(int *out) {
            Record outer{1}; { struct Record { int value; }; Record inner{2}; out[0] = inner.value; }
            Record restored = outer; out[1] = restored.value;
        }");
    let records: Vec<_> = module.entry_points[0]
        .function
        .local_variables
        .iter()
        .filter(|(_, local)| {
            local
                .name
                .as_deref()
                .is_some_and(|name| ["outer", "inner", "restored"].contains(&name))
        })
        .map(|(_, local)| local.ty)
        .collect();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0], records[2]);
    assert_ne!(records[0], records[1]);
    rejected("struct Record { int value; }; __global__ void kernel() { Record outer{}; { struct Record { int value; }; Record inner = outer; } }");
}

#[test]
fn cuda_structures_generated_name_lookalikes_cannot_merge_declarations() {
    for name in ["Record_1", "Record_2", "Record_inner_1", "Record_1_2"] {
        let (module, _) = validate(&format!(
            "struct Record {{ int value; }};
            __global__ void kernel(int *out) {{
                Record outer{{1}};
                {{ struct Record {{ int value; }}; Record inner{{2}}; out[0] = inner.value; }}
                struct {name} {{ int value; }}; {name} lookalike{{3}}; out[1] = lookalike.value;
            }}"
        ));
        let handles: Vec<_> = module.entry_points[0]
            .function
            .local_variables
            .iter()
            .filter(|(_, local)| {
                local
                    .name
                    .as_deref()
                    .is_some_and(|name| ["outer", "inner", "lookalike"].contains(&name))
            })
            .map(|(_, local)| local.ty)
            .collect();
        assert_eq!(handles.len(), 3);
        assert_ne!(handles[0], handles[1]);
        assert_ne!(handles[0], handles[2]);
        assert_ne!(handles[1], handles[2]);
    }
    validate("__global__ void kernel(int *out) { { struct float3 { int value; }; float3 local{7}; out[0] = local.value; } float3 builtin = make_float3(1, 2, 3); out[1] = int(builtin.z); }");
}

#[test]
fn cuda_structures_resolve_arbitrary_member_names_in_member_order() {
    let (module, _) = validate("struct Record { int w, x, xy, min, sinf; }; __global__ void kernel(Record *out) { Record value{1, 2, 3, 4, 5}; value.w += value.x; value.xy += value.min; value.sinf++; out[0] = value; }");
    let (record, _) = storage_record(&module, 0);
    assert_eq!(
        members(&module, record)
            .0
            .iter()
            .map(|member| member.name.as_deref().unwrap())
            .collect::<Vec<_>>(),
        ["w", "x", "xy", "min", "sinf"]
    );
    validate("struct Record { int Record; }; __global__ void kernel(int *out) { Record value{3}; out[0] = value.Record; }");
    rejected("struct Record { int value; }; __global__ void kernel() { Record record{}; record.missing; }");
    rejected("__global__ void kernel() { float2 value{}; value.xy; }");
}

#[test]
fn cuda_structures_member_declarators_use_their_class_lookup_scope() {
    validate("constexpr int Count = 4; struct Record { int values[Count], extra; }; __global__ void kernel(int *out) { Record value{}; value.extra = 7; out[0] = value.values[3] + value.extra; }");
    validate("constexpr int Count = 4; __global__ void kernel(int *out) { struct Record { int Count; }; Record value{7}; int scratch[Count]{}; out[0] = value.Count + scratch[3]; }");
    for source in [
        "constexpr int Count = 4; struct Record { int Count; float values[Count]; }; __global__ void kernel() {}",
        "using Value = int; struct Record { int Value; Value other; }; __global__ void kernel() {}",
        "struct Record { int value; }; __global__ void kernel() { struct Record { Record child; }; }",
    ] { rejected(source); }
}

#[test]
fn cuda_structures_scalar_fields_have_natural_offsets_and_tail_size() {
    let (module, _) = validate("struct Record { int first; unsigned second; float third; }; __global__ void kernel(Record *out) { out[0] = Record{1, 2u, 3.0f}; }");
    let (record, stride) = storage_record(&module, 0);
    let (fields, span) = members(&module, record);
    assert_eq!(
        fields
            .iter()
            .map(|member| member.offset)
            .collect::<Vec<_>>(),
        [0, 4, 8]
    );
    assert_eq!(span, 12);
    assert_eq!(stride, 12);
}

#[test]
fn cuda_structures_keep_cuda_vector_alignment_and_packed_three_component_layout() {
    for scalar in ["float", "int", "uint"] {
        for (width, offset, tail, size, alignment) in
            [(2, 8, 16, 24, 8), (3, 4, 16, 20, 4), (4, 16, 32, 48, 16)]
        {
            let (module, _) = validate(&format!("struct Record {{ float lead; {scalar}{width} value; float tail; }}; __global__ void kernel(Record *out) {{ out[0] = Record{{}}; }}"));
            let (record, stride) = storage_record(&module, 0);
            let (fields, span) = members(&module, record);
            assert_eq!(
                fields
                    .iter()
                    .map(|member| member.offset)
                    .collect::<Vec<_>>(),
                [0, offset, tail]
            );
            assert_eq!(span, size);
            assert_eq!(stride, size);
            let mut layouter = naga::proc::Layouter::default();
            layouter.update(module.to_ctx()).unwrap();
            assert_eq!(
                layouter[record].alignment,
                naga::proc::Alignment::new(alignment).unwrap()
            );
        }
    }
}

#[test]
fn cuda_structures_nested_records_and_multidimensional_arrays_have_exact_layouts() {
    let (module, _) = validate(FIXTURE);
    let (packet, stride) = storage_record(&module, 0);
    let (fields, span) = members(&module, packet);
    assert_eq!(
        fields
            .iter()
            .map(|member| member.offset)
            .collect::<Vec<_>>(),
        [0, 8, 16, 48, 64]
    );
    assert_eq!((span, stride), (80, 80));
    let naga::TypeInner::Array {
        base: sample,
        stride: sample_stride,
        ..
    } = module.types[fields[2].ty].inner
    else {
        panic!("sample array")
    };
    assert_eq!(sample_stride, 16);
    assert_eq!(
        members(&module, sample)
            .0
            .iter()
            .map(|member| member.offset)
            .collect::<Vec<_>>(),
        [0, 12]
    );
    let naga::TypeInner::Array {
        base: row,
        stride: row_stride,
        ..
    } = module.types[fields[3].ty].inner
    else {
        panic!("matrix")
    };
    assert_eq!(row_stride, 8);
    assert!(matches!(
        module.types[row].inner,
        naga::TypeInner::Array { stride: 4, .. }
    ));
}

#[test]
fn cuda_structures_stream_empty_partial_full_and_brace_elided_initializers() {
    validate("struct Pair { int first, second; }; struct Record { int key; Pair pair; int values[2][2]; int3 point; };
        __global__ void kernel(int *out) {
            Record empty{}; Record partial{1, {2}, {{3}}, {4}};
            Record full{1, {2, 3}, {{4, 5}, {6, 7}}, {8, 9, 10}};
            Record flat{1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
            out[0] = empty.values[1][1] + partial.pair.second + full.point.z + flat.values[1][0];
            int count = 0; Record ordered{count++, {count++, count++}, {{count++, count++}, {count++, count++}}, {count++, count++, count++}};
            out[1] = count + ordered.key + ordered.point.y;
            Record self{2, {self.key + 1, self.pair.first + 1}, {{self.pair.second, 5}, {6, 7}}, {8, 9, 10}};
            out[2] = self.values[0][0];
        }");
}

#[test]
fn cuda_structures_reject_narrowing_excess_and_mismatched_initializer_clauses() {
    for initializer in ["{1.5f}", "{-1}", "{1u, 2u}", "{missing}"] {
        rejected(&format!("struct Record {{ unsigned value; }}; __global__ void kernel() {{ Record value{initializer}; }}"));
    }
    rejected("struct Record { float value; }; __global__ void kernel(int runtime) { Record value{runtime}; }");
    validate(
        "struct Record { float value; }; __global__ void kernel() { Record value{16777216}; }",
    );
    rejected(
        "struct Record { float value; }; __global__ void kernel() { Record value{16777217}; }",
    );
}

#[test]
fn cuda_structures_typed_values_support_arguments_returns_and_temporary_field_reads() {
    validate("struct Record { int value; int values[2]; };
        __device__ Record identity(Record value) { return value; }
        __device__ Record make() { return Record{7, {8, 9}}; }
        __global__ void kernel(int *out) { out[0] = identity(Record{1, {2, 3}}).value; out[1] = make().values[1]; }");
    for source in [
        "struct Record { int value; }; constexpr Record Value{1}; __global__ void kernel() {}",
        "struct Record { int value; }; __global__ void kernel() { constexpr Record value{1}; }",
        "struct Record { int value; }; __global__ void kernel() { constexpr int Count = Record{3}.value; int values[Count]; }",
    ] { rejected(source); }
    for statement in ["make().value = 2;", "++make().value;", "make().value++;"] {
        rejected(&format!("struct Record {{ int value; }}; __device__ Record make() {{ return Record{{1}}; }} __global__ void kernel() {{ {statement} }}"));
    }
}

#[test]
fn cuda_structures_copy_nested_arrays_as_class_values_and_helper_parameter_locals() {
    let (module, _) = validate("struct Record { int values[2][2]; int3 point; };
        __device__ Record modify(Record value) { value.values[0][1] += 3; value.point.z++; return value; }
        __global__ void kernel(int *out) { Record first{{{1, 2}, {3, 4}}, {5, 6, 7}}; Record second = first; Record third{first}; second = modify(first); out[0] = first.values[0][1] + second.values[0][1] + third.point.z; }");
    let (_, helper) = module
        .functions
        .iter()
        .find(|(_, function)| function.name.as_deref() == Some("modify"))
        .unwrap();
    assert_eq!(helper.arguments[0].ty, helper.result.as_ref().unwrap().ty);
    assert!(matches!(
        module.types[helper.arguments[0].ty].inner,
        naga::TypeInner::Struct { .. }
    ));
}

#[test]
fn cuda_structures_reuse_existing_overloads_and_source_defined_operator_addition() {
    validate("struct First { int value; }; struct Second { int value; }; using Alias = First;
        __device__ int pick(First value) { return 1; } __device__ int pick(Second value) { return 2; }
        __device__ First operator+(First left, First right) { left.value += right.value; return left; }
        __global__ void kernel(int *out) { Alias first{3}; Second second{4}; out[0] = pick(first); out[1] = pick(second); out[2] = (first + first).value; }");
    rejected("struct First { int value; }; struct Second { int value; }; __device__ First bad() { return Second{1}; } __global__ void kernel() {}");
    rejected("struct Record { int value; }; __device__ int pick(Record value) { return 1; } __device__ int pick(const Record value) { return 2; } __global__ void kernel() {}");
}

#[test]
fn cuda_structures_const_objects_and_readonly_resources_protect_all_nested_leaves() {
    for declaration in [
        "const Record value{};",
        "using Fixed = const Record; Fixed value{};",
    ] {
        for statement in [
            "value.key = 1;",
            "value.point.x++;",
            "value.values[1] = 2;",
            "value = Record{};",
        ] {
            rejected(&format!("struct Record {{ int key; int3 point; int values[2]; }}; __global__ void kernel() {{ {declaration} {statement} }}"));
        }
    }
    for statement in [
        "input[0].key = 1;",
        "input[0].point.z++;",
        "input[0].values[1] += 2;",
        "input[0] = Record{};",
    ] {
        rejected(&format!("struct Record {{ int key; int3 point; int values[2]; }}; __global__ void kernel(const Record *input) {{ {statement} }}"));
    }
    validate("struct Record { int value; }; __global__ void kernel(const Record *input, int *out) { const Record first = input[0]; Record copy = first; copy.value++; out[0] = copy.value; }");
}

#[test]
fn cuda_structures_storage_pointers_preserve_resource_bindings_and_source_abi() {
    let (module, _) = validate(FIXTURE);
    assert_eq!(module.global_variables.len(), 2);
    for binding in 0..2 {
        let (_, global) = module
            .global_variables
            .iter()
            .find(|(_, global)| global.binding == Some(naga::ResourceBinding { group: 0, binding }))
            .unwrap();
        let access = if binding == 0 {
            naga::StorageAccess::LOAD
        } else {
            naga::StorageAccess::LOAD | naga::StorageAccess::STORE
        };
        assert_eq!(global.space, naga::AddressSpace::Storage { access });
        assert_eq!(storage_record(&module, binding).1, 80);
    }
    rejected("struct Record { int value; }; __global__ void kernel(Record value) {}");
    validate("struct Record { int value; }; __device__ void helper(Record *value) {} __global__ void kernel() {}");
}

#[test]
fn cuda_structures_ordinary_member_indices_snapshot_base_then_index_once() {
    let (module, _) = validate(
        "struct Record { int values[2][2]; };
        __device__ int next(int value) { return value; }
        __global__ void kernel(int *out) {
            Record records[2]{}; int row = 0, column = 0;
            records[next(row++)].values[next(column++)][1] += 7;
            out[0] = records[0].values[0][1]; out[1] = row; out[2] = column;
        }",
    );
    let mut calls = 0;
    naga::proc::any_statement(&module.entry_points[0].function.body, &mut |statement| {
        if let naga::Statement::Call { function, .. } = statement {
            calls += usize::from(module.functions[*function].name.as_deref() == Some("next"));
        }
        false
    });
    assert_eq!(calls, 2);
}

#[test]
fn cuda_structures_conditional_record_scalar_and_vector_fields_remain_lvalues() {
    validate(
        "struct Nested { int value; }; struct Record { Nested nested; int3 point; };
        __global__ void kernel(int *out, int choose) {
            Record first{{1}, {2, 3, 4}}, second{{5}, {6, 7, 8}};
            (choose ? first : second).nested.value += 2;
            (choose ? first : second).point.x++;
            out[0] = first.nested.value + second.point.x;
        }",
    );
}

#[test]
fn cuda_structures_conditional_array_subscripts_are_explicitly_unsupported() {
    for expression in [
        "(choose ? first : second).values[0]",
        "(choose ? first : second).values[index++]",
    ] {
        let source = format!("struct Record {{ int values[2]; }}; __global__ void kernel(int *out, int choose) {{ Record first{{}}, second{{}}; int index = 0; out[0] = {expression}; }}");
        let error = rejected(&source);
        assert!(error.message.contains("conditional"), "{}", error.message);
    }
}

#[test]
fn cuda_structures_array_members_reject_value_copies_and_accept_borrowed_decay() {
    let prefix = "struct Record { int values[2][2]; };";
    for body in [
        "Record first{}, second{}; first.values = second.values;",
        "Record first{}, second{}; first.values[0] = second.values[0];",
        "Record first{}; Record second = first.values;",
        "Record first{}; int value = first.values;",
        "Record first{}; consume(first.values);",
    ] {
        rejected(&format!("{prefix} __device__ int consume(int value) {{ return value; }} __global__ void kernel() {{ {body} }}"));
    }
    validate("struct Record { int values[2]; }; __global__ void kernel(int *out) { Record values[2]{}; int index = 0; out[0] = (values[index++].values, 7); out[1] = index; }");
    validate("struct Record { int values[2][2]; }; __global__ void kernel() { Record first{}; first.values + 1; }");
}

#[test]
fn cuda_structures_shared_records_reuse_existing_storage_and_barrier_policy() {
    let (module, _) = validate("struct Record { int key; int values[2]; }; __global__ void kernel(int *out) { __shared__ Record values[4]; unsigned lane = threadIdx.x; values[lane] = Record{int(lane), {1, 2}}; __syncthreads(); out[lane] = values[(lane + 1u) % 4u].values[1]; }");
    assert!(module
        .global_variables
        .iter()
        .any(
            |(_, global)| global.space == naga::AddressSpace::WorkGroup && global.binding.is_none()
        ));
    rejected("struct Record { int value; }; __global__ void kernel(int *out) { __shared__ Record value; if (threadIdx.x == 0u) __syncthreads(); out[0] = value.value; }");
    rejected("struct Record { int value; }; __device__ Record helper() { __shared__ Record value; return value; } __global__ void kernel() {}");
}

#[test]
fn cuda_structures_switch_lifetimes_distinguish_explicit_and_vacuous_initialization() {
    for declaration in [
        "Record value{};",
        "Record value{1};",
        "Record value = Record{1};",
    ] {
        rejected(&format!("struct Record {{ int value; }}; __global__ void kernel(int *out, int selector) {{ switch (selector) {{ case 0: {declaration} case 1: out[0] = 1; }} }}"));
    }
    validate("struct Record { int value; }; __global__ void kernel(int *out, int selector) { switch (selector) { case 0: Record value; case 1: value.value = 9; out[0] = value.value; break; default: out[0] = 1; } }");
    validate("struct Record { int value; }; __global__ void kernel(int *out, int selector) { switch (selector) { case 0: { Record value{}; out[0] = value.value; } case 1: out[1] = 7; } }");
}

#[test]
fn cuda_structures_reject_duplicate_definitions_members_and_incomplete_self_types() {
    for source in [
        "struct Record { int value; }; struct Record { int value; }; __global__ void kernel() {}",
        "struct Record { int value, value; }; __global__ void kernel() {}",
        "struct Record { Record value; }; __global__ void kernel() {}",
        "struct First { Second value; }; struct Second { int value; }; __global__ void kernel() {}",
        "struct Record { int value; }; __global__ void kernel() { struct Record { Record value; }; }",
    ] { rejected(source); }
}

#[test]
fn cuda_structures_reject_const_bool_pointer_and_advanced_class_members() {
    for body in [
        "",
        "const int value;",
        "using Fixed = const int; Fixed value;",
        "bool value;",
        "char value;",
        "double value;",
        "int *value;",
        "int &value;",
        "int value();",
        "static int value;",
        "int value : 3;",
        "int values[];",
        "int value = 3;",
        "Record() {}",
        "struct Nested { int value; }; Nested nested;",
        "union { int value; float other; };",
    ] {
        rejected(&format!(
            "struct Record {{ {body} }}; __global__ void kernel() {{}}"
        ));
    }
    rejected(
        "using Fixed = const int; struct Record { Fixed value; }; __global__ void kernel() {}",
    );
    for source in [
        "class Record { int value; }; __global__ void kernel() {}",
        "union Record { int value; float other; }; __global__ void kernel() {}",
        "struct alignas(16) Record { int value; }; __global__ void kernel() {}",
        "struct Record : Other { int value; }; __global__ void kernel() {}",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_structures_incomplete_and_elaborated_type_spellings_remain_unsupported() {
    for source in [
        "struct Record; __global__ void kernel() {}",
        "struct Record { int value; }; struct Record; __global__ void kernel() {}",
        "struct Record { int value; }; __global__ void kernel(struct Record *input) {}",
        "struct Record { int value; }; __global__ void kernel() { struct Record value; }",
        "struct Record { int value; }; __device__ struct Record helper() { return Record{}; } __global__ void kernel() {}",
        "struct Record { int value; }; struct Other { struct Record nested; }; __global__ void kernel() {}",
    ] { rejected(source); }
}

#[test]
fn cuda_structures_malformed_and_dead_declarations_keep_original_utf8_spans() {
    for source in [
        "/* λ */ struct Record { int value } ; __global__ void kernel() {}",
        "/* λ */ struct Record { int; }; __global__ void kernel() {}",
        "/* λ */ struct Record { int value; __global__ void kernel() {}",
        "/* λ */ struct Record { int value; }; __global__ void kernel() { return; Record value{missing}; }",
        "/* λ */ __global__ void kernel() { return; struct Record { bool value; }; }",
    ] { rejected(source); }
    validate("struct Record { int value; }; __global__ void kernel() { return; Record value{}; value.value = 2; }");
}

#[test]
fn cuda_structures_check_type_size_and_member_array_bound_overflow() {
    let count = naga::valid::MAX_TYPE_SIZE / 4;
    rejected(&format!(
        "struct Record {{ int values[{count}u]; int extra; }}; __global__ void kernel() {{}}"
    ));
    for bound in ["0", "-1", "0xffffffffu", "runtime", "1.5f"] {
        rejected(&format!(
            "struct Record {{ int values[{bound}]; }}; __global__ void kernel(int runtime) {{}}"
        ));
    }
    validate("constexpr int Count = 4; struct Record { int values[Count]; }; __global__ void kernel() { Record value{}; }");
}

#[test]
fn cuda_structures_bound_type_and_initializer_depth_without_recursive_panics() {
    for depth in [8, 128] {
        let mut source = String::from("struct Record0 { int value; };\n");
        for index in 1..=depth {
            source.push_str(&format!(
                "struct Record{index} {{ Record{} nested; }};\n",
                index - 1
            ));
        }
        source.push_str(&format!(
            "__global__ void kernel() {{ Record{depth} value{{}}; }}"
        ));
        if depth == 8 {
            validate(&source);
        } else {
            rejected(&source);
        }
    }
    let source = format!(
        "struct Record {{ int values[1]; }}; __global__ void kernel() {{ Record value{}0{}; }}",
        "{".repeat(128),
        "}".repeat(128)
    );
    rejected(&source);
}

#[test]
fn cuda_structures_flat_repeated_copies_reuse_types_and_lower_linearly() {
    let prefix = "struct Record { int key; int values[2]; };";
    let (single, _) = validate(&format!("{prefix} __global__ void kernel(int *out) {{ Record source{{1, {{2, 3}}}}; Record target = source; out[0] = target.key; }}"));
    let (many, _) = validate(&format!("{prefix} __global__ void kernel(int *out) {{ Record source{{1, {{2, 3}}}}; Record target = source; {} }}", "target = source; out[0] = target.values[1];".repeat(128)));
    assert_eq!(single.types.len(), many.types.len());
    assert!(
        many.entry_points[0].function.expressions.len()
            <= single.entry_points[0].function.expressions.len() * 128 + 256
    );
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[test]
fn cuda_structures_fixture_roundtrips_through_common_wgsl_helpers() {
    let (module, info) = super::helpers::validate_with_options(
        FIXTURE,
        &Options {
            workgroup_size: [2, 1, 1],
        },
    );
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}

#[cfg(spv_out)]
#[cfg_attr(miri, ignore)]
#[test]
fn cuda_structures_fixture_emits_valid_spirv_binary() {
    let (module, info) = super::helpers::validate_with_options(
        FIXTURE,
        &Options {
            workgroup_size: [2, 1, 1],
        },
    );
    let options = naga::back::spv::Options {
        lang_version: (1, 1),
        ..Default::default()
    };
    let pipeline = naga::back::spv::PipelineOptions {
        entry_point: "structures".to_string(),
        shader_stage: naga::ShaderStage::Compute,
    };
    let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
    crate::helpers::validate_spirv(&words);
}

#[test]
fn cuda_structures_member_names_shadow_compute_builtins_in_bounds() {
    for field_name in ["threadIdx", "blockIdx", "gridDim", "blockDim"] {
        validate(&format!(
            "__global__ void kernel(int *out) {{
                struct Record {{ int {field_name}; }};
                Record value{{7}};
                out[0] = value.{field_name} + int(blockDim.x);
            }}"
        ));
        for invalid_expression in [
            format!("false ? {field_name}.x : 4"),
            format!("true ? 4 : {field_name}.x"),
        ] {
            let source = format!(
                "struct Record {{ int {field_name}; int values[{invalid_expression}]; }};
                __global__ void kernel() {{}}"
            );
            let error = rejected(&source);
            assert!(error.message.contains("member"), "{field_name}: {error}");
            assert_eq!(&source[error.span.to_range().unwrap()], field_name);
        }
    }
}
