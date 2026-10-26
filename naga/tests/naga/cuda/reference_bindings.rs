use naga::front::cuda::{parse_str, Options};

use super::helpers::{validate, validate_with_options};

const FIXTURE: &str = include_str!("../../in/cuda/reference-bindings.cu");

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty() || range.start == source.len(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    error
}

fn root_expression(
    function: &naga::Function,
    mut pointer: naga::Handle<naga::Expression>,
) -> naga::Handle<naga::Expression> {
    loop {
        match function.expressions[pointer] {
            naga::Expression::Access { base, .. } | naga::Expression::AccessIndex { base, .. } => {
                pointer = base
            }
            _ => return pointer,
        }
    }
}

#[test]
fn cuda_local_references_preserve_mutable_live_aliases() {
    validate(
        "__global__ void kernel(int *out) {
            int value = 1; int &alias = value; int &other = value;
            alias = 5; value += 2; other += alias;
            int replacement = 3; alias = replacement; replacement = 9;
            out[0] = value; out[1] = alias; out[2] = other; out[3] = replacement;
        }",
    );
}

#[test]
fn cuda_local_references_preserve_const_views_of_mutable_objects() {
    validate(
        "__global__ void kernel(int *out) {
            int value = 3; const int &view = value;
            int &alias = value; alias = 7; value += view;
            out[0] = view; out[1] = value;
        }",
    );
    for statement in ["view = 2;", "++view;", "view += 2;"] {
        rejected(&format!(
            "__global__ void kernel() {{ int value = 1; const int &view = value; {statement} }}"
        ));
    }
    rejected("__global__ void kernel(unsigned choose) { const int first = 1; int second = 2; int &alias = choose ? first : second; }");
    rejected("struct Record { int values[2]; float2 vector; }; __global__ void kernel() { Record value{}; const Record &view = value; view.values[0] = 7; }");
    rejected("struct Record { int values[2]; float2 vector; }; __global__ void kernel() { Record value{}; const Record &view = value; view.vector.y = 7.0f; }");
}

#[test]
fn cuda_local_references_chain_bindings_without_replaying_initializers() {
    validate(
        "__global__ void kernel(int *out) {
            int values[2] = {3, 7}; int index = 0;
            int &first = values[index++]; int &second = first;
            const int &third = second; const int &fourth = third;
            index = 1; values[0] = 11; second += 2;
            out[0] = first; out[1] = third; out[2] = fourth; out[3] = index;
        }",
    );
    rejected(
        "__global__ void kernel() {
            int value = 1; const int &first = value; int &second = first;
        }",
    );
}

#[test]
fn cuda_local_references_bind_all_supported_scalar_types() {
    for (scalar, initial, update) in [
        ("bool", "false", "true"),
        ("int", "-3", "7"),
        ("unsigned", "0x80000000u", "9u"),
        ("float", "1.5f", "2.0f"),
    ] {
        validate(&format!(
            "using Value = {scalar}; using Frozen = const Value;
            __global__ void kernel(unsigned *out) {{
                Value value = {initial}; Value &alias = value; Frozen &view = alias;
                alias = {update}; out[0] = unsigned(view);
            }}"
        ));
    }
}

#[test]
fn cuda_local_references_bind_all_existing_vector_types_and_components() {
    for scalar in ["int", "uint", "float"] {
        for width in 2..=4 {
            let vector = format!("{scalar}{width}");
            let initial = core::iter::repeat_n("1", width)
                .collect::<Vec<_>>()
                .join(", ");
            let component_type = match scalar {
                "uint" => "unsigned",
                other => other,
            };
            let component = match width {
                2 => "y",
                3 => "z",
                _ => "w",
            };
            validate(&format!(
                "__global__ void kernel(unsigned *out) {{
                    {vector} value{{{initial}}}; {vector} &alias = value;
                    const {vector} &view = alias;
                    {component_type} &part = alias.{component};
                    part = 7; alias.x = 3;
                    out[0] = unsigned(view.{component}); out[1] = unsigned(value.x);
                }}"
            ));
        }
    }
}

