#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct float3_ {
    float x;
    float y;
    float z;
};
layout(std430) readonly buffer type_2_block_0Compute { float3_ _group_0_binding_0_cs[]; };

layout(std430) buffer type_2_block_1Compute { float3_ _group_0_binding_1_cs[]; };


float3_ shift(float3_ value) {
    float3_ value_1 = float3_(0.0, 0.0, 0.0);
    value_1 = value;
    float _e4 = value_1.x;
    value_1.x = (_e4 + 1.0);
    float _e9 = value_1.y;
    float _e10 = value_1.z;
    value_1.z = (_e9 + _e10);
    float3_ _e12 = value_1;
    return _e12;
}

float pair(float3_ value_2, int digit) {
    float3_ value_3 = float3_(0.0, 0.0, 0.0);
    int digit_1 = 0;
    value_3 = value_2;
    digit_1 = digit;
    float _e6 = value_3.x;
    int _e8 = digit_1;
    return ((_e6 * 10.0) + float(_e8));
}

void main() {
    int i = 0;
    float3_ copied = float3_(0.0, 0.0, 0.0);
    int index = 0;
    bool choose_first = false;
    float local = 0.0;
    float local_1 = 0.0;
    float3_ local_2 = float3_(0.0, 0.0, 0.0);
    bool pick_local = false;
    float3_ snapshot = float3_(0.0, 0.0, 0.0);
    {
        i = 0;
        bool loop_init = true;
        while(true) {
            if (!loop_init) {
                int _e7 = i;
                i = (_e7 + 1);
            }
            loop_init = false;
            int _e5 = i;
            if ((_e5 < 3)) {
            } else {
                break;
            }
            int _e11 = i;
            float3_ _e13 = _group_0_binding_0_cs[_e11];
            copied = _e13;
            float3_ _e16 = copied;
            float3_ _e17 = shift(_e16);
            int _e14 = i;
            _group_0_binding_1_cs[_e14] = _e17;
        }
    }
    float3_ _e22 = _group_0_binding_0_cs[0];
    _group_0_binding_1_cs[3] = _e22;
    float3_ _e28 = _group_0_binding_0_cs[1];
    float3_ _e29 = shift(_e28);
    _group_0_binding_1_cs[3].y = _e29.x;
    index = 0;
    index = 1;
    float _e42 = _group_0_binding_0_cs[2].x;
    int _e33 = index;
    index = (_e33 + 1);
    _group_0_binding_1_cs[_e33].x = _e42;
    int _e45 = index;
    float _e49 = _group_0_binding_0_cs[0].x;
    float _e53 = _group_0_binding_0_cs[1].x;
    _group_0_binding_1_cs[4] = float3_(float(_e45), _e49, _e53);
    choose_first = true;
    bool _e58 = choose_first;
    if (_e58) {
        float _e66 = _group_0_binding_1_cs[0].y;
        _group_0_binding_1_cs[0].y = (_e66 + 2.0);
    } else {
        float _e68 = _group_0_binding_1_cs[2].y;
        _group_0_binding_1_cs[2].y = (_e68 + 2.0);
    }
    bool _e73 = choose_first;
    if (_e73) {
        float _e81 = _group_0_binding_1_cs[0].z;
        _group_0_binding_1_cs[0].z = (_e81 + float(1));
        local = _e81;
    } else {
        float _e85 = _group_0_binding_1_cs[2].z;
        _group_0_binding_1_cs[2].z = (_e85 + float(1));
        local = _e85;
    }
    float _e89 = local;
    _group_0_binding_1_cs[4].x = _e89;
    int _e93 = index;
    _group_0_binding_1_cs[4].z = float(_e93);
    float3_ _e102 = _group_0_binding_0_cs[1];
    bool _e95 = choose_first;
    if (_e95) {
        _group_0_binding_1_cs[0] = _e102;
        float _e106 = _group_0_binding_1_cs[0].x;
        _group_0_binding_1_cs[0].x = (_e106 + float(1));
        local_1 = _e106;
    } else {
        _group_0_binding_1_cs[2] = _e102;
        float _e110 = _group_0_binding_1_cs[2].x;
        _group_0_binding_1_cs[2].x = (_e110 + float(1));
        local_1 = _e110;
    }
    float3_ _e118 = _group_0_binding_1_cs[0];
    _group_0_binding_1_cs[5] = _e118;
    local_2 = float3_(float(20), float(30), float(40));
    pick_local = false;
    pick_local = true;
    bool _e129 = pick_local;
    if (_e129) {
        local_2.z = 7.0;
    } else {
        _group_0_binding_1_cs[1].z = 7.0;
    }
    float3_ _e138 = local_2;
    _group_0_binding_1_cs[6] = _e138;
    float _e145 = _group_0_binding_1_cs[1].z;
    _group_0_binding_1_cs[6].x = _e145;
    float _e151 = _group_0_binding_1_cs[0].x;
    float _e155 = _group_0_binding_1_cs[1].x;
    float _e159 = _group_0_binding_1_cs[2].x;
    _group_0_binding_1_cs[7] = float3_(_e151, _e155, _e159);
    float3_ _e164 = _group_0_binding_0_cs[0];
    snapshot = _e164;
    float3_ _e167 = snapshot;
    snapshot.x = 9.0;
    float _e171 = pair(_e167, 4);
    float _e173 = snapshot.x;
    float _e177 = _group_0_binding_0_cs[2].z;
    _group_0_binding_1_cs[8] = float3_(_e171, _e173, _e177);
}

