#pragma once

#include <functional>
#include <vector>
#include <string>
#include <memory>

namespace scheduler {

// 前向声明
class Frame;

// 任务状态枚举
enum class TaskStatus {
    INIT,      // 初始化
    WAITING,   // 等待依赖
    READY,     // 就绪
    RUNNING,   // 运行中
    DONE,      // 完成
    FAILED,    // 失败
    SKIPPED    // 跳过
};

// 任务函数类型
using TaskFunc = std::function<bool(std::shared_ptr<Frame>)>;

// 任务参数（声明式配置）
struct TaskParam {
    std::string name;               // 任务名称
    std::string work_name;          // 工作单元名称
    int runner_id = -1;             // 绑定的runner ID，-1表示任意
    int step = 0;                   // 步骤号（用于帧间仲裁）
    std::vector<std::string> depend_tasks;  // 强依赖任务列表
    std::vector<std::string> after_tasks;   // 弱依赖任务列表
    bool wait_next = false;         // 是否等待下一帧
    int priority = 0;               // 优先级
    bool enable = true;             // 是否启用
};

// 任务实例（DAG节点）
class TaskInfo {
public:
    TaskInfo(const TaskParam& param);
    
    // 状态管理
    TaskStatus status() const { return status_; }
    void set_status(TaskStatus status) { status_ = status; }
    
    // 依赖检查
    bool check_depend_tasks() const;
    bool check_after_tasks() const;
    bool check_input_ready() const;
    
    // 标记为已检查
    void mark_checked() { checked_ = true; }
    bool is_checked() const { return checked_; }
    
    // 跳过任务
    void skip() { status_ = TaskStatus::SKIPPED; }
    
    // 获取参数
    const TaskParam& param() const { return param_; }
    
    // 依赖任务指针管理
    void add_depend_task(std::shared_ptr<TaskInfo> task) {
        depend_tasks_.push_back(task);
    }
    
    void add_after_task(std::shared_ptr<TaskInfo> task) {
        after_tasks_.push_back(task);
    }

private:
    TaskParam param_;
    TaskStatus status_ = TaskStatus::INIT;
    bool checked_ = false;
    
    // 依赖任务指针
    std::vector<std::weak_ptr<TaskInfo>> depend_tasks_;
    std::vector<std::weak_ptr<TaskInfo>> after_tasks_;
};

// 可执行任务闭包
class Task {
public:
    Task(const std::string& name, const std::vector<TaskFunc>& funcs);
    
    // 执行任务
    bool proc(std::shared_ptr<Frame> frame);
    
    // 获取名称
    const std::string& name() const { return name_; }

private:
    std::string name_;
    std::vector<TaskFunc> task_funcs_;
};

} // namespace scheduler