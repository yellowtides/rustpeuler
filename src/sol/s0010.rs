// p10 sol https://projecteuler.net/problem=10

use crate::base;

fn prime_sum(input: usize) -> u64 {

    let sieve = base::compute_sieve(input);
    // fetch the first input primes
    
    sieve.iter().sum()
}

pub fn solve() -> u64 {
    
    prime_sum(2e6 as usize)
}