use std::fs;
use std::cmp;

fn to_digit(byte: u8) -> u64 {
    (byte as u64)  - ('0' as u64)
}

fn fetch_best_seq(s: Vec<u8>) -> u64 {
    
    let mut best_prod: u64 = 1;
    let mut curr_prod: u64 = 1;
    let str_bytes = s;
    let mut prod_size = 0;

    for i in 0..str_bytes.len() {

        let curr_dig = to_digit(str_bytes[i]);
        if curr_dig == 0 {
            curr_prod = 1;
            prod_size = 0;
            continue;
        }

        if prod_size >= 13 && to_digit(str_bytes[i-13]) != 0 {
            curr_prod /= to_digit(str_bytes[i-13]);
        }
        
        curr_prod *= to_digit(str_bytes[i]);
        prod_size += 1;
        
        best_prod = cmp::max(best_prod, curr_prod);
    }

    best_prod
}

pub fn solve() -> u64 {
    
    let filename = "./src/sol/input0008.txt";
    let number = fs::read_to_string(filename)
                    .expect("rip").replace("\n", "");
    fetch_best_seq(number.as_bytes().to_vec())
}
