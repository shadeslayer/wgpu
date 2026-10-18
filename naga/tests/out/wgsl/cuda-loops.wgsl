struct type_2 {
    value: i32,
}

@group(0) @binding(0)
var<storage, read_write> out: array<i32>;
@group(0) @binding(1)
var<uniform> n_1: type_2;

@compute @workgroup_size(1, 1, 1)
fn loops(@builtin(local_invocation_id) threadIdx: vec3<u32>) {
    var n: i32;
    var sum: i32;
    var i: i32;
    var i_1: i32;

    let _e3 = n_1.value;
    n = _e3;
    sum = 0i;
    {
        i = 0i;
        loop {
            let _e9 = i;
            let _e10 = n;
            if (_e9 < _e10) {
            } else {
                break;
            }
            let _e16 = i;
            if (_e16 == 2i) {
                continue;
            }
            let _e19 = i;
            if (_e19 == 7i) {
                break;
            }
            let _e21 = sum;
            let _e22 = i;
            sum = (_e21 + _e22);
            continuing {
                let _e13 = i;
                i = (_e13 + 1i);
            }
        }
    }
    i_1 = 0i;
    loop {
        let _e27 = i_1;
        if (_e27 < 3i) {
        } else {
            break;
        }
        {
            let _e30 = i_1;
            i_1 = (_e30 + 1i);
            let _e33 = i_1;
            if (_e33 == 2i) {
                continue;
            }
            let _e36 = sum;
            sum = (_e36 + 10i);
        }
    }
    loop {
        let _e39 = sum;
        sum = (_e39 + 100i);
        let _e42 = sum;
        if (_e42 > 200i) {
            break;
        }
        continuing {
            let _e45 = sum;
            break if !((_e45 < 150i));
        }
    }
    let _e51 = sum;
    out[threadIdx.x] = _e51;
}
