use std::collections::VecDeque;

/// Given an array, rotate the array to the right by k steps, where k is
/// non-negative.
///
/// Try to come up with as many solutions as you can. There are at least three
/// different ways to solve this problem.
///
/// Could you do it in-place with O(1) extra space?
pub struct Solution();

impl Solution {
    pub fn rotate_sensible(nums: &mut Vec<i32>, k: i32) {
        let mut deque = VecDeque::from(nums.clone());
        for _ in 0..k {
            if let Some(x) = deque.pop_back() {
                deque.push_front(x);
            }
        }
        *nums = Vec::from(deque);
    }

    /// O(1) extra space, all in place
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            if let Some(x) = nums.pop() {
                nums.insert(0, x);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        test_impl(Solution::rotate);
    }

    #[test]
    fn test_rotate_sensible() {
        test_impl(Solution::rotate_sensible);
    }

    fn test_impl(rotate: impl Fn(&mut Vec<i32>, i32)) {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, [5, 6, 7, 1, 2, 3, 4]);

        let mut nums = vec![-1, -100, 3, 99];
        rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);

        let mut nums = vec![-1];
        rotate(&mut nums, 2);
        assert_eq!(nums, vec![-1]);
    }
}
