// p4 sol https://projecteuler.net/problem=4

use crate::base;

fn valid(input: u64) -> bool {
    input > 99 && input < 1000
}

fn find_pal() -> u64 {
    // number clearly lies in 100001..997799 (~900 numbers to check)
    
    // to check for a viable product, greedily split the prime factors into two products
    // and check if they're consisting of 3 digits
    
    for tr in (100..998).rev() {

        let mut curr_iter: u64 = tr;
        let mut disposable_tr: u64 = tr;
        // auxiliary
        
        while disposable_tr != 0 {
            
            curr_iter = curr_iter * 10 + disposable_tr % 10;
            disposable_tr /= 10;
        } 
        // construct possible palindrome
        
        let mut split_left: u64 = 1;
        let mut split_right: u64 = 1;
        // the two halves (greedily constructed prime factor products)
        
        let perma_iter = curr_iter;
        // the candidate palindrome

        let root: u64 = 1 + (curr_iter as f64).sqrt() as u64;
        // sqrt(tr)

        for i in 2..root {

            if curr_iter == 1 {
                break;
            }

            while curr_iter % i == 0 {
                // factor found!
                
                curr_iter /= i;
                if split_left < split_right {
                    split_left *= i;
                }
                else {
                    split_right *= i;
                }
            }
            // construct the two halves
        }

        if valid(split_left) && valid(split_right) {
            return perma_iter;
        }
    }

   0 
}

pub fn solve() -> u64 {
    
    println!("{:?}", base::compute_sieve(1000000));
    find_pal()

}