/// Implement an algorithm to determine if a string is a permutation of a palindrome
use std::collections::HashMap;

/// Just keep odd character counts in a hashmap,
/// then check if the map contains at most 1 character
///
/// O(n)
pub fn palindrome_permutation(s: &str) -> bool {
    // Only keep the relevant chars
    let mut s = s.to_ascii_lowercase();
    s.retain(|c| c.is_ascii() && !c.is_whitespace());

    // Count odd occurences
    let odd_char_count = s.chars().fold(HashMap::new(), |mut hm, c| {
        if hm.remove(&c).is_none() {
            hm.insert(c, 1);
        };
        hm
    });

    // Return whether or not there's at most one (the middle of palindrome)
    odd_char_count.len() <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_permutation() {
        assert!(palindrome_permutation("Tact Coa"));
        assert!(palindrome_permutation("carecar"));
        assert!(!palindrome_permutation("rarr"));
    }
}
