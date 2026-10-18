#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct type_5 {
    float value;
};
struct float3_ {
    float x;
    float y;
    float z;
};
struct type_7 {
    uint value;
};
uvec3 threadIdx_1 = uvec3(0u);

uvec3 blockIdx_1 = uvec3(0u);

uvec3 gridDim_1 = uvec3(0u);

layout(std430) readonly buffer type_4_block_0Compute { int _group_0_binding_0_cs[]; };

layout(std140) uniform type_5_block_1Compute { type_5 _group_0_binding_1_cs; };

layout(std430) readonly buffer type_6_block_2Compute { float3_ _group_0_binding_2_cs[]; };

layout(std140) uniform type_7_block_3Compute { type_7 _group_0_binding_3_cs; };

layout(std430) buffer type_8_block_4Compute { uint _group_0_binding_4_cs[]; };


uint thread_component(uint axis) {
    uint axis_1 = 0u;
    axis_1 = axis;
    uint _e3 = axis_1;
    if ((_e3 == 0u)) {
        uvec3 _e6 = threadIdx_1;
        return _e6.x;
    }
    uint _e9 = axis_1;
    if ((_e9 == 1u)) {
        uvec3 _e12 = threadIdx_1;
        return _e12.y;
    }
    uvec3 _e15 = threadIdx_1;
    return _e15.z;
}

uint group_component(uint axis_2) {
    uint axis_3 = 0u;
    axis_3 = axis_2;
    uint _e3 = axis_3;
    if ((_e3 == 0u)) {
        uvec3 _e6 = blockIdx_1;
        return _e6.x;
    }
    uint _e9 = axis_3;
    if ((_e9 == 1u)) {
        uvec3 _e12 = blockIdx_1;
        return _e12.y;
    }
    uvec3 _e15 = blockIdx_1;
    return _e15.z;
}

uint shape_component(uint axis_4) {
    uint axis_5 = 0u;
    axis_5 = axis_4;
    uint _e3 = axis_5;
    if ((_e3 == 0u)) {
        return 1u;
    }
    uint _e7 = axis_5;
    if ((_e7 == 1u)) {
        return 1u;
    }
    return 1u;
}

uint grid_component(uint axis_6) {
    uint axis_7 = 0u;
    axis_7 = axis_6;
    uint _e3 = axis_7;
    if ((_e3 == 0u)) {
        uvec3 _e6 = gridDim_1;
        return _e6.x;
    }
    uint _e9 = axis_7;
    if ((_e9 == 1u)) {
        uvec3 _e12 = gridDim_1;
        return _e12.y;
    }
    uvec3 _e15 = gridDim_1;
    return _e15.z;
}

uint component(uint axis_8) {
    uint axis_9 = 0u;
    axis_9 = axis_8;
    uint _e2 = axis_9;
    uint _e3 = thread_component(_e2);
    return _e3;
}

float component_1(float offset) {
    float offset_1 = 0.0;
    offset_1 = offset;
    uvec3 _e3 = blockIdx_1;
    float _e6 = offset_1;
    return (float(_e3.x) + _e6);
}

uint global_x() {
    uint _e1 = group_component(0u);
    uint _e3 = shape_component(0u);
    uint _e6 = component(0u);
    return ((_e1 * _e3) + _e6);
}

uint kind(uint value) {
    uint value_1 = 0u;
    value_1 = value;
    return 11u;
}

void observe() {
    uvec3 _e1 = gridDim_1;
    if ((_e1.z != 0u)) {
        return;
    } else {
        return;
    }
}

void main() {
    uvec3 threadIdx = gl_LocalInvocationID;
    uvec3 blockIdx = gl_WorkGroupID;
    uvec3 gridDim = gl_NumWorkGroups;
    float scale = 0.0;
    uint limit = 0u;
    float _e3 = _group_0_binding_1_cs.value;
    scale = _e3;
    uint _e8 = _group_0_binding_3_cs.value;
    limit = _e8;
    threadIdx_1 = threadIdx;
    blockIdx_1 = blockIdx;
    gridDim_1 = gridDim;
    uint _e17 = global_x();
    uint _e18 = limit;
    if ((_e17 >= _e18)) {
        return;
    }
    observe();
    uint _e23 = thread_component(0u);
    _group_0_binding_4_cs[0] = _e23;
    uint _e27 = thread_component(1u);
    _group_0_binding_4_cs[1] = _e27;
    uint _e31 = thread_component(2u);
    _group_0_binding_4_cs[2] = _e31;
    uint _e35 = group_component(0u);
    _group_0_binding_4_cs[3] = _e35;
    uint _e39 = group_component(1u);
    _group_0_binding_4_cs[4] = _e39;
    uint _e43 = group_component(2u);
    _group_0_binding_4_cs[5] = _e43;
    uint _e47 = shape_component(0u);
    _group_0_binding_4_cs[6] = _e47;
    uint _e51 = shape_component(1u);
    _group_0_binding_4_cs[7] = _e51;
    uint _e55 = shape_component(2u);
    _group_0_binding_4_cs[8] = _e55;
    uint _e59 = grid_component(0u);
    _group_0_binding_4_cs[9] = _e59;
    uint _e63 = grid_component(1u);
    _group_0_binding_4_cs[10] = _e63;
    uint _e67 = grid_component(2u);
    _group_0_binding_4_cs[11] = _e67;
    uint _e70 = global_x();
    _group_0_binding_4_cs[12] = _e70;
    uint _e74 = component(0u);
    _group_0_binding_4_cs[13] = _e74;
    float _e78 = component_1(2.5);
    _group_0_binding_4_cs[14] = floatBitsToUint(_e78);
    _group_0_binding_4_cs[15] = threadIdx.x;
    _group_0_binding_4_cs[16] = blockIdx.y;
    _group_0_binding_4_cs[17] = gridDim.z;
    _group_0_binding_4_cs[18] = 1u;
    int _e96 = _group_0_binding_0_cs[0];
    _group_0_binding_4_cs[19] = uint(_e96);
    float _e100 = scale;
    _group_0_binding_4_cs[20] = floatBitsToUint(_e100);
    float _e107 = _group_0_binding_2_cs[0].x;
    _group_0_binding_4_cs[21] = floatBitsToUint(_e107);
    float _e114 = _group_0_binding_2_cs[0].y;
    _group_0_binding_4_cs[22] = floatBitsToUint(_e114);
    float _e121 = _group_0_binding_2_cs[0].z;
    _group_0_binding_4_cs[23] = floatBitsToUint(_e121);
    uint _e125 = limit;
    _group_0_binding_4_cs[24] = _e125;
    uint _e129 = thread_component(0u);
    uint _e131 = group_component(1u);
    uint _e134 = shape_component(2u);
    uint _e137 = grid_component(0u);
    _group_0_binding_4_cs[25] = (((_e129 + _e131) + _e134) + _e137);
    uint _e141 = global_x();
    uint _e142 = global_x();
    _group_0_binding_4_cs[26] = (_e141 + _e142);
    int _e148 = _group_0_binding_0_cs[0];
    float _e150 = scale;
    float _e155 = _group_0_binding_2_cs[0].x;
    _group_0_binding_4_cs[27] = floatBitsToUint(((float(_e148) * _e150) + _e155));
    uint _e161 = kind(threadIdx.x);
    _group_0_binding_4_cs[28] = _e161;
    uint _e165 = grid_component(2u);
    _group_0_binding_4_cs[29] = _e165;
}

