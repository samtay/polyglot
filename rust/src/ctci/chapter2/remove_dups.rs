//! Write code to remove duplicates from an unsorted linked list.

use std::collections::HashSet;
use std::collections::LinkedList;
use std::hash::Hash;

/// Note: LinkedLists in Rust are hard. The code below is awkward because safe Rust won't let us
/// delete nodes based on a list containing references to previous nodes; this could lead to us
/// later using our mutable cursor access to delete previous nodes that are referenced in said
/// list, resulting in dangling pointers.
pub fn remove_dups<T: Eq + Hash + Copy>(list: &mut LinkedList<T>) {
    let mut seen = HashSet::new();
    let mut cursor = list.cursor_front_mut();

    while let Some(current) = cursor.current() {
        if seen.contains(current) {
            cursor.remove_current();
        } else {
            seen.insert(*current);
            cursor.move_next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_dups() {
        let mut numbers: LinkedList<u32> = LinkedList::new();
        numbers.extend(&[1, 21, 32, 4, 1, 20, 8, 90, 5, 20]);
        remove_dups(&mut numbers);
        let mut expected_numbers: LinkedList<u32> = LinkedList::new();
        expected_numbers.extend(&[1, 21, 32, 4, 20, 8, 90, 5]);
        assert_eq!(numbers, expected_numbers);
    }
}
