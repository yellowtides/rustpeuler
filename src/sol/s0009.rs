// p9 sol https://projecteuler.net/problem=9

fn find_product(sum: u64) -> u64 {

    for c in 1..sum {
        for a in 1..c {

            let b = sum - a - c;
            if c*c == a*a + b*b {
                return a*b*c;
            }
            
        }
    }

    0
}

pub fn solve() -> u64 {
    
    find_product(1000)
}