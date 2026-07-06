# ROS / CUDA / C++ / Rust Projects

WSL2 (Ubuntu 24.04) 开发环境，包含 ROS2、CUDA、调度器框架、Rust 学习等项目。

## 目录结构

```
.
├── ros2_demo/                    # ROS2 Jazzy 示例项目
│   ├── src/talker_listener/      # Talker-Listener 通信示例
│   └── src/scheduler/            # 声明式 DAG 调度器框架
│       ├── include/scheduler/    # 公共头文件
│       ├── src/                  # 实现
│       └── test/                 # 测试用例
├── cuda_demo/                    # CUDA 和视觉处理示例
│   ├── cuda_edge.cu              # CUDA Sobel 边缘检测 kernel
│   ├── edge_processor.cpp        # C++ 边缘处理（HTTP + CUDA）
│   ├── image_viewer.cpp          # 图片对比查看器（OpenCV）
│   ├── mediapipe_pose.py         # MediaPipe 人体姿势识别
│   ├── CMakeLists.txt            # CMake 构建配置
│   └── build.sh                  # 一键编译脚本
├── rust_demo/                    # Rust 学习示例
│   ├── Cargo.toml                # Rust 项目配置
│   └── src/main.rs               # 入门示例代码
├── build.sh                      # 一键构建所有模块
├── CMakeLists.txt                # 顶层 CMake 配置
└── README.md
```

## 一键构建

```bash
./build.sh          # 构建所有模块并运行测试
./build.sh clean    # 清理后重新构建
```

## 功能模块

### 1. 调度器框架 (ros2_demo/src/scheduler/)

声明式 DAG 调度器，参考自动驾驶感知流水线设计。

**核心特性**：
- 声明式 DAG：通过 TaskParam 声明任务依赖（depend_tasks/after_tasks）
- 中心化事件驱动 tick：TriggerScheduler 单线程决策，避免并发竞态
- 多 Runner 并行：绑核专属线程，跨帧 work 互斥
- 分层守卫：消息对齐 → 帧间仲裁 → 帧内依赖 → 硬件重入

**架构**：
```
SchedulerManager → Scheduler → FrameBuffer → Frame → TaskInfo → Task → TaskRunner
```

**依赖**：
```bash
sudo apt install libcurl4-openssl-dev libopencv-dev libgtest-dev
```

**编译和测试**：
```bash
cd ros2_demo/src/scheduler
./build.sh

# 或手动
mkdir build && cd build
cmake ..
cmake --build .
./scheduler_tests
```

**测试覆盖（26个用例）**：任务状态转换、DAG 依赖链、失败传播、多 Runner 并行、帧间仲裁（4帧×5step×3runner）、Topic 路由、回调闭环等。

### 2. CUDA 边缘检测 (cuda_demo/)

Sobel 边缘检测 CUDA 实现，支持实时视频流处理。

**编译**：
```bash
cd cuda_demo
bash build.sh
```

**运行**：
```bash
python3 cuda_edge_processor.py
```

### 3. 图片对比查看器 (cuda_demo/image_viewer.cpp)

遍历目录中的图片，左右并排显示原图和 CUDA 边缘检测结果。

**编译**：
```bash
cd cuda_demo
mkdir build && cd build
cmake ..
cmake --build .
```

**运行**：
```bash
./image_viewer /path/to/images
```

**操作**：任意键切换下一张，ESC 退出。

### 4. MediaPipe 人体姿势识别 (cuda_demo/)

基于 MediaPipe PoseLandmarker 的实时人体骨架检测。

**运行**：
```bash
cd cuda_demo
python3 mediapipe_pose.py
```

### 5. ROS2 Talker-Listener (ros2_demo/)

ROS2 Jazzy 的节点通信示例。

**编译和运行**：
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

### 6. Rust 学习 (rust_demo/)

Rust 入门示例，涵盖基础语法。

**运行**：
```bash
cd rust_demo
cargo run
```

**示例内容**：变量与可变性、元组数组、函数、控制流、所有权与借用、结构体、枚举与模式匹配、错误处理、迭代器。

## 环境

- **OS**: Ubuntu 24.04 (WSL2)
- **CUDA**: 13.3 (RTX 5070 Ti)
- **ROS2**: Jazzy
- **Rust**: 1.96.1
- **Python**: 3.12
- **MediaPipe**: 0.10.35
