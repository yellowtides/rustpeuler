// p4 sol https://projecteuler.net/problem=4

fn is_valid(input: u64) -> bool {
    input > 99 && input < 1000
}

fn find_pal() -> u64 {
    // number clearly lies in 100001..997799 (~900 numbers to check)
    
    // to check for a viable product, naively iterate over 3 digit numbers and divide
    
    for tr in (100..998).rev() {

        let mut curr_iter: u64 = tr;
        let mut disposable_tr: u64 = tr;
        // auxiliary
        
        while disposable_tr != 0 {
            
            curr_iter = curr_iter * 10 + disposable_tr % 10;
            disposable_tr /= 10;
        } 
        // construct possible palindrome
        

        for i in 100..1000 {
            
            let split_left = i;
            let split_right = curr_iter/i;
            if !is_valid(split_right) {
                // make sure it has 3 digits
                continue;
            }

            if split_left * split_right == curr_iter {
                return curr_iter;
            }
            // check if the product matches the palindrome and return if it does
        }
    }

   0 
   // if all fails, output 0
}

pub fn solve() -> u64 {
    
    find_pal()

}