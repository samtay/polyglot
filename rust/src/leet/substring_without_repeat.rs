//! Given a string s, find the length of the longest substring without repeating
//! characters.

use std::collections::HashSet;

pub struct Solution;
impl Solution {
    /// We'll use a sliding window
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n = s.len();
        let chars = s.as_bytes(); // guaranteed ascii only!
        if n == 0 {
            return 0;
        }
        let mut longest = 1;
        let mut set = HashSet::new();
        let mut i = 0;
        let mut j = 1;
        set.insert(chars[i]);

        while j < n {
            let c = chars[j];
            // Move the window
            while set.contains(&c) {
                set.remove(&chars[i]);
                i += 1;
            }
            // Enlarge the window
            set.insert(c);
            longest = std::cmp::max(longest, set.len() as i32);
            j += 1;
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substr() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
