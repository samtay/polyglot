// Implement an algorithm to determine if one string is a permutation of another

use std::collections::HashMap;
use std::collections::hash_map::Entry;

// Store characters of string s2 in hash map with counts: O(m)
// Iterate over s1, decrementing counts in hash map: O(n) amortized
// Return whether or not all hash map elements 0: O(m) amortized
//
// O(max(n, m)) amortized total
//
// Actually, since we exit in O(1) when n != m, this is just O(n) unambiguously.
fn check_permutation(s1: &str, s2: &str) -> bool {

    // Only keep the relevant chars
    let s1 = anagram_hash(s1);
    let s2 = anagram_hash(s2);

    // Quick return for obvious non-permutations
    if s1.len() != s2.len() {
        return false;
    }

    // Build hashmap of s2 char counts
    let mut char_counts = s2.chars().fold(HashMap::new(), |mut hm, c| {
        hm.entry(c)
          .and_modify(|ct| { *ct += 1 })
          .or_insert(1);
        hm
    });

    // Decrement the hashmap with s1 char counts
    for c in s1.chars() {
        if let Entry::Vacant(_) = char_counts.entry(c).and_modify(|e| { *e -= 1 }) {
            return false;
        }
    }

    // Return whether or not hashmap is empty
    char_counts.retain(|_, &mut ct| ct != 0);
    char_counts.is_empty()
}

fn anagram_hash(s: &str) -> String {
    let mut hash = s.to_lowercase();     // expensive reallocation
    hash.retain(|c| !c.is_whitespace()); // cheap in-place modification
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_permutation() {
        assert_eq!(check_permutation("Tom Marvolo Riddle", "Marmot Dildo Lover"), true);
        assert_eq!(check_permutation("Tommy", "Tammy"), false);
    }
}
