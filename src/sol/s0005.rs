// p5 sol https://projecteuler.net/problem=5

use crate::base;

fn prime_prod(input: usize) -> u64 {

    let sieve = base::compute_sieve(input);
    // fetch the first input primes
    
    let mut ans: u64 = 1;
    
    for x in sieve {
        
        let mut maxexp = 1;
        // maximum power of prime that would fit in the given input
        let mut container: u64 = input as u64;

        while container > 0 {
            container /= x;
            maxexp *= x;
        }
        maxexp /= x;
        // remove the last one as it did not fit
        
        ans *= maxexp;
        // multiply prime power to answer

    }
    
    ans
}

pub fn solve() -> u64 {
    
    prime_prod(20)
}