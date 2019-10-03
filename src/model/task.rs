use c3p0_common::Model;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

pub type TaskModel = Model<TaskData>;

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskData {
    pub task_name: String,
    pub task_instance: String,
    pub currently_owned_by: Option<Owner>,
    pub status: TaskStatus,
    pub payload: Option<Value>,
    pub schedule: Schedule,
    pub on_status_complete: OnComplete,
    pub on_status_failed: OnFailed,
    pub max_attempts_before_failure: Option<usize>,
    pub last_execution_status: Option<ExecutionStatus>,
    pub last_execution_epoch_ms: Option<i64>,
    pub next_execution_epoch_ms: Option<i64>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Owner {
    pub id: String,
    pub last_heartbeat_epoch_ms: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum TaskStatus {
    WAITING, RUNNING, COMPLETED, FAILED
}

#[derive(Deserialize, Serialize, Clone)]
pub enum ExecutionStatus {
    SUCCESS, FAILED
}

#[derive(Deserialize, Serialize, Clone)]
pub enum Schedule {
    TimedExecutions{executions: usize, interval_between_runs: Duration}, Cron(String), Interval(Duration)
}

#[derive(Deserialize, Serialize, Clone)]
pub enum OnComplete {
    KEEP, DELETE
}

#[derive(Deserialize, Serialize, Clone)]
pub enum OnFailed {
    KEEP, DELETE
}