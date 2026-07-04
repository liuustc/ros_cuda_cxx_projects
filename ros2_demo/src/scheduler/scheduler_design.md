# Runner / Task / Scheduler 抽象层全景梳理

> 研究对象：`（内部路径已脱敏）`
> 整理日期：2026-06-29

## 1. 层次总览（自顶向下）

```
┌─────────────────────────────────────────────────────────────────┐
│ SchedulerManager (门面)                                          │
│   - 持有多个 Scheduler，按 topic 路由消息，按场景切换            │
└─────────────────────────────────────────────────────────────────┘
        │ owns                                                    │ routes msg
        ▼                                                          ▼
┌─────────────────────────────────────────────────────────────────┐
│ Scheduler (单条流水线)                                          │
│   - SyncModule + FrameControler + FrameBuffer + TaskRunners     │
│   - TriggerScheduler() = 唯一派发入口                            │
└─────────────────────────────────────────────────────────────────┘
        │ owns
        ├─ SyncModule      (传感器对齐 → SensorFrame)
        ├─ FrameControler  (SensorFrame → Frame + 任务图)
        ├─ FrameBuffer     (多帧缓存 + 跨帧仲裁)
        ├─ TaskGenerator   (TaskParam → Task 缓存)
        └─ TaskRunner[]    (执行流)
                │
                ▼
┌─────────────────────────────────────────────────────────────────┐
│ Frame (一帧数据 + 任务图)                                       │
│   - frame_data_ (输入输出数据池)                                 │
│   - TaskListManager (每个 runner 一个 task 队列)                │
└─────────────────────────────────────────────────────────────────┘
        │ contains
        ▼
┌─────────────────────────────────────────────────────────────────┐
│ TaskInfo (任务实例，DAG 节点)                                    │
│   - task_param (work/runner/step/func/depend/after/wait_next)   │
│   - task_status (INIT→WAITING→READY→RUNNING→DONE)              │
└─────────────────────────────────────────────────────────────────┘
        │ bound to
        ▼
┌─────────────────────────────────────────────────────────────────┐
│ Task (可执行闭包)                                                │
│   - task_funcs_: [Preprocess, InferAsync, WaitInferDone, Post]  │
│   - Proc(frame): 顺序执行 task_funcs_                            │
└─────────────────────────────────────────────────────────────────┘
        │ bound to
        ▼
┌─────────────────────────────────────────────────────────────────┐
│ BaseModelWork (代码单元，工厂注册)                              │
│   - PreprocessImpl / InferAsyncImpl / WaitInferDoneImpl / Post  │
└─────────────────────────────────────────────────────────────────┘
```

## 2. 各层物理含义

| 层 | 物理含义 | 生命周期 | 数量级 |
|---|---|---|---|
| **SchedulerManager** | 整个感知系统（多场景/多模式） | 进程级 | 1 |
| **Scheduler** | 一条感知流水线（一个 yaml） | 进程级 | 1-4（场景A/场景B/场景C 等） |
| **SyncModule** | 传感器时间戳对齐器 | scheduler 内 | 1/scheduler |
| **FrameControler** | 帧工厂 + 任务图生成器 | scheduler 内 | 1/scheduler |
| **FrameBuffer** | 多帧流水线缓存 + 跨帧仲裁器 | scheduler 内 | 1/scheduler |
| **TaskRunner** | 一个专属线程（绑核） | scheduler 内 | 8-12/scheduler |
| **Frame** | 一帧感知数据 + 任务图 | 帧级（几十 ms） | 1-5 在飞 |
| **TaskInfo** | 一个 DAG 节点（调度视角） | 帧级 | ~30/帧 |
| **Task** | 可执行闭包（缓存复用） | 进程级缓存 | ~30 总数 |
| **BaseModelWork** | 模型推理代码（工厂注册） | 进程级单例 | ~30 |

## 3. 自顶向下调度流程

### 3.1 SchedulerManager → Scheduler（消息路由）

```
SchedulerManager::InsertMsg(topic, ts, msg)              [scheduler_manager.cpp:172]
  ├─ 乱序 ts 丢弃                                         [:177]
  ├─ 查 topic_scheduler_[topic]                           [:196]
  └─ 对每个命中的 scheduler:
      scheduler->InsertMsg(topic, ts, msg)
```

