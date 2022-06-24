//! Given a string s, return the longest palindromic substring in s.

pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // Solution must have size <= n. Start with the largest; find the first
        // one and return it.
        for i in (1..s.len() + 1).into_iter().rev() {
            for candidate in s.chars().into_iter().collect::<Vec<char>>().windows(i) {
                if is_palindrome(candidate) {
                    return candidate.iter().collect();
                }
            }
        }

        fn is_palindrome(cs: &[char]) -> bool {
            cs.iter().zip(cs.iter().rev()).all(|(a, b)| a == b)
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome_() {
        assert_eq!(
            Solution::longest_palindrome("yyyabcdz".to_string()),
            "yyy".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("yyyabcddcbazzz".to_string()),
            "abcddcba".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("yyyabcdcbazzz".to_string()),
            "abcdcba".to_string()
        );
        // or "aba"
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
        assert_eq!(Solution::longest_palindrome("".to_string()), "".to_string());
    }
}
