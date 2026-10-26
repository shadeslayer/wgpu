use naga::front::cuda::{parse_str, Options};

use super::helpers::{validate, validate_with_options};

const FIXTURE: &str = include_str!("../../in/cuda/local-pointers.cu");

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(!range.is_empty() || range.start == source.len(), "{source}");
    assert!(source.get(range).is_some(), "{source}");
    assert!(!error.message.is_empty(), "{source}");
    error
}

fn rejected_at(source: &str, token: &str, occurrence: usize) {
    let error = rejected(source);
    let start = source.match_indices(token).nth(occurrence).unwrap().0;
    assert_eq!(
        error.span,
        naga::Span::from(start..start + token.len()),
        "{source}: {error}"
    );
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
fn cuda_local_pointers_accept_initialized_forms_and_qualifiers() {
    validate(
        r#"__global__ void kernel(int *out) {
    int value = 3;
    int *copy = &value;
    int *direct{copy};
    int *listed = {direct,};
    int *const fixed = listed;
    const int *readonly = fixed;
    int const *const frozen{readonly,};
    *copy = 5;
    *direct += 2;
    out[0] = *frozen;
}
"#,
    );
    validate(
        r#"using Integer = int;
using Frozen = const Integer;
__global__ void kernel(int *out) {
    bool flag = false;
    bool *flag_pointer = &flag;
    *flag_pointer = true;
    unsigned bits = 7u;
    unsigned *bits_pointer{&bits};
    *bits_pointer += 2u;
    float weight = 1.5f;
    float *weight_pointer = {&weight};
    *weight_pointer += 0.5f;
    Integer number = -3;
    Integer *integer = &number;
    Frozen *view = integer;
    int2 pairs[2]{{1, 2}, {3, 4}};
    int2 *pair = pairs + 1;
    uint3 triples[2]{{5, 6, 7}, {8, 9, 10}};
    uint3 *triple{triples};
    float4 samples[2]{{1, 2, 3, 4}, {5, 6, 7, 8}};
    const float4 *sample = {samples + 1};
    pair->y += 1;
    triple->z += 2u;
    out[0] = int(flag) + int(bits) + int(weight) + *view;
    out[1] = pair->y + int(triple->z) + int(sample->w);
}
"#,
    );
}

#[test]
fn cuda_local_pointers_preserve_supported_array_object_and_address_spaces() {
    validate(
        r#"struct Leaf { int value; };
struct Record { Leaf leaves[2]; float2 vector; int matrix[2][2]; };
using Alias = Record;
__global__ void kernel(int *out) {
    Alias record{{{3}, {5}}, {7, 11}, {{13, 17}, {19, 23}}};
    Alias *whole = &record;
    Leaf *leaves = whole->leaves;
    int *field = &leaves[1].value;
    float *component = &whole->vector.y;
    int *row = whole->matrix[1];
    int &reference = *field;
    int *from_reference = &reference;
    *from_reference += 2;
    *component += 3.0f;
    row[1] += *field;
    out[0] = whole->leaves[1].value + int(*component) + row[1];
}
"#,
    );
    let (module, _) = validate_with_options(
        r#"struct Record { int value; int samples[2]; };
__device__ void add(int *destination, const int *source) {
    int *local = destination;
    const int *view = source;
    *local += *view;
}
__global__ void kernel(const Record *input, Record *output, int *out) {
    const Record *source = input;
    Record *destination{output};
    __shared__ Record shared[1];
    Record *workgroup = shared;
    *workgroup = source[0];
    __syncthreads();
    add(&workgroup->value, &source->samples[1]);
    *destination = *workgroup;
    out[0] = destination->value;
}
"#,
        &Options {
            workgroup_size: [1, 1, 1],
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
    let output = module
        .global_variables
        .iter()
        .find(|(_, variable)| variable.name.as_deref() == Some("output"))
        .unwrap()
        .1;
    assert_eq!(
        output.space,
        naga::AddressSpace::Storage {
            access: naga::StorageAccess::LOAD | naga::StorageAccess::STORE,
        }
    );
    assert_eq!(
        module
            .global_variables
            .iter()
            .filter(|(_, variable)| variable.binding.is_some())
            .count(),
        3
    );
    assert!(module
        .global_variables
        .iter()
        .any(|(_, variable)| variable.space == naga::AddressSpace::WorkGroup));
}

#[test]
fn cuda_local_pointers_capture_initializers_once_and_keep_live_aliases() {
    validate(FIXTURE);
    validate(
        r#"__global__ void kernel(int *out) {
    int first[3]{2, 3, 5};
    int second[3]{7, 11, 13};
    bool choose = true;
    int count = 0;
    int *selected = (++count, choose ? first + 1 : second + 1);
    int *end = first + 3;
    int *last = end - 1;
    choose = false;
    *selected += 2;
    *last += 3;
    out[0] = first[1];
    out[1] = second[1];
    out[2] = first[2];
    out[3] = count;
}
"#,
    );
    let (module, _) = validate(
        r#"__global__ void kernel() {
    int fresh;
    int *pointer = &fresh;
    int *same = pointer;
    *same = 23;
}
"#,
    );
    let function = &module.entry_points[0].function;
    let fresh = function
        .local_variables
        .iter()
        .find(|(_, variable)| variable.name.as_deref() == Some("fresh"))
        .unwrap()
        .0;
    for (_, expression) in function.expressions.iter() {
        if let naga::Expression::Load { pointer } = *expression {
            assert!(!matches!(
                function.expressions[root_expression(function, pointer)],
                naga::Expression::LocalVariable(local) if local == fresh
            ));
        }
    }
    assert!(naga::proc::any_statement(
        &function.body,
        &mut |statement| {
            match *statement {
                naga::Statement::Store { pointer, .. } => matches!(
                    function.expressions[root_expression(function, pointer)],
                    naga::Expression::LocalVariable(local) if local == fresh
                ),
                _ => false,
            }
        }
    ));
}

