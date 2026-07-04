#pragma once

#include "task.h"
#include "frame.h"
#include "task_runner.h"
#include <vector>
#include <map>
#include <mutex>
#include <memory>
#include <functional>
#include <queue>
#include <condition_variable>

namespace scheduler {

// 任务生成器（缓存复用）
class TaskGenerator {
public:
    // 获取任务（缓存或创建）
    std::shared_ptr<Task> get_task(const TaskParam& param);
    
    // 注册工作单元
    using WorkFunc = std::function<bool(std::shared_ptr<Frame>)>;
    void register_work(const std::string& work_name, 
                      const std::vector<WorkFunc>& funcs);

private:
    std::mutex mutex_;
    std::map<std::string, std::shared_ptr<Task>> tasks_;
    std::map<std::string, std::vector<WorkFunc>> work_funcs_;
};

// 帧缓冲区（多帧缓存 + 跨帧仲裁）
class FrameBuffer {
public:
    FrameBuffer(int max_running_frames = 3, int max_cache_frames = 10);
    
    // 插入帧
    void insert_frame(std::shared_ptr<Frame> frame);
    
    // 获取就绪任务
    std::vector<std::pair<std::shared_ptr<TaskInfo>, std::shared_ptr<Frame>>> 
    get_ready_tasks();
    
    // 清理超时帧
    void cleanup_timeout_frames(uint64_t current_time, uint64_t timeout_ms);
    
    // 获取缓存帧数量
    size_t cache_size() const;
    
    // 获取运行帧数量
    size_t running_size() const;
    
    // 消费计数器（读取并重置）
    struct Counters {
        uint64_t completed_frames = 0;
        uint64_t completed_tasks = 0;
        uint64_t dropped_frames = 0;
    };
    Counters consume_counters();

private:
    int max_running_frames_;
    int max_cache_frames_;
    
    mutable std::mutex mutex_;
    std::vector<std::shared_ptr<Frame>> cache_frames_;
    std::vector<std::shared_ptr<Frame>> running_frames_;
    
    // 计数器
    uint64_t completed_frames_ = 0;
    uint64_t completed_tasks_ = 0;
    uint64_t dropped_frames_ = 0;
    
    // 超时清理频率控制
    uint64_t tick_count_ = 0;
    static constexpr uint64_t CLEANUP_INTERVAL = 100;  // 每100次tick清理一次
    static constexpr uint64_t DEFAULT_TIMEOUT_MS = 5000; // 默认5秒超时
    
    // 跨帧仲裁：step检查
    bool check_step_arbitration(std::shared_ptr<Frame> frame) const;
    
    // 超时清理（调用时需持有mutex_）
    void cleanup_timeout_frames_unlocked(uint64_t current_time, uint64_t timeout_ms);
};

// 调度器（单条流水线）
class Scheduler {
public:
    Scheduler(const std::string& name, int runner_count = 4);
    ~Scheduler();
    
    // 启动调度器
    void start();
    
    // 停止调度器
    void stop();
    
    // 插入消息（简化版，直接创建帧）
    void insert_message(const std::string& topic, uint64_t timestamp, 
                       const std::any& data);
    
    // 触发调度（核心调度逻辑）
    void trigger_scheduler();
    
    // 注册任务参数
    void register_task_param(const TaskParam& param);
    
    // 注册工作单元
    void register_work(const std::string& work_name,
                      const std::vector<TaskGenerator::WorkFunc>& funcs);
    
    // 获取名称
    const std::string& name() const { return name_; }
    
    // 获取统计信息
    struct Stats {
        uint64_t total_frames = 0;
        uint64_t completed_frames = 0;
        uint64_t dropped_frames = 0;
        uint64_t total_tasks = 0;
        uint64_t completed_tasks = 0;
    };
    Stats get_stats() const;

private:
    std::string name_;
    bool running_ = false;
    
    // 任务参数列表
    std::vector<TaskParam> task_params_;
    
    // 任务生成器
    std::unique_ptr<TaskGenerator> task_generator_;
    
    // 帧缓冲区
    std::unique_ptr<FrameBuffer> frame_buffer_;
    
    // 任务运行器
    std::vector<std::unique_ptr<TaskRunner>> task_runners_;
    
    // 帧ID生成器
    uint64_t frame_id_counter_ = 0;
    
    // 统计信息
    Stats stats_;
    mutable std::mutex stats_mutex_;
    
    // 调度互斥锁（重入保护）
    std::mutex scheduler_mutex_;
    
    // 创建帧
    std::shared_ptr<Frame> create_frame(uint64_t timestamp);
    
    // 构建帧任务列表
    void construct_frame_task_list(std::shared_ptr<Frame> frame);
    
    // 分配任务到runner
    void dispatch_tasks(const std::vector<std::pair<std::shared_ptr<TaskInfo>, 
                      std::shared_ptr<Frame>>>& ready_tasks);
};

// 调度器管理器（门面）
class SchedulerManager {
public:
    SchedulerManager();
    ~SchedulerManager();
    
    // 添加调度器
    void add_scheduler(std::shared_ptr<Scheduler> scheduler);
    
    // 移除调度器
    void remove_scheduler(const std::string& name);
    
    // 注册 topic 到调度器的路由
    void add_topic_route(const std::string& topic, const std::string& scheduler_name);
    
    // 插入消息（按topic路由）
    void insert_message(const std::string& topic, uint64_t timestamp,
                       const std::any& data);
    
    // 启动所有调度器
    void start_all();
    
    // 停止所有调度器
    void stop_all();
    
    // 获取统计信息
    std::map<std::string, Scheduler::Stats> get_all_stats() const;

private:
    mutable std::mutex mutex_;
    std::map<std::string, std::shared_ptr<Scheduler>> schedulers_;
    
    // topic到调度器的映射
    std::map<std::string, std::vector<std::shared_ptr<Scheduler>>> topic_scheduler_;
};

} // namespace scheduler