//! This question is asked by Google. Create a class CallCounter that tracks the number of calls a
//! client has made within the last 3 seconds. Your class should contain one method, ping(int t)
//! that receives the current timestamp (in milliseconds) of a new call being made and returns the
//! number of calls made within the last 3 seconds.
//!
//! Note: you may assume that the time associated with each subsequent call to ping is strictly
//! increasing.

use std::collections::VecDeque;

pub struct CallCounter {
    queue: VecDeque<u32>,
    length: u32,
}

impl CallCounter {
    /// Create a call counter that keeps track of `length` milliseconds.
    pub fn new(length: u32) -> Self {
        Self {
            queue: VecDeque::new(),
            length,
        }
    }

    /// Given timestamp in ms, return the number of calls within last three seconds
    pub fn ping(&mut self, new: u32) -> usize {
        self.queue.push_back(new);
        loop {
            match self.queue.front() {
                Some(&old) if new - old > self.length => {
                    self.queue.pop_front();
                }
                _ => break,
            }
        }
        self.queue.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping() {
        let mut counter = CallCounter::new(3000);
        // 1 call within the last 3 seconds
        assert_eq!(counter.ping(1), 1);
        // 2 calls within the last 3 seconds
        assert_eq!(counter.ping(300), 2);
        // 3 calls within the last 3 seconds
        assert_eq!(counter.ping(3000), 3);
        // 3 calls within the last 3 seconds
        assert_eq!(counter.ping(3002), 3);
        // 1 call within the last 3 seconds
        assert_eq!(counter.ping(7000), 1);
    }
}
