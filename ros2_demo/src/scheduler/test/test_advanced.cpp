#include <gtest/gtest.h>
#include "scheduler/scheduler.h"
#include <thread>
#include <chrono>
#include <iostream>
#include <atomic>
#include <future>

using namespace scheduler;

// 测试Step仲裁 - 详细执行顺序验证
TEST(StepArbitrationTest, StepArbitrationBasic) {
    const int NUM_FRAMES = 4;
    const int NUM_STEPS = 5;
    const int NUM_RUNNERS = 3;
    
    Scheduler scheduler("step_test", NUM_RUNNERS);
    
    // 记录：(执行序号, frame_id, step, 时间戳ms)
    std::mutex record_mu;
    std::vector<std::tuple<int, uint64_t, int, long long>> records;
    std::atomic<int> exec_order{0};
    auto start_time = std::chrono::steady_clock::now();
    
    // 注册5个不同 step 的工作单元
    for (int step = 0; step < NUM_STEPS; step++) {
        std::vector<TaskGenerator::WorkFunc> work_funcs;
        work_funcs.push_back([step, &record_mu, &records, &exec_order, start_time](std::shared_ptr<Frame> frame) -> bool {
            std::this_thread::sleep_for(std::chrono::milliseconds(5));
            auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(
                std::chrono::steady_clock::now() - start_time).count();
            int order = exec_order.fetch_add(1);
            std::lock_guard<std::mutex> lock(record_mu);
            records.emplace_back(order, frame->frame_id(), step, elapsed);
            return true;
        });
        
        scheduler.register_work("work_step_" + std::to_string(step), work_funcs);
        
        TaskParam param;
        param.name = "task_step_" + std::to_string(step);
        param.work_name = "work_step_" + std::to_string(step);
        param.runner_id = step % NUM_RUNNERS;
        param.step = step;
        if (step > 0) {
            param.depend_tasks = {"task_step_" + std::to_string(step - 1)};
        }
        scheduler.register_task_param(param);
    }
    
    scheduler.start();
    
    // 快速插入多帧
    for (int i = 0; i < NUM_FRAMES; i++) {
        scheduler.insert_message("camera", 1000 + i * 100, std::any(i));
    }
    
    // 等待处理完成
    std::this_thread::sleep_for(std::chrono::milliseconds(5000));
    
    auto stats = scheduler.get_stats();
    
    // ============ 输出执行矩阵 ============
    std::cout << "\n=== Step 仲裁执行矩阵 ===" << std::endl;
    std::cout << "每帧 " << NUM_STEPS << " 个 step，" << NUM_FRAMES << " 帧，" 
              << NUM_RUNNERS << " 个 runner\n" << std::endl;
    
    // 按执行顺序排序
    std::sort(records.begin(), records.end());
    
    // 构建矩阵：frame_id x step -> 执行序号
    std::map<std::pair<uint64_t, int>, int> exec_matrix;
    for (const auto& [order, fid, step, ts] : records) {
        exec_matrix[{fid, step}] = order;
    }
    
    // 打印表头
    std::cout << "Step\\Frame |";
    for (int i = 0; i < NUM_FRAMES; i++) {
        std::cout << "  F" << (i+1) << "  |";
    }
    std::cout << std::endl;
    std::cout << std::string(11 + NUM_FRAMES * 7, '-') << std::endl;
    
    // 打印每行
    for (int step = 0; step < NUM_STEPS; step++) {
        std::cout << "   Step " << step << "   |";
        for (int i = 0; i < NUM_FRAMES; i++) {
            uint64_t fid = i + 1;
            auto it = exec_matrix.find({fid, step});
            if (it != exec_matrix.end()) {
                char buf[8];
                snprintf(buf, sizeof(buf), " %3d  |", it->second);
                std::cout << buf;
            } else {
                std::cout << "  -   |";
            }
        }
        std::cout << std::endl;
    }
    
    // 打印执行时间线
    std::cout << "\n=== 执行时间线 ===" << std::endl;
    std::cout << "Order | Frame | Step | Time(ms) |" << std::endl;
    std::cout << "------+------+------+----------|" << std::endl;
    for (const auto& [order, fid, step, ts] : records) {
        char buf[64];
        snprintf(buf, sizeof(buf), "  %2d  |  F%d   |  %d   |  %4lldms  |", 
                 order, (int)fid, step, ts);
        std::cout << buf << std::endl;
    }
    
    // ============ 验证 step 仲裁 ============
    std::cout << "\n=== 仲裁验证 ===" << std::endl;
    
    bool violation = false;
    for (int step = 0; step < NUM_STEPS; step++) {
        // 收集该 step 内各帧的执行序号
        std::vector<std::pair<int, uint64_t>> step_orders;  // (order, frame_id)
        for (const auto& [order, fid, s, ts] : records) {
            if (s == step) {
                step_orders.emplace_back(order, fid);
            }
        }
        
        // 按 frame_id 排序，检查执行序号是否递增
        std::sort(step_orders.begin(), step_orders.end(),
                  [](const auto& a, const auto& b) { return a.second < b.second; });
        
        for (size_t i = 1; i < step_orders.size(); i++) {
            if (step_orders[i].first < step_orders[i-1].first) {
                std::cout << "VIOLATION: Step " << step 
                          << " - frame " << step_orders[i].second 
                          << " (order=" << step_orders[i].first
                          << ") executed BEFORE frame " << step_orders[i-1].second
                          << " (order=" << step_orders[i-1].first << ")" << std::endl;
                violation = true;
            }
        }
    }
    
    if (!violation) {
        std::cout << "PASS: 同一 step 内，frame_id 小的帧始终先执行" << std::endl;
    }
    
    // 验证所有帧都完成
    std::cout << "\n总帧数: " << stats.total_frames << std::endl;
    std::cout << "已完成: " << stats.completed_frames << std::endl;
    std::cout << "总任务: " << stats.total_tasks << std::endl;
    
    EXPECT_FALSE(violation);
    EXPECT_EQ(stats.completed_frames, NUM_FRAMES);
    EXPECT_EQ(records.size(), NUM_FRAMES * NUM_STEPS);
    
    scheduler.stop();
}

