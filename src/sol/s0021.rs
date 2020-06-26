// p21 sol https://projecteuler.net/problem=21

use std::collections::HashSet;

fn fetch_d(x: u64) -> u64 {

    let mut ans: u64 = 0;

    for i in 1..x/2+1 {
        
        if x % i == 0 {
            ans += i;
        }
    }

    ans
}

pub fn solve() -> u64 {

    let mut seen_before: HashSet<u64> = HashSet::new();

    let mut ans = 0;

    for i in 2..10000 {
        
        let i: u64 = i as u64;

        if seen_before.contains(&i) {
            continue;
        }

        let div_sum: u64 = fetch_d(i);
        if fetch_d(div_sum) == i && i != div_sum {
            ans += i;
            if div_sum < 10000 {
                ans += div_sum;
            }
        }

        seen_before.insert(i);
        seen_before.insert(div_sum);
    }
    
    ans
}