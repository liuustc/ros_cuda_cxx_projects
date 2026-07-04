#include "scheduler/frame.h"
#include <algorithm>
#include <stdexcept>

namespace scheduler {

// Frame实现
Frame::Frame(uint64_t frame_id, uint64_t timestamp)
    : frame_id_(frame_id), timestamp_(timestamp) {}

void Frame::init_task_list(int runner_id, const std::vector<std::shared_ptr<TaskInfo>>& tasks) {
    std::lock_guard<std::mutex> lock(mutex_);
    
    // 创建任务列表管理器
    auto manager = std::make_unique<TaskListManager>(runner_id);
    for (const auto& task : tasks) {
        manager->add_task(task);
        all_tasks_.push_back(task);
    }
    
    task_list_managers_[runner_id] = std::move(manager);
}

std::vector<std::shared_ptr<TaskInfo>> Frame::get_ready_tasks() {
    std::lock_guard<std::mutex> lock(mutex_);
    
    // 先处理上一轮完成的任务（关键！否则任务状态卡在RUNNING）
    process_finish_tasks_unlocked();
    
    std::vector<std::shared_ptr<TaskInfo>> ready_tasks;
    
    // 遍历每个runner的队首任务
    for (auto& [runner_id, manager] : task_list_managers_) {
        auto task = manager->front();
        if (!task) continue;
        
        // 状态转换：INIT -> WAITING
        if (task->status() == TaskStatus::INIT) {
            task->set_status(TaskStatus::WAITING);
            task->mark_checked();
        }
        
        // 检查依赖
        if (task->status() == TaskStatus::WAITING) {
            // 检查强依赖（失败则跳过当前任务）
            if (!task->check_depend_tasks()) {
                // 检查是否有依赖任务 FAILED/SKIPPED
                bool dep_failed = false;
                for (const auto& dep_name : task->param().depend_tasks) {
                    // 遍历 all_tasks_ 查找依赖状态
                    for (const auto& t : all_tasks_) {
                        if (t->param().name == dep_name && 
                            (t->status() == TaskStatus::FAILED || 
                             t->status() == TaskStatus::SKIPPED)) {
                            dep_failed = true;
                            break;
                        }
                    }
                    if (dep_failed) break;
                }
                if (dep_failed) {
                    task->skip();  // 上游失败，跳过当前任务
                    manager->pop_front();  // 从队列移除，让下一个任务有机会执行
                    // 将下一个任务设为 WAITING
                    if (!manager->empty()) {
                        auto next = manager->front();
                        if (next && next->status() == TaskStatus::INIT) {
                            next->set_status(TaskStatus::WAITING);
                        }
                    }
                }
                continue;
            }
            
            // 检查弱依赖
            if (!task->check_after_tasks()) {
                continue;
            }
            
            // 检查输入数据
            if (!task->check_input_ready()) {
                continue;
            }
            
            // 所有条件满足，标记为就绪
            task->set_status(TaskStatus::READY);
            ready_tasks.push_back(task);
        }
    }
    
    return ready_tasks;
}

void Frame::task_finish(int runner_id, const std::string& task_name, bool success) {
    std::lock_guard<std::mutex> lock(mutex_);
    
    // 缓存完成状态（不直接修改TaskStatus，避免与TriggerScheduler竞争）
    cache_finish_runner_id_[runner_id] = success;
}

void Frame::process_completions() {
    std::lock_guard<std::mutex> lock(mutex_);
    process_finish_tasks_unlocked();
}

bool Frame::all_tasks_done() const {
    std::lock_guard<std::mutex> lock(mutex_);
    
    for (const auto& task : all_tasks_) {
        if (task->status() != TaskStatus::DONE && 
            task->status() != TaskStatus::SKIPPED) {
            return false;
        }
    }
    return true;
}

void Frame::set_task_running(int runner_id) {
    std::lock_guard<std::mutex> lock(mutex_);
    auto it = task_list_managers_.find(runner_id);
    if (it != task_list_managers_.end()) {
        auto task = it->second->front();
        if (task) {
            task->set_status(TaskStatus::RUNNING);
        }
    }
}

void Frame::set_async_task_running(int runner_id) {
    // 简化实现：与set_task_running相同
    set_task_running(runner_id);
}

void Frame::process_finish_tasks_unlocked() {
    // 遍历缓存的完成状态，更新任务状态
    for (const auto& [runner_id, success] : cache_finish_runner_id_) {
        auto it = task_list_managers_.find(runner_id);
        if (it == task_list_managers_.end()) continue;
        
        auto manager = it->second.get();
        auto task = manager->front();
        if (!task) continue;
        
        // 更新任务状态
        if (success) {
            task->set_status(TaskStatus::DONE);
        } else {
            task->set_status(TaskStatus::FAILED);
        }
        completed_tasks_count_++;
        
        // 弹出队首任务
        manager->pop_front();
        
        // 如果有下一个任务，设置为WAITING
        if (!manager->empty()) {
            auto next_task = manager->front();
            if (next_task) {
                next_task->set_status(TaskStatus::WAITING);
            }
        }
    }
    
    // 清空缓存
    cache_finish_runner_id_.clear();
}

} // namespace scheduler