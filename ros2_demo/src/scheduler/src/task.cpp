#include "scheduler/task.h"
#include "scheduler/frame.h"
#include <algorithm>

namespace scheduler {

// TaskInfo实现
TaskInfo::TaskInfo(const TaskParam& param) : param_(param) {
    status_ = TaskStatus::INIT;
}

bool TaskInfo::check_depend_tasks() const {
    for (const auto& weak_task : depend_tasks_) {
        auto task = weak_task.lock();
        if (!task) continue;  // 任务已析构，视为满足
        if (task->status() == TaskStatus::FAILED || 
            task->status() == TaskStatus::SKIPPED) {
            return false;  // 上游失败，下游应跳过
        }
        if (task->status() != TaskStatus::DONE) {
            return false;  // 未完成
        }
    }
    return true;
}

bool TaskInfo::check_after_tasks() const {
    for (const auto& weak_task : after_tasks_) {
        auto task = weak_task.lock();
        if (!task) continue;  // 任务已析构，视为满足
        if (task->status() != TaskStatus::DONE) {
            return false;  // 还有未完成的 after_task
        }
    }
    return true;  // 所有 after_task 都已完成（或已失效）
}

bool TaskInfo::check_input_ready() const {
    // 简化实现：检查所有依赖任务是否完成
    return check_depend_tasks();
}

// Task实现
Task::Task(const std::string& name, const std::vector<TaskFunc>& funcs)
    : name_(name), task_funcs_(funcs) {}

bool Task::proc(std::shared_ptr<Frame> frame) {
    for (const auto& func : task_funcs_) {
        if (!func(frame)) {
            return false;  // 失败短路
        }
    }
    return true;
}

} // namespace scheduler