#[test]
fn cuda_local_references_bind_nominal_records_and_completed_aliases() {
    validate(
        "struct Record { int x; int y; }; using Alias = Record; typedef Alias Copy;
        using Frozen = const Record;
        __global__ void kernel(int *out) {
            Record value{3, 5}; Alias &alias = value; Copy &same = alias;
            Frozen &view = same; same.x = 9; alias = Record{7, 11};
            out[0] = value.x; out[1] = view.y;
            { struct Record { int x; int y; }; Record inner{13, 17};
              Record &local = inner; out[2] = local.x; }
            Copy &restored = value; out[3] = restored.y;
        }",
    );
    for qualifier in ["", "const "] {
        rejected(&format!(
            "struct First {{ int x; }}; struct Second {{ int x; }};
            __global__ void kernel() {{ First value{{1}}; {qualifier}Second &wrong = value; }}"
        ));
    }
}

#[test]
fn cuda_local_references_project_nested_records_and_array_elements() {
    validate(
        "struct Leaf { int value; }; struct Record { Leaf leaves[2][2]; float2 vectors[2]; };
        __global__ void kernel(unsigned *out) {
            Record value{}; Record &record = value;
            Leaf &leaf = record.leaves[1][0]; int &field = leaf.value;
            float2 &vector = record.vectors[1]; float &component = vector.y;
            field = 7; component = 2.0f;
            const Record &view = record;
            out[0] = unsigned(view.leaves[1][0].value); out[1] = __float_as_uint(view.vectors[1].y);
            Record copies[2] = {value, value}; Record &copy = copies[1]; copy.leaves[0][0].value = 11;
            out[2] = unsigned(copies[1].leaves[0][0].value);
        }",
    );
    rejected(
        "struct Record { int values[2]; }; __global__ void kernel() {
            Record record{}; int &wrong = record.values;
        }",
    );
}

#[test]
fn cuda_local_references_bind_writable_storage_elements() {
    let (module, _) = validate(
        "struct Record { int value; int samples[2]; };
        __global__ void kernel(Record *records, int *out) {
            Record &record = records[1]; int &sample = record.samples[0];
            record.value = 3; sample = 7; int &destination = out[0];
            destination = record.value + sample;
        }",
    );
    assert_eq!(module.global_variables.len(), 2);
    for (_, global) in module.global_variables.iter() {
        assert_eq!(
            global.space,
            naga::AddressSpace::Storage {
                access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE,
            }
        );
    }
}

#[test]
fn cuda_local_references_propagate_readonly_storage_access() {
    let (module, _) = validate(
        "struct Record { int value; float2 vector; };
        __global__ void kernel(const Record *input, unsigned *out) {
            const Record &record = input[0]; const int &field = record.value;
            const float2 &vector = record.vector; const float &component = vector.y;
            out[0] = unsigned(field); out[1] = __float_as_uint(component);
        }",
    );
    let input = module
        .global_variables
        .iter()
        .find(|(_, global)| {
            global.binding
                == Some(naga::ResourceBinding {
                    group: 0,
                    binding: 0,
                })
        })
        .unwrap()
        .1;
    assert_eq!(
        input.space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD
        }
    );
    for statement in [
        "int &alias = input[0];",
        "const int &alias = input[0]; alias = 7;",
        "const int &alias = input[0]; int &other = alias;",
    ] {
        rejected(&format!(
            "__global__ void kernel(const int *input) {{ {statement} }}"
        ));
    }
}

#[test]
fn cuda_local_references_bind_workgroup_elements_under_existing_barrier_policy() {
    validate_with_options(
        "struct Record { int value; float2 vector; };
        __global__ void kernel(unsigned *out) {
            __shared__ Record values[4]; unsigned lane = threadIdx.x;
            values[lane].value = int(lane); values[lane].vector = make_float2(1.0f, 2.0f);
            __syncthreads();
            Record &record = values[lane]; const Record &view = record;
            int &field = record.value; float &component = record.vector.y;
            field += 3; component += 1.0f;
            __syncthreads();
            out[lane * 2u] = unsigned(view.value);
            out[lane * 2u + 1u] = __float_as_uint(view.vector.y);
        }",
        &Options {
            workgroup_size: [4, 1, 1],
        },
    );
    rejected("__device__ int helper() { __shared__ int value; int &alias = value; return alias; } __global__ void kernel() {}");
    rejected("__global__ void kernel() { int value = 1; __shared__ int &alias = value; }");
}

