use crate::HandlingRuntime;
use utils::thread_pool;
use anyhow::Result;

// =======================================================================================================

pub struct ThreadPoolHandlingRuntime {
    thread_pool: thread_pool::ThreadPool,
}

impl ThreadPoolHandlingRuntime {
    pub fn new() -> Self {
        Self {
            thread_pool: thread_pool::ThreadPool::new(4),
        }
    }
}

impl HandlingRuntime for ThreadPoolHandlingRuntime {
    fn execute(&self, job: thread_pool::Job) -> Result<()> {
        self.thread_pool.push_job(job)
    }
}
