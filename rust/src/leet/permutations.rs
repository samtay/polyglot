//! Given an array nums of distinct integers, return all the possible
//! permutations. You can return the answer in any order.

pub struct Solution;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Base cases
        let n = nums.len();
        if n < 2 {
            return vec![nums];
        }
        // Initialize perms
        let mut perms = vec![vec![nums[0]]];

        // For each num, stick it in all available indexes in each permutation.
        for num in nums.iter().skip(1) {
            perms = perms
                .into_iter()
                .flat_map(|perm| {
                    let m = perm.len();
                    (0..m + 1).map(move |ix| {
                        let mut perm = perm.clone();
                        perm.insert(ix, *num);
                        perm
                    })
                })
                .collect();
        }
        perms
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
        assert_eq!(Solution::permute(vec![1, 2]), vec![vec![2, 1], vec![1, 2]]);
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![3, 2, 1],
                vec![2, 3, 1],
                vec![2, 1, 3],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![1, 2, 3]
            ]
        );
    }
}