#[test]
fn cuda_local_references_materialize_converted_scalar_temporaries() {
    validate(
        "__global__ void kernel(unsigned *out) {
            int value = 3; const float &converted = value; const int &live = value;
            value = 9; out[0] = unsigned(converted); out[1] = unsigned(live);
            const int &truncated = 3.75f; const unsigned &wrapped = -1;
            const bool &truth = 2; const float &arithmetic = 1.0f + 0.5f;
            out[2] = unsigned(truncated); out[3] = wrapped;
            out[4] = unsigned(truth); out[5] = __float_as_uint(arithmetic);
        }",
    );
}

#[test]
fn cuda_local_references_distinguish_direct_alias_lists_from_temporary_lists() {
    validate(
        "struct Record { int value; };
        __global__ void kernel(int *out) {
            int value = 3; int &first{value}; const int &second = {first};
            first = 7; out[0] = second;
            const int &zero{}; const int &other_zero = {};
            const float &converted{3}; const unsigned &nonnegative = {7};
            Record record{9}; Record &record_alias{record}; const Record &view = {record_alias};
            record_alias.value = 11;
            out[1] = zero + other_zero + int(converted) + int(nonnegative); out[2] = view.value;
        }",
    );
    for declaration in [
        "const int &value{3.75f};",
        "const int &value = {3.75f};",
        "const unsigned &value{-1};",
        "const bool &value{2};",
        "int source = 7; const float &value{source};",
        "const int &value{1, 2};",
    ] {
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
}

#[test]
fn cuda_local_references_initialize_const_aggregate_temporaries() {
    validate(
        "struct Leaf { int value; }; struct Record { Leaf leaf; int values[2][2]; float2 vector; };
        __global__ void kernel(unsigned *out) {
            const Record &empty{}; const Record &copy_empty = {};
            const Record &partial{{3}, {{5}}, {1.0f, 2.0f}};
            const Record &complete = {{7}, {{11, 13}, {17, 19}}, {3.0f, 4.0f}};
            const Record &elided{23, 29, 31, 37, 41, 5.0f, 6.0f};
            const Record &typed = Record{{43}, {{47, 53}, {59, 61}}, {7.0f, 8.0f}};
            const float3 &vector{1.0f, 2.0f, 3.0f}; const float3 &zero = {};
            out[0] = unsigned(empty.leaf.value + copy_empty.values[1][1] + partial.values[1][1]);
            out[1] = unsigned(complete.values[1][1] + elided.leaf.value + typed.leaf.value);
            out[2] = __float_as_uint(vector.z + zero.x);
        }",
    );
    rejected(
        "struct Record { int value; }; __global__ void kernel() { const Record &value{1.5f}; }",
    );
}

#[test]
fn cuda_local_references_preserve_temporary_lifetime_and_single_evaluation() {
    validate(
        "struct Record { int value; int samples[2]; };
        __device__ Record make_record(int value) { return Record{value, {value + 1}}; }
        __global__ void kernel(int *out) {
            int counter = 3;
            const Record &record = make_record(counter++);
            const int &field = Record{counter++, {}}.value;
            counter = 11;
            for (int index = 0; index < 3; ++index) {
                const Record &iteration{index + 1, {index + 2}};
                out[index] = record.value + field + iteration.value + iteration.samples[1];
            }
            { const int &view = record.samples[0]; out[3] = view; }
            out[4] = record.value; out[5] = field; out[6] = counter;
        }",
    );
}

#[test]
fn cuda_local_references_freeze_array_index_side_effects() {
    validate(
        "__global__ void kernel(int *out) {
            int values[2][3] = {{1, 2, 3}, {4, 5, 6}};
            int row = 0, column = 1;
            int &alias = values[row++][column++];
            row = 1; column = 2; values[0][1] = 7;
            alias += 3; out[0] = alias; alias += 2;
            out[1] = values[0][1]; out[2] = row; out[3] = column;
        }",
    );
}

