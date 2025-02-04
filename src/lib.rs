/**
 * WorkerPool and Worker are provided from an external library and cannot be modified
 *
 * WorkerPool is responsible for creating new workers, and keeps track of how many workers are created
 * Workers perform some action and are closed when they are done
 */
mod read_only_lib;
use read_only_lib::*;

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
