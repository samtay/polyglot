//! Given two strings s1 and s2, return true if s2 contains a permutation of s1,
//! or false otherwise.
//!
//! In other words, return true if one of s1's permutations is the substring of
//! s2.

use std::collections::{hash_map::Entry, HashMap};

pub struct Solution;
impl Solution {
    /// Let's do a sliding window on s2 with a hashset of s1 chars
    ///
    /// Although, this window does just hop over unknown chars and re-start the calculation;
    /// Maybe it would be faster to split s2 into pieces based on unknown chars?
    ///
    /// This does some pretty liberal cloning of the hashmap for s1. Instead of
    /// waiting for the hashmap of s2 to be empty, we could just modify it until
    /// it is equal to s1; this would incur hashmap equality checks at each loop
    /// iteration, but better than the space complexity of this one.
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        // Some base edge cases
        if s1.is_empty() {
            return true;
        }
        if s2.is_empty() {
            return false;
        }
        // Make a hash map of all the char counts the window needs to have
        let s1_hm: HashMap<char, usize> = s1.chars().fold(HashMap::new(), |mut hm, c| {
            hm.entry(c).and_modify(|ct| *ct += 1).or_insert(1);
            hm
        });

        let mut hm = s1_hm.clone();
        let mut i = 0;
        let mut j = 0;

        let cs = s2.as_bytes();
        let n = s2.len();

        while j < n {
            let c = cs[j] as char;
            match hm.entry(c) {
                // Unsuccessful window
                Entry::Vacant(_) => {
                    if s1_hm.contains_key(&c) {
                        // If character is potentially useful, shift the window
                        while !hm.contains_key(&c) {
                            hm.entry(cs[i] as char)
                                .and_modify(|ct| *ct += 1)
                                .or_insert(1);
                            i += 1;
                        }
                        hm.remove(&c);
                    } else {
                        // When character cannot be used, just move the window and completely start over
                        i = j + 1;
                        hm = s1_hm.clone();
                    }
                }
                // Continue this window
                Entry::Occupied(mut e) => {
                    let ct = e.get_mut(); // or into_mut
                    *ct -= 1;
                    if *ct == 0 {
                        e.remove();
                    }
                    // Window successfully cleared the char counts!
                    if hm.is_empty() {
                        return true;
                    }
                }
            }
            j += 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substr() {
        assert!(Solution::check_inclusion(
            "ab".to_string(),
            "eidbaooo".to_string()
        ));
        assert!(!Solution::check_inclusion(
            "ab".to_string(),
            "eidboaoo".to_string()
        ));
        assert!(Solution::check_inclusion("a".to_string(), "a".to_string()));
        assert!(!Solution::check_inclusion("b".to_string(), "a".to_string()));
        assert!(!Solution::check_inclusion(
            "ba".to_string(),
            "a".to_string()
        ));
        assert!(Solution::check_inclusion("".to_string(), "".to_string()));
        assert!(!Solution::check_inclusion("a".to_string(), "".to_string()));
        assert!(Solution::check_inclusion("".to_string(), "a".to_string()));
        assert!(Solution::check_inclusion(
            "adc".to_string(),
            "dcda".to_string()
        ));
        assert!(Solution::check_inclusion(
            "adc".to_string(),
            "dcda".to_string() // ac, [d]cda
                               // a, [dc]da
                               // ad, d[cd]a
        ));
    }

    #[test]
    fn test_substr_2() {
        assert!(Solution::check_inclusion(
            "abc".to_string(),
            "eidbaacboo".to_string()
        ));
    }
}