#[test]
fn cuda_local_references_freeze_conditional_selection() {
    validate(
        "__global__ void kernel(int *out) {
            int first[2] = {3, 5}, second[2] = {7, 11};
            int first_index = 0, second_index = 0; bool choose = true;
            int &selected = choose ? first[first_index++] : second[second_index++];
            choose = false; selected += 2;
            out[0] = selected; out[1] = first[0]; out[2] = first_index; out[3] = second_index;
            int &unused = choose ? first[first_index++] : second[second_index++];
            out[4] = first_index; out[5] = second_index;
        }",
    );
}

#[test]
fn cuda_local_references_freeze_conditional_branch_indices_and_roots() {
    validate_with_options(
        "__global__ void kernel(int *out, unsigned choose) {
            __shared__ int shared[2]; shared[0] = 3; shared[1] = 5;
            __syncthreads();
            int local[2] = {7, 11}; int index = 0; out[4] = 13;
            int &first = choose ? local[index++] : shared[++index];
            int &second = choose ? first : out[4];
            choose = 1u; index = 0; first += 2; second += 3;
            out[0] = first; out[1] = second; out[2] = index;
        }",
        &Options {
            workgroup_size: [1, 1, 1],
        },
    );
}

#[test]
fn cuda_local_references_project_conditional_record_and_vector_lvalues() {
    validate(
        "struct Record { int x; int y; int values[2]; float2 vector; };
        __global__ void kernel(unsigned *out, unsigned choose) {
            Record first{3, 5, {7, 11}, {1.0f, 2.0f}};
            Record second{13, 17, {19, 23}, {3.0f, 4.0f}};
            Record &record = choose ? first : second;
            int &x = record.x; int &y = record.y;
            int &remapped = choose ? x : y;
            int index = 0; int &element = record.values[index++];
            float &component = record.vector.y;
            choose = 0u; index = 1; remapped += 2; element += 3; component += 1.0f;
            out[0] = unsigned(first.x); out[1] = unsigned(first.y);
            out[2] = unsigned(second.x); out[3] = unsigned(second.y);
            out[4] = unsigned(element); out[5] = unsigned(index); out[6] = __float_as_uint(component);
            Record &other_record = choose ? first : second;
            int &projected = other_record.values[index++];
            projected = 29; out[7] = unsigned(second.values[1]);
        }",
    );
    rejected("struct Record { int values[2]; }; __global__ void kernel(unsigned choose) { Record first{}, second{}; int &alias = (choose ? first : second).values[0]; }");
}

#[test]
fn cuda_local_references_bind_uninitialized_objects_without_loading() {
    for source in [
        "__global__ void kernel() { int uninitialized; int &alias = uninitialized; alias = 7; }",
        "__global__ void kernel(unsigned choose) {
            int uninitialized, other; int &alias = choose ? uninitialized : other; alias = 7;
        }",
        "struct Record { int value; int samples[2]; }; __global__ void kernel() {
            Record uninitialized; Record &alias = uninitialized;
            alias.value = 7; int &field = alias.samples[0]; field = 11;
        }",
    ] {
        let (module, _) = validate(source);
        let function = &module.entry_points[0].function;
        for (_, expression) in function.expressions.iter() {
            if let naga::Expression::Load { pointer } = *expression {
                let root = root_expression(function, pointer);
                if let naga::Expression::LocalVariable(local) = function.expressions[root] {
                    let name = function.local_variables[local].name.as_deref();
                    assert!(!matches!(name, Some("uninitialized" | "other")), "{source}");
                }
            }
        }
    }
}

#[test]
fn cuda_local_references_recognize_existing_lvalue_expression_forms() {
    validate(
        "__global__ void kernel(int *out) {
            int value = 1, other = 3, counter = 0;
            int &parenthesized = ((value)); int &prefix = ++value;
            int &assignment = (value = 5); int &comma = (++counter, other);
            assignment += 2; comma += 4;
            const int &postfix = value++; const int &arithmetic = value + 2;
            const float &cast = float(value);
            out[0] = parenthesized + prefix + assignment + comma + postfix + arithmetic + int(cast);
            out[1] = counter;
        }",
    );
    for initializer in ["value++", "value + 1", "int(value)"] {
        rejected(&format!(
            "__global__ void kernel() {{ int value = 1; int &alias = {initializer}; }}"
        ));
    }
}

