// p3 sol https://projecteuler.net/problem=3

fn get_largest_factor(input: u64) -> u64 {
    // iterate from 2 to sqrt, performing a factorization

    let root: u64 = 1 + (input as f64).sqrt() as u64;
    let mut ans: u64 = input;
    let mut input = input;

    for i in 2..root {

        if i > input || input == 1 {
            break;
        }

        while input % i == 0 {
            input /= i;
            ans = i;
        }
    }

    ans
}

pub fn solve() -> u64 {
    
    get_largest_factor(600851475143u64)
}