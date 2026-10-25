struct Record {
    int value;
    int samples[2];
};

__device__ int adjust(int value) {
    int &alias = value;
    const int &view = alias;
    alias += 2;
    return view;
}

__global__ void reference_bindings(unsigned *out) {
    int value = 1;
    int &alias = value;
    const int &view = value;
    alias = 5;
    value += 2;
    alias += view;
    out[0] = unsigned(alias);
    out[1] = unsigned(view);
    out[2] = unsigned(value);

    int left = 1, right = 2;
    int &first = left, &second = right;
    first = 5;
    second += 2;
    first += second;
    out[3] = unsigned(left);
    out[4] = unsigned(right);

    int source = 3;
    const float &converted = source;
    const int &live = source;
    source = 9;
    out[5] = unsigned(converted);
    out[6] = unsigned(live);
    out[7] = unsigned(source);

    int uninitialized;
    int &destination = uninitialized;
    destination = 7;
    unsigned &stored = out[8];
    stored = unsigned(uninitialized);

    int original = 2, replacement = 8;
    int &assigned = original;
    assigned = replacement;
    replacement = 10;
    out[9] = unsigned(original);
    out[10] = unsigned(assigned);
    out[11] = unsigned(replacement);

    const int &zero{};
    const float &fraction = 1.5f;
    const int &truncated = 3.75f;
    out[12] = unsigned(zero);
    out[13] = __float_as_uint(fraction);
    out[14] = unsigned(truncated);

    int updated = 1;
    int &prefix = ++updated;
    prefix += 3;
    out[15] = unsigned(prefix);
    int &assignment = (updated = 9);
    assignment += 2;
    out[16] = unsigned(assignment);

    int comma_left = 3, comma_right = 4, counter = 0;
    int &comma = (++counter, comma_right);
    comma += 5;
    out[17] = unsigned(comma_left);
    out[18] = unsigned(comma_right);
    out[19] = unsigned(counter);

    bool enabled = false;
    bool &flag = enabled;
    flag = true;
    out[20] = unsigned(enabled);
    unsigned bits = 0xffff0000u;
    unsigned &word = bits;
    word += 3u;
    out[21] = word;
    float floating = 2.0f;
    float &floating_alias = floating;
    floating_alias *= 0.5f;
    out[22] = __float_as_uint(floating);

    Record record{10, {2, 3}};
    Record &record_alias{record};
    const Record &record_view = {record_alias};
    record_alias.value = adjust(record_alias.value);
    out[23] = unsigned(record_view.value);
}
