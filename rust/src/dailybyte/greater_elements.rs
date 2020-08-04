//! This question is asked by Amazon. Given two arrays of numbers, where the first array is a
//! subset of the second array, return an array containing all the next greater elements for each
//! element in the first array, in the second array. If there is no greater element for any
//! element, output -1 for that number.

use std::collections::HashMap;

/// First let's try naive O(n*m) solution
pub fn greater_elements_naive(nums1: Vec<i8>, nums2: Vec<i8>) -> Vec<i8> {
    let mut out = Vec::new();
    for n in nums1 {
        let mut hit = false;
        let mut found = false;
        for &m in &nums2 {
            if m == n {
                hit = true;
            }
            if hit && n < m {
                out.push(m);
                found = true;
                break;
            }
        }
        if !found {
            out.push(-1);
        }
    }
    out
}

/// Let's use some extra space to save time. Build a hash map of the next greater elements, then
/// just iterate over nums1 and map each value to its key's value in the hashmap.
/// To build the hashmap, we'll iterate over nums2 while pushing the elements into a stack of keys
/// we need to find values for. As we iterate, if the current element is greater than the top of
/// the stack, continue to pop the stack and insert the popped element into the hash with value of
/// the current greater element.
///
/// Since we iterate over num2 doing constant-time operations, then iterate num1 in constant
/// iterations, this is just O(n + m) time. However the hashmap and stack are both the size of
/// nums2 so this is also O(m) space.
pub fn greater_elements(nums1: Vec<i8>, nums2: Vec<i8>) -> Vec<i8> {
    let mut greaters = HashMap::new();
    let mut stack = Vec::new();
    for n in nums2 {
        loop {
            match stack.last() {
                Some(&m) if m < n => {
                    greaters.insert(m, n);
                    stack.pop();
                }
                _ => break,
            }
        }
        stack.push(n);
    }
    nums1
        .into_iter()
        .map(|m| greaters.remove(&m).unwrap_or(-1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greater_elems() {
        assert_eq!(
            greater_elements(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
        assert_eq!(greater_elements(vec![2, 4], vec![1, 2, 3, 4]), vec![3, -1]);
    }
}
