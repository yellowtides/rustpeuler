// p2 sol https://projecteuler.net/problem=2

fn add_even_fib(input: u64) -> u64 {
    // basically, add terms whose indicies are divisible by 3
    
    let mut curr: u64 = 2;
    // current fib number (index mod 3 == 0)
    
    let mut prev: u64 = 1;
    // previous fib number (index mod 3 == 2)
    
    let mut ans: u64 = 2;
    // save the answer
    
    loop {

        for _ in 0..3 {
            // get next index divisibile by 3 by applying the fib iteration rule 3 times
            curr += prev;
            prev = curr - prev;
        }

        // break cond
        if curr > input {
            break;
        }

        ans += curr;
    }

    ans
}

pub fn solve() -> u64 {
    
    add_even_fib(4e6 as u64)
}