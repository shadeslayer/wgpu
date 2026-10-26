struct Record {
    int value;
    int samples[2];
};

__global__ void local_pointers(int *out) {
    int values[4]{2, 3, 5, 7};
    int index = 0;
    int *first = &values[index++];
    int *same{first};
    const int *view = {same};
    int *offset = values + index++;
    bool choose = true;
    int *selected = choose ? values + 2 : values + 3;
    choose = false;
    index = 3;
    *first += 11;
    same[0] += *view;
    offset[0] += 13;
    *selected += 17;
    values[3] = 29;
    out[0] = values[0];
    out[1] = values[1];
    out[2] = values[2];
    out[3] = values[3];
    out[4] = index;
    out[5] = int(choose);
    out[6] = *view;

    int value = 31;
    int *const object = &value;
    int &reference = *object;
    reference += 2;
    const int *const frozen = &reference;
    out[7] = *frozen;

    int2 points[2]{{2, 3}, {5, 7}};
    int2 *pair = points + 1;
    pair->x += 11;
    int *component = &pair->y;
    *component += 13;
    out[8] = pair->x;
    out[9] = points[1].y;

    Record records[2]{{3, {5, 7}}, {11, {13, 17}}};
    Record *record = records + 1;
    int *sample = record->samples;
    record->value += sample[1];
    sample[0] += record->value;
    out[10] = record->value;
    out[11] = sample[0];
    out[12] = sample[1];

    int matrix[2][2]{{19, 23}, {29, 31}};
    int *row = matrix[1];
    *row += 2;
    out[13] = row[0];
    out[14] = row[1];
    int *destination = out + 16;
    *destination = 37;
}
