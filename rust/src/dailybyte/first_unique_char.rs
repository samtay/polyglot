//! This question is asked by Microsoft. Given a string, return the index of its first unique
//! character. If a unique character does not exist, return -1.

use std::collections::HashMap;

/// Again, let's hashmap. Fold over the input once in O(n) time, building a hashmap with space
/// O(n') where n' is the number of distinct characters in the input. Then just iterate over the
/// the list again, finding the first count of 1, still O(n).
///
/// Note: if space was an issue, we'd first sort in O(n log n) then iterate until the first single
/// grouped char. This would have time O(n log n) and space O(1).
pub fn first_unique_char(s: &str) -> i16 {
    let chars: HashMap<char, i16> = s.chars().fold(HashMap::new(), |mut cs, c| {
        cs.entry(c).and_modify(|n| *n += 1).or_insert(1);
        cs
    });
    for (ix, c) in s.char_indices() {
        if let Some(1) = chars.get(&c) {
            return ix as i16;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_unique() {
        assert_eq!(first_unique_char("abcabd"), 2);
        assert_eq!(first_unique_char("thedailybyte"), 1);
        assert_eq!(first_unique_char("developer"), 0);
        assert_eq!(first_unique_char("no non unique iqe"), -1);
    }
}
