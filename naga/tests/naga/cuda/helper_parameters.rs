use naga::front::cuda::{parse_str, Options};

use super::helpers::{validate, validate_with_options};

const FIXTURE: &str = include_str!("../../in/cuda/helper-parameters.cu");

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty() || range.start == source.len(), "{error}");
    assert!(source.get(range).is_some(), "{error}");
    error
}

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
            naga::Statement::Switch { cases, .. } => {
                for case in cases {
                    result.extend(statements(&case.body));
                }
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
fn cuda_helper_parameters_bind_scalar_parameters_and_const_views() {
    for (ty, initial, replacement) in [
        ("bool", "false", "true"),
        ("int", "-3", "7"),
        ("unsigned", "0x80000000u", "9u"),
        ("float", "1.5f", "2.0f"),
    ] {
        validate(&format!("using Value = {ty}; using Frozen = const Value;
            __device__ void replace(Value &first, Frozen &second) {{ first = second; }}
            __global__ void kernel(unsigned *out) {{ Value first = {initial}; const Value second = {replacement}; replace(first, second); out[0] = unsigned(first); }}"));
    }
}

#[test]
fn cuda_helper_parameters_bind_all_vector_parameters_and_components() {
    for scalar in ["int", "uint", "float"] {
        for width in 2..=4 {
            let ty = format!("{scalar}{width}");
            let values = core::iter::repeat_n("1", width)
                .collect::<Vec<_>>()
                .join(", ");
            let component = match width {
                2 => "y",
                3 => "z",
                _ => "w",
            };
            let scalar_ty = if scalar == "uint" { "unsigned" } else { scalar };
            validate(&format!("__device__ void update({ty} &first, {scalar_ty} &second, const {ty} &view) {{ first.x = 3; second += view.x; }}
                __global__ void kernel(unsigned *out) {{ {ty} value{{{values}}}; update(value, value.{component}, value); out[0] = unsigned(value.{component}); }}"));
        }
    }
}

#[test]
fn cuda_helper_parameters_bind_nominal_records_and_array_members() {
    validate("struct Leaf { int value; }; struct Record { Leaf leaves[2][2]; float2 vectors[2]; };
        using Alias = Record; typedef Alias Copy;
        __device__ Copy update(Alias &record, int &field, const Copy &view) { record.leaves[1][0].value = 3; field += view.leaves[1][0].value; return record; }
        __global__ void kernel(int *out) { Record value{}; Copy copy = update(value, value.leaves[1][0].value, value); out[0] = copy.leaves[1][0].value; }");
    rejected("struct First { int value; }; struct Second { int value; }; __device__ void update(First &value) {} __global__ void kernel() { Second value{}; update(value); }");
    rejected("struct Record { int values[2]; }; __device__ void update(int &value) {} __global__ void kernel() { Record value{}; update(value.values); }");
}

#[test]
fn cuda_helper_parameters_preserve_identical_mutable_and_const_aliases() {
    validate("__device__ void combine(int &first, int &second) { first = 5; second += 2; first += second; }
        __device__ void live(int &write, const int &read) { write = 7; write += read; }
        __global__ void kernel(int *out) { int same = 1; combine(same, same); out[0] = same; int first = 1, second = 2; combine(first, second); out[1] = first; out[2] = second; live(first, first); out[3] = first; }");
}

#[test]
fn cuda_helper_parameters_preserve_whole_object_and_subobject_overlap() {
    validate("struct Record { int first; int second; int values[2]; };
        __device__ void overlap(Record &whole, int &part, const Record &view) { whole.first = 5; part += 2; whole.second = view.first; }
        __device__ void vector_overlap(float2 &whole, float &part) { whole.x = 5.0f; part += 2.0f; whole.y = part; }
        __global__ void kernel(unsigned *out) { Record value{1, 2, {3, 4}}; overlap(value, value.first, value); float2 vector{1.0f, 2.0f}; vector_overlap(vector, vector.x); out[0] = unsigned(value.second); out[1] = __float_as_uint(vector.y); }");
}

#[test]
fn cuda_helper_parameters_forward_existing_frozen_local_aliases() {
    validate("struct Record { int x; int y; int values[2]; };
        __device__ void leaf(int &first, int &second) { first += 1; second += 2; }
        __device__ void project(Record &first, Record &second, bool choose, int &result) {
            Record &record = choose ? first : second; int &x = record.x; int &y = record.y;
            choose = !choose; leaf(choose ? x : y, x); int index = 0;
            int &sample = record.values[index++]; leaf(sample, result); result += record.values[0];
        }
        __global__ void kernel(int *out, unsigned first_choice, unsigned second_choice) {
            Record values[3]{{3, 4, {5, 6}}, {7, 8, {9, 10}}, {11, 12, {13, 14}}};
            Record &first = first_choice ? values[0] : values[1]; Record &second = second_choice ? values[1] : values[2];
            int result = 0; project(first, second, bool(first_choice), result); out[0] = result;
        }");
}

#[test]
fn cuda_helper_parameters_bind_output_only_arguments_without_loading() {
    let (module, _) = validate("__device__ void ignore(const int &value) {} __device__ void ignore(float value) {}
        __device__ void initialize(int &value) { value = 23; } __device__ void initialize(float value) {}
        __global__ void kernel(int *out) { int fresh; ignore(fresh); initialize(fresh); out[0] = fresh; }");
    let function = &module.entry_points[0].function;
    let fresh = function
        .local_variables
        .iter()
        .find(|(_, local)| local.name.as_deref() == Some("fresh"))
        .unwrap()
        .0;
    let is_fresh = |pointer| matches!(function.expressions[root_expression(function, pointer)], naga::Expression::LocalVariable(local) if local == fresh);
    let mut written = false;
    let mut read_after_write = false;
    for statement in statements(&function.body) {
        match statement {
            naga::Statement::Store { pointer, .. } if is_fresh(*pointer) => written = true,
            naga::Statement::Emit(range) => {
                for expression in range.clone() {
                    if matches!(function.expressions[expression], naga::Expression::Load { pointer } if is_fresh(pointer))
                    {
                        assert!(written, "reference binding read the uninitialized actual");
                        read_after_write = true;
                    }
                }
            }
            _ => {}
        }
    }
    assert!(written && read_after_write);
}

#[test]
fn cuda_helper_parameters_bind_writable_and_readonly_storage() {
    let (module, _) = validate("struct Record { int value; int samples[2]; };
        __device__ void update(Record &record, int &sample, const int &input) { record.value = input; sample += input; }
        __global__ void kernel(const Record *input, Record *output) { update(output[1], output[1].samples[0], input[0].value); }");
    let input = module
        .global_variables
        .iter()
        .find(|(_, value)| {
            value.binding
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
    rejected("__device__ void write(int &value) { value = 7; } __global__ void kernel(const int *input) { write(input[0]); }");
    rejected("__device__ void write(const int &value) { value = 7; } __global__ void kernel() {}");
}

#[test]
fn cuda_helper_parameters_bind_shared_records_under_existing_barrier_rules() {
    validate_with_options("struct Record { int value; int samples[2]; };
        __device__ void update(Record &record, int &sample, const Record &view) { ++record.value; sample += view.value; }
        __global__ void kernel(int *out) { unsigned lane = threadIdx.x; __shared__ Record values[4]; values[lane] = Record{int(lane), {1, 2}}; __syncthreads(); update(values[lane], values[lane].samples[0], values[lane]); __syncthreads(); out[lane] = values[lane].samples[0]; }", &Options { workgroup_size: [4, 1, 1] });
    rejected("__device__ void wait(int &value) { __syncthreads(); ++value; } __global__ void kernel() {}");
    rejected("__device__ void update(int &value) { __shared__ int shared; shared = value; } __global__ void kernel() {}");
}

#[test]
fn cuda_helper_parameters_materialize_const_parameter_temporaries() {
    validate("struct Record { int value; int samples[2]; };
        __device__ int read(const int &value) { const int &alias = value; return alias; }
        __device__ float converted(const float &value) { return value; }
        __device__ int record(const Record &value) { return value.value + value.samples[1]; }
        __device__ int vector(const float2 &value) { return int(value.x + value.y); }
        __global__ void kernel(int *out) { out[0] = read(3); out[1] = read(Record{4, {5, 6}}.value); out[2] = int(converted(7)); out[3] = record(Record{}); out[4] = record(Record{8, {9}}); out[5] = vector(float2{1.0f, 2.0f}); out[6] = read(read(11)); }");
    for argument in ["{}", "{1}", "{1, 2}"] {
        rejected(&format!("__device__ void read(const int &value) {{}} __global__ void kernel() {{ read({argument}); }}"));
    }
}

#[test]
fn cuda_helper_parameters_snapshot_value_arguments_before_later_effects() {
    validate("struct Record { int value; int samples[2]; };
        __device__ int live(const int &value, int ignored) { return value; }
        __device__ int copied(int value, int ignored) { return value; }
        __device__ float converted(const float &value, int ignored) { return value; }
        __device__ int record(Record copy, const Record &view, int ignored) { return copy.value * 10 + view.value; }
        __global__ void kernel(unsigned *out) { int value = 3; out[0] = unsigned(live(value, (value = 9))); value = 3; out[1] = unsigned(copied(value, (value = 9))); value = 3; out[2] = __float_as_uint(converted(value, (value = 9))); Record source{3, {4, 5}}; out[3] = unsigned(record(source, source, (source.value = 9))); }");
}

#[test]
fn cuda_helper_parameters_evaluate_each_argument_address_once() {
    validate("__device__ void update(int &first, int &second, const int &unused) { ++first; second += 2; }
        __global__ void kernel(int *out, unsigned choose) { int values[4]{1, 2, 3, 4}; int index = 0; update(values[index++], choose ? values[index++] : values[++index], (++index, values[0])); out[0] = index; out[1] = values[0]; if (false) { update(values[index++], values[index++], values[index++]); } }");
}

#[test]
fn cuda_helper_parameters_preserve_existing_argument_value_categories() {
    validate("__device__ void write(int &value) { ++value; } __device__ int read(const int &value) { return value; }
        __global__ void kernel(int *out, unsigned choose) { int first = 1, second = 2; write((first)); write((0, first)); write(first = second); write(choose ? first : second); out[0] = read(choose ? first : 3); out[1] = read(int(first)); }");
    for expression in ["3", "value + 0", "int(value)", "true ? value : 3"] {
        rejected(&format!("__device__ void write(int &value) {{}} __global__ void kernel() {{ int value = 1; write({expression}); }}"));
    }
}

#[test]
fn cuda_helper_parameters_reject_invalid_mutable_bindings_and_const_drops() {
    for source in [
        "__device__ void write(float &value) {} __global__ void kernel() { int value = 1; write(value); }",
        "__device__ void write(int &value) {} __global__ void kernel() { const int value = 1; write(value); }",
        "__device__ void write(int &value) {} __device__ void forward(const int &value) { write(value); } __global__ void kernel() {}",
        "__device__ void read(const float2 &value) {} __global__ void kernel() { int2 value{1, 2}; read(value); }",
        "struct First { int value; }; struct Second { int value; }; __device__ void read(const First &value) {} __global__ void kernel() { read(Second{1}); }",
    ] { rejected(source); }
    let error = rejected("constexpr int Global = 3; __device__ void read(const int &value) {} __device__ void read(float value) {} __global__ void kernel() { read(Global); }");
    assert!(error.message.contains("host namespace constant"));
    validate("constexpr int Global = 3; __device__ float read(const float &value) { return value; } __global__ void kernel(float *out) { out[0] = read(Global); out[1] = read(float(Global)); }");
}

#[test]
fn cuda_helper_parameters_rank_mutable_and_const_reference_overloads() {
    validate("__device__ int rank(int &value) { ++value; return 1; } __device__ int rank(const int &value) { return 2; }
        __global__ void kernel(int *out) { int value = 3; const int fixed = 4; out[0] = rank(value); out[1] = rank(fixed); out[2] = rank(5); }");
    for reference in ["int &", "const int &"] {
        for argument in ["value", "fixed"] {
            if reference == "int &" && argument == "fixed" {
                continue;
            }
            let error = rejected(&format!("__device__ int rank(int value) {{ return 1; }} __device__ int rank({reference}value) {{ return 2; }} __global__ void kernel() {{ int value = 3; const int fixed = 4; rank({argument}); }}"));
            assert!(error.message.contains("ambiguous"));
        }
    }
}

#[test]
fn cuda_helper_parameters_retain_scalar_conversion_and_dominance_rules() {
    validate("__device__ int rank(const int &value) { return 1; } __device__ int rank(const float &value) { return 2; }
        __global__ void kernel(int *out) { bool enabled = true; int integer = 3; float real = 4.0f; out[0] = rank(enabled); out[1] = rank(integer); out[2] = rank(real); }");
    let error = rejected("__device__ int rank(const int &first, const float &second) { return 1; } __device__ int rank(const float &first, const int &second) { return 2; } __global__ void kernel() { int first = 1, second = 2; rank(first, second); }");
    assert!(error.message.contains("ambiguous"));
}

#[test]
fn cuda_helper_parameters_merge_reference_prototypes_by_canonical_signature() {
    validate("using Value = int; using Frozen = const Value;
        __device__ int update(Value &, Frozen &); __device__ int update(int &first, const int &second);
        __device__ int update(Value &first, Frozen &second) { first += second; return first; }
        __device__ int copy(int); __device__ int copy(const int value) { return value; }
        __device__ void unused(const int &); __device__ void unused(const int &another);
        __global__ void kernel(int *out) { int value = 3; out[0] = update(value, value); out[1] = copy(value); }");
    for prefix in [
        "__device__ int update(int &); __device__ float update(int &);",
        "__device__ int update(int &); __host__ __device__ int update(int &);",
        "__device__ int update(int &); static __device__ int update(int &);",
        "__forceinline__ __device__ int update(int &); __noinline__ __device__ int update(int &);",
    ] {
        rejected(&format!("{prefix} __global__ void kernel() {{}}"));
    }
    validate("static __noinline__ __device__ void update(int &); __device__ void update(int &value) { ++value; } __global__ void kernel() { int value = 1; update(value); }");
}

#[test]
fn cuda_helper_parameters_bind_only_source_visible_overload_candidates() {
    validate("__device__ int rank(const float &value) { return 1; } __device__ int before() { int value = 3; return rank(value); } __device__ int rank(int &value) { return 2; }
        __device__ int __mul24(float2 &value, float2 other) { value.x += other.x; return int(value.x); }
        __global__ void kernel(int *out) { int value = 3; float2 vector{1.0f, 2.0f}; out[0] = before(); out[1] = rank(value); out[2] = __mul24(vector, vector); }");
    rejected("__global__ void kernel() { int value = 1; later(value); } __device__ void later(int &value) {}");
    rejected("__device__ void update(int &value) {} __global__ void kernel() { int update = 1; update(update); }");
    let error = rejected("__device__ int __mul24(int &first, int &second) { return 3; } __global__ void kernel() { int first = 1, second = 2; __mul24(first, second); }");
    assert!(error.message.contains("ambiguous"));
}

#[test]
fn cuda_helper_parameters_remap_late_and_generated_helper_dependencies() {
    validate("__device__ float calculate(const float &); __device__ unsigned position(unsigned &);
        __global__ void kernel(unsigned *out) { float value = 4.0f; unsigned position_value; out[0] = __float_as_uint(calculate(value)); out[1] = position(position_value); }
        __device__ float calculate(const float &value) { return sqrtf(value) + sinf(0.0f) + cosf(0.0f); }
        __device__ unsigned position(unsigned &value) { value = threadIdx.x + threadIdx.y + threadIdx.z + blockIdx.x + blockIdx.y + blockIdx.z + blockDim.x + blockDim.y + blockDim.z + gridDim.x + gridDim.y + gridDim.z; return value; }");
}

#[test]
fn cuda_helper_parameters_return_all_supported_values_and_void() {
    validate("struct Record { int value; int samples[2]; };
        __device__ Record make_record(int &value) { ++value; return Record{value, {value + 1, value + 2}}; }
        __device__ int2 make_vector(int &value) { value += 2; return int2{value, value + 1}; }
        __device__ void finish(int &value, bool stop) { if (stop) { value = 7; return; } value = 9; }
        __global__ void kernel(int *out) { int value = 1; Record record = make_record(value); int2 vector = make_vector(value); finish(value, true); out[0] = record.samples[1] + vector.y + value; out[1] = (finish(value, false), value); }");
    rejected("__device__ int read(int &value) { if (value) { return value; } } __global__ void kernel() {}");
}

#[test]
fn cuda_helper_parameters_preserve_nested_reference_calls_and_parameter_locals() {
    validate("struct Record { int x; int y; int samples[2]; };
        __device__ void leaf(int &first, const int &second) { first += second; }
        __device__ void project(Record &first, Record &second, bool choose) { Record &whole = choose ? first : second; int &x = whole.x; int &y = whole.y; leaf(choose ? x : y, choose ? y : x); leaf(whole.samples[0], whole.samples[1]); }
        __device__ int copy(int value) { int &local = value; leaf(local, local); return value; }
        __global__ void kernel(int *out, unsigned choose) { Record first{1, 2, {3, 4}}, second{5, 6, {7, 8}}; project(choose ? first : second, choose ? second : first, bool(choose)); out[0] = copy(first.x); out[1] = second.y; }");
}

#[test]
fn cuda_helper_parameters_isolate_early_returns_from_caller_control_flow() {
    validate("__device__ int calculate(int &value, int mode) {
            for (int outer = 0; outer < 3; ++outer) {
                switch (mode) {
                    case 0: { for (int inner = 0; inner < 3; ++inner) { if (inner == 1) { return ++value; } value += 2; } break; }
                    case 1: value += 3;
                    default: if (outer == 1) { return value; } ++value;
                }
                if (outer == 0) { continue; } value += 5;
            }
            return value;
        }
        __device__ bool condition(int &value) { if (value >= 3) { return false; } ++value; return true; }
        __device__ void advance(int &value) { if (value == 1) { ++value; return; } ++value; }
        __global__ void kernel(int *out) { int value = 0, sum = 0; for (int iteration = 0; iteration < 3; ++iteration) { sum += calculate(value, iteration); sum += 10; } int cursor = 0; for (; cursor < 3; advance(cursor)) { ++sum; } do { ++sum; } while (condition(cursor)); out[0] = value; out[1] = sum; }");
    let mut source = String::from("__device__ int many(int &value) {");
    for index in 0..96 {
        source.push_str(&format!(
            "if (value == {index}) {{ return value + {index}; }}"
        ));
    }
    source.push_str(
        "return value; } __global__ void kernel(int *out) { int value = 7; out[0] = many(value); }",
    );
    let (module, info) = validate(&source);
    #[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
    let _ = (module, info);
}

#[test]
fn cuda_helper_parameters_initialize_callee_locals_on_every_dynamic_call() {
    validate("struct Record { int value; int samples[2]; };
        __device__ int once(int &value, bool stop) { int count = 1; int samples[2]{2, 3}; Record record{4, {5, 6}}; const int &temporary = value + 1; ++value; if (stop) { return count + samples[1] + record.samples[0] + temporary; } count += 7; return count + temporary; }
        __global__ void kernel(int *out) { int value = 0, sum = 0; for (int iteration = 0; iteration < 4; ++iteration) { sum += once(value, iteration % 2 == 0); } out[0] = sum; out[1] = value; }");
}

#[test]
fn cuda_helper_parameters_support_reference_parameters_in_operator_functions() {
    for symbol in ["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>"] {
        validate(&format!("struct Box {{ int value; }}; __device__ int operator{symbol}(Box &first, const Box &second) {{ ++first.value; return first.value + second.value; }} __global__ void kernel(int *out) {{ Box value{{3}}; out[0] = value {symbol} value; out[1] = operator{symbol}(value, value); }}"));
    }
    for symbol in ["+", "-", "~", "!", "*", "&"] {
        validate(&format!("struct Box {{ int value; }}; __device__ int operator{symbol}(Box &value) {{ return ++value.value; }} __global__ void kernel(int *out) {{ Box value{{3}}; out[0] = {symbol}value; out[1] = operator{symbol}(value); }}"));
        rejected(&format!("struct Box {{ int value; }}; __device__ int operator{symbol}(Box &value) {{ return value.value; }} __global__ void kernel() {{ {symbol}Box{{3}}; }}"));
    }
}

#[test]
fn cuda_helper_parameters_preserve_eager_logical_operator_reference_semantics() {
    validate("struct Box { int value; }; __device__ int operator&&(Box &first, Box second) { first.value += second.value; return first.value; } __device__ int operator||(Box &first, Box second) { first.value += second.value; return first.value; }
        __global__ void kernel(int *out) { int count = 0; Box value{0}; out[0] = value && Box{++count}; value = Box{1}; out[1] = value || Box{++count}; out[2] = false && (value && Box{++count}); out[3] = true || (value || Box{++count}); out[4] = count; }");
    validate("struct Box { int value; }; __device__ void operator&&(Box &first, const Box &second) { first.value += second.value; } __global__ void kernel() { Box value{3}; value && value; operator&&(value, value); }");
}

#[test]
fn cuda_helper_parameters_bind_shift_lvalues_without_copying_the_referent() {
    validate("struct Alias { int value; }; struct Copy { int value; };
        __device__ int operator<<(Alias &first, int second) { return first.value * 100 + second; }
        __device__ int operator<<(Copy first, int second) { return first.value * 100 + second; }
        __device__ int operator>>(int &first, Alias second) { return first * 100 + second.value; }
        __global__ void kernel(int *out) { Alias alias{3}; Copy copy{3}; out[0] = alias << (alias.value = 7); out[1] = copy << (copy.value = 7); int value = 3; out[2] = value >> (value = 7, Alias{5}); alias = Alias{3}; out[3] = operator<<(alias, (alias.value = 7)); }");
}

#[test]
fn cuda_helper_parameters_preserve_runtime_calls_and_source_constant_rules() {
    for declaration in [
        "constexpr int Count = value;",
        "constexpr int Count = (value, 4);",
        "const int &alias = value; constexpr int Count = (alias, 4);",
    ] {
        rejected(&format!("__device__ int read(const int &value) {{ {declaration} return 3; }} __global__ void kernel() {{}}"));
    }
    rejected("__device__ int read(const int &value) { return value; } __global__ void kernel() { int value = 3; constexpr int Count = read(value); int values[Count]{}; }");
    validate("__device__ int read(const int &value) { return value; } __global__ void kernel() { int value = 3; constexpr int Count = true ? 4 : read(value); constexpr bool Skip = false && read(value); int values[Count]{}; }");
    rejected("__device__ int read(const int &value) { constexpr int Count = (value, 4); return Count; } __global__ void kernel() { __shared__ int value; read(value); }");
    rejected("constexpr int Global = 3; __device__ void read(const int &value) {} __global__ void kernel() { read((0, Global)); }");
}

#[test]
fn cuda_helper_parameters_reject_missing_definitions_and_source_cycles() {
    for source in [
        "__device__ int missing(int &); __device__ int unused(int &value) { return missing(value); } __global__ void kernel() {}",
        "__device__ int missing(const int &); __global__ void kernel() { int value = 1; if (false) { missing(value); } }",
        "__device__ int direct(int &value) { return direct(value); } __global__ void kernel() {}",
        "__device__ int first(int &); __device__ int second(int &); __device__ int first(int &value) { return second(value); } __device__ int second(int &value) { return first(value); } __global__ void kernel() {}",
        "__device__ int first(int &); __device__ int second(int value) { return first(value); } __device__ int first(int &value) { return false ? second(value) : 1; } __global__ void kernel() {}",
    ] { rejected(source); }
    validate("__device__ int unused(int &); __device__ int rank(float value) { return 3; } __device__ int rank(int &value) { return rank(3.0f); } __global__ void kernel() { int value = 1; rank(value); }");
    rejected("__device__ int invalid(int &value) { if (value) { return value; } } __global__ void kernel() {}");
}

#[test]
fn cuda_helper_parameters_report_malformed_reference_declarators_with_original_spans() {
    for declaration in [
        "__device__ void write(int &&value) {}",
        "__device__ void write(int *&value) {}",
        "__device__ void write(int (&value)[2]) {}",
        "__device__ void write(int &value = 3) {}",
        "__device__ int &read(int &value) { return value; }",
        "__global__ void kernel(int &value) {}",
        "using Alias = int&; __global__ void kernel() {}",
    ] {
        rejected(&format!(
            "/* 漢字 */ {declaration} {}",
            if declaration.contains("__global__") {
                ""
            } else {
                "__global__ void kernel() {}"
            }
        ));
    }
    let source = "#define BAD 3\n/* λ */ __device__ void write(int &value) {} __global__ void kernel() { write(BAD); }";
    let error = rejected(source);
    let range = error.span.to_range().unwrap();
    assert!(range.start >= source.find("write(BAD)").unwrap());
    validate("__device__ void write(int &value) { ++value; } __global__ void kernel() { int value = 1; write(value); }");
}

#[test]
fn cuda_helper_parameters_bound_reference_expansion_and_call_dag_growth() {
    let mut linear = String::from("__device__ void step0(int &value) { ++value; }");
    for index in 1..48 {
        linear.push_str(&format!(
            "__device__ void step{index}(int &value) {{ step{}(value); }}",
            index - 1
        ));
    }
    linear.push_str(
        "__global__ void kernel(int *out) { int value = 0; step47(value); out[0] = value; }",
    );
    validate(&linear);

    let mut doubling = String::from("__device__ void step0(int &value) { ++value; }");
    for index in 1..25 {
        doubling.push_str(&format!(
            "__device__ void step{index}(int &value) {{ step{}(value); step{}(value); }}",
            index - 1,
            index - 1
        ));
    }
    doubling.push_str("__global__ void kernel() { int value = 0; step24(value); }");
    assert!(rejected(&doubling).message.contains("limit"));

    let name = core::iter::repeat_n('x', 131_072).collect::<String>();
    let mut payload = format!("__device__ void leaf(int &value) {{ int {name} = value; value = {name} + 1; }} __global__ void kernel() {{ int value = 0;");
    for _ in 0..1024 {
        payload.push_str("leaf(value);");
    }
    payload.push('}');
    assert!(rejected(&payload).message.contains("limit"));

    for (levels, accepted) in [(28, true), (48, false)] {
        let mut source = String::from("__device__ void step0(int &value) { ++value; }");
        for index in 1..=levels {
            source.push_str(&format!("__device__ void step{index}(int &value) {{ switch (value) {{ case 0: step{}(value); ++value; default: value += 2; }} }}", index - 1));
        }
        source.push_str(&format!("__global__ void kernel(int *out) {{ int value = 0; step{levels}(value); out[0] = value; }}"));
        if accepted {
            let (module, info) = validate(&source);
            #[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
            super::helpers::wgsl_roundtrip(
                &module,
                &info,
                naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
            );
            let _ = (module, info);
        } else {
            let error = rejected(&source);
            assert!(error.message.contains("limit") || error.message.contains("depth"));
        }
    }
}

#[test]
fn cuda_helper_parameters_roundtrip_aliasing_calls_through_shared_wgsl_validation() {
    let (module, info) = validate(FIXTURE);
    assert!(module
        .functions
        .iter()
        .all(
            |(_, function)| function.arguments.iter().all(|argument| !matches!(
                module.types[argument.ty].inner,
                naga::TypeInner::Pointer { .. }
            ))
        ));
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
}

#[cfg_attr(miri, ignore)]
#[test]
fn cuda_helper_parameters_validate_canonical_fixture_and_raw_spirv() {
    let (module, info) = validate(FIXTURE);
    #[cfg(spv_out)]
    {
        let options = naga::back::spv::Options {
            lang_version: (1, 1),
            ..Default::default()
        };
        let pipeline = naga::back::spv::PipelineOptions {
            entry_point: String::from("helper_parameters"),
            shader_stage: naga::ShaderStage::Compute,
        };
        let words = naga::back::spv::write_vec(&module, &info, &options, Some(&pipeline)).unwrap();
        crate::helpers::validate_spirv(&words);
    }
    let _ = (module, info);
}
