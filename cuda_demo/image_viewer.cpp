#include <opencv2/opencv.hpp>
#include <iostream>
#include <vector>
#include <string>
#include <filesystem>
#include <algorithm>

// CUDA 函数声明
extern "C" int cuda_edge_detect(unsigned char* h_input, unsigned char* h_output,
                              int width, int height, int channels);

namespace fs = std::filesystem;

// 获取目录下所有图片文件
std::vector<std::string> get_image_files(const std::string& dir_path) {
    std::vector<std::string> image_files;
    std::vector<std::string> extensions = {".jpg", ".jpeg", ".png", ".bmp", ".tiff", ".tif"};
    
    try {
        for (const auto& entry : fs::directory_iterator(dir_path)) {
            if (entry.is_regular_file()) {
                std::string ext = entry.path().extension().string();
                std::transform(ext.begin(), ext.end(), ext.begin(), ::tolower);
                
                for (const auto& valid_ext : extensions) {
                    if (ext == valid_ext) {
                        image_files.push_back(entry.path().string());
                        break;
                    }
                }
            }
        }
    } catch (const fs::filesystem_error& e) {
        std::cerr << "访问目录失败: " << e.what() << std::endl;
    }
    
    // 排序以便顺序浏览
    std::sort(image_files.begin(), image_files.end());
    return image_files;
}

// 处理单张图片并返回对比图
cv::Mat process_image(const std::string& image_path) {
    // 读取图片
    cv::Mat img = cv::imread(image_path);
    if (img.empty()) {
        std::cerr << "无法读取图片: " << image_path << std::endl;
        return cv::Mat();
    }
    
    int width = img.cols;
    int height = img.rows;
    int channels = img.channels();
    
    // 确保是3通道RGB图像
    cv::Mat rgb_img;
    if (channels == 1) {
        cv::cvtColor(img, rgb_img, cv::COLOR_GRAY2RGB);
        channels = 3;
    } else if (channels == 4) {
        cv::cvtColor(img, rgb_img, cv::COLOR_BGRA2RGB);
        channels = 3;
    } else {
        cv::cvtColor(img, rgb_img, cv::COLOR_BGR2RGB);
    }
    
    // 分配输出内存（灰度边缘图）
    std::vector<unsigned char> edge_data(width * height);
    
    // 调用CUDA处理
    cuda_edge_detect(rgb_img.data, edge_data.data(), width, height, channels);
    
    // 将边缘图转换为cv::Mat
    cv::Mat edge_img(height, width, CV_8UC1, edge_data.data());
    
    // 创建对比图（左右并排）
    cv::Mat comparison(height, width * 2, CV_8UC3);
    
    // 左侧：原图（BGR格式）
    cv::Mat left_roi = comparison(cv::Rect(0, 0, width, height));
    cv::cvtColor(rgb_img, left_roi, cv::COLOR_RGB2BGR);
    
    // 右侧：边缘图（转换为3通道）
    cv::Mat edge_color;
    cv::cvtColor(edge_img, edge_color, cv::COLOR_GRAY2BGR);
    cv::Mat right_roi = comparison(cv::Rect(width, 0, width, height));
    edge_color.copyTo(right_roi);
    
    // 添加标签
    cv::putText(comparison, "Original", cv::Point(10, 30), 
                cv::FONT_HERSHEY_SIMPLEX, 1, cv::Scalar(0, 255, 0), 2);
    cv::putText(comparison, "Edge Detection", cv::Point(width + 10, 30), 
                cv::FONT_HERSHEY_SIMPLEX, 1, cv::Scalar(0, 255, 0), 2);
    
    return comparison;
}

int main(int argc, char* argv[]) {
    // 默认图片目录
    std::string image_dir = "/mnt/d/Games";
    if (argc > 1) {
        image_dir = argv[1];
    }
    
    std::cout << "扫描图片目录: " << image_dir << std::endl;
    
    // 获取所有图片文件
    std::vector<std::string> image_files = get_image_files(image_dir);
    if (image_files.empty()) {
        std::cerr << "未找到图片文件" << std::endl;
        return 1;
    }
    
    std::cout << "找到 " << image_files.size() << " 张图片" << std::endl;
    
    // 创建窗口
    cv::namedWindow("CUDA Edge Detection Comparison", cv::WINDOW_NORMAL);
    cv::resizeWindow("CUDA Edge Detection Comparison", 1200, 600);
    
    int current_index = 0;
    
    while (true) {
        // 处理当前图片
        cv::Mat comparison = process_image(image_files[current_index]);
        if (comparison.empty()) {
            std::cerr << "处理图片失败，跳过" << std::endl;
            current_index = (current_index + 1) % image_files.size();
            continue;
        }
        
        // 显示图片
        cv::imshow("CUDA Edge Detection Comparison", comparison);
        
        // 显示信息
        std::cout << "显示: " << image_files[current_index] 
                  << " (" << current_index + 1 << "/" << image_files.size() << ")" << std::endl;
        std::cout << "按任意键显示下一张，按ESC退出" << std::endl;
        
        // 等待按键
        int key = cv::waitKey(0);
        
        // ESC键退出
        if (key == 27) {
            std::cout << "用户退出" << std::endl;
            break;
        }
        
        // 下一张图片
        current_index = (current_index + 1) % image_files.size();
    }
    
    cv::destroyAllWindows();
    return 0;
}