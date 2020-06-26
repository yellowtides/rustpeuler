use crate::base::log_pow;

fn sum_digit_power(x: u64, e: u64) -> u64 {

    x.to_string().chars()
    .fold(0, |sum, x| sum + log_pow(x as u64 - '0' as u64, e))
}

pub fn solve() -> u64 {
    
    let upper_bound: u64 = 5 * log_pow(9, 5) + 1;
    let mut ans: u64 = 0;

    for i in 10u64..upper_bound {

        if sum_digit_power(i, 5) == i {
            ans += i;
        }
    }

    ans
}