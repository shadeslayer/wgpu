
__global__ void loops(int *out, int n) {
    int sum = 0;
    for (int i = 0; i < n; i = i + 1) {
        if (i == 2) continue;
        if (i == 7) break;
        sum = sum + i;
    }
    int i = 0;
    while (i < 3) {
        i = i + 1;
        if (i == 2) continue;
        sum = sum + 10;
    }
    do {
        sum = sum + 100;
        if (sum > 200) break;
    } while (sum < 150);
    out[threadIdx.x] = sum;
}
