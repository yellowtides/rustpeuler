// p62 sol https://projecteuler.net/problem=62

fn conv_to_digit_array(mut input: u64) -> [u64; 10] {

    let mut fr: [u64; 10] = [0; 10];

    while input != 0 {
        let dig = input % 10;
        fr[dig as usize] += 1;
        input /= 10;
    }

    fr
}

pub fn solve() -> u64 {
    
    print!("{:?}", conv_to_digit_array(355361290));
    0
}