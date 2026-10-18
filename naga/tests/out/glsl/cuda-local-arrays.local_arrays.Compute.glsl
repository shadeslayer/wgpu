#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(std430) buffer type_3_block_0Compute { int _group_0_binding_0_cs[]; };


int local_sum(int value) {
    int value_1 = 0;
    int values[3] = int[3](0, 0, 0);
    value_1 = value;
    values = int[3](0, 0, 0);
    int _e6 = value_1;
    values[0] = _e6;
    int _e10 = value_1;
    values[1] = (_e10 + 1);
    int _e18 = values[0];
    int _e19 = values[1];
    values[2] = (_e18 + _e19);
    int _e23 = values[2];
    return _e23;
}

int reset_sum(int count) {
    int count_1 = 0;
    int result = 0;
    int i_1 = 0;
    int values_1[2] = int[2](0, 0);
    count_1 = count;
    result = 0;
    {
        i_1 = 0;
        bool loop_init = true;
        while(true) {
            if (!loop_init) {
                int _e9 = i_1;
                i_1 = (_e9 + 1);
            }
            loop_init = false;
            int _e6 = i_1;
            int _e7 = count_1;
            if ((_e6 < _e7)) {
            } else {
                break;
            }
            values_1 = int[2](0, 0);
            int _e16 = i_1;
            values_1[1] = _e16;
            int _e21 = values_1[0];
            int _e22 = values_1[1];
            int _e24 = result;
            result = (_e24 + (_e21 + _e22));
            values_1[0] = 99;
        }
    }
    int _e29 = result;
    return _e29;
}

