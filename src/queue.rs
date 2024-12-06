use std::collections::VecDeque;
use std::sync::Mutex;

pub struct UnboundedBlockingMPMCQueue<T>(Mutex<VecDeque<T>>);

impl<T> UnboundedBlockingMPMCQueue<T> {
    pub fn new() -> Self {
        Self(Mutex::new(VecDeque::new()))
    }

    pub fn push(&self, task: T) {
        self.0.lock().expect("queue poisoned").push_back(task);
    }

    pub fn pop(&self) -> Option<T> {
        self.0.lock().expect("queue poisoned").pop_front()
    }
}

pub struct BoundedBlockingMPMCQueue<T> {
    queue: Mutex<VecDeque<T>>,
}

impl<T> BoundedBlockingMPMCQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(capacity)),
        }
    }

    pub fn push(&self, task: T) -> Option<()> {
        let mut queue = self.queue.lock().expect("queue poisoned");
        if queue.len() == queue.capacity() {
            None
        } else {
            queue.push_back(task);
            Some(())
        }
    }

    pub fn pop(&self) -> Option<T> {
        self.queue.lock().expect("queue poisoned").pop_front()
    }
}
