#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct type_2 {
    int value;
};
layout(std430) buffer type_1_block_0Compute { int _group_0_binding_0_cs[]; };

layout(std140) uniform type_2_block_1Compute { type_2 _group_0_binding_1_cs; };


void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    int n = 0;
    int sum = 0;
    int i = 0;
    int i_1 = 0;
    int _e3 = _group_0_binding_1_cs.value;
    n = _e3;
    sum = 0;
    {
        i = 0;
        bool loop_init = true;
        while(true) {
            if (!loop_init) {
                int _e13 = i;
                i = (_e13 + 1);
            }
            loop_init = false;
            int _e9 = i;
            int _e10 = n;
            if ((_e9 < _e10)) {
            } else {
                break;
            }
            int _e16 = i;
            if ((_e16 == 2)) {
                continue;
            }
            int _e19 = i;
            if ((_e19 == 7)) {
                break;
            }
            int _e21 = sum;
            int _e22 = i;
            sum = (_e21 + _e22);
        }
    }
    i_1 = 0;
    while(true) {
        int _e27 = i_1;
        if ((_e27 < 3)) {
        } else {
            break;
        }
        {
            int _e30 = i_1;
            i_1 = (_e30 + 1);
            int _e33 = i_1;
            if ((_e33 == 2)) {
                continue;
            }
            int _e36 = sum;
            sum = (_e36 + 10);
        }
    }
    bool loop_init_1 = true;
    while(true) {
        if (!loop_init_1) {
            int _e45 = sum;
            if (!((_e45 < 150))) {
                break;
            }
        }
        loop_init_1 = false;
        int _e39 = sum;
        sum = (_e39 + 100);
        int _e42 = sum;
        if ((_e42 > 200)) {
            break;
        }
    }
    int _e51 = sum;
    _group_0_binding_0_cs[threadIdx.x] = _e51;
}