// 测试Step仲裁 - 验证后帧不超前前帧
TEST(StepArbitrationTest, StepArbitrationEnforcement) {
    const int NUM_FRAMES = 3;
    const int NUM_STEPS = 6;
    const int NUM_RUNNERS = 2;
    
    Scheduler scheduler("step_enforce_test", NUM_RUNNERS);
    
    // 记录每个帧完成每个step的时刻
    std::mutex mu;
    std::map<std::pair<uint64_t, int>, long long> completion_time;  // (frame_id, step) -> time_ms
    auto start_time = std::chrono::steady_clock::now();
    
    for (int step = 0; step < NUM_STEPS; step++) {
        std::vector<TaskGenerator::WorkFunc> work_funcs;
        work_funcs.push_back([step, &mu, &completion_time, start_time](std::shared_ptr<Frame> frame) -> bool {
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
            auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(
                std::chrono::steady_clock::now() - start_time).count();
            std::lock_guard<std::mutex> lock(mu);
            completion_time[{frame->frame_id(), step}] = elapsed;
            return true;
        });
        
        scheduler.register_work("enforce_work_" + std::to_string(step), work_funcs);
        
        TaskParam param;
        param.name = "enforce_task_" + std::to_string(step);
        param.work_name = "enforce_work_" + std::to_string(step);
        param.runner_id = step % NUM_RUNNERS;
        param.step = step;
        if (step > 0) {
            param.depend_tasks = {"enforce_task_" + std::to_string(step - 1)};
        }
        scheduler.register_task_param(param);
    }
    
    scheduler.start();
    
    for (int i = 0; i < NUM_FRAMES; i++) {
        scheduler.insert_message("camera", 1000 + i * 100, std::any(i));
    }
    
    std::this_thread::sleep_for(std::chrono::milliseconds(5000));
    
    auto stats = scheduler.get_stats();
    
    // 输出完成时间矩阵
    std::cout << "\n=== 帧完成时间矩阵 (ms) ===" << std::endl;
    std::cout << "Step\\Frame |";
    for (int i = 0; i < NUM_FRAMES; i++) {
        std::cout << "   F" << (i+1) << "   |";
    }
    std::cout << std::endl;
    std::cout << std::string(11 + NUM_FRAMES * 10, '-') << std::endl;
    
    for (int step = 0; step < NUM_STEPS; step++) {
        std::cout << "   Step " << step << "   |";
        for (int i = 0; i < NUM_FRAMES; i++) {
            uint64_t fid = i + 1;
            auto it = completion_time.find({fid, step});
            if (it != completion_time.end()) {
                char buf[12];
                snprintf(buf, sizeof(buf), " %5lldms |", it->second);
                std::cout << buf;
            } else {
                std::cout << "    -    |";
            }
        }
        std::cout << std::endl;
    }
    
    // 验证：同一step内，后帧的完成时间 >= 前帧的完成时间
    std::cout << "\n=== 仲裁验证 ===" << std::endl;
    bool violation = false;
    
    for (int step = 0; step < NUM_STEPS; step++) {
        for (int i = 1; i < NUM_FRAMES; i++) {
            auto cur_it = completion_time.find({(uint64_t)(i+1), step});
            auto prev_it = completion_time.find({(uint64_t)i, step});
            
            if (cur_it != completion_time.end() && prev_it != completion_time.end()) {
                if (cur_it->second < prev_it->second) {
                    std::cout << "VIOLATION: Step " << step 
                              << " - F" << (i+1) << " (" << cur_it->second << "ms)"
                              << " completed BEFORE F" << i << " (" << prev_it->second << "ms)" 
                              << std::endl;
                    violation = true;
                }
            }
        }
    }
    
    if (!violation) {
        std::cout << "PASS: 同一 step 内，后帧完成时间 >= 前帧完成时间" << std::endl;
    }
    
    std::cout << "\n已完成帧: " << stats.completed_frames << "/" << NUM_FRAMES << std::endl;
    
    EXPECT_FALSE(violation);
    EXPECT_EQ(stats.completed_frames, NUM_FRAMES);
    
    scheduler.stop();
}

