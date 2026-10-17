struct type_3 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

RWByteAddressBuffer out_ : register(u0);
ByteAddressBuffer input_a : register(t1);
ByteAddressBuffer input_b : register(t2);
cbuffer n_1 : register(b3) { type_3 n_1; }
groupshared int shared_values[64];

int shifted(int value_1)
{
    int value_2 = (int)0;

    value_2 = value_1;
    int _e3 = value_2;
    return asint(asuint(_e3) + asuint(int(10)));
}

void discard_(int value_3)
{
    int value_4 = (int)0;

    value_4 = value_3;
    int _e3 = value_4;
    value_4 = asint(asuint(_e3) + asuint(int(1)));
    return;
}

int pair(int x, int y)
{
    int x_1 = (int)0;
    int y_1 = (int)0;

    x_1 = x;
    y_1 = y;
    int _e5 = x_1;
    int _e7 = y_1;
    return asint(asuint(asint(asuint(_e5) * asuint(int(10)))) + asuint(_e7));
}

int naga_neg(int val) {
    return asint(-asuint(val));
}

uint naga_mod(uint lhs, uint rhs) {
    return lhs % (rhs == 0u ? 1u : rhs);
}

int naga_f2i32(float value) {
    return int(clamp(value, -2147483600.0, 2147483500.0));
}

