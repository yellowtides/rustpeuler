// p14 sol https://projecteuler.net/problem=14

use std::collections::HashMap;

fn count_collatz_steps(n: u64, mut mem: &mut HashMap<&u64, u64>) -> u64 {
    
    // if mem.contains_key(&n) {
    //     return *mem.get(&n).unwrap();
    // }

    let ans;
    
    if n == 1 {
        ans = 1;
    } 
    else if n % 2 == 0 {
        ans = 1 + count_collatz_steps((&n)/2, &mut mem);
    }
    else {
        ans = 1 + count_collatz_steps(3*(&n)+1, &mut mem);
    }

    // mem.insert(&n, ans);

    ans
}

fn find_longest_collatzchain(input: u64) -> u64 {
    
    let mut ans = 0;
    let mut max_chain_len = 0;

    let mut chain_len_all: HashMap<&u64, u64> = HashMap::new();

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