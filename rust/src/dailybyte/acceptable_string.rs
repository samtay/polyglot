//! Given a string, s, make it acceptable. An acceptable string is a string that
//! contains no two adjacent characters that are the same with one letter being
//! capitalized and the other being lowercased.
//! Note: An empty string is acceptable and only one distinct answer will exist.

pub fn acceptable(orig: String) -> String {
    let mut new = String::new();
    for b in orig.chars() {
        if let Some(a) = new.pop() {
            if a.is_uppercase() == b.is_uppercase()
                || a.to_ascii_lowercase() != b.to_ascii_lowercase()
            {
                new.push(a);
                new.push(b);
            }
        } else {
            new.push(b);
        }
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptable() {
        assert_eq!(acceptable(String::from("Aabb")), String::from("bb"));
        assert_eq!(acceptable(String::from("AabBcC")), String::from(""));
        assert_eq!(acceptable(String::from("dAabBcCD")), String::from(""));
        assert_eq!(acceptable(String::from("dAabBcCde")), String::from("dde"));
    }
}
