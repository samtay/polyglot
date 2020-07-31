//! This question is asked by Google. Given a string only containing the following characters (, ),
//! {, }, [, and ] return whether or not the opening and closing characters are in a valid order.

/// We just need a stack. In rust, we use a Vec, which pushes and pops in constant time.
/// The solution below takes space O(n) and time O(n).
pub fn valid(s: &str) -> bool {
    // Note: this is not capacity s.len() / 2 because the string might be invalid
    let mut stack = Vec::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' if stack.pop() == Some('(') => (),
            '}' if stack.pop() == Some('{') => (),
            ']' if stack.pop() == Some('[') => (),
            _ => return false,
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(valid("(){}[]"));
        assert!(valid("(({[]}))"));
        assert!(!valid("{(})"));
        // Need one more test to make sure we close all braces
        assert!(!valid("(){}[]("));
    }
}
