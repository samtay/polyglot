//! This question is asked by Facebook. Given a string and the ability to delete at most one
//! character, return whether or not it can form a palindrome.

/// Thoughts:
///   1) Naive: Just try palindrome, if it doesn't validate, then loop over each index and test
///      palindrome with that index removed. Since the `valid_palindrome` function is O(n), this
///      would be O(n^2).
///   2) Better: as we're iterating forward and backward a la valid_palindrome, allow one skipped
///      character at index i; if another pair fails, backtrack and try skipping n-i from backwards
///      iterator. If another pair fails after this, then we can't make a valid palindrome.
///      This is still O(n), however in the worst case of failing to make a valid palindrome, we do
///      need to allocate new iterators, so space complexity is slightly worse.
pub fn can_be_palindrome<S>(s: S) -> bool
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
    let mut removal = None;

    let mut f = forward.next();
    let mut b = backward.next();
    loop {
        match (f, b, removal) {
            // If we make it past the halfway point, it's a palindrome
            (Some((ixf, _)), Some((ixb, _)), _) if ixf >= ixb => return true,
            // If we've already backtracked a removal, it's not a palindrome
            (Some((_, fc)), Some((_, bc)), Some(Removal::Backward(_))) if fc != bc => return false,
            // Backtrack if we haven't
            (Some((_, fc)), Some((_, bc)), Some(Removal::Forward(ix))) if fc != bc => {
                // Reset iterators
                forward = s.char_indices();
                backward = s.char_indices().rev();
                // Start trying from ix again in forward dir
                f = forward.nth(ix);
                // Try dropping ix from backward dir
                b = backward.nth(ix + 1);
                removal = Some(Removal::Backward(ix));
            }
            // Remove in forward dir if we haven't yet
            (Some((ix, fc)), Some((_, bc)), None) if fc != bc => {
                f = forward.next();
                removal = Some(Removal::Forward(ix));
            }
            // Otherwise, this is still a candidate, just iterate
            (Some((_, fc)), Some((_, bc)), _) if fc == bc => {
                f = forward.next();
                b = backward.next();
            }
            _ => return true,
        }
    }
}

#[derive(Clone, Copy)]
enum Removal {
    Forward(usize),
    Backward(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_be_palindrome() {
        assert!(can_be_palindrome("abcba"));
        assert!(can_be_palindrome("foboof"));
        assert!(!can_be_palindrome("abccab"));
        assert!(can_be_palindrome("gfobof"));
        assert!(can_be_palindrome("fobofg"));
        assert!(!can_be_palindrome("aasdfghjklrkjhgfdsa")); // worst case?
    }
}
