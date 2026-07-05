cmake_minimum_required(VERSION 3.18)
project(ros_cuda_projects LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# ===== 选项 =====
option(BUILD_CUDA_DEMO "构建 cuda_demo 模块" ON)
option(BUILD_SCHEDULER "构建 scheduler 模块" ON)
option(BUILD_TALKER_LISTENER "构建 talker_listener 模块（需要 ROS2）" OFF)

# ===== 测试 =====
enable_testing()

# ===== cuda_demo =====
if(BUILD_CUDA_DEMO)
    enable_language(CUDA)
    set(CMAKE_CUDA_STANDARD 17)
    add_subdirectory(cuda_demo)
endif()

# ===== scheduler =====
if(BUILD_SCHEDULER)
    add_subdirectory(ros2_demo/src/scheduler)
endif()

# ===== talker_listener =====
if(BUILD_TALKER_LISTENER)
    find_package(ament_cmake QUIET)
    if(ament_cmake_FOUND)
        add_subdirectory(ros2_demo/src/talker_listener)
    else()
        message(WARNING "ament_cmake 未找到，跳过 talker_listener。\n"
            "请先 source /opt/ros/jazzy/setup.bash")
    endif()
endif()

message(STATUS "")
message(STATUS "===== 构建配置 =====")
message(STATUS "  cuda_demo:        ${BUILD_CUDA_DEMO}")
message(STATUS "  scheduler:        ${BUILD_SCHEDULER}")
message(STATUS "  talker_listener:  ${BUILD_TALKER_LISTENER}")
message(STATUS "====================")
message(STATUS "")
