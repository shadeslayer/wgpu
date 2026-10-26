use naga::front::cuda::{parse_str, Options};

use super::helpers::{validate, validate_with_options};

const FIXTURE: &str = include_str!("../../in/cuda/pointer-parameters.cu");

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty(), "{source}: {error}");
    assert!(source.get(range).is_some(), "{source}: {error}");
    error
}

fn root_expression(
    function: &naga::Function,
    mut expression: naga::Handle<naga::Expression>,
) -> naga::Handle<naga::Expression> {
    loop {
        match function.expressions[expression] {
            naga::Expression::Access { base, .. } | naga::Expression::AccessIndex { base, .. } => {
                expression = base
            }
            _ => return expression,
        }
    }
}

#[test]
fn cuda_pointer_parameters_bind_scalar_pointees_and_const_views() {
    for (name, initial, replacement) in [
        ("bool", "false", "true"),
        ("int", "-3", "7"),
        ("unsigned", "0x80000000u", "9u"),
        ("float", "1.5f", "2.0f"),
    ] {
        validate(&format!("using Value = {name}; using Frozen = const Value;
            __device__ void replace(Value *const destination, Frozen *source) {{ *destination = *source; }}
            __global__ void kernel(unsigned *out) {{ Value values[2]{{{initial}, {replacement}}}; replace(values, values + 1); out[0] = unsigned(values[0]); }}"));
    }
}

#[test]
fn cuda_pointer_parameters_bind_all_vector_pointees_and_components() {
    for scalar in ["int", "uint", "float"] {
        for width in 2..=4 {
            let name = format!("{scalar}{width}");
            let initial = core::iter::repeat_n("1", width)
                .collect::<Vec<_>>()
                .join(", ");
            let replacement = core::iter::repeat_n("2", width)
                .collect::<Vec<_>>()
                .join(", ");
            let component = match width {
                2 => "y",
                3 => "z",
                _ => "w",
            };
            let element = if scalar == "uint" { "unsigned" } else { scalar };
            validate(&format!("__device__ void replace({name} *destination, const {name} *source, {element} *field) {{ *destination = source[1]; *field += destination[0].x; }}
                __global__ void kernel(unsigned *out) {{ {name} values[2]{{{{{initial}}}, {{{replacement}}}}}; replace(values, values, &values[0].{component}); out[0] = unsigned(values[0].{component}); }}"));
        }
    }
}

#[test]
fn cuda_pointer_parameters_bind_nominal_records_and_array_members() {
    validate(
        r#"struct Leaf { int value; }; struct Record { Leaf leaves[2][2]; float2 vectors[2]; };
        using Alias = Record; typedef Alias Copy;
        __device__ Copy update(Alias *records, const Copy *view) { records[1].leaves[1][0].value += view[0].leaves[0][1].value; return records[1]; }
        __device__ void leaf(Leaf *value) { value[1].value += value[0].value; }
        __global__ void kernel(int *out) { Record records[2]{}; Copy result = update(records, records); leaf(&records[1].leaves[1][0]); out[0] = result.leaves[1][0].value; }"#,
    );
    rejected("struct First { int value; }; struct Second { int value; }; __device__ void write(First *value) {} __global__ void kernel() { Second value{}; write(&value); }");
    rejected("struct Record { int values[2]; }; __device__ void write(int *value) {} __global__ void kernel() { Record value{}; write(&value); }");
}

#[test]
fn cuda_pointer_parameters_forward_bare_kernel_resources_and_local_arrays() {
    let (module, _) = validate(
        r#"__device__ void copy(const int *source, int *destination) { destination[0] = source[1]; }
        __device__ void forward(const int *source, int *destination) { copy(source, destination); }
        __global__ void kernel(const int *input, int factor, int *out) { int local[3]{1, 2, 3}; forward(input, out); copy(local, out + 1); out[2] = factor; }"#,
    );
    assert_eq!(module.global_variables.len(), 3);
    for (index, (_, variable)) in module.global_variables.iter().enumerate() {
        assert_eq!(
            variable.binding,
            Some(naga::ResourceBinding {
                group: 0,
                binding: index as u32
            })
        );
    }
    validate(
        r#"__device__ void copy(const int *source, int *destination) { destination[0] = source[0]; }
        __global__ void kernel(int *out) { __shared__ int shared[4]; shared[threadIdx.x] = int(threadIdx.x); __syncthreads(); copy(shared, out); }"#,
    );
}

#[test]
fn cuda_pointer_parameters_preserve_identical_and_overlapping_live_aliases() {
    validate(
        r#"__device__ void combine(int *first, int *second, const int *view) { first[0] = 5; second[0] += view[0]; first[0] += second[0]; }
        __global__ void kernel(int *out) { int values[4]{1, 2, 3, 4}; combine(values, values, values); combine(values, values + 1, values + 1); out[0] = values[0]; out[1] = values[1]; }"#,
    );
    validate(
        r#"struct Record { int first; int second; };
        __device__ void overlap(Record *whole, int *field, const Record *view) { whole[0].first = 3; *field += view[0].first; whole[0].second = *field; }
        __global__ void kernel(int *out) { Record value{1, 2}; overlap(&value, &value.first, &value); out[0] = value.second; }"#,
    );
}

#[test]
fn cuda_pointer_parameters_combine_negative_indices_before_physical_access() {
    validate(
        r#"__device__ int previous(const int *value, int index) { return value[index] + value[-2]; }
        __global__ void kernel(int *out, const int *input, unsigned base) { int local[4]{2, 3, 5, 7}; out[0] = previous(&local[3], -1); out[1] = previous(&input[base + 2u], -1); }"#,
    );
    validate(
        r#"struct Record { int values[3]; };
        __device__ int previous(const int *value) { return value[-1]; }
        __global__ void kernel(int *out) { Record value{{2, 3, 5}}; out[0] = previous(&value.values[2]); }"#,
    );
}

#[test]
fn cuda_pointer_parameters_form_one_past_addresses_without_dereferencing() {
    validate(
        r#"__device__ int previous(const int *value) { return value[-1]; }
        __device__ void ignore(const int *value) {}
        __global__ void kernel(int *out) { int values[3]{2, 3, 5}; out[0] = previous(&values[3]); out[1] = previous(values + 3); ignore(&values[3]); ignore(&*(&values[3])); }"#,
    );
    validate(
        r#"struct Record { int values[3]; };
        __device__ int previous(const int *value) { return value[-1]; }
        __global__ void kernel(int *out, int end) { Record value{{2, 3, 5}}; out[0] = previous(&value.values[end]); }"#,
    );
}

#[test]
fn cuda_pointer_parameters_support_single_object_and_field_addresses() {
    validate(
        r#"struct Record { int value; int2 pair; };
        __device__ void write(int *value) { value[0] = 7; }
        __device__ int read(const int *value) { return *value; }
        __global__ void kernel(int *out) { int scalar = 1; Record record{2, {3, 4}}; write(&scalar); write(&record.value); write(&record.pair.y); out[0] = read(&scalar) + read(&record.value) + read(&record.pair.y); }"#,
    );
}

#[test]
fn cuda_pointer_parameters_recover_array_origins_through_local_references() {
    validate(
        r#"__device__ void neighbors(int *value) { value[-1] += 3; value[1] += 7; }
        __global__ void kernel(int *out, unsigned choose) { int first[4]{1, 2, 3, 4}, second[4]{5, 6, 7, 8}; int &saved = first[2]; neighbors(&saved); int &selected = choose ? first[2] : second[2]; neighbors(&selected); out[0] = first[1] + first[3] + second[1] + second[3]; }"#,
    );
    validate(
        r#"struct Record { int values[3]; };
        __device__ int previous(const int *value) { return value[-1]; }
        __global__ void kernel(int *out) { Record record{{2, 3, 5}}; const int &saved = record.values[2]; out[0] = previous(&saved); }"#,
    );
}

#[test]
fn cuda_pointer_parameters_preserve_nested_pointer_reference_forwarding() {
    validate(
        r#"__device__ void pointer_step(int *value) { value[1] += 7; }
        __device__ void reference_step(int &value) { pointer_step(&value); }
        __device__ void pointer_again(int *value) { reference_step(value[0]); }
        __device__ void projected(int &first, int &second, bool choose) { int &selected = choose ? first : second; pointer_again(&selected); }
        __global__ void kernel(int *out, unsigned choose) { int first[3]{1, 2, 3}, second[3]{4, 5, 6}; projected(first[1], second[1], bool(choose)); out[0] = first[2]; out[1] = second[2]; }"#,
    );
    validate(
        r#"struct Record { int values[3]; };
        __device__ void pointer_step(int *value) { value[1] += 7; }
        __device__ void reference_step(Record &value) { int &saved = value.values[1]; pointer_step(&saved); }
        __device__ void forward(Record *value) { reference_step(value[0]); }
        __global__ void kernel(int *out) { Record value{{1, 2, 3}}; forward(&value); out[0] = value.values[2]; }"#,
    );
}

#[test]
fn cuda_pointer_parameters_lower_indirection_and_address_value_categories() {
    let (module, _) = validate(
        r#"__device__ void ignore(const int *value) {}
        __device__ void initialize(int *value) { *value = 23; }
        __device__ void reference_write(int &value) { value = 29; }
        __device__ void pointer_forward(int *value) { reference_write(*value); }
        __global__ void kernel(int *out) { int fresh; ignore(&fresh); initialize(&fresh); pointer_forward(&*(&fresh)); out[0] = fresh; }"#,
    );
    let function = &module.entry_points[0].function;
    let fresh = function
        .local_variables
        .iter()
        .find(|(_, variable)| variable.name.as_deref() == Some("fresh"))
        .unwrap()
        .0;
    let is_fresh = |pointer| matches!(function.expressions[root_expression(function, pointer)], naga::Expression::LocalVariable(local) if local == fresh);
    let mut written = false;
    naga::proc::any_statement(&function.body, &mut |statement| {
        match *statement {
            naga::Statement::Store { pointer, .. } if is_fresh(pointer) => written = true,
            naga::Statement::Emit(ref expressions) => {
                for expression in expressions.clone() {
                    if matches!(function.expressions[expression], naga::Expression::Load { pointer } if is_fresh(pointer))
                    {
                        assert!(
                            written,
                            "binding an output-only pointer must not load its referent"
                        );
                    }
                }
            }
            _ => {}
        }
        false
    });
    assert!(written);
    validate(
        r#"__device__ void write(int *value) { *value = 7; }
        __global__ void kernel(int *out) { int first = 1, second = 2; write(&(first)); write(&(0, first)); write(&(first = second)); write(&++first); out[0] = first; }"#,
    );
}

#[test]
fn cuda_pointer_parameters_lower_borrowed_pointer_offsets_and_commuted_subscripts() {
    validate(
        r#"__device__ int read(const int *value) { return *(value + 1) + 2[value] + *(value - -1) + (+value)[0]; }
        __device__ int captured(const int *value, int index) { return index[(index = 1, value)]; }
        __global__ void kernel(int *out) { int values[4]{2, 3, 5, 7}; out[0] = read(values); out[1] = read(1 + values - 1); out[2] = captured(values, 0); }"#,
    );
    validate(
        r#"struct Record { int value; int samples[2]; };
        __global__ void standalone(unsigned *out) { int values[4]{3, 5, 7, 11}; *(&values[4] - 2) += 1; out[0] = unsigned(*(&values[4] - 1)); out[1] = unsigned((1 + values)[1]); out[2] = unsigned(2[values]); int3 point{2, 4, 6}; (*(&point)).x += 3; out[3] = unsigned((*(&point)).x); int &alias = values[1]; *(&alias + 1) += 2; out[4] = unsigned(*(&alias + 1)); out[5] = unsigned(values[1]); Record record{9, {12, 15}}; (*(&record)).samples[0] += 3; out[6] = unsigned((*(&record)).samples[0]); out[7] = unsigned(*(&record.samples[2] - 1)); }"#,
    );
}

#[test]
fn cuda_pointer_parameters_promote_integral_indices_and_reject_nonintegral_indices() {
    validate(
        r#"__device__ int read(const int *value, bool index) { return value[index] + index[value] + *(value + index); }
        __global__ void kernel(int *out) { int values[2]{2, 3}; out[0] = read(values, true); }"#,
    );
    for index in ["1.0f", "make_int2(1, 2)", "Record{1}", "nothing()"] {
        rejected(&format!("struct Record {{ int value; }}; __device__ void nothing() {{}} __device__ int read(const int *value) {{ return value[{index}]; }} __global__ void kernel() {{}}"));
    }
    rejected(
        "__device__ int read(const int *value) { return 1.0f[value]; } __global__ void kernel() {}",
    );
}

#[test]
fn cuda_pointer_parameters_materialize_each_argument_address_once() {
    validate(
        r#"__device__ int inspect(int *value, int ignored) { ++value[0]; return value[0]; }
        __device__ void pair(int *first, int *second) { first[0]++; second[0]++; }
        __global__ void kernel(int *out, unsigned choose) { int values[4]{1, 2, 3, 4}; int index = 0; pair(&values[index++], values + index++); out[0] = inspect(&(choose ? values[0] : values[1]), (choose = !choose)); out[1] = inspect(values + index++, (index = 0)); out[2] = index; }"#,
    );
}

#[test]
fn cuda_pointer_parameters_snapshot_value_arguments_before_later_effects() {
    validate(
        r#"struct Record { int value; };
        __device__ int live(const int *value, int ignored) { return value[0]; }
        __device__ int copied(int value, int *destination) { destination[0] = 9; return value; }
        __device__ int record(Record copy, Record *destination) { destination[0].value = 9; return copy.value; }
        __global__ void kernel(int *out) { int value = 3; out[0] = live(&value, (value = 9)); value = 3; out[1] = copied(value, &value); Record source{3}; out[2] = record(source, &source); out[3] = source.value; }"#,
    );
}

#[test]
fn cuda_pointer_parameters_select_conditional_views_and_common_constness() {
    validate(
        r#"__device__ int read(const int *value) { return value[0]; }
        __device__ void write(int *value) { value[0] = 7; }
        __global__ void kernel(int *out, unsigned choose) { int first[2]{1, 2}, second[3]{3, 4, 5}; const int fixed[4]{6, 7, 8, 9}; write(choose ? first : second); out[0] = read(choose ? first : fixed); out[1] = read(choose ? fixed : second); int count = 0; out[2] = read((++count, (choose ? first : second))); out[3] = count; }"#,
    );
    rejected("__device__ void write(int *value) {} __global__ void kernel(unsigned choose) { int first[2]{}; const int second[3]{}; write(choose ? first : second); }");
    rejected("__device__ void read(const int *value) {} __global__ void kernel(unsigned choose) { int first[2]{}; float second[3]{}; read(choose ? first : second); }");
    rejected("__device__ void read(const int *value) {} __global__ void kernel() { int values[2]{}; read(&values); }");
    validate("__device__ int read(const int *value) { return value[-1]; } __global__ void kernel(int *out) { int matrix[2][3]{{1, 2, 3}, {4, 5, 6}}; out[0] = read(&matrix[1][3]); out[1] = read(matrix[1] + 3); }");
}

#[test]
fn cuda_pointer_parameters_rank_pointer_qualification_and_source_types() {
    validate(
        r#"__device__ int rank(int *value) { return 1; }
        __device__ int rank(const int *value) { return 2; }
        __device__ int rank(int &value) { return 3; }
        __device__ int rank(float *value) { return 4; }
        __global__ void kernel(int *out) { int value = 3; const int fixed = 4; float sample = 5.0f; out[0] = rank(&value); out[1] = rank(&fixed); out[2] = rank(value); out[3] = rank(&sample); }"#,
    );
    validate(
        r#"__device__ int rank(int *value, float weight) { return 1; }
        __device__ int rank(const int *value, int weight) { return 2; }
        __global__ void kernel(int *out) { int value = 3; out[0] = rank(&value, 1.0f); }"#,
    );
    rejected("__device__ int rank(int *value, float weight) { return 1; } __device__ int rank(const int *value, int weight) { return 2; } __global__ void kernel() { int value = 3; rank(&value, 1); }");
    rejected("__device__ int rank(float *value) { return 1; } __global__ void kernel() { int value = 3; rank(&value); }");
    rejected("__device__ int rank(int value) { return 1; } __global__ void kernel() { int value = 3; rank(&value); }");
}

#[test]
fn cuda_pointer_parameters_merge_canonical_prototypes_and_source_qualifiers() {
    validate(
        r#"using Value = int; using Frozen = const Value;
        __device__ int read(Frozen *); __device__ int read(const int *const prototype_name);
        __device__ void write(Value *const); __device__ void write(int values[]); __device__ void write(int values[3]);
        __device__ void write(int [2]); __device__ void write(int *);
        __global__ void kernel(int *out) { int values[1]{3}; write(values); out[0] = read(values); }
        __device__ void write(Value values[2]) { values[0] += 1; }
        __device__ int read(int const *value) { return value[0]; }"#,
    );
    validate("constexpr int Count = 3; __device__ int read(const int values[Count]); __device__ int read(const int *value) { return value[0]; } __global__ void kernel(int *out) { int values[1]{7}; out[0] = read(values); }");
    for bound in ["0", "-1", "1.0f", "runtime"] {
        rejected(&format!("__device__ void write(int runtime, int values[{bound}]); __global__ void kernel() {{}}"));
    }
    rejected("__device__ int read(int *value); __device__ float read(int *value); __global__ void kernel() {}");
    rejected("__device__ int read(int *value) { return value[0]; } __device__ int read(int *const value) { return value[0]; } __global__ void kernel() {}");
}

#[test]
fn cuda_pointer_parameters_bind_only_visible_candidates_and_preserve_name_scopes() {
    validate(
        r#"__device__ int rank(const int *value) { return 1; }
        __device__ int before(int *value) { return rank(value); }
        __device__ int rank(int *value) { return 2; }
        __device__ int late(int *);
        __global__ void kernel(int *out) { int value = 3; out[0] = before(&value); out[1] = rank(&value); out[2] = late(&value); }
        __device__ int late(int *value) { return value[0]; }"#,
    );
    validate("__device__ int __mul24(int *value) { return value[0]; } __global__ void kernel(int *out) { int value = 3; out[0] = __mul24(&value); out[1] = __mul24(2, 3); }");
    rejected("__global__ void kernel() { int value = 3; later(&value); } __device__ int later(int *value) { return value[0]; }");
    rejected("__device__ int read(int *value) { return value[0]; } __global__ void kernel() { int value = 3; int read = 0; read(&value); }");
}

#[test]
fn cuda_pointer_parameters_support_source_operator_functions_with_real_class_parameters() {
    validate(
        r#"struct Box { int value; };
        __device__ int operator+(int *first, Box second) { return first[0] + second.value; }
        __device__ int read(const Box *value) { return value[0].value; }
        __global__ void kernel(int *out) { int values[2]{3, 4}; Box value{5}; out[0] = values + value; out[1] = operator+(values, value); out[2] = read(&value); }"#,
    );
    validate(
        r#"struct Box { int value; };
        __device__ int operator&(Box &value) { return ++value.value; }
        __device__ int read(const Box *value) { return value[0].value; }
        __global__ void kernel(int *out) { Box value{3}; const Box fixed{4}; out[0] = &value; out[1] = operator&(value); out[2] = read(&fixed); }"#,
    );
    rejected("struct Box { int value; }; __device__ int operator&(Box &value) { return value.value; } __global__ void kernel() { const Box value{3}; operator&(value); }");
    rejected("struct Box { int value; }; __device__ int operator&(Box &value) { return value.value; } __device__ int read(const Box *value) { return value[0].value; } __global__ void kernel() { Box value{3}; read(&value); }");
    rejected("struct Box { int value; }; __device__ int operator&(Box value) { return value.value; } __device__ int operator&(const Box &value) { return value.value; } __device__ int read(const Box *value) { return value[0].value; } __global__ void kernel() { Box value{3}; read(&value); }");
    rejected("struct Box { int value; }; __device__ Box operator*(Box *value) { return value[0]; } __global__ void kernel() {}");
    rejected("struct Box { int value; }; __device__ int operator+(Box *first, Box *second) { return 1; } __global__ void kernel() {}");
}

#[test]
fn cuda_pointer_parameters_reject_const_drops_and_invalid_argument_categories() {
    validate("__device__ int read(const int *value) { return value[0]; } __global__ void kernel(int *out) { const int value = 3; out[0] = read(&value); }");
    for source in [
        "__device__ void write(const int *value) { value[0] = 7; } __global__ void kernel() {}",
        "__device__ void write(int *value) {} __global__ void kernel(const int *input) { write(input); }",
        "__device__ void write(int *value) {} __global__ void kernel() { const int value = 3; write(&value); }",
        "__device__ void write(int *value) {} __global__ void kernel() { write(3); }",
        "__device__ void write(float *value) {} __global__ void kernel() { int value = 3; write(&value); }",
        "__device__ void read(const int *value) {} __global__ void kernel() { read(&3); }",
        "__device__ void read(const int *value) {} __global__ void kernel() { int value = 3; read(&int(value)); }",
        "constexpr int Global = 3; __device__ void read(const int *value) {} __global__ void kernel() { read(&Global); }",
        "__device__ void read(bool *value) {} __global__ void kernel(bool *input) { read(input); }",
        "__device__ void read(bool *value) {} __global__ void kernel() { __shared__ bool value; read(&value); }",
    ] { rejected(source); }
}

#[test]
fn cuda_pointer_parameters_bind_shared_and_storage_roots_with_existing_barrier_rules() {
    let (module, _) = validate_with_options(
        r#"struct Record { int3 point; int values[2]; };
        __device__ void update(Record *value, const Record *view) { value[0].point.x += view[0].values[1]; }
        __global__ void kernel(const Record *input, Record *out) { unsigned lane = threadIdx.x; __shared__ Record shared[4]; shared[lane] = input[lane]; __syncthreads(); update(&shared[lane], &shared[lane]); __syncthreads(); update(&out[lane], &shared[lane]); }"#,
        &Options {
            workgroup_size: [4, 1, 1],
        },
    );
    let input = module
        .global_variables
        .iter()
        .find(|(_, variable)| variable.name.as_deref() == Some("input"))
        .unwrap()
        .1;
    assert_eq!(
        input.space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD
        }
    );
    let naga::TypeInner::Array { stride, .. } = module.types[input.ty].inner else {
        panic!("expected the original packed record buffer");
    };
    assert_eq!(stride, 20);
    rejected("__device__ void wait(int *value) { __syncthreads(); } __global__ void kernel() {}");
    rejected(
        "__device__ void update(int *value) { __shared__ int shared; } __global__ void kernel() {}",
    );
    rejected("__device__ void update(int *value) {} __global__ void kernel(int *out) { if (threadIdx.x == 0u) __syncthreads(); update(out); }");
}

#[test]
fn cuda_pointer_parameters_return_supported_values_and_isolate_early_returns() {
    validate(
        r#"struct Record { int value; };
        __device__ Record record(int *value) { ++*value; return Record{*value}; }
        __device__ int2 vector(int *value) { *value += 2; return int2{*value, *value + 1}; }
        __device__ int step(int *value, int mode) {
            for (int iteration = 0; iteration < 3; ++iteration) { if (mode == 0) { ++*value; return *value; } if (iteration == 1) continue; *value += 2; }
            switch (mode) { case 1: ++*value; break; default: *value += 3; } return *value;
        }
        __device__ void advance(int *value) { ++*value; }
        __device__ bool condition(int *value) { if (*value > 3) return false; ++*value; return true; }
        __global__ void kernel(int *out) { int value = 0; Record result = record(&value); int2 pair = vector(&value); int total = 0; for (int iteration = 0; iteration < 3; ++iteration) { total += step(&value, iteration); total += 10; } int cursor = 0; for (; cursor < 3; advance(&cursor)) { ++total; } do { ++total; } while (condition(&cursor)); out[0] = total + result.value + pair.x; }"#,
    );
}

#[test]
fn cuda_pointer_parameters_reset_local_state_across_dynamic_invocations() {
    validate(
        r#"struct Record { int value; int samples[2]; };
        __device__ int once(int *value, bool stop) { int count = 1; int samples[2]{2, 3}; Record record{4, {5, 6}}; ++value[0]; if (stop) return count + samples[1] + record.samples[0]; count += 7; return count + value[0]; }
        __global__ void kernel(int *out) { int values[4]{1, 2, 3, 4}; int sum = 0; for (int iteration = 0; iteration < 4; ++iteration) { sum += once(&values[iteration], iteration % 2 == 0); } out[0] = sum; out[1] = values[3]; }"#,
    );
}

#[test]
fn cuda_pointer_parameters_preserve_runtime_calls_and_constant_expression_rules() {
    validate(
        r#"__device__ int read(const int *value) { return value[0]; }
        __global__ void kernel(int *out) { int values[2]{1, 2}; constexpr int Count = true ? 4 : read(values); constexpr bool Skip = false && read(values); int storage[Count]{}; out[0] = storage[3] + int(Skip); }"#,
    );
    rejected("__device__ int read(const int *value) { return value[0]; } __global__ void kernel() { int values[2]{}; constexpr int Count = read(values); }");
    validate("__device__ int count(const int *value) { constexpr int Count = (value, 4); return Count; } __global__ void kernel(int *out) { out[0] = count(out); }");
    for expression in [
        "(value[0], 4)",
        "(*value, 4)",
        "(+value, 4)",
        "(value + 0, 4)",
    ] {
        rejected(&format!("__device__ int read(const int *value) {{ constexpr int Count = {expression}; return Count; }} __global__ void kernel() {{ __shared__ int shared[2]; read(shared); }}"));
    }
}

#[test]
fn cuda_pointer_parameters_validate_unused_bodies_before_template_removal() {
    validate(
        "__device__ int read(const int *value) { return value[0]; } __global__ void kernel() {}",
    );
    for definition in [
        "__device__ int invalid(int *value) { if (value[0]) return 3; }",
        "__device__ int invalid(int *value) { return value[0].missing; }",
        "__device__ int invalid(int *value) { return value[1.0f]; }",
        "__device__ void invalid(const int *value) { *value = 3; }",
        "__device__ int invalid(int *value) { return missing(value); }",
    ] {
        rejected(&format!("{definition} __global__ void kernel() {{}}"));
    }
}

#[test]
fn cuda_pointer_parameters_reject_missing_definitions_and_call_cycles() {
    validate("__device__ int unused(int *); __device__ int read(const int *value) { return value[0]; } __global__ void kernel() {}");
    for source in [
        "__device__ int missing(int *); __device__ int unused(int *value) { return missing(value); } __global__ void kernel() {}",
        "__device__ int missing(int *); __global__ void kernel() { int value = 3; if (false) missing(&value); }",
        "__device__ int direct(int *value) { return direct(value); } __global__ void kernel() {}",
        "__device__ int first(int *); __device__ int second(int *); __device__ int first(int *value) { return second(value); } __device__ int second(int *value) { return first(value); } __global__ void kernel() {}",
        "__device__ int first(int *); __device__ int second(int &); __device__ int first(int *value) { return second(value[0]); } __device__ int second(int &value) { return first(&value); } __global__ void kernel() {}",
    ] { rejected(source); }
}

#[test]
fn cuda_pointer_parameters_reject_escaping_and_unsupported_pointer_storage() {
    validate("__device__ int read(const int *value) { return value[0]; } __global__ void kernel(int *out) { int value = 3; out[0] = read(&value); }");
    validate("__global__ void kernel() { int value = 3; int *local = &value; }");
    for source in [
        "struct Record { int *value; }; __global__ void kernel() {}",
        "__device__ void change(int *value) { value = value + 1; } __global__ void kernel() {}",
        "__device__ void change(int *value) { ++value; } __global__ void kernel() {}",
        "__device__ void change(int *value) { value--; } __global__ void kernel() {}",
        "__device__ int *read(int *value) { return value; } __global__ void kernel() {}",
        "__device__ void read(int *value) {} __global__ void kernel() { read(nullptr); }",
        "__device__ void read(int *value) {} __global__ void kernel() { read(0); }",
        "__device__ void read(void *value) {} __global__ void kernel() {}",
        "__device__ void read(int **value) {} __global__ void kernel() {}",
        "__device__ void read(int *&value) {} __global__ void kernel() {}",
        "__device__ void read(int (*value)[2]) {} __global__ void kernel() {}",
        "__device__ void read(int values[2][2]) {} __global__ void kernel() {}",
        "__device__ int read(int *first, int *second) { return first == second; } __global__ void kernel() {}",
        "__device__ int read(int *first, int *second) { return first - second; } __global__ void kernel() {}",
        "__device__ void read(int *first, int *second) { first + second; } __global__ void kernel() {}",
        "__device__ void read(int *value) { 1 - value; } __global__ void kernel() {}",
        "__device__ void read(int *value) { static_cast<unsigned>(value); } __global__ void kernel() {}",
        "__global__ void kernel(int *input) { (float *)input; }",
    ] { rejected(source); }
}

#[test]
fn cuda_pointer_parameters_report_malformed_declarators_and_original_spans() {
    validate("__device__ void write(int *value) {} __global__ void kernel() { int value = 3; write(&value); }");
    for declaration in [
        "__device__ void write(int **value) {}",
        "__device__ void write(int *&value) {}",
        "__device__ void write(int &&value) {}",
        "__device__ void write(int *value = 0) {}",
        "__device__ void write(int *const const value) {}",
        "__device__ void write(int values[][2]) {}",
    ] {
        rejected(&format!(
            "/* café 漢 */ {declaration} __global__ void kernel() {{}}"
        ));
    }
    let source = "#define BAD 3\n/* λ */ __device__ void write(int *value) {} __global__ void kernel() { write(BAD); }";
    let error = rejected(source);
    assert_eq!(&source[error.span.to_range().unwrap()], "BAD");
    validate("__device__ void write(int *value) { value[0] = 3; } __global__ void kernel(int *out) { write(out); }");
}

#[test]
fn cuda_pointer_parameters_bound_expansion_and_address_origin_work() {
    let mut linear = String::from("__device__ void step0(int *value) { value[0] += 1; }");
    for level in 1..48 {
        linear.push_str(&format!(
            "__device__ void step{level}(int *value) {{ step{}(value); }}",
            level - 1
        ));
    }
    linear.push_str(
        "__global__ void kernel(int *out) { int value = 0; step47(&value); out[0] = value; }",
    );
    let (module, info) = validate(&linear);
    #[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    let _ = (module, info);

    let mut doubling = String::from("__device__ void step0(int *value) { value[0] += 1; }");
    for level in 1..24 {
        doubling.push_str(&format!(
            "__device__ void step{level}(int *value) {{ step{}(value); step{}(value); }}",
            level - 1,
            level - 1
        ));
    }
    doubling.push_str("__global__ void kernel(int *out) { step23(out); }");
    let error = rejected(&doubling);
    assert!(error.message.contains("limit") || error.message.contains("depth"));

    let name = "identifier".to_owned() + &"x".repeat(131_072);
    let mut payload = format!("__device__ void leaf(int *value) {{ int {name} = value[0]; value[0] = {name} + 1; }} __global__ void kernel() {{ int value = 0;");
    payload.push_str(&"leaf(&value);".repeat(1024));
    payload.push('}');
    let error = rejected(&payload);
    assert!(error.message.contains("limit") || error.message.contains("depth"));

    for (length, accepted) in [(16, true), (96, false)] {
        let expression = "value".to_owned() + &" + 0".repeat(length);
        let source = format!("__device__ int read(int *value) {{ return *({expression}); }} __global__ void kernel(int *out) {{ out[0] = read(out); }}");
        if accepted {
            validate(&source);
        } else {
            let error = rejected(&source);
            assert!(error.message.contains("limit") || error.message.contains("depth"));
        }
    }
}

#[test]
fn cuda_pointer_parameters_roundtrip_aliasing_and_offsets_through_shared_wgsl_validation() {
    let (module, info) = validate(FIXTURE);
    assert!(info
        .get_entry_point(0)
        .available_stages
        .contains(naga::valid::ShaderStages::COMPUTE));
    #[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    let _ = (module, info);
}

#[cfg_attr(miri, ignore)]
#[test]
fn cuda_pointer_parameters_validate_canonical_fixture_and_raw_spirv() {
    let (module, info) = validate(FIXTURE);
    #[cfg(spv_out)]
    {
        let options = naga::back::spv::Options {
            lang_version: (1, 1),
            ..Default::default()
        };
        let pipeline = naga::back::spv::PipelineOptions {
            entry_point: String::from("pointer_parameters"),
            shader_stage: naga::ShaderStage::Compute,
        };
        let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
        crate::helpers::validate_spirv(&words);
    }
    let _ = (module, info);
}
