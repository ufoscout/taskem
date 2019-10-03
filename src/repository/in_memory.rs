use crate::repository::{SchedulerRepository};
use crate::error::SchedulerError;
use crate::model::task::{TaskModel, TaskData};
use c3p0_common::NewModel;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct InMemorySchedulerRepository{
    tasks: Arc<Mutex<Vec<TaskModel>>>
}

impl InMemorySchedulerRepository {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(vec![]))
        }
    }
}

impl SchedulerRepository for InMemorySchedulerRepository {

    fn start(&self) -> Result<(), SchedulerError> {
        Ok(())
    }

    fn fetch_all(&self) -> Result<Vec<TaskModel>, SchedulerError> {
        let lock = self
            .tasks
            .lock()
            .map_err(|err| SchedulerError::InternalError {
                message: format!(
                    "InMemorySchedulerRepository.fetch_all - Cannot obtain lock . Err: [{}]",
                    err
                ),
            })?;
        Ok(lock.clone())
    }

    fn save(&self, task: NewModel<TaskData>) -> Result<TaskModel, SchedulerError> {
        let mut lock = self
            .tasks
            .lock()
            .map_err(|err| SchedulerError::InternalError {
                message: format!(
                    "InMemorySchedulerRepository.save - Cannot obtain lock . Err: [{}]",
                    err
                ),
            })?;

        let model = TaskModel{
            id: lock.len() as i64,
            version: 0,
            data: task.data
        };

        lock.push(model.clone());

        Ok(model)
    }

    fn delete(&self, task: TaskModel) -> Result<u64, SchedulerError> {
        let mut lock = self
            .tasks
            .lock()
            .map_err(|err| SchedulerError::InternalError {
                message: format!(
                    "InMemorySchedulerRepository.delete - Cannot obtain lock . Err: [{}]",
                    err
                ),
            })?;

        let len: u64 = lock.len() as u64;

        lock.retain(|x|  x.id != task.id || x.version != task.version);

        Ok(len - lock.len() as u64)
    }

    fn delete_by_task_name(&self, task_name: &str) -> Result<u64, SchedulerError> {
        let mut lock = self
            .tasks
            .lock()
            .map_err(|err| SchedulerError::InternalError {
                message: format!(
                    "InMemorySchedulerRepository.delete_by_task_name - Cannot obtain lock . Err: [{}]",
                    err
                ),
            })?;

        let len: u64 = lock.len() as u64;

        lock.retain(|x|  x.data.task_name != task_name );

        Ok(len - lock.len() as u64)
    }
}