//! This question is asked by Google. Given the reference to the root of a binary search tree and a
//! search value, return the reference to the node that contains the value if it exists and `null`
//! otherwise.
//!
//! Note: all values in the binary search tree will be unique.

#[derive(Default)]
pub struct BST<T: Ord> {
    root: Link<T>,
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, val: T) {
        let mut current_link = &mut self.root;
        while let Some(ref mut current_node) = *current_link {
            if val < current_node.value {
                current_link = &mut current_node.left;
            } else {
                // OK bc we're assuming all unique values
                current_link = &mut current_node.right;
            }
        }
        *current_link = Some(Box::new(Node {
            value: val,
            left: None,
            right: None,
        }));
    }

    pub fn find_value(&self, val: &T) -> &Link<T> {
        let mut current_link = &self.root;
        loop {
            let tmp = current_link;
            match *tmp {
                Some(ref current_node) if val < &current_node.value => {
                    current_link = &current_node.left;
                }
                Some(ref current_node) if val > &current_node.value => {
                    current_link = &current_node.right;
                }
                Some(_) => return tmp,
                None => return &None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let mut bst = BST::new();
        bst.insert(3);
        bst.insert(1);
        bst.insert(4);
        assert_eq!(bst.find_value(&1).as_ref().unwrap().value, 1);

        let mut bst = BST::new();
        bst.insert(7);
        bst.insert(5);
        bst.insert(9);
        bst.insert(8);
        bst.insert(10);
        assert_eq!(bst.find_value(&9).as_ref().unwrap().value, 9);
        assert_eq!(
            bst.find_value(&9)
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value,
            8
        );

        let mut bst = BST::new();
        bst.insert(8);
        bst.insert(6);
        bst.insert(9);
        assert!(bst.find_value(&7).is_none());
    }
}
