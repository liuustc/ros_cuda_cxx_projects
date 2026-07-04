#include <gtest/gtest.h>
#include "scheduler/scheduler.h"
#include <thread>
#include <chrono>
#include <iostream>

using namespace scheduler;

// 测试TaskParam和TaskInfo
TEST(TaskTest, TaskParamCreation) {
    TaskParam param;
    param.name = "test_task";
    param.work_name = "test_work";
    param.runner_id = 0;
    param.step = 1;
    param.priority = 10;
    param.enable = true;
    
    EXPECT_EQ(param.name, "test_task");
    EXPECT_EQ(param.work_name, "test_work");
    EXPECT_EQ(param.runner_id, 0);
    EXPECT_EQ(param.step, 1);
    EXPECT_EQ(param.priority, 10);
    EXPECT_TRUE(param.enable);
}

TEST(TaskTest, TaskInfoStatusTransitions) {
    TaskParam param;
    param.name = "test_task";
    param.work_name = "test_work";
    
    TaskInfo task_info(param);
    
    // 初始状态
    EXPECT_EQ(task_info.status(), TaskStatus::INIT);
    EXPECT_FALSE(task_info.is_checked());
    
    // 状态转换
    task_info.set_status(TaskStatus::WAITING);
    EXPECT_EQ(task_info.status(), TaskStatus::WAITING);
    
    task_info.mark_checked();
    EXPECT_TRUE(task_info.is_checked());
    
    task_info.set_status(TaskStatus::READY);
    EXPECT_EQ(task_info.status(), TaskStatus::READY);
    
    task_info.set_status(TaskStatus::RUNNING);
    EXPECT_EQ(task_info.status(), TaskStatus::RUNNING);
    
    task_info.set_status(TaskStatus::DONE);
    EXPECT_EQ(task_info.status(), TaskStatus::DONE);
}

TEST(TaskTest, TaskDependencyCheck) {
    TaskParam param1;
    param1.name = "task1";
    param1.work_name = "work1";
    
    TaskParam param2;
    param2.name = "task2";
    param2.work_name = "work2";
    param2.depend_tasks = {"task1"};
    
    auto task1 = std::make_shared<TaskInfo>(param1);
    auto task2 = std::make_shared<TaskInfo>(param2);
    
    // 建立依赖关系
    task2->add_depend_task(task1);
    
    // task1未完成，task2应该检查失败
    EXPECT_FALSE(task2->check_depend_tasks());
    
    // 完成task1
    task1->set_status(TaskStatus::DONE);
    EXPECT_TRUE(task2->check_depend_tasks());
}

TEST(TaskTest, TaskExecution) {
    TaskParam param;
    param.name = "test_task";
    param.work_name = "test_work";
    
    // 创建任务函数
    std::vector<TaskFunc> funcs;
    funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        // 简单的测试函数
        return true;
    });
    
    Task task("test_task", funcs);
    
    // 创建测试帧
    auto frame = std::make_shared<Frame>(1, 1000);
    
    // 执行任务
    bool result = task.proc(frame);
    EXPECT_TRUE(result);
}

// 测试Frame
TEST(FrameTest, FrameCreation) {
    auto frame = std::make_shared<Frame>(1, 1000);
    
    EXPECT_EQ(frame->frame_id(), 1);
    EXPECT_EQ(frame->timestamp(), 1000);
    EXPECT_EQ(frame->status(), FrameStatus::CACHE);
    // 空帧的所有任务视为完成
    EXPECT_TRUE(frame->all_tasks_done());
}

TEST(FrameTest, FrameDataTask) {
    auto frame = std::make_shared<Frame>(1, 1000);
    
    // 测试数据插入和获取
    frame->data().insert("test_key", 42);
    EXPECT_TRUE(frame->data().has("test_key"));
    EXPECT_EQ(frame->data().get<int>("test_key"), 42);
    
    // 测试不存在的键
    EXPECT_FALSE(frame->data().has("nonexistent"));
}

TEST(FrameTest, FrameTaskListManagement) {
    auto frame = std::make_shared<Frame>(1, 1000);
    
    // 创建任务参数
    TaskParam param1;
    param1.name = "task1";
    param1.work_name = "work1";
    
    TaskParam param2;
    param2.name = "task2";
    param2.work_name = "work2";
    
    auto task1 = std::make_shared<TaskInfo>(param1);
    auto task2 = std::make_shared<TaskInfo>(param2);
    
    // 初始化任务列表
    std::vector<std::shared_ptr<TaskInfo>> tasks = {task1, task2};
    frame->init_task_list(0, tasks);
    
    // 获取任务列表管理器
    auto manager = frame->get_task_list_manager(0);
    EXPECT_NE(manager, nullptr);
    EXPECT_EQ(manager->size(), 2);
    
    // 测试队首任务
    auto front = manager->front();
    EXPECT_EQ(front->param().name, "task1");
}

// 测试TaskRunner
TEST(TaskRunnerTest, TaskRunnerCreation) {
    TaskRunner runner(0, "test_runner", 0);
    
    EXPECT_EQ(runner.id(), 0);
    EXPECT_EQ(runner.name(), "test_runner");
    EXPECT_FALSE(runner.is_running());
}

TEST(TaskRunnerTest, TaskRunnerStartStop) {
    TaskRunner runner(0, "test_runner", -1);
    
    runner.start();
    EXPECT_TRUE(runner.is_running());
    
    runner.stop();
    EXPECT_FALSE(runner.is_running());
}