#[test]
fn cuda_local_pointers_survive_helper_reentry_and_pointer_reference_forwarding() {
    validate(
        r#"__device__ int advance(int *values, int slot, bool stop) {
    int *selected = values + slot;
    int *same{selected};
    const int *view = {same};
    *selected += 2;
    if (stop) return *view;
    same[1] += *view;
    return same[1];
}

__device__ void accumulate(int *destination, const int *source) {
    int *write = destination;
    const int *read = source;
    *write += *read;
}

__device__ int read(const int *value) {
    const int *local = value;
    return *local;
}

__global__ void kernel(int *out) {
    int values[5]{1, 3, 5, 7, 11};
    int total = 0;
    for (int iteration = 0; iteration < 3; ++iteration) {
        total += advance(values, iteration, iteration == 1);
    }
    accumulate(values + 1, values + 2);
    int *first = values, count = *first, &alias = *first, *second = first + 1;
    *second += count;
    alias += 2;

    int value = 19;
    int *outer = &value;
    {
        int value = 23;
        int *outer = &value;
        *outer += 6;
        out[10] = *outer;
    }
    out[11] = *outer;

    int cursor = 0;
    for (int *head = values, limit = 2; cursor < limit; ++cursor) {
        int *current = head + cursor;
        *current += 1;
    }
    switch (cursor) {
        case 2: {
            int *within = values + 2;
            *within += 3;
            out[13] = *within;
            break;
        }
        default: out[13] = -7;
    }
    out[0] = total;
    out[1] = values[0];
    out[2] = values[1];
    out[3] = values[2];
    out[4] = values[3];
    out[5] = values[4];
    out[6] = count;
    out[7] = alias;
    out[8] = *second;
    out[9] = read(first);
    out[12] = cursor;
}
"#,
    );
}

