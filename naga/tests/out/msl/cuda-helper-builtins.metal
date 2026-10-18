// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

struct _mslBufferSizes {
    uint size3;
    uint size5;
    uint size7;
};

typedef int type_4[1];
struct type_5 {
    float value;
    char _pad1[12];
};
struct float3_ {
    float x;
    float y;
    float z;
};
typedef float3_ type_6[1];
struct type_7 {
    uint value;
    char _pad1[12];
};
typedef uint type_8[1];

uint thread_component(
    uint axis,
    thread metal::uint3& threadIdx_1
) {
    uint axis_1 = {};
    axis_1 = axis;
    uint _e3 = axis_1;
    if (_e3 == 0u) {
        metal::uint3 _e6 = threadIdx_1;
        return _e6.x;
    }
    uint _e9 = axis_1;
    if (_e9 == 1u) {
        metal::uint3 _e12 = threadIdx_1;
        return _e12.y;
    }
    metal::uint3 _e15 = threadIdx_1;
    return _e15.z;
}

uint group_component(
    uint axis_2,
    thread metal::uint3& blockIdx_1
) {
    uint axis_3 = {};
    axis_3 = axis_2;
    uint _e3 = axis_3;
    if (_e3 == 0u) {
        metal::uint3 _e6 = blockIdx_1;
        return _e6.x;
    }
    uint _e9 = axis_3;
    if (_e9 == 1u) {
        metal::uint3 _e12 = blockIdx_1;
        return _e12.y;
    }
    metal::uint3 _e15 = blockIdx_1;
    return _e15.z;
}

uint shape_component(
    uint axis_4
) {
    uint axis_5 = {};
    axis_5 = axis_4;
    uint _e3 = axis_5;
    if (_e3 == 0u) {
        return 1u;
    }
    uint _e7 = axis_5;
    if (_e7 == 1u) {
        return 1u;
    }
    return 1u;
}

uint grid_component(
    uint axis_6,
    thread metal::uint3& gridDim_1
) {
    uint axis_7 = {};
    axis_7 = axis_6;
    uint _e3 = axis_7;
    if (_e3 == 0u) {
        metal::uint3 _e6 = gridDim_1;
        return _e6.x;
    }
    uint _e9 = axis_7;
    if (_e9 == 1u) {
        metal::uint3 _e12 = gridDim_1;
        return _e12.y;
    }
    metal::uint3 _e15 = gridDim_1;
    return _e15.z;
}

uint component(
    uint axis_8,
    thread metal::uint3& threadIdx_1
) {
    uint axis_9 = {};
    axis_9 = axis_8;
    uint _e2 = axis_9;
    uint _e3 = thread_component(_e2, threadIdx_1);
    return _e3;
}

float component_1(
    float offset,
    thread metal::uint3& blockIdx_1
) {
    float offset_1 = {};
    offset_1 = offset;
    metal::uint3 _e3 = blockIdx_1;
    float _e6 = offset_1;
    return static_cast<float>(_e3.x) + _e6;
}

uint global_x(
    thread metal::uint3& threadIdx_1,
    thread metal::uint3& blockIdx_1
) {
    uint _e1 = group_component(0u, blockIdx_1);
    uint _e3 = shape_component(0u);
    uint _e6 = component(0u, threadIdx_1);
    return (_e1 * _e3) + _e6;
}

uint kind(
    uint value
) {
    uint value_1 = {};
    value_1 = value;
    return 11u;
}

void observe(
    thread metal::uint3& gridDim_1
) {
    metal::uint3 _e1 = gridDim_1;
    if (_e1.z != 0u) {
        return;
    } else {
        return;
    }
}

struct coordinatesInput {
};
kernel void coordinates(
  metal::uint3 threadIdx [[thread_position_in_threadgroup]]
, metal::uint3 blockIdx [[threadgroup_position_in_grid]]
, metal::uint3 gridDim [[threadgroups_per_grid]]
, device type_4 const& input [[user(fake0)]]
, constant type_5& scale_1 [[user(fake0)]]
, device type_6 const& points [[user(fake0)]]
, constant type_7& limit_1 [[user(fake0)]]
, device type_8& out [[user(fake0)]]
, constant _mslBufferSizes& _buffer_sizes [[user(fake0)]]
) {
    metal::uint3 threadIdx_1 = {};
    metal::uint3 blockIdx_1 = {};
    metal::uint3 gridDim_1 = {};
    float scale = {};
    uint limit = {};
    float _e3 = scale_1.value;
    scale = _e3;
    uint _e8 = limit_1.value;
    limit = _e8;
    threadIdx_1 = threadIdx;
    blockIdx_1 = blockIdx;
    gridDim_1 = gridDim;
    uint _e17 = global_x(threadIdx_1, blockIdx_1);
    uint _e18 = limit;
    if (_e17 >= _e18) {
        return;
    }
    observe(gridDim_1);
    uint _e23 = thread_component(0u, threadIdx_1);
    out[0] = _e23;
    uint _e27 = thread_component(1u, threadIdx_1);
    out[1] = _e27;
    uint _e31 = thread_component(2u, threadIdx_1);
    out[2] = _e31;
    uint _e35 = group_component(0u, blockIdx_1);
    out[3] = _e35;
    uint _e39 = group_component(1u, blockIdx_1);
    out[4] = _e39;
    uint _e43 = group_component(2u, blockIdx_1);
    out[5] = _e43;
    uint _e47 = shape_component(0u);
    out[6] = _e47;
    uint _e51 = shape_component(1u);
    out[7] = _e51;
    uint _e55 = shape_component(2u);
    out[8] = _e55;
    uint _e59 = grid_component(0u, gridDim_1);
    out[9] = _e59;
    uint _e63 = grid_component(1u, gridDim_1);
    out[10] = _e63;
    uint _e67 = grid_component(2u, gridDim_1);
    out[11] = _e67;
    uint _e70 = global_x(threadIdx_1, blockIdx_1);
    out[12] = _e70;
    uint _e74 = component(0u, threadIdx_1);
    out[13] = _e74;
    float _e78 = component_1(2.5, blockIdx_1);
    out[14] = as_type<uint>(_e78);
    out[15] = threadIdx.x;
    out[16] = blockIdx.y;
    out[17] = gridDim.z;
    out[18] = 1u;
    int _e96 = input[0];
    out[19] = static_cast<uint>(_e96);
    float _e100 = scale;
    out[20] = as_type<uint>(_e100);
    float _e107 = points[0].x;
    out[21] = as_type<uint>(_e107);
    float _e114 = points[0].y;
    out[22] = as_type<uint>(_e114);
    float _e121 = points[0].z;
    out[23] = as_type<uint>(_e121);
    uint _e125 = limit;
    out[24] = _e125;
    uint _e129 = thread_component(0u, threadIdx_1);
    uint _e131 = group_component(1u, blockIdx_1);
    uint _e134 = shape_component(2u);
    uint _e137 = grid_component(0u, gridDim_1);
    out[25] = ((_e129 + _e131) + _e134) + _e137;
    uint _e141 = global_x(threadIdx_1, blockIdx_1);
    uint _e142 = global_x(threadIdx_1, blockIdx_1);
    out[26] = _e141 + _e142;
    int _e148 = input[0];
    float _e150 = scale;
    float _e155 = points[0].x;
    out[27] = as_type<uint>((static_cast<float>(_e148) * _e150) + _e155);
    uint _e161 = kind(threadIdx.x);
    out[28] = _e161;
    uint _e165 = grid_component(2u, gridDim_1);
    out[29] = _e165;
}
