# WSL2 开发环境

本项目包含在 WSL2 (Ubuntu 24.04) 中配置的各种开发环境和工具。

## 目录结构

```
.
├── ros2_demo/          # ROS2 Jazzy 示例项目
│   └── src/talker_listener/  # Talker-Listener 通信示例
├── cuda_demo/          # CUDA 和 MediaPipe 示例
│   ├── cuda_edge.cu           # CUDA 边缘检测 kernel
│   ├── libcuda_edge.so        # 编译好的 CUDA 库
│   ├── cuda_edge_processor.py # Python 调用 CUDA 处理
│   ├── mediapipe_pose.py      # MediaPipe 人体姿势识别
│   ├── cam_client.py          # 摄像头客户端
│   ├── cuda_test.cu           # CUDA 基础测试
│   └── build.sh               # CUDA 编译脚本
└── README.md
```

## 功能模块

### 1. ROS2 Demo

- **ROS2 版本**: Jazzy (Ubuntu 24.04)
- **功能**: 两个 Node 互相发送消息
- **Talker**: 发送消息到 `chatter` topic
- **Listener**: 接收消息并回复到 `reply` topic

**编译和运行**:
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

### 2. CUDA 边缘检测

- **GPU**: NVIDIA GeForce RTX 5070 Ti
- **CUDA**: 13.3
- **功能**: Sobel 边缘检测，支持实时视频流处理

**编译**:
```bash
cd cuda_demo
bash build.sh
```

**运行**:
```bash
python3 cuda_edge_processor.py
```

### 3. MediaPipe 人体姿势识别

- **模型**: MediaPipe PoseLandmarker (33 个关键点)
- **功能**: 实时人体骨架检测和绘制
- **性能**: 30+ FPS
- **模型文件**: 首次运行自动下载 `pose_landmarker.task`

**运行**:
```bash
cd cuda_demo
python3 mediapipe_pose.py
```

### 4. 摄像头服务

- **Windows 端**: `H:\video_scripts\cam_server.py`
- **功能**: USB 摄像头采集 + OpenCV 显示
- **通信**: HTTP 协议

**Windows 端启动**:
```powershell
cd H:\video_scripts
python cam_server.py
```

## 环境配置

### 已安装软件

- ROS2 Jazzy
- CUDA 13.3
- clangd (代码跳转)
- Python3 + OpenCV
- MediaPipe 0.10.35

### VSCode 配置

- 使用 clangd 进行代码跳转
- 支持 compile_commands.json
- SSH 免密登录 WSL

## 使用说明

1. **ROS2 开发**: 使用 `colcon build` 编译，支持 `ros2 run` 命令
2. **CUDA 开发**: 使用 `nvcc` 编译，支持 Python ctypes 调用
3. **摄像头测试**: Windows 启动服务，WSL 调用处理

## 依赖

- Ubuntu 24.04 (WSL2)
- ROS2 Jazzy
- CUDA 13.3
- Python 3.x
- OpenCV (Python)
- MediaPipe 0.10.35
