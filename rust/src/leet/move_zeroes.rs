/// Given an integer array nums, move all 0's to the end of it while maintaining
/// the relative order of the non-zero elements.
///
/// Note that you must do this in-place without making a copy of the array.

pub struct Solution();
impl Solution {
    /// This is O(n^2) worst case, since `remove` is O(n).
    pub fn move_zeroes_bad(nums: &mut Vec<i32>) {
        let mut ix = 0;
        for _ in 0..nums.len() {
            if nums[ix] == 0 {
                nums.remove(ix);
                nums.push(0);
            } else {
                ix += 1;
            }
        }
    }

    /// This is O(n^2) too, technically, in the worst case with lots of zeroes,
    /// but it's better in the average case.
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        for i in 0..nums.len() - 1 {
            if nums[i] == 0 {
                let mut j = i + 1;
                while nums[j] == 0 {
                    j += 1;
                    if j == nums.len() {
                        return;
                    }
                }
                nums.swap(i, j);
            }
        }
    }

    pub fn move_zeroes_best(nums: &mut Vec<i32>) {
        let initial_len = nums.len();
        nums.retain(|x| *x != 0);
        let new_len = nums.len();
        for _ in 0..initial_len - new_len {
            nums.push(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);

        let mut nums = vec![1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1]);

        let mut nums = vec![1, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0]);

        let mut nums = vec![1, 0, 0, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 0, 0]);

        let mut nums = vec![1, 101, 3, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 101, 3, 3]);
    }
}
