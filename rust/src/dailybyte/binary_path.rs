//! You are given the root of a binary tree. Each node in the tree contains
//! either a value of zero or a value of one. Each path in the tree from root to
//! leaf forms a binary string and therefore a decimal value. Return the sum of
//! all root to leaf paths using each path’s decimal value.
//!
//! Note: It is guaranteed that the sum of all paths will fit in an integer
//! value.

#[derive(Default)]
pub struct BinaryTree<T> {
    pub root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

/// Recurses the tree to list all possible paths, and then sums all paths in
/// binary. We do traverse across each node so this is at least O(n); however
/// after the traversal, we also sum for each leaf, the path to that leaf. Given
/// a tree of size n, there are (n/2) leaves, and each path has length log base
/// 2 n. So this ends up being O(n log n) where n is the number of nodes in the
/// tree.
pub fn binary_path_sum(tree: BinaryTree<i32>) -> i32 {
    match tree.root {
        None => 0,
        Some(node) => binary_paths(*node)
            .iter()
            .map(|path| {
                let s = path
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (n, b)| acc + b * 2_i32.pow(n as u32));
                s
            })
            .sum(),
    }
}

fn binary_paths(node: Node<i32>) -> Vec<Vec<i32>> {
    let Node { value, left, right } = node;
    let mut lefts = left.map_or(vec![], |n| binary_paths(*n));
    let mut rights = right.map_or(vec![], |n| binary_paths(*n));
    lefts.append(&mut rights);
    for path in &mut lefts {
        path.push(value);
    }
    if lefts.is_empty() {
        vec![vec![value]]
    } else {
        lefts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_path_sum() {
        let t = BinaryTree {
            root: Some(Box::new(Node {
                value: 0,
                left: Some(Box::new(Node {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 1,
                    left: None,
                    right: None,
                })),
            })),
        };
        assert_eq!(binary_path_sum(t), 2);
        let t = BinaryTree {
            root: Some(Box::new(Node {
                value: 1,
                left: Some(Box::new(Node {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 0,
                    left: None,
                    right: None,
                })),
            })),
        };
        assert_eq!(binary_path_sum(t), 5);
        let t = BinaryTree {
            root: Some(Box::new(Node {
                value: 1,
                left: Some(Box::new(Node {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 0,
                    left: Some(Box::new(Node {
                        value: 1,
                        left: Some(Box::new(Node {
                            value: 1,
                            left: None,
                            right: None,
                        })),
                        right: Some(Box::new(Node {
                            value: 0,
                            left: None,
                            right: None,
                        })),
                    })),
                    right: None,
                })),
            })),
        };
        assert_eq!(binary_path_sum(t), 24);
    }
}
