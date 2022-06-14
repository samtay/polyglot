/// You are a product manager and currently leading a team to develop a new
/// product. Unfortunately, the latest version of your product fails the quality
/// check. Since each version is developed based on the previous version, all
/// the versions after a bad version are also bad.
///
/// Suppose you have n versions [1, 2, ..., n] and you want to find out the
/// first bad one, which causes all the following ones to be bad.
///
/// You are given an API bool isBadVersion(version) which returns whether
/// version is bad. Implement a function to find the first bad version. You
/// should minimize the number of calls to the API.
use std::cmp;

/// The API isBadVersion is defined for you.
/// isBadVersion(version:i32)-> bool;
/// to call it use self.isBadVersion(version)
pub struct Solution {
    pub bad: i32,
}
impl Solution {
    fn is_bad_version(&self, version: i32) -> bool {
        self.bad <= version
    }

    /// Just a generic binary search
    ///
    /// We can do this because there is an ordering on [1..n];
    /// if some k is not bad, then we know target > k, and if k is bad, then we
    /// know target <= k
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut interval = n / 2;
        let mut offset = 1;
        let mut ix = interval + offset;
        let mut first_found = None;
        loop {
            dbg!(&ix, &interval, &offset, &first_found);
            let last_ix = ix;
            if self.is_bad_version(ix) {
                first_found = Some(ix);
            } else {
                offset = ix + 1;
            }
            interval /= 2;
            ix = cmp::min(interval + offset, n);
            // If we're stuck, then we've already found it!
            if last_ix == ix {
                return first_found.unwrap_or(-1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_bad_version_basic() {
        let sol = Solution { bad: 4 };
        assert_eq!(sol.first_bad_version(5), 4);

        let sol = Solution { bad: 1 };
        assert_eq!(sol.first_bad_version(1), 1);
    }

    #[test]
    fn test_first_bad_version_more() {
        let sol = Solution { bad: 67768599 };
        assert_eq!(sol.first_bad_version(75804946), 67768599);
    }
}
