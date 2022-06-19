/// Write a function that reverses a string. The input string is given as an
/// array of characters s.

/// You must do this by modifying the input array in-place with O(1) extra
/// memory.

pub struct Solution();
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // ok given guarnatee of len > 0
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);

        let mut s = vec!['a'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['a']);

        let mut s = vec!['a', 'b'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['b', 'a']);
    }
}
