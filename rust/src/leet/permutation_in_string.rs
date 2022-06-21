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

        let mut hm = HashMap::new();
        let mut i = 0;
        let mut j = 0;

        let cs = s2.as_bytes();
        let n = s2.len();

        while j < n {
            let c = cs[j] as char;
            // Compare the two char counts hashmap
            match (s1_hm.get(&c).unwrap_or(&0), hm.get(&c).unwrap_or(&0)) {
                // Continue to widen this window
                (ct1, ct2) if ct1 > ct2 => {
                    hm.entry(c).and_modify(|ct| *ct += 1).or_insert(1);
                }

                // Unsuccessful window, with a useless char
                // In this case move the whole window over and start over
                (0, _) => {
                    i = j + 1;
                    hm = HashMap::new();
                }

                // Unsuccessful window, but still potentially salvagable
                // Shift i over until we can accept the new c.
                _ => {
                    while cs[i] as char != c {
                        let ct = hm
                            .entry(cs[i] as char)
                            .and_modify(|ct| *ct -= 1)
                            .or_insert(0);
                        if *ct == 0 {
                            hm.remove(&(cs[i] as char));
                        }
                        i += 1;
                    }
                    i += 1;
                }
            }
            // Window successfully cleared the char counts!
            if hm == s1_hm {
                return true;
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
    fn test_inclusion() {
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
    fn test_inclusion_2() {
        assert!(Solution::check_inclusion(
            "abc".to_string(),
            "eidbaacboo".to_string()
        ));
    }
}
