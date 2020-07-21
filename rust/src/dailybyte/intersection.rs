//! This question is asked by Google. Given two integer arrays, return their intersection.

use std::collections::HashSet;
use std::hash::Hash;

/// I'm going to retain the order of the first list. This is an arbitrary choice; it would be just
/// as easy to do so with the second list. Approach:
///   1) build a hashset of the elements in the second list -- time O(t) space O(t)
///   2) iterate first list, filtering anything found in the hashset -- time O(s)
///   3) re-collect into a Vec -- space O(d) where d is the length of intersection of O(s, t), e.g. O(min(s,t)).
///
/// Hence, total time is O(s + t) and space is O(min(s,t))
pub fn intersection<T: Eq + Hash>(s: Vec<T>, t: Vec<T>) -> Vec<T> {
    let mut set = HashSet::with_capacity(t.len());
    set.extend(t);

    s.into_iter()
        .filter(move |n| {
            if set.contains(&n) {
                set.remove(&n);
                return true;
            }
            false
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(intersection(vec![2, 4, 4, 2], vec![2, 4]), vec![2, 4]);
        assert_eq!(intersection(vec![1, 2, 3, 3], vec![3, 3]), vec![3]);
        assert_eq!(intersection(vec![2, 4, 6, 8], vec![1, 3, 5, 7]), vec![]);
        // Order maintained
        assert_eq!(
            intersection(vec![1, 2, 3, 4], vec![3, 2, 1, 0, 4, 5]),
            vec![1, 2, 3, 4]
        );
    }
}
