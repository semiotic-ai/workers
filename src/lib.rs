use std::fmt;

/**
 * WorkerPool and Worker are provided from an external library and cannot be modified
 *
 * WorkerPool is responsible for creating new workers, and keeps track of how many workers are created
 * Workers perform some action and are closed when they are done
 */
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

/// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// 
/// When someone is finished using a worker, we want to allow it to be reused.
///
/// Your task is to add any additional code you'd like, including modifying create_worker_pool,
/// without modifying the above libraries to get the tests to pass.
///
/// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Create a new worker pool
pub fn create_worker_pool() -> WorkerPool {
    WorkerPool::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_do_work_after_closing() {
        let mut worker_pool = create_worker_pool();
        let mut worker = worker_pool.get_worker();
        worker.close();

        assert!(worker.do_work().is_err())
    }

    #[test]
    fn workers_are_not_reused_if_still_open() {
        let mut worker_pool = create_worker_pool();
        let mut worker_a = worker_pool.get_worker();
        let mut worker_b = worker_pool.get_worker();
        worker_a.do_work().expect("Couldn't do work for worker a");
        worker_b.do_work().expect("Couldn't do work for worker b");
        worker_a.close();
        worker_b.close();

        assert_eq!(worker_pool.get_num_workers_created(), 2)
    }

    #[test]
    fn workers_are_reused_if_closed() {
        let mut worker_pool = create_worker_pool();
        let mut worker_a = worker_pool.get_worker();
        worker_a.do_work().expect("Couldn't do work for worker a");
        worker_a.close();

        let mut worker_b = worker_pool.get_worker();
        worker_b.do_work().expect("Couldn't do work for worker b");
        worker_b.close();

        assert_eq!(worker_pool.get_num_workers_created(), 1)
    }
}
