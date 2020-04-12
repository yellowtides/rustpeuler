fn solve(input: u32) -> u32 {
    (1..input).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
}

fn main() {
    println!("the sum is {}", solve(1000));
}
