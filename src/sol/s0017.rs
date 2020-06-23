// p17 sol https://projecteuler.net/problem=17

// x has to be up to 3 digits long
fn count_letters(x: usize) -> usize {

    fn transcribe_digit(y: usize) -> usize {
        match y {
            1 => "one".len(),
            2 => "two".len(),
            3 => "three".len(),
            4 => "four".len(),
            5 => "five".len(),
            6 => "six".len(),
            7 => "seven".len(),
            8 => "eight".len(),
            9 => "nine".len(),
            _ => 0
        }
    }

    let mut ans: usize = transcribe_digit(x % 10);

    if x / 10 % 10 == 1 {
        ans = 0;
    }

    ans += match x / 10 % 10 {
        1 => match x % 10 {
            1 => "eleven".len(),
            2 => "twelve".len(),
            3 => "thirteen".len(),
            4 => "fourteen".len(),
            5 => "fifteen".len(),
            6 => "sixteen".len(),
            7 => "seventeen".len(),
            8 => "eighteen".len(),
            9 => "nineteen".len(),
            _ => "ten".len()
        },
        2 => "twenty".len(),
        3 => "thirty".len(),
        4 => "forty".len(),
        5 => "fifty".len(),
        6 => "sixty".len(),
        7 => "seventy".len(),
        8 => "eighty".len(),
        9 => "ninety".len(),
        _ => 0
    };

    if x / 100 != 0 {
       
        if ans != 0 {
            ans += "and".len();
        }

        ans += transcribe_digit(x / 100) + "hundred".len();
    }

    ans
}

pub fn solve() -> u64 {
    
    let mut ans: u64 = 0;

    for i in 1..1000 {
        ans += count_letters(i) as u64;
    }

    ans += "onethousand".len() as u64;

    ans
}