#[test]
fn cuda_local_references_parse_mixed_declarators_in_sequence() {
    validate(
        "__global__ void kernel(int *out) {
            int value = 1, other = 3;
            int &first = value, copy = first, &second = other;
            int &updated = (first = 5), snapshot = first, &chain = updated;
            second = 7; chain = 11;
            out[0] = value; out[1] = other; out[2] = copy; out[3] = snapshot;
            const int &temporary = 13, ordinary = temporary, &view = value;
            value = 17; out[4] = temporary + ordinary + view;
        }",
    );
}

#[test]
fn cuda_local_references_preserve_for_initializer_and_iteration_scope() {
    validate(
        "__global__ void kernel(int *out) {
            int value = 0;
            for (int &cursor = value; cursor < 3; ++cursor) { if (cursor == 1) continue; out[cursor] = cursor; }
            int values[3] = {3, 5, 7};
            for (int index = 0; index < 3; ++index) { int &element = values[index]; element += index; }
            while (value < 5) { int &cursor = value; ++cursor; if (cursor == 4) continue; break; }
            do { int &cursor = value; ++cursor; } while (value < 6);
            out[3] = value; out[4] = values[2];
        }",
    );
    rejected("__global__ void kernel() { int value = 0; for (int &cursor = value; cursor < 1; ++cursor) {} cursor = 3; }");
}

#[test]
fn cuda_local_references_preserve_name_scope_and_declaration_locus() {
    validate(
        "__global__ void kernel(int *out) {
            int first = 3, second = 5; int &alias = first;
            { int &alias = second; alias = 7; }
            alias = 11; out[0] = first; out[1] = second;
        }",
    );
    for source in [
        "__global__ void kernel() { int value = 1; int &alias = value; int &alias = value; }",
        "__global__ void kernel() { int alias = 1; { int &alias = alias; } }",
        "__global__ void kernel() { int &alias = alias; }",
        "__global__ void kernel() { int value = 1; int &threadIdx = value; }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_local_references_reject_switch_jumps_over_reference_initializers() {
    for body in [
        "case 0: int &alias = value; break; default: break;",
        "case 0: const int &alias = value; return; case 1: break;",
        "case 0: if (false) { return; } int &alias = value; break; default: break;",
    ] {
        let error = rejected(&format!(
            "__global__ void kernel(int selector) {{ int value = 1; switch (selector) {{ {body} }} }}"
        ));
        assert!(error.message.contains("initializer"), "{error}");
    }
    validate(
        "__global__ void kernel(int *out, int selector) {
            int value = 1;
            switch (selector) {
                case 0: { int &alias = value; alias += 2; break; }
                default: { const int &alias = value; out[0] = alias; break; }
            }
        }",
    );
}

#[test]
fn cuda_local_references_allow_local_references_in_helper_and_kernel_bodies() {
    let (module, _) = validate(
        "__device__ int adjust(int value) { int &alias = value; alias += 2; return alias; }
        __device__ int read(const int value) { const int &view = value; return view; }
        __global__ void kernel(int *out, int value) {
            int &alias = value; alias += 3; out[0] = adjust(alias); out[1] = read(alias); out[2] = value;
        }",
    );
    assert_eq!(module.global_variables.len(), 2);
    let uniform = module
        .global_variables
        .iter()
        .find(|(_, global)| {
            global.binding
                == Some(naga::ResourceBinding {
                    group: 0,
                    binding: 1,
                })
        })
        .unwrap()
        .1;
    assert_eq!(uniform.space, naga::AddressSpace::Uniform);
    rejected("__device__ int read(const int value) { int &alias = value; return alias; } __global__ void kernel() {}");
}

