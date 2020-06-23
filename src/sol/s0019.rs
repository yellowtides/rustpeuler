// p19 sol https://projecteuler.net/problem=19

pub fn get_days_in_year(x: u64) -> u64 {

    if x % 4 == 0 && x % 100 != 0 {
        return 366;
    }

    return 365;
}

pub fn solve() -> u64 {
    
    let mut n_days: u64 = 0;

    for i in 1901..2001 {
        n_days += get_days_in_year(i as u64);
    }

    // 1 january 1901 (0th day) was a tuesday. therefore in this interval,
    // 5th day -> sunday
    // 12th day -> sunday
    // (-2 + 7i)th day -> sunday (more specifically, i sundays occured including this one)

    (n_days + 2) / 7
}