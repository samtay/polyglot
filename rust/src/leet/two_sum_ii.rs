/// Given a 1-indexed array of integers numbers that is already sorted in
/// non-decreasing order, find two numbers such that they add up to a specific
/// target number. Let these two numbers be numbers[index1] and numbers[index2]
/// where 1 <= index1 < index2 <= numbers.length.
///
/// Return the indices of the two numbers, index1 and index2, added by one as an
/// integer array [index1, index2] of length 2.
///
/// The tests are generated such that there is exactly one solution. You may not
/// use the same element twice.
///
/// Your solution must use only constant extra space.

pub struct Solution();
impl Solution {
    /// Ideas:
    /// 1. We could binary search for target/2, get the closest index ix, then
    ///    search (ix-1, ix+1), (ix-2, ix+2), etc.
    ///    The binary searh is O(log n) and the outward search is O(n), for a
    ///    total of O(n). Whoops this doesn't work at all, but might be a good
    ///    starting place heuristic depending on the input distribution.
    /// 2. We could, for each x in nums, binary search for target-x, but this
    ///    would be O(n log n).
    ///
    /// Let's go with idea 2;
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for ix1 in 0..numbers.len() {
            let t = target - numbers[ix1];
            // ix1 + 1 safe because solution (ix1, ix2) is guaranteed, and ix1 must exist before last element
            if let Ok(ix2) = numbers[ix1 + 1..].binary_search(&t) {
                return vec![(ix1 + 1) as i32, (ix1 + ix2 + 2) as i32];
            }
        }
        panic!("we were guaranteed a solution!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
        assert_eq!(
            Solution::two_sum(vec![2, 7, 11, 15, 18, 22], 22),
            vec![2, 4]
        );
        assert_eq!(
            Solution::two_sum(vec![2, 7, 11, 15, 18, 22], 26),
            vec![3, 4]
        );
        // This kind of example is a worst case for Idea 1
        assert_eq!(
            Solution::two_sum(vec![2, 7, 11, 15, 18, 22, 50], 52),
            vec![1, 7]
        );
        assert_eq!(
            Solution::two_sum(vec![1, 2, 3, 3, 3, 5, 20, 50], 7),
            vec![2, 6]
        );
        assert_eq!(
            Solution::two_sum(vec![1, 2, 3, 3, 3, 5, 20, 50], 25),
            vec![6, 7]
        );
    }
}
