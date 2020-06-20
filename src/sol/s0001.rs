// p1 sol https://projecteuler.net/problem=1

fn addreq(input: u32) -> u32 {
    // add all numbers div. by 3 or 5 from 1 to input inc.

    (1..input).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
    // filter with requirement
}

pub fn solve() -> u32 {

    addreq(1000)
}