**调度方式**：按 topic 路由，无优先级，全量转发。
**时序保证**：ts 乱序丢弃，保证时间递增。

### 3.2 Scheduler → SyncModule（消息对齐）

```
Scheduler::InsertMsg(topic, ts, msg)                     [scheduler.cpp:97]
  ├─ insert_msg_mutex_ 加锁
  ├─ sync_module_->InsertMsg(name, msg, ts, sync_data)
  └─ if sync_data 非空:
      frame_controler_->InsertSyncedMsg(sync_data)       [→ 3.3]
      TriggerScheduler()                                  [→ 3.4]
```

```
SyncModule::InsertMsg                                    [sync_module.cpp:293]
  ├─ check_data_mutex_ 加锁
  ├─ 投递到已有 SensorFrame 或新建
  ├─ 同步函数：lidar ts<base&&ts+100>base；其他 |Δ|<tolerance
  ├─ base_topics 全到齐 → sync_data + frame_start_status=1
  └─ 后台线程 100ms 清理超时帧
```

**调度方式**：按 base_timestamp 聚合，时间戳容差匹配。
**时序保证**：`sync_tolerance`（15ms）+ `frame_timeout`（150-200ms）。
**优先级**：无。

### 3.3 FrameControler → Frame（帧创建）

```
FrameControler::InsertSyncedMsg(sync_data)               [frame_control.cpp:306]
  ├─ if 新帧:
  │   new Frame(base_timestamp)
  │   分配 frame_id
  │   SetFrameStatus (Normal/ReuseLidar/PureVision)
  │   ConstructFrameTaskList(frame)                       [:382]
  │     ├─ DynamicControlerManager::GetControlerAndScene  [:397]
  │     │   按 frame_id 快照 scene
  │     ├─ 对每个 task_param:
  │     │   计算 actual_frame_valid / actual_enable
  │     │   命中条件: frame_idx>=begin && (idx-begin)%step==0 && enable
  │     │   → 建 TaskInfo
  │     ├─ 连接 depend_tasks / after_tasks 指针
  │     └─ frame->InitTaskList(runner_id, task_list)
  └─ frame_buffer_->InsertFrame(frame)                   [→ 3.4]
```

**调度方式**：按 yaml tasklist 静态声明 + 动态 scene 过滤。
**时序保证**：`depend_tasks`/`after_tasks` 共享指针连接（帧内拓扑序）。
**优先级**：`frame_valid` 决定哪些帧跑该 task（帧级 on/off）；`enable` 决定场景级开关。

### 3.4 Scheduler → FrameBuffer → Frame（跨帧仲裁）

```
Scheduler::TriggerScheduler()                            [scheduler.cpp:365]
  ├─ mutex_ 加锁（重入保护）
  ├─ frame_buffer_->GetReadyTasks(ready_tasks)           [frame_buffer.cpp:26]
  │   ├─ 清理 cache 超时帧
  │   ├─ 弹出 AllTasksDone 帧
  │   ├─ 弹出 step 超时帧
  │   ├─ cache→running 提升（最多 max_running_step_ 帧）
  │   ├─ 对每个 running frame:
  │   │   frame->GetReadyTasksAndDropWaitingInput()
  │   │     └─ GetReadyTasksImpl:                         [frame.cpp:316]
  │   │         遍历每个 runner 队首 TaskInfo:
  │   │           INIT→WAITING (MarkChecked)
  │   │           CheckTaskDepend (强依赖，失败则 SkipTask)
  │   │           CheckTaskAfter (弱依赖)
  │   │           CheckInputReady (数据依赖)
  │   │           通过 → READY
  │   ├─ 跨帧 step 仲裁:                                  [:146]
  │   │   cur_frame.GetStep() < prev_frame.GetStep() 才放行
  │   ├─ wait_next_task 检查下一帧任务状态                [:168]
  │   │   首次等待 → AddTimerCb(timeout, TriggerScheduler)
  │   └─ used_task_runner 防同 tick 同 runner 派两次
  └─ for (task_info, frame) in ready_tasks:
      task = task_generator_->GetTask(task_param)         [→ 3.5]
      if !task_runners_[id]->IsRunning():
        task_runners_[id]->InsertTaskAndFrame(...)        [→ 3.6]
```

