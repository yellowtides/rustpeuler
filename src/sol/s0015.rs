// p15 sol https://projecteuler.net/problem=15

fn find_longest_collatzchain(x: u64, y: u64) -> u64 {

    if (x == 1 || y == 1) {
        return 1;
    }

    return find_longest_collatzchain(x-1, y) + find_longest_collatzchain(x, y-1);
}

pub fn solve() -> u64 {
    
    count_lattice(20, 20)
}