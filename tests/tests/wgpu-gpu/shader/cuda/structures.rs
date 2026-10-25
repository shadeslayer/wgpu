use super::{module_case, ShaderBuffer, ShaderTest};

const SENTINEL: u32 = 0xdead_beef;

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        storage_layout(),
        values_and_side_effects(),
        shared_records(),
    ]
}

fn storage_layout() -> ShaderTest {
    let mut input = vec![SENTINEL; 60];
    input[..40].copy_from_slice(&[
        10,
        0x0123_4567,
        1.0_f32.to_bits(),
        2.0_f32.to_bits(),
        3.0_f32.to_bits(),
        4.0_f32.to_bits(),
        5.0_f32.to_bits(),
        6,
        7.0_f32.to_bits(),
        8.0_f32.to_bits(),
        9.0_f32.to_bits(),
        10,
        11.0_f32.to_bits(),
        12.0_f32.to_bits(),
        13.0_f32.to_bits(),
        14.0_f32.to_bits(),
        15.0_f32.to_bits(),
        16.0_f32.to_bits(),
        17.0_f32.to_bits(),
        18.0_f32.to_bits(),
        20,
        0x7654_3210,
        2.0_f32.to_bits(),
        4.0_f32.to_bits(),
        6.0_f32.to_bits(),
        8.0_f32.to_bits(),
        10.0_f32.to_bits(),
        12,
        14.0_f32.to_bits(),
        16.0_f32.to_bits(),
        18.0_f32.to_bits(),
        20,
        22.0_f32.to_bits(),
        24.0_f32.to_bits(),
        26.0_f32.to_bits(),
        28.0_f32.to_bits(),
        30.0_f32.to_bits(),
        32.0_f32.to_bits(),
        34.0_f32.to_bits(),
        36.0_f32.to_bits(),
    ]);
    let mut expected = [SENTINEL; 80];
    expected[20..60].copy_from_slice(&[
        23,
        SENTINEL,
        3.0_f32.to_bits(),
        2.0_f32.to_bits(),
        3.0_f32.to_bits(),
        4.0_f32.to_bits(),
        5.0_f32.to_bits(),
        24,
        19.0_f32.to_bits(),
        8.0_f32.to_bits(),
        9.0_f32.to_bits(),
        10,
        21.0_f32.to_bits(),
        12.0_f32.to_bits(),
        6.0_f32.to_bits(),
        14.0_f32.to_bits(),
        15.0_f32.to_bits(),
        16.0_f32.to_bits(),
        17.0_f32.to_bits(),
        23.0_f32.to_bits(),
        44,
        SENTINEL,
        6.0_f32.to_bits(),
        4.0_f32.to_bits(),
        6.0_f32.to_bits(),
        8.0_f32.to_bits(),
        10.0_f32.to_bits(),
        30,
        38.0_f32.to_bits(),
        16.0_f32.to_bits(),
        18.0_f32.to_bits(),
        20,
        42.0_f32.to_bits(),
        24.0_f32.to_bits(),
        12.0_f32.to_bits(),
        28.0_f32.to_bits(),
        30.0_f32.to_bits(),
        32.0_f32.to_bits(),
        34.0_f32.to_bits(),
        44.0_f32.to_bits(),
    ]);
    module_case(
        "natural Packet storage stride and nested fields, helper copies, record overloads and local class scope",
        include_str!("../../../../../naga/tests/in/cuda/structures.cu"),
        [2, 1, 1],
        &expected,
    ).module_buffers(
        vec![
            ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: true }, &input),
            ShaderBuffer::new(1, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 80]),
        ],
        1,
        [1, 1, 1],
    )
}

