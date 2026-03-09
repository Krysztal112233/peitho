# Peitho v1 架构设计（Pigeon + ZMQ）

## 1. 概述

本文定义 Peitho 的 v1 通讯架构：

- `peitho` 是唯一编排器（orchestrator）和唯一 IPC socket 持有者。
- `peitho-agent` 运行在容器内，负责与 LLM 交互并提交工具调用请求。
- `peitho-pigeon` 是基于 ZMQ 的通讯层，使用挂载的 IPC socket 进行通信。
- v1 工具调用语义采用异步句柄（`call_id`）模式。

该设计优先保证正确性、可审计性和边界清晰，而非早期性能优化。

## 2. 范围与约束（v1）

- 仅支持本地优先（local-first）运行。
- 不包含 dashboard / web UI / 控制平面。
- 当前策略阶段为 allow-by-default，但每个动作都必须有审计记录。
- v1 不引入按调用动态创建 channel/token 的机制。
- 使用固定、可挂载的 IPC socket 路径。

## 3. 角色与边界

### 3.1 `peitho`（编排器）

- 持有并绑定 IPC sockets。
- 执行策略评估并触发沙盒执行。
- 负责 history 与 memory 的合并。
- 统一路由用户输入/输出。
- 为完整调用生命周期产出审计事件。

### 3.2 `peitho-agent`

- 与 LLM 交互。
- 向 `peitho` 提交工具调用请求。
- 轮询调用状态，必要时可订阅事件。
- 不直接执行工具，不绕过策略/沙盒。

### 3.3 Watcher / 面向用户的适配层

- 仅连接 `peitho`。
- 从 `peitho` 接收输出与状态更新。
- 将用户输入转发给 `peitho`。
- 不直接连接 `peitho-agent`。

### 3.4 `peitho-pigeon`

- 提供基于 ZMQ 的传输与协议抽象。
- 负责稳定 JSON 消息的编解码。
- 为 `peitho` 与 `peitho-agent` 提供 server/client 接口能力。

## 4. 传输拓扑

挂载 IPC 根路径：

- `/run/peitho`

端点定义：

- 控制总线：`ipc:///run/peitho/pigeon-control.sock`
- 事件总线：`ipc:///run/peitho/pigeon-events.sock`

Socket 模式：

- 控制总线：`peitho` 使用 `ROUTER`，客户端（`peitho-agent`、watcher）使用 `DEALER`。
- 事件总线：`peitho` 使用 `PUB`，消费者（watcher，或可选 agent）使用 `SUB`。

## 5. 协议契约（JSON）

以下为逻辑消息类型。线上的实际载荷格式为字段名稳定的 JSON。

### 5.1 控制消息

- `SubmitToolCall`
  - 字段：`task_id`、`step_id`、`agent_id`、`tool`、`args`、`policy_snapshot_id`
- `SubmitAck`
  - 字段：`call_id`、`accepted_at`、`decision`（`accepted` 或 `rejected`）
- `GetCallStatus`
  - 字段：`call_id`
- `CallStatus`
  - 字段：`call_id`、`state`、`result_summary`、`reason_code`
  - `state`：`queued`、`running`、`completed`、`failed`、`blocked`
- `UserInput`
  - 字段：`task_id`、`step_id`、`content`、`actor`
- `UserOutput`
  - 字段：`task_id`、`step_id`、`content`、`actor`

### 5.2 校验规则

- 必填字段缺失时必须 fail-closed。
- 枚举值/类型非法时必须 fail-closed。
- `blocked` 和 `failed` 状态必须包含机器可读的 `reason_code`。
- 只要存在动作上下文，就必须携带 `task_id` 和 `step_id` 用于审计关联。

## 6. 执行流程

### 6.1 异步工具调用流程

1. `peitho-agent` 向 `peitho` 发送 `SubmitToolCall`。
2. `peitho` 校验请求 envelope，并返回包含 `call_id` 的 `SubmitAck`。
3. `peitho` 执行策略评估与沙盒执行。
4. `peitho` 更新调用状态（`queued -> running -> completed/failed/blocked`）。
5. `peitho-agent` 通过 `GetCallStatus` 拉取状态；watcher 从事件总线接收状态更新。

### 6.2 用户输入/输出路由流程

1. watcher 向 `peitho` 发送 `UserInput`。
2. `peitho` 将相关内容路由到 agent 侧流程，并记录 history/memory。
3. `peitho` 向 watcher 输出 `UserOutput` 与状态更新。

## 7. 审计映射

每次工具调用生命周期必须映射到以下审计事件类型：

- `planned`
- `policy_evaluated`
- `executed`
- `blocked`
- `failed`
- `completed`

每条事件应保持链式连续性，并包含关联上下文：

- `event_id`、`prev_hash`
- `task_id`、`step_id`
- `actor`、`action`、`decision`、`timestamp`

## 8. 故障模型

- socket 不可用：返回确定性传输错误，不允许静默丢弃请求。
- 策略拒绝或输入非法：返回 `blocked`/`failed` 并附带 `reason_code`。
- 沙盒超时/资源违规：映射到稳定失败原因并产出审计事件。
- 未知消息类型：拒绝并 fail-closed。

## 9. 验证清单

- 所有消息类型的 JSON 编解码稳定性。
- 缺失/非法字段场景符合 fail-closed。
- 多 agent 并发提交时，可通过 `call_id` 正确关联。
- 状态迁移合法，终态稳定。
- 工具调用全生命周期审计链完整。
- 事件总线消费者仅接收来自 `peitho` 的状态更新。

## 10. 实现映射

首批落地位置：

- `crates/peitho-pigeon`：协议类型与 ZMQ 传输抽象。
- `crates/peitho`：控制/事件服务端，编排、策略与沙盒集成。
- `crates/peitho-agent`：面向 LLM 驱动工具调用的提交与状态查询客户端流程。

v1 明确不包含：

- 动态 per-call channel 或动态派发 token。
- 多租户路由与远程/云端编排。
- UI/dashboard 能力。
