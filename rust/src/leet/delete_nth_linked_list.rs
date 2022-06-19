//! Given the head of a linked list, remove the nth node from the end of the
//! list and return its head.

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        // Ahead gets cloned data
        let mut ahead = head.clone();
        // Current must take a mutable ref to the head we're going to return
        let mut current = &mut head;
        let mut counter = 0;
        while let Some(node) = ahead {
            ahead = node.next;
            if counter >= n {
                let node = (*current).as_mut()?;
                current = &mut node.next;
            }
            counter += 1;
        }
        *current = current.clone()?.next;
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_nth() {
        let mut node_original = ListNode::new(1);
        node_original.extend([2, 3, 4, 5]);
        let mut node_expected = ListNode::new(1);
        node_expected.extend([2, 3, 5]);
        assert_eq!(
            Solution::remove_nth_from_end(Some(Box::new(node_original)), 2),
            Some(Box::new(node_expected))
        );

        let node_original = ListNode::new(1);
        assert!(Solution::remove_nth_from_end(Some(Box::new(node_original)), 1).is_none());

        let mut node_original = ListNode::new(1);
        node_original.extend([2]);
        let node_expected = ListNode::new(1);
        assert_eq!(
            Solution::remove_nth_from_end(Some(Box::new(node_original)), 1),
            Some(Box::new(node_expected))
        );
    }

    #[test]
    fn test_perf() {
        let mut node = ListNode::new(0);
        node.extend([100; 100]);
        Solution::remove_nth_from_end(Some(Box::new(node)), 20);
    }
}
