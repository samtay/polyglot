//! You are given an m x n binary matrix grid. An island is a group of 1's
//! (representing land) connected 4-directionally (horizontal or vertical.) You
//! may assume all four edges of the grid are surrounded by water.
//!
//! The area of an island is the number of cells with a value 1 in the island.
//!
//! Return the maximum area of an island in grid. If there is no island, return
//! 0.

use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
}

pub struct Flood<'a> {
    image: &'a mut Vec<Vec<i32>>,
    color: i32,
    og_color: i32,
    seen: HashSet<(usize, usize)>,
}

impl<'a> Flood<'a> {
    pub fn new(image: &'a mut Vec<Vec<i32>>, og_color: i32, color: i32) -> Self {
        Self {
            seen: HashSet::new(),
            og_color,
            color,
            image,
        }
    }

    pub fn fill(&mut self, sr: usize, sc: usize) {
        self.seen.insert((sr, sc));
        self.image[sr][sc] = self.color;

        if sr > 0 && self.should_fill(sr - 1, sc) {
            self.fill(sr - 1, sc);
        }
        if sc > 0 && self.should_fill(sr, sc - 1) {
            self.fill(sr, sc - 1);
        }
        if self.should_fill(sr + 1, sc) {
            self.fill(sr + 1, sc);
        }
        if self.should_fill(sr, sc + 1) {
            self.fill(sr, sc + 1);
        }
    }

    fn should_fill(&self, sr: usize, sc: usize) -> bool {
        sr < self.image.len()
            && sc < self.image[0].len()
            && self.image[sr][sc] == self.og_color
            && !self.seen.contains(&(sr, sc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );

        assert_eq!(
            Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]),
            0
        );
    }
}
