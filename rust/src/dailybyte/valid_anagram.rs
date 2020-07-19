//! This question is asked by Facebook. Given two strings s and t return whether or not s is an
//! anagram of t.

use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// The naive approach to iterate over one string s and for each char, find and remove a char from
/// t, has poor time complexity. Assuming a search and remove of O(log t) (depends on
/// implementation of string), this would be O(s log t).
///
/// Instead let's use some extra space, build a hashmap the character counts of s, and see if t
/// matches up.
/// This solution has time complexity O(s + t) and space complexity O(s') where s' is
/// the number of distinct characters in s.
pub fn valid_anagram(s: &str, t: &str) -> bool {
    let mut chars: HashMap<char, u16> = s.chars().fold(HashMap::new(), |mut cs, c| {
        cs.entry(c).and_modify(|n| *n += 1).or_insert(1);
        cs
    });
    for c in t.chars() {
        match chars.entry(c) {
            Entry::Vacant(_) => return false,
            Entry::Occupied(e) if *e.get() == 1 => {
                e.remove();
            }
            entry => {
                entry.and_modify(|n| *n -= 1);
            }
        }
    }
    chars.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_anagram() {
        assert!(valid_anagram("cat", "tac"));
        assert!(valid_anagram("listen", "silent"));
        assert!(!valid_anagram("program", "function"));
    }
}
