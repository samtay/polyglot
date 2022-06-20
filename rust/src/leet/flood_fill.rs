//! An image is represented by an m x n integer grid image where image[i][j]
//! represents the pixel value of the image.
//!
//! You are also given three integers sr, sc, and color. You should perform a
//! flood fill on the image starting from the pixel image[sr][sc].
//!
//! To perform a flood fill, consider the starting pixel, plus any pixels
//! connected 4-directionally to the starting pixel of the same color as the
//! starting pixel, plus any pixels connected 4-directionally to those pixels
//! (also with the same color), and so on. Replace the color of all of the
//! aforementioned pixels with color.

use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let sr = sr as usize;
        let sc = sc as usize;
        let og = image[sr][sc];

        let mut flood = Flood::new(&mut image, og, color);
        flood.fill(sr, sc);
        image
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
    fn test_flood_fill() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
            vec![vec![0, 0, 0], vec![0, 0, 0]]
        );
    }
}
