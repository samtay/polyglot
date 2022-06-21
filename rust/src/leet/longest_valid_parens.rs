//! Given a string containing just the characters '(' and ')', find the length
//! of the longest valid (well-formed) parentheses substring.

use std::collections::HashSet;

pub struct Solution;
impl Solution {
    /// We'll use a sliding window
    pub fn longest_valid_parentheses(s: String) -> i32 {
        // TODO implement parens logic

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
    fn test_valid_parens() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }
}
