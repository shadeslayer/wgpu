struct type_5 {
    value: f32,
}

struct float3_ {
    x: f32,
    y: f32,
    z: f32,
}

struct type_7 {
    value: u32,
}

var<private> threadIdx_1: vec3<u32>;
var<private> blockIdx_1: vec3<u32>;
var<private> gridDim_1: vec3<u32>;
@group(0) @binding(0)
var<storage> input: array<i32>;
@group(0) @binding(1)
var<uniform> scale_1: type_5;
@group(0) @binding(2)
var<storage> points: array<float3_>;
@group(0) @binding(3)
var<uniform> limit_1: type_7;
@group(0) @binding(4)
var<storage, read_write> out: array<u32>;

fn thread_component(axis: u32) -> u32 {
    var axis_1: u32;

    axis_1 = axis;
    let _e3 = axis_1;
    if (_e3 == 0u) {
        let _e6 = threadIdx_1;
        return _e6.x;
    }
    let _e9 = axis_1;
    if (_e9 == 1u) {
        let _e12 = threadIdx_1;
        return _e12.y;
    }
    let _e15 = threadIdx_1;
    return _e15.z;
}

fn group_component(axis_2: u32) -> u32 {
    var axis_3: u32;

    axis_3 = axis_2;
    let _e3 = axis_3;
    if (_e3 == 0u) {
        let _e6 = blockIdx_1;
        return _e6.x;
    }
    let _e9 = axis_3;
    if (_e9 == 1u) {
        let _e12 = blockIdx_1;
        return _e12.y;
    }
    let _e15 = blockIdx_1;
    return _e15.z;
}

fn shape_component(axis_4: u32) -> u32 {
    var axis_5: u32;

    axis_5 = axis_4;
    let _e3 = axis_5;
    if (_e3 == 0u) {
        return 1u;
    }
    let _e7 = axis_5;
    if (_e7 == 1u) {
        return 1u;
    }
    return 1u;
}

fn grid_component(axis_6: u32) -> u32 {
    var axis_7: u32;

    axis_7 = axis_6;
    let _e3 = axis_7;
    if (_e3 == 0u) {
        let _e6 = gridDim_1;
        return _e6.x;
    }
    let _e9 = axis_7;
    if (_e9 == 1u) {
        let _e12 = gridDim_1;
        return _e12.y;
    }
    let _e15 = gridDim_1;
    return _e15.z;
}

fn component(axis_8: u32) -> u32 {
    var axis_9: u32;

    axis_9 = axis_8;
    let _e2 = axis_9;
    let _e3 = thread_component(_e2);
    return _e3;
}

fn component_1(offset: f32) -> f32 {
    var offset_1: f32;

    offset_1 = offset;
    let _e3 = blockIdx_1;
    let _e6 = offset_1;
    return (f32(_e3.x) + _e6);
}

fn global_x() -> u32 {
    let _e1 = group_component(0u);
    let _e3 = shape_component(0u);
    let _e6 = component(0u);
    return ((_e1 * _e3) + _e6);
}

fn kind(value: u32) -> u32 {
    var value_1: u32;

    value_1 = value;
    return 11u;
}

fn observe() {
    let _e1 = gridDim_1;
    if (_e1.z != 0u) {
        return;
    } else {
        return;
    }
}

@compute @workgroup_size(1, 1, 1)
fn coordinates(@builtin(local_invocation_id) threadIdx: vec3<u32>, @builtin(workgroup_id) blockIdx: vec3<u32>, @builtin(num_workgroups) gridDim: vec3<u32>) {
    var scale: f32;
    var limit: u32;

    let _e3 = scale_1.value;
    scale = _e3;
    let _e8 = limit_1.value;
    limit = _e8;
    threadIdx_1 = threadIdx;
    blockIdx_1 = blockIdx;
    gridDim_1 = gridDim;
    let _e17 = global_x();
    let _e18 = limit;
    if (_e17 >= _e18) {
        return;
    }
    observe();
    let _e23 = thread_component(0u);
    out[0i] = _e23;
    let _e27 = thread_component(1u);
    out[1i] = _e27;
    let _e31 = thread_component(2u);
    out[2i] = _e31;
    let _e35 = group_component(0u);
    out[3i] = _e35;
    let _e39 = group_component(1u);
    out[4i] = _e39;
    let _e43 = group_component(2u);
    out[5i] = _e43;
    let _e47 = shape_component(0u);
    out[6i] = _e47;
    let _e51 = shape_component(1u);
    out[7i] = _e51;
    let _e55 = shape_component(2u);
    out[8i] = _e55;
    let _e59 = grid_component(0u);
    out[9i] = _e59;
    let _e63 = grid_component(1u);
    out[10i] = _e63;
    let _e67 = grid_component(2u);
    out[11i] = _e67;
    let _e70 = global_x();
    out[12i] = _e70;
    let _e74 = component(0u);
    out[13i] = _e74;
    let _e78 = component_1(2.5f);
    out[14i] = bitcast<u32>(_e78);
    out[15i] = threadIdx.x;
    out[16i] = blockIdx.y;
    out[17i] = gridDim.z;
    out[18i] = 1u;
    let _e96 = input[0i];
    out[19i] = u32(_e96);
    let _e100 = scale;
    out[20i] = bitcast<u32>(_e100);
    let _e107 = points[0i].x;
    out[21i] = bitcast<u32>(_e107);
    let _e114 = points[0i].y;
    out[22i] = bitcast<u32>(_e114);
    let _e121 = points[0i].z;
    out[23i] = bitcast<u32>(_e121);
    let _e125 = limit;
    out[24i] = _e125;
    let _e129 = thread_component(0u);
    let _e131 = group_component(1u);
    let _e134 = shape_component(2u);
    let _e137 = grid_component(0u);
    out[25i] = (((_e129 + _e131) + _e134) + _e137);
    let _e141 = global_x();
    let _e142 = global_x();
    out[26i] = (_e141 + _e142);
    let _e148 = input[0i];
    let _e150 = scale;
    let _e155 = points[0i].x;
    out[27i] = bitcast<u32>(((f32(_e148) * _e150) + _e155));
    let _e161 = kind(threadIdx.x);
    out[28i] = _e161;
    let _e165 = grid_component(2u);
    out[29i] = _e165;
}
