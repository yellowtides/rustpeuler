// p7 sol https://projecteuler.net/problem=7

use crate::base;

const MAXVAL: usize = 1e6 as usize;

fn nth_prime(input: usize) -> u64 {

    let sieve = base::compute_sieve(MAXVAL);
    // fetch the first 1 million primes
    
    sieve[input]
}

pub fn solve() -> u64 {
    
    nth_prime(10000)
}