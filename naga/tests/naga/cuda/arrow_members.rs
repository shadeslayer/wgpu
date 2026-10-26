use naga::front::cuda::{parse_str, Options};

use super::helpers::{validate, validate_with_options};

const FIXTURE: &str = include_str!("../../in/cuda/arrow-members.cu");
const SHARED_SOURCE: &str = r#"struct Sample { int3 point; int counts[2]; };
__device__ void update(Sample *value, const Sample *view, unsigned lane) {
    value->point.x += int(lane) + 1;
    value->counts[0] += view->point.x;
    value->counts[1] = value->point.x + value->counts[0];
}
__device__ void bump(int &value) { value += 2; }
__device__ int forward(Sample *value) {
    bump(value->point.y);
    return value->counts[1];
}
__device__ int previous_z(const Sample *value) { return (value - 1)->point.z; }
__global__ void arrow_spaces(unsigned *out, const Sample *input) {
    unsigned lane = threadIdx.x;
    __shared__ Sample shared[4];
    shared[lane] = input[lane];
    __syncthreads();
    update(&shared[lane], &shared[lane], lane);
    int result = forward(&shared[lane]);
    (&shared[lane].point)->z += int(lane) + 3;
    __syncthreads();
    unsigned slot = lane * 8u;
    out[slot] = unsigned((&shared[lane])->point.x);
    out[slot + 1u] = unsigned((&shared[lane])->counts[0]);
    out[slot + 2u] = unsigned(result);
    out[slot + 3u] = unsigned((&shared[lane])->point.y);
    out[slot + 4u] = unsigned(previous_z(&input[lane + 1u]));
    out[slot + 5u] = unsigned((&shared[lane].point)->z);
    out[slot + 6u] = unsigned((&shared[(lane + 1u) % 4u])->point.x);
    out[slot + 7u] = unsigned((&input[lane])->counts[1]);
}
"#;

fn rejected(source: &str) -> naga::front::cuda::ParseError {
    let error = parse_str(source, &Options::default()).unwrap_err();
    let range = error.span.to_range().unwrap();
    assert!(
        !range.is_empty() || range.start == source.len(),
        "{source}: {error}"
    );
    assert!(source.get(range).is_some(), "{source}: {error}");
    error
}

#[test]
fn cuda_arrow_members_accept_all_numeric_vector_pointees() {
    for source in [
        r#"__device__ void update(int2 *value, const int2 *view) { value->y += view->x; value->x = 3; ++value->y; value->y--; }
__global__ void kernel(unsigned *out) { int2 values[2]{{1, 1}, {2, 2}}; update(values, values + 1); out[0] = unsigned(values->y); out[1] = unsigned((&values[0])->x); }"#,
        r#"__device__ void update(int3 *value, const int3 *view) { value->z += view->x; value->x = 3; ++value->z; value->z--; }
__global__ void kernel(unsigned *out) { int3 values[2]{{1, 1, 1}, {2, 2, 2}}; update(values, values + 1); out[0] = unsigned(values->z); out[1] = unsigned((&values[0])->x); }"#,
        r#"__device__ void update(int4 *value, const int4 *view) { value->w += view->x; value->x = 3; ++value->w; value->w--; }
__global__ void kernel(unsigned *out) { int4 values[2]{{1, 1, 1, 1}, {2, 2, 2, 2}}; update(values, values + 1); out[0] = unsigned(values->w); out[1] = unsigned((&values[0])->x); }"#,
        r#"__device__ void update(uint2 *value, const uint2 *view) { value->y += view->x; value->x = 3; ++value->y; value->y--; }
__global__ void kernel(unsigned *out) { uint2 values[2]{{1, 1}, {2, 2}}; update(values, values + 1); out[0] = unsigned(values->y); out[1] = unsigned((&values[0])->x); }"#,
        r#"__device__ void update(uint3 *value, const uint3 *view) { value->z += view->x; value->x = 3; ++value->z; value->z--; }
__global__ void kernel(unsigned *out) { uint3 values[2]{{1, 1, 1}, {2, 2, 2}}; update(values, values + 1); out[0] = unsigned(values->z); out[1] = unsigned((&values[0])->x); }"#,
        r#"__device__ void update(uint4 *value, const uint4 *view) { value->w += view->x; value->x = 3; ++value->w; value->w--; }
__global__ void kernel(unsigned *out) { uint4 values[2]{{1, 1, 1, 1}, {2, 2, 2, 2}}; update(values, values + 1); out[0] = unsigned(values->w); out[1] = unsigned((&values[0])->x); }"#,
        r#"__device__ void update(float2 *value, const float2 *view) { value->y += view->x; value->x = 3; ++value->y; value->y--; }
__global__ void kernel(unsigned *out) { float2 values[2]{{1, 1}, {2, 2}}; update(values, values + 1); out[0] = unsigned(values->y); out[1] = unsigned((&values[0])->x); }"#,
        r#"__device__ void update(float3 *value, const float3 *view) { value->z += view->x; value->x = 3; ++value->z; value->z--; }
__global__ void kernel(unsigned *out) { float3 values[2]{{1, 1, 1}, {2, 2, 2}}; update(values, values + 1); out[0] = unsigned(values->z); out[1] = unsigned((&values[0])->x); }"#,
        r#"__device__ void update(float4 *value, const float4 *view) { value->w += view->x; value->x = 3; ++value->w; value->w--; }
__global__ void kernel(unsigned *out) { float4 values[2]{{1, 1, 1, 1}, {2, 2, 2, 2}}; update(values, values + 1); out[0] = unsigned(values->w); out[1] = unsigned((&values[0])->x); }"#,
    ] {
        validate(source);
    }
}

