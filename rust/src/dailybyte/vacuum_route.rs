//! This question is asked by Amazon. Given a string representing the sequence of moves a robot
//! vacuum makes, return whether or not it will return to its original position. The string will
//! only contain L, R, U, and D characters, representing left, right, up, and down respectively.

#[derive(Default)]
pub struct Position {
    horizontal: i8,
    vertical: i8,
}

impl Position {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            vertical: 0,
        }
    }

    pub fn move_to(&mut self, dir: char) {
        match dir {
            'L' => self.horizontal -= 1,
            'R' => self.horizontal += 1,
            'U' => self.vertical += 1,
            'D' => self.vertical -= 1,
            _ => panic!("Direction other than L,R,U,D"),
        }
    }

    pub fn is_at_origin(&self) -> bool {
        self.horizontal == 0 && self.vertical == 0
    }
}

pub fn vacuum_returns(s: &str) -> bool {
    s.chars()
        .fold(Position::new(), |mut pos, dir| {
            pos.move_to(dir);
            pos
        })
        .is_at_origin()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacuum_position() {
        assert!(vacuum_returns("LR"));
        assert!(!vacuum_returns("URURD"));
        assert!(vacuum_returns("RUULLDRD"));
    }
}