#[test]
fn cuda_local_references_preserve_reference_constant_expression_provenance() {
    validate(
        "constexpr int Global = 7;
        __global__ void kernel(int *out) {
            constexpr int count = Global; int values[count] = {};
            const int &arithmetic = Global + 0; const int &cast = int(Global);
            const int &positive = +Global; const float &converted = Global;
            out[0] = arithmetic + cast + positive + int(converted) + values[0];
        }",
    );
    for initializer in [
        "Global",
        "(Global)",
        "{Global}",
        "(0, Global)",
        "true ? Global : Other",
    ] {
        rejected(&format!(
            "constexpr int Global = 7; constexpr int Other = 11;
            __global__ void kernel() {{ const int &alias = {initializer}; }}"
        ));
    }
    rejected("constexpr int Global = 7; __global__ void kernel() { const int &alias{Global}; }");
    for binding in [
        "const int &alias = 2;",
        "const int value = 2; const int &alias = value;",
        "int value = 2; const int &alias = value;",
        "const int &first = 2; const int &alias = first;",
        "const int &alias = Global + 0;",
        "const float &alias = Global;",
        "__shared__ int shared; const int &alias = (helper(), shared);",
        "__shared__ int shared; int value = 1; const int &alias = (value + 0, shared);",
        "__shared__ int shared; const int &first = (helper(), shared); const int &alias = first;",
        "__shared__ int shared; int &alias = shared;",
        "__shared__ int shared[3]; const int &alias = shared[2];",
    ] {
        let error = rejected(&format!(
            "constexpr int Global = 2; __device__ int helper() {{ return 0; }}
            __global__ void kernel() {{ {binding} constexpr int result = int(alias); }}"
        ));
        assert!(error.message.contains("constant"), "{binding}: {error}");
    }
}

#[test]
fn cuda_local_references_preserve_discarded_glvalue_and_lazy_constant_rules() {
    validate(
        "__global__ void kernel(int *out) {
            __shared__ int shared; __shared__ int shared_values[3];
            int &direct = shared; const int &view = direct;
            int &element = shared_values[2]; int &chain = element;
            int &comma = (0, shared); int &selected = true ? shared_values[0] : shared_values[1];
            int local = 1; int &static_selected = true ? shared : local;
            constexpr int discarded = (direct, 3); constexpr int viewed = (view, 5);
            constexpr int indexed = (element, 7); constexpr int chained = (chain, 11);
            constexpr int comma_value = (comma, 13); constexpr int selected_value = (selected, 17);
            constexpr int static_value = (static_selected, 19);
            int &alias = local;
            constexpr int lazy_read = false ? alias : 5;
            constexpr bool lazy_and = false && alias;
            constexpr bool lazy_or = true || alias;
            constexpr int lazy_discard = true ? 7 : (alias, 9);
            out[0] = discarded + viewed + indexed + chained + comma_value + selected_value
                + static_value + lazy_read + int(lazy_and) + int(lazy_or) + lazy_discard;
        }",
    );
    for binding in [
        "int value = 1; int &alias = value;",
        "const int &alias = 1;",
        "int values[2] = {1, 2}; int index = 0; int &alias = values[index++];",
        "__shared__ int shared; const int &alias = (helper(), shared);",
        "__shared__ int shared[3]; int index = 1; int &alias = shared[index];",
        "__shared__ int shared; int value = 1; int &alias = (value++, shared);",
        "__shared__ int shared; int &first = (helper(), shared); int &alias = first;",
        "int &alias = out[0];",
        "int &alias = parameter;",
        "__shared__ int shared[2]; int &alias = choose ? shared[0] : shared[1];",
        "__shared__ int shared; int local = 1; int &alias = false ? shared : local;",
        "__shared__ int shared[3]; int &alias = shared[3];",
    ] {
        let error = rejected(&format!(
            "constexpr int Global = 2; __device__ int helper() {{ return 0; }}
            __global__ void kernel(int *out, int parameter, unsigned choose) {{ {binding} constexpr int discarded = (alias, 3); }}"
        ));
        assert!(error.message.contains("constant"), "{binding}: {error}");
    }
}

#[test]
fn cuda_local_references_reject_invalid_mutable_and_const_bindings() {
    for declaration in [
        "int &alias;",
        "const int &alias;",
        "int &alias = 1;",
        "int value = 1; float &alias = value;",
        "unsigned value = 1u; int &alias = value;",
        "const int value = 1; int &alias = value;",
        "int value = 1; int &alias = value++;",
        "int &alias{};",
        "int &alias = {};",
        "int value = 1; int &alias{value, value};",
        "int value = 1; int &const alias = value;",
        "int value = 1; int &alias(value);",
        "int value = 1; int (&alias) = value;",
    ] {
        rejected(&format!("__global__ void kernel() {{ {declaration} }}"));
    }
}

