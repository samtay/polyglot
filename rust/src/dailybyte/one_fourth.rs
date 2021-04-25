//! Given an array of integers, nums, sorted in ascending order, return the
//! element that occurs more than one fourth of the time.  Note: If no element
//! appears more than a fourth of the time, return -1.

use std::collections::HashMap;

/// I wrote this before noticing the list is ascending. This runs in O(n) space
/// and O(n) time.
pub fn one_fourth_bad(xs: &[i16]) -> i16 {
    // Build frequency hash map
    let quarter_length = xs.len() / 4;
    let xs_counts: HashMap<i16, i16> = xs.iter().fold(HashMap::new(), |mut cs, &x| {
        cs.entry(x).and_modify(|n| *n += 1).or_insert(1);
        cs
    });

    for (x, c) in xs_counts {
        if c > quarter_length as i16 {
            return x;
        }
    }

    -1
}

/// Knowing that the list is ascending, we can just traverse it once without
/// building a hashmap. This runs in O(n) time and constant space.
pub fn one_fourth(xs: &[i16]) -> i16 {
    // Build frequency hash map
    let quarter_length = (xs.len() / 4) as i16;
    let mut last_elem = None;
    let mut counter = 0;

    for x in xs {
        if let Some(y) = last_elem {
            if x == y {
                if counter + 1 > quarter_length {
                    return *x;
                }
            } else {
                counter = 0;
            }
        }
        last_elem = Some(x);
        counter += 1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_fourth_bad() {
        assert_eq!(one_fourth_bad(&[1, 2, 2, 3, 4]), 2);
        assert_eq!(one_fourth_bad(&[1, 2, 3, 4]), -1);
    }

    #[test]
    fn test_one_fourth() {
        assert_eq!(one_fourth(&[1, 2, 2, 3, 4]), 2);
        assert_eq!(one_fourth(&[1, 2, 3, 4]), -1);
    }
}
