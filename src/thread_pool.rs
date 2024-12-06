use crate::queue::{BoundedBlockingMPMCQueue, UnboundedBlockingMPMCQueue};
use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    n_threads: usize,
    workers: BoundedBlockingMPMCQueue<JoinHandle<()>>,
    queue: UnboundedBlockingMPMCQueue<Box<dyn FnOnce() + 'static>>,
    executor_handle: Option<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(n_threads: usize) -> Self {
        let mut this = Self {
            n_threads,
            workers: BoundedBlockingMPMCQueue::new(n_threads),
            queue: UnboundedBlockingMPMCQueue::new(),
            executor_handle: None,
        };
        this.start();
        this
    }

    pub fn submit<T: FnOnce() + 'static>(&self, task: T) {
        let task = Box::new(task) as Box<dyn FnOnce()>;
        self.queue.push(task);
    }

    fn start(&mut self) {
        self.join_executor();
        let handle = thread::spawn(|| {
            
        });
        self.executor_handle = Some(handle);
    }
    
    fn join_executor(&mut self) {
        if let Some(handle) = self.executor_handle.take() {
            handle.join().expect("Executor thread panicked!");
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.join_executor();
    }
}