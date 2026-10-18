use super::{case, ShaderTest};

pub(super) fn cases() -> Vec<ShaderTest> {
    vec![
        case(
            "vector values, exact copies, overloads and left-to-right argument snapshots",
            include_str!("../../../../../naga/tests/in/cuda/vector-values.cu"),
            &[
                3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            &[
                4, 6, 4, 1, 3, 5, 1, 4, 4, 19, 32, 5, 35, 6, 216, 6, 7, 3, 8, 5, 334, 4, 14, 9,
            ],
        ),
        case(
            "projected vector references evaluate assignment RHS and selected indices once",
            r#"__global__ void vector_references(int *out) {
    int3 left = make_int3(1, 2, 3);
    int3 right = make_int3(4, 5, 6);
    bool choose_left = true;
    out[0] = ((choose_left ? left : right).x = (choose_left = false, 9));
    out[1] = left.x * 10 + right.x;
    out[2] = int(choose_left);
    out[3] = ((choose_left ? left : right).z += (choose_left = true, 2));
    out[4] = left.z * 10 + right.z;
    int3 bank[2];
    bank[0] = left;
    bank[1] = right;
    int index = 0;
    out[5] = (choose_left ? (false ? right : bank[index++]) : bank[index++]).y++;
    out[6] = index;
    out[7] = bank[0].y * 10 + bank[1].y;
    out[8] = (left = right).y++;
    out[9] = left.y * 10 + right.y;
    const float3 zero{};
    out[10] = int(zero.x + zero.y + zero.z);
    int total = 0;
    for (int i = 0; i < 3; ++i) {
        int4 reset = {};
        total += reset.x + i;
        reset.x = 9;
    }
    out[11] = total;
}"#,
            &[0; 12],
            &[9, 19, 0, 5, 56, 2, 1, 35, 5, 65, 0, 3],
        ),
    ]
}
