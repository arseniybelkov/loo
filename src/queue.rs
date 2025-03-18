use std::collections::VecDeque;
use std::sync::{Mutex, Condvar};

struct Queue<T> {
    items: VecDeque<T>,
    closed: bool,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
            closed: false,
        }
    }
    
    fn push(&mut self, item: T) -> Result<(), ()> {
        if (self.closed) {
            return Err(())
        }
        Ok(self.items.push_back(item))
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    
    fn close(&mut self) {
        self.closed = true;
    }
} 

pub struct UnboundedBlockingMPMCQueue<T> {
    queue: Mutex<Queue<T>>,
    queue_not_empty: Condvar,
}

impl<T> UnboundedBlockingMPMCQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(Queue::new()),
            queue_not_empty: Condvar::new(),
        }
    }

    pub fn push(&self, value: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(value).unwrap();
    }

    pub fn pop(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        while queue.items.is_empty() {
            if (queue.closed) {
                return None;
            }
            queue = self.queue_not_empty.wait(queue).unwrap();
        }
        queue.pop()
    }
    
    pub fn close(&self) {
        let mut queue = self.queue.lock().unwrap();
        queue.close();
    }
}