#[test]
fn cuda_arrow_members_project_nominal_nested_fields_and_array_decay() {
    for source in [
        r#"struct Leaf { int value; }; struct Record { Leaf nested; Leaf leaves[2]; int samples[2][2]; }; using Alias = Record; using Frozen = const Alias; __device__ int read(Frozen *value) { return value->nested.value + value->leaves[1].value + value->samples[1][0]; } __device__ void leaf(Leaf *value) { value->value += 3; } __global__ void kernel(int *out) { Alias records[2]{}; records->nested.value = 2; leaf(records->leaves); (&records[1])->leaves[1].value = 5; out[0] = read(records) + (records + 1)->leaves[1].value; }"#,
        r#"struct First { int value; }; struct Second { int value; }; __device__ int read(const First *value) { return value->value; } __global__ void kernel(int *out) { First value{3}; out[0] = read(&value); }"#,
    ] {
        validate(source);
    }
}

#[test]
fn cuda_arrow_members_preserve_signed_offsets_and_one_past_origins() {
    validate(
        r#"struct Record { int value; }; __device__ int previous(const Record *value) { return (value - 1)->value + (value + -2)->value; } __global__ void kernel(int *out) { Record records[3]{{2}, {3}, {5}}; out[0] = previous(records + 3); out[1] = (&records[3] - 2)->value; int3 vectors[2]{{7, 11, 13}, {17, 19, 23}}; out[2] = (&vectors[2] - 1)->z; }"#,
    );
}

#[test]
fn cuda_arrow_members_preserve_conditional_pointer_and_reference_choices() {
    validate(
        r#"struct Record { int value; int3 point; }; __global__ void kernel(int *out, unsigned choose) { Record first[2]{{1, {2, 3, 4}}, {}}, second[3]{{5, {6, 7, 8}}, {}, {}}; (choose ? first : second)->value += 2; (&(choose ? first[0] : second[0]))->point.x++; const Record fixed{9, {10, 11, 12}}; out[0] = (choose ? &first[0] : &fixed)->value; }"#,
    );
    rejected(
        r#"struct Record { int value; }; __global__ void kernel(unsigned choose) { Record first{}; const Record second{}; (choose ? &first : &second)->value = 1; }"#,
    );
}

