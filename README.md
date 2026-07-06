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

Rust 核心特性学习示例，涵盖从基础到进阶的所有重要内容。

**运行**：
```bash
cd rust_demo
cargo run
```

**示例内容**：

| 文件 | 主题 | 核心概念 |
|------|------|----------|
| **基础部分** |||
| `_01_ownership.rs` | 所有权系统 | move、借用、切片、生命周期 |
| `_02_concurrency.rs` | 多线程 | Arc、Mutex、channel、RwLock、Barrier |
| `_03_std_components.rs` | 标准库 | Vec、String、HashMap、迭代器、Option/Result |
| **核心特性** |||
| `_04_trait.rs` | Trait 系统 | trait定义、动态分发、关联类型、运算符重载 |
| `_05_generic.rs` | 泛型 | 函数泛型、结构体泛型、const泛型、trait约束 |
| `_06_pattern_matching.rs` | 模式匹配 | match、解构、守卫、if let、@绑定 |
| `_07_closure.rs` | 闭包 | Fn/FnMut/FnOnce、捕获环境、迭代器适配 |
| `_08_smart_pointer.rs` | 智能指针 | Box、Rc、Cell、RefCell、Cow、Weak、Drop |
| `_09_async.rs` | 异步编程 | Future、async/await、Pin、异步运行时 |
| `_10_error_handling.rs` | 错误处理 | Result、Option、?运算符、自定义错误 |
| `_11_macro.rs` | 宏 | 声明宏、过程宏、derive宏、宏卫生性 |
| **生态工具** |||
| `_12_testing.rs` | 单元测试 | #[test]、断言宏、参数化测试、并发测试 |
| `_13_networking.rs` | 网络库 | reqwest、tokio、TCP/UDP、WebSocket、axum |
| `_14_logging.rs` | 日志库 | log、tracing、结构化日志、日志级别 |
| `_15_ffi.rs` | C/C++ 互操作 | extern "C"、类型映射、bindgen、cbindgen |
| `_16_cargo.rs` | 依赖管理 | Cargo.toml、features、workspace、构建脚本 |

**学习路径建议**：
1. 先学 01-03（基础）
2. 再学 04-08（核心特性）
3. 然后学 09-11（进阶）
4. 最后学 12-16（生态工具）

## 环境

- **OS**: Ubuntu 24.04 (WSL2)
- **CUDA**: 13.3 (RTX 5070 Ti)
- **ROS2**: Jazzy
- **Rust**: 1.96.1
- **Python**: 3.12
- **MediaPipe**: 0.10.35
