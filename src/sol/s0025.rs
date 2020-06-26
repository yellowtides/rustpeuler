use num_bigint::{BigInt, ToBigInt};

pub fn solve() -> u64 {

    let mut f_prev: BigInt = 0.to_bigint().unwrap();
    let mut f_curr: BigInt = 1.to_bigint().unwrap();
    let mut ans = 1;
    
    loop {
        f_curr = f_curr + &f_prev;
        f_prev = &f_curr - f_prev;
        ans += 1;

        if f_curr.to_string().len() >= 1000 {
            break; 
        }
    }
    
    ans
}