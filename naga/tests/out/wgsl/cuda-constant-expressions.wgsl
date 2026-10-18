struct float3_ {
    x: f32,
    y: f32,
    z: f32,
}

const Base_1: i32 = 2i;
const High: u32 = 2147483648u;
const Scale: f32 = 0.5f;
const Second: i32 = 3i;
const After: i32 = 4i;

@group(0) @binding(0)
var<storage> input: array<i32>;
@group(0) @binding(1)
var<storage, read_write> out: array<u32>;

fn accumulate(runtime_1: i32) -> i32 {
    var runtime_2: i32;
    var Size_1: i32;
    var local_8: i32;
    var local_9: array<i32, 3>;
    var total: i32;
    var i_1: i32;

    runtime_2 = runtime_1;
    Size_1 = 3i;
    local_9 = array<i32, 3>();
    total = 0i;
    {
        i_1 = 0i;
        loop {
            let _e10 = i_1;
            let _e11 = Size_1;
            if (_e10 < _e11) {
            } else {
                break;
            }
            let _e18 = runtime_2;
            let _e19 = i_1;
            let _e16 = i_1;
            local_9[_e16] = (_e18 + _e19);
            let _e21 = i_1;
            let _e23 = local_9[_e21];
            let _e24 = total;
            total = (_e24 + _e23);
            continuing {
                let _e13 = i_1;
                i_1 = (_e13 + 1i);
            }
        }
    }
    let _e26 = total;
    return _e26;
}

@compute @workgroup_size(1, 1, 1)
fn constant_expressions() {
    var runtime: i32;
    var Chosen: i32;
    var local: i32;
    var Size: i32;
    var values: array<i32, 5>;
    var SignedHigh: i32;
    var local_1: u32;
    var Negative: i32;
    var local_2: u32;
    var All: u32;
    var Wrapped: u32;
    var local_3: u32;
    var TruncatedZero: u32;
    var Minimum: i32;
    var Large: u32;
    var Ready: bool;
    var Comma: i32;
    var Lazy: i32;
    var local_4: i32;
    var Skipped: bool;
    var local_5: bool;
    var Yes: bool;
    var local_6: bool;
    var VectorCount: i32;
    var vectors: array<float3_, 2>;
    var Base: array<i32, 2>;
    var i: i32;
    var Count: i32;
    var scratch: array<i32, 3>;
    var Safe: i32;
    var local_7: i32;

    let _e5 = input[0i];
    runtime = _e5;
    Chosen = 3i;
    Size = 4i;
    values = array<i32, 5>();
    let _e14 = runtime;
    values[0i] = _e14;
    let _e19 = input[1i];
    let _e15 = Size;
    values[_e15] = _e19;
    out[0i] = u32(Base_1);
    out[1i] = High;
    out[2i] = bitcast<u32>(Scale);
    let _e33 = Chosen;
    out[3i] = u32(_e33);
    let _e37 = runtime;
    let _e38 = accumulate(_e37);
    out[4i] = u32(_e38);
    let _e42 = Size;
    let _e44 = values[_e42];
    out[5i] = u32(_e44);
    let _e50 = values[1i];
    out[6i] = u32(_e50);
    let _e54 = runtime;
    out[7i] = u32((f32(_e54) * Scale));
    SignedHigh = i32(-2147483648);
    Negative = -2i;
    All = 4294967295u;
    Wrapped = 0u;
    let _e69 = SignedHigh;
    out[8i] = u32(_e69);
    let _e73 = Negative;
    out[9i] = u32(_e73);
    let _e77 = All;
    out[10i] = _e77;
    let _e80 = Wrapped;
    out[11i] = _e80;
    TruncatedZero = 0u;
    Minimum = i32(-2147483648);
    Large = 4294967040u;
    Ready = true;
    let _e91 = TruncatedZero;
    out[12i] = _e91;
    let _e94 = Minimum;
    out[13i] = u32(_e94);
    let _e98 = Large;
    out[14i] = _e98;
    let _e101 = Ready;
    out[15i] = u32(_e101);
    Comma = 4i;
    Lazy = 6i;
    let _e109 = Comma;
    out[16i] = u32(_e109);
    let _e113 = Lazy;
    out[17i] = u32(_e113);
    let _e117 = runtime;
    out[18i] = u32(_e117);
    Skipped = false;
    Yes = true;
    let _e125 = Skipped;
    out[19i] = u32(_e125);
    let _e129 = Yes;
    out[20i] = u32(_e129);
    let _e133 = runtime;
    out[21i] = u32(_e133);
    VectorCount = 2i;
    vectors = array<float3_, 2>();
    let _e144 = input[1i];
    vectors[1i].y = f32(_e144);
    let _e151 = vectors[1i].y;
    out[22i] = u32(_e151);
    {
        Base = array<i32, 2>();
        let _e159 = Base[1i];
        out[23i] = u32(_e159);
    }
    {
        i = 0i;
        loop {
            let _e164 = i;
            if (_e164 < 3i) {
            } else {
                break;
            }
            Count = 3i;
            scratch = array<i32, 3>();
            let _e179 = scratch[2i];
            let _e174 = i;
            out[(24i + _e174)] = u32(_e179);
            let _e183 = runtime;
            let _e184 = i;
            scratch[2i] = (_e183 + _e184);
            continuing {
                let _e166 = i;
                i = (_e166 + 1i);
            }
        }
    }
    let _e190 = input[1i];
    let _e191 = accumulate(_e190);
    out[27i] = u32(_e191);
    out[28i] = u32(Second);
    Safe = 3i;
    let _e203 = runtime;
    let _e204 = input[1i];
    let _e206 = Safe;
    out[29i] = u32((((_e203 + _e204) + _e206) - 3i));
}