**调度方式**：中心化 tick，全局扫描 ready task。
**时序保证**：
- 帧内：depend_task（成功依赖）/ after_task（完成依赖）/ inputs（数据依赖）
- 帧间：step 仲裁（cur < prev 才放行）+ wait_next_task（跨帧等待）
- 丢弃：cache 超时 / step 超时 / 输入等待超时

**优先级**：
- 帧间：running_frames_ 按到达顺序，N 帧优先于 N+1
- 帧内：runner_id 升序评估
- task 级：`priority` 字段（声明优先级，scheduler 当前实现按声明顺序派发）

### 3.5 TaskGenerator → Task（闭包生成）

```
TaskGenerator::GetTask(task_param)                       [task.cpp:45]
  ├─ 查 tasks_ 缓存，命中则返回
  └─ 未命中:
      CreateModelTask 或 CreateCommonTask
      把 func: [Preprocess, InferAsync, ...] 编译成 vector<TaskFunc>
      每个 TaskFunc = lambda 调 model_job->对应阶段
      缓存并返回
```

**调度方式**：惰性创建 + 缓存复用。
**时序保证**：task_funcs_ 顺序 = yaml func 顺序，runner 内串行执行。
**优先级**：无（task 是被动闭包）。

### 3.6 Scheduler → TaskRunner（派发）

```
TaskRunner::InsertTaskAndFrame(task, task_info, frame)   [task_runner.cpp:26]
  ├─ mutex_ 加锁
  ├─ running_ = true
  ├─ task_ = task; frame_ = frame; task_info_ = task_info
  ├─ frame->SetTaskRunning(runner_id) 或 SetAsyncTaskRunning
  └─ cv_.notify_one()                                     [→ 3.7]
```

**调度方式**：直接指定，无队列（runner 内只存一份）。
**时序保证**：`IsRunning()` 守卫，单 runner 一次一个 task。
**优先级**：无（scheduler 已决定派哪个）。

### 3.7 TaskRunner 内部执行

```
TaskRunner::RealRun()                                    [task_runner.cpp:82]
  while !exit_:
    cv_.wait(exit_ || running_)                           [:97]
    if async_run:
      runner_state_->SetAsyncRunning(work_name)           [跨帧 work 互斥]
      async_runner_->InsertTaskAndFrame(...)              [→ AsyncTaskRunner]
    else:
      if !runner_state_->SetRunning(work_name):            [跨帧 work 互斥]
        frame->TaskFinish(runner_id, task_name, false)    [skip]
        continue
      task->Proc(frame_)                                  [→ 3.8]
      frame->TaskFinish(runner_id, task_name, result)     [→ Frame 缓存完成]
      runner_state_->SetNotRunning(work_name)
    running_ = false
    callback_()                                           [= TriggerScheduler, 回到 3.4]
```

**调度方式**：事件驱动（cv_ 唤醒），无抢占。
**时序保证**：`RunnerState` 跨帧同 work 互斥（防硬件重入）。
**优先级**：线程级 SCHED_RR + priority=20（硬件平台），绑核 cpu_id。

### 3.8 Task → BaseModelWork（阶段执行）

```
Task::Proc(frame)                                        [task.cpp:13]
  for func in task_funcs_:
    if !func(frame): break                               [失败短路]
  return result
```

```
BaseModelWork 各阶段:
  Preprocess(frame)    → PreprocessImpl, 失败 skip_=true   [model_work.cpp:13]
  InferAsync()         → InferAsyncImpl (提交推理任务, 不阻塞)  [:60]
  WaitInferDone()      → WaitInferDoneImpl (阻塞等推理完成)     [:93]
  PostProcess(frame)   → PostProcessImpl
                         frame->InsertData(name, ptr)      [:142]
                         (输出写回 frame_data_)
```

**调度方式**：顺序执行 task_funcs_，失败短路。
**时序保证**：四阶段串行；`InferAsync` 提交后不阻塞，`WaitInferDone` 才阻塞——这是"runner 不等待推理"的关键。
**优先级**：无。

### 3.9 Frame 完成回写

```
Frame::TaskFinish(runner_id, task_name, result)          [frame.cpp:162]
  ├─ 写入 cache_finish_runner_id_（不改 TaskStatus）
  └─ 下次 TriggerScheduler 时 ProcessFinishTask:          [:197]
      ├─ task_mutex_ 加锁
      ├─ cache 中的任务标 DONE
      ├─ 弹出 runner 队首，下一个 task WAITING
      └─ 触发下游 CheckInputReady 通过（因 PostProcess 已写 frame_data_）
```

