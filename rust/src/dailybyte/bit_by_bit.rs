//! Given an array of integers, nums, return an array that contains nums sorted
//! in ascending order according to their number of one bits.
//!
//! Note: If two values have the same number of one bits, they should be sorted
//! in ascending order.

/// There are no shortcuts here. We will at the very least have to take the hit
/// of a sorting algorithm like O(n log n). What we should try to
/// do, however, is make sure that our operations before sorting stay
/// under O(n log n). The below accomplishes this, since pairing each element
/// with its bit_count runs in O(n log n).
pub fn bit_order(xs: Vec<u32>) -> Vec<u32> {
    let mut xs = xs.iter().map(|&x| (bit_count(x), x)).collect::<Vec<_>>();
    xs.sort_unstable_by(|(n1, x1), (n2, x2)| n1.cmp(n2).then(x1.cmp(x2)));
    xs.iter().map(|(_, x)| *x).collect()
}

/// Return the number of 1s in the number's binary representation
/// Runs in log_2 (n)
fn bit_count(mut n: u32) -> u32 {
    let mut count = 0;
    while n > 0 {
        count += n % 2;
        n /= 2;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_order() {
        assert_eq!(bit_order(vec![5, 2, 8]), vec![2, 8, 5]);
        assert_eq!(bit_order(vec![4, 5, 5, 1, 3, 9]), vec![1, 4, 3, 5, 5, 9]);
    }
}
