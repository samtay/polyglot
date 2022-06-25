//! You are given two binary trees root1 and root2.
//!
//! Imagine that when you put one of them to cover the other, some nodes of the
//! two trees are overlapped while the others are not. You need to merge the two
//! trees into a new binary tree. The merge rule is that if two nodes overlap,
//! then sum node values up as the new value of the merged node. Otherwise, the
//! NOT null node will be used as the node of the new tree.
//!
//! Return the merged tree.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root1
            .clone()
            .zip(root2.clone())
            .map(|(r1, r2)| {
                let r1 = r1.borrow();
                let r2 = r2.borrow();
                let left = Self::merge_trees(r1.left.clone(), r2.left.clone());
                let right = Self::merge_trees(r1.right.clone(), r2.right.clone());
                let val = r1.val + r2.val;
                Rc::new(RefCell::new(TreeNode { val, left, right }))
            })
            .or(root1)
            .or(root2)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_merge_trees() {
        let root1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let r2c1 = Rc::new(RefCell::new(TreeNode::new(3)));
        let r2c2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let r3c1 = Rc::new(RefCell::new(TreeNode::new(5)));
        r2c1.borrow_mut().left = Some(r3c1);
        root1.borrow_mut().left = Some(r2c1);
        root1.borrow_mut().right = Some(r2c2);

        let root2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let r2c1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let r2c2 = Rc::new(RefCell::new(TreeNode::new(3)));
        let r3c2 = Rc::new(RefCell::new(TreeNode::new(4)));
        let r3c4 = Rc::new(RefCell::new(TreeNode::new(7)));
        r2c1.borrow_mut().right = Some(r3c2);
        r2c2.borrow_mut().right = Some(r3c4);
        root2.borrow_mut().left = Some(r2c1);
        root2.borrow_mut().right = Some(r2c2);

        let root3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let r2c1 = Rc::new(RefCell::new(TreeNode::new(4)));
        let r2c2 = Rc::new(RefCell::new(TreeNode::new(5)));
        let r3c1 = Rc::new(RefCell::new(TreeNode::new(5)));
        let r3c2 = Rc::new(RefCell::new(TreeNode::new(4)));
        let r3c4 = Rc::new(RefCell::new(TreeNode::new(7)));
        r2c1.borrow_mut().left = Some(r3c1);
        r2c1.borrow_mut().right = Some(r3c2);
        r2c2.borrow_mut().right = Some(r3c4);
        root3.borrow_mut().left = Some(r2c1);
        root3.borrow_mut().right = Some(r2c2);

        assert_eq!(Solution::merge_trees(Some(root1), Some(root2)), Some(root3));
    }
}