**关键**：TaskStatus 只在 `TriggerScheduler`（持有 mutex_）中改，runner 不直接改——避免数据竞争。

## 4. 调度闭环图

```
消息到达 ─→ InsertMsg ─→ SyncModule ─→ FrameControler ─→ FrameBuffer.InsertFrame
                                                                │
                                                                ▼
                                                          ┌──────────┐
                                                          │ Trigger  │◄─────┐
                                                          │ Scheduler│      │ callback_
                                                          └──────────┘      │
                                                                │            │
                                          ┌─────────────────────┴────────────┐
                                          ▼                                   ▼
                                   GetReadyTasks                        TaskRunner::RealRun
                                   (跨帧仲裁+依赖检查)                        │
                                          │                                   │
                                          ▼                                   ▼
                                   派发 task+frame                       task->Proc(frame)
                                   给空闲 runner                               │
                                          │                                   ▼
                                          └──────────────────────────► frame->TaskFinish
                                                                         (缓存完成)
                                                                              │
                                                                              └──→ callback_() 回到 TriggerScheduler
```

**闭环特性**：
- 事件驱动：消息到达、task 完成、wait_next 超时都会触发
- 无轮询：runner 完成立即回调，不靠定时器
- 中心化：所有派发决策在 TriggerScheduler 内，mutex_ 串行

## 5. 时序保证汇总（分层）

| 层 | 时序机制 | 粒度 |
|---|---|---|
| SyncModule | `sync_tolerance` 时间戳容差 + `frame_timeout` 超时丢弃 | 消息级 |
| FrameControler | `frame_id` 递增 + scene 快照（`frame_scene_map_`） | 帧级 |
| FrameBuffer | `max_running_step_` 流水深度 + step 仲裁 + `wait_next_task` | 帧间 |
| Frame | `depend_task`/`after_task`/`inputs` 三种 DAG 边 | 帧内 task 级 |
| TaskRunner | `IsRunning()` 单 task 守卫 + `RunnerState` work 互斥 | runner 级 |
| Task | `task_funcs_` 顺序执行 + 失败短路 | 阶段级 |
| BaseModelWork | Pre/InferAsync/Wait/Post 四阶段契约 | 代码级 |

## 6. 优先级汇总（分层）

| 层 | 优先级机制 | 说明 |
|---|---|---|
| SchedulerManager | scene 切换（场景1/场景2/场景3） | 整条流水线切换 work 集合 |
| FrameControler | `frame_valid:[begin,step]` | 帧级 on/off（如偶数帧才跑） |
| FrameControler | `enable` + dynamic_control | 场景级 work 开关 |
| FrameBuffer | running_frames_ 到达顺序 | N 帧优先于 N+1 帧 |
| FrameBuffer | step 仲裁 | cur_step < prev_step 才放行 |
| Scheduler | `priority` 字段（声明优先级） | 同 runner 队列内排序依据 |
| Scheduler | `used_task_runner` | 同 tick 同 runner 只派一个 |
| TaskRunner | `IsRunning()` | 空闲才接新 task |
| TaskRunner | 线程 SCHED_RR + priority=20 | OS 级实时优先级 |
| TaskRunner | `cpu_id` 绑核 | 避免核间迁移 |
| RunnerState | work 互斥 | 同模型不可重入 |
| AsyncTaskRunner | 高/低优先级线程池 | async task 优先级隔离 |

## 7. 各层调度方式对比

| 层 | 调度范式 | 决策时机 | 状态位置 |
|---|---|---|---|
| SchedulerManager | 路由表 | 消息到达时 | `topic_scheduler_` map |
| SyncModule | 时间戳聚合 | 消息到达时 | `frame_buffer_` (SensorFrame) |
| FrameControler | 声明式 DAG + 动态过滤 | 帧创建时 | `task_work_flow_` |
| FrameBuffer | 中心化 tick + 跨帧仲裁 | 每次触发 | `running_frames_`/`cache_frames_` |
| TaskRunner | 事件驱动（cv_） | task 派发时 | runner 内部三指针 |
| Task | 顺序执行 | runner 调用 Proc 时 | `task_funcs_` |
| BaseModelWork | 阶段契约 | runner 调用各阶段时 | work 内部状态 |