#[test]
fn cuda_local_pointers_support_mixed_declarators_and_pending_binding_names() {
    validate(
        r#"using Number = int;
__global__ void kernel(int *out) {
    int values[3]{2, 3, 5};
    int *pointer = values, copy = *pointer, &alias = *pointer, *other{pointer + 1};
    const int *view = pointer, frozen = 7, &reference = alias;
    Number *Number = values, next = Number[1], *tail = Number + 2;
    *other += copy;
    alias += frozen;
    out[0] = *view + reference + next + *tail;
}
"#,
    );
    validate(
        r#"__global__ void kernel(int *out) {
    int values[2]{2, 3};
    int *begin = values, local[2]{7, 11}, &tail = local[1], *end = local + 2;
    tail += begin[1];
    out[0] = *(end - 1);
}
"#,
    );
    for source in [
        r#"__global__ void kernel() {
    int *pointer = pointer;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    { int *pointer = pointer + 1; }
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values, *pointer = values + 1;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *first = second, *second = values;
}
"#,
        r#"__global__ void kernel() {
    using Value = int;
    int value = 3;
    int *Value = &value;
}
"#,
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_local_pointers_follow_lexical_loop_and_switch_initialization_scopes() {
    validate(
        r#"__global__ void kernel(int *out) {
    int values[2]{3, 5};
    int *outside = values;
    {
        int *outside = values + 1;
        *outside += 2;
    }
    int index = 0;
    for (int *head = values, limit = 2; index < limit; ++index) {
        int *element = head + index;
        if (index == 0) continue;
        *element += 3;
    }
    switch (index) {
        case 2: { int *inside = values + 1; *inside += 4; }
        default: break;
    }
    out[0] = *outside;
}
"#,
    );
    for source in [
        r#"__global__ void kernel() {
    int value = 3;
    { int *hidden = &value; }
    *hidden = 4;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int index = 0;
    for (int *hidden = values; index < 2; ++index) {}
    *hidden = 4;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    switch (0) { case 0: { int *hidden = &value; } }
    *hidden = 4;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    switch (0) { int *pointer = &value; case 0: break; }
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    switch (0) { case 0: int *pointer = &value; case 1: break; }
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    switch (0) { case 0: const int *const pointer{&value}; default: break; }
}
"#,
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_local_pointers_reject_invalid_types_const_loss_and_initializer_values() {
    for source in [
        r#"__global__ void kernel() {
    float value = 1.0f;
    int *pointer = &value;
}
"#,
        r#"__global__ void kernel() {
    int3 value{1, 2, 3};
    int2 *pointer = &value;
}
"#,
        r#"struct First { int value; };
struct Second { int value; };
__global__ void kernel() {
    First value{3};
    Second *pointer = &value;
}
"#,
        r#"__global__ void kernel() {
    const int value = 3;
    int *pointer = &value;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    const int *view = &value;
    int *pointer = view;
}
"#,
        r#"__global__ void kernel() {
    int first[2]{2, 3};
    const int second[2]{5, 7};
    int *pointer = threadIdx.x ? first : second;
}
"#,
        r#"__global__ void kernel(const int *input) {
    int *pointer = input;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    const int *pointer = &value;
    *pointer = 4;
}
"#,
        r#"__global__ void kernel(const int *input) {
    const int *pointer = input;
    pointer[0] += 1;
}
"#,
        r#"__global__ void kernel() {
    int *pointer;
}
"#,
        r#"__global__ void kernel() {
    int *const pointer;
}
"#,
        r#"__global__ void kernel() {
    int *pointer{};
}
"#,
        r#"__global__ void kernel() {
    int *pointer = {};
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer{values, values + 1};
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    int *pointer{{&value}};
}
"#,
        r#"__global__ void kernel() {
    int *pointer = nullptr;
}
"#,
        r#"__global__ void kernel() {
    int *pointer = 0;
}
"#,
        r#"__global__ void kernel() {
    int *pointer{0};
}
"#,
        r#"__global__ void kernel() {
    int *pointer = 3;
}
"#,
        r#"__global__ void kernel() {
    int *pointer = int2{1, 2};
}
"#,
        r#"__device__ void consume() {}
__global__ void kernel() {
    int *pointer = consume();
}
"#,
        r#"__global__ void kernel() {
    int *pointer = &3;
}
"#,
        r#"struct Record { int samples[2]; int2 point; };
