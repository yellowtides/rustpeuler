extern crate plist;

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
        e @ _ => println!("uh oh, you might want a time machine for problem #{}", e),
    }
}