#[test]
fn cuda_arrow_members_evaluate_pointer_operands_once() {
    let (module, _) = validate(
        r#"struct Record { int value; int samples[2]; }; __device__ int next(int value) { return value; } __global__ void kernel(int *out) { Record records[2]{{3, {5, 7}}, {11, {13, 17}}}; int index = 0; out[0] = (&records[next(index++)])->value; (&records[next(index++)])->samples[0] += 2; out[1] = index; }"#,
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
fn cuda_arrow_members_forward_references_and_preserve_live_aliases() {
    validate(
        r#"struct Record { int value; int samples[2]; }; __device__ void step(int &value) { value += 3; } __device__ void forward(Record *value) { step(value->value); step(value->samples[1]); } __device__ void overlap(Record *whole, int *field, const Record *view) { whole->value = 5; *field += view->value; whole->samples[0] = *field; } __global__ void kernel(int *out) { Record value{1, {2, 3}}; forward(&value); overlap(&value, &value.value, &value); out[0] = value.samples[0]; }"#,
    );
}

#[test]
fn cuda_arrow_members_preserve_storage_workgroup_roots_and_bindings() {
    let (module, _) = validate_with_options(
        SHARED_SOURCE,
        &Options {
            workgroup_size: [4, 1, 1],
        },
    );
    assert_eq!(
        module
            .global_variables
            .iter()
            .filter(|(_, variable)| variable.binding.is_some())
            .count(),
        2
    );
    for (binding, access) in [
        (0, naga::StorageAccess::LOAD | naga::StorageAccess::STORE),
        (1, naga::StorageAccess::LOAD),
    ] {
        let (_, variable) = module
            .global_variables
            .iter()
            .find(|(_, variable)| {
                variable.binding == Some(naga::ResourceBinding { group: 0, binding })
            })
            .unwrap();
        assert_eq!(variable.space, naga::AddressSpace::Storage { access });
        if binding == 1 {
            assert!(matches!(
                module.types[variable.ty].inner,
                naga::TypeInner::Array { stride: 20, .. }
            ));
        }
    }
    assert!(module
        .global_variables
        .iter()
        .any(|(_, variable)| variable.space == naga::AddressSpace::WorkGroup));
}

#[test]
fn cuda_arrow_members_preserve_constness_and_writable_lvalues() {
    validate(
        r#"struct Leaf { int value; }; struct Record { unsigned enabled; Leaf nested; int3 point; }; __global__ void kernel(int *out) { Record value{}; (&value)->enabled = 1u; (&value)->nested = Leaf{3}; (&value)->point = int3{5, 7, 11}; (&value)->nested.value += 2; out[0] = (&value)->nested.value++; out[1] = ++(&value)->point.y; }"#,
    );
    for source in [
        r#"struct Record { int value; }; __device__ void write(const Record *value) { value->value = 1; } __global__ void kernel() {}"#,
        r#"struct Record { int value; }; __device__ void write(const Record *value) { value->value++; } __global__ void kernel() {}"#,
        r#"struct Record { int samples[2]; }; __global__ void kernel(const Record *input) { input->samples[1] += 1; }"#,
        r#"struct Record { int3 point; }; __global__ void kernel() { const Record value{}; (&value)->point.x = 1; }"#,
        r#"__global__ void kernel() { const float2 value{}; (&value)->y++; }"#,
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_arrow_members_reject_invalid_operand_and_member_categories() {
    for source in [
        r#"__global__ void kernel() { int value = 1; value->x; }"#,
        r#"struct Record { int value; }; __global__ void kernel() { Record value{}; value->value; }"#,
        r#"__global__ void kernel() { int value = 1; (&value)->x; }"#,
        r#"struct Record { int value; }; __global__ void kernel() { Record values[2]{}; (&values)->value; }"#,
        r#"struct Record { int value; }; __global__ void kernel() { Record value{}; (&value)->missing; }"#,
        r#"__global__ void kernel() { int2 value{}; (&value)->z; }"#,
        r#"__global__ void kernel() { uint3 value{}; (&value)->w; }"#,
        r#"__global__ void kernel() { float4 value{}; (&value)->xy; }"#,
        r#"struct Record { int value; }; __global__ void kernel() { Record first{}; (&first)->value = Record{}; }"#,
        r#"struct Record { int values[2]; }; __global__ void kernel() { Record first{}, second{}; (&first)->values = second.values; }"#,
        r#"struct Record { int value; }; __global__ void kernel() { Record value{}; (&value)->*value; }"#,
        r#"struct Record { int value; }; __device__ Record operator->(Record value) { return value; } __global__ void kernel() {}"#,
    ] {
        rejected(source);
    }

    for (source, field) in [
        (
            r#"struct Record { int value; }; __global__ void kernel() { Record value{}; (&value)->missing; }"#,
            "missing",
        ),
        (
            r#"__global__ void kernel() { int2 value{}; (&value)->z; }"#,
            "z",
        ),
        (
            r#"__global__ void kernel() { uint3 value{}; (&value)->w; }"#,
            "w",
        ),
        (
            r#"__global__ void kernel() { float4 value{}; (&value)->xy; }"#,
            "xy",
        ),
    ] {
        assert_eq!(&source[rejected(source).span.to_range().unwrap()], field);
    }
}

#[test]
fn cuda_arrow_members_preserve_member_and_macro_error_spans() {
    for (source, expected) in [
        (
            r#"/* 漢 */ struct Record { int value; }; __global__ void kernel() { Record value{}; (&value)->missing; }"#,
            "missing",
        ),
        (
            r#"#define FIELD(pointer) (pointer)->missing
/* 漢 */ struct Record { int value; }; __global__ void kernel() { Record value{}; FIELD(&value); }"#,
            "FIELD(&value)",
        ),
        (
            r#"struct Record { int value; }; __global__ void kernel() { Record value{}; (&value)->; }"#,
            ";",
        ),
        (
            r#"struct Record { int value; }; __global__ void kernel() { Record value{}; (&value)->7; }"#,
            "7",
        ),
    ] {
        assert_eq!(&source[rejected(source).span.to_range().unwrap()], expected);
    }
    for source in [
        r#"struct Record { int value; }; __global__ void kernel() { Record value{}; (&value) - > value; }"#,
        r#"struct Record { int value; }; __global__ void kernel() { Record value{}; (&value) -/**/> value; }"#,
    ] {
        rejected(source);
    }
}

#[test]
fn cuda_arrow_members_preserve_constant_expression_eligibility() {
    for source in [
        r#"struct Record { int value; }; __global__ void kernel() { __shared__ Record value; constexpr int Count = ((&value)->value, 3); int values[Count]{}; }"#,
        r#"__global__ void kernel() { __shared__ int2 values[2]; constexpr int Count = ((values + 1)->y, 3); int scratch[Count]{}; }"#,
        r#"struct Record { int value; }; __device__ int count(const Record *value) { constexpr int Count = true ? 3 : value->value; return Count; } __global__ void kernel(int *out) { Record value{}; out[0] = count(&value); }"#,
    ] {
        validate(source);
    }

    for (source, name) in [
        (
            r#"struct Record { int value; }; __global__ void kernel() { __shared__ Record value; constexpr int Count = ((&value)->value, 3); int values[Count]{}; }"#,
            "values",
        ),
        (
            r#"__global__ void kernel() { __shared__ int2 values[2]; constexpr int Count = ((values + 1)->y, 3); int scratch[Count]{}; }"#,
            "scratch",
        ),
    ] {
        let (module, _) = validate(source);
        let (_, local) = module.entry_points[0]
            .function
            .local_variables
            .iter()
            .find(|(_, local)| local.name.as_deref() == Some(name))
            .unwrap();
        let naga::TypeInner::Array {
            size: naga::ArraySize::Constant(count),
            ..
        } = module.types[local.ty].inner
        else {
            panic!("expected a constant-sized local array");
        };
        assert_eq!(count.get(), 3);
    }
    for source in [
        r#"struct Record { int value; }; __device__ int count(const Record *value) { constexpr int Count = value->value; return Count; } __global__ void kernel() {}"#,
        r#"struct Record { int value; }; __device__ int count(const Record *value) { constexpr int Count = (value->value, 3); return Count; } __global__ void kernel() {}"#,
        r#"struct Record { int value; }; __global__ void kernel(int runtime) { __shared__ Record values[2]; constexpr int Count = ((values + runtime)->value, 3); }"#,
        r#"struct Record { int value; }; __global__ void kernel() { __shared__ Record values[2]; int index = 0; constexpr int Count = ((&values[index++])->value, 3); }"#,
        r#"struct Record { int value; }; __global__ void kernel() { __shared__ Record value; int index = 0; constexpr int Count = (((++index, &value))->value, 3); }"#,
        r#"struct Record { int value; }; __global__ void kernel() { __shared__ Record value; constexpr int Count = (&value)->value; }"#,
    ] {
        rejected(source);
    }
}

#[cfg(all(feature = "wgsl-out", feature = "wgsl-in"))]
#[test]
fn cuda_arrow_members_fixture_roundtrips_through_shared_wgsl_helpers() {
    let (module, info) = validate(FIXTURE);
    super::helpers::wgsl_roundtrip(
        &module,
        &info,
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES,
    );
}
