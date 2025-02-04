
use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct WorkerPool {
    num_workers: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Worker {
    closed: bool,
}

impl WorkerPool {
    pub fn new() -> WorkerPool {
        WorkerPool::default()
    }

    pub fn get_worker(&mut self) -> Worker {
        self.num_workers += 1;
        Worker::default()
    }

    pub fn get_num_workers_created(&self) -> u32 {
        self.num_workers
    }
}

impl Worker {
    pub fn do_work(&self) -> Result<(), WorkerClosedError> {
        if self.closed {
            return Err(WorkerClosedError {});
        }

        // Do something expensive
        Ok(())
    }

    pub fn close(&mut self) {
        self.closed = true
    }
}

#[derive(Debug, Clone)]
pub struct WorkerClosedError;

impl fmt::Display for WorkerClosedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "worker is already closed")
    }
}
