#!/bin/bash

# 一键构建所有模块

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BUILD_DIR="${SCRIPT_DIR}/build"

echo -e "${GREEN}===== 构建所有模块 =====${NC}"

# 清理旧构建
if [ "$1" = "clean" ]; then
    echo -e "${YELLOW}清理构建目录...${NC}"
    rm -rf "${BUILD_DIR}"
fi

# 检测 ROS2 环境
ROS2_AVAILABLE=0
if source /opt/ros/jazzy/setup.bash 2>/dev/null && command -v ros2 &>/dev/null; then
    ROS2_AVAILABLE=1
fi

# 配置
echo -e "${GREEN}[1/3] CMake 配置...${NC}"
CMAKE_ARGS=(
    -B "${BUILD_DIR}"
    -DBUILD_CUDA_DEMO=ON
    -DBUILD_SCHEDULER=ON
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
)

if [ "$ROS2_AVAILABLE" = "1" ]; then
    CMAKE_ARGS+=(-DBUILD_TALKER_LISTENER=ON)
fi

cmake "${CMAKE_ARGS[@]}"

# 编译
echo -e "${GREEN}[2/3] 编译...${NC}"
cmake --build "${BUILD_DIR}" -j$(nproc)

# 运行测试
echo -e "${GREEN}[3/3] 运行调度器测试...${NC}"
"${BUILD_DIR}/ros2_demo/src/scheduler/scheduler_tests" --gtest_brief=1

echo ""
echo -e "${GREEN}===== 构建完成 =====${NC}"
echo ""
echo "产物位置："
echo "  compile_commands.json: ${BUILD_DIR}/compile_commands.json"
echo "  scheduler_tests:       ${BUILD_DIR}/ros2_demo/src/scheduler/scheduler_tests"
echo "  libcuda_edge.so:       ${BUILD_DIR}/cuda_demo/libcuda_edge.so"
echo "  edge_processor:        ${BUILD_DIR}/cuda_demo/edge_processor"
echo "  image_viewer:          ${BUILD_DIR}/cuda_demo/image_viewer"
if [ "$ROS2_AVAILABLE" = "1" ]; then
    echo "  talker:                ${BUILD_DIR}/ros2_demo/src/talker_listener/talker"
    echo "  listener:              ${BUILD_DIR}/ros2_demo/src/talker_listener/listener"
fi