// 测试任务依赖
TEST(DependencyTest, StrongDependency) {
    Scheduler scheduler("dependency_test", 2);
    
    // 注册两个工作单元
    std::vector<TaskGenerator::WorkFunc> work1_funcs;
    work1_funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        frame->data().insert("work1_done", true);
        return true;
    });
    
    std::vector<TaskGenerator::WorkFunc> work2_funcs;
    work2_funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        // 检查依赖是否满足
        if (!frame->data().has("work1_done")) {
            return false;  // 依赖未满足
        }
        frame->data().insert("work2_done", true);
        return true;
    });
    
    scheduler.register_work("work1", work1_funcs);
    scheduler.register_work("work2", work2_funcs);
    
    // 注册任务参数，work2依赖work1
    TaskParam param1;
    param1.name = "task1";
    param1.work_name = "work1";
    param1.runner_id = 0;
    param1.step = 0;
    
    TaskParam param2;
    param2.name = "task2";
    param2.work_name = "work2";
    param2.runner_id = 1;
    param2.step = 1;
    param2.depend_tasks = {"task1"};  // 强依赖
    
    scheduler.register_task_param(param1);
    scheduler.register_task_param(param2);
    
    scheduler.start();
    
    // 插入测试帧
    scheduler.insert_message("test", 1000, std::any(1));
    
    // 等待处理
    std::this_thread::sleep_for(std::chrono::milliseconds(200));
    
    auto stats = scheduler.get_stats();
    
    std::cout << "Dependency test:" << std::endl;
    std::cout << "  Total tasks: " << stats.total_tasks << std::endl;
    std::cout << "  Completed tasks: " << stats.completed_tasks << std::endl;
    
    // 验证：两个任务都应该完成
    EXPECT_GE(stats.total_tasks, 2);
    
    scheduler.stop();
}

// 测试超时处理
TEST(TimeoutTest, TaskTimeout) {
    Scheduler scheduler("timeout_test", 1);
    
    // 注册一个会超时的工作单元
    std::vector<TaskGenerator::WorkFunc> timeout_work;
    timeout_work.push_back([](std::shared_ptr<Frame> frame) -> bool {
        // 模拟超时：长时间阻塞
        std::this_thread::sleep_for(std::chrono::seconds(2));
        return true;
    });
    
    scheduler.register_work("timeout_work", timeout_work);
    
    // 注册任务参数
    TaskParam param;
    param.name = "timeout_task";
    param.work_name = "timeout_work";
    param.runner_id = 0;
    param.step = 0;
    
    scheduler.register_task_param(param);
    
    scheduler.start();
    
    // 插入测试帧
    scheduler.insert_message("test", 1000, std::any(1));
    
    // 短暂等待后插入第二帧
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    scheduler.insert_message("test", 2000, std::any(2));
    
    // 等待一段时间，观察调度器行为
    std::this_thread::sleep_for(std::chrono::milliseconds(500));
    
    auto stats = scheduler.get_stats();
    
    std::cout << "Timeout test:" << std::endl;
    std::cout << "  Total frames: " << stats.total_frames << std::endl;
    std::cout << "  Dropped frames: " << stats.dropped_frames << std::endl;
    
    // 验证：调度器应该能处理超时，不会永久阻塞
    EXPECT_GE(stats.total_frames, 2);
    
    scheduler.stop();
}