void main() {
    int seed = 0;
    int signed_values[3] = int[3](0, 0, 0);
    uint words[2] = uint[2](0u, 0u);
    float samples[2] = float[2](0.0, 0.0);
    bool flags[3] = bool[3](false, false, false);
    int zeros[2] = int[2](0, 0);
    bool zero_flags[2] = bool[2](false, false);
    bool local = false;
    uint bank[2] = uint[2](0u, 0u);
    float bank_1[2] = float[2](0.0, 0.0);
    int signed_values_1[2] = int[2](0, 0);
    int mixed[2] = int[2](0, 0);
    int next = 0;
    int reset_total = 0;
    int i = 0;
    int reset[2] = int[2](0, 0);
    int total = 0;
    int scratch[2] = int[2](0, 0);
    int j = 0;
    float zero_samples[2] = float[2](0.0, 0.0);
    int left[2] = int[2](0, 0);
    int right[2] = int[2](0, 0);
    int temporary[2] = int[2](0, 0);
    int temporary_1[2] = int[2](0, 0);
    bool choose = false;
    int index = 0;
    int rhs = 0;
    int local_1 = 0;
    int local_2 = 0;
    int _e4 = _group_0_binding_0_cs[0];
    seed = _e4;
    int _e8 = seed;
    signed_values[0] = _e8;
    signed_values[1] = -(2);
    signed_values[2] = 4;
    int _e22 = signed_values[0];
    int _e23 = signed_values[1];
    int _e27 = signed_values[2];
    _group_0_binding_0_cs[0] = ((_e22 + _e23) + _e27);
    words = uint[2](0u, 0u);
    words[0] = 4294967295u;
    words[1] = 1u;
    uint _e43 = words[0];
    uint _e44 = words[1];
    _group_0_binding_0_cs[1] = int(((_e43 + _e44) == 0u));
    samples = float[2](0.0, 0.0);
    samples[0] = 1.5;
    samples[1] = 2.25;
    float _e63 = samples[0];
    float _e64 = samples[1];
    _group_0_binding_0_cs[2] = int(((_e63 + _e64) * 4.0));
    flags = bool[3](false, false, false);
    int _e74 = seed;
    flags[0] = (_e74 != 0);
    flags[2] = true;
    bool _e83 = flags[0];
    bool _e89 = flags[1];
    bool _e96 = flags[2];
    _group_0_binding_0_cs[3] = (((int(_e83) * 100) + (int(_e89) * 10)) + int(_e96));
    zeros = int[2](0, 0);
    int _e107 = zeros[0];
    int _e108 = zeros[1];
    _group_0_binding_0_cs[4] = (_e107 + _e108);
    int _e112 = seed;
    int _e113 = local_sum(_e112);
    _group_0_binding_0_cs[5] = _e113;
    int _e117 = reset_sum(3);
    _group_0_binding_0_cs[6] = _e117;
    zero_flags = bool[2](false, false);
    bool _e124 = zero_flags[0];
    local = _e124;
    if (_e124) {
    } else {
        bool _e128 = zero_flags[1];
        local = _e128;
    }
    bool _e129 = local;
    _group_0_binding_0_cs[7] = int(_e129);
    bank = uint[2](0u, 0u);
    bank[0] = 3u;
    bank[1] = 4u;
    {
        bank_1 = float[2](0.0, 0.0);
        bank_1[1] = 6.5;
        float _e150 = bank_1[0];
        float _e151 = bank_1[1];
        _group_0_binding_0_cs[8] = int((_e150 + _e151));
    }
    uint _e160 = bank[0];
    uint _e161 = bank[1];
    _group_0_binding_0_cs[9] = int((_e160 + _e161));
    {
        int _e167 = seed;
        signed_values_1[0] = _e167;
        int _e170 = seed;
        signed_values_1[1] = _e170;
        int _e177 = signed_values_1[0];
        int _e178 = signed_values_1[1];
        _group_0_binding_0_cs[10] = (_e177 + _e178);
    }
    int _e184 = signed_values[0];
    _group_0_binding_0_cs[11] = _e184;
    mixed = int[2](0, 0);
    int _e188 = seed;
    seed = (_e188 + 1);
    int _e191 = seed;
    next = _e191;
    int _e194 = next;
    mixed[0] = _e194;
    int _e197 = seed;
    seed = (_e197 + 1);
    mixed[1] = _e197;
    int _e205 = mixed[0];
    int _e209 = mixed[1];
    _group_0_binding_0_cs[12] = ((_e205 * 10) + _e209);
    int _e213 = seed;
    _group_0_binding_0_cs[13] = _e213;
    reset_total = 0;
    {
        i = 0;
        bool loop_init_1 = true;
        while(true) {
            if (!loop_init_1) {
                int _e221 = i;
                i = (_e221 + 1);
            }
            loop_init_1 = false;
            int _e219 = i;
            if ((_e219 < 3)) {
            } else {
                break;
            }
            reset = int[2](0, 0);
            int _e227 = i;
            if ((_e227 == 1)) {
                reset[0] = 90;
                continue;
            }
            int _e234 = reset[0];
            int _e235 = i;
            int _e237 = reset_total;
            reset_total = (_e237 + (_e234 + _e235));
            reset[0] = 99;
        }
    }
    int _e244 = reset_total;
    _group_0_binding_0_cs[14] = _e244;
    total = 0;
    {
        scratch = int[2](0, 0);
        j = 0;
        bool loop_init_2 = true;
        while(true) {
            if (!loop_init_2) {
                int _e254 = j;
                j = (_e254 + 1);
            }
            loop_init_2 = false;
            int _e252 = j;
            if ((_e252 < 3)) {
            } else {
                break;
            }
            int _e258 = j;
            int _e262 = scratch[(_e258 - 2 * (_e258 / 2))];
            scratch[(_e258 - 2 * (_e258 / 2))] = (_e262 + 1);
            int _e265 = j;
            if ((_e265 == 1)) {
                continue;
            }
            int _e270 = scratch[0];
            int _e274 = scratch[1];
            int _e276 = total;
            total = (_e276 + ((_e270 * 10) + _e274));
        }
    }
    int _e280 = total;
    _group_0_binding_0_cs[15] = _e280;
    zero_samples = float[2](0.0, 0.0);
    float _e289 = zero_samples[0];
    float _e290 = zero_samples[1];
    _group_0_binding_0_cs[16] = int((_e289 + _e290));
    left = int[2](0, 0);
    right = int[2](0, 0);
    left[0] = 5;
    left[1] = 7;
    right[0] = 11;
    right[1] = 13;
    int _e315 = left[0];
    int _e316 = right[0];
    _group_0_binding_0_cs[17] = (_e315 + _e316);
    {
        temporary = int[2](0, 0);
        int _e322 = seed;
        temporary[0] = _e322;
        {
            temporary_1 = int[2](0, 0);
            temporary_1[1] = 3;
            int _e334 = temporary_1[0];
            int _e335 = temporary_1[1];
            _group_0_binding_0_cs[18] = (_e334 + _e335);
        }
        int _e341 = temporary[0];
        _group_0_binding_0_cs[19] = _e341;
    }
    int _e344 = seed;
    choose = ((_e344 & 1) == 0);
    index = 0;
    rhs = 20;
    int _e363 = rhs;
    rhs = (_e363 + 1);
    int _e366 = rhs;
    bool _e354 = choose;
    if (_e354) {
        int _e355 = index;
        index = (_e355 + 1);
        int _e367 = left[_e355];
        left[_e355] = (_e367 + _e366);
        int _e372 = left[_e355];
        local_1 = _e372;
    } else {
        int _e359 = index;
        index = (_e359 + 1);
        int _e369 = right[_e359];
        right[_e359] = (_e369 + _e366);
        int _e373 = right[_e359];
        local_1 = _e373;
    }
    int _e374 = local_1;
    _group_0_binding_0_cs[20] = _e374;
    int _e377 = index;
    _group_0_binding_0_cs[21] = _e377;
    int _e383 = left[0];
    int _e387 = right[0];
    _group_0_binding_0_cs[22] = ((_e383 * 100) + _e387);
    bool _e391 = choose;
    if (_e391) {
        right[1] = 9;
        int _e398 = right[1];
        right[1] = (_e398 + 1);
        local_2 = _e398;
    } else {
        left[1] = 9;
        int _e401 = left[1];
        left[1] = (_e401 + 1);
        local_2 = _e401;
    }
    int _e404 = local_2;
    _group_0_binding_0_cs[23] = _e404;
    int _e410 = left[1];
    int _e414 = right[1];
    _group_0_binding_0_cs[24] = ((_e410 * 100) + _e414);
}

