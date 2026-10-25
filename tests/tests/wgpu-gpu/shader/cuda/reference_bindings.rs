use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![live_aliases(), frozen_addresses(), workgroup_records()]
}

fn live_aliases() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..24].copy_from_slice(&[
        14,
        14,
        14,
        9,
        4,
        3,
        9,
        9,
        7,
        8,
        8,
        10,
        0,
        0x3fc0_0000,
        3,
        5,
        11,
        3,
        9,
        1,
        1,
        0xffff_0003,
        0x3f80_0000,
        12,
    ]);
    module_case(
        "live scalar and record aliases, const views, converted temporaries and helper-local references",
        include_str!("../../../../../naga/tests/in/cuda/reference-bindings.cu"),
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 32],
        )],
        0,
        [1, 1, 1],
    )
}

fn frozen_addresses() -> ShaderTest {
    let mut expected = [SENTINEL; 40];
    expected[..32].copy_from_slice(&[
        16,
        16,
        2,
        17,
        3,
        8,
        8,
        4,
        0,
        17,
        22,
        3,
        1,
        3,
        13,
        13,
        0x4100_0000,
        0x4080_0000,
        1,
        9,
        7,
        9,
        7,
        1,
        3,
        24,
        13,
        6,
        43,
        47,
        1,
        43,
    ]);
    module_case(
        "array indices and conditional scalar, record, vector and storage addresses are bound once",
        r#"struct Record { int x; int y; int values[2]; };
        __global__ void frozen(unsigned *out) {
            int values[4] = {10, 20, 30, 40};
            int index = 0;
            int &element = values[index++];
            values[0] = 11; index = 2; element += 5;
            out[0] = unsigned(element); out[1] = unsigned(values[0]); out[2] = unsigned(index);
            int &chain = element;
            index = 3; chain += 1;
            out[3] = unsigned(chain); out[4] = unsigned(index);

            bool choose = true;
            int left = 3, right = 4;
            int &selected = choose ? left : right;
            choose = false; selected += 5;
            out[5] = unsigned(selected); out[6] = unsigned(left);
            out[7] = unsigned(right); out[8] = unsigned(choose);

            index = 0;
            int &indexed = choose ? values[index++] : values[++index];
            choose = true; index = 3; indexed += 2;
            out[9] = unsigned(values[0]); out[10] = unsigned(values[1]);
            out[11] = unsigned(index); out[12] = unsigned(choose);

            Record first{3, 4}, second{5, 6};
            choose = false;
            int &member = (choose ? first : second).x;
            choose = true; second.x = 11; member += 2;
            out[13] = unsigned(first.x); out[14] = unsigned(second.x); out[15] = unsigned(member);

            float2 first_vector{1.0f, 2.0f}, second_vector{3.0f, 4.0f};
            float &component = (choose ? first_vector : second_vector).y;
            choose = false; component = 8.0f;
            out[16] = __float_as_uint(first_vector.y); out[17] = __float_as_uint(second_vector.y);

            unsigned local = 9u;
            int counter = 0;
            unsigned &unused = (++counter, local);
            out[18] = unsigned(counter); out[19] = local;
            out[20] = 5u;
            unsigned &cross_root = choose ? local : out[20];
            choose = true; cross_root += 2u;
            out[21] = local; out[22] = cross_root; out[23] = unsigned(counter);

            Record &record = choose ? first : second;
            int &x = record.x; int &y = record.y;
            choose = false;
            int &remapped = choose ? x : y;
            remapped += 20;
            out[24] = unsigned(first.x); out[25] = unsigned(first.y);
            out[26] = unsigned(second.x); out[27] = unsigned(second.y);

            first.values[0] = 31; second.values[0] = 41; second.values[1] = 47;
            Record &array_record = choose ? first : second;
            index = 0;
            int &array_element = array_record.values[index++];
            choose = true; index = 1; array_element += 2;
            out[28] = unsigned(second.values[0]); out[29] = unsigned(second.values[1]);
            out[30] = unsigned(index); out[31] = unsigned(array_element);
        }"#,
        [1, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 40],
        )],
        0,
        [1, 1, 1],
    )
}

fn workgroup_records() -> ShaderTest {
    let mut expected = [SENTINEL; 32];
    expected[..24].copy_from_slice(&[
        15, 15, 12, 3, 0, 7, 18, 18, 13, 4, 0, 8, 17, 17, 14, 5, 0, 9, 20, 20, 15, 6, 0, 10,
    ]);
    module_case(
        "workgroup record and array-member aliases, local loop bindings, switch scopes and temporary lifetime",
        r#"struct Record { int value; int samples[2]; };
        __global__ void records(unsigned *out) {
            unsigned lane = threadIdx.x;
            __shared__ Record shared[4];
            shared[lane].value = int(lane) + 1;
            shared[lane].samples[0] = int(lane) + 10;
            shared[lane].samples[1] = 0;
            __syncthreads();
            Record &record = shared[lane];
            const Record &view = record;
            record.value += 10;
            int &sample = record.samples[0];
            sample += 2;
            for (int iteration = 0; iteration < 2; ++iteration) {
                int &current = record.value;
                current += iteration;
            }
            switch (lane % 2u) {
                case 0u: { int &current = record.value; current += 3; break; }
                default: { int &current = record.value; current += 5; break; }
            }
            const Record &temporary{int(lane) + 3, {int(lane) + 4}};
            const int &temporary_member = Record{int(lane) + 7, {}}.value;
            __syncthreads();
            unsigned slot = lane * 6u;
            out[slot] = unsigned(record.value); out[slot + 1u] = unsigned(view.value);
            out[slot + 2u] = unsigned(sample); out[slot + 3u] = unsigned(temporary.value);
            out[slot + 4u] = unsigned(temporary.samples[1]); out[slot + 5u] = unsigned(temporary_member);
        }"#,
        [4, 1, 1],
        &expected,
    )
    .module_buffers(
        vec![ShaderBuffer::new(
            0,
            wgpu::BufferBindingType::Storage { read_only: false },
            &[SENTINEL; 32],
        )],
        0,
        [1, 1, 1],
    )
}
