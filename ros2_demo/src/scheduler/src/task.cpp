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
        if (!task || task->status() != TaskStatus::DONE) {
            return false;
        }
    }
    return true;
}

bool TaskInfo::check_after_tasks() const {
    for (const auto& weak_task : after_tasks_) {
        auto task = weak_task.lock();
        if (!task || task->status() == TaskStatus::DONE) {
            return true;  // 至少一个after任务完成
        }
    }
    return after_tasks_.empty();  // 如果没有after任务，返回true
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