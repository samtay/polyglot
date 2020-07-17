//! This question is asked by Amazon. Given a string representing your stones and another string
//! representing a list of jewels, return the number of stones that you have that are also jewels.

use std::collections::HashMap;

/// Iterates over jewels, then stones, so time complexity is O(j + s)
/// Creates a hashmap of jewels, so has space complexity O(j)
fn num_jewels(stones: &str, jewels: &str) -> u16 {
    let mut jewels: HashMap<char, u16> = jewels.chars().fold(HashMap::new(), |mut js, c| {
        js.insert(c, 0);
        js
    });
    for c in stones.chars() {
        jewels.entry(c).and_modify(|i| *i += 1);
    }
    jewels.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_jewels() {
        assert_eq!(num_jewels("abc", "ac"), 2);
        assert_eq!(num_jewels("AaaddfFf", "Af"), 3);
        assert_eq!(num_jewels("AYOPD", "ayopd"), 0);
    }
}
