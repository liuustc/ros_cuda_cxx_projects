#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>
#include <vector>
#include <string>

// CUDA 函数声明
extern "C" int cuda_edge_detect(unsigned char* h_input, unsigned char* h_output,
                              int width, int height, int channels);

// libcurl 回调：写入内存
struct MemoryBlock {
    unsigned char* data;
    size_t size;
};

static size_t write_callback(void* contents, size_t size, size_t nmemb, void* userp) {
    size_t realsize = size * nmemb;
    MemoryBlock* mem = (MemoryBlock*)userp;

    unsigned char* ptr = (unsigned char*)realloc(mem->data, mem->size + realsize);
    if (!ptr) return 0;

    mem->data = ptr;
    memcpy(&(mem->data[mem->size]), contents, realsize);
    mem->size += realsize;

    return realsize;
}

// 简易 JPEG 解码（使用 libjpeg-turbo 或 stb_image）
// 这里用一个简化版本，实际需要链接图像库
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

// stb 写入回调：收集数据到内存
struct StbMemoryBlock {
    unsigned char* data;
    size_t size;
    size_t capacity;
};

static void stb_write_callback(void *context, void *data, int size) {
    StbMemoryBlock *mem = (StbMemoryBlock*)context;
    size_t new_size = mem->size + size;
    
    if (new_size > mem->capacity) {
        size_t new_capacity = mem->capacity ? mem->capacity * 2 : 1024;
        while (new_capacity < new_size) new_capacity *= 2;
        
        unsigned char* ptr = (unsigned char*)realloc(mem->data, new_capacity);
        if (!ptr) return;
        
        mem->data = ptr;
        mem->capacity = new_capacity;
    }
    
    memcpy(mem->data + mem->size, data, size);
    mem->size = new_size;
}

// 从 HTTP 获取图片
bool fetch_image(const char* url, std::vector<unsigned char>& buffer) {
    CURL* curl = curl_easy_init();
    if (!curl) return false;

    MemoryBlock chunk = {nullptr, 0};

    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void*)&chunk);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 5L);

    CURLcode res = curl_easy_perform(curl);
    curl_easy_cleanup(curl);

    if (res != CURLE_OK || !chunk.data) {
        free(chunk.data);
        return false;
    }

    buffer.assign(chunk.data, chunk.data + chunk.size);
    free(chunk.data);
    return true;
}

// 发送处理后的图片到 Windows
bool send_result(const char* url, unsigned char* data, size_t size) {
    CURL* curl = curl_easy_init();
    if (!curl) return false;

    struct curl_slist* headers = nullptr;
    headers = curl_slist_append(headers, "Content-Type: application/octet-stream");

    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, data);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)size);
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);

    CURLcode res = curl_easy_perform(curl);
    curl_easy_cleanup(curl);
    curl_slist_free_all(headers);

    return res == CURLE_OK;
}

int main(int argc, char* argv[]) {
    const char* windows_ip = "172.26.112.1";
    if (argc > 1) windows_ip = argv[1];

    char fetch_url[256], send_url[256];
    snprintf(fetch_url, sizeof(fetch_url), "http://%s:8080/snapshot", windows_ip);
    snprintf(send_url, sizeof(send_url), "http://%s:8080/upload", windows_ip);

    printf("Windows IP: %s\n", windows_ip);
    printf("获取图片: %s\n", fetch_url);
    printf("发送结果: %s\n", send_url);

    // 获取图片
    std::vector<unsigned char> jpeg_data;
    if (!fetch_image(fetch_url, jpeg_data)) {
        fprintf(stderr, "获取图片失败!\n");
        return 1;
    }

    // 解码 JPEG
    int width, height, channels;
    unsigned char* image = stbi_load_from_memory(jpeg_data.data(), jpeg_data.size(),
                                                  &width, &height, &channels, 3);
    if (!image) {
        fprintf(stderr, "解码图片失败!\n");
        return 1;
    }

    printf("图片尺寸: %dx%d, 通道: %d\n", width, height, channels);

    // 分配输出内存（灰度边缘图）
    unsigned char* edge_data = (unsigned char*)malloc(width * height);

    // CUDA 处理
    printf("CUDA 边缘提取...\n");
    cuda_edge_detect(image, edge_data, width, height, channels);

    // 编码为 JPEG
    StbMemoryBlock out_mem = {nullptr, 0, 0};
    int success = stbi_write_jpg_to_func(stb_write_callback, &out_mem, 
                                         width, height, 1, edge_data, 80);
    if (!success) {
        fprintf(stderr, "JPEG 编码失败!\n");
        stbi_image_free(image);
        free(edge_data);
        return 1;
    }
    unsigned char* out_jpeg = out_mem.data;
    size_t out_size = out_mem.size;

    // 发送结果
    printf("发送结果到 Windows...\n");
    if (send_result(send_url, out_jpeg, out_size)) {
        printf("处理完成!\n");
    } else {
        fprintf(stderr, "发送失败!\n");
    }

    // 清理
    stbi_image_free(image);
    free(edge_data);
    free(out_jpeg);

    return 0;
}
