# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

WSL2 (Ubuntu 24.04) 开发环境，包含 ROS2、CUDA、MediaPipe 等工具链，支持摄像头实时处理和人体姿势识别。

## 核心架构

### 1. ROS2 Demo (ros2_demo/)

ROS2 Jazzy 的 Talker-Listener 示例，演示节点间通信。

**关键文件**：
- `src/talker_listener/src/talker.cpp` - 发送消息节点
- `src/talker_listener/src/listener.cpp` - 接收消息节点

**构建和运行**：
```bash
cd ros2_demo
source /opt/ros/jazzy/setup.bash
colcon build --cmake-args -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
source install/setup.bash

# 终端 1
ros2 run talker_listener listener

# 终端 2
ros2 run talker_listener talker
```

### 2. CUDA 边缘检测 (test/)

Sobel 边缘检测 CUDA 实现，支持实时视频流处理。

**关键文件**：
- `cuda_edge.cu` - CUDA kernel 实现
- `cuda_edge_processor.py` - Python 调用脚本
- `build.sh` - 编译脚本

**编译**：
```bash
cd test
bash build.sh
```

**运行**：
```bash
python3 cuda_edge_processor.py
```

### 3. MediaPipe 人体姿势识别 (test/)

基于 MediaPipe PoseLandmarker 的实时人体骨架检测。

**关键文件**：
- `mediapipe_pose.py` - 主处理脚本
- `pose_landmarker_heavy.task` - 模型文件（自动下载）

**运行**：
```bash
cd test
python3 mediapipe_pose.py
```

### 4. 摄像头服务

Windows 端摄像头采集 + WSL 端处理的架构。

**架构**：
```
Windows (cam_server.py)  ←HTTP→  WSL (处理脚本)
        ↓
   OpenCV 显示
```

**Windows 端**：`H:\video_scripts\cam_server.py`
**WSL 端**：`test/cuda_edge_processor.py` 或 `test/mediapipe_pose.py`

## 环境配置

- **CUDA**: 13.3 (RTX 5070 Ti)
- **ROS2**: Jazzy
- **Python**: 3.12
- **MediaPipe**: 0.10.35

## 常用命令

### ROS2 开发
```bash
colcon build --cmake-args -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
ros2 run <package> <node>
ros2 topic list
```

### CUDA 开发
```bash
nvcc -shared -Xcompiler -fPIC -o libcuda_edge.so cuda_edge.cu -lcudart
```

### Python 环境
```bash
pip3 install --break-system-packages mediapipe
```

## 代码风格

- **C++**: ROS2 标准风格
- **Python**: 遵循 PEP 8
- **CUDA**: 使用 extern "C" 导出函数

## 调试技巧

1. **ROS2 调试**：使用 `ros2 topic echo` 查看消息
2. **CUDA 调试**：检查 `nvidia-smi` 和 `nvcc --version`
3. **MediaPipe 调试**：检查模型文件是否存在

## 注意事项

- WSL2 中需要手动设置 `ROS_DOMAIN_ID`
- CUDA 库需要设置 `LD_LIBRARY_PATH`
- MediaPipe 模型首次运行自动下载
- Windows 端需要开启多线程 HTTP 服务器
