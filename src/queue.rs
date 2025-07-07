use std::collections::VecDeque;

pub struct Queue<T> {
    queue: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            queue: VecDeque::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.queue.len()
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push_back(item)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.front()
    }
}
