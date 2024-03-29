use crate::repository::{SchedulerRepository};
use std::sync::Arc;
use crate::service::task::TaskService;

pub mod error;
mod model;
pub mod repository;
mod service;
pub mod utils;


pub struct Schedulem {
    service: TaskService
}

impl Schedulem {
    pub fn new(repo: Arc<dyn SchedulerRepository + Send + Sync>) -> Self {
        Self {
            service: TaskService::new(repo)
        }
    }
}