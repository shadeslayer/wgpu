struct Leaf { int value; };
struct Record { Leaf nested; int3 point; int samples[3]; };

__device__ void add(Record *record, const Record *view) {
    record->nested.value += view->samples[0];
    record->point.y += view->point.z;
    record->samples[1] += record->nested.value;
}
__device__ void reference_step(int &value) { value += 3; }
__device__ void pointer_step(Record *value) {
    reference_step(value->nested.value);
    reference_step(value->point.x);
}
__device__ void overlap(Record *whole, int *field, const Record *view) {
    whole->nested.value = 5;
    *field += view->nested.value;
    whole->samples[0] = *field;
}
__device__ int previous(const Record *value) { return (value - 1)->samples[2]; }
__device__ int pair_sum(const int *value) { return value[0] + value[1]; }

__global__ void arrow_members(unsigned *out) {
    Record records[3]{
        {{2}, {3, 4, 5}, {6, 7, 8}},
        {{11}, {13, 17, 19}, {23, 29, 31}},
        {{37}, {41, 43, 47}, {53, 59, 61}}
    };
    out[0] = unsigned(records->nested.value);
    records->nested.value += 3;
    out[1] = unsigned(records->nested.value);
    out[2] = unsigned((records + 1)->point.y++);
    out[3] = unsigned((records + 1)->point.y);
    out[4] = unsigned(++(records + 1)->point.z);
    (records + 1)->samples[1] *= 2;
    out[5] = unsigned((records + 1)->samples[1]);
    add(records + 1, records);
    out[6] = unsigned((records + 1)->nested.value);
    out[7] = unsigned((records + 1)->point.y);
    out[8] = unsigned((records + 1)->samples[1]);
    pointer_step(records + 1);
    out[9] = unsigned((records + 1)->nested.value);
    out[10] = unsigned((records + 1)->point.x);
    overlap(records, &records->nested.value, records);
    out[11] = unsigned(records->nested.value);
    out[12] = unsigned(records->samples[0]);
    out[13] = unsigned(previous(records + 3));
    out[14] = unsigned((&records[3] - 2)->nested.value);
    out[15] = unsigned(pair_sum((records + 1)->samples));
    int index = 0;
    (&records[index++])->point.x += 2;
    out[16] = unsigned(records->point.x);
    out[17] = unsigned(index);
    out[18] = unsigned((&records[index++])->samples[0]++);
    out[19] = unsigned((records + 1)->samples[0]);
    out[20] = unsigned(index);
    bool choose = true;
    (choose ? records + 1 : records + 2)->nested.value += 7;
    out[21] = unsigned((records + 1)->nested.value);
    choose = false;
    (&(choose ? records[1] : records[2]))->point.z -= 2;
    out[22] = unsigned((records + 2)->point.z);
    const Record frozen{{67}, {71, 73, 79}, {83, 89, 97}};
    out[23] = unsigned((&frozen)->point.z);
    const int2 pair{101, 103};
    out[24] = unsigned((&pair)->y);
    int2 mutable_pair{107, 109};
    (&mutable_pair)->x += 4;
    out[25] = unsigned((&mutable_pair)->x);
    bool select = false;
    (&(select ? records[0].nested : records[1].nested))->value++;
    out[26] = unsigned((records + 1)->nested.value);
    int &alias = (records + 2)->samples[1];
    reference_step(alias);
    out[27] = unsigned((records + 2)->samples[1]);
    out[28] = unsigned(records[0].point.x);
    out[29] = unsigned(records[1].point.x);
    out[30] = unsigned(records[2].nested.value);
    out[31] = unsigned(index);
}
