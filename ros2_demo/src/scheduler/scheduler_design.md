# 调度器设计文档

> 本文件记录声明式 DAG 调度器的设计思路，作为实现参考。

## 1. 架构总览

```
SchedulerManager (门面)
  └─ Scheduler[] (单条流水线)
       ├─ SyncModule      (传感器对齐 → SensorFrame)
       ├─ FrameControler  (SensorFrame → Frame + 任务图)
       ├─ FrameBuffer     (多帧缓存 + 跨帧仲裁)
       ├─ TaskGenerator   (TaskParam → Task 缓存)
       └─ TaskRunner[]    (执行流)

Frame (一帧数据 + 任务图)
  ├─ frame_data_ (输入输出数据池)
  └─ TaskListManager (每个 runner 一个 task 队列)

TaskInfo (任务实例，DAG 节点)
  └─ task_param (work/runner/step/func/depend/after/wait_next)

Task (可执行闭包)
  └─ task_funcs_: [阶段1, 阶段2, ...]
```

## 2. 核心设计哲学

### 2.1 声明式 DAG

通过 TaskParam 声明任务依赖关系：
- `depend_tasks`: 强依赖，所有前置任务必须 DONE
- `after_tasks`: 弱依赖，所有前置任务完成才放行
- `step`: 帧间错峰标识
- `enable`: 场景级开关

### 2.2 中心化事件驱动 tick

TriggerScheduler 是唯一的派发入口：
- 单线程决策，避免并发竞态
- 事件驱动：消息到达、task 完成、超时都会触发
- 无轮询：runner 完成立即回调

### 2.3 分层守卫

每层只解决一个问题，正交不重叠：
- 消息层：时间戳对齐、乱序丢弃
- 帧层：跨帧仲裁、依赖检查
- 任务层：状态机管理
- 执行层：绑核线程、work 互斥

## 3. 调度流程

```
消息到达 → InsertMsg → FrameBuffer.InsertFrame (cache)
  ↓
TriggerScheduler
  ↓
FrameBuffer.GetReadyTasks:
  1. process_completions (处理上轮完成缓存)
  2. 清理 all_tasks_done 的运行帧
  3. 提升 cache → running (max_running_frames 限制)
  4. 遍历运行帧，INIT→WAITING→检查依赖→READY
  ↓
DispatchTasks: 按 runner_id 指定或找空闲 runner
  ↓
TaskRunner.RealRun:
  set_task_running → work 互斥 → task->Proc → task_finish → callback
  ↓
callback_() → TriggerScheduler (闭环)
```

## 4. 任务状态机

```
INIT → WAITING → READY → RUNNING → DONE
                  ↓
               FAILED / SKIPPED
```

- INIT→WAITING: 首次被调度器评估
- WAITING→READY: 所有依赖满足
- READY→RUNNING: 被派发到 runner
- RUNNING→DONE: 执行成功
- RUNNING→FAILED: 执行失败
- WAITING→SKIPPED: 上游依赖失败，跳过

## 5. 时序保证

| 层 | 机制 |
|---|---|
| 消息层 | 时间戳容差 + 超时丢弃 |
| 帧层 | max_running_frames 流水深度 + step 仲裁 |
| 任务层 | depend_task / after_task / input_ready 三种依赖 |
| 执行层 | IsRunning 单 task 守卫 + RunnerState work 互斥 |

## 6. 优先级机制

| 维度 | 说明 |
|---|---|
| 帧间 | running_frames 按到达顺序，N 帧优先于 N+1 |
| 帧内 | runner_id 升序评估 |
| 任务级 | priority 字段（声明优先级） |
| 线程级 | OS 实时优先级 + 绑核 |

## 7. 关键设计约束

1. **TaskStatus 写操作仅在 TriggerScheduler 中发生**：runner 不直接改状态，避免数据竞争
2. **Runner 内无队列**：队列管理在 Frame 的 TaskListManager 中，runner 只消费指针
3. **work 互斥**：同一 work 不可重入，防止硬件上下文冲突
4. **失败传播**：depend_task FAILED 时，下游任务自动 SKIPPED

## 8. 已知限制（待实现）

- `check_step_arbitration`: 跨帧 step 仲裁未实现
- `wait_next_task`: 跨帧等待机制未实现
- `check_input_ready`: 数据依赖检查未实现（当前等同于 check_depend_tasks）
- `priority`: 优先级调度未实现（按注册顺序派发）
- `cleanup_timeout_frames`: 超时清理已实现但需手动调用
