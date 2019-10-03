use crate::error::SchedulerError;
use crate::model::task::{TaskModel, TaskData};
use c3p0_common::NewModel;

pub mod in_memory;

pub trait SchedulerRepository {

    fn start(&self) -> Result<(), SchedulerError>;

    fn fetch_all(&self) -> Result<Vec<TaskModel>, SchedulerError>;

    fn save(&self, task: NewModel<TaskData>) -> Result<TaskModel, SchedulerError>;

    fn delete(&self, task: TaskModel) -> Result<u64, SchedulerError>;

    fn delete_by_task_name(&self, task_name: &str) -> Result<u64, SchedulerError>;

}