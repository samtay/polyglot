//! This question is asked by Facebook. Given a linked list and a value n, remove the nth to last
//! node and return the resulting list.

use std::collections::LinkedList;

/// Pretending remove, len, etc. don't exist.
/// Panic on out of bounds index
pub fn remove_nth_rev<T>(a: &mut LinkedList<T>, n: usize) {
    let mut cursor = a.cursor_back_mut();
    for _ in 0..n {
        if cursor.as_cursor().current().is_none() {
            panic!("Index out of bounds");
        }
        cursor.move_prev();
    }
    cursor.remove_current().expect("Index out of bounds");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth() {
        let mut list: LinkedList<i8> = LinkedList::new();
        list.extend(&[1, 2, 3, 4, 5, 6]);
        remove_nth_rev(&mut list, 0);
        let mut expected = LinkedList::new();
        expected.extend(&[1, 2, 3, 4, 5]);
        assert_eq!(list, expected);

        remove_nth_rev(&mut list, 1);
        let mut expected = LinkedList::new();
        expected.extend(&[1, 2, 3, 5]);
        assert_eq!(list, expected);

        remove_nth_rev(&mut list, 3);
        let mut expected = LinkedList::new();
        expected.extend(&[2, 3, 5]);
        assert_eq!(list, expected);
    }

    #[test]
    #[should_panic]
    fn test_remove_nth_panic() {
        let mut list: LinkedList<i8> = LinkedList::new();
        list.extend(&[1, 2, 3, 4, 5, 6]);
        remove_nth_rev(&mut list, 29);
    }
}