fn values_and_side_effects() -> ShaderTest {
    let mut expected = [SENTINEL; 48];
    expected[..43].copy_from_slice(&[
        5, 5, 7, 7, 10, 9, 39, 39, 14, 10, 19, 14, 3, 0, 4, 0, 5, 0, 3, 8, 7, 6, 8, 12, 13, 10, 39,
        1, 1, 10, 0, 2, 5, 8, 21, 22, 23, 24, 5, 7, 4, 3, 4,
    ]);
    module_case(
        "record initialization order, partial zeros, copied arrays, typed temporaries, conditional fields and address-once updates",
        "struct Leaf { int w, x; };
        struct Data { int key; Leaf leaf; int values[2][2]; int3 point; };
        using Fixed = const Data;
        __device__ Data alter(Data value) {
            value.key += 2; value.leaf.x += 3;
            value.values[1][0] += 4; value.point.z += 5;
            return value;
        }
        __device__ Data identity(Data value) { return value; }
        __device__ Data make() { return Data{1, {2, 3}, {{4, 5}, {6, 7}}, {8, 9, 10}}; }
        __global__ void records(unsigned *out) {
            Data first{5, {6, 7}, {{8, 9}, {10, 11}}, {12, 13, 14}};
            Data second{}; second = first; second.values[0][1] += 30;
            Data changed = alter(second);
            out[0] = unsigned(first.key); out[1] = unsigned(second.key); out[2] = unsigned(changed.key);
            out[3] = unsigned(first.leaf.x); out[4] = unsigned(changed.leaf.x);
            out[5] = unsigned(first.values[0][1]); out[6] = unsigned(second.values[0][1]); out[7] = unsigned(changed.values[0][1]);
            out[8] = unsigned(changed.values[1][0]); out[9] = unsigned(first.values[1][0]);
            out[10] = unsigned(changed.point.z); out[11] = unsigned(first.point.z);
            Data partial{2, {3}, {{4}}, {5}};
            out[12] = unsigned(partial.leaf.w); out[13] = unsigned(partial.leaf.x);
            out[14] = unsigned(partial.values[0][0]); out[15] = unsigned(partial.values[1][1]);
            out[16] = unsigned(partial.point.x); out[17] = unsigned(partial.point.z);
            out[18] = unsigned(make().leaf.x);
            out[19] = unsigned(identity(Data{2, {3, 4}, {{5, 6}, {7, 8}}, {9, 10, 11}}).values[1][1]);
            Fixed fixed = changed; out[20] = unsigned(fixed.key);
            bool choose = false;
            (choose ? first : second).leaf.w += 2;
            out[21] = unsigned(first.leaf.w); out[22] = unsigned(second.leaf.w);
            (choose ? first : second).point.x++;
            out[23] = unsigned(first.point.x); out[24] = unsigned(second.point.x);
            Data items[2]{first, second}; int row = 0, column = 0;
            items[row++].values[column++][1] += 1;
            out[25] = unsigned(items[0].values[0][1]); out[26] = unsigned(items[1].values[0][1]);
            out[27] = unsigned(row); out[28] = unsigned(column);
            int counter = 0;
            Data ordered{counter++, {counter++, counter++}, {{counter++, counter++}, {counter++, counter++}}, {counter++, counter++, counter++}};
            out[29] = unsigned(counter); out[30] = unsigned(ordered.key); out[31] = unsigned(ordered.leaf.x);
            out[32] = unsigned(ordered.values[1][0]); out[33] = unsigned(ordered.point.y);
            { struct Data { int xy, sinf, w, x; }; Data local{21, 22, 23, 24};
                out[34] = unsigned(local.xy); out[35] = unsigned(local.sinf); out[36] = unsigned(local.w); out[37] = unsigned(local.x);
            }
            out[38] = unsigned(first.key); out[39] = unsigned((true ? first : make()).leaf.x);
            Data self{2, {self.key + 1, self.leaf.w + 1}, {{self.leaf.x, 5}, {6, 7}}, {8, 9, 10}};
            out[40] = unsigned(self.values[0][0]); out[41] = unsigned(self.leaf.w); out[42] = unsigned(self.leaf.x);
        }",
        [1, 1, 1],
        &expected,
    ).module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 48])],
        0,
        [1, 1, 1],
    )
}

fn shared_records() -> ShaderTest {
    let mut expected = [SENTINEL; 48];
    for group in 0..2 {
        for lane in 0..4 {
            let neighbor = (lane + 1) % 4;
            let key = group * 10 + neighbor;
            let row = (group * 4 + lane) * 5;
            expected[row..row + 5].copy_from_slice(&[
                key as u32,
                neighbor as u32 + 1,
                neighbor as u32 + 5 + key as u32,
                (neighbor as f32).to_bits(),
                neighbor as u32 + 5,
            ]);
        }
    }
    module_case(
        "shared records with arrays, packed vector members, neighbor snapshots and private copy updates",
        "struct Shared { int key; int values[2]; float3 direction; };
        __global__ void records(unsigned *out) {
            __shared__ Shared records[4];
            unsigned lane = threadIdx.x, group = blockIdx.x;
            records[lane] = Shared{int(group * 10u + lane), {int(lane + 1u), int(lane + 5u)}, {float(lane), float(group), float(lane + group)}};
            __syncthreads();
            Shared copied = records[(lane + 1u) % 4u];
            copied.values[1] += copied.key;
            __syncthreads();
            unsigned row = (group * 4u + lane) * 5u;
            out[row] = unsigned(copied.key);
            out[row + 1u] = unsigned(copied.values[0]);
            out[row + 2u] = unsigned(copied.values[1]);
            out[row + 3u] = __float_as_uint(copied.direction.x);
            out[row + 4u] = unsigned(records[(lane + 1u) % 4u].values[1]);
        }",
        [4, 1, 1],
        &expected,
    ).module_buffers(
        vec![ShaderBuffer::new(0, wgpu::BufferBindingType::Storage { read_only: false }, &[SENTINEL; 48])],
        0,
        [2, 1, 1],
    )
}
