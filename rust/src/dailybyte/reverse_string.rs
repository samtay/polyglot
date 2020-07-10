//! Pretending slice::reverse doesn't exist..

use unicode_segmentation::UnicodeSegmentation;

// Just creating a new one in O(n) space
fn reverse(s: impl Into<String>) -> String {
    let mut s = s.into();
    let mut new = String::new();
    while let Some(c) = s.pop() {
        new.push(c);
    }
    new
}

// Still O(n) space, creates a new String, but handles arbitrary unicode
fn reverse_better(s: &str) -> String {
    // Only allocate what we need
    let mut output = String::with_capacity(s.len());
    // Make sure we don't split at non-UTF8 boundary
    output.extend(s.graphemes(true).rev());
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("Cat"), "taC");
        assert_eq!(reverse("The Daily Byte"), "etyB yliaD ehT");
        assert_eq!(reverse("civic"), "civic");
    }

    #[test]
    fn test_reverse_better() {
        assert_eq!(reverse_better("The Daily Byte"), "etyB yliaD ehT");
        assert_eq!(reverse_better("civic"), "civic");
        assert_eq!(reverse_better("a̐éö̲\r\n"), "\r\nö̲éa̐");
        assert_eq!(reverse_better("two flags: 🇷🇺🇸🇹"), "🇸🇹🇷🇺 :sgalf owt");
    }
}