## 8. 一句话总结

整套抽象是**声明式 DAG + 中心化事件驱动 tick + 分层守卫**的调度体系：

- **SchedulerManager/SyncModule/FrameControler** 负责"准备数据"——把传感器消息对齐成帧、按 yaml 装配任务图
- **FrameBuffer/Scheduler::TriggerScheduler** 负责"决策派发"——跨帧仲裁、依赖检查、按空闲 runner 派 task
- **TaskRunner/Task/BaseModelWork** 负责"执行"——绑核线程串行跑四阶段，完成回调闭环

时序靠**分层守卫**保证：SyncModule 管消息时序、FrameBuffer 管帧间时序、Frame 管帧内 task 时序、RunnerState 管硬件重入。优先级靠**多维度叠加**：scene（最高层开关）→ frame_valid（帧级）→ step（错峰）→ priority（声明）→ OS 实时优先级（线程级）。每层只解决一个问题，正交不重叠——这就是这套抽象的设计哲学。





## DeepSeek分析

这份梳理已经极其详尽，堪称“源码级”的架构文档。基于这份材料，我为你提炼**架构的核心设计哲学**，并补充几个**容易被忽略的工程陷阱与设计权衡**，以助你更深刻地理解这套抽象层。

### 一、架构核心设计哲学（3个“一次”）

1. **一次对齐，全局复用（数据面）**：`SyncModule` 承担了最复杂的传感器时间戳乱序容忍与聚合逻辑。一旦拼成 `SensorFrame`，后续 `Frame`、`TaskInfo`、`Task` 全生命周期均**不再处理时间戳**，只处理 `frame_id` 和逻辑依赖。这极大简化了后续调度器的复杂度。
2. **一次决策，批量派发（控制面）**：`TriggerScheduler` 是**单线程（持有 mutex_）**的全局决策中心。它在一个 tick 内完成所有 `Frame` 的状态遍历、依赖解析、跨帧仲裁，并批量派发。这种设计避免了多线程并发修改 DAG 状态带来的死锁和竞态，将调度复杂度降为 O(帧数 × 任务数)，虽然牺牲了部分实时性（调度耗时算在流水线开销中），但换来了绝对的确定性。
3. **一次绑定，异步解耦（执行面）**：`TaskRunner` 内部**没有任务队列**（只存一份当前任务），队列管理实际上在 `Frame` 的 `TaskListManager` 中。这意味着 `TaskRunner` 是“无情的执行奴隶”，它不关心下一个任务是什么，只负责把手里的事干完并回调。这种“调度器维护队列，执行器仅消费指针”的模式，使得 CPU 绑核线程无需维护复杂的内存队列，减少了 cache miss。

---

### 二、文档未明说但极其关键的“隐含规则”

| 隐含规则 | 具体表现 | 若违反的后果 |
| :--- | :--- | :--- |
| **TaskStatus 写操作仅在 TriggerScheduler 持有 mutex_ 时发生** | 文档 3.9 提到 `TaskFinish` 只写 cache，真正的 `DONE` 状态由下一次 `TriggerScheduler` 的 `ProcessFinishTask` 改写。 | Runner 线程若直接改 `TaskInfo` 状态，会和 `TriggerScheduler` 发生数据竞争，导致 DAG 依赖检查（`CheckTaskDepend`）读到中间状态，引发 task 被错误 Skip 或永久 WAITING。 |
| **`wait_next_task` 是“弱等待”** | 它依赖 `AddTimerCb` 超时重触发 `TriggerScheduler`，而非阻塞当前线程。 | 若超时时间设置过长，会导致流水线前端（SyncModule）虽然数据就绪，但后端因等待旧帧结束而无法推进，造成帧积累丢弃（`frame_buffer` 溢出）。 |
| **`step` 仲裁是“帧间错峰”，而非“帧间优先级”** | `cur_step < prev_step` 才放行。 | 这意味着如果第 N 帧的 Step 2 没跑完，第 N+1 帧的 Step 1 即使依赖全满足，也必须等待。这是为了**保证同一模型在 硬件上按顺序推理**（防止乱序导致硬件上下文冲突），而非逻辑上的必要性。 |
| **`InferAsync` 与 `WaitInferDone` 必须成对且同 Runner** | 代码中虽提交给推理引擎，但 `WaitInferDone` 阻塞当前 `TaskRunner` 线程。 | 如果 `Preprocess` 和 `Post` 耗时极短，而 推理耗时很长，这个 Runner 线程会被卡死在 `WaitInferDone` 上。此时即使该 Runner 绑了核，也无法处理后续 Step，**等价于同步推理**。这是设计上留给用户的陷阱——必须确保预处理/后处理时间远小于推理时间，或使用 `AsyncTaskRunner` 池化来缓解。 |

