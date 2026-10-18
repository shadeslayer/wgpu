struct type_3 {
    int value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

struct type_4 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
cbuffer value_1 : register(b1) { type_3 value_1; }
cbuffer amount_1 : register(b2) { type_4 amount_1; }

int naga_neg(int val) {
    return asint(-asuint(val));
}

int naga_mod(int lhs, int rhs) {
    int divisor = ((lhs == int(-2147483647 - 1) & rhs == -1) | (rhs == 0)) ? 1 : rhs;
    return lhs - (lhs / divisor) * divisor;
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

[numthreads(1, 1, 1)]
void integer_ops(uint3 threadIdx : SV_GroupThreadID)
{
    int value = (int)0;
    uint amount = (uint)0;
    int remainder = (int)0;
    uint bits = (uint)0;
    uint local = (uint)0;

    int _e3 = value_1.value;
    value = _e3;
    uint _e7 = amount_1.value;
    amount = _e7;
    int _e12 = value;
    remainder = naga_mod(_e12, naga_neg(int(3)));
    int _e15 = remainder;
    bits = (uint(_e15) & 15u);
    uint _e20 = bits;
    bits = (_e20 | 8u);
    uint _e23 = bits;
    bits = (_e23 ^ 1u);
    uint _e25 = amount;
    uint _e26 = bits;
    local = _e25;
    uint _e28 = local;
    bits = (_e26 << _e28);
    uint _e31 = bits;
    bits = (_e31 >> 1u);
    uint _e34 = bits;
    bits = naga_mod(_e34, 97u);
    uint _e37 = bits;
    bits = (_e37 & 255u);
    uint _e42 = bits;
    out_.Store(threadIdx.x*4, asuint(~(_e42)));
}
