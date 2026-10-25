struct Sample {
    float3 position;
    unsigned tag;
};

struct Packet {
    unsigned serial;
    float2 uv;
    Sample samples[2];
    float weights[2][2];
    float4 colour;
};

using PacketAlias = Packet;
using ConstPacket = const Packet;

__device__ PacketAlias adjust(PacketAlias value, int bias) {
    value.serial += unsigned(bias);
    value.samples[1].position.x += value.weights[0][1];
    value.weights[1][0] = float(value.samples[0].tag);
    return value;
}

__device__ Packet operator+(Packet left, Packet right) {
    left.serial += right.serial;
    left.uv.x += right.uv.y;
    return left;
}

__global__ void structures(const Packet *input, Packet *output) {
    unsigned lane = threadIdx.x;
    if (lane >= 2u) return;
    ConstPacket original = input[lane];
    Packet changed = adjust(original, 3 + int(lane));
    {
        struct Packet {
            int w;
            int x;
        };
        Packet local{7, 11};
        changed.samples[0].tag += unsigned(local.w + local.x);
    }
    PacketAlias combined = changed + original;
    combined.weights[0][0] += float(original.serial);
    combined.colour.w = float(combined.serial);
    unsigned slot = lane + 1u;
    output[slot].serial = combined.serial;
    output[slot].uv = combined.uv;
    for (int sample = 0; sample < 2; ++sample) {
        output[slot].samples[sample] = combined.samples[sample];
    }
    for (int row = 0; row < 2; ++row) {
        for (int column = 0; column < 2; ++column) {
            output[slot].weights[row][column] = combined.weights[row][column];
        }
    }
    output[slot].colour = combined.colour;
}
