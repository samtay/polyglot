/// Given an integer array nums sorted in non-decreasing order, return an array
/// of the squares of each number sorted in non-decreasing order.
///
/// Follow up: Squaring each element and sorting the new array is very trivial,
/// could you find an O(n) solution using a different approach?
pub struct Solution();

impl Solution {
    /// Let's go for the O(n) solution. We know that before we get to 0, the
    /// squares  will be in reverse order, and afterwards, the nonnegative
    /// numbers will already be properly sorted.
    ///
    /// We should be able to pop off the negative numbers into a stack, and then
    /// insert them while continuing through the loop into the nonnegative part
    /// of the list.
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut negatives = Vec::with_capacity(nums.len());
        let mut sorted = Vec::with_capacity(nums.len());
        for n in nums {
            if n < 0 {
                negatives.push(n.pow(2));
            }
            if n >= 0 {
                // Need to loop, there might be multiple negatives to stick in here
                loop {
                    match negatives.last() {
                        Some(&x) if x <= n.pow(2) => {
                            sorted.push(x);
                            negatives.pop();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                sorted.push(n.pow(2));
            }
        }
        while let Some(x) = negatives.pop() {
            sorted.push(x);
        }
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
        assert_eq!(Solution::sorted_squares(vec![-1]), vec![1]);
        assert_eq!(Solution::sorted_squares(vec![]), vec![]);

        assert_eq!(
            Solution::sorted_squares(vec![-10000, -9999, -7, -5, 0, 0, 10000]),
            vec![0, 0, 25, 49, 99980001, 100000000, 100000000]
        );
    }
}
