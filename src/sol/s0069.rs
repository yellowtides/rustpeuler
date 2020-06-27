// p69 sol https://projecteuler.net/problem=69

use crate::base;

const MAXVAL: usize = 1e2 as usize;

fn capped_prime_prod(input: u64) -> u64 {

    let sieve = base::compute_sieve(MAXVAL);
    // fetch the first 100 primes
    
    let mut ans: u64 = 1;
    let mut i: usize = 0;
    loop {
        if ans * sieve[i] < input {
            ans *= sieve[i];
            i += 1;
        }
        else {
            break;
        }
    }

    ans
}

pub fn solve() -> u64 {
    
    // this solution is based on a heuristic approach
    // try to minimise the totient by computing a product of unique minimal primes (using a sive)
    capped_prime_prod(1e6 as u64)
}