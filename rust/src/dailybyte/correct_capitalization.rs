//! This question is asked by Google. Given a string, return whether or not it uses capitalization
//! correctly. A string correctly uses capitalization if all letters are capitalized, no letters are
//! capitalized, or only the first letter is capitalized.

pub fn is_correct<S>(s: S) -> bool
where
    S: Into<String>,
{
    let s = s.into();
    let mut cs = s.chars();
    match (
        cs.next().map(|c| c.is_uppercase()),
        cs.next().map(|c| c.is_uppercase()),
    ) {
        (Some(true), Some(true)) => cs.all(|c| c.is_uppercase()),
        (Some(true), Some(false)) | (Some(false), Some(false)) => cs.all(|c| c.is_lowercase()),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacuum_position() {
        assert!(is_correct("USA"));
        assert!(is_correct("Calvin"));
        assert!(!is_correct("compUter"));
        assert!(is_correct("coding"));
    }
}
