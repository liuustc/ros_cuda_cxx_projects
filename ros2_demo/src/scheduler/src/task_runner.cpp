#include "scheduler/task_runner.h"
#include <iostream>
#include <sched.h>
#include <pthread.h>

namespace scheduler {

TaskRunner::TaskRunner(int id, const std::string& name, int cpu_id)
    : id_(id), name_(name), cpu_id_(cpu_id) {
    runner_state_ = std::make_unique<RunnerState>(name);
}

TaskRunner::~TaskRunner() {
    stop();
}

void TaskRunner::start() {
    if (running_) return;
    
    exit_ = false;
    running_ = true;
    thread_ = std::thread(&TaskRunner::real_run, this);
}

void TaskRunner::stop() {
    if (!running_) return;
    
    exit_ = true;
    cv_.notify_all();
    
    if (thread_.joinable()) {
        thread_.join();
    }
    
    running_ = false;
}

void TaskRunner::insert_task_and_frame(std::shared_ptr<Task> task,
                                      std::shared_ptr<TaskInfo> task_info,
                                      std::shared_ptr<Frame> frame) {
    std::lock_guard<std::mutex> lock(mutex_);
    
    current_task_ = task;
    current_task_info_ = task_info;
    current_frame_ = frame;
    busy_ = true;
    
    // 通知线程有新任务
    cv_.notify_one();
}

void TaskRunner::real_run() {
    // 绑定CPU
    if (cpu_id_ >= 0) {
        bind_cpu();
    }
    
    while (!exit_) {
        std::unique_lock<std::mutex> lock(mutex_);
        
        // 等待任务或退出信号
        cv_.wait(lock, [this] { 
            return exit_ || (current_task_ && current_frame_); 
        });
        
        if (exit_) break;
        
        // 获取当前任务
        auto task = current_task_;
        auto task_info = current_task_info_;
        auto frame = current_frame_;
        
        // 清空当前任务
        current_task_ = nullptr;
        current_task_info_ = nullptr;
        current_frame_ = nullptr;
        
        lock.unlock();
        
        if (!task || !task_info || !frame) continue;
        
        // 设置任务运行状态
        frame->set_task_running(task_info);
        
        // 检查跨帧work互斥
        std::string work_name = task_info->param().work_name;
        if (!runner_state_->set_running(work_name)) {
            // 该work正在运行，跳过任务
            frame->task_finish(id_, task_info->param().name, false);
            busy_ = false;
            if (callback_) callback_();
            continue;
        }
        
        // 执行任务
        bool success = task->proc(frame);
        
        // 任务完成
        frame->task_finish(id_, task_info->param().name, success);
        
        // 重置运行状态
        runner_state_->set_not_running();
        busy_ = false;
        
        // 调用回调（触发下一次调度）
        if (callback_) {
            callback_();
        }
    }
}

void TaskRunner::bind_cpu() {
    cpu_set_t cpuset;
    CPU_ZERO(&cpuset);
    CPU_SET(cpu_id_, &cpuset);
    
    int result = pthread_setaffinity_np(pthread_self(), sizeof(cpu_set_t), &cpuset);
    
    if (result != 0) {
        std::cerr << "Failed to bind thread to CPU " << cpu_id_ << std::endl;
    }
}

} // namespace scheduler