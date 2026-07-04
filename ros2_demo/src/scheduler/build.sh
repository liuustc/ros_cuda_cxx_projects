#!/bin/bash

# 调度器构建脚本

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}开始构建调度器...${NC}"

# 创建构建目录
BUILD_DIR="build"
if [ -d "$BUILD_DIR" ]; then
    echo -e "${YELLOW}清理旧的构建目录...${NC}"
    rm -rf "$BUILD_DIR"
fi

mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# 配置CMake
echo -e "${GREEN}配置CMake...${NC}"
cmake .. -DBUILD_TESTS=ON

# 编译
echo -e "${GREEN}编译调度器...${NC}"
make -j$(nproc)

# 运行测试
echo -e "${GREEN}运行测试...${NC}"
./scheduler_tests

echo -e "${GREEN}构建完成！${NC}"
echo ""
echo "可用命令："
echo "  运行测试: ./build/scheduler_tests"
echo "  安装库: cd build && make install"
echo ""
echo "测试文件位置："
echo "  源码: src/"
echo "  测试: test/"
echo "  构建: build/"