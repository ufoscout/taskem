use crate::model::task::{TaskData, TaskModel};
use c3p0_common::NewModel;
use crate::error::SchedulerError;
use crate::repository::SchedulerRepository;
use std::sync::Arc;

pub struct TaskService {
    repo: Arc<dyn SchedulerRepository + Send + Sync>
}

impl TaskService {
    pub fn new(repo: Arc<dyn SchedulerRepository + Send + Sync>) -> Self {
        Self {
            repo
        }
    }

    pub fn register<T: Into<NewModel<TaskData>>>( &self, task: T ) -> Result<TaskModel, SchedulerError> {
        self.repo.save(task.into())
    }

}
