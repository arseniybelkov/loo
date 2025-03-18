use crate::queue::UnboundedBlockingMPMCQueue;
use std::thread::{self, JoinHandle};

type Worker = JoinHandle<()>;

pub struct ThreadPool<Task: FnOnce() + Send + 'static> {
    n_threads: usize,
    workers: Vec<Worker>,
    queue: UnboundedBlockingMPMCQueue<Task>,
}

impl<Task: FnOnce() + Send + 'static> ThreadPool<Task> {
    pub fn new(n_threads: usize) -> Self {
        let mut this = Self {
            n_threads,
            workers: Vec::with_capacity(n_threads),
            queue: UnboundedBlockingMPMCQueue::new(),
        };
        this.start();
        this
    }
    
    fn start(&mut self) {
        todo!("mpmc");
        for _ in 0..self.n_threads {
            self.workers.push(thread::spawn(|| {
               self.work(); 
            }));
        }
    }
    
    fn stop(&mut self) {
        self.queue.close();
        while let Some(worker) = self.workers.pop() {
            worker.join().unwrap();
        }
    }
    
    fn work(&self) {
        while let Some(task) = self.queue.pop() {
            task();
        }
    }

    pub fn submit(&self, task: Task) {
        self.queue.push(task);
    }
    
}

impl<Task> Drop for ThreadPool<Task>
where
    Task: FnOnce() + Send + 'static
{
    fn drop(&mut self) {
        self.stop();
    }
}