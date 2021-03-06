use plist;

use std::env;

// macro rules <=> pretty code

macro_rules! fetch_pres {
    ($arg:ident) => {
        println!("what you're looking for is {}", plist::$arg::solve());
    };
}

fn main() {
   
    // fetch + sanitise argument(s)

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("you forgot the magic number!1!!");
        return;
    }

    if args.len() > 2 {
        println!("your number of arguments is very impressive, you must be very proud");
        return;
    }
    // check for sole argument

    let arg = match args[1].parse::<u16>() {
        Ok(n)  => n, 
        // unwraps the given Result
        Err(_) => panic!("that's illegal you can't do that (only unsigned integers allowed!)")
    };
    // sanitise given argument

    // match argument
    match arg {
        1 => fetch_pres!(s0001),
        2 => fetch_pres!(s0002),
        3 => fetch_pres!(s0003),
        4 => fetch_pres!(s0004),
        5 => fetch_pres!(s0005),
        6 => fetch_pres!(s0006),
        7 => fetch_pres!(s0007),
        8 => fetch_pres!(s0008),
        9 => fetch_pres!(s0009),
        10 => fetch_pres!(s0010),
        12 => fetch_pres!(s0012),
        14 => fetch_pres!(s0014),
        15 => fetch_pres!(s0015),
        16 => fetch_pres!(s0016),
        17 => fetch_pres!(s0017),
        19 => fetch_pres!(s0019),
        20 => fetch_pres!(s0020),
        21 => fetch_pres!(s0021),
        25 => fetch_pres!(s0025),
        30 => fetch_pres!(s0030),
        51 => fetch_pres!(s0051),
        62 => fetch_pres!(s0062),
        69 => fetch_pres!(s0069),
        e @ _ => println!("uh oh, you might want a time machine for problem #{}", e),
    }
}
