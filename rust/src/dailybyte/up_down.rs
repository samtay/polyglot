//! You are given a string, s, where each character in the string is either 'U'
//! (Up) or 'D' (Down). Return any permutation of an array, A, such that for
//! every index in the array where s[i] == U then A[i] < A[i + 1] and for every
//! index where s[i] == D then A[i] > A[i + 1].

use std::cmp;

pub fn up_down(s: &str) -> Vec<i32> {
    s.chars().fold(vec![0], |mut xs, c| {
        match c {
            'U' => xs.push(*xs.last().unwrap() + 1),
            'D' => xs.push(*xs.last().unwrap() - 1),
            _ => panic!("Bad input string"),
        }
        xs
    })
}

/// The first problem is dumb. I think by "permutation" they meant "set", which
/// makes this problem much more interesting. If you naively just do the same
/// thing but skip previously used numbers by searching the current list, I
/// think you'd end up with O(n!) in the worst case! E.g. input 'UDUDUDUD'.
///
/// So instead we'll keep around max/mins to jump to boundaries to avoid
/// previously used integers. Still runs in O(n).
pub fn up_down_set(s: &str) -> Vec<i32> {
    let mut min = 0;
    let mut max = 0;
    let mut last_x = 0;
    let mut last_dir: Option<char> = None;
    s.chars().fold(vec![0], |mut xs, c| {
        let next = match (last_dir, c) {
            (Some('D'), 'U') => max + 1,
            (Some('U'), 'D') => min - 1,
            (_, 'U') => last_x + 1,
            (_, 'D') => last_x - 1,
            (_, _) => panic!("Invalid string input"),
        };
        xs.push(next);
        min = cmp::min(next, min);
        max = cmp::max(next, max);
        last_x = next;
        last_dir = Some(c);
        xs
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up_down() {
        assert_eq!(up_down("UD"), vec![0, 1, 0]);
        assert_eq!(up_down("DUU"), vec![0, -1, 0, 1]);
    }
    #[test]
    fn test_up_down_set() {
        assert_eq!(up_down_set("UD"), vec![0, 1, -1]);
        assert_eq!(up_down_set("DUU"), vec![0, -1, 1, 2]);
        assert_eq!(up_down_set("UDUDUD"), vec![0, 1, -1, 2, -2, 3, -3]);
    }
}
