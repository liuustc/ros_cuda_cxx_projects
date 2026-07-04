#pragma once

#include "task.h"
#include "frame.h"
#include <thread>
#include <mutex>
#include <condition_variable>
#include <functional>
#include <atomic>
#include <memory>

namespace scheduler {

// 前向声明
class Scheduler;

// Runner状态（跨帧work互斥）
class RunnerState {
public:
    RunnerState(const std::string& name) : name_(name) {}
    
    // 设置运行状态
    bool set_running(const std::string& work_name) {
        std::lock_guard<std::mutex> lock(mutex_);
        if (running_) {
            return false;  // 已经在运行
        }
        running_ = true;
        current_work_ = work_name;
        return true;
    }
    
    void set_not_running() {
        std::lock_guard<std::mutex> lock(mutex_);
        running_ = false;
        current_work_.clear();
    }
    
    bool is_running() const {
        std::lock_guard<std::mutex> lock(mutex_);
        return running_;
    }
    
    std::string current_work() const {
        std::lock_guard<std::mutex> lock(mutex_);
        return current_work_;
    }

private:
    std::string name_;
    bool running_ = false;
    std::string current_work_;
    mutable std::mutex mutex_;
};

// 任务运行器（一个专属线程）
class TaskRunner {
public:
    TaskRunner(int id, const std::string& name, int cpu_id = -1);
    ~TaskRunner();
    
    // 启动线程
    void start();
    
    // 停止线程
    void stop();
    
    // 插入任务和帧
    void insert_task_and_frame(std::shared_ptr<Task> task, 
                              std::shared_ptr<TaskInfo> task_info,
                              std::shared_ptr<Frame> frame);
    
    // 检查线程是否存活
    bool is_running() const { return running_; }
    
    // 检查是否正在执行任务
    bool is_busy() const { return busy_; }
    
    // 获取ID
    int id() const { return id_; }
    
    // 获取名称
    const std::string& name() const { return name_; }
    
    // 设置回调函数（任务完成时调用）
    void set_callback(std::function<void()> callback) { callback_ = callback; }

private:
    int id_;
    std::string name_;
    int cpu_id_;
    
    std::thread thread_;
    std::mutex mutex_;
    std::condition_variable cv_;
    
    std::atomic<bool> exit_{false};
    std::atomic<bool> running_{false};   // 线程存活
    std::atomic<bool> busy_{false};      // 正在执行任务
    
    // 当前任务和帧
    std::shared_ptr<Task> current_task_;
    std::shared_ptr<TaskInfo> current_task_info_;
    std::shared_ptr<Frame> current_frame_;
    
    // Runner状态（跨帧work互斥）
    std::unique_ptr<RunnerState> runner_state_;
    
    // 回调函数
    std::function<void()> callback_;
    
    // 线程函数
    void real_run();
    
    // 绑定CPU
    void bind_cpu();
};

} // namespace scheduler