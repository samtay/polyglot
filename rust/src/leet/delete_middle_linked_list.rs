//! You are given the head of a linked list. Delete the middle node, and return
//! the head of the modified linked list.
//!
//! The middle node of a linked list of size n is the ⌊n / 2⌋th node from the
//! start using 0-based indexing, where ⌊x⌋ denotes the largest integer less
//! than or equal to x.

// Definition for singly-linked list provided by LC.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn insert(&mut self, val: i32) {
        let mut curr = &mut self.next;
        while let Some(ref mut node) = *curr {
            curr = &mut node.next
        }
        *curr = Some(Box::new(Self::new(val)));
    }

    pub fn extend<I>(&mut self, vals: I)
    where
        I: IntoIterator<Item = i32>,
    {
        for i in vals.into_iter() {
            self.insert(i);
        }
    }
}

pub struct Solution;
impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut double_speed = head.clone();
        let mut current = &mut head;
        let mut skip = true;
        while let Some(node) = double_speed {
            double_speed = node.next;
            if !skip {
                if let Some(ref mut node) = *current {
                    current = &mut node.next;
                }
            }
            skip ^= true;
        }
        *current = current.clone()?.next;
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_middle_node() {
        let mut node_original = ListNode::new(1);
        node_original.extend([3, 4, 7, 1, 2, 6]);
        let mut node_expected = ListNode::new(1);
        node_expected.extend([3, 4, 1, 2, 6]);
        assert_eq!(
            Solution::delete_middle(Some(Box::new(node_original))),
            Some(Box::new(node_expected))
        );

        let mut node_original = ListNode::new(1);
        node_original.extend([2, 3, 4]);
        let mut node_expected = ListNode::new(1);
        node_expected.extend([2, 4]);
        assert_eq!(
            Solution::delete_middle(Some(Box::new(node_original))),
            Some(Box::new(node_expected))
        );

        let mut node_original = ListNode::new(2);
        node_original.extend([1]);
        let node_expected = ListNode::new(2);
        assert_eq!(
            Solution::delete_middle(Some(Box::new(node_original))),
            Some(Box::new(node_expected))
        );
    }

    #[test]
    fn test_perf() {
        let mut node = ListNode::new(0);
        node.extend([100; 100]);
        Solution::delete_middle(Some(Box::new(node)));
    }
}