__global__ void kernel() {
    Record value{{2, 3}, {5, 7}};
    const Record *view = &value;
    view->samples[1] += 1;
}
"#,
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_local_pointers_keep_updates_escaping_and_unsupported_declarators_rejected() {
    for source in [
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    pointer = values + 1;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    pointer += 1;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    pointer -= 1;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    ++pointer;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    pointer++;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    --pointer;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    pointer--;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    int *pointer = &value;
    int **address = &pointer;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    int *pointer = &value;
    int *&reference = pointer;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointers[2]{values, values + 1};
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    int (*pointer) = &value;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    int *pointer(&value);
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int (*pointer)[2] = &values;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    void *pointer = &value;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    int *volatile pointer = &value;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    int *const const pointer = &value;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    __shared__ int *pointer = &value;
}
"#,
        r#"__global__ void kernel() {
    __shared__ int value;
    constexpr int *pointer = &value;
}
"#,
        r#"__device__ int *escape(int *value) {
    int *local = value;
    return local;
}
__global__ void kernel() {}
"#,
        r#"struct Record { int *value; };
__global__ void kernel() {}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    bool equal = pointer == values;
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    int distance = pointer - values;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    int *pointer = &value;
    unsigned bits = unsigned(pointer);
}
"#,
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_local_pointers_preserve_existing_constant_expression_eligibility() {
    validate(
        r#"__global__ void kernel(int *out) {
    __shared__ int values[2];
    int *pointer = values;
    const int *const frozen = pointer;
    constexpr int Count = (pointer, 4);
    constexpr int Other = (frozen, 3);
    int first[Count]{};
    int second[Other]{};
    out[0] = first[3] + second[2];
}
"#,
    );
    validate(
        r#"__global__ void kernel(int *out) {
    __shared__ int values[2];
    int *pointer = values;
    constexpr int Count = true ? 4 : *pointer;
    constexpr bool Skip = false && *pointer;
    int storage[Count]{};
    out[0] = storage[3] + int(Skip);
}
"#,
    );
    for source in [
        r#"__global__ void kernel() {
    __shared__ int values[2];
    int *pointer = values;
    constexpr int Count = (+pointer, 4);
    int storage[Count]{};
}
"#,
        r#"__global__ void kernel() {
    __shared__ int values[2];
    int *pointer = values;
    constexpr int Count = (pointer + 0, 4);
    int storage[Count]{};
}
"#,
        r#"__global__ void kernel() {
    __shared__ int values[2];
    int *pointer = values;
    constexpr int Count = (*pointer, 4);
    int storage[Count]{};
}
"#,
        r#"__global__ void kernel() {
    __shared__ int values[2];
    int *pointer = values;
    constexpr int Count = (pointer[0], 4);
    int storage[Count]{};
}
"#,
        r#"__global__ void kernel() {
    __shared__ int values[2];
    int *pointer = values;
    constexpr int Count = (&*pointer, 4);
    int storage[Count]{};
}
"#,
        r#"__global__ void kernel() {
    __shared__ int value;
    const int *const pointer = &value;
    constexpr int Count = (*pointer, 4);
}
"#,
        r#"__global__ void kernel() {
    int values[2]{2, 3};
    int *pointer = values;
    constexpr int Count = (pointer + 0, 4);
}
"#,
    ] {
        let error = rejected(source);
        assert!(error.message.contains("constant"), "{error}");
    }
}

#[test]
fn cuda_local_pointers_check_unreachable_sources_and_original_diagnostic_spans() {
    validate(
        r#"__device__ void unused(int *value) {
    int *local = value;
    *local = 3;
}
__global__ void kernel() {}
"#,
    );
    for source in [
        r#"__global__ void kernel() {
    return;
    int *pointer;
}
"#,
        r#"__global__ void kernel() {
    int value = 3;
    const int *view = &value;
    return;
    int *pointer = view;
}
"#,
        r#"__device__ void unused(int *value) {
    float *local = value;
}
__global__ void kernel() {}
"#,
    ] {
        rejected(source);
    }
    rejected_at(
        r#"#define BAD 1.5f
/* λ 漢 */
__global__ void kernel() {
    int *pointer = BAD;
}
"#,
        "BAD",
        1,
    );
    rejected_at(
        r#"/* λ 漢 */
__global__ void kernel() {
    int value = 3;
    int *pointer = &value;
    ++pointer;
}
"#,
        "pointer",
        1,
    );
}
