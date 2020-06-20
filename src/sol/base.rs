// a prime generator for Erasthotene's sieve
pub fn compute_sieve(n: usize) -> Vec<u64> {
    // n <= 1 mil

    const MAX_ALLOWED: usize = 1e6 as usize;

    if n > MAX_ALLOWED {
        panic!("sieve input exceeds 1 million, which cannot be allowed");
    }

    let mut table: [bool; MAX_ALLOWED] = [true; MAX_ALLOWED];
    for i in 2..n {
        for j in (i+i..n).step_by(i) {
            table[j] = false;
        }
    }

    let mut ans = Vec::new();
    for i in 2..n {
        if table[i] == true {
            ans.push(i as u64);
        }
    }

    ans
}

// logarithmic exponentiation
pub fn log_pow(base: u64, exp: u64) -> u64 {

    if exp == 0 {
        return 1;
    }

    match exp % 2 {
        0 => { 
            let geom_half = log_pow(base, exp/2);
            geom_half * geom_half
        },
        _ => base * log_pow(base, exp-1)
    }
}