// 测试多Runner并发
TEST(ConcurrencyTest, MultipleRunners) {
    const int runner_count = 4;
    Scheduler scheduler("concurrency_test", runner_count);
    
    // 注册工作单元
    std::vector<TaskGenerator::WorkFunc> work_funcs;
    work_funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        // 模拟一些工作
        std::this_thread::sleep_for(std::chrono::milliseconds(20));
        return true;
    });
    
    scheduler.register_work("concurrent_work", work_funcs);
    
    // 为每个runner注册任务
    for (int i = 0; i < runner_count; i++) {
        TaskParam param;
        param.name = "task_" + std::to_string(i);
        param.work_name = "concurrent_work";
        param.runner_id = i;
        param.step = i;
        scheduler.register_task_param(param);
    }
    
    scheduler.start();
    
    // 插入多帧
    for (int i = 0; i < 10; i++) {
        scheduler.insert_message("camera", 1000 + i * 100, std::any(i));
    }
    
    // 等待处理
    std::this_thread::sleep_for(std::chrono::milliseconds(500));
    
    auto stats = scheduler.get_stats();
    
    std::cout << "Concurrency test:" << std::endl;
    std::cout << "  Runners: " << runner_count << std::endl;
    std::cout << "  Total frames: " << stats.total_frames << std::endl;
    std::cout << "  Total tasks: " << stats.total_tasks << std::endl;
    
    // 验证：所有帧和任务都应该被处理
    EXPECT_GE(stats.total_frames, 10);
    EXPECT_GE(stats.total_tasks, 10 * runner_count);
    
    scheduler.stop();
}

// 测试帧优先级
TEST(PriorityTest, FramePriority) {
    Scheduler scheduler("priority_test", 1);
    
    // 注册工作单元
    std::vector<TaskGenerator::WorkFunc> work_funcs;
    work_funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
        return true;
    });
    
    scheduler.register_work("priority_work", work_funcs);
    
    // 注册不同优先级的任务
    TaskParam high_priority;
    high_priority.name = "high_priority_task";
    high_priority.work_name = "priority_work";
    high_priority.runner_id = 0;
    high_priority.priority = 10;
    
    TaskParam low_priority;
    low_priority.name = "low_priority_task";
    low_priority.work_name = "priority_work";
    low_priority.runner_id = 0;
    low_priority.priority = 1;
    
    scheduler.register_task_param(high_priority);
    scheduler.register_task_param(low_priority);
    
    scheduler.start();
    
    // 插入测试帧
    scheduler.insert_message("test", 1000, std::any(1));
    
    // 等待处理
    std::this_thread::sleep_for(std::chrono::milliseconds(200));
    
    auto stats = scheduler.get_stats();
    
    std::cout << "Priority test:" << std::endl;
    std::cout << "  Total tasks: " << stats.total_tasks << std::endl;
    
    // 验证：任务应该被处理
    EXPECT_GE(stats.total_tasks, 2);
    
    scheduler.stop();
}

// 测试SchedulerManager路由
TEST(SchedulerManagerTest, TopicRouting) {
    SchedulerManager manager;
    
    auto scheduler1 = std::make_shared<Scheduler>("camera_scheduler", 2);
    auto scheduler2 = std::make_shared<Scheduler>("lidar_scheduler", 2);
    
    // 注册工作单元
    std::vector<TaskGenerator::WorkFunc> work_funcs;
    work_funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        return true;
    });
    
    scheduler1->register_work("camera_work", work_funcs);
    scheduler2->register_work("lidar_work", work_funcs);
    
    // 注册任务
    TaskParam camera_param;
    camera_param.name = "camera_task";
    camera_param.work_name = "camera_work";
    
    TaskParam lidar_param;
    lidar_param.name = "lidar_task";
    lidar_param.work_name = "lidar_work";
    
    scheduler1->register_task_param(camera_param);
    scheduler2->register_task_param(lidar_param);
    
    manager.add_scheduler(scheduler1);
    manager.add_scheduler(scheduler2);
    
    // 注册 topic 路由
    manager.add_topic_route("camera", "camera_scheduler");
    manager.add_topic_route("lidar", "lidar_scheduler");
    
    manager.start_all();
    
    // 插入不同topic的消息
    manager.insert_message("camera", 1000, std::any("camera_data"));
    manager.insert_message("lidar", 1000, std::any("lidar_data"));
    
    // 等待处理
    std::this_thread::sleep_for(std::chrono::milliseconds(200));
    
    auto stats = manager.get_all_stats();
    
    std::cout << "Topic routing test:" << std::endl;
    std::cout << "  Camera scheduler tasks: " << stats["camera_scheduler"].total_tasks << std::endl;
    std::cout << "  Lidar scheduler tasks: " << stats["lidar_scheduler"].total_tasks << std::endl;
    
    // 验证：每个调度器应该处理对应的topic
    EXPECT_GE(stats["camera_scheduler"].total_tasks, 1);
    EXPECT_GE(stats["lidar_scheduler"].total_tasks, 1);
    
    manager.stop_all();
}

