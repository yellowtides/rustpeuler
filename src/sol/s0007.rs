// p5 sol https://projecteuler.net/problem=5

use crate::base;

const MAXVAL: usize = 1e6 as usize;

fn nth_prime(input: usize) -> u64 {

    let sieve = base::compute_sieve(MAXVAL);
    // fetch the first input primes
    
    sieve[input]
}

pub fn solve() -> u64 {
    
    nth_prime(10000)
}