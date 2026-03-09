use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SubmitToolCall {
    pub task_id: Uuid,
    pub step_id: Uuid,
    pub agent_id: Uuid,
    pub tool: String,
    pub args: Value,
    pub policy_snapshot_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubmitDecision {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SubmitAck {
    pub call_id: Uuid,
    pub accepted_at: String,
    pub decision: SubmitDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GetCallStatus {
    pub call_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CallState {
    Queued,
    Running,
    Completed,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CallStatus {
    pub call_id: Uuid,
    pub state: CallState,
    pub result_summary: Option<String>,
    pub reason_code: Option<String>,
}

impl CallStatus {
    pub fn validate(&self) -> Result<(), ControlValidationError> {
        match self.state {
            CallState::Blocked | CallState::Failed if self.reason_code.is_none() => {
                Err(ControlValidationError::MissingReasonCodeForTerminalErrorState)
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageActor {
    User,
    Agent,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UserInput {
    pub task_id: Uuid,
    pub step_id: Uuid,
    pub content: String,
    pub actor: MessageActor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UserOutput {
    pub task_id: Uuid,
    pub step_id: Uuid,
    pub content: String,
    pub actor: MessageActor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlValidationError {
    MissingReasonCodeForTerminalErrorState,
}
