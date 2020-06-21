// p12 sol https://projecteuler.net/problem=12

use crate::base;

const MAXVAL: usize = 2e6 as usize;

fn find_triangle(input: u64) -> u64 {

    let sieve = base::compute_sieve(MAXVAL);
    // fetch the first 2 million primes
    
    let mut n = 1;
    loop {

        let triangle_equiv: u64 = n*(n+1) / 2;
        let n_div = base::count_div(triangle_equiv, &sieve);

        if n_div > input {
            return triangle_equiv;
        }
        
        n += 1;
    }
}

pub fn solve() -> u64 {
    
    find_triangle(500)
}