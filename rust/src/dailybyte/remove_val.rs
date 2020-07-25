//! This question is asked by Google. Given a linked list and a value, remove all nodes containing
//! the provided value, and return the resulting list.

use std::collections::LinkedList;

/// Just traverse and remove whenever we see equal values. O(n) time and O(1) space.
pub fn remove<T: Eq>(list: &mut LinkedList<T>, val: &T) {
    let mut cursor = list.cursor_front_mut();
    while let Some(current) = cursor.current() {
        if current == val {
            cursor.remove_current();
        } else {
            cursor.move_next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        let mut list: LinkedList<i8> = LinkedList::new();
        list.extend(&[1, 2, 3]);
        remove(&mut list, &3);
        let mut expected = LinkedList::new();
        expected.extend(&[1, 2]);
        assert_eq!(list, expected);

        let mut list: LinkedList<i8> = LinkedList::new();
        list.extend(&[8, 1, 1, 4, 12]);
        remove(&mut list, &1);
        let mut expected = LinkedList::new();
        expected.extend(&[8, 4, 12]);
        assert_eq!(list, expected);

        let mut list: LinkedList<i8> = LinkedList::new();
        list.extend(&[7, 12, 2, 9]);
        remove(&mut list, &7);
        let mut expected = LinkedList::new();
        expected.extend(&[12, 2, 9]);
        assert_eq!(list, expected);
    }
}
