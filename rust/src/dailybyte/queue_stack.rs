//! Design a class to implement a stack using only a single queue. Your class, QueueStack, should
//! support the following stack methods: push() (adding an item), pop() (removing an item), peek()
//! (returning the top value without removing it), and empty() (whether or not the stack is empty).

use std::collections::VecDeque;

/// Note that VecDeque is actually a double ended queue, but in the interest of problem solving
/// we'll use it as a single ended queue.
/// In particular, we need to suffer at least one O(n) operation; we'll do this on `push`.
#[derive(Default)]
pub struct QueueStack<T> {
    queue: VecDeque<T>,
}

impl<T> QueueStack<T> {
    /// Create a stack
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Push item onto the stack
    pub fn push(&mut self, item: T) {
        self.queue.push_back(item);
        for _ in 0..self.queue.len() - 1 {
            let tmp = self.queue.pop_front().unwrap();
            self.queue.push_back(tmp);
        }
    }

    /// Pop item from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    /// Peek at the top value without removing it
    pub fn peek(&self) -> Option<&T> {
        self.queue.front()
    }

    /// Check if stack is empty
    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = QueueStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.peek(), Some(&1));
        assert!(!stack.empty());
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.empty());
        assert_eq!(stack.pop(), None);
    }
}
