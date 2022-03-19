use rand::{distributions::Uniform, Rng};
use std::env;

pub fn main() {
    let n = env::args()
        .nth(1)
        .ok_or_else(|| panic!("expected argument INT"))
        .and_then(|s| s.parse::<usize>())
        .unwrap_or_else(|_| panic!("couldn't parse input into u8"));
    if n < 3 {
        panic!("expected input greater than 2");
    }
    let range = Uniform::<u8>::from(0..2);
    let mut row: Vec<u8> = rand::thread_rng().sample_iter(&range).take(n).collect();
    loop {
        println!("{:?}", row);
        if is_finished(&row) {
            break;
        }
        row = apply(row);
    }
}

struct Neighborhood<T> {
    left: T,
    center: T,
    right: T,
}

impl Neighborhood<u8> {
    pub fn rule(&self) -> u8 {
        match (self.left, self.center, self.right) {
            (1, 1, 1) => 0,
            (1, 1, 0) => 1,
            (1, 0, 1) => 1,
            (1, 0, 0) => 0,
            (0, 1, 1) => 1,
            (0, 1, 0) => 1,
            (0, 0, 1) => 1,
            (0, 0, 0) => 0,
            _ => panic!("should have used a better datatype!"),
        }
    }
}

/// Applies rule 110 to a row, i.e. a vector of bits.
/// N.B. using a Vec gives constant time look up to the neighborhood, hence this
/// function runs in O(n) time where n is the length of the vec.
pub fn apply(row: Vec<u8>) -> Vec<u8> {
    let n = row.len();
    let mut next = vec![0; n];
    for (ix, c) in next.iter_mut().enumerate() {
        let left = row[(ix + n - 1) % n];
        let center = row[ix];
        let right = row[(ix + 1) % n];
        *c = Neighborhood {
            left,
            center,
            right,
        }
        .rule();
    }
    next
}

pub fn is_finished(row: &[u8]) -> bool {
    row.iter().all(|&c| c == row[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule110() {
        assert_eq!(apply(vec![0, 1, 1, 1, 0, 1]), vec![1, 1, 0, 1, 1, 1])
    }
}
