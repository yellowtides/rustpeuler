// p16 sol https://projecteuler.net/problem=16

use crate::base;

pub fn solve() -> u64 {
    
    base::log_pow_big(2, 1000)
        .to_string().chars()
        .fold(0, |sum, x| sum + x as u64 - '0' as u64)
}