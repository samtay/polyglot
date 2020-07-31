//! This question is asked by Amazon. Given two strings s and t, which represents a sequence of
//! keystrokes, where # denotes a backspace, return whether or not the sequences produce the same
//! result.

/// Notice that we really won't be able short circuit early, because wherever we are in the
/// iteration, one string might backspace over its entire contents later on. So we need to iterate
/// over both lists, using a stack for each, then compare the strings, hence this is O(s + t) space
/// and O(s + t) time.
pub fn compare_keystrokes<'a>(s: &'a str, t: &'a str) -> bool {
    parse_keystrokes(s) == parse_keystrokes(t)
}

/// Space O(n) and time O(n)
pub fn parse_keystrokes(keys: &str) -> String {
    keys.chars()
        .fold::<Vec<char>, _>(Vec::with_capacity(keys.len()), |mut chars, key| {
            match key {
                '#' => {
                    chars.pop();
                }
                _ => chars.push(key),
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
    fn test_keystrokes() {
        // Test parsing
        assert_eq!(parse_keystrokes("ABC#"), String::from("AB"));
        assert_eq!(parse_keystrokes("como#pur#ter"), String::from("computer"));
        assert_eq!(parse_keystrokes("cof#dim#ng"), String::from("coding"));
        // DB examples
        assert!(compare_keystrokes("ABC#", "CD##AB"));
        assert!(compare_keystrokes("como#pur#ter", "computer"));
        assert!(!compare_keystrokes("cof#dim#ng", "code"));
    }
}
