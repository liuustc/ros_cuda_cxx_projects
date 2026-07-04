cmake_minimum_required(VERSION 3.18)
project(cuda_demo LANGUAGES CXX CUDA)

# 设置 C++ 和 CUDA 标准
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CUDA_STANDARD 17)

# 查找 CUDA 工具包
find_package(CUDAToolkit REQUIRED)
message(STATUS "CUDA 版本: ${CUDAToolkit_VERSION}")
message(STATUS "CUDA 库: ${CUDAToolkit_CUDA_LIBRARY}")

# 选项：是否构建 edge_processor
option(BUILD_EDGE_PROCESSOR "构建 edge_processor 可执行文件" ON)

# 查找 libcurl（如果构建 edge_processor）
if(BUILD_EDGE_PROCESSOR)
    find_package(CURL QUIET)
    if(NOT CURL_FOUND)
        message(FATAL_ERROR "libcurl 未找到。\n"
            "请安装 libcurl 开发包：\n"
            "  Ubuntu/Debian: sudo apt install libcurl4-openssl-dev\n"
            "  CentOS/RHEL: sudo yum install libcurl-devel\n"
            "  或设置 -DBUILD_EDGE_PROCESSOR=OFF 跳过构建 edge_processor")
    endif()
endif()

# 设置 stb 头文件路径（假设 stb 仓库在 /home/liuhui/code/stb）
set(STB_INCLUDE_DIR "/home/liuhui/code/stb")
if(NOT EXISTS "${STB_INCLUDE_DIR}/stb_image.h")
    message(FATAL_ERROR "stb_image.h 未找到。\n"
        "请确保 stb 仓库在 ${STB_INCLUDE_DIR} 目录下。\n"
        "或者修改 CMakeLists.txt 中的 STB_INCLUDE_DIR 路径。")
endif()

# 编译 CUDA 共享库
add_library(cuda_edge SHARED cuda_edge.cu)
target_include_directories(cuda_edge PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(cuda_edge CUDA::cudart)
set_target_properties(cuda_edge PROPERTIES
    POSITION_INDEPENDENT_CODE ON
    CUDA_SEPARABLE_COMPILATION ON
)

# 编译 C++ 处理程序（如果启用）
if(BUILD_EDGE_PROCESSOR)
    add_executable(edge_processor edge_processor.cpp)
    target_include_directories(edge_processor PRIVATE 
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${STB_INCLUDE_DIR}
        ${CURL_INCLUDE_DIRS}
    )
    target_link_libraries(edge_processor 
        cuda_edge 
        CURL::libcurl
    )
    
    # 设置 RPATH 以便找到共享库
    set_target_properties(edge_processor PROPERTIES
        INSTALL_RPATH "$ORIGIN"
        BUILD_WITH_INSTALL_RPATH TRUE
    )
endif()

# 选项：是否构建 image_viewer
option(BUILD_IMAGE_VIEWER "构建 image_viewer 图片查看器" ON)

# 查找 OpenCV（如果构建 image_viewer）
if(BUILD_IMAGE_VIEWER)
    find_package(OpenCV QUIET)
    if(NOT OpenCV_FOUND)
        message(FATAL_ERROR "OpenCV 未找到。\n"
            "请安装 OpenCV 开发包：\n"
            "  Ubuntu/Debian: sudo apt install libopencv-dev\n"
            "  CentOS/RHEL: sudo yum install opencv-devel\n"
            "  或设置 -DBUILD_IMAGE_VIEWER=OFF 跳过构建 image_viewer")
    endif()
    
    # 编译 image_viewer
    add_executable(image_viewer image_viewer.cpp)
    target_include_directories(image_viewer PRIVATE 
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${OpenCV_INCLUDE_DIRS}
    )
    target_link_libraries(image_viewer 
        cuda_edge 
        ${OpenCV_LIBS}
    )
    
    # 设置 RPATH
    set_target_properties(image_viewer PROPERTIES
        INSTALL_RPATH "$ORIGIN"
        BUILD_WITH_INSTALL_RPATH TRUE
    )
endif()