//! Given a string s, return the longest palindromic substring in s.

use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let chars = s.as_bytes(); // guaranteed ascii only!
        if n == 0 {
            return String::new();
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

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        // Or "aba"
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("b".to_string()),
            "b".to_string()
        );
    }
}
