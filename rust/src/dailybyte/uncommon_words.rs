//! This question is asked by Amazon. Given two strings representing sentences, return the words
//! that are not common to both strings (i.e. the words that only appear in one of the sentences).
//! You may assume that each sentence is a sequence of words (without punctuation) correctly
//! separated using space characters.

use std::collections::HashMap;
use std::collections::HashSet;

enum Found {
    Once,
    Twice,
}

/// Note: the test cases given by DB don't have a consistent ordering. So, I'm just going to return
/// a set.
///
/// Approach: Build a hashmap containing the words in sentence `a`. Build a hashset by iterating
/// over the words in `b`, keeping only the words _not_ found in the `a`; for those that _are_
/// found, flag them in the hashmap as common. Then, extend the hashset by those elements in the
/// hashmap that are _not_ common.
pub fn uncommon_words<'a>(a: &'a str, b: &'a str) -> HashSet<&'a str> {
    let mut found = HashMap::new();
    let mut uncommon = HashSet::new();
    found.extend(a.split_whitespace().map(|w| (w, Found::Once)));
    for w in b.split_whitespace() {
        if found.contains_key(&w) {
            found.insert(w, Found::Twice);
        } else {
            uncommon.insert(w);
        }
    }
    found.retain(|_, f| match f {
        Found::Once => true,
        Found::Twice => false,
    });
    uncommon.extend(found.keys());
    uncommon
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_uncommon_words() {
        assert_eq!(
            uncommon_words("the quick", "brown fox"),
            HashSet::from_iter(vec!["the", "quick", "brown", "fox"])
        );
        assert_eq!(
            uncommon_words(
                "the tortoise beat the haire",
                "the tortoise lost to the haire"
            ),
            HashSet::from_iter(vec!["beat", "to", "lost"])
        );
        assert_eq!(
            uncommon_words("copper coffee pot", "hot coffee pot"),
            HashSet::from_iter(vec!["copper", "hot"])
        );
    }
}
