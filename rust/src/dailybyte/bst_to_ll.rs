//! Given a binary search tree, rearrange the tree such that it forms a linked list where all its
//! values are in ascending order.
//!
//! Note: all values in the binary search tree will be unique.

#[derive(Default)]
pub struct BST<T: Ord> {
    root: BSTLink<T>,
}

#[derive(Debug)]
pub struct BSTNode<T> {
    value: T,
    left: BSTLink<T>,
    right: BSTLink<T>,
}

type BSTLink<T> = Option<Box<BSTNode<T>>>;

#[derive(Default, Debug)]
pub struct LL<T> {
    pub root: LLLink<T>,
}

#[derive(Debug)]
pub struct LLNode<T> {
    value: T,
    next: LLLink<T>,
}

type LLLink<T> = Option<Box<LLNode<T>>>;

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
        *current_link = Some(Box::new(BSTNode {
            value: val,
            left: None,
            right: None,
        }));
    }
}

impl<T> LL<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Append value
    pub fn insert(&mut self, val: T) {
        let mut current_link = &mut self.root;
        while let Some(ref mut current_node) = *current_link {
            current_link = &mut current_node.next;
        }
        *current_link = Some(Box::new(LLNode {
            value: val,
            next: None,
        }));
    }

    /// Append node
    pub fn insert_node(&mut self, link: LLLink<T>) {
        let mut current_link = &mut self.root;
        while let Some(ref mut current_node) = *current_link {
            current_link = &mut current_node.next;
        }
        *current_link = link;
    }

    /// Append list
    pub fn append(&mut self, list: LL<T>) {
        self.insert_node(list.root);
    }
}

impl<T: PartialEq> PartialEq for LL<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut self_link = &self.root;
        let mut other_link = &other.root;
        loop {
            match (self_link, other_link) {
                (Some(ref self_node), Some(ref other_node)) => {
                    if self_node.value != other_node.value {
                        return false;
                    }
                    self_link = &self_node.next;
                    other_link = &other_node.next;
                }
                (None, None) => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
    }
}

impl<T: Ord> From<BST<T>> for LL<T> {
    /// The key thing to remember is that in a BST, everything to the right of a given node is
    /// greater than everything to the left.
    fn from(bst: BST<T>) -> Self {
        bst_to_ll(bst.root)
    }
}

/// This is the solution. It's a simple recursive function that always chooses to build a list with
/// the left most values before the rightmost values. Because of its recursion, it uses space O(log
/// n) (since there are log n calls added to the stack). The time complexity is also O(n) because
/// we traverse all n nodes.
///
/// TODO or do I have time and space complexities mixed up here?
fn bst_to_ll<T: Ord>(bst: BSTLink<T>) -> LL<T> {
    match bst {
        None => LL::new(),
        Some(bst_node) => {
            let BSTNode { value, left, right } = *bst_node;
            let mut ll = LL::new();
            if left.is_some() {
                ll.append(bst_to_ll(left));
            }
            ll.insert(value);
            if right.is_some() {
                ll.append(bst_to_ll(right));
            }
            ll
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let bst = [5, 1, 6].iter().fold(BST::new(), |mut bst, &n| {
            bst.insert(n);
            bst
        });
        let ll = LL::from(bst);
        let expected_ll = [1, 5, 6].iter().fold(LL::new(), |mut ll, &n| {
            ll.insert(n);
            ll
        });
        assert_eq!(ll, expected_ll);

        let bst = [5, 2, 9, 1, 3].iter().fold(BST::new(), |mut bst, &n| {
            bst.insert(n);
            bst
        });
        let ll = LL::from(bst);
        let expected_ll = [1, 2, 3, 5, 9].iter().fold(LL::new(), |mut ll, &n| {
            ll.insert(n);
            ll
        });
        assert_eq!(ll, expected_ll);

        let bst = [5, 6].iter().fold(BST::new(), |mut bst, &n| {
            bst.insert(n);
            bst
        });
        let ll = LL::from(bst);

        let expected_ll = [5, 6].iter().fold(LL::new(), |mut ll, &n| {
            ll.insert(n);
            ll
        });
        assert_eq!(ll, expected_ll);
    }
}
