// p6 sol https://projecteuler.net/problem=6

pub fn solve() -> u64 {
    
    let n = 100;
    n*n*(n+1)*(n+1)/4 - (1..n+1).fold(0, |sum, x| sum + x*x)
}