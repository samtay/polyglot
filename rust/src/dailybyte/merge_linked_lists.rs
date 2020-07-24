//! This question is asked by Apple. Given two sorted linked lists, merge them together in
//! ascending order and return a reference to the merged list.

use std::collections::LinkedList;

/// Since they are already sorted, we can just iterate across each one and choose whichever value
/// is smallest first. I'm just going to make a new list which will have space O(n + m) and time
/// O(n + m). (Removing elements is constant time because we only ever remove the head of lhe
/// list.)
pub fn merge<T: Ord>(mut a: LinkedList<T>, mut b: LinkedList<T>) -> LinkedList<T> {
    let mut a = a.cursor_front_mut();
    let mut b = b.cursor_front_mut();
    let mut out = LinkedList::new();
    loop {
        match (a.as_cursor().current(), b.as_cursor().current()) {
            (Some(n), Some(m)) if n < m => {
                let n = a.remove_current().unwrap();
                out.push_back(n);
            }
            (Some(_), None) => {
                let n = a.remove_current().unwrap();
                out.push_back(n);
            }
            (None, None) => {
                break;
            }
            // Dont dupe code, just swap branches
            _ => {
                std::mem::swap(&mut a, &mut b);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut list1: LinkedList<i8> = LinkedList::new();
        list1.extend(&[1, 2, 3]);
        let mut list2 = LinkedList::new();
        list2.extend(&[4, 5, 6]);
        let mut expected = LinkedList::new();
        expected.extend(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(merge(list1, list2), expected);

        let mut list1: LinkedList<i8> = LinkedList::new();
        list1.extend(&[1, 3, 5]);
        let mut list2 = LinkedList::new();
        list2.extend(&[2, 4, 6]);
        let mut expected = LinkedList::new();
        expected.extend(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(merge(list1, list2), expected);

        let mut list1: LinkedList<i8> = LinkedList::new();
        list1.extend(&[4, 4, 7]);
        let mut list2 = LinkedList::new();
        list2.extend(&[1, 5, 6]);
        let mut expected = LinkedList::new();
        expected.extend(&[1, 4, 4, 5, 6, 7]);
        assert_eq!(merge(list1, list2), expected);
    }
}
