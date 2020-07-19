//! This question is asked by Facebook. Given a string, return whether or not it forms a palindrome
//! ignoring case and non-alphabetical characters.

/// O(n) where n is the length of input.
pub fn valid_palindrome<S>(s: S) -> bool
where
    S: Into<String>,
{
    let mut s = s.into();
    // ignore case
    s.make_ascii_lowercase();
    // ignore non-alphabetical characters
    s.retain(|c| c.is_ascii() && c.is_alphabetic());

    let mut forward = s.char_indices();
    let mut backward = s.char_indices().rev();
    loop {
        match (forward.next(), backward.next()) {
            (Some((ixf, fc)), Some((ixb, bc))) if ixf <= ixb => {
                if fc != bc {
                    return false;
                }
            }
            _ => break,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_palindrome() {
        assert!(valid_palindrome("level"));
        assert!(valid_palindrome("levvel"));
        assert!(!valid_palindrome("algorithm"));
        assert!(valid_palindrome("A man, a plan, a canal: Panama."));
    }
}