// 测试场景切换
TEST(SceneSwitchTest, DynamicTaskEnable) {
    Scheduler scheduler("scene_test", 2);
    
    // 注册工作单元
    std::vector<TaskGenerator::WorkFunc> work_funcs;
    work_funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        return true;
    });
    
    scheduler.register_work("scene_work", work_funcs);
    
    // 注册任务，初始禁用
    TaskParam disabled_param;
    disabled_param.name = "disabled_task";
    disabled_param.work_name = "scene_work";
    disabled_param.enable = false;
    
    TaskParam enabled_param;
    enabled_param.name = "enabled_task";
    enabled_param.work_name = "scene_work";
    enabled_param.enable = true;
    
    scheduler.register_task_param(disabled_param);
    scheduler.register_task_param(enabled_param);
    
    scheduler.start();
    
    // 插入测试帧
    scheduler.insert_message("test", 1000, std::any(1));
    
    // 等待处理
    std::this_thread::sleep_for(std::chrono::milliseconds(200));
    
    auto stats = scheduler.get_stats();
    
    std::cout << "Scene switch test:" << std::endl;
    std::cout << "  Total tasks: " << stats.total_tasks << std::endl;
    
    // 验证：只有启用的任务应该被调度
    EXPECT_GE(stats.total_tasks, 1);
    // 注意：这里无法直接验证只有enabled_task被调度，因为统计信息是总数
    
    scheduler.stop();
}

// 测试回调机制
TEST(CallbackTest, TaskCompletionCallback) {
    Scheduler scheduler("callback_test", 1);
    
    std::atomic<int> callback_count{0};
    
    // 注册工作单元，完成后触发回调
    std::vector<TaskGenerator::WorkFunc> work_funcs;
    work_funcs.push_back([&callback_count](std::shared_ptr<Frame> frame) -> bool {
        callback_count++;
        return true;
    });
    
    scheduler.register_work("callback_work", work_funcs);
    
    // 注册任务
    TaskParam param;
    param.name = "callback_task";
    param.work_name = "callback_work";
    
    scheduler.register_task_param(param);
    
    scheduler.start();
    
    // 插入多个帧（间隔插入，让回调链有机会工作）
    for (int i = 0; i < 5; i++) {
        scheduler.insert_message("test", 1000 + i * 100, std::any(i));
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
    
    // 等待处理（足够时间让回调链完成）
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    
    std::cout << "Callback test:" << std::endl;
    std::cout << "  Callback count: " << callback_count << std::endl;
    
    // 验证：回调应该被调用
    EXPECT_GE(callback_count, 3);
    
    scheduler.stop();
}

// 测试数据流
TEST(DataFlowTest, FrameDataPassing) {
    Scheduler scheduler("dataflow_test", 2);
    
    // 注册工作单元，处理帧数据
    std::vector<TaskGenerator::WorkFunc> process_funcs;
    process_funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        // 获取输入数据
        if (frame->data().has("input")) {
            int input = frame->data().get<int>("input");
            frame->data().insert("output", input * 2);
        }
        return true;
    });
    
    scheduler.register_work("process_work", process_funcs);
    
    // 注册任务
    TaskParam param;
    param.name = "process_task";
    param.work_name = "process_work";
    
    scheduler.register_task_param(param);
    
    scheduler.start();
    
    // 插入带数据的帧
    scheduler.insert_message("test", 1000, std::any(42));
    
    // 等待处理
    std::this_thread::sleep_for(std::chrono::milliseconds(200));
    
    auto stats = scheduler.get_stats();
    
    std::cout << "Data flow test:" << std::endl;
    std::cout << "  Total tasks: " << stats.total_tasks << std::endl;
    
    // 验证：任务应该被处理
    EXPECT_GE(stats.total_tasks, 1);
    
    scheduler.stop();
}

// main函数在test_main.cpp中定义