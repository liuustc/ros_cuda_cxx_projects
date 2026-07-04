#pragma once

#include "task.h"
#include <map>
#include <vector>
#include <deque>
#include <mutex>
#include <any>
#include <stdexcept>
#include <string>
#include <memory>
#include <functional>

namespace scheduler {

// 前向声明
class TaskRunner;

// 帧数据容器
class FrameData {
public:
    // 插入数据
    template<typename T>
    void insert(const std::string& key, const T& value) {
        std::lock_guard<std::mutex> lock(mutex_);
        data_[key] = value;
    }
    
    // 获取数据
    template<typename T>
    T get(const std::string& key) const {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = data_.find(key);
        if (it != data_.end()) {
            return std::any_cast<T>(it->second);
        }
        throw std::runtime_error("Key not found: " + key);
    }
    
    // 检查是否存在
    bool has(const std::string& key) const {
        std::lock_guard<std::mutex> lock(mutex_);
        return data_.find(key) != data_.end();
    }

private:
    std::map<std::string, std::any> data_;
    mutable std::mutex mutex_;
};

// 任务列表管理器（每个runner一个队列）
class TaskListManager {
public:
    TaskListManager(int runner_id) : runner_id_(runner_id) {}
    
    // 添加任务
    void add_task(std::shared_ptr<TaskInfo> task) {
        std::lock_guard<std::mutex> lock(mutex_);
        task_queue_.push_back(task);
    }
    
    // 获取队首任务
    std::shared_ptr<TaskInfo> front() const {
        std::lock_guard<std::mutex> lock(mutex_);
        if (task_queue_.empty()) return nullptr;
        return task_queue_.front();
    }
    
    // 弹出队首任务
    void pop_front() {
        std::lock_guard<std::mutex> lock(mutex_);
        if (!task_queue_.empty()) {
            task_queue_.pop_front();
        }
    }
    
    // 检查队列是否为空
    bool empty() const {
        std::lock_guard<std::mutex> lock(mutex_);
        return task_queue_.empty();
    }
    
    // 获取队列大小
    size_t size() const {
        std::lock_guard<std::mutex> lock(mutex_);
        return task_queue_.size();
    }
    
    // 获取runner ID
    int runner_id() const { return runner_id_; }

private:
    int runner_id_;
    std::deque<std::shared_ptr<TaskInfo>> task_queue_;
    mutable std::mutex mutex_;
};

// 帧状态
enum class FrameStatus {
    CACHE,     // 缓存中
    RUNNING,   // 运行中
    DONE,      // 完成
    TIMEOUT    // 超时
};

// 帧类（一帧数据 + 任务图）
class Frame : public std::enable_shared_from_this<Frame> {
public:
    Frame(uint64_t frame_id, uint64_t timestamp);
    
    // 帧ID
    uint64_t frame_id() const { return frame_id_; }
    
    // 时间戳
    uint64_t timestamp() const { return timestamp_; }
    
    // 帧状态
    FrameStatus status() const { return status_; }
    void set_status(FrameStatus status) { status_ = status; }
    
    // 帧数据
    FrameData& data() { return data_; }
    const FrameData& data() const { return data_; }
    
    // 任务图管理
    void init_task_list(int runner_id, const std::vector<std::shared_ptr<TaskInfo>>& tasks);
    
    // 获取就绪任务
    std::vector<std::shared_ptr<TaskInfo>> get_ready_tasks();
    
    // 任务完成回调
    void task_finish(int runner_id, const std::string& task_name, bool success);
    
    // 处理完成的任务缓存（公开接口，供 FrameBuffer 调用）
    void process_completions();
    
    // 消费已完成任务计数
    uint64_t consume_completed_tasks() {
        std::lock_guard<std::mutex> lock(mutex_);
        uint64_t c = completed_tasks_count_;
        completed_tasks_count_ = 0;
        return c;
    }
    
    // 检查所有任务是否完成
    bool all_tasks_done() const;
    
    // 获取当前完成的最大 step（用于帧间仲裁）
    int get_current_step() const;
    
    // 设置步骤号（保留兼容）
    int get_step() const { return step_; }
    void set_step(int step) { step_ = step; }
    
    // 设置任务运行状态（使用 task_info 自身的 runner_id，而非实际执行 runner 的 id）
    void set_task_running(std::shared_ptr<TaskInfo> task_info);
    void set_async_task_running(std::shared_ptr<TaskInfo> task_info);
    
    // 获取任务列表管理器
    TaskListManager* get_task_list_manager(int runner_id) {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = task_list_managers_.find(runner_id);
        if (it != task_list_managers_.end()) {
            return it->second.get();
        }
        return nullptr;
    }

private:
    uint64_t frame_id_;
    uint64_t timestamp_;
    FrameStatus status_ = FrameStatus::CACHE;
    int step_ = 0;
    
    FrameData data_;
    mutable std::mutex mutex_;
    
    // 每个runner的任务队列
    std::map<int, std::unique_ptr<TaskListManager>> task_list_managers_;
    
    // 所有任务
    std::vector<std::shared_ptr<TaskInfo>> all_tasks_;
    
    // 任务状态缓存（用于批量更新）
    std::map<int, bool> cache_finish_runner_id_;
    
    // 已完成任务计数
    uint64_t completed_tasks_count_ = 0;
    
    // 处理完成的任务（调用时需持有mutex_）
    void process_finish_tasks_unlocked();
};

} // namespace scheduler