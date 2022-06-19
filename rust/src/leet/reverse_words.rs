//! Given a string s, reverse the order of characters in each word within a
//! sentence while still preserving whitespace and initial word order.

pub struct Solution();
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .map(|word| {
                // safe given ascii constraint
                word.chars().rev().collect::<String>()
            })
            .into_iter()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
        assert_eq!(
            Solution::reverse_words("God Ding".to_string()),
            "doG gniD".to_string()
        );
    }
}
