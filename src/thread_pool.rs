use std::thread::{self, JoinHandle};
use crate::queue::{BoundedBlockingMPMCQueue, UnboundedBlockingMPMCQueue};

pub struct ThreadPool {
    n_threads: usize,
    workers: BoundedBlockingMPMCQueue<JoinHandle<()>>,
    queue: UnboundedBlockingMPMCQueue<fn()>,
}

impl ThreadPool {
    pub fn new(n_threads: usize) -> Self {
        let this = Self {
            n_threads,
            workers: BoundedBlockingMPMCQueue::new(n_threads),
            queue: UnboundedBlockingMPMCQueue::new(),
        };
        this.start();
        this
    }

    pub fn submit(&self, task: fn()) {
        self.queue.push(task);
    }

    fn start(&self) {
        thread::spawn(|| {
           loop {
               if let Some(task) = self.queue.pop() {
                   
               } else {
                   thread::yield_now();
               }
           }
        });
    }
}
