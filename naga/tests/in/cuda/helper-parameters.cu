struct Pair { int x; int y; };
struct Record { int x; int y; int values[2]; };
struct Box { int value; };
__device__ void combine(int &first, int &second) { first = 5; second += 2; first += second; }
__device__ void live(int &write, const int &read) { write = 7; write += read; }
__device__ void overlap(Record &whole, int &field) { whole.x = 5; field += 2; whole.y = field; }
__device__ void vector_overlap(float2 &whole, float &field) { whole.x = 5.0f; field += 2.0f; whole.y = field; }
__device__ void initialize(int &target) { target = 23; }
__device__ int forward(int &write, const int &read) { int &alias = write; live(alias, read); return alias; }
__device__ int copied_record(Record &live, Record snapshot) { live.x = 9; return live.x + snapshot.x; }
__device__ Record copy_arrays(Record value, const Record &source) { value.values[0] += source.values[1]; return value; }
__device__ int record_sum(const Record &value) { return value.x + value.values[1]; }
__device__ float scalar_view(const float &value) { return value; }
__device__ void toggle(bool &value) { value = !value; }
__device__ void vector_alias(int2 &value, const int &view) { value.y += view; value.x += value.y; }
__device__ Pair make_pair(int &value) { value += 3; return Pair{value, value + 1}; }
__device__ int operator-(Record &value) { ++value.x; return value.x; }
__device__ int operator+(Record &left, const Record &right) { left.x += right.x; return left.x; }
__device__ int rank(int &value) { ++value; return 10; }
__device__ int rank(const int &value) { return 20; }
__device__ int operator<<(Box &left, int right) { return left.value * 100 + right; }
__device__ int operator>>(int &left, Box right) { return left * 100 + right.value; }
__device__ int operator&&(Box &left, Box right) { left.value += right.value; return left.value; }
__device__ int operator||(Box &left, Box right) { left.value += right.value; return left.value; }
__global__ void helper_parameters(unsigned *out) {
    int same = 1; combine(same, same); out[0] = unsigned(same);
    int first = 1, second = 2; combine(first, second); out[1] = unsigned(first); out[2] = unsigned(second);
    int shared_value = 1; live(shared_value, shared_value); out[3] = unsigned(shared_value);
    Record record{3, 4, {5, 6}}; overlap(record, record.x); out[4] = unsigned(record.x); out[5] = unsigned(record.y);
    float2 vector{1.0f, 2.0f}; vector_overlap(vector, vector.x); out[6] = __float_as_uint(vector.x); out[7] = __float_as_uint(vector.y);
    int fresh; initialize(fresh); out[8] = unsigned(fresh);
    int nested = 1; out[9] = unsigned(forward(nested, nested)); out[10] = unsigned(nested);
    Record original{3, 4, {5, 6}}; out[11] = unsigned(copied_record(original, original)); out[12] = unsigned(original.x);
    Record copied = copy_arrays(original, original); out[13] = unsigned(copied.values[0]); out[14] = unsigned(original.values[0]);
    out[15] = unsigned(record_sum(Record{2, 3, {4, 5}}));
    out[16] = __float_as_uint(scalar_view(13)); out[17] = __float_as_uint(scalar_view(1.5f));
    bool enabled = false; toggle(enabled); out[18] = unsigned(enabled);
    int2 integers{2, 3}; vector_alias(integers, integers.y); out[19] = unsigned(integers.x); out[20] = unsigned(integers.y);
    int made = 4; Pair pair = make_pair(made); out[21] = unsigned(pair.x); out[22] = unsigned(pair.y); out[23] = unsigned(made);
    Record operand{4, 0, {0, 0}}; out[24] = unsigned(-operand); out[25] = unsigned(operand.x);
    out[26] = unsigned(operand + operand); out[27] = unsigned(operand.x);
    int mutable_value = 7; const int fixed = 4; out[28] = unsigned(rank(mutable_value)); out[29] = unsigned(mutable_value); out[30] = unsigned(rank(fixed)); out[31] = unsigned(rank(4));
    Box shifted{3}; out[32] = unsigned(shifted << (shifted.value = 7)); out[33] = unsigned(shifted.value);
    int scalar = 3; out[34] = unsigned(scalar >> (scalar = 7, Box{5})); out[35] = unsigned(scalar);
    int counter = 0; Box logical{0}; out[36] = unsigned(logical && Box{++counter}); out[37] = unsigned(counter);
    logical = Box{1}; out[38] = unsigned(logical || Box{++counter}); out[39] = unsigned(counter);
}
