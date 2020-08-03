//! This question is asked by Facebook. Given a string s containing only lowercase letters,
//! continuously remove adjacent characters that are the same and return the result.

/// This will be easiest by allocating an extra stack. We iterate over `s`, and peek at the last
/// element on the stack. If the two chars are equal, we pop the stack and skip the element.
/// Otherwise we push the current char. This is O(s) time and O(s) space.
pub fn remove_adj_dupes(s: &str) -> String {
    s.chars()
        .fold::<Vec<char>, _>(Vec::with_capacity(s.len()), |mut chars, c| {
            match chars.last() {
                Some(&ch) if ch == c => {
                    chars.pop();
                }
                _ => chars.push(c),
            }
            chars
        })
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_adj_dupes() {
        assert_eq!(remove_adj_dupes("abccba"), String::from(""));
        assert_eq!(remove_adj_dupes("foobar"), String::from("fbar"));
        assert_eq!(remove_adj_dupes("abccbefggfe"), String::from("a"));
    }
}
