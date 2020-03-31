use std::io::Result;

mod shared_queue;

trait ThreadPool {
    fn new(n: i32) -> Result<Self>
    where
        Self: Sized;
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
pub use shared_queue::SharedQueueThreadPool;

#[cfg(test)]
mod tests {
    use crate::shared_queue::SharedQueueThreadPool;
    use crate::ThreadPool;
    use crossbeam_utils::sync::WaitGroup;
    use std::io::Result;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    fn spawn_counter<P: ThreadPool>(pool: P) -> Result<()> {
        const TASK_NUM: usize = 20;
        const ADD_COUNT: usize = 1000;

        let wg = WaitGroup::new();
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..TASK_NUM {
            let counter = Arc::clone(&counter);
            let wg = wg.clone();
            pool.spawn(move || {
                for _ in 0..ADD_COUNT {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
                drop(wg);
            })
        }

        wg.wait();
        assert_eq!(counter.load(Ordering::SeqCst), TASK_NUM * ADD_COUNT);
        Ok(())
    }

    #[test]
    fn shared_queue_thread_pool_spawn_counter() -> Result<()> {
        let pool = SharedQueueThreadPool::new(4)?;
        spawn_counter(pool)
    }
}