[numthreads(64, 1, 1)]
void conditionals(uint3 blockIdx : SV_GroupID, uint3 threadIdx : SV_GroupThreadID, uint local_invocation_index : SV_GroupIndex)
{
    if (local_invocation_index == 0) {
        shared_values = (int[64])0;
    }
    GroupMemoryBarrierWithGroupSync();
    uint n = (uint)0;
    uint i = (uint)0;
    uint base = (uint)0;
    bool choose_a = (bool)0;
    int local = (int)0;
    int a = (int)0;
    int b = (int)0;
    int local_1 = (int)0;
    int index_a = (int)0;
    int index_b = (int)0;
    int local_2 = (int)0;
    int local_3 = (int)0;
    int c = (int)0;
    int local_4 = (int)0;
    int condition = (int)0;
    uint lane = (uint)0;
    int local_value = (int)0;
    float local_5 = (float)0;
    uint local_6 = (uint)0;
    bool local_7 = (bool)0;
    int local_8 = (int)0;
    uint local_9 = (uint)0;
    int shift_value = (int)0;
    int local_10 = (int)0;
    uint local_11 = (uint)0;
    int value = (int)0;
    int other = (int)0;
    int local_12 = (int)0;
    int count = (int)0;
    int left = (int)0;
    int right = (int)0;

    uint _e5 = n_1.value;
    n = _e5;
    i = ((blockIdx.x * 64u) + threadIdx.x);
    uint _e16 = i;
    uint _e17 = n;
    if ((_e16 < _e17)) {
        uint _e21 = i;
        base = (_e21 * 25u);
        uint _e25 = i;
        choose_a = (naga_mod(_e25, 2u) == 0u);
        bool _e31 = choose_a;
        if (_e31) {
            uint _e32 = i;
            int _e37 = asint(input_a.Load(_e32*4));
            local = _e37;
        } else {
            uint _e34 = i;
            int _e38 = asint(input_b.Load(_e34*4));
            local = _e38;
        }
        int _e39 = local;
        uint _e29 = base;
        out_.Store(_e29*4, asuint(_e39));
        a = int(3);
        b = int(5);
        bool _e48 = choose_a;
        if (_e48) {
            int _e49 = a;
            a = asint(asuint(_e49) + asuint(int(1)));
            const int _e52 = shifted(_e49);
            local_1 = _e52;
        } else {
            int _e53 = b;
            b = asint(asuint(_e53) + asuint(int(1)));
            const int _e56 = shifted(_e53);
            local_1 = _e56;
        }
        int _e58 = local_1;
        uint _e45 = base;
        out_.Store((_e45 + 1u)*4, asuint(_e58));
        int _e64 = a;
        int _e66 = b;
        uint _e60 = base;
        out_.Store((_e60 + 2u)*4, asuint(asint(asuint(asint(asuint(_e64) * asuint(int(10)))) + asuint(_e66))));
        bool _e68 = choose_a;
        if (_e68) {
            int _e69 = a;
            a = asint(asuint(_e69) + asuint(int(1)));
            discard_(_e69);
        } else {
            int _e72 = b;
            b = asint(asuint(_e72) + asuint(int(1)));
            discard_(_e72);
        }
        int _e80 = a;
        int _e82 = b;
        uint _e76 = base;
        out_.Store((_e76 + 3u)*4, asuint(asint(asuint(asint(asuint(_e80) * asuint(int(10)))) + asuint(_e82))));
        index_a = int(0);
        index_b = int(0);
        bool _e88 = choose_a;
        if (_e88) {
            int _e89 = index_a;
            index_a = asint(asuint(_e89) + asuint(int(1)));
        } else {
            int _e92 = index_b;
            index_b = asint(asuint(_e92) + asuint(int(1)));
        }
        int _e100 = index_a;
        int _e102 = index_b;
        uint _e96 = base;
        out_.Store((_e96 + 4u)*4, asuint(asint(asuint(asint(asuint(_e100) * asuint(int(10)))) + asuint(_e102))));
        a = int(3);
        b = int(5);
        bool _e106 = choose_a;
        if (_e106) {
            a = int(7);
        } else {
            b = int(7);
        }
        int _e113 = a;
        int _e115 = b;
        uint _e109 = base;
        out_.Store((_e109 + 5u)*4, asuint(asint(asuint(asint(asuint(_e113) * asuint(int(10)))) + asuint(_e115))));
        bool _e117 = choose_a;
        if (_e117) {
            int _e119 = a;
            a = asint(asuint(_e119) + asuint(int(2)));
        } else {
            int _e121 = b;
            b = asint(asuint(_e121) + asuint(int(2)));
        }
        int _e128 = a;
        int _e130 = b;
        uint _e124 = base;
        out_.Store((_e124 + 6u)*4, asuint(asint(asuint(asint(asuint(_e128) * asuint(int(10)))) + asuint(_e130))));
        bool _e136 = choose_a;
        if (_e136) {
            int _e137 = a;
            a = asint(asuint(_e137) + asuint(int(1)));
            int _e144 = a;
            local_2 = _e144;
        } else {
            int _e140 = b;
            b = asint(asuint(_e140) + asuint(int(1)));
            int _e145 = b;
            local_2 = _e145;
        }
        int _e146 = local_2;
        uint _e133 = base;
        out_.Store((_e133 + 7u)*4, asuint(_e146));
        bool _e151 = choose_a;
        if (_e151) {
            int _e153 = a;
            a = asint(asuint(_e153) + asuint(int(1)));
            local_3 = _e153;
        } else {
            int _e156 = b;
            b = asint(asuint(_e156) + asuint(int(1)));
            local_3 = _e156;
        }
        int _e159 = local_3;
        uint _e148 = base;
        out_.Store((_e148 + 8u)*4, asuint(_e159));
        int _e165 = a;
        int _e167 = b;
        uint _e161 = base;
        out_.Store((_e161 + 9u)*4, asuint(asint(asuint(asint(asuint(_e165) * asuint(int(100)))) + asuint(_e167))));
        bool _e170 = choose_a;
        c = int(_e170);
        b = int(7);
        int _e177 = c;
        if ((_e177 != int(0))) {
            c = int(0);
            int _e182 = c;
            c = asint(asuint(_e182) + asuint(int(1)));
            local_4 = _e182;
        } else {
            b = int(0);
            int _e185 = b;
            b = asint(asuint(_e185) + asuint(int(1)));
            local_4 = _e185;
        }
        int _e188 = local_4;
        uint _e174 = base;
        out_.Store((_e174 + 10u)*4, asuint(_e188));
        int _e194 = c;
        int _e196 = b;
        uint _e190 = base;
        out_.Store((_e190 + 11u)*4, asuint(asint(asuint(asint(asuint(_e194) * asuint(int(10)))) + asuint(_e196))));
        condition = int(0);
        a = int(2);
        b = int(4);
        condition = int(1);
        int _e206 = condition;
        int _e202 = condition;
        if ((_e202 != int(0))) {
            a = _e206;
        } else {
            b = _e206;
        }
        int _e212 = a;
        int _e214 = b;
        uint _e208 = base;
        out_.Store((_e208 + 12u)*4, asuint(asint(asuint(asint(asuint(_e212) * asuint(int(10)))) + asuint(_e214))));
        a = int(2);
        b = int(4);
        c = int(6);
        bool _e219 = choose_a;
        if (_e219) {
            uint _e221 = i;
            if ((naga_mod(_e221, 4u) == 0u)) {
                int _e226 = a;
                a = asint(asuint(_e226) + asuint(int(3)));
            } else {
                int _e228 = b;
                b = asint(asuint(_e228) + asuint(int(3)));
            }
        } else {
            int _e230 = c;
            c = asint(asuint(_e230) + asuint(int(3)));
        }
        int _e237 = a;
        int _e240 = b;
        int _e243 = c;
        uint _e233 = base;
        out_.Store((_e233 + 13u)*4, asuint(asint(asuint(asint(asuint(asint(asuint(_e237) * asuint(int(100)))) + asuint(asint(asuint(_e240) * asuint(int(10)))))) + asuint(_e243))));
        lane = threadIdx.x;
        uint _e247 = lane;
        shared_values[min(uint(_e247), 63u)] = int(20);
        local_value = int(10);
        bool _e252 = choose_a;
        if (_e252) {
            int _e256 = local_value;
            local_value = asint(asuint(_e256) + asuint(int(5)));
        } else {
            uint _e253 = lane;
            int _e258 = shared_values[min(uint(_e253), 63u)];
            shared_values[min(uint(_e253), 63u)] = asint(asuint(_e258) + asuint(int(5)));
        }
        int _e265 = local_value;
        uint _e267 = lane;
        int _e269 = shared_values[min(uint(_e267), 63u)];
        uint _e261 = base;
        out_.Store((_e261 + 14u)*4, asuint(asint(asuint(asint(asuint(_e265) * asuint(int(100)))) + asuint(_e269))));
        bool _e271 = choose_a;
        if (_e271) {
            uint _e272 = lane;
            shared_values[min(uint(_e272), 63u)] = int(30);
        } else {
            uint _e275 = base;
            out_.Store((_e275 + 15u)*4, asuint(int(30)));
        }
        uint _e283 = lane;
        int _e285 = shared_values[min(uint(_e283), 63u)];
        uint _e280 = base;
        out_.Store((_e280 + 16u)*4, asuint(_e285));
        bool _e290 = choose_a;
        if (_e290) {
            local_5 = 1.5;
        } else {
            local_5 = float(int(3));
        }
        float _e295 = local_5;
        uint _e287 = base;
        out_.Store((_e287 + 17u)*4, asuint(naga_f2i32(_e295)));
        bool _e301 = choose_a;
        if (_e301) {
            local_6 = uint(naga_neg(int(1)));
        } else {
            local_6 = 2u;
        }
        uint _e307 = local_6;
        uint _e298 = base;
        out_.Store((_e298 + 18u)*4, asuint(int(_e307)));
        bool _e313 = choose_a;
        if (_e313) {
            local_7 = true;
        } else {
            local_7 = false;
        }
        bool _e317 = local_7;
        uint _e310 = base;
        out_.Store((_e310 + 19u)*4, asuint(int(_e317)));
        bool _e323 = choose_a;
        if (_e323) {
            local_8 = int(true);
        } else {
            local_8 = int(2);
        }
        int _e328 = local_8;
        uint _e320 = base;
        out_.Store((_e320 + 20u)*4, asuint(_e328));
        bool _e333 = choose_a;
        if (_e333) {
            local_9 = 4u;
        } else {
            local_9 = 8u;
        }
        uint _e337 = local_9;
        uint _e330 = base;
        out_.Store((_e330 + 21u)*4, asuint(int(_e337)));
        a = int(3);
        shift_value = int(2);
        bool _e346 = choose_a;
        if (_e346) {
            int _e348 = shift_value;
            local_10 = _e348;
        } else {
            int _e349 = a;
            local_10 = _e349;
        }
        int _e350 = local_10;
        shift_value = int(1);
        int _e352 = shift_value;
        local_11 = uint(_e352);
        uint _e355 = local_11;
        uint _e343 = base;
        out_.Store((_e343 + 22u)*4, asuint((_e350 << _e355)));
        value = int(3);
        other = int(7);
        bool _e365 = choose_a;
        if (_e365) {
            int _e367 = value;
            local_12 = _e367;
        } else {
            int _e368 = other;
            local_12 = _e368;
        }
        int _e369 = local_12;
        int _e370 = value;
        value = asint(asuint(_e370) + asuint(int(1)));
        const int _e373 = pair(_e369, _e370);
        uint _e362 = base;
        out_.Store((_e362 + 23u)*4, asuint(_e373));
        count = int(0);
        left = int(0);
        right = int(0);
        {
            uint2 loop_bound = uint2(4294967295u, 4294967295u);
            bool loop_init = true;
            while(true) {
                if (all(loop_bound == uint2(0u, 0u))) { break; }
                loop_bound -= uint2(loop_bound.y == 0u, 1u);
                if (!loop_init) {
                    bool _e383 = choose_a;
                    if (_e383) {
                        int _e384 = left;
                        left = asint(asuint(_e384) + asuint(int(1)));
                    } else {
                        int _e387 = right;
                        right = asint(asuint(_e387) + asuint(int(1)));
                    }
                }
                loop_init = false;
                int _e381 = count;
                if ((_e381 < int(3))) {
                } else {
                    break;
                }
                int _e390 = count;
                count = asint(asuint(_e390) + asuint(int(1)));
                int _e394 = count;
                if ((_e394 == int(2))) {
                    continue;
                }
            }
        }
        int _e401 = left;
        int _e404 = right;
        int _e407 = count;
        uint _e397 = base;
        out_.Store((_e397 + 24u)*4, asuint(asint(asuint(asint(asuint(asint(asuint(_e401) * asuint(int(100)))) + asuint(asint(asuint(_e404) * asuint(int(10)))))) + asuint(_e407))));
    }
}
