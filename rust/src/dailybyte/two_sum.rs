//! This question is asked by Google. Given an array of integers, return whether or not two numbers
//! sum to a given target, k.
//!
//! Note: you may not sum a number with itself.

use std::cmp::Ordering;

/// Note: given an arbitrary unordered list, we'd have to check each unique pair, of which there
/// are (n choose 2), and thus O(n^2).
///
/// Since we can sort in O(n log n) we might as well do that first, in the hope of a better time
/// complexity checking two sum over an ordered list.
/// Once we're sorted, we can iterate across each index, and for each, binary search for the other
/// half of the pair. This is O(n log n), so as long as we've sorted, we can't do any better than
/// this.
fn two_sum(mut ints: Vec<i32>, sum: i32) -> bool {
    ints.sort_unstable();
    for i in 0..ints.len() {
        let target = sum - ints[i];
        let (_, the_rest) = ints.split_at(i + 1);
        if binary_search(the_rest, &target) {
            return true;
        }
    }
    false
}

fn binary_search<T: Ord>(xs: &[T], x: &T) -> bool {
    // Handle empty case
    let len = xs.len();
    if len == 0 {
        return false;
    }
    // Initial binary search parameters
    let mut min = 0;
    let mut max = xs.len() - 1;
    loop {
        let mid = (max + min) / 2;
        match xs[mid].cmp(x) {
            Ordering::Equal => return true,
            Ordering::Greater => {
                if mid == 0 {
                    return false;
                }
                max = mid - 1;
            }
            Ordering::Less => {
                if mid == len {
                    return false;
                }
                min = mid + 1;
            }
        }
        if min > max {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert!(two_sum(vec![1, 3, 8, 2], 10));
        assert!(!two_sum(vec![3, 9, 13, 7], 8));
        assert!(two_sum(vec![4, 2, 6, 5, 2], 4));
    }

    #[test]
    fn test_binary_search() {
        assert!(binary_search(&[1, 2, 3, 5, 8, 13, 21, 34, 55, 89], &1));
        assert!(binary_search(&[1, 2, 3, 5, 8, 13, 21, 34, 55, 89], &2));
        assert!(binary_search(&[1, 2, 3, 5, 8, 13, 21, 34, 55, 89], &89));
        assert!(!binary_search(&[1, 2, 3, 5, 8, 13, 21, 34, 55, 89], &90));
        assert!(binary_search(&[1, 2, 3, 5, 8, 13, 21, 34, 55, 89], &13));
        assert!(!binary_search(&[1, 2, 3, 5, 8, 13, 21, 34, 55, 89], &14));
        assert!(!binary_search(
            &[1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 90],
            &14
        ));
    }
}
