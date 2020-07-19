//! This question is asked by Microsoft. Given an array of strings, return the longest common prefix
//! that is shared amongst all strings.  Note: you may assume all strings only contain lowercase
//! alphabetical characters.

pub fn longest_prefix<'a>(strings: &[&'a str]) -> &'a str {
    let mut length = 0;
    loop {
        let ixs = strings.iter().map(|s| s.chars().nth(length));
        if all_equal(ixs) {
            length += 1;
        } else {
            break;
        }
    }
    &strings.first().unwrap_or(&"")[..length]
}

fn all_equal<I>(list: I) -> bool
where
    I: IntoIterator,
    I::Item: Eq,
{
    let mut list = list.into_iter();
    if let Some(first) = list.next() {
        list.all(|item| item == first)
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix() {
        assert_eq!(longest_prefix(&["colorado", "color", "cold"]), "col");
        assert_eq!(longest_prefix(&["a", "b", "c"]), "");
        assert_eq!(longest_prefix(&["spot", "spotty", "spotted"]), "spot");
    }
}
