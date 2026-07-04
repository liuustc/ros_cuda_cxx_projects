#include "scheduler/scheduler.h"
#include <algorithm>
#include <iostream>
#include <chrono>

namespace scheduler {

// TaskGenerator实现
std::shared_ptr<Task> TaskGenerator::get_task(const TaskParam& param) {
    std::lock_guard<std::mutex> lock(mutex_);
    
    // 查找缓存
    auto it = tasks_.find(param.name);
    if (it != tasks_.end()) {
        return it->second;
    }
    
    // 创建新任务
    auto work_it = work_funcs_.find(param.work_name);
    if (work_it == work_funcs_.end()) {
        throw std::runtime_error("Work not found: " + param.work_name);
    }
    
    auto task = std::make_shared<Task>(param.name, work_it->second);
    tasks_[param.name] = task;
    return task;
}

void TaskGenerator::register_work(const std::string& work_name,
                                 const std::vector<WorkFunc>& funcs) {
    std::lock_guard<std::mutex> lock(mutex_);
    work_funcs_[work_name] = funcs;
}

// FrameBuffer实现
FrameBuffer::FrameBuffer(int max_running_frames, int max_cache_frames)
    : max_running_frames_(max_running_frames), 
      max_cache_frames_(max_cache_frames) {}

void FrameBuffer::insert_frame(std::shared_ptr<Frame> frame) {
    std::lock_guard<std::mutex> lock(mutex_);
    
    // 添加到缓存
    cache_frames_.push_back(frame);
    
    // 如果缓存满了，丢弃最旧的帧
    while (cache_frames_.size() > max_cache_frames_) {
        cache_frames_.front()->set_status(FrameStatus::TIMEOUT);
        cache_frames_.erase(cache_frames_.begin());
        dropped_frames_++;
    }
}

std::vector<std::pair<std::shared_ptr<TaskInfo>, std::shared_ptr<Frame>>> 
FrameBuffer::get_ready_tasks() {
    std::lock_guard<std::mutex> lock(mutex_);
    
    // 周期性清理超时帧
    if (++tick_count_ >= CLEANUP_INTERVAL) {
        tick_count_ = 0;
        auto now = std::chrono::steady_clock::now().time_since_epoch().count() / 1000000;
        cleanup_timeout_frames_unlocked(now, DEFAULT_TIMEOUT_MS);
    }
    
    std::vector<std::pair<std::shared_ptr<TaskInfo>, std::shared_ptr<Frame>>> ready_tasks;
    
    // 第一步：处理所有运行帧的任务完成缓存
    for (auto& frame : running_frames_) {
        frame->process_completions();
        completed_tasks_ += frame->consume_completed_tasks();
    }
    
    // 第二步：清理已完成的运行帧
    running_frames_.erase(
        std::remove_if(running_frames_.begin(), running_frames_.end(),
                      [this](const std::shared_ptr<Frame>& frame) {
                          if (frame->all_tasks_done()) {
                              completed_frames_++;
                              return true;
                          }
                          return false;
                      }),
        running_frames_.end()
    );
    
    // 第三步：将缓存帧提升为运行帧
    while (!cache_frames_.empty() && 
           running_frames_.size() < max_running_frames_) {
        auto frame = cache_frames_.front();
        cache_frames_.erase(cache_frames_.begin());
        frame->set_status(FrameStatus::RUNNING);
        running_frames_.push_back(frame);
    }
    
    // 第四步：获取每个运行帧的就绪任务
    for (auto& frame : running_frames_) {
        auto tasks = frame->get_ready_tasks();
        for (auto& task : tasks) {
            if (check_step_arbitration(frame)) {
                ready_tasks.emplace_back(task, frame);
            }
        }
    }
    
    return ready_tasks;
}

void FrameBuffer::cleanup_timeout_frames(uint64_t current_time, uint64_t timeout_ms) {
    std::lock_guard<std::mutex> lock(mutex_);
    cleanup_timeout_frames_unlocked(current_time, timeout_ms);
}

void FrameBuffer::cleanup_timeout_frames_unlocked(uint64_t current_time, uint64_t timeout_ms) {
    // 清理缓存超时帧
    auto before = cache_frames_.size();
    cache_frames_.erase(
        std::remove_if(cache_frames_.begin(), cache_frames_.end(),
                      [current_time, timeout_ms](const std::shared_ptr<Frame>& frame) {
                          return (current_time - frame->timestamp()) > timeout_ms;
                      }),
        cache_frames_.end()
    );
    dropped_frames_ += (before - cache_frames_.size());
    
    // 清理运行超时帧
    running_frames_.erase(
        std::remove_if(running_frames_.begin(), running_frames_.end(),
                      [current_time, timeout_ms, this](const std::shared_ptr<Frame>& frame) {
                          if ((current_time - frame->timestamp()) > timeout_ms) {
                              dropped_frames_++;
                              return true;
                          }
                          return false;
                      }),
        running_frames_.end()
    );
}

size_t FrameBuffer::cache_size() const {
    std::lock_guard<std::mutex> lock(mutex_);
    return cache_frames_.size();
}

size_t FrameBuffer::running_size() const {
    std::lock_guard<std::mutex> lock(mutex_);
    return running_frames_.size();
}

FrameBuffer::Counters FrameBuffer::consume_counters() {
    std::lock_guard<std::mutex> lock(mutex_);
    Counters c;
    c.completed_frames = completed_frames_;
    c.completed_tasks = completed_tasks_;
    c.dropped_frames = dropped_frames_;
    completed_frames_ = 0;
    completed_tasks_ = 0;
    dropped_frames_ = 0;
    return c;
}

bool FrameBuffer::check_step_arbitration(std::shared_ptr<Frame> frame) const {
    // 简化实现：检查当前帧的step是否小于前一帧的step
    // 实际实现需要维护帧的顺序
    return true;
}

// Scheduler实现
Scheduler::Scheduler(const std::string& name, int runner_count) 
    : name_(name) {
    task_generator_ = std::make_unique<TaskGenerator>();
    frame_buffer_ = std::make_unique<FrameBuffer>();
    
    // 创建任务运行器
    for (int i = 0; i < runner_count; i++) {
        auto runner = std::make_unique<TaskRunner>(i, "runner_" + std::to_string(i), i);
        runner->set_callback([this]() { trigger_scheduler(); });
        task_runners_.push_back(std::move(runner));
    }
}

Scheduler::~Scheduler() {
    stop();
}

void Scheduler::start() {
    if (running_) return;
    
    running_ = true;
    
    // 启动所有runner
    for (auto& runner : task_runners_) {
        runner->start();
    }
}

void Scheduler::stop() {
    if (!running_) return;
    
    running_ = false;
    
    // 停止所有runner
    for (auto& runner : task_runners_) {
        runner->stop();
    }
}

void Scheduler::insert_message(const std::string& topic, uint64_t timestamp,
                              const std::any& data) {
    if (!running_) return;
    
    // 创建帧
    auto frame = create_frame(timestamp);
    
    // 将消息数据写入帧
    frame->data().insert("topic", topic);
    frame->data().insert("input", data);
    
    // 构建任务列表
    construct_frame_task_list(frame);
    
    // 插入帧缓冲区
    frame_buffer_->insert_frame(frame);
    
    // 更新统计
    {
        std::lock_guard<std::mutex> stats_lock(stats_mutex_);
        stats_.total_frames++;
    }
    
    // 触发调度
    trigger_scheduler();
}

void Scheduler::trigger_scheduler() {
    if (!running_) return;
    
    // 重入保护
    std::lock_guard<std::mutex> lock(scheduler_mutex_);
    
    // 获取就绪任务
    auto ready_tasks = frame_buffer_->get_ready_tasks();
    
    if (!ready_tasks.empty()) {
        // 分配任务到runner
        dispatch_tasks(ready_tasks);
    }
    
    // 消费 FrameBuffer 计数器，更新统计
    auto counters = frame_buffer_->consume_counters();
    {
        std::lock_guard<std::mutex> stats_lock(stats_mutex_);
        stats_.total_tasks += ready_tasks.size();
        stats_.completed_tasks += counters.completed_tasks;
        stats_.completed_frames += counters.completed_frames;
        stats_.dropped_frames += counters.dropped_frames;
    }
}

void Scheduler::register_task_param(const TaskParam& param) {
    task_params_.push_back(param);
}

void Scheduler::register_work(const std::string& work_name,
                             const std::vector<TaskGenerator::WorkFunc>& funcs) {
    task_generator_->register_work(work_name, funcs);
}

Scheduler::Stats Scheduler::get_stats() const {
    std::lock_guard<std::mutex> lock(stats_mutex_);
    return stats_;
}

std::shared_ptr<Frame> Scheduler::create_frame(uint64_t timestamp) {
    uint64_t frame_id = ++frame_id_counter_;
    return std::make_shared<Frame>(frame_id, timestamp);
}

void Scheduler::construct_frame_task_list(std::shared_ptr<Frame> frame) {
    // 第一步：为每个 task_param 创建 TaskInfo（全部，跨 runner 建立依赖）
    std::map<std::string, std::shared_ptr<TaskInfo>> task_map;
    std::vector<std::shared_ptr<TaskInfo>> all_task_infos;
    
    for (const auto& param : task_params_) {
        if (!param.enable) continue;
        auto task_info = std::make_shared<TaskInfo>(param);
        task_map[param.name] = task_info;
        all_task_infos.push_back(task_info);
    }
    
    // 第二步：根据 depend_tasks / after_tasks 建立依赖关系
    for (auto& task_info : all_task_infos) {
        for (const auto& dep_name : task_info->param().depend_tasks) {
            auto it = task_map.find(dep_name);
            if (it != task_map.end()) {
                task_info->add_depend_task(it->second);
            }
        }
        for (const auto& after_name : task_info->param().after_tasks) {
            auto it = task_map.find(after_name);
            if (it != task_map.end()) {
                task_info->add_after_task(it->second);
            }
        }
    }
    
    // 第三步：按 runner_id 分组，初始化每个 runner 的任务队列
    std::map<int, std::vector<std::shared_ptr<TaskInfo>>> runner_tasks;
    for (auto& task_info : all_task_infos) {
        int rid = task_info->param().runner_id;
        runner_tasks[rid].push_back(task_info);
    }
    
    for (auto& [runner_id, tasks] : runner_tasks) {
        frame->init_task_list(runner_id, tasks);
    }
}

void Scheduler::dispatch_tasks(const std::vector<std::pair<std::shared_ptr<TaskInfo>, 
                              std::shared_ptr<Frame>>>& ready_tasks) {
    for (const auto& [task_info, frame] : ready_tasks) {
        int runner_id = task_info->param().runner_id;
        
        if (runner_id >= 0 && runner_id < static_cast<int>(task_runners_.size())) {
            if (!task_runners_[runner_id]->is_busy()) {
                auto task = task_generator_->get_task(task_info->param());
                task_runners_[runner_id]->insert_task_and_frame(task, task_info, frame);
            }
        } else {
            for (auto& runner : task_runners_) {
                if (!runner->is_busy()) {
                    auto task = task_generator_->get_task(task_info->param());
                    runner->insert_task_and_frame(task, task_info, frame);
                    break;
                }
            }
        }
    }
}

// SchedulerManager实现
SchedulerManager::SchedulerManager() {}

SchedulerManager::~SchedulerManager() {
    stop_all();
}

void SchedulerManager::add_scheduler(std::shared_ptr<Scheduler> scheduler) {
    std::lock_guard<std::mutex> lock(mutex_);
    schedulers_[scheduler->name()] = scheduler;
}

void SchedulerManager::add_topic_route(const std::string& topic, const std::string& scheduler_name) {
    std::lock_guard<std::mutex> lock(mutex_);
    auto it = schedulers_.find(scheduler_name);
    if (it != schedulers_.end()) {
        topic_scheduler_[topic].push_back(it->second);
    }
}

void SchedulerManager::remove_scheduler(const std::string& name) {
    std::lock_guard<std::mutex> lock(mutex_);
    schedulers_.erase(name);
    
    // 清理topic映射
    for (auto it = topic_scheduler_.begin(); it != topic_scheduler_.end(); ) {
        auto& schedulers = it->second;
        schedulers.erase(
            std::remove_if(schedulers.begin(), schedulers.end(),
                          [&name](const std::shared_ptr<Scheduler>& s) {
                              return s->name() == name;
                          }),
            schedulers.end()
        );
        
        if (schedulers.empty()) {
            it = topic_scheduler_.erase(it);
        } else {
            ++it;
        }
    }
}

void SchedulerManager::insert_message(const std::string& topic, uint64_t timestamp,
                                     const std::any& data) {
    std::lock_guard<std::mutex> lock(mutex_);
    
    // 查找对应的调度器
    auto it = topic_scheduler_.find(topic);
    if (it != topic_scheduler_.end()) {
        for (auto& scheduler : it->second) {
            scheduler->insert_message(topic, timestamp, data);
        }
    }
}

void SchedulerManager::start_all() {
    std::lock_guard<std::mutex> lock(mutex_);
    for (auto& [name, scheduler] : schedulers_) {
        scheduler->start();
    }
}

void SchedulerManager::stop_all() {
    std::lock_guard<std::mutex> lock(mutex_);
    for (auto& [name, scheduler] : schedulers_) {
        scheduler->stop();
    }
}

std::map<std::string, Scheduler::Stats> SchedulerManager::get_all_stats() const {
    std::lock_guard<std::mutex> lock(mutex_);
    
    std::map<std::string, Scheduler::Stats> stats;
    for (const auto& [name, scheduler] : schedulers_) {
        stats[name] = scheduler->get_stats();
    }
    return stats;
}

} // namespace scheduler