//! This question is asked by Apple. Given two binary strings (strings containing only 1s and 0s)
//! return their sum (also as a binary string).  Note: neither binary string will contain leading 0s
//! unless the string itself is 0

use std::cmp::max;

pub fn add<S>(a: S, b: S) -> String
where
    S: Into<String>,
{
    let a = a.into();
    let b = b.into();
    let mut a_bits = a.chars().map(|c| c.to_digit(2).unwrap()).rev();
    let mut b_bits = b.chars().map(|c| c.to_digit(2).unwrap()).rev();
    let mut out = String::with_capacity(max(a.len(), b.len()) + 1);
    let mut carry = 0;

    loop {
        let a = a_bits.next();
        let b = b_bits.next();

        // Stopping point
        if let (None, None) = (a, b) {
            // No leading zeros
            if carry == 1 {
                out.push_str(&carry.to_string());
            }
            break out.chars().rev().collect();
        }

        // Binary add logic
        let num_ones = [a, b].iter().map(|m| m.unwrap_or(0)).sum();
        match num_ones {
            0 => {
                out.push_str(&carry.to_string());
                carry = 0;
            }
            1 => {
                if carry == 1 {
                    out.push('0');
                } else {
                    out.push('1');
                }
            }
            2 => {
                out.push_str(&carry.to_string());
                carry = 1;
            }
            _ => panic!("Invalid input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add("100", "1"), String::from("101"));
        assert_eq!(add("11", "1"), String::from("100"));
        assert_eq!(add("1", "0"), String::from("1"));
    }
}
