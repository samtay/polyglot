// Implement an algorithm to determine if a string has all unique characters.
// What if you cannot use additional data structures?

// Thought Process:
// A: sort first, then check adjacent -> O(n log n + n) == O(n log n)
// B: iterate across string, checking or appending to "seen" list -> O(n)

use std::collections::HashMap;

fn is_unique(s: &str) -> bool {
    // Store seen characters in hash table
    //   O(s) outer loop where s is length of string
    //     O(1) hash inserts and lookups (amortized)
    //
    // O(s) total
    let mut seen = HashMap::new();
    s.chars().fold(true, |uniq, c| {
        if !uniq || seen.contains_key(&c) {
            return false;
        }
        seen.insert(c, true);
        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert_eq!(is_unique("i am not unique"), false);
        assert_eq!(is_unique("me tho"), true);
    }
}
