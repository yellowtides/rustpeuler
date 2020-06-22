// p14 sol https://projecteuler.net/problem=14

use std::collections::HashMap;

fn count_collatz_steps(n: u64, mem: &mut HashMap<u64, u64>) -> u64 {
    
    if n == 1 {
        return 1;
    } 

    if let Some(x) = mem.get(&n) {
        return *x;
    }
    
    let ans = 1 + match n % 2 {
        1 => count_collatz_steps(n*3+1, mem),
        _ => count_collatz_steps(n/2, mem)
    };
    mem.insert(n, ans);
    
    ans
}

fn find_longest_collatzchain(input: u64) -> u64 {
    
    let mut ans = 0;
    let mut max_chain_len = 0;

    let mut chain_len_all: HashMap<u64, u64> = HashMap::new();

    for i in 1..input {

        let curr_len = count_collatz_steps(i, &mut chain_len_all);

       if max_chain_len < curr_len {
            max_chain_len = curr_len;
            ans = i;
        }
    }

    ans
}

pub fn solve() -> u64 {
    
    find_longest_collatzchain(1e6 as u64)
}