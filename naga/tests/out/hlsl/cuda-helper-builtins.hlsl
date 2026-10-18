struct NagaConstants {
    int first_vertex;
    int first_instance;
    uint other;
};
ConstantBuffer<NagaConstants> _NagaConstants: register(b0, space1);

struct type_5 {
    float value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

struct float3_ {
    float x;
    float y;
    float z;
};

struct type_7 {
    uint value;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

static uint3 threadIdx_1 = (uint3)0;
static uint3 blockIdx_1 = (uint3)0;
static uint3 gridDim_1 = (uint3)0;
ByteAddressBuffer input : register(t0);
cbuffer scale_1 : register(b1) { type_5 scale_1; }
ByteAddressBuffer points : register(t2);
cbuffer limit_1 : register(b3) { type_7 limit_1; }
RWByteAddressBuffer out_ : register(u4);

uint thread_component(uint axis)
{
    uint axis_1 = (uint)0;

    axis_1 = axis;
    uint _e3 = axis_1;
    if ((_e3 == 0u)) {
        uint3 _e6 = threadIdx_1;
        return _e6.x;
    }
    uint _e9 = axis_1;
    if ((_e9 == 1u)) {
        uint3 _e12 = threadIdx_1;
        return _e12.y;
    }
    uint3 _e15 = threadIdx_1;
    return _e15.z;
}

uint group_component(uint axis_2)
{
    uint axis_3 = (uint)0;

    axis_3 = axis_2;
    uint _e3 = axis_3;
    if ((_e3 == 0u)) {
        uint3 _e6 = blockIdx_1;
        return _e6.x;
    }
    uint _e9 = axis_3;
    if ((_e9 == 1u)) {
        uint3 _e12 = blockIdx_1;
        return _e12.y;
    }
    uint3 _e15 = blockIdx_1;
    return _e15.z;
}

uint shape_component(uint axis_4)
{
    uint axis_5 = (uint)0;

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

uint grid_component(uint axis_6)
{
    uint axis_7 = (uint)0;

    axis_7 = axis_6;
    uint _e3 = axis_7;
    if ((_e3 == 0u)) {
        uint3 _e6 = gridDim_1;
        return _e6.x;
    }
    uint _e9 = axis_7;
    if ((_e9 == 1u)) {
        uint3 _e12 = gridDim_1;
        return _e12.y;
    }
    uint3 _e15 = gridDim_1;
    return _e15.z;
}

uint component(uint axis_8)
{
    uint axis_9 = (uint)0;

    axis_9 = axis_8;
    uint _e2 = axis_9;
    const uint _e3 = thread_component(_e2);
    return _e3;
}

float component_1(float offset)
{
    float offset_1 = (float)0;

    offset_1 = offset;
    uint3 _e3 = blockIdx_1;
    float _e6 = offset_1;
    return (float(_e3.x) + _e6);
}

uint global_x()
{
    const uint _e1 = group_component(0u);
    const uint _e3 = shape_component(0u);
    const uint _e6 = component(0u);
    return ((_e1 * _e3) + _e6);
}

uint kind(uint value)
{
    uint value_1 = (uint)0;

    value_1 = value;
    return 11u;
}

void observe()
{
    uint3 _e1 = gridDim_1;
    if ((_e1.z != 0u)) {
        return;
    } else {
        return;
    }
}

[numthreads(1, 1, 1)]
void coordinates(uint3 threadIdx : SV_GroupThreadID, uint3 blockIdx : SV_GroupID, uint3 gridDim : SV_GroupID)
{
    float scale = (float)0;
    uint limit = (uint)0;

    float _e3 = scale_1.value;
    scale = _e3;
    uint _e8 = limit_1.value;
    limit = _e8;
    threadIdx_1 = threadIdx;
    blockIdx_1 = blockIdx;
    gridDim_1 = uint3(_NagaConstants.first_vertex, _NagaConstants.first_instance, _NagaConstants.other);
    const uint _e17 = global_x();
    uint _e18 = limit;
    if ((_e17 >= _e18)) {
        return;
    }
    observe();
    const uint _e23 = thread_component(0u);
    out_.Store(int(0)*4, asuint(_e23));
    const uint _e27 = thread_component(1u);
    out_.Store(int(1)*4, asuint(_e27));
    const uint _e31 = thread_component(2u);
    out_.Store(int(2)*4, asuint(_e31));
    const uint _e35 = group_component(0u);
    out_.Store(int(3)*4, asuint(_e35));
    const uint _e39 = group_component(1u);
    out_.Store(int(4)*4, asuint(_e39));
    const uint _e43 = group_component(2u);
    out_.Store(int(5)*4, asuint(_e43));
    const uint _e47 = shape_component(0u);
    out_.Store(int(6)*4, asuint(_e47));
    const uint _e51 = shape_component(1u);
    out_.Store(int(7)*4, asuint(_e51));
    const uint _e55 = shape_component(2u);
    out_.Store(int(8)*4, asuint(_e55));
    const uint _e59 = grid_component(0u);
    out_.Store(int(9)*4, asuint(_e59));
    const uint _e63 = grid_component(1u);
    out_.Store(int(10)*4, asuint(_e63));
    const uint _e67 = grid_component(2u);
    out_.Store(int(11)*4, asuint(_e67));
    const uint _e70 = global_x();
    out_.Store(int(12)*4, asuint(_e70));
    const uint _e74 = component(0u);
    out_.Store(int(13)*4, asuint(_e74));
    const float _e78 = component_1(2.5);
    out_.Store(int(14)*4, asuint(asuint(_e78)));
    out_.Store(int(15)*4, asuint(threadIdx.x));
    out_.Store(int(16)*4, asuint(blockIdx.y));
    out_.Store(int(17)*4, asuint(uint3(_NagaConstants.first_vertex, _NagaConstants.first_instance, _NagaConstants.other).z));
    out_.Store(int(18)*4, asuint(1u));
    int _e96 = asint(input.Load(int(0)*4));
    out_.Store(int(19)*4, asuint(uint(_e96)));
    float _e100 = scale;
    out_.Store(int(20)*4, asuint(asuint(_e100)));
    float _e107 = asfloat(points.Load(0+int(0)*12));
    out_.Store(int(21)*4, asuint(asuint(_e107)));
    float _e114 = asfloat(points.Load(4+int(0)*12));
    out_.Store(int(22)*4, asuint(asuint(_e114)));
    float _e121 = asfloat(points.Load(8+int(0)*12));
    out_.Store(int(23)*4, asuint(asuint(_e121)));
    uint _e125 = limit;
    out_.Store(int(24)*4, asuint(_e125));
    const uint _e129 = thread_component(0u);
    const uint _e131 = group_component(1u);
    const uint _e134 = shape_component(2u);
    const uint _e137 = grid_component(0u);
    out_.Store(int(25)*4, asuint((((_e129 + _e131) + _e134) + _e137)));
    const uint _e141 = global_x();
    const uint _e142 = global_x();
    out_.Store(int(26)*4, asuint((_e141 + _e142)));
    int _e148 = asint(input.Load(int(0)*4));
    float _e150 = scale;
    float _e155 = asfloat(points.Load(0+int(0)*12));
    out_.Store(int(27)*4, asuint(asuint(((float(_e148) * _e150) + _e155))));
    const uint _e161 = kind(threadIdx.x);
    out_.Store(int(28)*4, asuint(_e161));
    const uint _e165 = grid_component(2u);
    out_.Store(int(29)*4, asuint(_e165));
}
