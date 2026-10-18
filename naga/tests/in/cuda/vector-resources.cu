using Triple = float3;
using Frozen = const Triple;

__device__ Triple shift(Triple value) {
    value.x += 1.0f;
    value.z = value.y + value.z;
    return value;
}

__device__ float pair(Triple value, int digit) {
    return value.x * 10.0f + digit;
}

__global__ void vector_resources(Frozen *input, Triple *output) {
    for (int i = 0; i < 3; ++i) {
        Triple copied = input[i];
        output[i] = shift(copied);
    }
    output[3] = input[0];
    output[3].y = shift(input[1]).x;
    int index = 0;
    output[index++].x = (index = 1, input[2].x);
    output[4] = make_float3(index, input[0].x, input[1].x);
    bool choose_first = true;
    (choose_first ? output[0] : output[2]).y += 2.0f;
    output[4].x = (choose_first ? output[0] : output[2]).z++;
    output[4].z = index;
    ((choose_first ? output[0] : output[2]) = input[1]).x++;
    output[5] = output[0];
    Triple local = make_float3(20, 30, 40);
    bool pick_local = false;
    (pick_local ? local : output[1]).z = (pick_local = true, 7.0f);
    output[6] = local;
    output[6].x = output[1].z;
    output[7] = make_float3(output[0].x, output[1].x, output[2].x);
    Triple snapshot = input[0];
    output[8] = make_float3(pair(snapshot, (snapshot.x = 9.0f, 4)), snapshot.x, input[2].z);
}
