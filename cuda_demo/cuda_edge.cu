#include <stdio.h>
#include <cuda_runtime.h>
#include <math.h>

// Sobel 边缘检测 kernel
__global__ void sobel_edge(const unsigned char* gray, unsigned char* output,
                           int width, int height) {
    int x = blockIdx.x * blockDim.x + threadIdx.x;
    int y = blockIdx.y * blockDim.y + threadIdx.y;

    if (x <= 0 || x >= width - 1 || y <= 0 || y >= height - 1) {
        if (x < width && y < height)
            output[y * width + x] = 0;
        return;
    }

    int gx = 0, gy = 0;
    const int sobel_x[3][3] = {{-1, 0, 1}, {-2, 0, 2}, {-1, 0, 1}};
    const int sobel_y[3][3] = {{-1, -2, -1}, {0, 0, 0}, {1, 2, 1}};

    for (int ky = -1; ky <= 1; ky++) {
        for (int kx = -1; kx <= 1; kx++) {
            unsigned char pixel = gray[(y + ky) * width + (x + kx)];
            gx += pixel * sobel_x[ky + 1][kx + 1];
            gy += pixel * sobel_y[ky + 1][kx + 1];
        }
    }

    int magnitude = (int)sqrtf((float)(gx * gx + gy * gy));
    output[y * width + x] = (unsigned char)(magnitude > 255 ? 255 : magnitude);
}

// RGB 转灰度
__global__ void rgb_to_gray(const unsigned char* rgb, unsigned char* gray,
                            int width, int height) {
    int x = blockIdx.x * blockDim.x + threadIdx.x;
    int y = blockIdx.y * blockDim.y + threadIdx.y;

    if (x >= width || y >= height) return;

    int idx = (y * width + x) * 3;
    gray[y * width + x] = (unsigned char)(0.299f * rgb[idx] + 0.587f * rgb[idx + 1] + 0.114f * rgb[idx + 2]);
}

extern "C" {

int cuda_edge_detect(unsigned char* h_input, unsigned char* h_output,
                     int width, int height, int channels) {
    unsigned char *d_input = nullptr, *d_gray = nullptr, *d_output = nullptr;
    size_t rgb_size = width * height * channels;
    size_t gray_size = width * height;

    cudaMalloc(&d_input, rgb_size);
    cudaMalloc(&d_gray, gray_size);
    cudaMalloc(&d_output, gray_size);

    cudaMemcpy(d_input, h_input, rgb_size, cudaMemcpyHostToDevice);

    dim3 block(16, 16);
    dim3 grid((width + 15) / 16, (height + 15) / 16);

    rgb_to_gray<<<grid, block>>>(d_input, d_gray, width, height);
    sobel_edge<<<grid, block>>>(d_gray, d_output, width, height);

    cudaMemcpy(h_output, d_output, gray_size, cudaMemcpyDeviceToHost);

    cudaFree(d_input);
    cudaFree(d_gray);
    cudaFree(d_output);

    return 0;
}

} // extern "C"
