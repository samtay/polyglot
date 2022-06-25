//! You are climbing a staircase. It takes n steps to reach the top.
//!
//! Each time you can either climb 1 or 2 steps. In how many distinct ways can
//! you climb to the top?

pub struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }
        let n = n as usize;
        let mut climbs = vec![0; n];
        climbs[0] = 1;
        climbs[1] = 2;
        for i in 2..n {
            climbs[i] = climbs[i - 1] + climbs[i - 2];
        }
        climbs[n - 1] as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
