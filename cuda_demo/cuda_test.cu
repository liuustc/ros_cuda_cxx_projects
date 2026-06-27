#include <stdio.h>

__global__ void hello() {
    printf("Hello from GPU thread %d, block %d\n", threadIdx.x, blockIdx.x);
}

int main() {
    int device_count = 0;
    cudaGetDeviceCount(&device_count);
    printf("Found %d CUDA device(s)\n", device_count);

    if (device_count == 0) {
        printf("No CUDA devices found!\n");
        return 1;
    }

    // 打印设备信息
    cudaDeviceProp prop;
    cudaGetDeviceProperties(&prop, 0);
    printf("Device: %s\n", prop.name);
    printf("Compute Capability: %d.%d\n", prop.major, prop.minor);
    printf("Memory: %.1f GB\n", prop.totalGlobalMem / 1024.0 / 1024.0 / 1024.0);

    // 启动 kernel
    printf("\nLaunching kernel...\n");
    hello<<<2, 4>>>();
    cudaDeviceSynchronize();

    printf("\nCUDA test passed!\n");
    return 0;
}
