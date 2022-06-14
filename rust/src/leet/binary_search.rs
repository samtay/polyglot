/// Given an array of integers nums which is sorted in ascending order, and an
/// integer target, write a function to search target in nums. If target exists,
/// then return its index. Otherwise, return -1.

/// You must write an algorithm with O(log n) runtime complexity.
use std::cmp;

pub struct Solution();
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        // The interval is the width of the next search
        let mut interval = len / 2;
        // As we progress, only search indices greater than or equal to the offset
        let mut offset = 0;
        let mut ix = interval + offset;
        loop {
            let last_ix = ix;
            match target.cmp(&nums[ix]) {
                cmp::Ordering::Equal => return ix as i32,
                cmp::Ordering::Less => {
                    interval /= 2;
                }
                cmp::Ordering::Greater => {
                    interval /= 2;
                    offset = ix + 1;
                }
            }
            ix = cmp::min(interval + offset, len - 1);
            // If we're stuck, the element is not in the list
            if last_ix == ix {
                return -1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(Solution::search(vec![12], 10), -1);
        assert_eq!(Solution::search(vec![12], 12), 0);
        assert_eq!(Solution::search(vec![12, 13], 12), 0);
        assert_eq!(Solution::search(vec![12, 13], 13), 1);
        assert_eq!(Solution::search(vec![-1, 0, 5], 5), 2);
        assert_eq!(Solution::search(vec![1, 2, 5], 0), -1);
        assert_eq!(Solution::search(vec![1, 2, 5], 1), 0);
        assert_eq!(Solution::search(vec![1, 2, 5], 6), -1);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 11, 12], 13), -1);
        assert_eq!(
            Solution::search(vec![-10, -8, -1, 0, 3, 5, 9, 11, 12], 12),
            8
        );
    }
}