#[test]
fn cuda_local_references_retain_unsupported_reference_grammar_boundaries() {
    validate("__device__ int helper(int &value) { return value; } __global__ void kernel() { int value = 3; helper(value); }");
    validate("__device__ int helper(const int &value); __global__ void kernel() {}");
    for source in [
        "__global__ void kernel(int &value) {}",
        "__device__ int &helper(int value) { return value; } __global__ void kernel() {}",
        "using Reference = int&; __global__ void kernel() {}",
        "typedef int &Reference; __global__ void kernel() {}",
        "__global__ void kernel() { int value = 1; int &&alias = value; }",
        "__global__ void kernel() { int value[2] = {}; int (&alias)[2] = value; }",
        "__global__ void kernel() { int &values[2]; }",
        "__global__ void kernel(int *input) { int *&alias = input; }",
        "struct Record { int &member; }; __global__ void kernel() {}",
        "__global__ void kernel() { int value = 1; volatile int &alias = value; }",
        "constexpr int Global = 1; const int &Alias = Global; __global__ void kernel() {}",
        "constexpr int Global = 1; __global__ void kernel() { constexpr const int &alias = Global; }",
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_local_references_report_original_utf8_and_macro_spans() {
    for source in [
        "/* 漢字 */ __global__ void kernel() { int value = 1; const int &alias = value; alias = 7; }",
        "#define BAD 1.5f\n/* λ */ __global__ void kernel() { const int &alias{BAD}; }",
        "#define TARGET value\n/* 漢 */ __global__ void kernel() { const int value = 1; int &alias = TARGET; }",
        "/* λ */ __global__ void kernel() { if (false) { int &alias = 3; } }",
        "/* λ */ __global__ void kernel() { int value = 1; int &alias = ; }",
    ] {
        let error = rejected(source);
        let range = error.span.to_range().unwrap();
        assert!(range.start >= source.find("__global__").unwrap(), "{source}: {error}");
    }
}

#[test]
fn cuda_local_references_bound_address_graph_and_parser_work() {
    let mut source = String::from(
        "__global__ void kernel(int *out, unsigned choose) { int first = 1, second = 2;
        int &alias0 = choose ? first : second;",
    );
    for index in 1..=257 {
        let previous = index - 1;
        source.push_str(&format!(
            "int &alias{index} = choose ? alias{previous} : alias{previous};"
        ));
    }
    source.push_str("alias257 += 3; out[0] = alias257; }");
    validate(&source);

    let mut source = String::from(
        "__global__ void kernel(unsigned choose) { int values[400]; int &alias0 = values[0];",
    );
    for index in 1..400 {
        let previous = index - 1;
        source.push_str(&format!(
            "int &alias{index} = choose ? alias{previous} : values[{index}];"
        ));
    }
    source.push_str("alias399 = 7; }");
    assert!(rejected(&source).message.contains("limit"));

    let mut expression = String::from("value");
    for _ in 0..256 {
        expression = format!("choose ? value : ({expression})");
    }
    let error = rejected(&format!(
        "__global__ void kernel(unsigned choose) {{ int value = 1; int &alias = {expression}; }}"
    ));
    assert!(error.message.contains("depth") || error.message.contains("limit"));
}

#[cfg_attr(miri, ignore)]
#[test]
fn cuda_local_references_validate_and_roundtrip_shared_backends() {
    let (module, info) = validate(FIXTURE);
    let function = &module.entry_points[0].function;
    assert!(info
        .get_entry_point(0)
        .available_stages
        .contains(naga::valid::ShaderStages::COMPUTE));
    assert!(function.local_variables.iter().all(|(_, local)| !matches!(
        module.types[local.ty].inner,
        naga::TypeInner::Pointer { .. }
    )));
    assert!(module
        .functions
        .iter()
        .all(
            |(_, function)| function.arguments.iter().all(|argument| !matches!(
                module.types[argument.ty].inner,
                naga::TypeInner::Pointer { .. }
            ))
        ));
    #[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    #[cfg(spv_out)]
    {
        let options = naga::back::spv::Options {
            lang_version: (1, 1),
            ..Default::default()
        };
        let pipeline = naga::back::spv::PipelineOptions {
            entry_point: "reference_bindings".to_string(),
            shader_stage: naga::ShaderStage::Compute,
        };
        let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
        crate::helpers::validate_spirv(&words);
    }
}
