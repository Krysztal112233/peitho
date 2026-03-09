# Peitho v1 Architecture (Pigeon + ZMQ)

## 1. Summary

This document defines the v1 communication architecture for Peitho:

- `peitho` is the only orchestrator and the only IPC socket owner.
- `peitho-agent` runs in container, talks to LLM, and submits tool-call requests.
- `peitho-pigeon` is the communication layer using ZMQ over mounted IPC sockets.
- v1 tool call semantics are asynchronous handle based (`call_id`).

This design prioritizes correctness, auditability, and clear boundaries over early optimization.

## 2. Scope and Constraints (v1)

- Local-first runtime only.
- No dashboard/web UI/control plane.
- Allow-by-default policy phase, but every action must be audit-logged.
- No dynamic per-call channels/tokens in v1.
- Use fixed mounted IPC socket paths.

## 3. Roles and Boundaries

### 3.1 `peitho` (Orchestrator)

- Owns and binds IPC sockets.
- Evaluates policy and invokes sandbox execution.
- Merges history and memory state.
- Routes user input/output.
- Emits audit events for the full call lifecycle.

### 3.2 `peitho-agent`

- Interacts with LLM.
- Submits tool-call requests to `peitho`.
- Polls call status and optionally subscribes events.
- Does not execute tools directly and does not bypass policy/sandbox.

### 3.3 Watcher / User-facing Adapter

- Connects to `peitho` only.
- Receives output/status updates from `peitho`.
- Forwards user input to `peitho`.
- Does not directly connect to `peitho-agent`.

### 3.4 `peitho-pigeon`

- Provides transport and protocol abstractions for ZMQ communication.
- Encodes/decodes stable JSON messages.
- Exposes server/client APIs used by `peitho` and `peitho-agent`.

## 4. Transport Topology

Mounted IPC path root:

- `/run/peitho`

Endpoints:

- Control bus: `ipc:///run/peitho/pigeon-control.sock`
- Event bus: `ipc:///run/peitho/pigeon-events.sock`

Socket patterns:

- Control bus: `ROUTER` at `peitho`, `DEALER` at clients (`peitho-agent`, watcher).
- Event bus: `PUB` at `peitho`, `SUB` at consumers (watcher, optional agent).

## 5. Protocol Contract (JSON)

All protocol names below are logical message types. Wire format is JSON with stable field names.

### 5.1 Control Messages

- `SubmitToolCall`
  - Fields: `task_id`, `step_id`, `agent_id`, `tool`, `args`, `policy_snapshot_id`
- `SubmitAck`
  - Fields: `call_id`, `accepted_at`, `decision` (`accepted` or `rejected`)
- `GetCallStatus`
  - Fields: `call_id`
- `CallStatus`
  - Fields: `call_id`, `state`, `result_summary`, `reason_code`
  - `state`: `queued`, `running`, `completed`, `failed`, `blocked`
- `UserInput`
  - Fields: `task_id`, `step_id`, `content`, `actor`
- `UserOutput`
  - Fields: `task_id`, `step_id`, `content`, `actor`

### 5.2 Validation Rules

- Missing required fields must fail closed.
- Invalid enum/type values must fail closed.
- `blocked` and `failed` states must include machine-readable `reason_code`.
- Where action context exists, include `task_id` and `step_id` for audit correlation.

## 6. Execution Flows

### 6.1 Asynchronous Tool Call

1. `peitho-agent` sends `SubmitToolCall` to `peitho`.
2. `peitho` validates request envelope and returns `SubmitAck` with `call_id`.
3. `peitho` performs policy evaluation and sandbox execution.
4. `peitho` updates call state (`queued -> running -> completed/failed/blocked`).
5. `peitho-agent` fetches state via `GetCallStatus`; watcher receives state events from event bus.

### 6.2 User Input/Output Routing

1. Watcher sends `UserInput` to `peitho`.
2. `peitho` routes relevant content to agent-side workflow and records history/memory.
3. `peitho` emits `UserOutput` and state updates to watcher.

## 7. Audit Mapping

Each tool-call lifecycle must map to audit event types:

- `planned`
- `policy_evaluated`
- `executed`
- `blocked`
- `failed`
- `completed`

Every event should preserve chain continuity and include correlation context:

- `event_id`, `prev_hash`
- `task_id`, `step_id`
- `actor`, `action`, `decision`, `timestamp`

## 8. Failure Model

- Socket unavailable: return deterministic transport error and do not silently drop requests.
- Policy reject or invalid input: return `blocked`/`failed` with `reason_code`.
- Sandbox timeout/resource violation: map to stable failure reason and emit audit event.
- Unknown message type: reject and fail closed.

## 9. Validation Checklist

- Protocol JSON encode/decode stability for all message types.
- Missing/invalid field behavior is fail-closed.
- Multi-agent concurrent submissions are correctly correlated by `call_id`.
- Status transitions are valid and terminal states are stable.
- Audit chain completeness across full tool-call lifecycle.
- Event bus consumers receive state updates from `peitho` only.

## 10. Implementation Mapping

Initial landing points:

- `crates/peitho-pigeon`: protocol types and ZMQ transport abstractions.
- `crates/peitho`: control/event servers, orchestration, policy/sandbox integration.
- `crates/peitho-agent`: client-side submit/status flow for LLM-driven tool requests.

Out of scope for v1:

- Dynamic per-call channels or minted channel tokens.
- Multi-tenant routing and remote/cloud orchestration.
- UI/dashboard features.
