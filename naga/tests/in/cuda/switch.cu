using Count = unsigned;

__device__ int case_chain(int selector) {
    int result = 0;
    switch (selector) {
        case -1:
            result += 1;
        case 0:
            result += 10;
        case 1:
            result += 100;
            break;
        default:
            result += 1000;
        case 2:
            result += 10000;
            break;
    }
    return result;
}

__device__ int grouped_labels(Count selector) {
    int result = 0;
    switch (selector) {
        case 0u:
        case 1u:
            result += 3;
        case 2u:
            result += 5;
            break;
        default:
            result = 7;
            break;
    }
    return result;
}

__device__ int nested_switches(int outer, int inner) {
    int result = 0;
    switch (outer) {
        case 0:
            result += 1;
            switch (inner) {
                case 0:
                    result += 10;
                default:
                    result += 20;
                    break;
            }
        case 1:
            result += 100;
            break;
        default:
            result = 900;
            break;
    }
    return result;
}

__device__ int loop_jumps(int selector) {
    int result = 0;
    for (int iteration = 0; iteration < 4; ++iteration) {
        switch (selector) {
            case 0:
                if (iteration == 1) {
                    continue;
                }
                result += iteration + 1;
            case 1:
                if (iteration == 2) {
                    break;
                }
                result += 10;
                break;
            default:
                result += 100;
                continue;
        }
        result += 1000;
    }
    return result;
}

__device__ int terminal_paths(int selector, int flag) {
    int result = 0;
    switch (selector) {
        case 0:
            if (flag != 0) {
                return 3;
            }
            result += 1;
        case 1:
            return result + 5;
        default:
            result = 9;
        case 2: {
            return result + 10;
        }
    }
    return -1;
}

__device__ int update_value(int value, int token) {
    return value + token;
}

__device__ int2 call_results(int selector, int counter) {
    int result = 2;
    switch (selector) {
        case 0: {
            int named_value = update_value(result, ++counter);
            result = named_value + named_value;
        }
        case 1: {
            int named_value = update_value(result, ++counter);
            result = named_value * named_value;
        }
        default: {
            int named_value = update_value(result, ++counter);
            result = named_value + named_value;
            break;
        }
    }
    return make_int2(result, counter);
}

__global__ void switches(int *out) {
    out[0] = case_chain(-1);
    out[1] = case_chain(0);
    out[2] = case_chain(1);
    out[3] = case_chain(2);
    out[4] = case_chain(8);
    out[5] = grouped_labels(0u);
    out[6] = grouped_labels(1u);
    out[7] = grouped_labels(2u);
    out[8] = grouped_labels(8u);
    out[9] = nested_switches(0, 0);
    out[10] = nested_switches(0, 8);
    out[11] = nested_switches(1, 0);
    out[12] = nested_switches(8, 0);
    out[13] = loop_jumps(0);
    out[14] = loop_jumps(1);
    out[15] = loop_jumps(8);
    out[16] = terminal_paths(0, 1);
    out[17] = terminal_paths(0, 0);
    out[18] = terminal_paths(1, 1);
    out[19] = terminal_paths(8, 0);
    out[20] = terminal_paths(2, 0);
    int counter = 0;
    switch (counter++) {
        case 0:
            out[21] = counter;
        case 1:
            out[22] = counter + 1;
            break;
        default:
            out[21] = 99;
            break;
    }
    out[23] = counter;
    int2 first = call_results(0, 0);
    int2 second = call_results(1, first.y);
    int2 third = call_results(8, second.y);
    out[24] = first.x;
    out[25] = second.x;
    out[26] = third.x;
    out[27] = third.y;
    counter = third.y;
    out[28] = update_value(third.x, ++counter);
    out[29] = counter;
}
