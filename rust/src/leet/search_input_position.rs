/// Given a sorted array of distinct integers and a target value, return the
/// index if the target is found. If not, return the index where it would be if
/// it were inserted in order.

/// You must write an algorithm with O(log n) runtime complexity.
use std::cmp;

pub struct Solution();

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
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
            // If we're stuck, the element is not in the list, and the index is where it should go
            if last_ix == ix {
                return if nums[ix] < target { ix + 1 } else { ix } as i32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);

        assert_eq!(Solution::search_insert(vec![1, 3, 5], 4), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5], 6), 3);
        assert_eq!(Solution::search_insert(vec![1, 3, 5], 1), 0);
        assert_eq!(Solution::search_insert(vec![1, 3, 5], -1), 0);

        assert_eq!(Solution::search_insert(vec![1, 3], 3), 1);
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3], 4), 2);
    }
}
