// a prime generator for Erasthotene's sieve
pub fn compute_sieve(n: usize) -> Vec<u64> {
    // n <= 1 mil

    const MAX_ALLOWED: usize = 2e6 as usize;

    if n > MAX_ALLOWED {
        panic!("sieve input exceeds 10 million, which cannot be allowed");
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

// counts the number of divisors using a prime sieve and repeated division
pub fn count_div(n: u64, sieve: &Vec<u64>) -> u64 {

    let mut factor_this = n;
    let mut curr_prime = 0;
    let mut ans = 1;

    loop {

        if factor_this == 1 || factor_this < sieve[curr_prime] {
            break;
        }
    
        let mut no_div_curr = 0;
        while factor_this % sieve[curr_prime] == 0 {
            no_div_curr += 1; 
            factor_this /= sieve[curr_prime];
        }

        ans *= no_div_curr + 1;
        curr_prime += 1;
    }

    ans
}