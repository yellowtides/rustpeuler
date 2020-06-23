// p20 sol https://projecteuler.net/problem=20

use num_bigint::{ToBigInt};

pub fn solve() -> u64 {

    (1..101).fold(1.to_bigint().unwrap(), |prod, x| prod * x.to_bigint().unwrap())
    .to_string().chars()
    .fold(0, |sum, x| sum + x as u64 - '0' as u64)
    
}