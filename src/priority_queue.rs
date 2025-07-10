use std::cmp::Reverse;
use std::collections::BinaryHeap;

// min-heap priority queue, useful for path finding algorithms
pub struct PriorityQueue<T> {
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.heap.len()
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(Reverse(item))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(item)| item)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|Reverse(item)| item)
    }
}
