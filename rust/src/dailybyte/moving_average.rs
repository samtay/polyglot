//! This question is asked by Microsoft. Design a class, MovingAverage, which contains a method,
//! next that is responsible for returning the moving average from a stream of integers.
//!
//! Note: a moving average is the average of a subset of data at a given point in time.

use std::collections::VecDeque;

pub struct MovingAverage {
    capacity: usize,
    queue: VecDeque<i32>,
}

impl MovingAverage {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            queue: VecDeque::with_capacity(capacity),
        }
    }

    pub fn next(&mut self, n: i32) -> i32 {
        if self.queue.len() == self.capacity {
            self.queue.pop_front();
        }
        self.queue.push_back(n);
        self.queue.iter().sum::<i32>() / self.queue.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_avg() {
        let mut m = MovingAverage::new(3);
        assert_eq!(m.next(3), 3);
        assert_eq!(m.next(5), 4);
        assert_eq!(m.next(7), 5);
        assert_eq!(m.next(6), 6);
    }
}