// 测试TaskGenerator
TEST(TaskGeneratorTest, WorkRegistration) {
    TaskGenerator generator;
    
    // 注册工作单元
    std::vector<TaskGenerator::WorkFunc> funcs;
    funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        return true;
    });
    
    generator.register_work("test_work", funcs);
    
    // 获取任务
    TaskParam param;
    param.name = "test_task";
    param.work_name = "test_work";
    
    auto task = generator.get_task(param);
    EXPECT_NE(task, nullptr);
    EXPECT_EQ(task->name(), "test_task");
}

// 测试FrameBuffer
TEST(FrameBufferTest, FrameBufferCreation) {
    FrameBuffer buffer(3, 10);
    
    EXPECT_EQ(buffer.cache_size(), 0);
    EXPECT_EQ(buffer.running_size(), 0);
}

TEST(FrameBufferTest, FrameInsertion) {
    FrameBuffer buffer(3, 10);
    
    auto frame1 = std::make_shared<Frame>(1, 1000);
    auto frame2 = std::make_shared<Frame>(2, 2000);
    
    buffer.insert_frame(frame1);
    buffer.insert_frame(frame2);
    
    EXPECT_EQ(buffer.cache_size(), 2);
}

// 测试Scheduler
TEST(SchedulerTest, SchedulerCreation) {
    Scheduler scheduler("test_scheduler", 2);
    
    EXPECT_EQ(scheduler.name(), "test_scheduler");
}

TEST(SchedulerTest, TaskRegistration) {
    Scheduler scheduler("test_scheduler", 2);
    
    // 注册工作单元
    std::vector<TaskGenerator::WorkFunc> funcs;
    funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        return true;
    });
    scheduler.register_work("test_work", funcs);
    
    // 注册任务参数
    TaskParam param;
    param.name = "test_task";
    param.work_name = "test_work";
    param.runner_id = 0;
    scheduler.register_task_param(param);
    
    // 启动调度器
    scheduler.start();
    
    // 插入消息
    scheduler.insert_message("test_topic", 1000, std::any(42));
    
    // 等待处理
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    
    // 获取统计信息
    auto stats = scheduler.get_stats();
    EXPECT_GE(stats.total_frames, 1);
    
    scheduler.stop();
}

// 测试SchedulerManager
TEST(SchedulerManagerTest, ManagerCreation) {
    SchedulerManager manager;
    
    auto scheduler1 = std::make_shared<Scheduler>("scheduler1", 2);
    auto scheduler2 = std::make_shared<Scheduler>("scheduler2", 2);
    
    manager.add_scheduler(scheduler1);
    manager.add_scheduler(scheduler2);
    
    // 启动所有调度器
    manager.start_all();
    
    // 插入消息
    manager.insert_message("topic1", 1000, std::any(1));
    manager.insert_message("topic2", 2000, std::any(2));
    
    // 等待处理
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    
    // 获取统计信息
    auto stats = manager.get_all_stats();
    EXPECT_EQ(stats.size(), 2);
    
    manager.stop_all();
}

// 集成测试
TEST(IntegrationTest, EndToEndProcessing) {
    // 创建调度器
    Scheduler scheduler("integration_scheduler", 2);
    
    // 注册工作单元
    std::vector<TaskGenerator::WorkFunc> preprocess_funcs;
    preprocess_funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        // 预处理：设置输入数据
        frame->data().insert("preprocessed", true);
        return true;
    });
    
    std::vector<TaskGenerator::WorkFunc> process_funcs;
    process_funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        // 处理：检查预处理结果
        if (!frame->data().has("preprocessed")) {
            return false;
        }
        frame->data().insert("processed", true);
        return true;
    });
    
    std::vector<TaskGenerator::WorkFunc> postprocess_funcs;
    postprocess_funcs.push_back([](std::shared_ptr<Frame> frame) -> bool {
        // 后处理：检查处理结果
        if (!frame->data().has("processed")) {
            return false;
        }
        frame->data().insert("completed", true);
        return true;
    });
    
    scheduler.register_work("preprocess", preprocess_funcs);
    scheduler.register_work("process", process_funcs);
    scheduler.register_work("postprocess", postprocess_funcs);
    
    // 注册任务参数
    TaskParam param1;
    param1.name = "preprocess_task";
    param1.work_name = "preprocess";
    param1.runner_id = 0;
    param1.step = 0;
    
    TaskParam param2;
    param2.name = "process_task";
    param2.work_name = "process";
    param2.runner_id = 1;
    param2.step = 1;
    param2.depend_tasks = {"preprocess_task"};
    
    TaskParam param3;
    param3.name = "postprocess_task";
    param3.work_name = "postprocess";
    param3.runner_id = 0;
    param3.step = 2;
    param3.depend_tasks = {"process_task"};
    
    scheduler.register_task_param(param1);
    scheduler.register_task_param(param2);
    scheduler.register_task_param(param3);
    
    // 启动调度器
    scheduler.start();
    
    // 插入测试数据
    scheduler.insert_message("camera", 1000, std::any("frame1"));
    scheduler.insert_message("camera", 2000, std::any("frame2"));
    
    // 等待处理完成
    std::this_thread::sleep_for(std::chrono::milliseconds(500));
    
    // 检查统计信息
    auto stats = scheduler.get_stats();
    EXPECT_GE(stats.total_frames, 2);
    EXPECT_GE(stats.total_tasks, 4);  // 至少完成4个任务
    
    scheduler.stop();
    
    std::cout << "Integration test completed." << std::endl;
    std::cout << "Total frames: " << stats.total_frames << std::endl;
    std::cout << "Total tasks: " << stats.total_tasks << std::endl;
}

int main(int argc, char **argv) {
    testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}