---

### 三、现有架构的“性能天花板”与潜在优化点

基于文档的当前设计，存在三个明显的扩展瓶颈：

1. **`TriggerScheduler` 单核瓶颈**：所有调度逻辑（遍历所有 Frame 的所有 TaskInfo）串行执行。随着场景复杂化（任务数增至 50+），单次 `TriggerScheduler` 耗时可能超过 1ms，在高频感知（10Hz）下占用 1% CPU 尚可，但若扩展到 20Hz，调度开销将显著增加。
   - *优化方向*：引入“增量式调度”，仅扫描状态发生变化的 Frame（如刚完成 Task 的帧，或刚插入的新帧），而非全量扫描。

2. **`FrameBuffer` 的 `running_frames_` 深拷贝**：`GetReadyTasks` 中为了做跨帧仲裁，频繁对 `running_frames_` 做遍历和状态读取。虽然加了大锁，但在多帧（5帧）多任务（30个）场景下，指针容器的频繁迭代会拖累 Cache。
   - *优化方向*：将 `running_frames_` 改为 `std::list` 或 intrusive list，并维护一个 `ready_task_heap`（按优先级和 step 排序），让调度器直接 pop 堆顶，减少无效遍历。

3. **`BaseModelWork` 工厂注册的单例化**：虽利于复用，但 `PreprocessImpl` 内部若持有线程局部存储（TLS）或临时缓存，在多 `Scheduler`（如 场景A 和 场景B 同时运行）共用同一 Work 实例时，可能因并发调用导致数据污染（尽管有 `RunnerState` 互斥，但互斥仅阻止同 work 重入，不阻止不同 Scheduler 的同一 work 同时进入）。
   - *优化方向*：Work 内部应设计为**无状态（Stateless）**，所有临时变量必须存放在 `Frame` 的 `frame_data_` 中。

---

### 四、如何快速定位调度卡顿（Debug 路径）

根据分层抽象，若发生感知帧率掉底，可按此路径排查：

- **观察 SyncModule 后台清理日志** → 若频繁超时丢弃，说明传感器时间戳不同步（硬触发问题）。
- **观察 FrameBuffer 的 cache_frames_ 堆积** → 若堆积大于 `max_running_step_`，说明 `TriggerScheduler` 派发速度跟不上，大概率是某个 `TaskRunner` 卡在 `WaitInferDone`（查推理负载）。
- **观察 Frame 的 TaskInfo 卡在 WAITING 状态** → 去查 `depend_tasks` 指针指向的 Task 是否因异常被 `SkipTask` 了（常常是前置预处理失败导致），此时需检查 `BaseModelWork::Preprocess` 的返回值。
- **观察 TaskRunner 的 callback_ 是否被频繁触发** → 若 `callback_` 触发间隔远大于预期（如 50ms），说明 OS 调度器可能未按 `SCHED_RR` 实时抢占，检查绑核是否与系统中断核冲突。

---

### 总结

这套抽象层最成功之处在于**严格的“职责下放”**：时间对齐丢给 Sync，DAG 构建丢给 FrameControler，状态机维护丢给 Frame，互斥锁丢给 RunnerState，**Scheduler 只做“纯逻辑判断”**。这使得新增一个算法 Work 时，开发者几乎不需要理解调度细节，只需配置 yaml 中的 `depend/after/step`。

**后续若需扩展，建议严守“不改动 TriggerScheduler 核心逻辑，只在 Frame 层增加新的依赖类型（如 OR 依赖、超时强制跳过）”**，以维持这套架构的稳定性。

如果你需要针对**某个特定 Work 的接入**，或者**如何调整 `frame_valid` 和 `step` 来优化双帧/三帧流水线**做更深入的分析，我可以继续为你展开。你想深入哪一块
