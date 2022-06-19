//! Given the head of a singly linked list, return the middle node of the linked
//! list.
//!
//! If there are two middle nodes, return the second middle node.

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &head;
        let mut double_speed = &head;
        let mut skip = true;
        loop {
            if let Some(ref node) = *double_speed {
                double_speed = &node.next;
            } else {
                return current.clone();
            }
            if !skip {
                if let Some(ref node) = *current {
                    current = &node.next;
                }
            }
            skip ^= true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_node() {
        let mut node3 = ListNode::new(3);
        node3.extend([4, 5]);
        let mut node2 = ListNode::new(2);
        node2.next = Some(Box::new(node3.clone()));
        let mut node1 = ListNode::new(1);
        node1.next = Some(Box::new(node2));
        assert_eq!(
            Solution::middle_node(Some(Box::new(node1.clone()))),
            Some(Box::new(node3))
        );

        let mut node4 = ListNode::new(4);
        node4.extend([5, 6]);
        let mut node3 = ListNode::new(3);
        node3.next = Some(Box::new(node4.clone()));
        let mut node2 = ListNode::new(2);
        node2.next = Some(Box::new(node3.clone()));
        let mut node1 = ListNode::new(1);
        node1.next = Some(Box::new(node2));
        assert_eq!(
            Solution::middle_node(Some(Box::new(node1.clone()))),
            Some(Box::new(node4))
        );
    }

    #[test]
    fn test_perf() {
        let mut node = ListNode::new(0);
        node.extend([100; 100]);
        Solution::middle_node(Some(Box::new(node)));
    }
}
