typedef unsigned int Slot, Count;
typedef int Signed, Offset;
using Weight = float;
typedef bool Predicate;
typedef const Weight FixedWeight;
using WeightChain = FixedWeight;

__device__ Signed transform(Weight value, FixedWeight multiplier, Offset offset) {
    return value * multiplier + offset;
}

__global__ void scalar_aliases(Slot *out, const Signed *input, Count n) {
    __shared__ Slot shared_values[64];
    using LocalSlot = Slot;
    using LocalWeight = WeightChain;
    LocalSlot i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        LocalSlot base = i * 10u;
        LocalSlot lane = threadIdx.x;
        shared_values[lane] = Slot(i + 10u);
        out[base] = shared_values[lane];
        out[base + 1u] = Slot(transform(Weight(input[i]), FixedWeight(2.0f), Offset(1)));

        using Count = Signed;
        Count signed_count = -3;
        out[base + 2u] = Slot(signed_count);
        {
            typedef Weight Count;
            Count fraction = 3.75f;
            out[base + 3u] = Slot(fraction);
        }
        Count after_block = -5;
        out[base + 4u] = Slot(after_block);

        LocalWeight coefficient = 1.25f;
        out[base + 5u] = Slot(coefficient * 4.0f);
        Predicate selected = i % 2u == 0u;
        out[base + 6u] = Slot(selected);

        typedef Weight Step;
        Signed total = 0;
        for (typedef Signed Step; total < 3;) {
            Step current = total;
            total = current + 1;
        }
        out[base + 7u] = Slot(total);
        Step after_loop = 1.5f;
        out[base + 8u] = Slot(after_loop * 2.0f);
        out[base + 9u] = (Slot)(LocalWeight(n));
    }
}
