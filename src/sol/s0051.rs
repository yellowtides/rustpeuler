// p51 sol https://projecteuler.net/problem=51
// for this problem we assume that the solution is a 6 digit (or less) number


/*  generate the ~11^6 possible "number families" (i.e., 1*21, 31**) with at least 1 wildcard digit
    replace the wildcards with 0..9, see if at least 8 match (using a prime hashset)
        > if yes, put the smallest digit for which it was prime and compare to the minimum
        > if no, continue
    in total, there should be < 20 million operations performed (~1s) */

const ALPHABET: [char; 11] = ['*', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const MAX_SIZE: usize = 6;

fn fetch_words() -> Vec<String> {

    let mut ans: Vec<String> = [].to_vec();

    // construct all words via a rust implementation of backtrack
    fn backtrack(mut curr_gen: &mut Vec<char>, curr_index: usize, mut ans: &mut Vec<String>) -> () {

        if curr_index == MAX_SIZE {
            let candidate: Vec<char> = curr_gen.to_vec();
            
            if (&candidate).into_iter().fold(false, |check, x| check || (*x == '*')) {
                // check for the existence of at least 1 wildcard (this conditional must be true for all words
                ans.push(candidate.into_iter().collect::<String>());
            }

            return ();
        }

        for x in &ALPHABET {
            curr_gen.push(*x);
            backtrack(&mut curr_gen, curr_index+1, &mut ans);
            curr_gen.pop();
        }
    }

    backtrack(&mut [].to_vec(), 0, &mut ans);

    ans
}

// fn check_family(num: String) -> bool {
    
//     let st: usize = 0;

//     if num[0] == '*' {
//         st = 1;
//     }
//     // edge case: *53 with * = 0 is not a prime, it is an invalid number

//     for wildcard in st..10 {
//         if replace_p
//     }
// }

pub fn solve() -> u64 {
    
    println!("{:?}", fetch_words());
    0
}