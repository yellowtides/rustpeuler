// p15 sol https://projecteuler.net/problem=15

use std::collections::HashMap;

fn count_lattice(x: u64, y: u64, mem: &mut HashMap<(u64, u64), u64>) -> u64 {

    if let Some(z) = mem.get(&(x, y)) {
        return *z;
    }

    if x == 1 || y == 1 {
        return 1;
    }

    let ans = count_lattice(x-1, y, mem) + count_lattice(x, y-1, mem);
    mem.insert((x, y), ans);

    ans
}

pub fn solve() -> u64 {
    
    let mut lattice_mem: HashMap<(u64, u64), u64> = HashMap::new();

    count_lattice(21, 21, &mut lattice_mem)
}